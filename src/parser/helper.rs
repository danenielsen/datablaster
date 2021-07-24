macro_rules! peek_parsed {
    ($parser_result:expr) => {{
        let log_seperator: &str = "<========================>";
        let mid_seperator: &str = "--------------------------";
        match $parser_result {
            Ok((input, matched)) => trace!(
                "\n{}\n{}\n{}\nmatched: ```{:?}```\nremaining: ```{}```\n{}",
                log_seperator,
                stringify!($parser_result),
                mid_seperator,
                matched,
                input,
                log_seperator
            ),
            Err(e) => trace!(
                "\n{}\n{}\n{}\nERROR: {}\n{}",
                log_seperator,
                stringify!($parser_result),
                mid_seperator,
                e,
                log_seperator
            ),
        };
        $parser_result
    }};
}
