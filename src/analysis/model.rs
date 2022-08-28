use crate::report::model::Report;
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
}

impl Analysis {
    pub fn new(report: Report) -> Self {
        Analysis {
            no_minus_balance: Analysis::check_minus_balance(&report),
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
}
