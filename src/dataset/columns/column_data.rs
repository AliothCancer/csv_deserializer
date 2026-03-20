use crate::dataset::{
    cell_wrapper::CsvCell,
    columns::accumulator::{ColumnTypes, RawCol},
};
#[derive(Debug)]
pub enum ColumnData {
    Ints(Vec<i64>),
    Floats(Vec<f64>),
    Strings(Vec<String>),
    NullableInts(Vec<Option<i64>>),
    NullableFloats(Vec<Option<f64>>),
    NullableStrings(Vec<Option<String>>),
}
impl ColumnData {
    pub(crate) fn new(raw_col: RawCol) -> Self {
        let types = raw_col.types;
        let nullable = types.has_null || types.has_empty;

        match types {
            ColumnTypes { has_str: true, .. } => {
                if nullable {
                    ColumnData::NullableStrings(
                        raw_col
                            .cells
                            .into_iter()
                            .map(|c| match c {
                                CsvCell::Str(s) => Some(s),
                                CsvCell::Int(i) => Some(i.to_string()),
                                CsvCell::Float(f) => Some(f.to_string()),
                                CsvCell::Null | CsvCell::Empty => None,
                            })
                            .collect(),
                    )
                } else {
                    ColumnData::Strings(
                        raw_col
                            .cells
                            .into_iter()
                            .map(|c| match c {
                                CsvCell::Str(s) => s,
                                CsvCell::Int(i) => i.to_string(),
                                CsvCell::Float(f) => f.to_string(),
                                _ => unreachable!(),
                            })
                            .collect(),
                    )
                }
            }

            ColumnTypes {
                has_float: true, ..
            } => {
                // promuovi Int → f64
                if nullable {
                    ColumnData::NullableFloats(
                        raw_col
                            .cells
                            .into_iter()
                            .map(|c| match c {
                                CsvCell::Float(f) => Some(f),
                                CsvCell::Int(i) => Some(i as f64),
                                CsvCell::Null | CsvCell::Empty => None,
                                _ => unreachable!(),
                            })
                            .collect(),
                    )
                } else {
                    ColumnData::Floats(
                        raw_col
                            .cells
                            .into_iter()
                            .map(|c| match c {
                                CsvCell::Float(f) => f,
                                CsvCell::Int(i) => i as f64,
                                _ => unreachable!(),
                            })
                            .collect(),
                    )
                }
            }

            ColumnTypes { has_int: true, .. } => {
                if nullable {
                    ColumnData::NullableInts(
                        raw_col
                            .cells
                            .into_iter()
                            .map(|c| match c {
                                CsvCell::Int(i) => Some(i),
                                CsvCell::Null | CsvCell::Empty => None,
                                _ => unreachable!(),
                            })
                            .collect(),
                    )
                } else {
                    ColumnData::Ints(
                        raw_col
                            .cells
                            .into_iter()
                            .map(|c| match c {
                                CsvCell::Int(i) => i,
                                _ => unreachable!(),
                            })
                            .collect(),
                    )
                }
            }

            // colonna interamente vuota/null
            _ => ColumnData::NullableStrings(raw_col.cells.into_iter().map(|_| None).collect()),
        }
    }
}
