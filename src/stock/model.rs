use crate::balance::model::Balance;
use crate::db::PgPool;
use crate::income::model::Income;
use crate::ratios::{comparative_ratios::ComparativeRatios, per_share_ratios::PerShareRatios};
use crate::schema::stocks::*;
use crate::stock::req;

use actix_web::web;
use diesel::{dsl::exists, select, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ReportIdentifier {
    pub stock_id: i32,
    pub year: i32,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Stock {
    pub id: i32,
    pub code: String,
    pub name: String,
}

impl Stock {
    pub fn check_existence(pool: web::Data<PgPool>, stock_code: String) -> QueryResult<bool> {
        let conn = &pool.get().unwrap();
        select(exists(dsl::stocks.filter(code.eq(stock_code.clone())))).get_result(conn)
    }

    pub fn add(pool: web::Data<PgPool>, body: web::Json<req::AddStockReq>) -> QueryResult<Stock> {
        let conn = &pool.get().unwrap();
        let data = (&code.eq(&body.code), &name.eq(&body.name));

        diesel::insert_into(dsl::stocks)
            .values(data)
            .get_result(conn)
    }

    pub fn view_all(pool: web::Data<PgPool>) -> QueryResult<Vec<Stock>> {
        let conn = &pool.get().unwrap();
        table.load::<Stock>(conn)
    }

    pub fn get_id(pool: web::Data<PgPool>, stock_code: String) -> QueryResult<i32> {
        let conn = &pool.get().unwrap();

        table
            .filter(&code.eq(stock_code))
            .select(id)
            .get_result::<i32>(conn)
    }

    pub fn get_balance_and_income(
        pool: web::Data<PgPool>,
        identifier: ReportIdentifier,
    ) -> (QueryResult<Balance>, QueryResult<Income>) {
        let balance_data = Balance::get(pool.clone(), identifier.clone());
        let income_data = Income::get(pool.clone(), identifier.clone());

        (balance_data, income_data)
    }

    pub fn create_ratios(pool: web::Data<PgPool>, identifier: ReportIdentifier) {
        let balance_income_data = Stock::get_balance_and_income(pool.clone(), identifier.clone());

        match balance_income_data {
            (Ok(balance), Ok(income)) => {
                PerShareRatios::create(pool.clone(), (balance.clone(), income.clone()));
                ComparativeRatios::create(pool, (balance, income));
            }
            _ => println!("Skipped creating ratios {:?}", identifier),
        }
    }
}
