// mod csv_types;
// use csv_types::*;

use std::{error::Error, fs::File};

use csv_deserializer::{CsvDataset, NullValues, create_enum, enum_gen::generate_enums_from, struct_gen::gen_struct};

fn main() -> Result<(), Box<dyn Error>> {
    let path = "train.csv";
    let file = File::open(path)?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let dataset = CsvDataset::new(rdr, NullValues(&["NA"]));
    
    let enums = generate_enums_from(&dataset);
    let struc = gen_struct(&dataset);
    println!("#![allow(unused,non_snake_case)]\nuse csv_deserializer::create_enum;
\n{}\n{}", enums, struc);
    Ok(())
}

create_enum!(MyEnum;
    "ciao" | "hallo" => Ciao,
    "How" | "come" => Come,
    "Aller" | "va" => Va
);
