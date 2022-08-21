use crate::balance::{model, req};
use crate::db::PgPool;
use crate::report::model::ReportIdentifier;
use crate::stock::model::Stock;
use actix_web::{post, web, HttpResponse};

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
                    let insert_result = model::Balance::add(pool, body, id);
                    HttpResponse::Ok().body(insert_result)
                }
            }
        }

        Err(err) => HttpResponse::BadRequest().body(format!("Error Stock ID {:?}", err)),
    }
}

// Routing for income
pub fn route(config: &mut web::ServiceConfig) {
    config.service(add_balance);
}
