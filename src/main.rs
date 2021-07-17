mod args;
mod schema;
mod data_repr;
mod data_gen;
mod writer;

use env_logger::Env;
#[allow(unused_imports)]
use log::{info, error, trace, warn};
use schema::{FieldType, RecordSchema, FieldSchema, DataFunctionGenerator, FieldDefinition};
use std::fs::File;
use data_gen::*;
use writer::json::TupleToJsonSerializer;
use writer::csv::TupleToCSVSerializer;

fn iterate_over_schema(schema: &RecordSchema, ) {
    iterate_over_schema_internal(schema, "")
}


pub fn iterate_over_schema_internal(schema: &RecordSchema, indent: &str) {
    for (i, col_schema) in schema.iter().enumerate() {
        match col_schema.get_type() {
            FieldType::Record(rs) => {
                info!("{}{}: ColumnSchema {{ name: \"{}\", col_type: \"Record\" }}", indent, i, col_schema.get_name());
                iterate_over_schema_internal(rs, [indent, "  "].join("").as_str())
            },
            _ => info!("{}{}: {:?}", indent, i, col_schema),
        };
    }
}


fn main() {
    // Init logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let matches = args::parse_args();
    let output_file = matches.value_of(args::OUTPUT_FILE).unwrap();
    let number_of_records =
        if let Some(nr) = matches.value_of(args::RECORDS_TO_CREATE) {
            nr.parse::<i64>().unwrap_or(10)
        } else {
            10
        };

    /*
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
    */

    let schema = RecordSchema::new()
        .with_column(FieldSchema::new("id", FieldType::Integer(Default::default())))
        .with_column(FieldSchema::new("fname", FieldType::String(FieldDefinition::new(Box::new(DataFunctionGenerator::new(|| "fname".to_string()))))))
        .with_column(FieldSchema::new("lname", FieldType::String(FieldDefinition::new(Box::new(DataFunctionGenerator::new(|| "lname".to_string()))))))
        .with_column(FieldSchema::new("salary", FieldType::Integer(Default::default())))
    ;

    iterate_over_schema(&schema);

    info!("{:?}\n\n", schema);

    let mut file = File::create(output_file).expect("Couldn't open file");
    let file_writer = writer::FileWriter::new(TupleToCSVSerializer::new());
    info!("Writing out to file");
    let mut next_print = 1;
    for i in 0..number_of_records {
        let output_data = create_data_from_schema(&schema);
        file_writer.write_to_file(&output_data, &mut file);
        if next_print <= i + 1 {
            info!("Wrote {} records to file", i+1);
            next_print *= 10;
        }
    }

    info!("{} records written to file", number_of_records);
}
