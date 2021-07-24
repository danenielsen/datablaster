extern crate serde_json;

use super::*;
use crate::data_repr::ColumnData;
use crate::data_repr::*;
use serde_json::{json, to_string_pretty, Map, Value};

pub struct TupleToJsonSerializer {
    pretty_print: bool,
}

impl TupleToJsonSerializer {
    pub fn new(pretty_print: bool) -> Self {
        TupleToJsonSerializer { pretty_print }
    }

    pub fn to_pretty_json_data(&self, tuple: &Tuple) -> String {
        let output_value = self.to_json_data_recurse(tuple);
        match to_string_pretty(&output_value) {
            Err(e) => panic!("Error converting to pretty print json: {}", e),
            Ok(s) => s,
        }
    }

    fn to_json_data_recurse(&self, tuple: &Tuple) -> Value {
        let mut json_map = Map::new();
        for (field_name, field_value) in tuple {
            json_map.insert(
                field_name.to_string(),
                self.column_data_to_json_value(field_value),
            );
        }
        json!(json_map)
    }

    fn column_data_to_json_value(&self, col_data: &ColumnData) -> Value {
        match col_data {
            ColumnData::Integer(v) => json!(v),
            ColumnData::Float(v) => json!(v),
            ColumnData::String(v) => json!(v),
            ColumnData::Record(t) => self.to_json_data_recurse(t),
            ColumnData::List(v) => {
                let mut list = Vec::new();
                for cd in v {
                    list.push(self.column_data_to_json_value(cd))
                }
                json!(list)
            }
        }
    }
}

impl TupleSerializer for TupleToJsonSerializer {
    fn supports_list(&self) -> bool {
        true
    }
    fn supports_record(&self) -> bool {
        true
    }

    fn tuple_to_string(&self, tuple: &Tuple) -> String {
        if self.pretty_print {
            self.to_pretty_json_data(tuple)
        } else {
            self.to_json_data_recurse(tuple).to_string()
        }
    }
}
