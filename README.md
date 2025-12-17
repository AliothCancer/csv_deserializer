# Typed CSV Usage Guide

To use this library for generating and utilizing a typed Rust interface for your CSV files, follow these steps:

## 1. Loading the Dataset
First, load your CSV file using a `csv::Reader`. You then create a `CsvDataset` by providing the reader and specifying which strings should be treated as null values.

```rust
let file = File::open("train.csv")?;
let rdr = csv::ReaderBuilder::new()
    .has_headers(true)
    .from_reader(file);

let mut dataset = CsvDataset::new(rdr, NullValues(&["NA"]));
```

## 2. Generating Rust Code
Call the `print_csv_rust_code` function. This function accepts a `&mut CsvDataset`, generates the necessary Enums and Structs, and prints the resulting Rust code to `stdout`. You can redirect this output to a file from your command line to save the generated types.

```rust
// This will print the generated code to the console
print_csv_rust_code(&mut dataset);
```

## 3. Using the Generated Code
Once the code is saved into a file (e.g., `csv_types.rs`), you can import it into your project. To work with the typed data, initialize a `CsvDataFrame` type by passing the `CsvDataset` you created earlier.

```rust
mod csv_types;
use csv_types::*;

let df = CsvDataFrame::new(dataset);
```

## 4. Iris Dataset ETL Example
*(Reserved for Iris ETL examples)*

## 5. Type Recognition: Categorical vs Numerical
The library identifies types by attempting to parse each raw CSV value.
* **Numerical**: If a value parses as an `i64`, it is treated as an `Int`; if it parses as an `f64`, it is treated as a `Float`.
* **Categorical**: Values that cannot be parsed as numbers are treated as `Str`.
* **Metadata**: `ColumnInfo` tracks the count of these types and stores unique variants to facilitate categorical Enum generation.