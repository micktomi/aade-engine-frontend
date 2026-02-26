use serde::{Deserialize, Serialize};
use std::fmt;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InvoiceType {
    #[serde(rename = "1.1")]
    SalesInvoice, // Τιμολόγιο Πώλησης
    #[serde(rename = "1.2")]
    SalesInvoiceIntra, // Τιμολόγιο Πώλησης / Ενδοκοινοτικές Παραδόσεις
    #[serde(rename = "1.3")]
    SalesInvoiceThirdCountry, // Τιμολόγιο Πώλησης / Τρίτες Χώρες
    #[serde(rename = "1.4")]
    SalesInvoiceRelated, // Τιμολόγιο Πώλησης / Συνδεδεμένη
    #[serde(rename = "1.5")]
    SalesInvoiceRetail, // Τιμολόγιο Πώλησης / Δ.Λ.
    #[serde(rename = "1.6")]
    SalesInvoiceForeignVAT, // Τιμολόγιο Πώλησης / Ξένο ΦΠΑ
    #[serde(rename = "2.1")]
    ServiceInvoice, // Τιμολόγιο Παροχής Υπηρεσιών
    #[serde(rename = "2.2")]
    ServiceInvoiceIntra, // Τιμολόγιο Παροχής / Ενδοκοινοτική
    #[serde(rename = "2.3")]
    ServiceInvoiceThirdCountry, // Τιμολόγιο Παροχής / Τρίτες Χώρες
    #[serde(rename = "2.4")]
    ServiceInvoiceRelated, // Τιμολόγιο Παροχής / Συνδεδεμένη
    #[serde(rename = "5.1")]
    CreditNote, // Πιστωτικό Τιμολόγιο
    #[serde(rename = "5.2")]
    DebitNote, // Χρεωστικό Τιμολόγιο
    #[serde(rename = "11.1")]
    RetailReceipt, // ΑΛΠ - Απόδειξη Λιανικής Πώλησης
    #[serde(rename = "11.2")]
    ServiceReceipt, // ΑΠΥ - Απόδειξη Παροχής Υπηρεσιών
    #[serde(rename = "11.3")]
    SimplifiedInvoice, // Απλοποιημένο Τιμολόγιο
    #[serde(rename = "11.4")]
    RetailCreditNote, // Πιστωτικό Στοιχείο Λιανικής
    #[serde(rename = "11.5")]
    ServiceCreditNote, // Πιστωτικό Στοιχείο Παροχής
    #[default]
    Unknown,
}

impl fmt::Display for InvoiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvoiceType::SalesInvoice => write!(f, "1.1"),
            InvoiceType::SalesInvoiceIntra => write!(f, "1.2"),
            InvoiceType::SalesInvoiceThirdCountry => write!(f, "1.3"),
            InvoiceType::SalesInvoiceRelated => write!(f, "1.4"),
            InvoiceType::SalesInvoiceRetail => write!(f, "1.5"),
            InvoiceType::SalesInvoiceForeignVAT => write!(f, "1.6"),
            InvoiceType::ServiceInvoice => write!(f, "2.1"),
            InvoiceType::ServiceInvoiceIntra => write!(f, "2.2"),
            InvoiceType::ServiceInvoiceThirdCountry => write!(f, "2.3"),
            InvoiceType::ServiceInvoiceRelated => write!(f, "2.4"),
            InvoiceType::CreditNote => write!(f, "5.1"),
            InvoiceType::DebitNote => write!(f, "5.2"),
            InvoiceType::RetailReceipt => write!(f, "11.1"),
            InvoiceType::ServiceReceipt => write!(f, "11.2"),
            InvoiceType::SimplifiedInvoice => write!(f, "11.3"),
            InvoiceType::RetailCreditNote => write!(f, "11.4"),
            InvoiceType::ServiceCreditNote => write!(f, "11.5"),
            InvoiceType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum VatCategory {
    #[serde(rename = "1")]
    #[default]
    Vat24,
    #[serde(rename = "2")]
    Vat13,
    #[serde(rename = "3")]
    Vat6,
    #[serde(rename = "4")]
    Vat17,
    #[serde(rename = "5")]
    Vat9,
    #[serde(rename = "6")]
    Vat4,
    #[serde(rename = "7")]
    Vat0, // 0%
    #[serde(rename = "8")]
    Excluded, // Άνευ ΦΠΑ
}

impl VatCategory {
    pub fn rate(&self) -> Decimal {
        match self {
            VatCategory::Vat24 => dec!(0.24),
            VatCategory::Vat13 => dec!(0.13),
            VatCategory::Vat6 => dec!(0.06),
            VatCategory::Vat17 => dec!(0.17),
            VatCategory::Vat9 => dec!(0.09),
            VatCategory::Vat4 => dec!(0.04),
            VatCategory::Vat0 | VatCategory::Excluded => dec!(0.00),
        }
    }
}

impl fmt::Display for VatCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VatCategory::Vat24 => write!(f, "1"),
            VatCategory::Vat13 => write!(f, "2"),
            VatCategory::Vat6 => write!(f, "3"),
            VatCategory::Vat17 => write!(f, "4"),
            VatCategory::Vat9 => write!(f, "5"),
            VatCategory::Vat4 => write!(f, "6"),
            VatCategory::Vat0 => write!(f, "7"),
            VatCategory::Excluded => write!(f, "8"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    #[serde(rename = "1")]
    Cash, // Επαγγ. Λογαριασμός Πληρωμών Ημεδαπής
    #[serde(rename = "3")]
    BankCheck, // Επιταγή
    #[serde(rename = "5")]
    Credit, // Επί Πιστώσει
    #[serde(rename = "7")]
    Pos, // POS / e-POS
}

impl fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaymentMethod::Cash => write!(f, "1"),
            PaymentMethod::BankCheck => write!(f, "3"),
            PaymentMethod::Credit => write!(f, "5"),
            PaymentMethod::Pos => write!(f, "7"),
        }
    }
}
