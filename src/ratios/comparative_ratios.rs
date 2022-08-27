use crate::report::model::Report;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    pub fn new(report: Report) -> Self {
        Self {
            stock_id: report.stock_id,
            year: report.year,
            gross_profit_margin: (report.gross_profit as f32 / report.revenue as f32) * 100.0,
            operating_profit_margin: (report.operating_profit as f32 / report.revenue as f32)
                * 100.0,
            net_profit_margin: (report.net_profit as f32 / report.revenue as f32) * 100.0,
            current_asset_return: (report.net_profit as f32 / report.net_current_asset as f32)
                * 100.0,
            tang_asset_return: (report.net_profit as f32 / report.net_tangible_asset as f32)
                * 100.0,
            total_liability_return: (report.net_profit as f32 / report.total_liabilities as f32)
                * 100.0,
            revenue_receivable_return: (report.revenue as f32 / report.receivables as f32) * 100.0,
            revenue_inventory_return: (report.revenue as f32 / report.inventories as f32) * 100.0,
            current_asset_liabilities_return: (report.current_asset as f32
                / report.st_liabilities as f32)
                * 100.0,
            tang_asset_total_liabilities_return: (report.tangible_asset as f32
                / report.total_liabilities as f32)
                * 100.0,
        }
    }
}
