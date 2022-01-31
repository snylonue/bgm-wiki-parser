use crate::{Data, Item, Wiki};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while},
    character::{complete::multispace0, is_newline, is_space},
    combinator::rest,
    multi::many0,
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};
use std::collections::HashMap;

pub fn item(inp: &str) -> IResult<&str, Item> {
    let (inp, inner) = delimited(
        tag("["),
        alt((take_until("]\n"), take_until("]\r\n"), take_until("]"))),
        tag("]"),
    )
    .parse(inp)?;
    let (v, name) = take_till(|c| c == '|').parse(inner)?;
    let item = match (name, v) {
        ("", "") => Item::None,
        (name, "") => Item::Single(name.trim().to_string()), // `|` doesn't exist
        (name, v) => Item::Named(
            name.trim().to_string(),
            v.trim_start_matches('|').trim().to_string(),
        ),
    };
    Ok((inp, item))
}

fn array(inp: &str) -> IResult<&str, Vec<Item>> {
    many0(preceded(multispace0, item)).parse(inp)
}

fn take_till_newline(inp: &str) -> IResult<&str, &str> {
    alt((take_until("\r\n"), take_until("\n"), rest)).parse(inp)
}

pub fn data(inp: &str) -> IResult<&str, Data> {
    alt((
        delimited(tag("{"), take_until("}"), tag("}"))
            .and_then(array)
            .map(Data::Array),
        take_till_newline.map(ToString::to_string).map(Data::Scalar),
    ))
    .parse(inp)
}

fn is_ws(c: char) -> bool {
    // may panic since c is not guaranteened an ASCII character
    is_space(c as u8) || is_newline(c as u8)
}

pub fn info(inp: &str) -> IResult<&str, (String, Data)> {
    let (inp, key) = preceded(
        tag("|").and(take_while(is_ws)),
        terminated(
            take_till(|c| c == '=' || is_ws(c)),
            take_while(|c| c == '=' || is_ws(c)),
        ),
    )
    .parse(inp)?;
    let (inp, value) = data.parse(inp)?;
    Ok((inp, (key.to_string(), value)))
}

fn tag_newline(inp: &str) -> IResult<&str, &str> {
    alt((tag("\r\n"), tag("\n"))).parse(inp)
}

pub fn wiki(inp: &str) -> IResult<&str, Wiki> {
    if inp.is_empty() {
        return Ok(("", Wiki::default()));
    }
    let (inp, content) = delimited(
        tag("{{").and(take_while(is_ws)).and(tag("Infobox")),
        take_until("}}"),
        tag("}}"),
    )
    .parse(inp)?;
    let (data, kind) = take_till_newline.parse(content)?;
    let (_, infos) = many0(preceded(many0(tag_newline), info)).parse(data)?;
    Ok((
        inp,
        Wiki {
            kind: kind.trim().to_string(),
            data: HashMap::from_iter(infos),
        },
    ))
}
