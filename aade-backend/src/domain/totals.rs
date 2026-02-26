use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceTotals {
    pub total_net_amount: Decimal,
    pub total_vat_amount: Decimal,
    pub total_withheld_amount: Decimal,
    pub total_fees_amount: Decimal,
    pub total_stamp_duty_amount: Decimal,
    pub total_deductions_amount: Decimal,
    pub total_gross_amount: Decimal,
}
