use crate::ratios::model::Ratios;
use crate::report::model::Report;
use crate::traits::report_response::ReportHttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum AnalysisStatus {
    Wonderful,
    Pass,
    Mediocre,
    Fail,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Analysis {
    pub stock_id: i32,
    pub year: i32,
    pub no_minus_balance: AnalysisStatus,
    pub no_minus_income: AnalysisStatus,
    pub healthy_cashflow: AnalysisStatus,
    pub curr_asset_vs_st_liability: AnalysisStatus,
    pub fixed_asset_vs_lt_liability: AnalysisStatus,
    pub revenue_receivable_ratio: AnalysisStatus,
    pub revenue_inventory_ratio: AnalysisStatus,
    pub gross_profit_margin: AnalysisStatus,
    pub operating_profit_margin: AnalysisStatus,
    pub net_profit_margin: AnalysisStatus,
}

impl ReportHttpResponse for Analysis {}

impl Analysis {
    pub fn new_list(reports: Vec<Report>) -> Vec<Self> {
        reports.into_iter().map(|rep| Analysis::new(rep)).collect()
    }

    pub fn new(report: Report) -> Self {
        let ratios = Ratios::create(report.clone());

        Analysis {
            stock_id: report.stock_id.clone(),
            year: report.year.clone(),
            no_minus_balance: Analysis::check_minus_balance(&report),
            no_minus_income: Analysis::check_minus_income(&report),
            healthy_cashflow: Analysis::check_cashflow_health(&report),
            curr_asset_vs_st_liability: Analysis::check_asset_ratio(
                ratios.comparative_ratios.current_asset_liabilities_return,
            ),
            fixed_asset_vs_lt_liability: Analysis::check_asset_ratio(
                ratios
                    .comparative_ratios
                    .tang_asset_total_liabilities_return,
            ),
            revenue_receivable_ratio: Analysis::check_revenue_ratio(
                ratios.comparative_ratios.revenue_receivable_return,
            ),
            revenue_inventory_ratio: Analysis::check_revenue_ratio(
                ratios.comparative_ratios.revenue_inventory_return,
            ),
            gross_profit_margin: Analysis::check_margin_ratio(
                "Gross",
                ratios.comparative_ratios.gross_profit_margin,
            ),
            operating_profit_margin: Analysis::check_margin_ratio(
                "Operating",
                ratios.comparative_ratios.operating_profit_margin,
            ),
            net_profit_margin: Analysis::check_margin_ratio(
                "Net",
                ratios.comparative_ratios.net_profit_margin,
            ),
        }
    }

    pub fn check_minus_balance(report: &Report) -> AnalysisStatus {
        // TODO: For non IT company, adjust if inventories and
        // fixed_asset is less than 0 then fail
        if (report.cash > 0)
            | (report.receivables > 0)
            | (report.net_current_asset > 0)
            | (report.net_tangible_asset > 0)
        {
            if (report.net_cash_asset > 0) && (report.net_quick_asset > 0) {
                AnalysisStatus::Pass
            } else {
                AnalysisStatus::Mediocre
            }
        } else {
            AnalysisStatus::Fail
        }
    }

    pub fn check_minus_income(report: &Report) -> AnalysisStatus {
        if (report.revenue > 0)
            | (report.gross_profit > 0)
            | (report.operating_profit > 0)
            | (report.net_profit > 0)
        {
            if report.operating_profit > report.net_profit {
                AnalysisStatus::Pass
            } else {
                AnalysisStatus::Fail
            }
        } else {
            AnalysisStatus::Fail
        }
    }

    pub fn check_cashflow_health(report: &Report) -> AnalysisStatus {
        if report.total_cashflow > 0 {
            if report.operating_cashflow > report.financing_cashflow {
                AnalysisStatus::Pass
            } else {
                AnalysisStatus::Fail
            }
        } else {
            AnalysisStatus::Mediocre
        }
    }

    pub fn check_asset_ratio(ratio: f32) -> AnalysisStatus {
        if ratio > 0.0 {
            if ratio > 75.0 {
                AnalysisStatus::Pass
            } else {
                AnalysisStatus::Mediocre
            }
        } else {
            AnalysisStatus::Fail
        }
    }

    pub fn check_revenue_ratio(ratio: f32) -> AnalysisStatus {
        if ratio > 100.0 {
            if ratio > 400.0 {
                AnalysisStatus::Wonderful
            } else {
                AnalysisStatus::Pass
            }
        } else {
            AnalysisStatus::Fail
        }
    }

    pub fn check_margin_ratio(margin_type: &str, ratio: f32) -> AnalysisStatus {
        if ratio > 20.0 {
            AnalysisStatus::Wonderful
        } else {
            if ratio > 10.0 {
                match margin_type {
                    "Gross" => AnalysisStatus::Pass,
                    "Operating" => AnalysisStatus::Pass,
                    "Net" => AnalysisStatus::Wonderful,
                    _ => AnalysisStatus::Mediocre,
                }
            } else {
                if ratio > 5.0 {
                    match margin_type {
                        "Gross" => AnalysisStatus::Fail,
                        "Operating" => AnalysisStatus::Fail,
                        "Net" => AnalysisStatus::Pass,
                        _ => AnalysisStatus::Mediocre,
                    }
                } else {
                    AnalysisStatus::Fail
                }
            }
        }
    }
}
