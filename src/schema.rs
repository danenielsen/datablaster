use rand::prelude::*;


/**
 * FieldSchema
 */
#[derive(Debug, Clone)]
pub struct FieldSchema {
    name: String,
    field_type: FieldType
}

impl FieldSchema {
    pub fn new<S: Into<String>>(name: S, field_type: FieldType) -> Self {
        FieldSchema {
            name: name.into(),
            field_type
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_type(&self) -> &FieldType {
        &self.field_type
    }
}


/**
 * FieldDefinition
 */
#[derive(Debug, Clone, Copy)]
pub struct FieldDefinition<T> {
    generator: fn() -> T,
}

pub trait DefaultGenerator {
    fn default_gen() -> fn() -> Self;
}

impl<T> FieldDefinition<T> {
    pub fn new(gen_func: fn() -> T) -> FieldDefinition<T> {
        FieldDefinition { generator: gen_func }
    }

    pub fn generate(&self) -> T {
        (self.generator)()
    }
}

impl<T: DefaultGenerator> Default for FieldDefinition<T> {
    fn default() -> FieldDefinition<T> {
        FieldDefinition::new(T::default_gen())
    }
}

impl DefaultGenerator for i64 {
    fn default_gen() -> fn() -> Self {
        || rand::thread_rng().gen_range(0..100)
    }
}

impl DefaultGenerator for f64 {
    fn default_gen() -> fn() -> Self {
        || rand::thread_rng().gen_range(0.0..100.0)
    }
}

impl DefaultGenerator for std::string::String {
    fn default_gen() -> fn() -> Self {
        || "placeholder".to_string()
    }
}


/**
 * FieldType
 */
#[derive(Debug, Clone)]
pub enum FieldType {
    Integer(FieldDefinition<i64>),
    Float(FieldDefinition<f64>),
    String(FieldDefinition<std::string::String>),
    List(Box<FieldType>),
    Record(RecordSchema),
}


/**
 * RecordSchema
 */
#[derive(Debug, Clone)]
pub struct RecordSchema {
    column_list: Vec<FieldSchema>,
}

impl RecordSchema {
    pub fn new() -> Self {
        RecordSchema {
            column_list: Vec::new(),
        }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &FieldSchema> {
        self.column_list.iter()
    }

    pub fn add_column(&mut self, column: FieldSchema) {
        self.column_list.push(column);
    }

    pub fn with_column(mut self, column: FieldSchema) -> Self {
        self.add_column(column);
        self
    }
}

impl IntoIterator for RecordSchema {
    type Item = FieldSchema;
    type IntoIter = std::vec::IntoIter<FieldSchema>;
    fn into_iter(self) -> Self::IntoIter {
        self.column_list.into_iter()
    }
}
