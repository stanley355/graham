use crate::db::PgPool;
use crate::stock::{model, req};
use actix_web::{get, post, web, HttpResponse};

#[post("/")]
async fn add_stock(pool: web::Data<PgPool>, body: web::Json<req::AddStockReq>) -> HttpResponse {
    let stock_code = body.code.clone();
    let stock_exist = model::Stock::check_existence(pool.clone(), stock_code).unwrap();

    match stock_exist {
        true => HttpResponse::BadRequest().body(format!("Error : Stock Code already exists")),
        false => match model::Stock::add(pool, body) {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(err) => HttpResponse::BadRequest().body(format!("Error {:?}", err)),
        },
    }
}

#[get("/")]
async fn view_stocks(pool: web::Data<PgPool>) -> HttpResponse {
    match model::Stock::view_all(pool) {
        Ok(all_stocks) => HttpResponse::Ok().json(all_stocks),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error {:?}", err)),
    }
}

// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
    config.service(add_stock).service(view_stocks);
}
