pub mod dataset_info;

pub mod enum_gen;
pub mod struct_gen;

pub mod sanitizer;

use std::io;

use csv::Reader;

use crate::{dataset_info::{ColumnInfo, Variant}, sanitizer::sanitize_identifier};

pub const COLUMN_TYPE_ENUM_NAME: &str = "CsvColumn";
pub const MAIN_STRUCT_NAME: &str = "CsvDataFrame";

/// This is a form to represent the dataset
/// which does not deep typization but can still
/// be usefull, also info field holds some info about the
/// types for each column
#[derive(Debug)]
pub struct CsvDataset<'a> {
    pub names: Vec<ColName>,
    pub values: Vec<Vec<CsvAny>>,
    pub null_values: NullValues<'a>,
    pub info: Vec<ColumnInfo>,
}

#[derive(Debug, Clone, Copy)]
pub struct ValueNamesView<'a> {
    pub values: &'a [Vec<CsvAny>],
    pub names: &'a [ColName],
}

#[derive(Debug, Clone)]
pub struct ColName {
    pub raw: String,
    pub sanitized: SanitizedStr,
}

#[derive(Debug, Clone)]
pub struct SanitizedStr(pub String);

#[derive(Debug)]
pub struct NullValues<'a>(pub Vec<&'a str>);

impl<'a> CsvDataset<'a> {
    pub fn new<R: io::Read>(mut reader: Reader<R>, null_values: NullValues<'a>) -> Self {
        let names: Vec<ColName> = reader
            .headers()
            .unwrap()
            .iter()
            .map(|str| {
                let sanitized = SanitizedStr(sanitize_identifier(str));
                let raw = str.to_string();
                ColName { raw, sanitized }
            })
            .collect();
        let mut values: Vec<Vec<CsvAny>> = (0..names.len()).map(|_| Vec::new()).collect();
        reader.into_records().for_each(|x| {
            x.unwrap()
                .iter()
                .enumerate()
                .for_each(|(column_index, value)| {
                    let k = values.get_mut(column_index).unwrap();
                    k.push(RawCsvValue(value).as_csvany(&null_values));
                });
        });

        Self {
            names,
            values,
            null_values,
            info: Vec::new(),
        }
    }
    pub fn view_names_and_values(&self) -> ValueNamesView<'_> {
        ValueNamesView {
            values: &self.values,
            names: &self.names,
        }
    }
    pub fn split_view_and_info(&mut self) -> (ValueNamesView<'_>, &mut Vec<ColumnInfo>) {
        (
            ValueNamesView {
                values: &self.values,
                names: &self.names,
            },
            &mut self.info,
        )
    }

    /// Analyze every cell in the csv file to extract every unique value
    pub fn populate_column_infos(dataset: &mut CsvDataset){
    
    let (value_names_view, info) = dataset.split_view_and_info();
    let col_name = value_names_view.names;
    
    for col_name in col_name {

        let mut col_info = ColumnInfo::new(value_names_view, &col_name.raw);

        if !col_info.unique_values.iter().any(|x| x.csvany == CsvAny::Null){
            let str = String::from("Null");
            col_info.unique_values.push(Variant{ raw: str.clone(), sanitized: str, csvany: CsvAny::Null});
        }

        info.push(col_info.clone());
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
