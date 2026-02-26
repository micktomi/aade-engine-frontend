use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveTime};
use rust_decimal::Decimal;
use super::enums::{InvoiceType, VatCategory};
use super::totals::InvoiceTotals;
use super::vat::VatBreakdown;
use super::classification::IncomeClassification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceHeader {
    pub series: String,
    pub aa: String,
    pub issue_date: NaiveDate,
    pub issue_time: Option<NaiveTime>, // Optional in strict XML sometimes
    pub invoice_type: InvoiceType,
    pub currency: String,
    // Exchange rate might be needed for non-EUR
    pub exchange_rate: Option<Decimal>,
    // Correlation/reference to original invoice (important for credit notes)
    pub correlated_invoices: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issuer {
    pub vat_number: String,
    pub country: String,
    pub branch: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counterpart {
    pub vat_number: String,
    pub country: String,
    pub branch: i32,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLine {
    pub line_number: i32,
    pub description: String, // Or product description
    pub net_value: Decimal,
    pub vat_category: VatCategory,
    pub vat_amount: Decimal,
    pub vat_exemption_cause: Option<String>, // Required when VAT category is 0% in many cases
    pub quantity: Option<Decimal>,
    pub measurement_unit: Option<i32>, // Enum eventually
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetail {
    pub r#type: i32,
    pub amount: Decimal,
    pub payment_method_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub uid: Option<String>, // Generated internally for tracking
    pub header: InvoiceHeader,
    pub issuer: Issuer,
    pub counterpart: Option<Counterpart>, // Optional for B2C retail sometimes
    pub lines: Vec<InvoiceLine>,
    pub payment_methods: Vec<PaymentMethodDetail>,
    pub totals: InvoiceTotals,
    pub vat_breakdown: Vec<VatBreakdown>,
    pub income_classifications: Vec<IncomeClassification>,
}
