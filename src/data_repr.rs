use std::collections::HashMap;

// Data Repr
#[derive(Debug, Clone)]
pub enum ColumnData {
    Integer(i64),
    Float(f64),
    String(String),
    Record(Tuple),
    List(Vec<ColumnData>),
}


#[derive(Debug, Clone)]
pub struct Tuple {
    fields: HashMap<String, ColumnData>,
}

type TupleIter = std::collections::hash_map::IntoIter<String, ColumnData>;
type TupleIter2<'a> = std::collections::hash_map::Iter<'a, String, ColumnData>;

impl Tuple {
    pub fn new() -> Tuple {
        Tuple { fields: HashMap::new() }
    }

    pub fn add_column_data<S: Into<String>>(&mut self, name: S, data: ColumnData) {
        self.fields.insert(name.into(), data);
    }
}

impl IntoIterator for Tuple {
    type Item = (String, ColumnData);
    type IntoIter = TupleIter;
    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

impl<'a> IntoIterator for &'a Tuple {
    type Item = (&'a String, &'a ColumnData);
    type IntoIter = TupleIter2<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.fields.iter()
    }
}
