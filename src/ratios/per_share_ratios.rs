use crate::balance::model::Balance;
use crate::db::PgPool;
use crate::schema::per_share_ratios::*;

use actix_web::web;
use diesel::{
    dsl::exists, select, BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct PerShareRatios {
    pub id: i32,
    pub stock_id: i32,
    pub year: i32,
    pub cash_equity: Option<i64>,
    pub quick_equity: Option<i64>,
    pub current_equity: Option<i64>,
    pub tangible_equity: Option<i64>,
    pub gross_profit: Option<i64>,
    pub operating_profit: Option<i64>,
    pub net_profit: Option<i64>,
    pub cashflow: Option<i64>,
}

impl PerShareRatios {
    pub fn check_existence(pool: web::Data<PgPool>, payload: Balance) -> QueryResult<bool> {
        let conn = &pool.get().unwrap();

        select(exists(dsl::per_share_ratios.filter(
            stock_id.eq(&payload.stock_id).and(year.eq(&payload.year)),
        )))
        .get_result(conn)
    }

    pub fn add_balance_ratios(pool: web::Data<PgPool>, body: Balance) {
        let conn = &pool.get().unwrap();

        let data = (
            (stock_id.eq(&body.stock_id)),
            (year.eq(&body.year)),
            (cash_equity.eq(&body.net_cash_asset / &body.share_outstanding)),
            (quick_equity.eq(&body.net_quick_asset / &body.share_outstanding)),
            (current_equity.eq(&body.net_current_asset / &body.share_outstanding)),
            (tangible_equity.eq(&body.net_tangible_asset / &body.share_outstanding)),
        );

        let insert_result = diesel::insert_into(dsl::per_share_ratios)
            .values(data)
            .get_result::<PerShareRatios>(conn);

        match insert_result {
            Ok(_) => println!("Balance Sheet ratios created successfully"),
            Err(err) => println!("Error in creating Balance Sheet ratios : {:?}", err),
        }
    }

    pub fn update_balance_ratios(
        pool: web::Data<PgPool>,
        body: Balance,
    ) {
        let conn = &pool.get().unwrap();

        let data = (
            (stock_id.eq(&body.stock_id)),
            (year.eq(&body.year)),
            (cash_equity.eq(&body.net_cash_asset / &body.share_outstanding)),
            (quick_equity.eq(&body.net_quick_asset / &body.share_outstanding)),
            (current_equity.eq(&body.net_current_asset / &body.share_outstanding)),
            (tangible_equity.eq(&body.net_tangible_asset / &body.share_outstanding)),
        );

        let update_result = diesel::update(dsl::per_share_ratios)
            .filter(stock_id.eq(&body.stock_id).and(year.eq(&body.year)))
            .set(data)
            .get_result::<PerShareRatios>(conn);

        match update_result {
            Ok(_) => println!("Balance Sheet ratios created successfully"),
            Err(err) => println!("Error in creating Balance Sheet ratios : {:?}", err),
        }
    }
}
