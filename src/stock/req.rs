use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddStockReq {
  pub code: String,
  pub name: String,
}