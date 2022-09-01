use crate::balance::req::AddBalanceReq;
use calamine::{open_workbook, DataType, Range, Reader, Xlsx, XlsxError};

pub type WorksheetRange = Result<Range<DataType>, XlsxError>;

#[derive(Debug, Clone)]
pub struct ExcelSheet {}

impl ExcelSheet {
    pub fn new(path: &String, sheet: &str) -> Option<WorksheetRange> {
        let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
        return workbook.worksheet_range(sheet);
    }

    pub fn read_balance(path: &String, sheet: &str) {
        let work_sheet = ExcelSheet::new(path, sheet);

        match work_sheet {
            Some(Ok(range)) => {
                let balance = AddBalanceReq {
                    code: ExcelSheet::parse_string(&range, (0, 1)),
                    year: ExcelSheet::parse_int(&range, (1, 1)) as i32,
                    cash: ExcelSheet::parse_int(&range, (2, 1)),
                    receivables: ExcelSheet::parse_int(&range, (3, 1)),
                    inventories: ExcelSheet::parse_int(&range, (4, 1)),
                    fixed_asset: ExcelSheet::parse_int(&range, (5, 1)),
                    st_liabilities: ExcelSheet::parse_int(&range, (6, 1)),
                    lt_liabilities: ExcelSheet::parse_int(&range, (7, 1)),
                    share_outstanding: ExcelSheet::parse_int(&range, (8, 1)),
                };

                println!("{:?}", balance);
            }
            _ => ()
        }

        ()
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
