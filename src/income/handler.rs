use crate::db::PgPool;
use crate::income::{model, req};
use crate::stock::model::Stock;
use crate::report::model::ReportIdentifier;
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
                true => {
                    HttpResponse::BadRequest().body(format!("Error : Income Statement exists!"))
                }
                false => {
                    let insert_result = model::Income::add(pool, body, id);
                    HttpResponse::Ok().body(insert_result)
                }
            }
        }
        Err(err) => HttpResponse::BadRequest().body(format!("Error Stock ID {:?}", err)),
    }
}

// Routing for income
pub fn route(config: &mut web::ServiceConfig) {
    config.service(add_income);
}
