use crate::income::{model, req};
use crate::db::PgPool;
use crate::stock::model::Stock;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn add_balance(pool: web::Data<PgPool>, body: web::Json<req::AddIncomeReq>) -> HttpResponse {
    let stock_id = Stock::get_id(pool.clone(), body.code.clone());

    match stock_id {
        Ok(id) => {
            let income_identifier = model::IncomeIdentifier {
                stock_id: id,
                year: body.year.clone(),
            };
            let income_exist = model::Income::check_existence(pool.clone(), income_identifier);
            HttpResponse::Ok().body(format!("The stock Exist? {:?}", income_exist))
        }

        Err(err) => HttpResponse::BadRequest().body(format!("Error Stock ID {:?}", err)),
    }
}

// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
    config.service(add_balance);
}
