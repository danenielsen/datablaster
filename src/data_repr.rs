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
    fields: Vec<(String, ColumnData)>,
}

type TupleIter = std::vec::IntoIter<(String, ColumnData)>;
type TupleIter2<'a> = std::slice::Iter<'a, (String, ColumnData)>;

impl Tuple {
    pub fn new() -> Tuple {
        Tuple { fields: Vec::new() }
    }

    pub fn add_field_data<S: Into<String>>(&mut self, name: S, data: ColumnData) {
        self.fields.push((name.into(), data));
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
    type Item = &'a (String, ColumnData);
    type IntoIter = TupleIter2<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.fields.iter()
    }
}
