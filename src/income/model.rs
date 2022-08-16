use crate::balance::model::Balance;
use crate::db::PgPool;
use crate::income::req;
use crate::ratios::per_share_ratios::PerShareRatios;
use crate::schema::income::*;
use crate::stock::model::ReportIdentifier;

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
        payload: ReportIdentifier,
    ) -> QueryResult<bool> {
        let conn = &pool.get().unwrap();

        select(exists(dsl::income.filter(
            stock_id.eq(&payload.stock_id).and(year.eq(&payload.year)),
        )))
        .get_result(conn)
    }

    pub fn add(
        pool: web::Data<PgPool>,
        body: web::Json<req::AddIncomeReq>,
        stck_id: i32,
    ) -> String {
        let conn = &pool.get().unwrap();

        let data = (
            (&stock_id.eq(&stck_id)),
            (&year.eq(&body.year)),
            (&revenue.eq(&body.revenue)),
            (&gross_profit.eq(&body.gross_profit)),
            (&operating_profit.eq(&body.operating_profit)),
            (&net_profit.eq(&body.net_profit)),
            (&customer_cashflow.eq(&body.customer_cashflow)),
            (&operating_cashflow.eq(&body.operating_cashflow)),
            (&investing_cashflow.eq(&body.investing_cashflow)),
            (&financing_cashflow.eq(&body.financing_cashflow)),
            (&total_cashflow.eq(&body.operating_cashflow
                + &body.investing_cashflow
                + &body.financing_cashflow)),
        );

        let insert_result = diesel::insert_into(dsl::income)
            .values(data)
            .get_result::<Income>(conn);

        match insert_result {
            Ok(income) => {
                let identifier = ReportIdentifier {
                    stock_id: income.stock_id,
                    year: income.year,
                };
                let outstanding_shares = Balance::get_outstanding_shares(pool.clone(), identifier);

                Income::create_ps_ratios(pool.clone(), income, outstanding_shares.unwrap());

                format!("Income Statement created successfully")
            }
            Err(err) => format!("Error in creating Income Statement: {:?}", err),
        }
    }

    pub fn create_ps_ratios(pool: web::Data<PgPool>, income_statement: Income, shares: i64) {
        let identifier = ReportIdentifier {
            stock_id: income_statement.stock_id,
            year: income_statement.year,
        };
        let balance_ratios_exist = PerShareRatios::check_existence(pool.clone(), identifier);
        match balance_ratios_exist.unwrap() {
            true => PerShareRatios::update_income_ratios(pool, income_statement, shares),
            false => PerShareRatios::add_income_ratios(pool, income_statement, shares),
        };
    }
}
