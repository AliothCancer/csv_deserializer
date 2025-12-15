use std::fmt::Display;

use itertools::Itertools;

use crate::{CsvAny, CsvColumn, CsvDataset};

pub struct ColumnMetrics {
    pub column_name: String,
    pub number_of_empties: u32,
    pub number_of_nulls: u32,
    pub number_of_strings: u32,
    pub number_of_floats: u32,
    pub number_of_ints: u32,
    pub unique_values: Vec<CsvAny>,
}

impl ColumnMetrics {
    pub fn new(dataset: &CsvDataset, column_name: &str) -> Self {
        let (index, column_name) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(_, x)| column_name.contains(x.as_str()))
            .unwrap_or_else(|| panic!("No column named {column_name} found!"));

        let mut number_of_empties = 0;
        let mut number_of_nulls: u32 = 0;
        let mut number_of_strings: u32 = 0;
        let mut number_of_floats: u32 = 0;
        let mut number_of_ints: u32 = 0;

        let mut values: Vec<&CsvAny> = dataset
            .values
            .iter()
            .map(|x| {
                if let CsvColumn::AnyColumn(x) = x {
                    &x[index]
                } else {
                    unimplemented!()
                }
            })
            .collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let unique_values = values
            .into_iter()
            .inspect(|&x| {
                match x {
                    CsvAny::Str(_) => number_of_strings += 1,
                    CsvAny::Int(_) => number_of_ints += 1,
                    CsvAny::Float(_) => number_of_floats += 1,
                    CsvAny::Null => number_of_nulls += 1,
                    CsvAny::Empty => number_of_empties += 1,
                };
            })
            .dedup_by(|a, b| a == b)
            .cloned()
            .collect::<Vec<CsvAny>>();

        Self {
            column_name: column_name.to_string(),
            number_of_empties,
            number_of_nulls,
            number_of_strings,
            number_of_floats,
            number_of_ints,
            unique_values,
        }
    }
}

impl Display for ColumnMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let render = [
            (self.number_of_empties, "Empties"),
            (self.number_of_nulls, "Nulls"),
            (self.number_of_strings, "Strings"),
            (self.number_of_floats, "Floats"),
            (self.number_of_ints, "Ints"),
        ]
        .into_iter()
        .map(|(x, str)| match x {
            0 => "".to_string(),
            n => format!("\n\t{str}: {n}"),
        })
        .collect::<String>();

        let unique_values = self
            .unique_values
            .iter()
            .map(|x| format!("\n\t{:?}", x))
            .collect::<String>();
        write!(
            f,
            "Name: {}\n\nTypes:{}\n\nUnique Values:{}",
            self.column_name, render, unique_values
        )
    }
}
