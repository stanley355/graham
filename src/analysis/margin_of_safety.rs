use crate::analysis::{analysis_count::AnalysisCount, model::Analysis, model::AnalysisStatus};
use crate::ratios::model::Ratios;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarginOfSafety {
    asset_price: f32,
    eps: f32,
    price_limit: f32,
    safety_price_limit: f32,
    analysis_count: AnalysisCount,
}

impl MarginOfSafety {
    pub fn new(ratios: &Ratios, analysis: &Analysis) -> Self {
        Self {
            asset_price: ratios.per_share_ratios.tangible_equity,
            eps: ratios.per_share_ratios.net_profit,
            price_limit: MarginOfSafety::count_price_limit(&ratios, &analysis),
            safety_price_limit: MarginOfSafety::count_safety_price_limit(&ratios, &analysis),
            analysis_count: AnalysisCount::new(analysis),
        }
    }

    pub fn new_list(ratios: Vec<Ratios>, analysis: Vec<Analysis>) -> Vec<Self> {
        let mut mos_vec = Vec::new();
        let mut i = 0;

        while i < ratios.len() {
            let mos = MarginOfSafety::new(&ratios[i], &analysis[i]);
            mos_vec.push(mos);
            i += 1;
        }

        mos_vec
    }

    pub fn count_price_limit(ratios: &Ratios, analysis: &Analysis) -> f32 {
        let liability_return =
            Analysis::check_liability_return(ratios.comparative_ratios.total_liability_return);

        match liability_return {
            AnalysisStatus::Wonderful => ratios.per_share_ratios.net_profit * 11.0,
            _ => ratios.per_share_ratios.net_profit * 10.0,
        }
    }

    pub fn count_safety_price_limit(ratios: &Ratios, analysis: &Analysis) -> f32 {
        let price_limit = MarginOfSafety::count_price_limit(ratios, analysis);

        price_limit * (3.0 / 4.0)
    }
}
