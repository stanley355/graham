use crate::db::PgPool;
use crate::income::{model, req};
use crate::stock::model::{Stock, ReportIdentifier};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn add_balance(pool: web::Data<PgPool>, body: web::Json<req::AddIncomeReq>) -> HttpResponse {
    let stock_id = Stock::get_id(pool.clone(), body.code.clone());

    match stock_id {
        Ok(id) => {
            let identifier = ReportIdentifier {
                stock_id: id,
                year: body.year.clone(),
            };
            let income_exist = model::Income::check_existence(pool.clone(), identifier);
            match income_exist.unwrap() {
                true => HttpResponse::BadRequest().body(format!("Error : Income Sheet exists!")),
                false => match model::Income::add(pool, body, id) {
                    Ok(res) => HttpResponse::Ok().json(res),
                    Err(err) => {
                        HttpResponse::InternalServerError().body(format!("Error {:?}", err))
                    }
                },
            }
        }

        Err(err) => HttpResponse::BadRequest().body(format!("Error Stock ID {:?}", err)),
    }
}

// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
    config.service(add_balance);
}
