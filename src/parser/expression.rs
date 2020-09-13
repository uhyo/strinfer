use crate::ast::Expression;
use crate::parser::ws;
use nom::branch::alt;
use nom::bytes::complete::escaped;
use nom::character::complete::char;
use nom::character::complete::none_of;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::separated_nonempty_list;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

pub fn parse_expression<'a>(code: &'a str) -> IResult<&'a str, Expression<'a>> {
    parse_union(code)
}

fn parse_union<'a>(code: &'a str) -> IResult<&'a str, Expression<'a>> {
    let parser = preceded(
        opt(char('|')),
        separated_nonempty_list(ws(char('|')), parse_atomic_expression),
    );
    map(parser, |mut exprs| {
        if exprs.len() >= 1 {
            Expression::Union { exprs }
        } else {
            exprs.pop().unwrap()
        }
    })(code)
}

fn parse_atomic_expression<'a>(code: &'a str) -> IResult<&'a str, Expression<'a>> {
    let parser = alt((parse_string_literal, parse_map));
    parser(code)
}

fn parse_string_literal<'a>(code: &'a str) -> IResult<&'a str, Expression<'a>> {
    map(parse_string, |value| Expression::StringLiteral { value })(code)
}

fn parse_map<'a>(code: &'a str) -> IResult<&'a str, Expression<'a>> {
    let parser = tuple((
        char('{'),
        many0(tuple((
            ws(parse_string),
            char(':'),
            ws(parse_expression),
            char(';'),
        ))),
        ws(char('}')),
    ));
    map(parser, |(_, pairs, _)| Expression::Map {
        pairs: pairs
            .into_iter()
            .map(|(key, _, value, _)| (key, value))
            .collect(),
    })(code)
}

fn parse_string<'a>(code: &'a str) -> IResult<&'a str, &'a str> {
    let parser = tuple((
        char('"'),
        escaped(none_of("\"\\\r\n"), '\\', none_of("\r\n")),
        char('"'),
    ));
    map(parser, |(_, value, _)| value)(code)
}
