use std::{error::Error, fs::File, io};

use csv::Reader;
use csv_deserializer::{CsvDataset, NullValues, dataset_metrics::ColumnMetrics};

fn main() -> Result<(), Box<dyn Error>> {
    let path = "train.csv";
    let file = File::open(path)?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let dataset = CsvDataset::new(rdr, NullValues(&["NA"]));

    dataset.names.iter().for_each(|name| {
        let col_metrics = ColumnMetrics::new(&dataset, name);
        if col_metrics.unique_values.len() < 30{
            println!("\n\n#####\n\n{col_metrics}");
        }
    });

    Ok(())
}
