use crate::balance::model::Balance;
use crate::db::PgPool;
use crate::income::model::Income;
use crate::schema::per_share_ratios::*;

use actix_web::web;
use diesel::{ExpressionMethods, RunQueryDsl};
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
    pub fn create(pool: web::Data<PgPool>, payload: (Balance, Income)) {
        let balance = payload.0;
        let income = payload.1;
        let conn = &pool.get().unwrap();

        let data = (
            (stock_id.eq(&balance.stock_id)),
            (year.eq(&balance.year)),
            (cash_equity.eq(&balance.net_cash_asset / &balance.share_outstanding)),
            (quick_equity.eq(&balance.net_quick_asset / &balance.share_outstanding)),
            (current_equity.eq(&balance.net_current_asset / &balance.share_outstanding)),
            (tangible_equity.eq(&balance.net_tangible_asset / &balance.share_outstanding)),
            (gross_profit.eq(&income.gross_profit / &balance.share_outstanding)),
            (operating_profit.eq(&income.operating_profit / &balance.share_outstanding)),
            (net_profit.eq(&income.net_profit / &balance.share_outstanding)),
            (cashflow.eq(&income.total_cashflow / &balance.share_outstanding)),
        );

        let insert_result = diesel::insert_into(dsl::per_share_ratios)
            .values(data)
            .get_result::<PerShareRatios>(conn);

        match insert_result {
            Ok(_) => println!("Per Share ratios created successfully!"),
            Err(err) => {
                println!("Error in creating Per Share ratios!");
                println!("Error in creating Per Share ratios, error : {:?}", err);
            }
        }
    }
}
