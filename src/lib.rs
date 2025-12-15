pub mod dataset_metrics;

#[macro_use]
pub mod enum_gen;

pub mod enum_sanitizer;

use std::io;

use csv::Reader;

#[derive(Debug)]
pub struct CsvDataset {
    pub names: Vec<String>,
    pub values: Vec<CsvColumn>,
    pub null_values: NullValues,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CsvColumn {
    IntColumn(Vec<CsvInt>),
    FloatColumn(Vec<CsvFloat>),
    StringColumn(Vec<CsvString>),
    AnyColumn(Vec<CsvAny>),
}

#[derive(Debug)]
pub struct NullValues(pub &'static [&'static str]);

impl CsvDataset {
    pub fn new<R: io::Read>(mut reader: Reader<R>, null_values: NullValues) -> Self {
        let names = reader.headers().unwrap().iter().map(String::from).collect();

        let values = reader
            .into_records()
            .map(|x| {
                let csv_col = x
                    .unwrap()
                    .iter()
                    .map(|value| RawCsvValue(value).as_csvany(&null_values))
                    .collect::<Vec<_>>();

                CsvColumn::AnyColumn(csv_col)
            })
            .collect::<Vec<_>>();

        Self {
            names,
            values,
            null_values,
        }
    }
}

#[derive(Debug)]
struct RawCsvValue<'reader>(&'reader str);

impl<'reader> RawCsvValue<'reader> {
    fn as_csvany(&self, null_values: &NullValues) -> CsvAny {
        if self.0.is_empty() {
            return CsvAny::Empty;
        } else if null_values.0.contains(&self.0) {
            /*  */
            return CsvAny::Null;
        }

        let try_float = self.0.parse::<f64>();
        let try_int = self.0.parse::<i64>();

        match (try_float, try_int) {
            (_, Ok(int)) => CsvAny::Int(int),
            (Ok(float), Err(_)) => CsvAny::Float(float),
            (Err(_), Err(_)) => CsvAny::Str(self.0.to_owned()),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum CsvAny {
    Str(String),
    Int(i64),
    Float(f64),
    Null,  // to represent null values
    Empty, // if it is just empty
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum CsvInt {
    Int(i64),
    Empty,
    Null,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum CsvFloat {
    Float(f64),
    Empty,
    Null,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum CsvString {
    Str(String),
    Empty,
    Null,
}

use std::convert::TryFrom;

impl TryFrom<CsvAny> for CsvInt {
    type Error = String;

    fn try_from(value: CsvAny) -> Result<Self, Self::Error> {
        match value {
            CsvAny::Int(i) => Ok(CsvInt::Int(i)),
            CsvAny::Empty => Ok(CsvInt::Empty),
            CsvAny::Null => Ok(CsvInt::Null),
            _ => Err(format!(
                "Type Mismatch: Cannot convert {:?} to CsvInt",
                value
            )),
        }
    }
}

impl TryFrom<CsvAny> for CsvFloat {
    type Error = String;

    fn try_from(value: CsvAny) -> Result<Self, Self::Error> {
        match value {
            CsvAny::Float(f) => Ok(CsvFloat::Float(f)),
            CsvAny::Empty => Ok(CsvFloat::Empty),
            CsvAny::Null => Ok(CsvFloat::Null),
            _ => Err(format!(
                "Type Mismatch: Cannot convert {:?} to CsvFloat",
                value
            )),
        }
    }
}

impl TryFrom<CsvAny> for CsvString {
    type Error = String;

    fn try_from(value: CsvAny) -> Result<Self, Self::Error> {
        match value {
            CsvAny::Str(s) => Ok(CsvString::Str(s)),
            CsvAny::Empty => Ok(CsvString::Empty),
            CsvAny::Null => Ok(CsvString::Null),
            _ => Err(format!(
                "Type Mismatch: Cannot convert {:?} to CsvString",
                value
            )),
        }
    }
}
