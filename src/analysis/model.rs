use crate::report::model::Report;
use crate::traits::report_response::ReportHttpResponse;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AnalysisStatus {
    Pass,
    Mediocre,
    Fail,
}

impl fmt::Display for AnalysisStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AnalysisStatus::Pass => write!(f, "Pass"),
            AnalysisStatus::Mediocre => write!(f, "Mediocre"),
            AnalysisStatus::Fail => write!(f, "Fail"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Analysis {
    pub no_minus_balance: String,
    pub no_minus_income: String,
    pub healthy_cashflow: String,
}

impl ReportHttpResponse for Analysis {}

impl Analysis {
    pub fn new_list(reports: Vec<Report>) -> Vec<Self> {
        reports.into_iter().map(|rep| Analysis::new(rep)).collect()
    }

    pub fn new(report: Report) -> Self {
        Analysis {
            no_minus_balance: Analysis::check_minus_balance(&report),
            no_minus_income: Analysis::check_minus_income(&report),
            healthy_cashflow: Analysis::check_cashflow_health(&report),
        }
    }

    pub fn check_minus_balance(report: &Report) -> String {
        // TODO: For non IT company, adjust if inventories and
        // fixed_asset is less than 0 then fail
        if (report.cash > 0)
            | (report.receivables > 0)
            | (report.net_current_asset > 0)
            | (report.net_tangible_asset > 0)
        {
            if (report.net_cash_asset > 0) && (report.net_quick_asset > 0) {
                AnalysisStatus::Pass.to_string()
            } else {
                AnalysisStatus::Mediocre.to_string()
            }
        } else {
            AnalysisStatus::Fail.to_string()
        }
    }

    pub fn check_minus_income(report: &Report) -> String {
        if (report.revenue > 0)
            | (report.gross_profit > 0)
            | (report.operating_profit > 0)
            | (report.net_profit > 0)
        {
            if report.operating_profit > report.net_profit {
                AnalysisStatus::Pass.to_string()
            } else {
                AnalysisStatus::Fail.to_string()
            }
        } else {
            AnalysisStatus::Fail.to_string()
        }
    }

    pub fn check_cashflow_health(report: &Report) -> String {
        if report.total_cashflow > 0 {
            if report.operating_cashflow > report.financing_cashflow {
                AnalysisStatus::Pass.to_string()
            } else {
                AnalysisStatus::Fail.to_string()
            }
        } else {
            AnalysisStatus::Mediocre.to_string()
        }
    }
}
