use crate::balance::model::Balance;
use crate::db::PgPool;
use crate::schema::per_share_ratios::*;

use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
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
    pub fn add_balance_ratios(
        pool: web::Data<PgPool>,
        body: Balance,
    ) -> QueryResult<PerShareRatios> {
        let conn = &pool.get().unwrap();

        let data = (
            (stock_id.eq(&body.stock_id)),
            (year.eq(&body.year)),
            (cash_equity.eq(&body.net_cash_asset / &body.share_outstanding)),
            (quick_equity.eq(&body.net_quick_asset / &body.share_outstanding)),
            (current_equity.eq(&body.net_current_asset / &body.share_outstanding)),
            (tangible_equity.eq(&body.net_tangible_asset / &body.share_outstanding)),
        );

        diesel::insert_into(dsl::per_share_ratios)
            .values(data)
            .on_conflict(id)
            .do_update()
            .set(data)
            .get_result(conn)
    }
}
