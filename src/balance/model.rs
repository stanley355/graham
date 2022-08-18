use crate::balance::req;
use crate::db::PgPool;
use crate::income::model::Income;
use crate::ratios::{comparative_ratios::ComparativeRatios, per_share_ratios::PerShareRatios};
use crate::schema::balance::*;
use crate::stock::model::ReportIdentifier;

use actix_web::web;
use diesel::{
    dsl::exists, select, BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Balance {
    pub id: i32,
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
}

impl Balance {
    pub fn check_existence(
        pool: web::Data<PgPool>,
        identifier: ReportIdentifier,
    ) -> QueryResult<bool> {
        let conn = &pool.get().unwrap();

        select(exists(
            dsl::balance.filter(
                stock_id
                    .eq(&identifier.stock_id)
                    .and(year.eq(&identifier.year)),
            ),
        ))
        .get_result(conn)
    }

    pub fn get(pool: web::Data<PgPool>, identifier: ReportIdentifier) -> QueryResult<Balance> {
        let conn = &pool.get().unwrap();

        table
            .filter(
                stock_id
                    .eq(identifier.stock_id)
                    .and(year.eq(identifier.year)),
            )
            .get_result::<Balance>(conn)
    }

    pub fn add(
        pool: web::Data<PgPool>,
        body: web::Json<req::AddBalanceReq>,
        stck_id: i32,
    ) -> String {
        let conn = &pool.get().unwrap();

        let new_quick_asset = &body.cash + &body.receivables;
        let new_current_asset = &new_quick_asset + &body.inventories;
        let new_tangible_asset = &new_current_asset + &body.fixed_asset;

        let data = (
            (&stock_id.eq(&stck_id)),
            (&year.eq(&body.year)),
            (&cash.eq(&body.cash)),
            (&receivables.eq(&body.receivables)),
            (&inventories.eq(&body.inventories)),
            (&fixed_asset.eq(&body.fixed_asset)),
            (&quick_asset.eq(&new_quick_asset)),
            (&current_asset.eq(&new_current_asset)),
            (&tangible_asset.eq(&new_tangible_asset)),
            (&st_liabilities.eq(&body.st_liabilities)),
            (&lt_liabilities.eq(&body.lt_liabilities)),
            (&total_liabilities.eq(&body.st_liabilities + &body.lt_liabilities)),
            (&net_cash_asset.eq(&body.cash - &body.st_liabilities)),
            (&net_quick_asset.eq(&new_quick_asset - &body.st_liabilities)),
            (&net_current_asset.eq(&new_current_asset - &body.st_liabilities)),
            (&net_tangible_asset
                .eq(&new_tangible_asset - &body.st_liabilities - &body.lt_liabilities)),
            (&share_outstanding.eq(&body.share_outstanding)),
        );

        let insert_result = diesel::insert_into(dsl::balance)
            .values(data)
            .get_result::<Balance>(conn);

        match insert_result {
            Ok(balance) => {
                let identifier = ReportIdentifier {
                    stock_id: balance.stock_id,
                    year: balance.year,
                };
                PerShareRatios::add(pool.clone(), identifier);
                // Balance::create_comparative_ratios(pool.clone(), balance);
                format!("Balance Sheet created successfully")
            }
            Err(err) => format!("Error in inserting balance sheet: {:?}", err),
        }
    }


    pub fn create_comparative_ratios(pool: web::Data<PgPool>, balance_sheet: Balance) {
        let identifier = ReportIdentifier {
            stock_id: balance_sheet.stock_id,
            year: balance_sheet.year,
        };

        let income_sheet_exist = Income::check_existence(pool.clone(), identifier.clone()).unwrap();

        match income_sheet_exist {
            true => {
                let income_sheet = Income::get(pool.clone(), identifier);

                match income_sheet {
                    Ok(income) => ComparativeRatios::add(pool.clone(), balance_sheet, income),
                    Err(err) => println!("Failed creating Comparative Ratios error: {:?}", err),
                }
            }
            false => println!("Skipped creating Comparative Ratios of {:?}", identifier),
        }
    }
}
