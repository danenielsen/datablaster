extern crate serde_json;

use serde_json::{Map, Value, json, to_string_pretty};
use crate::data_repr::*;
use crate::data_repr::ColumnData;

pub fn to_json_data(tuple: &Tuple) -> String {
    let output_value = to_json_data_recurse(tuple);
    output_value.to_string()
}

pub fn to_pretty_json_data(tuple: &Tuple) -> String {
    let output_value = to_json_data_recurse(tuple);
    match to_string_pretty(&output_value) {
        Err(e) => panic!("Error converting to pretty print json: {}", e),
        Ok(s) => s,
    }
}

fn to_json_data_recurse(tuple: &Tuple) -> Value {
    let mut json_map = Map::new();
    for (field_name, field_value) in tuple {
        json_map.insert(field_name.to_string(), column_data_to_json_value(field_value));
    }
    Value::Object(json_map)
}

fn column_data_to_json_value(col_data: &ColumnData) -> Value {
    match col_data {
        ColumnData::Integer(v) => json!(v),
        ColumnData::Float(v) => json!(v),
        ColumnData::String(v) => json!(v),
        ColumnData::Record(t) => to_json_data_recurse(t),
        ColumnData::List(v) => {
            let mut list = Vec::new();
            for cd in v {
                list.push(column_data_to_json_value(cd))
            }
            json!(list)
        },
    }
}

fn write_data_as_csv() {
    
}