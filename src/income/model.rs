use crate::db::PgPool;
use crate::schema::income::*;
use crate::income::req;

use actix_web::web;
use diesel::{
    dsl::exists, select, BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

pub struct IncomeIdentifier {
    pub stock_id: i32,
    pub year: i32
}

pub struct Income {
    pub id: i32,
    pub stock_id: i32,
    pub year: i32,
    pub revenue: i64,
    pub gross_profit: i64,
    pub operating_profit: i64,
    pub net_profit: i64,
    pub customer_cashflow: i64,
    pub operating_cashflow: i64,
    pub investing_cashflow: i64,
    pub financing_cashflow: i64,
    pub total_cashflow: i64,
}

impl Income {
    pub fn check_existence(
        pool: web::Data<PgPool>,
        payload: IncomeIdentifier,
    ) -> QueryResult<bool> {
        let conn = &pool.get().unwrap();

        select(exists(dsl::income.filter(
            stock_id.eq(&payload.stock_id).and(year.eq(&payload.year)),
        )))
        .get_result(conn)
    }
}
