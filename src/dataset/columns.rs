pub mod accumulator;
pub mod column_data;

use std::io::Read;

use crate::dataset::{
    Layout,
    cell_wrapper::CsvCell,
    columns::{accumulator::ColumnAccumulator, column_data::ColumnData},
};

#[derive(Debug)]
pub struct Columns {
    col_names: Vec<String>,
    data: Vec<ColumnData>, // una per colonna
}

impl Columns {
    pub fn from_reader<R: Read>(mut reader: csv::Reader<R>) -> Result<Self, csv::Error> {
        let col_names = reader
            .headers()?
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let num_cols = col_names.len();

        let mut acc = ColumnAccumulator::new(num_cols);

        let mut record = csv::ByteRecord::new();
        while reader.read_byte_record(&mut record)? {
            for (i, field) in record.iter().enumerate() {
                let s = std::str::from_utf8(field).unwrap_or("");
                let cell = CsvCell::parse(s);

                acc.get_mut_column(i).update(cell);
            }
        }

        // Conversion from RawCol to Inferred variant of ColumnData
        let data = acc.raw_columns.into_iter().map(ColumnData::new).collect();

        Ok(Columns { col_names, data })
    }
}

impl Layout for Columns {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn parse_csv() {
        let p = Path::new("example").join("iris").join("iris.csv");
        assert!(p.exists());
        let rdr = csv::ReaderBuilder::new().from_path(p).unwrap();
        let df = Columns::from_reader(rdr).unwrap();

        dbg!(df);
    }
}
