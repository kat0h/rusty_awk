/*
 * file: value.rs
 * author: kota kato 2022
 * description:
 *   value
 */

use crate::ast::def::{AWKFloat, AWKStr, AWKVal};
use crate::ast::{number::parse_number, string::parse_string};
use nom::{branch::alt, combinator::map, IResult};

pub fn parse_value(input: &str) -> IResult<&str, AWKVal> {
    alt((
        map(parse_number, |n: AWKFloat| -> AWKVal { AWKVal::Num(n) }),
        map(parse_string, |s: AWKStr| -> AWKVal { AWKVal::Str(s) }),
    ))(input)
}

#[test]
fn test_parse_value() {
    assert_eq!(
        Ok(("", AWKVal::Num(parse_number("123").unwrap().1))),
        parse_value("123")
    );
    assert_eq!(
        Ok(("", AWKVal::Str(parse_string("\"hoge\"").unwrap().1))),
        parse_value("\"hoge\"")
    );

    let mut all = nom::combinator::all_consuming(parse_value);
    assert!(all("123 ").is_err());
    assert!(all(" 123").is_err());
}
