mod csv_types;
use csv_types::*;

use std::{error::Error, fs::File};

use csv_deserializer::{CsvDataset, NullValues, create_enum, enum_gen::generate_enums_from, print_csv_rust_code, struct_gen::gen_struct};

fn main() -> Result<(), Box<dyn Error>> {
    let path = "train.csv";
    let file = File::open(path)?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let dataset = CsvDataset::new(rdr, NullValues(&["NA"]));
    // print_csv_rust_code(&mut dataset);
    let df = CsvDataFrame::new(dataset);
    
    Ok(())
}

create_enum!(MyEnum;
    "ciao" | "hallo" => Ciao,
    "How" | "come" => Come,
    "Aller" | "va" => Va
);
