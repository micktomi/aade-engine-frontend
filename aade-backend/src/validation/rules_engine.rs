use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::domain::invoice::Invoice;
use crate::validation::result::{ValidationReport, Severity};
use rust_decimal::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RuleLogic {
    /// Checks if a field on a specific line matches allowed values
    /// Example: "line.vat_category" must be in ["1", "2"]
    LineValueAllowed {
        field_path: String, // e.g., "vat_category" (relative to line)
        allowed_values: Vec<String>,
    },

    /// Complex check: If Header Field == X, then Line Field must be in [Y, Z]
    /// Example: If invoice_type == "1.1", then line.vat_category in ["1", "2", "3"]
    HeaderDependencyLine {
        header_field: String,      // e.g., "invoice_type"
        header_value: String,      // e.g., "1.1"
        line_check_field: String,  // e.g., "vat_category"
        allowed_values: Vec<String>,
    },

    /// Checks if a header field value is in allowed values.
    HeaderValueAllowed {
        field_path: String,      // e.g., "invoice_type"
        allowed_values: Vec<String>,
    },

    /// Cross-field check on header:
    /// If `when_field` is in `when_values`, then `required_field` must be present (non-empty)
    HeaderConditionalRequired {
        when_field: String,      // e.g., "invoice_type"
        when_values: Vec<String>,// e.g., ["5.1"]
        required_field: String,  // e.g., "correlated_invoices"
    },

    /// Cross-field check on line level:
    /// If `when_field` is in `when_values`, then `required_field` must be present (non-empty)
    LineConditionalRequired {
        when_field: String,      // e.g., "vat_category"
        when_values: Vec<String>,// e.g., ["7"]
        required_field: String,  // e.g., "vat_exemption_cause"
    },

    /// Checks if payment method detail field has allowed values.
    PaymentMethodValueAllowed {
        field_path: String,      // e.g., "type"
        allowed_values: Vec<String>,
    },

    /// Check if counterpart is required based on invoice type
    /// Example: Invoice type 1.1 requires counterpart
    CounterpartRequired {
        invoice_types: Vec<String>, // e.g., ["1.1", "1.2"]
    },

    /// Check if classification is required based on invoice type
    ClassificationRequired {
        invoice_types: Vec<String>,
        min_classifications: usize, // Minimum number required
    },

    /// Check currency rules
    /// Example: If currency != EUR, exchange_rate must exist
    CurrencyExchangeRate {
        default_currency: String, // "EUR"
    },

    /// Check that counterpart country matches expected pattern
    /// Example: For type 1.2 (intra-EU), country must not be GR
    CounterpartCountry {
        invoice_types: Vec<String>,
        excluded_countries: Vec<String>,
    },

    /// Check that all lines have negative values (for credit notes)
    NegativeAmountsOnly {
        invoice_types: Vec<String>,
    },

    /// Check that no lines have negative values (for normal invoices)
    NoNegativeAmounts {
        invoice_types: Vec<String>,
    },

    /// Check specific classification types are present
    ClassificationTypeRequired {
        invoice_types: Vec<String>,
        required_types: Vec<String>, // e.g., ["E3_561_001"]
    },

    /// Range check for total fields (non-negative amounts)
    /// Example: total_withheld_amount >= 0
    TotalFieldNonNegative {
        field_path: String,           // e.g., "total_withheld_amount"
        invoice_types: Option<Vec<String>>,
    },

    /// Comparison between two total fields
    /// Example: deductions <= net_amount
    TotalFieldComparison {
        field_a: String,               // e.g., "total_deductions_amount"
        field_b: String,               // e.g., "total_net_amount"
        comparison: String,            // "LessThanOrEqual"
    },

    /// Range check for numeric fields (branch, amounts)
    /// Example: issuer.branch >= 0
    FieldNonNegative {
        field_path: String,            // e.g., "issuer.branch"
    },

    /// Range check for line fields
    /// Example: line.net_value > 0 (warning if zero)
    LineFieldNonZero {
        field_path: String,            // e.g., "net_value"
    },

    /// Withheld tax percentage validation (20%)
    WithheldTaxPercentage {
        expected_rate: f64,            // e.g., 0.20 (20%)
        tolerance: f64,                // e.g., 0.001 (0.1%)
    },

    /// Stamp duty percentage validation (3.6%)
    StampDutyPercentage {
        expected_rate: f64,            // e.g., 0.036 (3.6%)
        tolerance: f64,                // e.g., 0.001 (0.1%)
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDefinition {
    pub id: String,
    pub description: String,
    pub severity: Severity,
    pub logic: RuleLogic,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    pub version: String,
    pub rules: Vec<RuleDefinition>,
}

#[derive(Default)]
pub struct RulesEngine {
    rules: Vec<RuleDefinition>,
}

impl RulesEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_yaml(content: &str) -> Result<Self, serde_yaml::Error> {
        let rule_set: RuleSet = serde_yaml::from_str(content)?;
        Ok(Self { rules: rule_set.rules })
    }

    pub fn apply(&self, invoice: &Invoice, report: &mut ValidationReport) {
        let inv_type_str = invoice.header.invoice_type.to_string();
        let currency = &invoice.header.currency;
        let header_json = serde_json::to_value(&invoice.header).ok();
        let line_json_cache: Vec<Option<Value>> = invoice.lines
            .iter()
            .map(|line| serde_json::to_value(line).ok())
            .collect();
        let payment_method_json_cache: Vec<Option<Value>> = invoice.payment_methods
            .iter()
            .map(|pm| serde_json::to_value(pm).ok())
            .collect();

        for rule in &self.rules {
            match &rule.logic {
                RuleLogic::LineValueAllowed { field_path, allowed_values } => {
                    let field_path = strip_known_prefix(field_path, "line.");
                    for (idx, line_json) in line_json_cache.iter().enumerate() {
                        let val = line_json
                            .as_ref()
                            .and_then(|line| json_field_to_string(line, field_path));

                        if let Some(v) = val {
                            if !allowed_values.contains(&v) {
                                let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                                msg = msg.replace("{line}", &(idx + 1).to_string());
                                Self::add_rule_error(report, rule, &msg, Some(&format!("line[{}].{}", idx+1, field_path)), Some(&v));
                            }
                        }
                    }
                },

                RuleLogic::HeaderDependencyLine { header_field, header_value, line_check_field, allowed_values } => {
                    let header_match = header_json
                        .as_ref()
                        .and_then(|header| json_field_to_string(header, strip_known_prefix(header_field, "header.")))
                        .is_some_and(|value| value == *header_value);

                    if header_match {
                        let line_check_field = strip_known_prefix(line_check_field, "line.");
                        for (idx, line_json) in line_json_cache.iter().enumerate() {
                            let line_val = line_json
                                .as_ref()
                                .and_then(|line| json_field_to_string(line, line_check_field));

                            if let Some(v) = line_val {
                                if !allowed_values.contains(&v) {
                                    let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                                    msg = msg.replace("{line}", &(idx + 1).to_string());
                                    Self::add_rule_error(report, rule, &msg, Some(&format!("line[{}].{}", idx+1, line_check_field)), Some(&v));
                                }
                            }
                        }
                    }
                },

                RuleLogic::HeaderValueAllowed { field_path, allowed_values } => {
                    let field_path = strip_known_prefix(field_path, "header.");
                    let header_value = header_json
                        .as_ref()
                        .and_then(|header| json_field_to_string(header, field_path));

                    if let Some(v) = header_value {
                        if !allowed_values.contains(&v) {
                            let msg = render_message(&rule.error_message, &inv_type_str, currency);
                            Self::add_rule_error(report, rule, &msg, Some(field_path), Some(&v));
                        }
                    }
                },

                RuleLogic::HeaderConditionalRequired { when_field, when_values, required_field } => {
                    let when_field = strip_known_prefix(when_field, "header.");
                    let required_field = strip_known_prefix(required_field, "header.");

                    let when_value = header_json
                        .as_ref()
                        .and_then(|header| json_field_to_string(header, when_field));

                    if let Some(v) = when_value {
                        if when_values.contains(&v) {
                            let required_value = header_json
                                .as_ref()
                                .and_then(|header| json_field_to_string(header, required_field));
                            let is_missing = required_value
                                .as_ref()
                                .is_none_or(|value| value.trim().is_empty());

                            if is_missing {
                                let msg = render_message(&rule.error_message, &inv_type_str, currency);
                                Self::add_rule_error(report, rule, &msg, Some(required_field), Some(&v));
                            }
                        }
                    }
                },

                RuleLogic::LineConditionalRequired { when_field, when_values, required_field } => {
                    let when_field = strip_known_prefix(when_field, "line.");
                    let required_field = strip_known_prefix(required_field, "line.");

                    for (idx, line_json) in line_json_cache.iter().enumerate() {
                        let Some(line_json) = line_json.as_ref() else {
                            continue;
                        };

                        let Some(when_value) = json_field_to_string(line_json, when_field) else {
                            continue;
                        };

                        if !when_values.contains(&when_value) {
                            continue;
                        }

                        let required_value = json_field_to_string(line_json, required_field);
                        let is_missing = required_value
                            .as_ref()
                            .is_none_or(|v| v.trim().is_empty());

                        if is_missing {
                            let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                            msg = msg.replace("{line}", &(idx + 1).to_string());
                            Self::add_rule_error(
                                report,
                                rule,
                                &msg,
                                Some(&format!("line[{}].{}", idx + 1, required_field)),
                                Some(&when_value),
                            );
                        }
                    }
                },

                RuleLogic::PaymentMethodValueAllowed { field_path, allowed_values } => {
                    let field_path = strip_known_prefix(field_path, "payment.");

                    for (idx, pm_json) in payment_method_json_cache.iter().enumerate() {
                        let value = pm_json
                            .as_ref()
                            .and_then(|pm| json_field_to_string(pm, field_path));

                        if let Some(v) = value {
                            if !allowed_values.contains(&v) {
                                let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                                msg = msg.replace("{line}", &(idx + 1).to_string());
                                Self::add_rule_error(
                                    report,
                                    rule,
                                    &msg,
                                    Some(&format!("paymentMethodDetails[{}].{}", idx + 1, field_path)),
                                    Some(&v),
                                );
                            }
                        }
                    }
                },

                RuleLogic::CounterpartRequired { invoice_types } => {
                    if invoice_types.contains(&inv_type_str) && invoice.counterpart.is_none() {
                        let msg = render_message(&rule.error_message, &inv_type_str, currency);
                        Self::add_rule_error(report, rule, &msg, Some("counterpart"), None);
                    }
                },

                RuleLogic::ClassificationRequired { invoice_types, min_classifications } => {
                    if invoice_types.contains(&inv_type_str)
                        && invoice.income_classifications.len() < *min_classifications {
                            let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                            msg = msg.replace("{count}", &invoice.income_classifications.len().to_string());
                            Self::add_rule_error(report, rule, &msg, Some("incomeClassification"), None);
                        }
                },

                RuleLogic::CurrencyExchangeRate { default_currency } => {
                    if invoice.header.currency != *default_currency && invoice.header.exchange_rate.is_none() {
                        let msg = render_message(&rule.error_message, &inv_type_str, currency);
                        Self::add_rule_error(report, rule, &msg, Some("exchangeRate"), Some(&invoice.header.currency));
                    }
                },

                RuleLogic::CounterpartCountry { invoice_types, excluded_countries } => {
                    if invoice_types.contains(&inv_type_str) {
                        if let Some(cp) = &invoice.counterpart {
                            if excluded_countries.contains(&cp.country) {
                                let msg = render_message(&rule.error_message, &inv_type_str, currency);
                                Self::add_rule_error(report, rule, &msg, Some("counterpart.country"), Some(&cp.country));
                            }
                        }
                    }
                },

                RuleLogic::NegativeAmountsOnly { invoice_types } => {
                    if invoice_types.contains(&inv_type_str) {
                        for (idx, line) in invoice.lines.iter().enumerate() {
                            if line.net_value.is_sign_positive() {
                                let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                                msg = msg.replace("{line}", &(idx + 1).to_string());
                                Self::add_rule_error(report, rule, &msg, Some(&format!("line[{}].netValue", idx+1)), Some(&line.net_value.to_string()));
                            }
                        }
                    }
                },

                RuleLogic::NoNegativeAmounts { invoice_types } => {
                    if invoice_types.contains(&inv_type_str) {
                        for (idx, line) in invoice.lines.iter().enumerate() {
                            if line.net_value.is_sign_negative() {
                                let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                                msg = msg.replace("{line}", &(idx + 1).to_string());
                                Self::add_rule_error(report, rule, &msg, Some(&format!("line[{}].netValue", idx+1)), Some(&line.net_value.to_string()));
                            }
                        }
                    }
                },

                RuleLogic::ClassificationTypeRequired { invoice_types, required_types } => {
                    if invoice_types.contains(&inv_type_str) {
                        for req_type in required_types {
                            let found = invoice.income_classifications.iter()
                                .any(|c| c.classification_type.as_ref() == Some(req_type));

                            if !found {
                                let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                                msg = msg.replace("{type}", req_type);
                                Self::add_rule_error(report, rule, &msg, Some("incomeClassification"), Some(req_type));
                            }
                        }
                    }
                },

                RuleLogic::TotalFieldNonNegative { field_path, invoice_types } => {
                    // Check if rule applies to this invoice type
                    if let Some(types) = invoice_types {
                        if !types.contains(&inv_type_str) {
                            continue;
                        }
                    }

                    let value = match field_path.as_str() {
                        "total_withheld_amount" => Some(invoice.totals.total_withheld_amount),
                        "total_stamp_duty_amount" => Some(invoice.totals.total_stamp_duty_amount),
                        "total_fees_amount" => Some(invoice.totals.total_fees_amount),
                        "total_deductions_amount" => Some(invoice.totals.total_deductions_amount),
                        _ => None,
                    };

                    if let Some(val) = value {
                        if val.is_sign_negative() {
                            let msg = render_message(&rule.error_message, &inv_type_str, currency);
                            Self::add_rule_error(report, rule, &msg, Some(field_path), Some(&val.to_string()));
                        }
                    }
                },

                RuleLogic::TotalFieldComparison { field_a, field_b, comparison } => {
                    let val_a = match field_a.as_str() {
                        "total_deductions_amount" => Some(invoice.totals.total_deductions_amount),
                        "total_withheld_amount" => Some(invoice.totals.total_withheld_amount),
                        _ => None,
                    };

                    let val_b = match field_b.as_str() {
                        "total_net_amount" => Some(invoice.totals.total_net_amount),
                        "total_gross_amount" => Some(invoice.totals.total_gross_amount),
                        _ => None,
                    };

                    if let (Some(a), Some(b)) = (val_a, val_b) {
                        let violates = match comparison.as_str() {
                            "LessThanOrEqual" => a > b,
                            "LessThan" => a >= b,
                            "GreaterThanOrEqual" => a < b,
                            "GreaterThan" => a <= b,
                            _ => false,
                        };

                        if violates {
                            let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                            msg = msg.replace("{value_a}", &a.to_string());
                            msg = msg.replace("{value_b}", &b.to_string());
                            Self::add_rule_error(report, rule, &msg, Some(field_a), Some(&a.to_string()));
                        }
                    }
                },

                RuleLogic::FieldNonNegative { field_path } => {
                    let value = match field_path.as_str() {
                        "issuer.branch" => Some(invoice.issuer.branch),
                        "counterpart.branch" => invoice.counterpart.as_ref().map(|cp| cp.branch),
                        _ => None,
                    };

                    if let Some(val) = value {
                        if val < 0 {
                            let msg = render_message(&rule.error_message, &inv_type_str, currency);
                            Self::add_rule_error(report, rule, &msg, Some(field_path), Some(&val.to_string()));
                        }
                    }
                },

                RuleLogic::LineFieldNonZero { field_path } => {
                    for (idx, line) in invoice.lines.iter().enumerate() {
                        let value = match field_path.as_str() {
                            "net_value" => Some(line.net_value),
                            "vat_amount" => Some(line.vat_amount),
                            _ => None,
                        };

                        if let Some(val) = value {
                            if val.is_zero() {
                                let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                                msg = msg.replace("{line}", &(idx + 1).to_string());
                                Self::add_rule_error(report, rule, &msg, Some(&format!("line[{}].{}", idx+1, field_path)), Some("0"));
                            }
                        }
                    }
                },

                RuleLogic::WithheldTaxPercentage { expected_rate, tolerance } => {
                    let withheld = invoice.totals.total_withheld_amount;
                    let net = invoice.totals.total_net_amount;

                    if !withheld.is_zero() && !net.is_zero() {
                        let actual_rate = (withheld / net).to_f64().unwrap_or(0.0);
                        let diff = (actual_rate - expected_rate).abs();

                        if diff > *tolerance {
                            let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                            msg = msg.replace("{actual}", &format!("{:.1}%", actual_rate * 100.0));
                            msg = msg.replace("{expected}", &format!("{:.1}%", expected_rate * 100.0));
                            Self::add_rule_error(report, rule, &msg, Some("totals.total_withheld_amount"), Some(&withheld.to_string()));
                        }
                    }
                },

                RuleLogic::StampDutyPercentage { expected_rate, tolerance } => {
                    let stamp = invoice.totals.total_stamp_duty_amount;
                    let net = invoice.totals.total_net_amount;

                    if !stamp.is_zero() && !net.is_zero() {
                        let actual_rate = (stamp / net).to_f64().unwrap_or(0.0);
                        let diff = (actual_rate - expected_rate).abs();

                        if diff > *tolerance {
                            let mut msg = render_message(&rule.error_message, &inv_type_str, currency);
                            msg = msg.replace("{actual}", &format!("{:.2}%", actual_rate * 100.0));
                            msg = msg.replace("{expected}", &format!("{:.2}%", expected_rate * 100.0));
                            Self::add_rule_error(report, rule, &msg, Some("totals.total_stamp_duty_amount"), Some(&stamp.to_string()));
                        }
                    }
                },
            }
        }
    }

    fn add_rule_error(report: &mut ValidationReport, rule: &RuleDefinition, message: &str, field: Option<&str>, value: Option<&str>) {
        match rule.severity {
            Severity::Error => report.add_error(&rule.id, message, field, value),
            Severity::Warning => report.add_warning(&rule.id, message),
            Severity::Info => report.add_info(&rule.id, message, field, value),
        }
    }
}

fn strip_known_prefix<'a>(field_path: &'a str, prefix: &str) -> &'a str {
    field_path.strip_prefix(prefix).unwrap_or(field_path)
}

