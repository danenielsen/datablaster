use super::*;
use crate::data_repr::ColumnData;
use crate::data_repr::*;
use std::io::{Error, ErrorKind, Write};

pub struct TupleToCSVSerializer<T: Write> {
    writer: T,
}

impl<T: Write> TupleToCSVSerializer<T> {
    pub fn new(writer: T) -> Self {
        TupleToCSVSerializer { writer }
    }
}

impl<T: Write> TupleWriter for TupleToCSVSerializer<T> {
    fn supports_list(&self) -> bool {
        false
    }
    fn supports_record(&self) -> bool {
        false
    }

    fn write_tuple(&mut self, tuple: &Tuple) -> std::io::Result<()> {
        let mut row: Vec<String> = vec![];
        for (_, data) in tuple {
            match data {
                ColumnData::Integer(v) => row.push(v.to_string()),
                ColumnData::Float(v) => row.push(v.to_string()),
                ColumnData::String(v) => row.push(v.to_string()),
                ColumnData::Record(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Record not supported by CSV",
                    ))
                }
                ColumnData::List(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "List not supported by CSV",
                    ))
                }
            }
        }
        let output = row.join(",");
        self.writer.write_all(output.as_bytes())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
