pub mod csv;
pub mod json;

use crate::data_repr::*;
use std::fs::File;
use std::io::prelude::*;

pub struct FileWriter {
    serializer: Box<dyn TupleSerializer>,
}

impl FileWriter {
    pub fn new(serializer: Box<dyn TupleSerializer>) -> Self {
        FileWriter { serializer }
    }

    pub fn write_to_file(&self, t: &Tuple, file: &mut File) {
        let mut line = self.serializer.tuple_to_string(t);
        if line.chars().last().expect("Serialized data is empty") != '\n' {
            line.push('\n')
        }
        file.write_all(line.as_bytes())
            .expect("Error writing to data to file");
    }
}

pub trait TupleSerializer {
    fn supports_list(&self) -> bool {
        false
    }
    fn supports_record(&self) -> bool {
        false
    }
    fn tuple_to_string(&self, tuple: &Tuple) -> String;
}

pub trait TupleToBytes {
    fn tuple_to_bytes(&self, tuple: &Tuple) -> Vec<u8>;
}
