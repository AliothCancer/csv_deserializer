#![allow(unused_variables)]

mod iris_latest_for_diff;
use std::{error::Error, fs::File, io};

use csv_deserializer::{NullValues, csv_dataset::CsvDataset};

//use crate::iris::*;
use crate::iris_latest_for_diff::*;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "example/iris/iris.csv";
    let mut dataset = read_csv_dataset(path, NullValues(vec![])).expect("failed to open file");
    dataset.populate_columns_infos();

    let target_info = dataset.get_info_column("target");
    
    println!("{}\n{}", target_info.type_countmap, target_info.variants_countmap);

    Ok(())
}

fn read_csv_dataset<'a>(path: &'a str, null_values: NullValues<'a>) -> io::Result<CsvDataset<'a>> {
    let file = File::open(path)?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    Ok(CsvDataset::new(rdr, null_values))
}
