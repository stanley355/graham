use crate::schema::income;
use diesel::{AsChangeset, Queryable};
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

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, AsChangeset)]
#[table_name = "income"]
pub struct UpdateIncomeReq {
    pub stock_id: i32,
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
