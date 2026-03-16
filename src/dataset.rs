pub mod cell_wrapper;
pub mod columns;
pub mod rows;

use std::{io::BufReader, path::Path};

use csv::ReaderBuilder;

use crate::dataset::cell_wrapper::CsvCell;

pub struct CsvDataset<L: Layout> {
    columns: Vec<String>,
    datas: L,
}

/// Assumes a default config for `csv::ReaderBuilder`:
/// - presence of headers (column names)
/// - delimiter is comma (the `,` char)
fn reader_from_path(path: &Path) -> csv::Reader<std::fs::File> {
    ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path(path)
        .unwrap()
}

/// Mostly a marker trait for the two type of layout
/// warning: it is not an abstraction over the csv
/// that define the same operation for 2 different layout
/// because of performance and coherence over eteregeneous types
pub trait Layout {}

#[cfg(test)]
mod tests {}
