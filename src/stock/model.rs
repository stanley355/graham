use crate::db::PgPool;
use crate::schema::stocks::dsl::*;
use crate::stock::req;

use actix_web::web;
use diesel::{dsl::exists, select, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Stock {
    pub id: i32,
    pub code: String,
    pub name: String,
}

impl Stock {
    pub fn check_existence(pool: web::Data<PgPool>, stock_code: String) -> QueryResult<bool> {
        let conn = &pool.get().unwrap();
        let exist = select(exists(stocks.filter(code.eq(stock_code.clone())))).get_result(conn);

        exist
    }

    pub fn add(pool: web::Data<PgPool>, body: web::Json<req::AddStockReq>) -> QueryResult<Stock> {
        let conn = &pool.get().unwrap();
        let data = (&code.eq(&body.code), &name.eq(&body.name));

        diesel::insert_into(stocks).values(data).get_result(conn)
    }
}
