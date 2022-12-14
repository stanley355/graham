use crate::db::PgPool;
use crate::income::req;
use crate::report::model::ReportIdentifier;
use crate::schema::income::*;

use actix_web::web;
use diesel::{
    dsl::exists, select, BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
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
        identifier: ReportIdentifier,
    ) -> QueryResult<bool> {
        let conn = &pool.get().unwrap();

        select(exists(
            dsl::income.filter(
                stock_id
                    .eq(&identifier.stock_id)
                    .and(year.eq(&identifier.year)),
            ),
        ))
        .get_result(conn)
    }

    pub fn add(
        pool: web::Data<PgPool>,
        body: web::Json<req::AddIncomeReq>,
        stck_id: i32,
    ) -> QueryResult<Income> {
        let conn = &pool.get().unwrap();

        let data = (
            (stock_id.eq(&stck_id)),
            (year.eq(&body.year)),
            (revenue.eq(&body.revenue)),
            (gross_profit.eq(&body.gross_profit)),
            (operating_profit.eq(&body.operating_profit)),
            (net_profit.eq(&body.net_profit)),
            (customer_cashflow.eq(&body.customer_cashflow)),
            (operating_cashflow.eq(&body.operating_cashflow)),
            (investing_cashflow.eq(&body.investing_cashflow)),
            (financing_cashflow.eq(&body.financing_cashflow)),
            (total_cashflow.eq(&body.operating_cashflow
                + &body.investing_cashflow
                + &body.financing_cashflow)),
        );

        diesel::insert_into(dsl::income)
            .values(data)
            .get_result::<Income>(conn)
    }

    pub fn update(
        pool: web::Data<PgPool>,
        body: web::Json<req::UpdateIncomeReq>,
    ) -> QueryResult<Income> {
        let conn = &pool.get().unwrap();

        let data = (
            (revenue.eq(&body.revenue)),
            (gross_profit.eq(&body.gross_profit)),
            (operating_profit.eq(&body.operating_profit)),
            (net_profit.eq(&body.net_profit)),
            (customer_cashflow.eq(&body.customer_cashflow)),
            (operating_cashflow.eq(&body.operating_cashflow)),
            (investing_cashflow.eq(&body.investing_cashflow)),
            (financing_cashflow.eq(&body.financing_cashflow)),
            (total_cashflow.eq(&body.operating_cashflow
                + &body.investing_cashflow
                + &body.financing_cashflow)),
        );

        diesel::update(dsl::income)
            .filter(stock_id.eq(body.stock_id).and(year.eq(body.year)))
            .set(data)
            .get_result::<Income>(conn)
    }

    pub fn get(pool: web::Data<PgPool>, identifier: ReportIdentifier) -> QueryResult<Income> {
        let conn = &pool.get().unwrap();
        dsl::income
            .filter(
                stock_id
                    .eq(identifier.stock_id)
                    .and(year.eq(identifier.year)),
            )
            .get_result::<Income>(conn)
    }
}
