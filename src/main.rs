mod args;
mod data_gen;

use std::fmt::Display;
use std::fmt::Debug;
use rand::prelude::*;
use data_gen::{ColumnType, RecordSchema, ColumnSchema, ColumnData, Tuple};

fn iterate_over_schema(schema: &RecordSchema, ) {
    iterate_over_schema_internal(schema, "")
}

fn iterate_over_schema_internal(schema: &RecordSchema, indent: &str) {
    for (i, col_schema) in schema.iter().enumerate() {
        match &col_schema.col_type {
            ColumnType::Record(rs) => {
                println!("{}{}: ColumnSchema {{ name: \"{}\", col_type: \"Record\" }}", indent, i, col_schema.name);
                iterate_over_schema_internal(rs, [indent, "  "].join("").as_str())
            },
            _ => println!("{}{}: {:?}", indent, i, col_schema),
        };
    }
}

fn create_data_from_schema<'a>(schema: &'a RecordSchema<'a>) -> Tuple<'a> {
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
    match &col_type {
        ColumnType::Float => {
            ColumnData::Float(rand::random())
        },
        ColumnType::Integer => {
            ColumnData::Integer(rand::random())
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

fn main() {
    let matches = args::parse_args();
    let number_of_records =
        if let Some(nr) = matches.value_of(args::RECORDS_TO_CREATE) {
            nr.parse::<i64>().unwrap_or(10)
        } else {
            10
        };

    let schema = RecordSchema::new()
        .with_column(ColumnSchema::new("total", ColumnType::Float))
        .with_column(ColumnSchema::new("transaction_id", ColumnType::Integer))
        .with_column(ColumnSchema::new("line_items", ColumnType::Record(
            RecordSchema::new()
                .with_column(ColumnSchema::new("item", ColumnType::String))
                .with_column(ColumnSchema::new("sub_items", ColumnType::Record(
                    RecordSchema::new()
                        .with_column(ColumnSchema::new("name", ColumnType::String))
                        .with_column(ColumnSchema::new("amount", ColumnType::Integer))
                        .with_column(ColumnSchema::new("cost", ColumnType::Float))
                )))
                .with_column(ColumnSchema::new("amount", ColumnType::Integer))
                .with_column(ColumnSchema::new("cost", ColumnType::Float))
        )))
        .with_column(ColumnSchema::new("sales_agents", ColumnType::List(Box::new(ColumnType::String))))
        .with_column(ColumnSchema::new("team", ColumnType::List(Box::new(
            ColumnType::Record(
                RecordSchema::new()
                .with_column(ColumnSchema::new("project_manager", ColumnType::String))
                .with_column(ColumnSchema::new("team_members", ColumnType::List(Box::new(ColumnType::String))))
                .with_column(ColumnSchema::new("budget", ColumnType::Float))
            )
        ))))
    ;

    println!("Iterating");
    iterate_over_schema(&schema);

    println!("{:?}\n\n", schema);

    for _ in 0..number_of_records {
        let output_data = create_data_from_schema(&schema);
        println!("{:?}", output_data);
    }
    
}
