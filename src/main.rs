mod args;
mod schema;
mod data_repr;
mod data_gen;
mod writer;

use schema::{FieldType, RecordSchema, FieldSchema};
use data_gen::*;

fn iterate_over_schema(schema: &RecordSchema, ) {
    iterate_over_schema_internal(schema, "")
}


fn iterate_over_schema_internal(schema: &RecordSchema, indent: &str) {
    for (i, col_schema) in schema.iter().enumerate() {
        match col_schema.get_type() {
            FieldType::Record(rs) => {
                println!("{}{}: ColumnSchema {{ name: \"{}\", col_type: \"Record\" }}", indent, i, col_schema.get_name());
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
        .with_column(FieldSchema::new("total", FieldType::Float(Default::default())))
        .with_column(FieldSchema::new("transaction_id", FieldType::Integer(Default::default())))
        .with_column(FieldSchema::new("line_items", FieldType::List(Box::new(FieldType::Record(
            RecordSchema::new()
                .with_column(FieldSchema::new("item", FieldType::String(Default::default())))
                .with_column(FieldSchema::new("sub_items", FieldType::Record(
                    RecordSchema::new()
                        .with_column(FieldSchema::new("name", FieldType::String(Default::default())))
                        .with_column(FieldSchema::new("amount", FieldType::Integer(Default::default())))
                        .with_column(FieldSchema::new("cost", FieldType::Float(Default::default())))
                )))
                .with_column(FieldSchema::new("amount", FieldType::Integer(Default::default())))
                .with_column(FieldSchema::new("cost", FieldType::Float(Default::default())))
        )))))
        .with_column(FieldSchema::new("sales_agents", FieldType::List(Box::new(FieldType::String(Default::default())))))
        .with_column(FieldSchema::new("team", FieldType::Record(
            RecordSchema::new()
                .with_column(FieldSchema::new("project_manager", FieldType::String(Default::default())))
                .with_column(FieldSchema::new("team_members", FieldType::List(Box::new(FieldType::String(Default::default())))))
                .with_column(FieldSchema::new("budget", FieldType::Float(Default::default())))
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
