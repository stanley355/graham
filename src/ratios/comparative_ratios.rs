use crate::balance::model::Balance;
use crate::db::PgPool;
use crate::income::model::Income;
use crate::schema::comparative_ratios::*;

use actix_web::web;
use diesel::{ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct ComparativeRatios {
    id: i32,
    stock_id: i32,
    year: i32,
    gross_profit_margin: i32,
    operating_profit_margin: i32,
    net_profit_margin: i32,
    current_asset_return: i32,
    tangible_asset_return: i32,
    total_liability_return: i32,
    revenue_receivable_return: i32,
    revenue_inventory_return: i32,
    current_asset_liabilities_return: i32,
    tangible_asset_total_liabilities_return: i32,
}

impl ComparativeRatios {
    pub fn create(pool: web::Data<PgPool>, payload: (Balance, Income)) {
        let balance = payload.0;
        let income = payload.1;
        let conn = &pool.get().unwrap();

        let gross_prof_margin = (income.gross_profit as f64 / income.revenue as f64) * 100.0;
        let op_prof_margin = (income.operating_profit as f64 / income.revenue as f64) * 100.0;
        let net_prof_margin = (income.net_profit as f64 / income.revenue as f64) * 100.0;
        let cur_asset_return =
            (income.net_profit as f64 / balance.net_current_asset as f64) * 100.0;
        let tang_asset_return =
            (income.net_profit as f64 / balance.net_tangible_asset as f64) * 100.0;
        let total_liabil_return =
            (income.net_profit as f64 / balance.total_liabilities as f64) * 100.0;
        let reven_receivable_return = (income.revenue as f64 / balance.receivables as f64) * 100.0;
        let reven_inventory_return = (income.revenue as f64 / balance.inventories as f64) * 100.0;
        let current_asset_to_liabilitiy_return =
            (balance.current_asset as f64 / balance.st_liabilities as f64) * 100.0;
        let tang_asset_to_total_liability_return =
            (balance.tangible_asset as f64 / balance.total_liabilities as f64) * 100.0;

        let data = (
            (stock_id.eq(&balance.stock_id)),
            (year.eq(&balance.year)),
            (gross_profit_margin.eq(gross_prof_margin as i32)),
            (operating_profit_margin.eq(op_prof_margin as i32)),
            (net_profit_margin.eq(net_prof_margin as i32)),
            (current_asset_return.eq(cur_asset_return as i32)),
            (tangible_asset_return.eq(tang_asset_return as i32)),
            (total_liability_return.eq(total_liabil_return as i32)),
            (revenue_receivable_return.eq(reven_receivable_return as i32)),
            (revenue_inventory_return.eq(reven_inventory_return as i32)),
            (current_asset_liabilities_return.eq(current_asset_to_liabilitiy_return as i32)),
            (tangible_asset_total_liabilities_return
                .eq(tang_asset_to_total_liability_return as i32)),
        );

        let insert_result = diesel::insert_into(dsl::comparative_ratios)
            .values(data)
            .get_result::<ComparativeRatios>(conn);

        match insert_result {
            Ok(_) => println!("Comparative ratios created successfully!"),
            Err(err) => println!("Error in creating Comparative ratios : {:?}", err),
        }
    }
}
