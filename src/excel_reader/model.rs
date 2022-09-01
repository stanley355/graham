use calamine::{open_workbook, DataType, Reader, Xlsx};

pub fn read(path: &String) {
  let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    // Read whole worksheet data and provide some statistics
    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.used_cells().count();
        println!(
            "Found {} cells in 'Sheet1', including {} non empty cells",
            total_cells, non_empty_cells
        );
        // alternatively, we can manually filter rows
        assert_eq!(
            non_empty_cells,
            range
                .rows()
                .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
                .count()
        );
    }
}
