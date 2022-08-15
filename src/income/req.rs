use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct AddIncomeReq {
    pub code: String,
    pub year: i32,
    pub revenue: i64,
    pub gross_profit: i64,
    pub operating_profit: i64,
    pub net_profit: i64,
    pub customer_cashflow: i64,
    pub operating_cashflow: i64,
    pub investing_cashflow: i64,
    pub financing_cashflow: i64,
}