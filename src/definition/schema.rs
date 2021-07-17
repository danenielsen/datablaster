use super::gen::{DataGenerator, DefaultGenerator};

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
#[derive(Debug, Clone)]
pub struct FieldDefinition<T> {
    generator: Box<dyn DataGenerator<T>>,
}

impl<T> FieldDefinition<T> {
    pub fn new(generator: Box<dyn DataGenerator<T>>) -> FieldDefinition<T> {
        FieldDefinition { generator }
    }

    pub fn generate(&self) -> T {
        self.generator.generate_data()
    }
}

impl<T: DefaultGenerator> Default for FieldDefinition<T> {
    fn default() -> FieldDefinition<T> {
        FieldDefinition::new(T::default_gen())
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

    pub fn add_field(&mut self, column: FieldSchema) {
        self.column_list.push(column);
    }

    pub fn with_field(mut self, column: FieldSchema) -> Self {
        self.add_field(column);
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
