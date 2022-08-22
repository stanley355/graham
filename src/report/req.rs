use serde::{Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub struct ReportParam {
  pub code: Option<String>,
  pub year: Option<i32>
}
