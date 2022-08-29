use crate::db::PgPool;
use crate::schema::{balance, income};
use crate::traits::report_response::ReportHttpResponse;
use actix_web::web;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, QueryResult, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone)]
pub struct ReportIdentifier {
    pub stock_id: i32,
    pub year: i32,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Report {
    pub stock_id: i32,
    pub year: i32,
    pub cash: i64,
    pub receivables: i64,
    pub inventories: i64,
    pub fixed_asset: i64,
    pub quick_asset: i64,
    pub current_asset: i64,
    pub tangible_asset: i64,
    pub st_liabilities: i64,
    pub lt_liabilities: i64,
    pub total_liabilities: i64,
    pub net_cash_asset: i64,
    pub net_quick_asset: i64,
    pub net_current_asset: i64,
    pub net_tangible_asset: i64,
    pub share_outstanding: i64,
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

impl ReportHttpResponse for Report {}

impl Report {
    pub fn get_report(
        pool: web::Data<PgPool>,
        identifier: ReportIdentifier,
    ) -> QueryResult<Report> {
        let conn = &pool.get().unwrap();

        let selection = (
            balance::stock_id,
            balance::year,
            balance::cash,
            balance::receivables,
            balance::inventories,
            balance::fixed_asset,
            balance::quick_asset,
            balance::current_asset,
            balance::tangible_asset,
            balance::st_liabilities,
            balance::lt_liabilities,
            balance::total_liabilities,
            balance::net_cash_asset,
            balance::net_quick_asset,
            balance::net_current_asset,
            balance::net_tangible_asset,
            balance::share_outstanding,
            income::revenue,
            income::gross_profit,
            income::operating_profit,
            income::net_profit,
            income::customer_cashflow,
            income::operating_cashflow,
            income::investing_cashflow,
            income::financing_cashflow,
            income::total_cashflow,
        );

        balance::table
            .filter(
                balance::stock_id
                    .eq(identifier.stock_id)
                    .and(balance::year.eq(identifier.year)),
            )
            .inner_join(
                income::table.on(balance::stock_id
                    .eq(income::stock_id)
                    .and(balance::year.eq(income::year))),
            )
            .select(selection)
            .get_result::<Report>(conn)
    }

    pub fn get_reports(pool: web::Data<PgPool>, stck_id: i32) -> QueryResult<Vec<Report>> {
        let conn = &pool.get().unwrap();

        let selection = (
            balance::stock_id,
            balance::year,
            balance::cash,
            balance::receivables,
            balance::inventories,
            balance::fixed_asset,
            balance::quick_asset,
            balance::current_asset,
            balance::tangible_asset,
            balance::st_liabilities,
            balance::lt_liabilities,
            balance::total_liabilities,
            balance::net_cash_asset,
            balance::net_quick_asset,
            balance::net_current_asset,
            balance::net_tangible_asset,
            balance::share_outstanding,
            income::revenue,
            income::gross_profit,
            income::operating_profit,
            income::net_profit,
            income::customer_cashflow,
            income::operating_cashflow,
            income::investing_cashflow,
            income::financing_cashflow,
            income::total_cashflow,
        );

        balance::table
            .filter(balance::stock_id.eq(stck_id))
            .inner_join(
                income::table.on(balance::stock_id
                    .eq(income::stock_id)
                    .and(balance::year.eq(income::year))),
            )
            .select(selection)
            .order(balance::year.desc())
            .get_results::<Report>(conn)
    }
}
