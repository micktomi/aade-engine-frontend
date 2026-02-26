use crate::domain::invoice::{Counterpart, Invoice, InvoiceHeader, InvoiceLine, Issuer, PaymentMethodDetail};
use crate::domain::totals::InvoiceTotals;
use crate::domain::enums::{InvoiceType, VatCategory};
use crate::domain::classification::IncomeClassification;
use super::parser::XmlInvoice;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use chrono::NaiveDate;

// The Normalizer converts XmlInvoice (messy) -> Invoice (clean domain)

pub struct Normalizer;

impl Normalizer {
    pub fn normalize(xml_invoice: XmlInvoice) -> Result<Invoice, String> {
        // 1. Header
        let issue_date = NaiveDate::parse_from_str(&xml_invoice.invoice_header.issue_date, "%Y-%m-%d")
            .map_err(|e| format!("Invalid date format: {}", e))?;
        
        // Enum conversion
        let inv_type_str = xml_invoice.invoice_header.invoice_type.as_str();
        let invoice_type = serde_json::from_value(serde_json::json!(inv_type_str))
            .unwrap_or(InvoiceType::Unknown); // Simplified for now

        let header = InvoiceHeader {
            series: xml_invoice.invoice_header.series,
            aa: xml_invoice.invoice_header.aa,
            issue_date,
            issue_time: None, // Parse if exists
            invoice_type,
            currency: xml_invoice.invoice_header.currency.unwrap_or_else(|| "EUR".to_string()),
            exchange_rate: xml_invoice.invoice_header.exchange_rate
                .and_then(Decimal::from_f64),
            correlated_invoices: xml_invoice.invoice_header.correlated_invoices
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty()),
        };

        // 2. Issuer
        let issuer_xml = xml_invoice.issuer.ok_or("Missing issuer")?;
        let issuer = Issuer {
            vat_number: issuer_xml.vat_number,
            country: issuer_xml.country,
            branch: issuer_xml.branch,
        };

        // 3. Counterpart
        let counterpart = if let Some(cp) = xml_invoice.counterpart {
            let vat_number = cp.vat_number.ok_or("Counterpart missing vatNumber")?;
            let country = cp.country.ok_or("Counterpart missing country")?;
            let branch = cp.branch.ok_or("Counterpart missing branch")?;

            Some(Counterpart {
                vat_number,
                country,
                branch,
                name: cp.name,
            })
        } else {
            None
        };

        // 4. Lines
        let mut lines = Vec::new();
        for row in xml_invoice.invoice_details {
             let net = Decimal::from_f64(row.net_value)
                .ok_or_else(|| format!("Invalid net value on line {}", row.line_number))?;
             let vat_amt = Decimal::from_f64(row.vat_amount)
                .ok_or_else(|| format!("Invalid vat amount on line {}", row.line_number))?;

             // Map int to Enum
             let vat_cat_str = row.vat_category.to_string();
             let vat_cat: VatCategory = serde_json::from_value(serde_json::json!(vat_cat_str))
                .map_err(|_| format!("Invalid VAT category {} on line {}", row.vat_category, row.line_number))?;

             lines.push(InvoiceLine {
                 line_number: row.line_number,
                 description: "From XML".to_string(),
                 net_value: net,
                 vat_category: vat_cat,
                 vat_amount: vat_amt,
                 vat_exemption_cause: row.vat_exemption_cause
                    .map(|v| v.trim().to_string())
                    .filter(|v| !v.is_empty()),
                 quantity: None,
                 measurement_unit: None,
             });
        }

        // 5. Payment Methods
        let mut payment_methods = Vec::new();
        if let Some(xml_payment_methods) = xml_invoice.payment_methods {
            for detail in xml_payment_methods.payment_method_details {
                let amount = Decimal::from_f64(detail.amount)
                    .ok_or_else(|| format!("Invalid payment method amount for type {}", detail.r#type))?;

                payment_methods.push(PaymentMethodDetail {
                    r#type: detail.r#type,
                    amount,
                    payment_method_info: detail.payment_method_info
                        .map(|v| v.trim().to_string())
                        .filter(|v| !v.is_empty()),
                });
            }
        }

        // 6. Totals
        let sum = xml_invoice.invoice_summary;
        let totals = InvoiceTotals {
            total_net_amount: Decimal::from_f64(sum.total_net_value)
                .ok_or("Invalid total_net_value")?,
            total_vat_amount: Decimal::from_f64(sum.total_vat_amount)
                .ok_or("Invalid total_vat_amount")?,
            total_withheld_amount: Decimal::from_f64(sum.total_withheld_amount)
                .unwrap_or_default(),
            total_fees_amount: Decimal::from_f64(sum.total_fees_amount)
                .unwrap_or_default(),
            total_stamp_duty_amount: Decimal::from_f64(sum.total_stamp_duty_amount)
                .unwrap_or_default(),
            total_deductions_amount: Decimal::from_f64(sum.total_deductions_amount)
                .unwrap_or_default(),
            total_gross_amount: Decimal::from_f64(sum.total_gross_value)
                .ok_or("Invalid total_gross_value")?,
        };

        // 7. Classifications
        let mut income_classifications = Vec::new();
        if let Some(xml_classifications) = sum.income_classification {
            for item in xml_classifications {
                let amount = Decimal::from_f64(item.amount)
                    .ok_or_else(|| format!(
                        "Invalid classification amount for type {:?} category {:?}",
                        item.classification_type, item.classification_category
                    ))?;

                income_classifications.push(IncomeClassification {
                    classification_type: item.classification_type,
                    classification_category: item.classification_category,
                    amount,
                });
            }
        }

        Ok(Invoice {
            uid: None,
            header,
            issuer,
            counterpart,
            lines,
            payment_methods,
            totals,
            vat_breakdown: vec![], // To be calculated or extracted if detailed
            income_classifications,
        })
    }
}
