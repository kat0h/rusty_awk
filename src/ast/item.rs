/*
 * file: item.rs
 * author: kota kato 2022
 * description:
 *   Parse item list
 */

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

use crate::ast::{def::*, expr::parse_expr, stmt::parse_statement, util::*};

pub fn parse_item(input: &str) -> IResult<&str, AWKItem> {
    alt((
        // Pattern Action
        map(
            tuple((
                alt((parse_special_pattern, parse_pattern)),
                wss,
                parse_action,
            )),
            |(pattern, _, action)| AWKItem::PatternAction(AWKPatternAction { pattern, action }),
        ),
        // Pattern only
        // print $0
        map(parse_pattern, |pattern| {
            AWKItem::PatternAction(AWKPatternAction {
                pattern,
                action: vec![AWKStat::Print(AWKPrint {
                    exprlist: vec![AWKExpr::Field(Box::new(AWKExpr::Value(AWKVal::Num(0.0))))],
                })],
            })
        }),
        // Action only
        map(parse_action, |action| {
            AWKItem::PatternAction(AWKPatternAction {
                pattern: AWKPattern::Always,
                action,
            })
        }),
    ))(input)
}

fn parse_pattern(input: &str) -> IResult<&str, AWKPattern> {
    map(parse_expr, AWKPattern::Expr)(input)
}

fn parse_special_pattern(input: &str) -> IResult<&str, AWKPattern> {
    let (input, tag) = alt((tag("BEGIN"), tag("END")))(input)?;
    let tag = match tag {
        "BEGIN" => AWKPattern::Begin,
        "END" => AWKPattern::End,
        _ => unreachable!(),
    };
    Ok((input, tag))
}

pub fn parse_action(input: &str) -> IResult<&str, Vec<AWKStat>> {
    fn parse_terminate(input: &str) -> IResult<&str, ()> {
        let (input, _) =
            tuple((wss, alt((char(';'), nl)), many0(alt((char(';'), ws, nl)))))(input)?;
        Ok((input, ()))
    }
    fn parse_statement_list(input: &str) -> IResult<&str, Vec<AWKStat>> {
        let (input, ret) = separated_list0(parse_terminate, parse_statement)(input)?;
        let (input, _) = opt(alt((parse_terminate, map(wss, |_| ()))))(input)?;

        Ok((input, ret))
    }
    delimited(
        char('{'),
        map(
            tuple((opt(many0(alt((char(';'), ws, nl)))), parse_statement_list)),
            |(_, list): (_, Vec<AWKStat>)| list,
        ),
        char('}'),
    )(input)
}

#[test]
fn test_parse_item() {
    let a = parse_item("{}");
    let e = AWKItem::PatternAction(AWKPatternAction {
        pattern: AWKPattern::Always,
        action: vec![],
    });
    assert_eq!(Ok(("", e)), a);

    let a = parse_item("BEGIN{}");
    let e = AWKItem::PatternAction(AWKPatternAction {
        pattern: AWKPattern::Begin,
        action: vec![],
    });
    assert_eq!(Ok(("", e)), a);

    // white space -> OK NEWLINE -> NG
    assert!(parse_item("BEGIN {}").is_ok());
    assert!(parse_item("BEGIN \n{}").is_err());

    assert_eq!(parse_item(r#"1"#), parse_item(r#"1{print $0}"#),);
}

#[test]
fn test_parse_action() {
    let expect = parse_action("{print(\"hoge\");1+2;print(23)}");

    assert!(expect.is_ok());
    assert_eq!(
        expect,
        parse_action(
            r#"{ ;;

                print("hoge");

                1+2

                ;
                ;

                ;


                print(23)
                ; }"#
        )
    );
    assert_eq!(
        expect,
        parse_action(r#"{  print("hoge"); 1+2 ; ; ; print(23) ; }"#)
    );
    assert_eq!(
        expect,
        parse_action(
            r#"{  print("hoge"); 1+2  
                     print(23)  }"#
        )
    );

    let mut all = nom::combinator::all_consuming(parse_action);
    assert!(all(" { print(\"hoge\")}").is_err());
    assert!(all("{ print(\"hoge\")} ").is_err());
}
