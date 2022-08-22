use crate::balance::model::Balance;
use crate::income::model::Income;

use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct PerShareRatios {
    pub stock_id: i32,
    pub year: i32,
    pub cash_equity: f32,
    pub quick_equity: f32,
    pub current_equity: f32,
    pub tangible_equity: f32,
    pub gross_profit: f32,
    pub operating_profit: f32,
    pub net_profit: f32,
    pub cashflow: f32,
}

impl PerShareRatios {
    pub fn new(balance: Balance, income: Income) -> Self{
        Self { 
            stock_id: balance.stock_id, 
            year: balance.year, 
            cash_equity: balance.net_cash_asset as f32 / balance.share_outstanding as f32, 
            quick_equity: balance.net_quick_asset as f32 / balance.share_outstanding as f32, 
            current_equity: balance.net_current_asset as f32 / balance.share_outstanding as f32, 
            tangible_equity: balance.net_tangible_asset as f32 / balance.share_outstanding as f32, 
            gross_profit: income.gross_profit as f32 / balance.share_outstanding as f32, 
            operating_profit: income.operating_profit as f32 / balance.share_outstanding as f32, 
            net_profit: income.net_profit as f32 / balance.share_outstanding as f32, 
            cashflow: income.total_cashflow as f32 / balance.share_outstanding as f32
        }
    }
}
