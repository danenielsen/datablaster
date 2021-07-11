

// Schema Code
#[derive(Debug, Clone)]
pub enum ColumnType<'a> {
    Integer,
    Float,
    String,
    Record(RecordSchema<'a>),
    List(Box<ColumnType<'a>>),
}

#[derive(Debug, Clone)]
pub struct ColumnSchema<'a> {
    pub name: &'a str,
    pub col_type: ColumnType<'a>,
}

impl<'a> ColumnSchema<'a> {
    pub fn new(name: &'a str, col_type: ColumnType<'a>) -> ColumnSchema<'a> {
        ColumnSchema {name: name, col_type: col_type}
    }
}

#[derive(Debug, Clone)]
pub struct RecordSchema<'a> {
    column_list: Vec<ColumnSchema<'a>>,
}

impl<'a> RecordSchema<'a> {
    pub fn new() -> Self {
        RecordSchema{
            column_list: Vec::new(),
        }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &ColumnSchema<'a>> {
        self.column_list.iter()
    }

    pub fn add_column(&mut self, column: ColumnSchema<'a>) {
        self.column_list.push(column);
    }

    pub fn with_column(mut self, column: ColumnSchema<'a>) -> Self {
        self.add_column(column);
        self
    }
}

impl<'a> IntoIterator for RecordSchema<'a> {
    type Item = ColumnSchema<'a>;
    type IntoIter = std::vec::IntoIter<ColumnSchema<'a>>;
    fn into_iter(self) -> Self::IntoIter {
        self.column_list.into_iter()
    }
}
