use crate::report::model::Report;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct GrowthRatios {
    cast_growth: f32,
    st_liability_growth: f32,
    lt_liability_growth: f32,
    total_liability_growth: f32,
    net_profit_growth: f32,
}

impl GrowthRatios {
    pub fn create_yearly(reports: Vec<Report>) -> Vec<GrowthRatios> {
        let mut yearly_growth: Vec<GrowthRatios> = Vec::new();
        for (index, rep) in reports.iter().enumerate() {
            if index == (&reports.len() - 2) {
                break;
            }

            let new_ratio = GrowthRatios::create_ratios(&rep, &reports[index - 1]);
            yearly_growth.push(new_ratio);
        }

        yearly_growth
    }

    pub fn create_ratios(current_report: &Report, next_report: &Report) -> Self {
        Self {
            cast_growth: GrowthRatios::count_growth(current_report.cash, next_report.cash),
            st_liability_growth: GrowthRatios::count_growth(
                current_report.st_liabilities,
                next_report.st_liabilities,
            ),
            lt_liability_growth: GrowthRatios::count_growth(
                current_report.lt_liabilities,
                next_report.lt_liabilities,
            ),
            total_liability_growth: GrowthRatios::count_growth(
                current_report.total_liabilities,
                next_report.total_liabilities,
            ),
            net_profit_growth: GrowthRatios::count_growth(
                current_report.net_profit,
                next_report.net_profit,
            ),
        }
    }

    pub fn count_growth(current_number: i64, next_number: i64) -> f32 {
        ((current_number as f32 - next_number as f32) / next_number as f32) * 100.0
    }
}
