use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while},
    character::{complete::multispace0, is_newline, is_space},
    multi::many0,
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    Single(String),
    Named(String, String),
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Data {
    Array(Vec<Item>),
    Scalar(String),
}

pub struct Wiki {
    pub kind: String,
    pub data: HashMap<String, Data>,
}

pub fn item(inp: &str) -> IResult<&str, Item> {
    let (inp, inner) = delimited(tag("["), take_until("]"), tag("]")).parse(inp)?;
    let (v, name) = take_till(|c| c == '|').parse(inner)?;
    dbg!(v, name);
    let item = match (name, v) {
        ("", "") => Item::None,
        (name, "") => Item::Single(name.to_string()), // `|` doesn't exist
        (name, v) => Item::Named(name.to_string(), v.trim_start_matches('|').to_string()),
    };
    Ok((inp, item))
}

fn array(inp: &str) -> IResult<&str, Vec<Item>> {
    dbg!(inp);
    many0(preceded(multispace0, item)).parse(inp)
}

pub fn data(inp: &str) -> IResult<&str, Data> {
    alt((
        delimited(tag("{"), take_until("}"), tag("}"))
            .and_then(array)
            .map(Data::Array),
        take_till(|c| c == '\n')
            .map(ToString::to_string)
            .map(Data::Scalar),
    ))
    .parse(inp)
}

fn is_ws(c: char) -> bool {
    // may panic since c is not guaranteened an ASCII character
    is_space(c as u8) || is_newline(c as u8)
}

pub fn info(inp: &str) -> IResult<&str, (String, Data)> {
    let (inp, key) = tag("|")
        .and_then(terminated(
            take_till(|c| c == '=' || is_ws(c)),
            take_while(|c| c == '=' || is_ws(c)),
        ))
        .parse(inp)?;
    let (inp, value) = data.parse(inp)?;
    Ok((inp, (key.to_string(), value)))
}

pub fn wiki(inp: &str) -> IResult<&str, Wiki> {
    let (inp, content) = delimited(tag("{{"), take_until("}}"), tag("}}")).parse(inp)?;
    let (data, kind) = take_until("\n").parse(content)?;
    let (_, infos) = many0(preceded(take_while(is_ws), info)).parse(data)?;
    Ok((
        inp,
        Wiki {
            kind: kind.to_string(),
            data: HashMap::from_iter(infos),
        },
    ))
}
