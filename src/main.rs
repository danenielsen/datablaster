mod args;
mod data_gen;
mod data_repr;
mod definition;
mod parser;
mod writer;

use data_gen::*;
use env_logger::fmt::Formatter;
use log::LevelFilter;
use log::Record;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use parser::*;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use writer::csv::TupleToCSVSerializer;
use writer::json::TupleToJsonSerializer;
use writer::*;

fn run() -> Result<(), Box<dyn Error>> {
    let matches = args::parse_args();

    // These arguments are required, so we can safely unwrap them without checking
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

    let schema_file_string = fs::read_to_string(schema_file)?;
    let schema = parse(&schema_file_string)?;

    //let schema = parse_result.map_err(|e| format!("\nParse Error: {:?}\non input: ```{}```", e.code, e.input))?;

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

    let file = File::create(output_file)?;
    let mut tuple_serializer: Box<dyn TupleWriter> = match output_file_format {
        "csv" => Box::new(TupleToCSVSerializer::new(file)),
        "json" => Box::new(TupleToJsonSerializer::new(file, false)),
        _ => return Err(format!("Unknown output format: {}", output_file_format).into()),
    };
    if schema.contains_record() && !tuple_serializer.supports_record() {
        return Err("Records not supported".into());
    }
    if schema.contains_list() && !tuple_serializer.supports_list() {
        return Err("Lists not supported".into());
    }

    info!("Writing out to file");
    let mut next_print = 1;
    for i in 0..number_of_records {
        let output_data = create_data_from_schema(&schema);
        if let Err(e) = tuple_serializer.write_tuple(&output_data) {
            return Err(format!("Error writing tuple: {}", e).into());
        };
        if next_print <= i + 1 {
            info!("Wrote {} records to file", i + 1);
            next_print *= 10;
        }
    }
    tuple_serializer.flush()?;

    info!("{} records written to file", number_of_records);
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1)
        }
    }
}
