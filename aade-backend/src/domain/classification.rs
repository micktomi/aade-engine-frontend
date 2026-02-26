use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeClassification {
    #[serde(rename = "classificationType")]
    pub classification_type: Option<String>,
    #[serde(rename = "classificationCategory")]
    pub classification_category: Option<String>,
    pub amount: Decimal,
}
