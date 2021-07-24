use super::*;
use crate::data_repr::ColumnData;
use crate::data_repr::*;

pub struct TupleToCSVSerializer {}

impl TupleToCSVSerializer {
    pub fn new() -> Self {
        TupleToCSVSerializer {}
    }
}

impl TupleSerializer for TupleToCSVSerializer {
    fn supports_list(&self) -> bool {
        false
    }
    fn supports_record(&self) -> bool {
        false
    }

    fn tuple_to_string(&self, tuple: &Tuple) -> String {
        let mut row: Vec<String> = vec![];
        for (_, data) in tuple {
            match data {
                ColumnData::Integer(v) => row.push(v.to_string()),
                ColumnData::Float(v) => row.push(v.to_string()),
                ColumnData::String(v) => row.push(v.to_string()),
                ColumnData::Record(_) => panic!("Record not supported"),
                ColumnData::List(_) => panic!("List not supported"),
            }
        }
        row.join(",")
    }
}
