use crate::dataset::cell_wrapper::CsvCell;

pub(crate) struct ColumnAccumulator {
    // Gather all csv records in a columnar structure
    pub(crate) raw_columns: Vec<RawCol>,
}
impl ColumnAccumulator {
    pub(crate) fn new(num_cols: usize) -> Self {
        ColumnAccumulator {
            raw_columns: vec![RawCol::new(); num_cols],
        }
    }
    pub(crate) fn get_mut_column(&mut self, index: usize) -> &mut RawCol {
        self.raw_columns.get_mut(index).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ColumnTypes {
    pub(crate) has_int: bool,
    pub(crate) has_float: bool,
    pub(crate) has_str: bool,
    pub(crate) has_null: bool,
    pub(crate) has_empty: bool,
}

impl ColumnTypes {
    fn new() -> Self {
        ColumnTypes {
            has_int: false,
            has_float: false,
            has_str: false,
            has_null: false,
            has_empty: false,
        }
    }

    fn update(&mut self, cell: &CsvCell) {
        match cell {
            CsvCell::Int(_) => self.has_int = true,
            CsvCell::Float(_) => self.has_float = true,
            CsvCell::Str(_) => self.has_str = true,
            CsvCell::Null => self.has_null = true,
            CsvCell::Empty => self.has_empty = true,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RawCol {
    pub(crate) cells: Vec<CsvCell>,
    pub(crate) types: ColumnTypes,
}
impl RawCol {
    fn new() -> Self {
        Self {
            cells: vec![],
            types: ColumnTypes::new(),
        }
    }
    pub(crate) fn update(&mut self, cell: CsvCell) {
        self.types.update(&cell);
        self.cells.push(cell);
    }
}
