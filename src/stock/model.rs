use crate::db::PgPool;
use crate::schema::stocks::dsl::*;
use crate::stock::req;

use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Stock {
    pub id: i32,
    pub code: String,
    pub name: String,
}

impl Stock {
    pub fn add(pool: web::Data<PgPool>, body: web::Json<req::AddStockReq>) -> QueryResult<Stock> {
        let conn = &pool.get().unwrap();
        let data = (&code.eq(&body.code), &name.eq(&body.name));

        diesel::insert_into(stocks).values(data).get_result(conn)
    }
}
