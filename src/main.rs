#[macro_use]
mod enum_gen;

use std::{error::Error, fs::File};

use csv_deserializer::{
    CsvDataset, CsvAny, NullValues, dataset_metrics::ColumnMetrics,
    enum_sanitizer::sanitize_identifier,
};

fn main() -> Result<(), Box<dyn Error>> {
    let path = "train.csv";
    let file = File::open(path)?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let dataset = CsvDataset::new(rdr, NullValues(&["NA"]));

    dataset.names.iter().skip(2).take(1).for_each(|name| {
        let col_metrics = ColumnMetrics::new(&dataset, name);

        let name = sanitize_identifier(name);

        let variants = col_metrics
            .unique_values
            .iter()
            .map(|var| match var {
                CsvAny::Int(int) => sanitize_identifier(&int.to_string()),
                CsvAny::Str(str) => format!("\"{}\" => {}", str, sanitize_identifier(str)),
                CsvAny::Empty => "Empty".to_string(),
                CsvAny::Null => "Null".to_string(),
                CsvAny::Float(_) => panic!("Should not be used on float"),
            } + ",\n")
            .collect::<String>();
        println!("create_enum!({};\n{})", name, variants);
    });
    Ok(())
}
create_enum!(MyEnum;
    "ciao" | "hallo" => Ciao,
    "How" | "come" => Come,
    "Aller" | "va" => Va
);
