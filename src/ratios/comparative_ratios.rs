use crate::balance::model::Balance;
use crate::db::PgPool;
use crate::income::model::Income;
use crate::schema::comparative_ratios::*;
use crate::stock::model::ReportIdentifier;

use actix_web::web;
use diesel::{
    dsl::exists, select, BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl,
};
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
    pub fn check_existence(
        pool: web::Data<PgPool>,
        payload: ReportIdentifier,
    ) -> QueryResult<bool> {
        let conn = &pool.get().unwrap();

        select(exists(dsl::comparative_ratios.filter(
            stock_id.eq(&payload.stock_id).and(year.eq(&payload.year)),
        )))
        .get_result(conn)
    }

    pub fn add(pool: web::Data<PgPool>, balance: Balance, income: Income) {
        let conn = &pool.get().unwrap();

        let gross_prof_margin = (&income.gross_profit / &income.revenue) * 100;
        let op_prof_margin = (&income.operating_profit / &income.revenue) * 100;
        let net_prof_margin = (&income.net_profit / &income.revenue) * 100;
        let cur_asset_return = (&income.net_profit / &balance.net_current_asset) * 100;
        let tang_asset_return = (&income.net_profit / &balance.net_tangible_asset) * 100;
        let total_liabil_return = (&income.net_profit / &balance.total_liabilities) * 100;
        let reven_receivable_return = (&income.revenue / &balance.receivables) * 100;
        let reven_inventory_return = (&income.revenue / &balance.inventories) * 100;
        let current_asset_to_liabilitiy_return = (&balance.current_asset / &balance.st_liabilities) * 100;
        let tang_asset_to_total_liability_return = (&balance.tangible_asset / &balance.total_liabilities) * 100;

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
            (tangible_asset_total_liabilities_return.eq(tang_asset_to_total_liability_return as i32))
        );

        let insert_result = diesel::insert_into(dsl::comparative_ratios)
            .values(data)
            .get_result::<ComparativeRatios>(conn);

        match insert_result {
            Ok(_) => println!("Comparative ratios created successfully"),
            Err(err) => println!("Error in creating Comparative ratios : {:?}", err),
        }
    }
}
