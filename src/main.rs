mod args;
mod data_gen;
mod data_repr;
mod definition;
mod parser;
mod writer;

use data_gen::*;
use definition::schema::{FieldType, RecordSchema};
use env_logger::fmt::Formatter;
use log::LevelFilter;
use log::Record;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use parser::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::str;
use writer::csv::TupleToCSVSerializer;
use writer::json::TupleToJsonSerializer;
use writer::*;

fn iterate_over_schema(schema: &RecordSchema) {
    iterate_over_schema_internal(schema, "")
}

pub fn iterate_over_schema_internal(schema: &RecordSchema, indent: &str) {
    for (i, col_schema) in schema.iter().enumerate() {
        match col_schema.get_type() {
            FieldType::Record(rs) => {
                info!(
                    "{}{}: ColumnSchema {{ name: \"{}\", col_type: \"Record\" }}",
                    indent,
                    i,
                    col_schema.get_name()
                );
                iterate_over_schema_internal(rs, [indent, "  "].join("").as_str())
            }
            _ => info!("{}{}: {:?}", indent, i, col_schema),
        };
    }
}

fn main() {
    let matches = args::parse_args();
    let output_file_format = matches.value_of(args::FORMAT).unwrap(); //required
    let output_file = matches.value_of(args::OUTPUT_FILE).unwrap(); //required
    let schema_file = matches.value_of(args::SCHEMA).unwrap(); //required
    let number_of_records = if let Some(nr) = matches.value_of(args::RECORDS_TO_CREATE) {
        nr.parse::<i64>().unwrap_or(10)
    } else {
        10
    };

    // Init logger
    let log_level = match matches.occurrences_of(args::VERBOSE) {
        0 => LevelFilter::Info,  // No verbose
        1 => LevelFilter::Debug, // -v
        _ => LevelFilter::Trace, // -vv
    };
    env_logger::Builder::from_default_env()
        .format(|buf: &mut Formatter, record: &Record| {
            write!(buf, "[{}", buf.timestamp_seconds())?;
            let level_style = buf.default_level_style(record.level());
            write!(buf, " {}", level_style.value(record.level()))?;
            match (record.module_path(), record.line()) {
                (Some(module_path), Some(line)) => write!(buf, " {}:{}", module_path, line)?,
                (Some(module_path), _) => write!(buf, " {}", module_path)?,
                _ => (),
            };
            writeln!(buf, "] {}", record.args())
        })
        .filter(None, log_level)
        .init();
    info!("info");
    debug!("debug");
    trace!("trace");

    let schema_file_string = fs::read_to_string(schema_file).unwrap();
    let parse_result = parse(&schema_file_string);

    let schema = match parse_result {
        Ok(r) => r,
        Err(e) => panic!("\nParse Error: {:?}\non input: ```{}```", e.code, e.input),
    };

    /*
    let schema = RecordSchema::new()
        .with_field(FieldSchema::new("total", FieldType::Float(Default::default())))
        .with_field(FieldSchema::new("transaction_id", FieldType::Integer(Default::default())))
        .with_field(FieldSchema::new("line_items", FieldType::List(Box::new(FieldType::Record(
            RecordSchema::new()
                .with_field(FieldSchema::new("item", FieldType::String(Default::default())))
                .with_field(FieldSchema::new("sub_items", FieldType::Record(
                    RecordSchema::new()
                        .with_field(FieldSchema::new("name", FieldType::String(Default::default())))
                        .with_field(FieldSchema::new("amount", FieldType::Integer(Default::default())))
                        .with_field(FieldSchema::new("cost", FieldType::Float(Default::default())))
                )))
                .with_field(FieldSchema::new("amount", FieldType::Integer(Default::default())))
                .with_field(FieldSchema::new("cost", FieldType::Float(Default::default())))
        )))))
        .with_field(FieldSchema::new("sales_agents", FieldType::List(Box::new(FieldType::String(Default::default())))))
        .with_field(FieldSchema::new("team", FieldType::Record(
            RecordSchema::new()
                .with_field(FieldSchema::new("project_manager", FieldType::String(Default::default())))
                .with_field(FieldSchema::new("team_members", FieldType::List(Box::new(FieldType::String(Default::default())))))
                .with_field(FieldSchema::new("budget", FieldType::Float(Default::default())))
            )
        ))
    ;

    let schema = RecordSchema::new()
        .with_field(FieldSchema::new("id", FieldType::Integer(Default::default())))
        .with_field(FieldSchema::new("fname", FieldType::String(FieldDefinition::new(Box::new(DataFunctionGenerator::new(|| "fname".to_string()))))))
        .with_field(FieldSchema::new("lname", FieldType::String(FieldDefinition::new(Box::new(DataFunctionGenerator::new(|| "lname".to_string()))))))
        .with_field(FieldSchema::new("salary", FieldType::Integer(Default::default())))
    ;
    */

    let tuple_serializer: Box<dyn TupleSerializer> = match output_file_format {
        "csv" => Box::new(TupleToCSVSerializer::new()),
        "json" => Box::new(TupleToJsonSerializer::new(false)),
        _ => panic!("Unknown output format: {}", output_file_format),
    };
    if schema.contains_record() && !tuple_serializer.supports_record() {
        panic!("Records not supported")
    }
    if schema.contains_list() && !tuple_serializer.supports_list() {
        panic!("Lists not supported")
    }
    let file_writer = writer::FileWriter::new(tuple_serializer);

    let mut file = File::create(output_file).expect("Couldn't open file");
    info!("Writing out to file");
    let mut next_print = 1;
    for i in 0..number_of_records {
        let output_data = create_data_from_schema(&schema);
        file_writer.write_to_file(&output_data, &mut file);
        if next_print <= i + 1 {
            info!("Wrote {} records to file", i + 1);
            next_print *= 10;
        }
    }

    info!("{} records written to file", number_of_records);
}