fn json_field_to_string(root: &Value, field_path: &str) -> Option<String> {
    let mut current = root;

    for segment in field_path.split('.') {
        let Value::Object(map) = current else {
            return None;
        };

        if let Some(next) = map.get(segment) {
            current = next;
            continue;
        }

        let camel = snake_to_camel(segment);
        current = map.get(&camel)?;
    }

    match current {
        Value::Null => None,
        Value::String(v) => Some(v.clone()),
        Value::Bool(v) => Some(v.to_string()),
        Value::Number(v) => Some(v.to_string()),
        other => Some(other.to_string()),
    }
}

fn snake_to_camel(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut upper_next = false;

    for ch in input.chars() {
        if ch == '_' {
            upper_next = true;
            continue;
        }

        if upper_next {
            result.push(ch.to_ascii_uppercase());
            upper_next = false;
        } else {
            result.push(ch);
        }
    }

    result
}

fn render_message(template: &str, invoice_type: &str, currency: &str) -> String {
    template
        .replace("{invoice_type}", invoice_type)
        .replace("{currency}", currency)
}

#[cfg(test)]
mod tests {
    use super::RulesEngine;
    use crate::domain::classification::IncomeClassification;
    use crate::domain::enums::{InvoiceType, VatCategory};
    use crate::domain::invoice::{Invoice, InvoiceHeader, InvoiceLine, Issuer, PaymentMethodDetail};
    use crate::domain::totals::InvoiceTotals;
    use crate::validation::result::ValidationReport;
    use chrono::NaiveDate;
    use rust_decimal_macros::dec;

