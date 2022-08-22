use crate::balance::model::Balance;
use crate::db::PgPool;
use crate::ratios::{per_share_ratios::PerShareRatios, comparative_ratios::ComparativeRatios};
use crate::income::model::Income;

use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Ratios {
    pub per_share_ratios: PerShareRatios
}

impl Ratios {
    pub fn new(balance: Balance, income: Income) -> Self{
        let ps_ratios = PerShareRatios::new(balance, income);

        Self {
          per_share_ratios: ps_ratios
        }
    }
}
