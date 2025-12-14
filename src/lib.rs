

pub mod dataset_metrics;

use std::io;

use csv::Reader;


#[derive(Debug)]
pub struct CsvDataset {
    pub names: Vec<String>,
    pub values: Vec<Vec<CsvValue>>,
    pub null_values: NullValues,
}
#[derive(Debug)]
pub struct NullValues( pub &'static [&'static str]);

impl CsvDataset {
    pub fn new<R: io::Read>(mut reader: Reader<R>, null_values: NullValues) -> Self {
        let names = reader.headers().unwrap().iter().map(String::from).collect();
            
        let values = reader
            .into_records()
            .map(|x| {
                x.unwrap()
                    .iter()
                    .map(|value| RawCsvValue(value).auto_cast(&null_values))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            names,
            values,
            null_values,
        }
    }
}

#[derive(Debug)]
struct RawCsvValue<'reader>(&'reader str);

impl<'reader> RawCsvValue<'reader> {
    fn auto_cast(&self, null_values: &NullValues) -> CsvValue {
        if self.0.is_empty() {
            return CsvValue::Empty;
        } else if null_values.0.contains(&self.0) {
            /*  */
            return CsvValue::Null;
        }

        let try_float = self.0.parse::<f64>();
        let try_int = self.0.parse::<i64>();

        match (try_float, try_int) {
            (_, Ok(int)) => CsvValue::Int(int),
            (Ok(float), Err(_)) => CsvValue::Float(float),
            (Err(_), Err(_)) => CsvValue::Str(self.0.to_owned()),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum CsvValue {
    Str(String),
    Int(i64),
    Float(f64),
    Null,  // to represent null values
    Empty, // if it is just empty
}
