pub mod csv;
pub mod json;

use crate::data_repr::*;

pub trait TupleWriter {
    fn supports_list(&self) -> bool {
        false
    }
    fn supports_record(&self) -> bool {
        false
    }
    fn write_tuple(&mut self, tuple: &Tuple) -> std::io::Result<()>;
    fn flush(&mut self) -> std::io::Result<()>;
}
