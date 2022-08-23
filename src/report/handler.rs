use crate::db::PgPool;
use crate::report::{model::*, req::ReportParam};
use crate::stock::model::Stock;
use actix_web::{get, web, HttpResponse};

#[get("/")]
async fn view_reports(pool: web::Data<PgPool>, param: web::Query<ReportParam>) -> HttpResponse {
    match (param.code.clone(), param.year) {
        (Some(code), Some(year)) => {
            let stock_id = Stock::get_id(pool.clone(), code);

            match stock_id {
                Ok(id) => {
                    let identifier = ReportIdentifier {
                        stock_id: id,
                        year: year,
                    };

                    let report_result = Report::get_report(pool, identifier);

                    match report_result {
                        Ok(report) => HttpResponse::Ok().json(report),
                        Err(err) => {
                            HttpResponse::InternalServerError().body(format!("Error {:?}", err))
                        }
                    }
                }
                Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
            }
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

pub fn route(config: &mut web::ServiceConfig) {
    config.service(view_reports);
}
