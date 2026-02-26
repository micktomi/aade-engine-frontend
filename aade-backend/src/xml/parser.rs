use serde::{Deserialize, Serialize};

// Intermediate structs that map directly to AADE XML structure

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")] // Default strategy, specific overrides below
pub struct AadeBook {
    #[serde(rename = "invoice", default)]
    pub invoices: Vec<XmlInvoice>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XmlInvoice {
    // Header
    pub issuer: Option<XmlIssuer>,
    pub counterpart: Option<XmlCounterpart>,
    
    #[serde(rename = "invoiceHeader")]
    pub invoice_header: XmlInvoiceHeader,
    
    #[serde(rename = "paymentMethods")]
    pub payment_methods: Option<XmlPaymentMethods>,
    
    #[serde(rename = "invoiceDetails")]
    pub invoice_details: Vec<XmlInvoiceRow>,
    
    #[serde(rename = "invoiceSummary")]
    pub invoice_summary: XmlInvoiceSummary,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XmlIssuer {
    #[serde(rename = "vatNumber")]
    pub vat_number: String,
    pub country: String,
    pub branch: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XmlCounterpart {
    #[serde(rename = "vatNumber")]
    pub vat_number: Option<String>,
    pub country: Option<String>,
    pub branch: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XmlInvoiceHeader {
    pub series: String,
    pub aa: String,
    #[serde(rename = "issueDate")]
    pub issue_date: String, // YYYY-MM-DD
    #[serde(rename = "invoiceType")]
    pub invoice_type: String, // e.g., "1.1"
    pub currency: Option<String>,
    #[serde(rename = "exchangeRate")]
    pub exchange_rate: Option<f64>,
    #[serde(rename = "correlatedInvoices")]
    pub correlated_invoices: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XmlInvoiceRow {
    #[serde(rename = "lineNumber")]
    pub line_number: i32,
    #[serde(rename = "netValue")]
    pub net_value: f64, 
    #[serde(rename = "vatCategory")]
    pub vat_category: i32,
    #[serde(rename = "vatAmount")]
    pub vat_amount: f64,
    #[serde(rename = "vatExemptionCause", alias = "vatExemptionCategory")]
    pub vat_exemption_cause: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XmlInvoiceSummary {
    #[serde(rename = "totalNetValue")]
    pub total_net_value: f64,
    #[serde(rename = "totalVatAmount")]
    pub total_vat_amount: f64,
    #[serde(rename = "totalWithheldAmount")]
    pub total_withheld_amount: f64,
    #[serde(rename = "totalFeesAmount")]
    pub total_fees_amount: f64,
    #[serde(rename = "totalStampDutyAmount")]
    pub total_stamp_duty_amount: f64,
    #[serde(rename = "totalDeductionsAmount")]
    pub total_deductions_amount: f64,
    #[serde(rename = "totalGrossValue")]
    pub total_gross_value: f64,
    #[serde(rename = "incomeClassification")]
    pub income_classification: Option<Vec<XmlIncomeClassification>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XmlIncomeClassification {
    #[serde(rename = "classificationType")]
    pub classification_type: Option<String>,
    #[serde(rename = "classificationCategory")]
    pub classification_category: Option<String>,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XmlPaymentMethods {
    #[serde(rename = "paymentMethodDetails")]
    pub payment_method_details: Vec<XmlPaymentMethodDetail>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XmlPaymentMethodDetail {
    pub r#type: i32,
    pub amount: f64,
    #[serde(rename = "paymentMethodInfo")]
    pub payment_method_info: Option<String>,
}
