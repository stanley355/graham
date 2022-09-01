use crate::balance::{model::Balance, req::AddBalanceReq};
use crate::db::PgPool;
use crate::income::{model::Income, req::AddIncomeReq};
use crate::stock::model::Stock;
use actix_web::web;
use calamine::{open_workbook, DataType, Range, Reader, Xlsx, XlsxError};

pub type WorksheetRange = Result<Range<DataType>, XlsxError>;

#[derive(Debug, Clone)]
pub struct ExcelSheet {}

impl ExcelSheet {
    pub fn new(path: &String, sheet: &str) -> Option<WorksheetRange> {
        let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
        return workbook.worksheet_range(sheet);
    }

    pub fn migrate_balance(pool: web::Data<PgPool>, path: &String, sheet: &str) {
        let work_sheet = ExcelSheet::new(path, sheet);

        match work_sheet {
            Some(Ok(range)) => {
                let column_length = range.end().unwrap().1;

                let mut i = 1;

                while i <= column_length {
                    let balance = AddBalanceReq {
                        code: ExcelSheet::parse_string(&range, (0, i)),
                        year: ExcelSheet::parse_int(&range, (1, i)) as i32,
                        cash: ExcelSheet::parse_int(&range, (2, i)),
                        receivables: ExcelSheet::parse_int(&range, (3, i)),
                        inventories: ExcelSheet::parse_int(&range, (4, i)),
                        fixed_asset: ExcelSheet::parse_int(&range, (5, i)),
                        st_liabilities: ExcelSheet::parse_int(&range, (6, i)),
                        lt_liabilities: ExcelSheet::parse_int(&range, (7, i)),
                        share_outstanding: ExcelSheet::parse_int(&range, (8, i)),
                    };
                    ExcelSheet::add_balance(pool.clone(), web::Json(balance));
                    i += 1;
                }
            }
            _ => println!("Workshet migration fail: path: {} ; sheet: {}", path, sheet),
        }
    }

    pub fn add_balance(pool: web::Data<PgPool>, body: web::Json<AddBalanceReq>) {
        let stock_id = Stock::get_id(pool.clone(), body.code.clone());

        let balance_res = Balance::add(pool, body, stock_id.unwrap());

        match balance_res {
            Ok(balance) => println!(
                "Added balance with stock_id: {} and year: {}",
                balance.stock_id, balance.year
            ),
            Err(_) => println!("Failed adding balance !"),
        }
    }

    pub fn migrate_income(pool: web::Data<PgPool>, path: &String, sheet: &str) {
        let work_sheet = ExcelSheet::new(path, sheet);

        match work_sheet {
            Some(Ok(range)) => {
                let column_length = range.end().unwrap().1;

                let mut i = 1;

                while i <= column_length {
                    let income = AddIncomeReq {
                        code: ExcelSheet::parse_string(&range, (9, i)),
                        year: ExcelSheet::parse_int(&range, (10, i)) as i32,
                        revenue: ExcelSheet::parse_int(&range, (11, i)),
                        gross_profit: ExcelSheet::parse_int(&range, (12, i)),
                        operating_profit: ExcelSheet::parse_int(&range, (13, i)),
                        net_profit: ExcelSheet::parse_int(&range, (14, i)),
                        customer_cashflow: ExcelSheet::parse_int(&range, (15, i)),
                        operating_cashflow: ExcelSheet::parse_int(&range, (16, i)),
                        investing_cashflow: ExcelSheet::parse_int(&range, (17, i)),
                        financing_cashflow: ExcelSheet::parse_int(&range, (18, i)),
                    };
                    ExcelSheet::add_income(pool.clone(), web::Json(income));
                    i += 1;
                }
            }
            _ => println!("Workshet migration fail: path: {} ; sheet: {}", path, sheet),
        }
    }

    pub fn add_income(pool: web::Data<PgPool>, body: web::Json<AddIncomeReq>) {
        let stock_id = Stock::get_id(pool.clone(), body.code.clone());

        let income_res = Income::add(pool, body, stock_id.unwrap());

        match income_res {
            Ok(income) => println!(
                "Added income with stock_id: {} and year: {}",
                income.stock_id, income.year
            ),
            Err(_) => println!("Failed adding income !"),
        }
    }

    pub fn parse_string(range: &Range<DataType>, position: (u32, u32)) -> String {
        range.get_value(position).unwrap().to_string()
    }

    pub fn parse_int(range: &Range<DataType>, position: (u32, u32)) -> i64 {
        range
            .get_value(position)
            .unwrap()
            .to_string()
            .parse::<i64>()
            .unwrap()
    }
}
