use crate::db::PgPool;
use crate::income::{model, req};
use crate::report::model::ReportIdentifier;
use crate::stock::model::Stock;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn add_income(pool: web::Data<PgPool>, body: web::Json<req::AddIncomeReq>) -> HttpResponse {
    let stock_id = Stock::get_id(pool.clone(), body.code.clone());

    match stock_id {
        Ok(id) => {
            let identifier = ReportIdentifier {
                stock_id: id,
                year: body.year.clone(),
            };
            let income_exist = model::Income::check_existence(pool.clone(), identifier);

            match income_exist.unwrap() {
                true => HttpResponse::BadRequest().body(format!("Error : Income exists!")),
                false => {
                    let new_income = model::Income::add(pool, body, id);
                    match new_income {
                        Ok(income) => HttpResponse::Ok().json(income),
                        Err(err) => {
                            HttpResponse::InternalServerError().body(format!("Error : {:?}", err))
                        }
                    }
                }
            }
        }
        Err(err) => HttpResponse::BadRequest().body(format!("Error : {:?}", err)),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(add_income);
}
