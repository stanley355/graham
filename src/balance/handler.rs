use crate::balance::{model, req};
use crate::db::PgPool;
use crate::ratios::model::PerShareRatios;
use crate::stock::model::Stock;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn add_balance(pool: web::Data<PgPool>, body: web::Json<req::AddBalanceReq>) -> HttpResponse {
    let stock_id = Stock::get_id(pool.clone(), body.code.clone());

    match stock_id {
        Ok(id) => {
            let balance_identifier = model::BalanceIdentifier {
                stock_id: id,
                year: body.year.clone(),
            };
            let balance_exist = model::Balance::check_existence(pool.clone(), balance_identifier);

            match balance_exist.unwrap() {
                true => HttpResponse::BadRequest().body(format!("Error : Balance Sheet exists!")),
                false => match model::Balance::add(pool.clone(), body, id) {
                    Ok(balance) => {
                        let update_balance_ratios =
                            PerShareRatios::add_balance_ratios(pool, balance.clone());
                        println!(
                            "The balance ratios update: {:?}",
                            update_balance_ratios.unwrap()
                        );
                        HttpResponse::Ok().json(balance)
                    }
                    Err(err) => {
                        HttpResponse::InternalServerError().body(format!("Error {:?}", err))
                    }
                },
            }
        }

        Err(err) => HttpResponse::BadRequest().body(format!("Error Stock ID {:?}", err)),
    }
}

// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
    config.service(add_balance);
}
