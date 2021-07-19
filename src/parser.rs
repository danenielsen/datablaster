
use nom::{
    IResult,
    bytes::complete::{tag, is_not, take_while_m_n, take_while1},
    character::complete::{multispace0, multispace1},
    sequence::{preceded, delimited, tuple, terminated},
    combinator::{map_res, eof, iterator},
    multi::{many1},
    error::{Error, ErrorKind, ParseError},
    Err,
  };
use log::{info, debug, error, trace};
use std::str;

const LOG_SEPERATOR: &str = "<========================>";


pub fn peek_parsed<'a>(res: IResult<&'a str, &'a str>) -> IResult<&'a str, &'a str> {
    debug!("{}", parse_result_to_string(&res));
    res
}

pub fn parse_result_to_string(res: &IResult<&str, &str>) -> String {
    match res {
        Ok((input, matched)) => parse_pieces_to_string(input, matched),
        Err(e) => format!("{}\nParsing Error: {}\n{}", LOG_SEPERATOR, e, LOG_SEPERATOR),
    }
}

pub fn parse_pieces_to_string(input: &str, matched: &str) -> String {
    format!("\n{}\nmatched: ```{}```\nremaining: ```{}```\n{}", LOG_SEPERATOR, matched, input, LOG_SEPERATOR)
}

fn obj_declaration(input: &str) -> IResult<&str, &str> {
    trace!("obj_declaration");
    tag("table")(input)
}

fn token_named(input: &str) -> IResult<&str, &str> {
    trace!("token_named");
    take_while1(|c: char| {c.is_alphanumeric() || c == '_'})(input)
}

fn field_def(input: &str) -> IResult<&str, &str> {
    trace!("field_def");
    let (input, field_name) = peek_parsed(preceded(multispace0, token_named)(input))?;
    peek_parsed(preceded(multispace1, token_named)(input))
}


fn take_till_delimiter_closed(opening_delim: char, closing_delim: char) -> impl FnMut(&str) -> IResult<&str, &str> {
    move |input| {
        let mut delim_count = 0;
        let mut ending_index = 0;
        for (i, c) in input.chars().enumerate() {
            ending_index = i;
            match c {
                c if c == closing_delim => if delim_count == 0 {
                    break;
                } else {
                    delim_count += 1;
                },
                c if c == opening_delim => delim_count -= 1,
                _ => (),
            }
        }
        if delim_count != 0 {
            Err(Err::Error(Error::from_error_kind(input, ErrorKind::TakeUntil)))
        } else {
            Ok((&input[ending_index..], &input[..ending_index]))
        }
    }
}

pub fn parser(input: &str) -> IResult<&str, &str> {
    let (input, declaration_type) = preceded(multispace0, obj_declaration)(input)?;
    debug!("DECLARATION_TYPE: {}", declaration_type);
    let (input, table_name) = preceded(multispace1, token_named)(input)?;
    debug!("TABLE_NAME: {}", table_name);
    let (input, fields) = peek_parsed(delimited(preceded(multispace0, tag("(")), take_till_delimiter_closed('(', ')'), preceded(multispace0, tag(")")))(input))?;
    let mut it = iterator(fields, terminated(field_def, tag(",")));
    for f in &mut it {
        debug!("{}", f);
    }
    debug!("{:?}", it.finish());

    let (input, _) = preceded(multispace0, tag(";"))(input)?;
    preceded(multispace0, eof)(input)
}