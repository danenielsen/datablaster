use crate::definition::schema::{FieldType, FieldDefinition, FieldSchema};
use crate::definition::schema::RecordSchema;
use std::fmt::Debug;
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
use nom::Finish;

#[macro_use]
mod helper;


fn obj_declaration(input: &str) -> IResult<&str, &str> {
    tag("table")(input)
}

fn token_named(input: &str) -> IResult<&str, &str> {
    // A named token is alphanumeric and underscores only
    take_while1(|c: char| {c.is_alphanumeric() || c == '_'})(input)
}

fn field_def(input: &str) -> IResult<&str, FieldSchema> {
    // Get the field_name
    let (i, field_name) = preceded(multispace0, token_named)(input)?;
    // Get the field_type
    let (i, field_type) = peek_parsed!(preceded(multispace1, field_type)(i))?;
    Ok((i, FieldSchema::new(field_name, field_type)))
}

fn field_type(input: &str) -> IResult<&str, FieldType> {
    // Get type name and turn it into a FieldType
    let (i, type_name) = token_named(input)?;
    match type_name {
        f if f.to_lowercase() == "string" => Ok((i, FieldType::String(Default::default()))),
        f if f.to_lowercase() == "integer" => Ok((i, FieldType::Integer(Default::default()))),
        f if f.to_lowercase() == "float" => Ok((i, FieldType::Float(Default::default()))),
        f if f.to_lowercase() == "list" => {
            let (i, field) = delimited(tag("("), field_type, tag(")"))(i)?;
            Ok((i, FieldType::List(Box::new(field))))
        },
        f => {
            error!("Unknown field type: {}", f);
            Err(Err::Error(Error::from_error_kind(input, ErrorKind::TakeUntil)))
        },
    }
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

fn table_record(input: &str) -> IResult<&str, RecordSchema> {
    debug!("Creating RecordSchema");
    let mut record = RecordSchema::new();
    let mut it = iterator(input, terminated(field_def, tag(",")));
    for f in &mut it {
        debug!("FieldSchema: {:?}", f);
        record.add_field(f);
    }
    let (field_input, _) = it.finish()?;
    preceded(multispace0, eof)(field_input)?;
    Ok(("", record))
}

pub fn parser(input: &str) -> IResult<&str, RecordSchema> {
    let (input, declaration_type) = preceded(multispace0, obj_declaration)(input)?;
    debug!("DECLARATION_TYPE: {}", declaration_type);
    let (input, table_name) = preceded(multispace1, token_named)(input)?;
    debug!("TABLE_NAME: {}", table_name);
    let (input, fields) = peek_parsed!(delimited(preceded(multispace0, tag("(")), take_till_delimiter_closed('(', ')'), preceded(multispace0, tag(")")))(input))?;
    let (_, record) = table_record(fields)?;
    let (input, _) = preceded(multispace0, tag(";"))(input)?;
    preceded(multispace0, eof)(input)?;
    Ok(("", record))
}

pub fn parse(input: &str) -> Result<RecordSchema, nom::error::Error<&str>> {
    let (_, record) = parser(input).finish()?;
    Ok(record)
}