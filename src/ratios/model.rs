use crate::ratios::{comparative_ratios::ComparativeRatios, per_share_ratios::PerShareRatios};
use crate::report::model::Report;
use crate::traits::report_response::ReportHttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ratios {
    pub capitalization: f32,
    pub per_share_ratios: PerShareRatios,
    pub comparative_ratios: ComparativeRatios,
}

impl ReportHttpResponse for Ratios {}

impl Ratios {
    pub fn create(report: Report) -> Self {
        let ps_ratios = PerShareRatios::new(report.clone());
        let compar_ratios = ComparativeRatios::new(report.clone());

        Self {
            capitalization: (report.share_outstanding as f32 * ps_ratios.net_profit) * 100.0,
            per_share_ratios: ps_ratios,
            comparative_ratios: compar_ratios,
        }
    }

    pub fn create_list(reports: Vec<Report>) -> Vec<Self> {
        reports.into_iter().map(|r| Ratios::create(r)).collect()
    }
}
