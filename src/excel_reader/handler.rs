use crate::excel_reader::model::read;
use serde::{Deserialize, Serialize};
use actix_web::{post, web, HttpResponse};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExcelParam {
  file: String
}


#[post("/")]
async fn read_file(param: web::Query<ExcelParam>) -> HttpResponse {
    let file_path = &param.file;
    read(file_path);

    HttpResponse::Ok().body("Hi")
}

// Routing for stocks
pub fn route(config: &mut web::ServiceConfig) {
    config.service(read_file);
}
