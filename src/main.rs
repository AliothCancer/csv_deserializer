use std::path::Path;

use csv_deserializer::dataset::columns::Columns;

fn main() {
    let p = Path::new("example").join("iris").join("iris.csv");
    assert!(p.exists());
    let rdr = csv::ReaderBuilder::new().from_path(p).unwrap();
    let df = Columns::from_reader(rdr).unwrap();

    dbg!(df);
}
