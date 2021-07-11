use rand::prelude::*;
use crate::schema::{ColumnType, RecordSchema};
use crate::data_repr::{ColumnData, Tuple};


pub fn create_data_from_schema<'a>(schema: &'a RecordSchema<'a>) -> Tuple<'a> {
    let tuple = Tuple::new();
    create_data_from_schema_recurse(schema, tuple)
}


fn create_data_from_schema_recurse<'a>(schema: &'a RecordSchema<'a>, mut tuple: Tuple<'a>) -> Tuple<'a> {
    for cs in schema.iter() {
        tuple.add_column_data(cs.name, create_data_from_column_type(&cs.col_type))
    }
    tuple
}


fn create_data_from_column_type<'a>(col_type: &'a ColumnType) -> ColumnData<'a> {
    let mut rng: ThreadRng = rand::thread_rng();
    match &col_type {
        ColumnType::Float => {
            ColumnData::Float(rng.gen_range(0.0..500.0))
        },
        ColumnType::Integer => {
            ColumnData::Integer(rng.gen_range(-1000..1000))
        },
        ColumnType::String => {
            ColumnData::String("placeholder")
        },
        ColumnType::List(v) => {
            let mut list = Vec::new();
            for _ in 0..10 {
                list.push(create_data_from_column_type(v))
            };
            ColumnData::List(list)
        },
        ColumnType::Record(v) => {
            let sub_tuple = create_data_from_schema_recurse(v, Tuple::new());
            ColumnData::Record(sub_tuple)
        },
    }
}
