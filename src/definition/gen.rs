use rand::prelude::*;
use std::fmt::Debug;

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
 * Default Generators
 */

pub trait DefaultGenerator {
    fn default_gen() -> Box<dyn DataGenerator<Self>>;
}

impl DefaultGenerator for i64 {
    fn default_gen() -> Box<dyn DataGenerator<Self>> {
        Box::new(DataFunctionGenerator::new(|| {
            rand::thread_rng().gen_range(0..100)
        }))
    }
}

impl DefaultGenerator for f64 {
    fn default_gen() -> Box<dyn DataGenerator<Self>> {
        Box::new(DataFunctionGenerator::new(|| {
            rand::thread_rng().gen_range(0.0..100.0)
        }))
    }
}

impl DefaultGenerator for std::string::String {
    fn default_gen() -> Box<dyn DataGenerator<Self>> {
        Box::new(DataFunctionGenerator::new(|| "placeholder".to_string()))
    }
}
