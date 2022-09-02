use crate::db::PgPool;
use crate::income::{model, req};
use crate::report::{model::ReportIdentifier, req::ReportParam};
use crate::stock::model::Stock;
use actix_web::{get, post, put, web, HttpResponse};

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

#[put("/")]
async fn update_income(
    pool: web::Data<PgPool>,
    payload: web::Json<req::UpdateIncomeReq>,
) -> HttpResponse {
    let update_income = model::Income::update(pool, payload);

    match update_income {
        Ok(income) => HttpResponse::Ok().json(income),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
    }
}

#[get("")]
async fn view_income(pool: web::Data<PgPool>, param: web::Query<ReportParam>) -> HttpResponse {
    match (param.code.clone(), param.year) {
        (Some(code), Some(year)) => {
            let stock_id = Stock::get_id(pool.clone(), code);
            let identifier = ReportIdentifier {
                stock_id: stock_id.unwrap(),
                year: year,
            };
            let income_res = model::Income::get(pool, identifier);

            match income_res {
                Ok(income) => HttpResponse::Ok().json(income),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
            }
        }
        _ => HttpResponse::BadRequest().body(format!("Missing Parameter: code, year")),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(add_income)
        .service(update_income)
        .service(view_income);
}
