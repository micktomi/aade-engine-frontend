use serde::{Deserialize, Serialize};
use super::enums::VatCategory;
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VatBreakdown {
    pub category: VatCategory,
    pub net_amount: Decimal,
    pub vat_amount: Decimal,
}

impl VatBreakdown {
    #[allow(dead_code)]
    pub fn new(category: VatCategory, net: Decimal, vat: Decimal) -> Self {
        Self {
            category,
            net_amount: net,
            vat_amount: vat,
        }
    }
}
