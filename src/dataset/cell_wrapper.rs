#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum CsvCell {
    Str(String),
    Int(i64),
    Float(f64),
    // to represent null values for f64 NaN or other result which produced
    // invalid values or value that are not representable
    Null,
    // if it is just empty: the source was already missing and
    // it wasn't produced in the process of elaboration of this library
    Empty,
}
