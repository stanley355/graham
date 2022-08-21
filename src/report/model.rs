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
pub struct Report {
    pub id: i32,
    pub code: String,
    pub name: String,
}
