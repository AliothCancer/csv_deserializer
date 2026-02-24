use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use itertools::Itertools;

use crate::{ColName, CsvAny, ValuesNamesView, sanitizer::sanitize_identifier};

#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub column_name: ColName,
    pub unique_values: Vec<Variant>,
    pub type_countmap: TypeCountMap,
    pub variants_countmap: VariantsCountMap,
}
#[derive(Debug, Clone)]
pub struct VariantsCountMap (pub HashMap<String, usize>);

impl fmt::Display for VariantsCountMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Collect and sort keys for consistent output
        let mut entries: Vec<_> = self.0.iter().collect();
        entries.sort_by_key(|entry| entry.0);

        writeln!(f, "+----------------------+------------+")?;
        writeln!(f, "| {:<20} | {:<10} |", "Variant", "Count")?;
        writeln!(f, "+----------------------+------------+")?;

        for (variant, count) in entries {
            // Truncate variant if too long to maintain table structure
            let variant_display = if variant.len() > 20 {
                &variant[..20]
            } else {
                variant
            };
            writeln!(f, "| {:<20} | {:<10} |", variant_display, count)?;
        }

        write!(f, "+----------------------+------------+")
    }
}


#[derive(Debug, Clone, Copy)]
pub struct TypeCountMap {
    pub number_of_empties: u32,
    pub number_of_nulls: u32,
    pub number_of_strings: u32,
    pub number_of_floats: u32,
    pub number_of_ints: u32,
}
impl fmt::Display for TypeCountMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Calculate the total so we can show a summary line
        let total = self.number_of_empties
            + self.number_of_nulls
            + self.number_of_strings
            + self.number_of_floats
            + self.number_of_ints;

        // Define column widths for alignment
        let label_width = 12;
        let count_width = 8;

        writeln!(f, "📊 Data Type Distribution")?;
        writeln!(f, "{:-<25}", "")?; // Prints a separator line "-------------------------"

        // Helper macro to keep lines clean (Label : Count)
        // {:<w} aligns left, {:>w} aligns right
        writeln!(
            f,
            "  {:<w$} : {:>w2$}",
            "Integers",
            self.number_of_ints,
            w = label_width,
            w2 = count_width
        )?;
        writeln!(
            f,
            "  {:<w$} : {:>w2$}",
            "Floats",
            self.number_of_floats,
            w = label_width,
            w2 = count_width
        )?;
        writeln!(
            f,
            "  {:<w$} : {:>w2$}",
            "Strings",
            self.number_of_strings,
            w = label_width,
            w2 = count_width
        )?;
        writeln!(
            f,
            "  {:<w$} : {:>w2$}",
            "Nulls",
            self.number_of_nulls,
            w = label_width,
            w2 = count_width
        )?;
        writeln!(
            f,
            "  {:<w$} : {:>w2$}",
            "Empties",
            self.number_of_empties,
            w = label_width,
            w2 = count_width
        )?;

        writeln!(f, "{:-<25}", "")?;
        // Use bold formatting if your terminal supports it, otherwise plain text
        writeln!(
            f,
            "  {:<w$} : {:>w2$}",
            "Total",
            total,
            w = label_width,
            w2 = count_width
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub raw: String,
    pub sanitized: String,
    pub csvany: CsvAny,
}

impl ColumnInfo {
    pub fn new(names_and_values_view: ValuesNamesView, column_name: &str) -> Self {
        let ValuesNamesView { values, names } = names_and_values_view;
        let (column_index, column_name) = names
            .iter()
            .enumerate()
            .find(|(_, x)| column_name == x.raw.as_str())
            .unwrap_or_else(|| panic!("No column named {column_name} found!"));

        let mut number_of_empties = 0;
        let mut number_of_nulls: u32 = 0;
        let mut number_of_strings: u32 = 0;
        let mut number_of_floats: u32 = 0;
        let mut number_of_ints: u32 = 0;

        let mut variants_countmap: VariantsCountMap = VariantsCountMap(HashMap::new());
        let mut values: Vec<&CsvAny> = values[column_index].iter().collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let unique_values = values
            .into_iter()
            .inspect(|x| {
                match x {
                    CsvAny::Str(s) => {
                        number_of_strings += 1;

                        variants_countmap
                            .0
                            .entry(s.to_string())
                            .and_modify(|x| *x += 1)
                            .or_insert(1);
                    }
                    CsvAny::Int(_) => number_of_ints += 1,
                    CsvAny::Float(_) => number_of_floats += 1,
                    CsvAny::Null => number_of_nulls += 1,
                    CsvAny::Empty => number_of_empties += 1,
                };
            })
            .dedup_by(|a, b| a == b)
            .cloned()
            .map(|unique_val| match unique_val {
                CsvAny::Str(str) => Variant {
                    raw: str.clone(),
                    sanitized: sanitize_identifier(&str),
                    csvany: CsvAny::Str(str),
                },
                CsvAny::Int(i) => {
                    let raw = i.to_string();
                    let sanitized = sanitize_identifier(&raw);
                    Variant {
                        raw,
                        sanitized,
                        csvany: CsvAny::Int(i),
                    }
                }
                CsvAny::Null => Variant {
                    raw: "Null".to_string(),
                    sanitized: "Null".to_string(),
                    csvany: CsvAny::Null,
                },
                CsvAny::Empty => Variant {
                    raw: "Empty".to_string(),
                    sanitized: "Empty".to_string(),
                    csvany: CsvAny::Empty,
                },
                CsvAny::Float(f) => Variant {
                    raw: f.to_string(),
                    sanitized: "".to_string(),
                    csvany: CsvAny::Float(f),
                },
            })
            .collect::<Vec<Variant>>();

        Self {
            column_name: column_name.clone(),
            type_countmap: TypeCountMap {
                number_of_empties,
                number_of_nulls,
                number_of_strings,
                number_of_floats,
                number_of_ints,
            },
            unique_values,
            variants_countmap,
        }
    }

    /// Get the unique variants for this column
    pub fn get_unique_values_as_str(&self) -> Vec<&str> {
        self.unique_values
            .iter()
            .map(|x| x.raw.as_str())
            .collect::<Vec<_>>()
    }
    /// Get the unique variants for this column
    pub fn get_unique_values_as_csvany(&self) -> Vec<&CsvAny> {
        self.unique_values
            .iter()
            .map(|x| &x.csvany)
            .collect::<Vec<_>>()
    }
}

impl Display for ColumnInfo{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cnt_map = &self.type_countmap;
        let render = [
            (cnt_map.number_of_empties, "Empties"),
            (cnt_map.number_of_nulls, "Nulls"),
            (cnt_map.number_of_strings, "Strings"),
            (cnt_map.number_of_floats, "Floats"),
            (cnt_map.number_of_ints, "Ints"),
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
            self.column_name.sanitized.0, render, unique_values
        )
    }
}
