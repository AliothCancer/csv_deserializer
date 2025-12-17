use crate::{
    COLUMN_TYPE_ENUM_NAME, ColName, CsvDataset, SanitizedStr,
    dataset_info::{ColumnInfo, Variant},
};

/// It generates a struct named `CsvDataFrame` which
/// contains all Vec<T> where T is the generated enums
/// for each columns
pub fn gen_struct(dataset: &CsvDataset) -> String {
    let mut final_str = String::from("pub struct CsvDataFrame{\n");

    final_str.push_str(&format!("\tpub columns: Vec<{COLUMN_TYPE_ENUM_NAME}>,\n"));
    dataset.names.iter().for_each(|name| {
        final_str.push_str(&format!(
            "\tpub {}: Vec<{}>,\n",
            name.sanitized.0.to_lowercase(), name.sanitized.0
        ));
    });
    final_str.push('}');

    let new_method = gen_new_method(&dataset.names, &dataset.info);
    final_str.push_str(&new_method);

    final_str
}

fn gen_new_method(col_names: &[ColName], cols_info: &[ColumnInfo]) -> String {
    let vecs_of_enums = cols_info
        .iter()
        .map(|col_info| gen_vec_of_enums(&col_info.column_name, &col_info.unique_values) + "\n\n")
        .collect::<String>();
    let fields_list = String::from("columns,\n");
    let fields_list = fields_list + &col_names
        .iter()
        .map(|colname| format!("{},\n\t\t\t", colname.sanitized.0.to_lowercase()))
        .collect::<String>();
    format!(
        "\
impl CsvDataFrame{{

    pub fn new(dataset: csv_deserializer::CsvDataset) -> Self{{
        {vecs_of_enums}
        
        let columns: Vec<CsvColumn> = dataset
            .names
            .iter()
            .enumerate()
            .filter_map(|(col_index, name)| CsvColumn::from_str(&name.raw).ok())
            .collect();

        CsvDataFrame{{
            {fields_list}
        }}
    }}
}}
"
    )
}

fn gen_vec_of_enums(col_name: &ColName, unique_values: &[Variant]) -> String {
    let ColName {
        raw: _raw,
        sanitized,
    } = col_name;
    let SanitizedStr(sanitized) = sanitized;
    let sanitized_lower = sanitized.to_lowercase();
    let mut float_case_already_written = false;
    let mut int_case_already_written = false;
    let mut str_case_already_written = false;
    let match_arms = unique_values
        .iter()
        .filter_map(|v| match &v.csvany {
            crate::CsvAny::Str(_) if !str_case_already_written=> {
                str_case_already_written = true;
                Some(format!("csv_deserializer::CsvAny::Str(s) => {sanitized}::from_str(s).unwrap(),\n"))
            }
            crate::CsvAny::Int(_) if !int_case_already_written => {
                int_case_already_written = true;
                Some(format!("csv_deserializer::CsvAny::Int(i) => {sanitized}::Int(*i),\n"))
            }
            crate::CsvAny::Float(_) if !float_case_already_written => {
                float_case_already_written = true;
                Some(format!("csv_deserializer::CsvAny::Float(f) => {sanitized}::Float(*f),\n"))
            }
            crate::CsvAny::Null => {
                Some(format!("csv_deserializer::CsvAny::Null => {sanitized}::Null,\n"))
            }
            crate::CsvAny::Empty => {
                Some(format!("csv_deserializer::CsvAny::Empty => {sanitized}::Null,\n"))
            }
            _ => None
        })
        .collect::<String>();
    format!(
        "\
let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == \"{sanitized}\")
            .unwrap();
let {sanitized_lower} = dataset.values[index].iter().map(|val| match val{{
    {match_arms}
    _ => panic!(),
}}).collect::<Vec<{sanitized}>>();
    "
    )
}

