#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod balance;
mod db;
mod income;
mod schema;
mod stock;

async fn serve_web(address: String, pool: db::PgPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/v1/stocks").configure(stock::handler::route))
            .service(web::scope("/v1/balance").configure(balance::handler::route))
            .service(web::scope("/v1/income").configure(income::handler::route))
    })
    .bind(address)?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = &env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = &env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("{}:{}", host, port);

    let pool = db::connect_pool();

    serve_web(address, pool).await
}
