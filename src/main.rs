mod args;
mod data_gen;

use std::fmt::Display;
use std::fmt::Debug;
use data_gen::{ColumnType, RecordSchema, ColumnSchema};

fn print_column<T: Display>(column: &data_gen::Column<T>) {
    println!("Column {} has data {} of type {}", column.name, column.data, std::any::type_name::<T>())
}

fn print_column_debug<T: Debug>(column: &data_gen::Column<T>) {
    println!("Column {} has data {:?} of type {}", column.name, column.data, std::any::type_name::<T>())
}

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

fn main() {
    let matches = args::parse_args();
    let number_of_records =
        if let Some(nr) = matches.value_of(args::RECORDS_TO_CREATE) {
            nr.parse::<i64>().unwrap_or(10)
        } else {
            10
        };

    let column1 = data_gen::Column::new("column1", 64);
    print_column(&column1);

    let column2_name = "column2".to_string();
    let column2 = data_gen::Column::new(column2_name.as_str(), "test");
    print_column(&column2);

    let column3 = data_gen::Column::new("column1", 4.3);
    print_column(&column3);

    let column4 = data_gen::Column::new("column1", vec!["test", "one", "two"]);
    print_column_debug(&column4);

    let column5 = data_gen::Column::new("column1", 64);
    print_column(&column5);

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
    ;

    let second_schema = schema.clone().with_column(ColumnSchema::new("customer", ColumnType::String));


    
    println!("Iterating");
    iterate_over_schema(&schema);

    println!("{:?}", schema);
    println!("{:?}", second_schema);

    //for x in 0..number_of_records {
    //    println!("{} Hello, world!", x);
    //}
    
}
