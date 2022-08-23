use crate::db::PgPool;
use crate::ratios::model::Ratios;
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

                    let report_result = Report::get_report(pool.clone(), identifier);

                    match report_result {
                        Ok(report) => {
                            let ratio = Ratios::new(report);
                            HttpResponse::Ok().json(ratio)
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

pub fn route(config: &mut web::ServiceConfig) {
    config.service(view_reports);
}
