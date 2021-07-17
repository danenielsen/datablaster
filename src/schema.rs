use rand::prelude::*;
use std::fmt::Debug;

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
 * DataGenerator
 */
pub trait DataGenerator<T>: Debug + DataGeneratorClone<T> {
    fn generate_data(&self) -> T;
}

pub trait DataGeneratorClone<T> {
    fn clone_box(&self) -> Box<dyn DataGenerator<T>>;
}

impl<D, T> DataGeneratorClone<T> for D
where
    D: 'static + DataGenerator<T> + Clone,
{
    fn clone_box(&self) -> Box<dyn DataGenerator<T>> {
        Box::new(self.clone())
    }
}

impl<T> Clone for Box<dyn DataGenerator<T>> {
    fn clone(&self) -> Box<dyn DataGenerator<T>> {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct DataFunctionGenerator<T: Clone> {
    gen_fn: fn() -> T,
}

impl<T: Clone> DataFunctionGenerator<T> {
    pub fn new(gen_fn: fn() -> T) -> Self {
        DataFunctionGenerator::<T> { gen_fn }
    }
}

impl<T: 'static + Debug + Clone> DataGenerator<T> for DataFunctionGenerator<T> {
    fn generate_data(&self) -> T {
        (self.gen_fn)()
    }
}


/**
 * FieldDefinition
 */
#[derive(Debug, Clone)]
pub struct FieldDefinition<T> {
    generator: Box<dyn DataGenerator<T>>,
}

pub trait DefaultGenerator {
    fn default_gen() -> Box<dyn DataGenerator<Self>>;
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

impl DefaultGenerator for i64 {
    fn default_gen() -> Box<dyn DataGenerator<Self>> {
        Box::new(DataFunctionGenerator::new(|| rand::thread_rng().gen_range(0..100)))
    }
}

impl DefaultGenerator for f64 {
    fn default_gen() -> Box<dyn DataGenerator<Self>> {
        Box::new(DataFunctionGenerator::new(|| rand::thread_rng().gen_range(0.0..100.0)))
    }
}

impl DefaultGenerator for std::string::String {
    fn default_gen() -> Box<dyn DataGenerator<Self>> {
        Box::new(DataFunctionGenerator::new(|| "placeholder".to_string()))
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
