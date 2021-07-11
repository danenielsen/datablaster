use std::collections::HashMap;

// Data Repr
#[derive(Debug, Clone)]
pub enum ColumnData<'a> {
    Integer(i64),
    Float(f32),
    String(&'a str),
    Record(Tuple<'a>),
    List(Vec<ColumnData<'a>>),
}


#[derive(Debug, Clone)]
pub struct Tuple<'a> {
    fields: HashMap<&'a str, ColumnData<'a>>,
}


impl<'a> Tuple<'a> {
    pub fn new() -> Tuple<'a> {
        Tuple { fields: HashMap::new() }
    }

    pub fn add_column_data(&mut self, name: &'a str, data: ColumnData<'a>) {
        self.fields.insert(name, data);
    }
}
