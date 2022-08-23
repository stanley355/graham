use crate::balance::model::Balance;
use crate::income::model::Income;

use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct ComparativeRatios {
    stock_id: i32,
    year: i32,
    gross_profit_margin: f32,
    operating_profit_margin: f32,
    net_profit_margin: f32,
    current_asset_return: f32,
    tang_asset_return: f32,
    total_liability_return: f32,
    revenue_receivable_return: f32,
    revenue_inventory_return: f32,
    current_asset_liabilities_return: f32,
    tang_asset_total_liabilities_return: f32,
}

impl ComparativeRatios {
    pub fn new(balance: Balance, income: Income) -> Self {
        Self {
            stock_id: balance.stock_id,
            year: balance.year,
            gross_profit_margin: (income.gross_profit as f32 / income.revenue as f32) * 100.0,
            operating_profit_margin: (income.operating_profit as f32 / income.revenue as f32)
                * 100.0,
            net_profit_margin: (income.net_profit as f32 / income.revenue as f32) * 100.0,
            current_asset_return: (income.net_profit as f32 / balance.net_current_asset as f32)
                * 100.0,
            tang_asset_return: (income.net_profit as f32 / balance.net_tangible_asset as f32)
                * 100.0,
            total_liability_return: (income.net_profit as f32 / balance.total_liabilities as f32)
                * 100.0,
            revenue_receivable_return: (income.revenue as f32 / balance.receivables as f32) * 100.0,
            revenue_inventory_return: (income.revenue as f32 / balance.inventories as f32) * 100.0,
            current_asset_liabilities_return: (balance.current_asset as f32
                / balance.st_liabilities as f32)
                * 100.0,
            tang_asset_total_liabilities_return: (balance.tangible_asset as f32
                / balance.total_liabilities as f32)
                * 100.0,
        }
    }
}
