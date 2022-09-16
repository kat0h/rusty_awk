/*
 * file: item.rs
 * author: kota kato 2022
 * description:
 *   Parse item list
 */

use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

use crate::ast::def::{AWKItem, AWKPattern, AWKPatternAction, AWKStatement};

use super::statement::parse_statement;

/*
 * action
 * pattern action
 * normal_pattern TODO
 */
pub fn parse_item(input: &str) -> IResult<&str, AWKItem> {
    alt((
        map(parse_action, |action: Vec<AWKStatement>| {
            AWKItem::AWKPatternAction(AWKPatternAction {
                pattern: AWKPattern::Always,
                action,
            })
        }),
        map(
            permutation((parse_pattern, parse_action)),
            |(pattern, action): (AWKPattern, Vec<AWKStatement>)| {
                AWKItem::AWKPatternAction(AWKPatternAction { pattern, action })
            },
        ),
    ))(input)
}

fn parse_action(input: &str) -> IResult<&str, Vec<AWKStatement>> {
    delimited(
        char('{'),
        separated_list0(char(';'), parse_statement),
        char('}'),
    )(input)
}

fn parse_pattern(input: &str) -> IResult<&str, AWKPattern> {
    parse_special_pattern(input)
}

fn parse_normal_pattern(_input: &str) -> IResult<&str, AWKPattern> {
    unimplemented!()
}

fn parse_special_pattern(input: &str) -> IResult<&str, AWKPattern> {
    let (input, tag) = alt((tag("BEGIN"), tag("END")))(input)?;
    let tag = match tag {
        "BEGIN" => AWKPattern::Begin,
        _ => AWKPattern::End,
    };
    return Ok((input, tag));
}

#[test]
fn test_parse_item() {
    let a = parse_item("{}");
    let e = AWKItem::AWKPatternAction(AWKPatternAction {
        pattern: AWKPattern::Always,
        action: vec![],
    });
    assert_eq!(Ok(("", e)), a);

    let a = parse_item("BEGIN{}");
    let e = AWKItem::AWKPatternAction(AWKPatternAction {
        pattern: AWKPattern::Begin,
        action: vec![],
    });
    assert_eq!(Ok(("", e)), a);
}