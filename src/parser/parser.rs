use std::collections::HashMap;
use crate::parser::csv_reader::CsvObject;
use crate::parser::json_writer::JsonObject;

pub fn parse(csv_object: CsvObject) -> Result<JsonObject, String> {
    let mut json_object = JsonObject {
        data: Vec::new(),
    };
    
    for data_row in csv_object.data {
        let mut sub_object = HashMap::new();
        for i in 0..csv_object.header.len() {
            sub_object.insert(csv_object.header[i].clone().into(), data_row[i].clone().into());
        }
        
        json_object.data.push(sub_object);
    }
    
    Ok(json_object)
}