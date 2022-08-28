use crate::analysis::model::Analysis;
use crate::db::PgPool;
use crate::ratios::{growth_ratios::GrowthRatios, model::Ratios};
use crate::report::{model::*, req::ReportParam};
use crate::stock::model::Stock;
use crate::traits::report_response::{ReportHttpResponse, ReportRequestParam, ReportType};
use actix_web::{get, web, HttpResponse};

#[get("/")]
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
            let stock_id = Stock::get_id(pool.clone(), code);

            match stock_id {
                Ok(id) => {
                    let report_results = Report::get_reports(pool, id);

                    match report_results {
                        Ok(reports) => HttpResponse::Ok().json(reports),
                        Err(err) => {
                            HttpResponse::InternalServerError().body(format!("Error {:?}", err))
                        }
                    }
                }
                Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
            }
        }
        _ => HttpResponse::BadRequest().body(format!("Missing Parameter: code, year")),
    }
}

#[get("/analysis/")]
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
            let stock_id = Stock::get_id(pool.clone(), code);

            match stock_id {
                Ok(id) => {
                    let report_results = Report::get_reports(pool, id);

                    match report_results {
                        Ok(reports) => {
                            let analysis = Analysis::new_list(reports);
                            HttpResponse::Ok().json(analysis)
                        }
                        Err(err) => {
                            HttpResponse::InternalServerError().body(format!("Error {:?}", err))
                        }
                    }
                }
                Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
            }
        }
        _ => HttpResponse::BadRequest().body(format!("Missing Parameter: code, year")),
    }
}

#[get("/ratios/")]
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
            let stock_id = Stock::get_id(pool.clone(), code);

            match stock_id {
                Ok(id) => {
                    let report_list_result = Report::get_reports(pool.clone(), id);

                    match report_list_result {
                        Ok(report_list) => {
                            let ratio_list = Ratios::create_list(report_list);
                            HttpResponse::Ok().json(ratio_list)
                        }
                        Err(err) => {
                            HttpResponse::InternalServerError().body(format!("Error {:?}", err))
                        }
                    }
                }
                Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
            }
        }
        _ => HttpResponse::BadRequest().body(format!("Missing Parameter: code, year")),
    }
}

#[get("/ratios/growth/")]
async fn view_growth_ratios(
    pool: web::Data<PgPool>,
    param: web::Query<ReportParam>,
) -> HttpResponse {
    match param.code.clone() {
        Some(code) => {
            let stock_id = Stock::get_id(pool.clone(), code);

            match stock_id {
                Ok(id) => {
                    let report_results = Report::get_reports(pool.clone(), id);

                    match report_results {
                        Ok(reports) => {
                            HttpResponse::Ok().json(GrowthRatios::create_yearly(reports))
                        }
                        Err(err) => {
                            HttpResponse::InternalServerError().body(format!("Error {:?}", err))
                        }
                    }
                }
                Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
            }
        }
        None => HttpResponse::BadRequest().body(format!("Missing Parameter: code")),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(view_reports)
        .service(view_analysis)
        .service(view_ratios)
        .service(view_growth_ratios);
}