    fn base_invoice(vat_category: VatCategory, vat_exemption_cause: Option<&str>) -> Invoice {
        Invoice {
            uid: None,
            header: InvoiceHeader {
                series: "A".to_string(),
                aa: "1".to_string(),
                issue_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                issue_time: None,
                invoice_type: InvoiceType::SalesInvoice,
                currency: "EUR".to_string(),
                exchange_rate: None,
                correlated_invoices: None,
            },
            issuer: Issuer {
                vat_number: "123456789".to_string(),
                country: "GR".to_string(),
                branch: 0,
            },
            counterpart: None,
            lines: vec![InvoiceLine {
                line_number: 1,
                description: "test".to_string(),
                net_value: dec!(100),
                vat_category,
                vat_amount: dec!(0),
                vat_exemption_cause: vat_exemption_cause.map(|v| v.to_string()),
                quantity: None,
                measurement_unit: None,
            }],
            payment_methods: vec![],
            totals: InvoiceTotals {
                total_net_amount: dec!(100),
                total_vat_amount: dec!(0),
                total_withheld_amount: dec!(0),
                total_fees_amount: dec!(0),
                total_stamp_duty_amount: dec!(0),
                total_deductions_amount: dec!(0),
                total_gross_amount: dec!(100),
            },
            vat_breakdown: vec![],
            income_classifications: Vec::<IncomeClassification>::new(),
        }
    }

