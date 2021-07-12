use crate::schema::{FieldType, RecordSchema};
use crate::data_repr::{ColumnData, Tuple};


pub fn create_data_from_schema(schema: &RecordSchema) -> Tuple {
    let tuple = Tuple::new();
    create_data_from_schema_recurse(schema, tuple)
}


fn create_data_from_schema_recurse<'a>(schema: &'a RecordSchema, mut tuple: Tuple) -> Tuple {
    for cs in schema.iter() {
        tuple.add_column_data(cs.get_name(), create_data_from_column_type(cs.get_type()))
    }
    tuple
}


fn create_data_from_column_type<'a>(col_type: &'a FieldType) -> ColumnData {
    match &col_type {
        FieldType::Float(def) => {
            ColumnData::Float(def.generate())
        },
        FieldType::Integer(def) => {
            ColumnData::Integer(def.generate())
        },
        FieldType::String(def) => {
            ColumnData::String(def.generate())
        },
        FieldType::List(v) => {
            let mut list = Vec::new();
            for _ in 0..4 {
                list.push(create_data_from_column_type(v))
            };
            ColumnData::List(list)
        },
        FieldType::Record(v) => {
            let sub_tuple = create_data_from_schema_recurse(v, Tuple::new());
            ColumnData::Record(sub_tuple)
        },
    }
}
