use std::{fs::File, path::Path};

use csv::Reader;

use crate::dataset::{Layout, cell_wrapper::CsvCell};

/// Row major version of csv dataset
pub struct Rows(Vec<Row>);
pub struct Row(Vec<CsvCell>);

impl Rows {
    pub fn from_reader(rdr: Reader<File>) -> Self {}
}

impl Layout for Rows {}
