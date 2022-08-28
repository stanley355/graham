use crate::analysis::model::Analysis;
use crate::db::PgPool;
use crate::ratios::{growth_ratios::GrowthRatios, model::Ratios};
use crate::report::model::*;
use crate::stock::model::Stock;
use actix_web::{web, HttpResponse};

pub enum ReportType {
    Normal,
    Analysis,
    Ratios,
    GrowthRatios,
}

pub struct ReportRequestParam {
    pub report_type: ReportType,
    pub code: String,
    pub year: i32,
}

pub trait ReportHttpResponse {
    fn single_http_response(pool: web::Data<PgPool>, request: ReportRequestParam) -> HttpResponse {
        let stock_id = Stock::get_id(pool.clone(), request.code.clone());

        match stock_id {
            Ok(id) => {
                let identifier = ReportIdentifier {
                    stock_id: id,
                    year: request.year.clone(),
                };
                let report_result = Report::get_report(pool, identifier);

                match report_result {
                    Ok(report) => match request.report_type {
                        ReportType::Normal => HttpResponse::Ok().json(report),
                        ReportType::Analysis => {
                            let analysis = Analysis::new(report);
                            HttpResponse::Ok().json(analysis)
                        }
                        ReportType::Ratios => {
                            let ratio = Ratios::create(report);
                            HttpResponse::Ok().json(ratio)
                        }
                        ReportType::GrowthRatios => HttpResponse::Ok()
                            .body("Growth Ratios only available for multi reponse"),
                    },
                    Err(err) => {
                        HttpResponse::InternalServerError().body(format!("Error {:?}", err))
                    }
                }
            }
            Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
        }
    }

    fn array_http_response(pool: web::Data<PgPool>, request: ReportRequestParam) -> HttpResponse {
        let stock_id = Stock::get_id(pool.clone(), request.code.clone());

        match stock_id {
            Ok(id) => {
                let report_results = Report::get_reports(pool, id);

                match report_results {
                    Ok(reports) => match request.report_type {
                        ReportType::Normal => HttpResponse::Ok().json(reports),
                        ReportType::Analysis => {
                            let analysis = Analysis::new_list(reports);
                            HttpResponse::Ok().json(analysis)
                        }
                        ReportType::Ratios => {
                            let ratio_list = Ratios::create_list(reports);
                            HttpResponse::Ok().json(ratio_list)
                        }
                        ReportType::GrowthRatios => {
                            let growth_ratios = GrowthRatios::create_yearly(reports);
                            HttpResponse::Ok().json(growth_ratios)
                        }
                    },
                    Err(err) => {
                        HttpResponse::InternalServerError().body(format!("Error {:?}", err))
                    }
                }
            }
            Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
        }
    }
}
