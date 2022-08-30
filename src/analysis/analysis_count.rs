use crate::analysis::model::{Analysis, AnalysisStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnalysisCount {
    wonderful: i32,
    pass: i32,
    mediocre: i32,
    fail: i32,
}

impl AnalysisCount {
    pub fn new(analysis: &Analysis) -> Self {
        let analysis_array = AnalysisCount::create_analysis_array(&analysis);

        let mut count = AnalysisCount {
            wonderful: 0,
            pass: 0,
            mediocre: 0,
            fail: 0,
        };

        for analysis in analysis_array {
            match analysis {
                AnalysisStatus::Wonderful => count.wonderful += 1,
                AnalysisStatus::Pass => count.pass += 1,
                AnalysisStatus::Mediocre => count.mediocre += 1,
                AnalysisStatus::Fail => count.fail += 1,
            }
        }

        return count;
    }

    pub fn create_analysis_array(analysis: &Analysis) -> Vec<AnalysisStatus> {
        vec![
            analysis.no_minus_balance,
            analysis.no_minus_income,
            analysis.healthy_cashflow,
            analysis.curr_asset_vs_st_liability,
            analysis.fixed_asset_vs_lt_liability,
            analysis.revenue_receivable_ratio,
            analysis.revenue_inventory_ratio,
            analysis.gross_profit_margin,
            analysis.operating_profit_margin,
            analysis.net_profit_margin,
            analysis.curr_asset_return,
            analysis.tangible_asset_return,
            analysis.liability_return,
        ]
    }
}
