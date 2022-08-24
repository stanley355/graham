use crate::ratios::{comparative_ratios::ComparativeRatios, per_share_ratios::PerShareRatios};
use crate::report::model::Report;

use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Ratios {
    pub per_share_ratios: PerShareRatios,
    pub comparative_ratios: ComparativeRatios,
}

impl Ratios {
    pub fn create(report: Report) -> Self {
        let ps_ratios = PerShareRatios::new(report.clone());
        let compar_ratios = ComparativeRatios::new(report);

        Self {
            per_share_ratios: ps_ratios,
            comparative_ratios: compar_ratios,
        }
    }

    pub fn create_list(reports: Vec<Report>) -> Vec<Self> {
        reports.into_iter().map(|r| Ratios::create(r)).collect()
    }
}
