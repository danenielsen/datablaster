pub mod json;
pub mod csv;

use std::fs::File;
use std::io::prelude::*;
use crate::data_repr::*;

pub struct FileWriter<T>
    where T: TupleSerializer + TupleToString {
    serializer: T,
}

impl<T> FileWriter<T> where T: TupleSerializer + TupleToString {
    pub fn new(serializer: T) -> Self {
        FileWriter { serializer }
    }

    pub fn write_to_file(&self, t: &Tuple, file: &mut File)
    {
        let mut line = self.serializer.tuple_to_string(t);
        if line.chars().last().expect("Serialized data is empty") != '\n' {
            line.push('\n')
        }
        file.write_all(line.as_bytes()).expect("Error writing to data to file");
    }
}

pub trait TupleSerializer {
    fn supports_list(&self) -> bool { false }
    fn supports_record(&self) -> bool { false }
}

pub trait TupleToString {
    fn tuple_to_string(&self, tuple: &Tuple) -> String;
}

pub trait TupleToBytes {
    fn tuple_to_bytes(&self, tuple: &Tuple) -> Vec<u8>;
}
