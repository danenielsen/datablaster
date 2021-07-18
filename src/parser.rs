
use nom::{
    IResult,
    bytes::complete::{tag, is_not, take_while_m_n, take_while1},
    character::complete::{multispace0, multispace1},
    sequence::{preceded, delimited, tuple},
    combinator::{map_res, eof, iterator},
    multi::{many1},
  };
use log::{info, error};
use std::str;

const LOG_SEPERATOR: &str = "<========================>";

pub fn print_parse_result(res: &IResult<&[u8], &[u8]>) {
    match res {
        Ok((input, matched)) => print_parse_pieces(input, matched),
        Err(e) => error!("{}\nParsing Error: {}", LOG_SEPERATOR, e),
    }
}

pub fn peek_parsed<'a>(res: IResult<&'a [u8], &'a [u8]>) -> IResult<&'a [u8], &'a [u8]> {
    print_parse_result(&res);
    res
}


pub fn print_parse_pieces(input: &[u8], matched: &[u8]) {
    info!("{}\nmatched: '{}'\nremaining: ```{}```", LOG_SEPERATOR, str::from_utf8(matched).unwrap(), str::from_utf8(input).unwrap());
}

fn obj_declaration(input: &str) -> IResult<&str, &str> {
    tag("table")(input)
}

fn token_named(input: &str) -> IResult<&str, &str> {
    take_while1(char::is_alphanumeric)(input)
}

fn field_def(input: &str) -> IResult<&str, &str> {
    let (input, field_name) = preceded(multispace0, token_named)(input)?;
    preceded(multispace1, token_named)(input)
}

/*
fn take_till_closing_delimited(opening_delim: char, closing_delim: char) -> impl FnMut(&str) -> IResult<&str, &str> {
    unimplemented!()
}
*/

pub fn parser(input: &str) -> IResult<&str, &str> {
    let (input, declaration_type) = preceded(multispace0, obj_declaration)(input)?;
    let (input, table_name) = preceded(multispace1, token_named)(input)?;
    let (input, matched) = delimited(preceded(multispace0, tag("(")), many1(is_not(")")), preceded(multispace0, tag(")")))(input)?;
    eof(input)
}