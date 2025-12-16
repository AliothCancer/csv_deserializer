use crate::{COLUMN_TYPE_ENUM_NAME, CsvDataset, SanitizedStr, dataset_info::Variant};

/// It generates a struct named `CsvDataFrame` which
/// contains all Vec<T> where T is the generated enums
/// for each columns
pub fn gen_struct(dataset: &CsvDataset) -> String {
    let mut final_str = String::from("pub struct CsvDataFrame{\n");

    final_str.push_str(&format!("\tcolumns: Vec<{COLUMN_TYPE_ENUM_NAME}>,\n"));
    dataset.names.iter().for_each(|name| {
        final_str.push_str(&format!(
            "\t{}: Vec<{}>,\n",
            name.sanitized.0, name.sanitized.0
        ));
    });
    final_str.push('}');

    final_str
}

fn gen_new_method(unique_values: &[Variant]) -> String {
    format!("\
impl {COLUMN_TYPE_ENUM_NAME}{{

    fn new() -> Self{{

    }}

}}
")
}

#[allow(unused)]
fn gen_try_from_csvany(name: SanitizedStr, unique_values: &[Variant]) -> String {
    let mut try_from_str = String::new();
    format!(
        "\
impl TryFrom<CsvAny> for {}
", name.0
    );
    todo!()
}
