use std::io;

use csv::Reader;

use crate::{
    ColName, NullValues, RawCsvValue, RemovedColumn, SanitizedStr, ValueNamesMut, ValuesNamesView,
    csv_types::CsvAny,
    dataset_info::ColumnInfo,
    sanitizer::sanitize_identifier,
};

/// This is a form to represent the dataset
/// which does not deep typization but can still
/// be useful, also info field holds some info about the
/// types for each column
#[derive(Debug, Default)]
pub struct CsvDataset<'a> {
    pub names: Vec<ColName>,
    pub values: Vec<Vec<CsvAny>>,
    pub null_values: NullValues<'a>,
    pub info: Vec<ColumnInfo>,
}

impl<'a> CsvDataset<'a> {
    /// Get the `ColumnInfo` for the column
    /// matching the raw string name (non sanitized)
    pub fn get_info_column(&self, raw_colname: &str) -> &ColumnInfo {
        self.info
            .iter()
            .find(|x| x.column_name.raw == raw_colname)
            .expect("There is a column named target (non sanitized)")
    }

    pub fn columns_as_str(&self) -> Vec<&String> {
        self.names.iter().map(|x| &x.raw).collect::<Vec<_>>()
    }

    /// Lenght of column values are not checked, so every column can have
    /// different lenght, be aware of row indexing
    pub fn push(&mut self, col_name: &str, col_values: Vec<CsvAny>) {
        self.names.push(ColName::new(col_name));
        self.values.push(col_values);
    }

    pub fn remove(&mut self, col_name: &str) -> RemovedColumn {
        let (col_index, name) = self
            .names
            .iter()
            .enumerate()
            .find(|(_, name)| name.raw == col_name)
            .unwrap();
        RemovedColumn {
            col_values: self.values.remove(col_index),
            name: name.clone(),
        }
    }

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
    pub fn names_and_values_view(&self) -> ValuesNamesView<'_> {
        ValuesNamesView {
            values: &self.values,
            names: &self.names,
        }
    }
    pub fn names_and_values_mut(&mut self) -> ValueNamesMut<'_> {
        ValueNamesMut {
            values: self.values.as_mut_slice(),
            names: self.names.as_mut_slice(),
        }
    }

    pub fn split_view_and_info(&mut self) -> (ValuesNamesView<'_>, &mut Vec<ColumnInfo>) {
        (
            ValuesNamesView {
                values: &self.values,
                names: &self.names,
            },
            &mut self.info,
        )
    }

    /// Analyze every cell in the csv file to extract every unique value
    pub fn populate_columns_infos(&mut self) {
        let (value_names_view, info) = self.split_view_and_info();
        let col_name = value_names_view.names;

        for col_name in col_name {
            let col_info = ColumnInfo::new(value_names_view, &col_name.raw);
            info.push(col_info.clone());
        }
    }
}
