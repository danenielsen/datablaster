mod args;
mod schema;
mod data_repr;
mod data_gen;
mod writer;

use schema::{ColumnType, RecordSchema, ColumnSchema};
use data_gen::*;

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

    let schema = RecordSchema::new()
        .with_column(ColumnSchema::new("total", ColumnType::Float))
        .with_column(ColumnSchema::new("transaction_id", ColumnType::Integer))
        .with_column(ColumnSchema::new("line_items", ColumnType::List(Box::new(
            ColumnType::Record(
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
        ))
        .with_column(ColumnSchema::new("sales_agents", ColumnType::List(Box::new(ColumnType::String))))
        .with_column(ColumnSchema::new("team", ColumnType::Record(
            RecordSchema::new()
                .with_column(ColumnSchema::new("project_manager", ColumnType::String))
                .with_column(ColumnSchema::new("team_members", ColumnType::List(Box::new(ColumnType::String))))
                .with_column(ColumnSchema::new("budget", ColumnType::Float))
            )
        ))
    ;

    println!("Iterating");
    iterate_over_schema(&schema);

    println!("{:?}\n\n", schema);

    for _ in 0..number_of_records {
        let output_data = create_data_from_schema(&schema);
        println!("Raw: {:?}", output_data);
        println!("json: {}", writer::to_pretty_json_data(&output_data))
    }
    
}
