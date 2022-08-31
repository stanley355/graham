use crate::analysis::{analysis_count::AnalysisCount, model::Analysis};
use crate::ratios::model::Ratios;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceAnalysis {
    pub stock_id: i32,
    pub year: i32,
    pub asset_price: f32,
    pub eps: f32,
    pub price_limit: f32,
    pub safety_price_limit: f32,
    pub analysis_count: AnalysisCount,
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
