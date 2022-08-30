use crate::analysis::{analysis_count::AnalysisCount, model::Analysis};
use crate::ratios::model::Ratios;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceAnalysis {
    stock_id: i32,
    year: i32,
    asset_price: f32,
    eps: f32,
    price_limit: f32,
    safety_price_limit: f32,
    analysis_count: AnalysisCount,
}

impl PriceAnalysis {
    pub fn new(ratios: &Ratios, analysis: &Analysis) -> Self {
        Self {
            stock_id: analysis.stock_id,
            year: analysis.year,
            asset_price: ratios.per_share_ratios.tangible_equity,
            eps: ratios.per_share_ratios.net_profit,
            price_limit: PriceAnalysis::count_price_limit(&ratios, &analysis),
            safety_price_limit: PriceAnalysis::count_safety_price_limit(&ratios, &analysis),
            analysis_count: AnalysisCount::new(analysis),
        }
    }

    pub fn new_list(ratios: Vec<Ratios>, analysis: Vec<Analysis>) -> Vec<Self> {
        let mut mos_vec = Vec::new();
        let mut i = 0;

        while i < ratios.len() {
            let mos = PriceAnalysis::new(&ratios[i], &analysis[i]);
            mos_vec.push(mos);
            i += 1;
        }

        mos_vec
    }

    pub fn count_price_limit(ratios: &Ratios, analysis: &Analysis) -> f32 {
        let analysis_count = AnalysisCount::new(analysis);
        ratios.per_share_ratios.net_profit
            * (analysis_count.wonderful as f32 + analysis_count.pass as f32)
    }

    pub fn count_safety_price_limit(ratios: &Ratios, analysis: &Analysis) -> f32 {
        let price_limit = PriceAnalysis::count_price_limit(ratios, analysis);

        price_limit * (3.0 / 4.0)
    }
}
