use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct AddBalanceReq {
    pub code: String,
    pub year: i32,
    pub cash: i64,
    pub receivables: i64,
    pub inventories: i64,
    pub fixed_asset: i64,
    pub st_liabilities: i64,
    pub lt_liabilities: i64,
    pub share_outstanding: i64,
}