    #[test]
    fn line_conditional_required_reports_missing_field() {
        let yaml = r#"
version: "test"
rules:
  - id: "VAT-EXM-001"
    description: "Require VAT exemption cause for 0% VAT lines"
    severity: "Error"
    logic:
      type: "LineConditionalRequired"
      when_field: "vat_category"
      when_values: ["7"]
      required_field: "vat_exemption_cause"
    error_message: "Line {line} is missing exemption cause"
"#;

        let engine = RulesEngine::load_from_yaml(yaml).expect("rules should load");
        let invoice = base_invoice(VatCategory::Vat0, None);
        let mut report = ValidationReport::new();

        engine.apply(&invoice, &mut report);

        assert_eq!(report.errors.len(), 1);
        assert_eq!(report.errors[0].code, "VAT-EXM-001");
    }

    #[test]
    fn line_conditional_required_passes_when_field_exists() {
        let yaml = r#"
version: "test"
rules:
  - id: "VAT-EXM-001"
    description: "Require VAT exemption cause for 0% VAT lines"
    severity: "Error"
    logic:
      type: "LineConditionalRequired"
      when_field: "vat_category"
      when_values: ["7"]
      required_field: "vat_exemption_cause"
    error_message: "Line {line} is missing exemption cause"
"#;

        let engine = RulesEngine::load_from_yaml(yaml).expect("rules should load");
        let invoice = base_invoice(VatCategory::Vat0, Some("article_43"));
        let mut report = ValidationReport::new();

        engine.apply(&invoice, &mut report);

        assert!(report.errors.is_empty());
    }

