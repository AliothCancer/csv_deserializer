use std::num::{ParseFloatError, ParseIntError};

type Integer = i64;
type Float = f64;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum CsvCell {
    Str(String),
    Int(Integer),
    Float(Float),
    // to represent null values for f64 NaN or other result which produced
    // invalid values or value that are not representable
    Null,
    // if it is just empty: the source was already missing and
    // it wasn't produced in the process of elaboration of this library
    Empty,
}

impl CsvCell {
    pub fn parse(cell_str: &str) -> Self {
        match RawCellMetadata::new(cell_str) {
            RawCellMetadata { is_empty: true, .. } => CsvCell::Empty,
            RawCellMetadata {
                parsable_int: Ok(int),
                ..
            } => CsvCell::Int(int),
            RawCellMetadata {
                parsable_float: Ok(float),
                ..
            } => {
                if float.is_finite() {
                    CsvCell::Float(float)
                } else {
                    CsvCell::Null
                }
            }
            RawCellMetadata { .. } => CsvCell::Str(cell_str.to_string()),
        }
    }
}

struct RawCellMetadata {
    is_empty: bool,
    parsable_int: Result<Integer, ParseIntError>,
    parsable_float: Result<Float, ParseFloatError>,
}

impl RawCellMetadata {
    fn new(s: &str) -> Self {
        Self {
            is_empty: s.is_empty(),
            parsable_int: s.parse(),
            parsable_float: s.parse(),
        }
    }
}
