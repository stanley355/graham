use crate::db::PgPool;
use crate::stock::{model, req};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn add_stock(pool: web::Data<PgPool>, body: web::Json<req::AddStockReq>) -> HttpResponse {
    match model::Stock::add(pool, body) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
    }
}

// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
    config.service(add_stock);
}
