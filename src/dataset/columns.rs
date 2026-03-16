use std::path::Path;

use csv::ReaderBuilder;

use crate::dataset::{Layout, cell_wrapper::CsvCell, reader_from_path};

/// Column major version of csv dataset
#[derive(Debug)]
pub struct Columns(Vec<Column>);

impl Columns {
    pub fn from_path(p: &Path) -> Self {
        let rdr = reader_from_path(p);
    }
}

#[derive(Debug)]
pub struct Column(Vec<CsvCell>);

impl Layout for Columns {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn parse_csv() {
        let p = Path::new("example").join("iris").join("iris.csv");
        assert!(p.exists());
        let df = Columns::parse(&p);
        dbg!(df);
    }
}
