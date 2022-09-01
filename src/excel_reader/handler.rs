use crate::db::PgPool;
use crate::excel_reader::model::ExcelSheet;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExcelParam {
    file: String,
}

#[post("/")]
async fn read_file(pool: web::Data<PgPool>, param: web::Query<ExcelParam>) -> HttpResponse {
    let file_path = &param.file;
    ExcelSheet::migrate_balance(pool, file_path, "Sheet1");

    HttpResponse::Ok().body("Done")
}

// Routing for stocks
pub fn route(config: &mut web::ServiceConfig) {
    config.service(read_file);
}
