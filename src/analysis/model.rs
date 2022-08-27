use crate::report::model::Report;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AnalysisStatus {
    Pass,
    Fail,
}

impl fmt::Display for AnalysisStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AnalysisStatus::Pass => write!(f, "Pass"),
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
        if (report.cash < 0)
            | (report.receivables < 0)
            | (report.inventories < 0)
            | (report.fixed_asset < 0)
        {
            AnalysisStatus::Fail.to_string()
        } else {
            AnalysisStatus::Pass.to_string()
        }
    }
}
