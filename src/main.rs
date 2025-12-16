mod csv_types;
use csv_types::*;

use std::{error::Error, fs::File};

use csv_deserializer::{CsvDataset, NullValues, create_enum, enum_gen::generate_enums_from};

fn main() -> Result<(), Box<dyn Error>> {
    let path = "train.csv";
    let file = File::open(path)?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let dataset = CsvDataset::new(rdr, NullValues(&["NA"]));
    
    let enums = generate_enums_from(&dataset);
    println!("#![allow(unused)]\nuse csv_deserializer::create_enum;
\n{}", enums);
    Ok(())
}

create_enum!(MyEnum;
    "ciao" | "hallo" => Ciao,
    "How" | "come" => Come,
    "Aller" | "va" => Va
);
