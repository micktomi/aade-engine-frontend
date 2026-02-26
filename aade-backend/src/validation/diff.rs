use crate::domain::invoice::Invoice;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldChange {
    pub path: String,
    pub old_value: String,
    pub new_value: String,
}

impl FieldChange {
    fn new(path: &str, old: String, new: String) -> Self {
        Self {
            path: path.to_string(),
            old_value: old,
            new_value: new,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceDiff {
    pub has_changes: bool,
    pub changes: Vec<FieldChange>,
}

impl InvoiceDiff {
    pub fn empty() -> Self {
        Self {
            has_changes: false,
            changes: vec![],
        }
    }
}

pub fn compare(old: &Invoice, new: &Invoice) -> InvoiceDiff {
    let mut changes = Vec::new();

    // 1. Header Comparison
    if old.header.series != new.header.series {
        changes.push(FieldChange::new("header.series", old.header.series.clone(), new.header.series.clone()));
    }
    if old.header.aa != new.header.aa {
        changes.push(FieldChange::new("header.aa", old.header.aa.clone(), new.header.aa.clone()));
    }
    if old.header.issue_date != new.header.issue_date {
        changes.push(FieldChange::new("header.issue_date", old.header.issue_date.to_string(), new.header.issue_date.to_string()));
    }
    if old.header.invoice_type != new.header.invoice_type {
        changes.push(FieldChange::new("header.invoice_type", old.header.invoice_type.to_string(), new.header.invoice_type.to_string()));
    }
    if old.header.correlated_invoices != new.header.correlated_invoices {
        changes.push(FieldChange::new(
            "header.correlated_invoices",
            old.header.correlated_invoices.as_deref().unwrap_or("").to_string(),
            new.header.correlated_invoices.as_deref().unwrap_or("").to_string(),
        ));
    }

    // 2. Issuer/Counterpart (High Level)
    if old.issuer.vat_number != new.issuer.vat_number {
        changes.push(FieldChange::new("issuer.vat_number", old.issuer.vat_number.clone(), new.issuer.vat_number.clone()));
    }
    if let (Some(old_cp), Some(new_cp)) = (&old.counterpart, &new.counterpart) {
        if old_cp.vat_number != new_cp.vat_number {
            changes.push(FieldChange::new("counterpart.vat_number", old_cp.vat_number.clone(), new_cp.vat_number.clone()));
        }
    } else if old.counterpart.is_some() != new.counterpart.is_some() {
        changes.push(FieldChange::new("counterpart", 
            if old.counterpart.is_some() { "Present".into() } else { "None".into() },
            if new.counterpart.is_some() { "Present".into() } else { "None".into() }
        ));
    }

    // 3. Totals Comparison
    if old.totals.total_net_amount != new.totals.total_net_amount {
        changes.push(FieldChange::new("totals.net_amount", old.totals.total_net_amount.to_string(), new.totals.total_net_amount.to_string()));
    }
    if old.totals.total_vat_amount != new.totals.total_vat_amount {
        changes.push(FieldChange::new("totals.vat_amount", old.totals.total_vat_amount.to_string(), new.totals.total_vat_amount.to_string()));
    }
    if old.totals.total_gross_amount != new.totals.total_gross_amount {
        changes.push(FieldChange::new("totals.gross_amount", old.totals.total_gross_amount.to_string(), new.totals.total_gross_amount.to_string()));
    }

    // 3b. Payment Methods Comparison
    let old_payment_types: Vec<String> = old.payment_methods.iter().map(|p| p.r#type.to_string()).collect();
    let new_payment_types: Vec<String> = new.payment_methods.iter().map(|p| p.r#type.to_string()).collect();
    if old_payment_types != new_payment_types {
        changes.push(FieldChange::new(
            "paymentMethods.paymentMethodDetails.type",
            old_payment_types.join(","),
            new_payment_types.join(","),
        ));
    }

    // 4. Smart Line Comparison (Match by line_number)
    let old_lines: HashMap<i32, &crate::domain::invoice::InvoiceLine> = old.lines.iter().map(|l| (l.line_number, l)).collect();
    let new_lines: HashMap<i32, &crate::domain::invoice::InvoiceLine> = new.lines.iter().map(|l| (l.line_number, l)).collect();

    // Check for modifications and deletions
    for (num, old_line) in &old_lines {
        match new_lines.get(num) {
            Some(new_line) => {
                // Compare critical line fields
                if old_line.net_value != new_line.net_value {
                    changes.push(FieldChange::new(
                        &format!("line[{}].net_value", num),
                        old_line.net_value.to_string(),
                        new_line.net_value.to_string()
                    ));
                }
                if old_line.vat_category != new_line.vat_category {
                    changes.push(FieldChange::new(
                        &format!("line[{}].vat_category", num),
                        old_line.vat_category.to_string(),
                        new_line.vat_category.to_string()
                    ));
                }
                if old_line.vat_exemption_cause != new_line.vat_exemption_cause {
                    changes.push(FieldChange::new(
                        &format!("line[{}].vat_exemption_cause", num),
                        old_line.vat_exemption_cause.as_deref().unwrap_or("").to_string(),
                        new_line.vat_exemption_cause.as_deref().unwrap_or("").to_string()
                    ));
                }
            },
            None => {
                changes.push(FieldChange::new(
                    &format!("line[{}]", num),
                    "Exists".into(),
                    "Deleted".into()
                ));
            }
        }
    }

    // Check for additions
    for num in new_lines.keys() {
        if !old_lines.contains_key(num) {
             changes.push(FieldChange::new(
                    &format!("line[{}]", num),
                    "None".into(),
                    "Created".into()
                ));
        }
    }

    InvoiceDiff {
        has_changes: !changes.is_empty(),
        changes,
    }
}
