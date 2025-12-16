impl CsvDataFrame {
    fn new(dataset: &mut CsvDataset) -> Self {
        let Id = dataset.values.pop().unwrap().into_iter().map(|id|{
            match id {
                csv_deserializer::CsvAny::Int(i) => Id::Int(i),
                csv_deserializer::CsvAny::Null => Id::Null,
                _ => panic!()
            }
        }).collect::<Vec<Id>>();
        
        dataset
            .raw_names
            .iter()
            .enumerate()
            .map(|(col_index, name)| match CsvColumns::from_str(name){
                Ok(csv_col) => {
                    let k = &dataset.values[col_index].iter().map(|value|{
                        
                    });
                },
                Err(err) => panic!("{err}"),
            });



        todo!()
    }
}
