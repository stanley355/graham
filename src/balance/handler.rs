use crate::balance::{model, req};
use crate::db::PgPool;
use crate::stock::model::Stock;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn add_balance(pool: web::Data<PgPool>, body: web::Json<req::AddBalanceReq>) -> HttpResponse {
    let stock_id = Stock::get_id(pool.clone(), body.code.clone());

    match stock_id {
        Ok(id) => match model::Balance::add(pool, body, id) {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
        },
        Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
    }
}

// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
    config.service(add_balance);
}
