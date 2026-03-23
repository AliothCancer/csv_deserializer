use std::io::Read;

use crate::dataset::{
    cell_wrapper::CsvCell,
    columns::{Columns, column_data::ColData},
};

pub(crate) struct ColumnAccumulator<S: ColAccState, R> {
    col_names: Vec<String>,
    raw_columns: Vec<RawCol>,
    reader: csv::Reader<R>,
    _state: State<S>,
}
pub struct Empty;
pub struct Populated;
pub struct State<T: ColAccState>(T);
pub trait ColAccState {}
impl ColAccState for Empty {}
impl ColAccState for Populated {}
impl<R: Read> ColumnAccumulator<Empty, R> {
    // Resp: object construction
    pub(crate) fn new(mut reader: csv::Reader<R>) -> Result<Self, csv::Error> {
        let col_names = reader
            .headers()
            .unwrap()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let num_cols = col_names.len();
        Ok(ColumnAccumulator {
            col_names,
            raw_columns: vec![RawCol::new(); num_cols],
            reader,
            _state: State(Empty),
        })
    }

    // Resp: mutable access to single column
    pub(crate) fn get_mut_column(&mut self, index: usize) -> &mut RawCol {
        self.raw_columns.get_mut(index).unwrap()
    }

    // Resp: handling I/O
    pub fn accumulate(mut self) -> Result<ColumnAccumulator<Populated, R>, csv::Error> {
        let mut record = csv::StringRecord::new();
        while self.reader.read_record(&mut record)? {
            for (i, field) in record.iter().enumerate() {
                let cell = CsvCell::parse(field);

                self.get_mut_column(i).update(cell);
            }
        }
        Ok(ColumnAccumulator {
            _state: State(Populated),
            raw_columns: self.raw_columns,
            reader: self.reader,
            col_names: self.col_names,
        })
    }
}
impl<R: Read> ColumnAccumulator<Populated, R> {
    pub fn infer_columns(self) -> Result<Columns, csv::Error> {
        Ok(Columns {
            col_names: self.col_names,
            data: self.raw_columns.into_iter().map(ColData::new).collect(),
        })
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
