use crate::excel_reader::model::ExcelSheet;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExcelParam {
    file: String,
}

#[post("/")]
async fn read_file(param: web::Query<ExcelParam>) -> HttpResponse {
    let file_path = &param.file;
    let b = ExcelSheet::read_balance(file_path, "Sheet1");

    HttpResponse::Ok().body("hi")
}

// Routing for stocks
pub fn route(config: &mut web::ServiceConfig) {
    config.service(read_file);
}
