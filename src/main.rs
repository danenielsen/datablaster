mod args;
mod data_gen;

fn main() {
    let matches = args::parse_args();
    let number_of_records =
        if let Some(nr) = matches.value_of(args::RECORDS_TO_CREATE) {
            nr.parse::<i64>().unwrap_or(10)
        } else {
            10
        };

    for x in 0..number_of_records {
        println!("{} Hello, world!", x);
    }
    
}