    #[test]
    fn line_value_allowed_can_read_new_optional_field() {
        let yaml = r#"
version: "test"
rules:
  - id: "FIELD-001"
    description: "Allowed values for vat_exemption_cause"
    severity: "Error"
    logic:
      type: "LineValueAllowed"
      field_path: "vat_exemption_cause"
      allowed_values: ["article_43"]
    error_message: "line {line} has invalid exemption cause"
"#;

        let engine = RulesEngine::load_from_yaml(yaml).expect("rules should load");
        let invoice = base_invoice(VatCategory::Vat0, Some("article_99"));
        let mut report = ValidationReport::new();

        engine.apply(&invoice, &mut report);

        assert_eq!(report.errors.len(), 1);
        assert_eq!(report.errors[0].code, "FIELD-001");
    }

    #[test]
    fn header_conditional_required_reports_missing_correlated_invoice() {
        let yaml = r#"
version: "test"
rules:
  - id: "CRN-001"
    description: "Credit invoices require correlation"
    severity: "Error"
    logic:
      type: "HeaderConditionalRequired"
      when_field: "invoice_type"
      when_values: ["5.1"]
      required_field: "correlated_invoices"
    error_message: "missing correlation"
"#;

        let engine = RulesEngine::load_from_yaml(yaml).expect("rules should load");
        let mut invoice = base_invoice(VatCategory::Vat24, None);
        invoice.header.invoice_type = InvoiceType::CreditNote;
        invoice.header.correlated_invoices = None;
        let mut report = ValidationReport::new();

        engine.apply(&invoice, &mut report);

        assert_eq!(report.errors.len(), 1);
        assert_eq!(report.errors[0].code, "CRN-001");
    }

    #[test]
    fn payment_method_value_allowed_reports_invalid_type() {
        let yaml = r#"
version: "test"
rules:
  - id: "PAY-001"
    description: "Payment method type must be valid"
    severity: "Error"
    logic:
      type: "PaymentMethodValueAllowed"
      field_path: "type"
      allowed_values: ["1", "3", "5", "7"]
    error_message: "invalid payment method on line {line}"
"#;

        let engine = RulesEngine::load_from_yaml(yaml).expect("rules should load");
        let mut invoice = base_invoice(VatCategory::Vat24, None);
        invoice.payment_methods.push(PaymentMethodDetail {
            r#type: 99,
            amount: dec!(124),
            payment_method_info: None,
        });
        let mut report = ValidationReport::new();

        engine.apply(&invoice, &mut report);

        assert_eq!(report.errors.len(), 1);
        assert_eq!(report.errors[0].code, "PAY-001");
    }
}
