use crate::db::PgPool;
use crate::report::model::Report;
use crate::stock::model::Stock;
use actix_web::{get, post, web, HttpResponse};

#[get("/{stock_code}")]
async fn view_reports(pool: web::Data<PgPool>, path: web::Path<String>) -> HttpResponse {
    let stock_code = path.into_inner();

    let stock_id = Stock::get_id(pool.clone(), stock_code);

    match stock_id {
        Ok(id) => {
            let company_reports = Report::get_company_reports(pool.clone(), id);

            match company_reports {
                Ok(reports) => HttpResponse::Ok().json(reports),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error {:?}", err)),
            }
        }
        Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(view_reports);
}
