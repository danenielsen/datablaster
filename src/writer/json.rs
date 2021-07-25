extern crate serde_json;

use super::*;
use crate::data_repr::ColumnData;
use crate::data_repr::*;
use serde_json::{json, to_string_pretty, Map, Value};
use std::io::Write;

pub struct TupleToJsonSerializer<T: Write> {
    wrt: T,
    pretty_print: bool,
}

impl<T: Write> TupleToJsonSerializer<T> {
    pub fn new(wrt: T, pretty_print: bool) -> Self {
        TupleToJsonSerializer { wrt, pretty_print }
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

impl<T: Write> TupleWriter for TupleToJsonSerializer<T> {
    fn supports_list(&self) -> bool {
        true
    }
    fn supports_record(&self) -> bool {
        true
    }

    fn write_tuple(&mut self, tuple: &Tuple) -> std::io::Result<()> {
        let mut record = if self.pretty_print {
            self.to_pretty_json_data(tuple)
        } else {
            self.to_json_data_recurse(tuple).to_string()
        };
        record.push('\n');
        self.wrt.write_all(record.as_bytes())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.wrt.flush()
    }
}
