use crate::db::PgPool;
use crate::report::{model::*, req::ReportParam};
use crate::traits::report_response::{ReportHttpResponse, ReportRequestParam, ReportType};
use actix_web::{get, web, HttpResponse};

#[get("")]
async fn view_reports(pool: web::Data<PgPool>, param: web::Query<ReportParam>) -> HttpResponse {
    match (param.code.clone(), param.year) {
        (Some(code), Some(year)) => {
            let request = ReportRequestParam {
                report_type: ReportType::Normal,
                code: code,
                year: year,
            };
            Report::single_http_response(pool, request)
        }
        (Some(code), None) => {
            let request = ReportRequestParam {
                report_type: ReportType::Normal,
                code: code,
                year: 0,
            };

            Report::array_http_response(pool, request)
        }
        _ => HttpResponse::BadRequest().body(format!("Missing Parameter: code, year")),
    }
}

#[get("/analysis")]
async fn view_analysis(pool: web::Data<PgPool>, param: web::Query<ReportParam>) -> HttpResponse {
    match (param.code.clone(), param.year) {
        (Some(code), Some(year)) => {
            let request = ReportRequestParam {
                report_type: ReportType::Analysis,
                code: code,
                year: year,
            };
            Report::single_http_response(pool, request)
        }
        (Some(code), None) => {
            let request = ReportRequestParam {
                report_type: ReportType::Analysis,
                code: code,
                year: 0,
            };

            Report::array_http_response(pool, request)
        }
        _ => HttpResponse::BadRequest().body(format!("Missing Parameter: code, year")),
    }
}

#[get("/analysis/price")]
async fn view_margin_of_safety(
    pool: web::Data<PgPool>,
    param: web::Query<ReportParam>,
) -> HttpResponse {
    match param.code.clone() {
        Some(code) => {
            let request = ReportRequestParam {
                report_type: ReportType::PriceAnalysis,
                code: code,
                year: 0,
            };

            Report::array_http_response(pool, request)
        }
        None => HttpResponse::BadRequest().body(format!("Missing Parameter: code")),
    }
}

#[get("/ratios")]
async fn view_ratios(pool: web::Data<PgPool>, param: web::Query<ReportParam>) -> HttpResponse {
    match (param.code.clone(), param.year) {
        (Some(code), Some(year)) => {
            let request = ReportRequestParam {
                report_type: ReportType::Ratios,
                code: code,
                year: year,
            };
            Report::single_http_response(pool, request)
        }
        (Some(code), None) => {
            let request = ReportRequestParam {
                report_type: ReportType::Ratios,
                code: code,
                year: 0,
            };

            Report::array_http_response(pool, request)
        }
        _ => HttpResponse::BadRequest().body(format!("Missing Parameter: code, year")),
    }
}

#[get("/ratios/growth")]
async fn view_growth_ratios(
    pool: web::Data<PgPool>,
    param: web::Query<ReportParam>,
) -> HttpResponse {
    match param.code.clone() {
        Some(code) => {
            let request = ReportRequestParam {
                report_type: ReportType::GrowthRatios,
                code: code,
                year: 0,
            };

            Report::array_http_response(pool, request)
        }
        None => HttpResponse::BadRequest().body(format!("Missing Parameter: code")),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(view_reports)
        .service(view_analysis)
        .service(view_ratios)
        .service(view_growth_ratios)
        .service(view_margin_of_safety);
}
