extern crate clap;
use clap::{Arg, App, ArgMatches};

pub const RECORDS_TO_CREATE: &str = "count";
pub const OUTPUT_FILE: &str = "OUTPUT_FILE";

pub fn parse_args<'a>() -> ArgMatches<'a> {
    let matches = App::new("DataMe")
                            .about("Creates data")
                            .arg(Arg::with_name(RECORDS_TO_CREATE)
                                .short("r")
                                .long("records")
                                .help("Number of records to create")
                                .takes_value(true))
                            .arg(Arg::with_name(OUTPUT_FILE)
                                .help("Output file path")
                                .required(true))
                            .get_matches();
    matches
}
