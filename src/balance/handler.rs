use crate::balance::{model, req};
use crate::db::PgPool;
use crate::report::{model::ReportIdentifier, req::ReportParam};
use crate::stock::model::Stock;
use actix_web::{get, post, put, web, HttpResponse};

#[post("/")]
async fn add_balance(pool: web::Data<PgPool>, body: web::Json<req::AddBalanceReq>) -> HttpResponse {
    let stock_id = Stock::get_id(pool.clone(), body.code.clone());

    match stock_id {
        Ok(id) => {
            let identifier = ReportIdentifier {
                stock_id: id,
                year: body.year.clone(),
            };
            let balance_exist = model::Balance::check_existence(pool.clone(), identifier);

            match balance_exist.unwrap() {
                true => HttpResponse::BadRequest().body(format!("Error : Balance Sheet exists!")),
                false => {
                    let new_balance = model::Balance::add(pool, body, id);
                    match new_balance {
                        Ok(balance) => HttpResponse::Ok().json(balance),
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
async fn update_balance(
    pool: web::Data<PgPool>,
    payload: web::Json<req::UpdateBalanceReq>,
) -> HttpResponse {
    let update_balance = model::Balance::update(pool, payload);

    match update_balance {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
    }
}

#[get("")]
async fn view_balance(pool: web::Data<PgPool>, param: web::Query<ReportParam>) -> HttpResponse {
    match (param.code.clone(), param.year) {
        (Some(code), Some(year)) => {
            let stock_id = Stock::get_id(pool.clone(), code);
            let identifier = ReportIdentifier {
                stock_id: stock_id.unwrap(),
                year: year,
            };
            let balance_res = model::Balance::get(pool, identifier);

            match balance_res {
                Ok(balance) => HttpResponse::Ok().json(balance),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
            }
        }
        _ => HttpResponse::BadRequest().body(format!("Missing Parameter: code, year")),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(add_balance)
        .service(update_balance)
        .service(view_balance);
}
