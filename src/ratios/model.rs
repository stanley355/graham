use crate::balance::model::Balance;
use crate::income::model::Income;
use crate::ratios::{comparative_ratios::ComparativeRatios, per_share_ratios::PerShareRatios};

use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Ratios {
    pub per_share_ratios: PerShareRatios,
    pub comparative_ratios: ComparativeRatios
}

impl Ratios {
    pub fn new(balance: Balance, income: Income) -> Self {
        let ps_ratios = PerShareRatios::new(balance.clone(), income.clone());
        let compar_ratios = ComparativeRatios::new(balance, income);

        Self {
            per_share_ratios: ps_ratios,
            comparative_ratios: compar_ratios
        }
    }
}
