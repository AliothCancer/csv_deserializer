pub mod accumulator;
pub mod column_data;

use std::io::Read;

use crate::dataset::{
    Layout,
    columns::{accumulator::ColumnAccumulator, column_data::ColData},
};

#[derive(Debug)]
pub struct Columns {
    pub col_names: Vec<String>,
    pub data: Vec<ColData>, // una per colonna
}

impl Columns {
    pub fn from_reader<R: Read>(reader: csv::Reader<R>) -> Result<Self, csv::Error> {
        ColumnAccumulator::new(reader)?
            .accumulate()?
            .infer_columns()
    }
    pub fn get_column(&self, index: usize) -> Option<&ColData> {
        self.data.get(index)
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
