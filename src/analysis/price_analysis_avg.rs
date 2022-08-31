use crate::analysis::{price_analysis::PriceAnalysis, analysis_count::AnalysisCount};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceAnalysisAverage {
    pub current_year: PriceAnalysis,
    pub three_year: PriceAnalysis,
    pub five_year: PriceAnalysis,
}

impl PriceAnalysisAverage {
    pub fn count(list: Vec<PriceAnalysis>) -> Self {
        PriceAnalysisAverage {
            current_year: list[0].clone(),
            three_year: PriceAnalysisAverage::count_avg(3, &list),
            five_year: PriceAnalysisAverage::count_avg(5, &list),
        }
    }

    pub fn count_mean(list: Vec<f32>) -> f32 {
        list.iter().sum::<f32>() / list.len() as f32
    }

    pub fn count_avg(count: usize, list: &Vec<PriceAnalysis>) -> PriceAnalysis {
        let asset_price_list = PriceAnalysisAverage::create_price_list("asset_price", list);
        let eps_list = PriceAnalysisAverage::create_price_list("eps", list);
        let price_limit_list = PriceAnalysisAverage::create_price_list("price_limit", list);
        let safety_price_limit_list =
            PriceAnalysisAverage::create_price_list("safety_price_limit", list);
        let analysis_count_list = list
            .into_iter()
            .map(|count| count.analysis_count.clone())
            .collect::<Vec<AnalysisCount>>();

        if (list.len() > count) && (list.len() != 0) {
            PriceAnalysis {
                stock_id: list[0].stock_id,
                year: list[0].year,
                asset_price: PriceAnalysisAverage::count_mean(asset_price_list[..=count].to_vec()),
                eps: PriceAnalysisAverage::count_mean(eps_list[..=count].to_vec()),
                price_limit: PriceAnalysisAverage::count_mean(price_limit_list[..=count].to_vec()),
                safety_price_limit: PriceAnalysisAverage::count_mean(
                    safety_price_limit_list[..=count].to_vec(),
                ),
                analysis_count: PriceAnalysisAverage::count_analysis_count_avg(analysis_count_list),
            }
        } else {
            PriceAnalysis {
                stock_id: list[0].stock_id,
                year: list[0].year,
                asset_price: PriceAnalysisAverage::count_mean(asset_price_list),
                eps: PriceAnalysisAverage::count_mean(eps_list),
                price_limit: PriceAnalysisAverage::count_mean(price_limit_list),
                safety_price_limit: PriceAnalysisAverage::count_mean(safety_price_limit_list),
                analysis_count: PriceAnalysisAverage::count_analysis_count_avg(analysis_count_list),
            }
        }
    }

    pub fn create_price_list(price_type: &str, list: &Vec<PriceAnalysis>) -> Vec<f32> {
        match price_type {
            "asset_price" => list
                .into_iter()
                .map(|analysis| analysis.asset_price)
                .collect::<Vec<f32>>(),
            "eps" => list
                .into_iter()
                .map(|analysis| analysis.eps)
                .collect::<Vec<f32>>(),
            "price_limit" => list
                .into_iter()
                .map(|analysis| analysis.price_limit)
                .collect::<Vec<f32>>(),
            "safety_price_limit" => list
                .into_iter()
                .map(|analysis| analysis.safety_price_limit)
                .collect::<Vec<f32>>(),
            _ => list
                .into_iter()
                .map(|analysis| analysis.eps)
                .collect::<Vec<f32>>(),
        }
    }

    pub fn count_analysis_count_avg(list: Vec<AnalysisCount>) -> AnalysisCount {
        AnalysisCount {
            wonderful: list.iter().map(|count| count.wonderful).sum::<i32>() / list.len() as i32,
            pass: list.iter().map(|count| count.pass).sum::<i32>() / list.len() as i32,
            mediocre: list.iter().map(|count| count.mediocre).sum::<i32>() / list.len() as i32,
            fail: list.iter().map(|count| count.fail).sum::<i32>() / list.len() as i32,
        }
    }
}
