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
    fields: HashMap<String, ColumnData<'a>>,
}

type TupleIter<'a> = std::collections::hash_map::IntoIter<String, ColumnData<'a>>;
type TupleIter2<'a> = std::collections::hash_map::Iter<'a, String, ColumnData<'a>>;

impl<'a> Tuple<'a> {
    pub fn new() -> Tuple<'a> {
        Tuple { fields: HashMap::new() }
    }

    pub fn add_column_data<S: Into<String>>(&mut self, name: S, data: ColumnData<'a>) {
        self.fields.insert(name.into(), data);
    }
}

impl<'a> IntoIterator for Tuple<'a> {
    type Item = (String, ColumnData<'a>);
    type IntoIter = TupleIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

impl<'a> IntoIterator for &'a Tuple<'a> {
    type Item = (&'a String, &'a ColumnData<'a>);
    type IntoIter = TupleIter2<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.fields.iter()
    }
}
