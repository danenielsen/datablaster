
#[derive(Debug)]
pub enum ColumnType<'a> {
    Integer,
    Float,
    String,
    Record(RecordSchema<'a>),
    List(Box<ColumnType<'a>>),
}

pub struct Column<'a, T> {
    pub name: &'a str,
    pub data: T,
}

impl<'a, T> Column<'a, T> {
    pub fn new(name: &'a str, data: T) -> Column<'a, T> {
        Column {name: name, data: data}
    }
}

#[derive(Debug)]
pub struct ColumnSchema<'a> {
    pub name: &'a str,
    pub col_type: ColumnType<'a>,
}

impl<'a> ColumnSchema<'a> {
    pub fn new(name: &'a str, col_type: ColumnType<'a>) -> ColumnSchema<'a> {
        ColumnSchema {name: name, col_type: col_type}
    }
}

#[derive(Debug)]
pub struct RecordSchema<'a> {
    column_list: Vec<ColumnSchema<'a>>,
}

impl<'a> RecordSchema<'a> {
    pub fn new() -> Self {
        RecordSchema{
            column_list: Vec::new(),
        }
    }

    pub fn add_column(&mut self, column: ColumnSchema<'a>) {
        self.column_list.push(column);
    }

    pub fn with_column(mut self, column: ColumnSchema<'a>) -> Self {
        self.add_column(column);
        self
    }
}
