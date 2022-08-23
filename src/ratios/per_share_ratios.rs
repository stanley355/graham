use crate::report::model::Report;
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
    pub fn new(report: Report) -> Self{
        Self { 
            stock_id: report.stock_id, 
            year: report.year, 
            cash_equity: report.net_cash_asset as f32 / report.share_outstanding as f32, 
            quick_equity: report.net_quick_asset as f32 / report.share_outstanding as f32, 
            current_equity: report.net_current_asset as f32 / report.share_outstanding as f32, 
            tangible_equity: report.net_tangible_asset as f32 / report.share_outstanding as f32, 
            gross_profit: report.gross_profit as f32 / report.share_outstanding as f32, 
            operating_profit: report.operating_profit as f32 / report.share_outstanding as f32, 
            net_profit: report.net_profit as f32 / report.share_outstanding as f32, 
            cashflow: report.total_cashflow as f32 / report.share_outstanding as f32
        }
    }
}
