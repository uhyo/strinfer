use crate::ast::Expression;
use crate::parser::ws;
use nom::branch::alt;
use nom::bytes::complete::escaped;
use nom::character::complete::char;
use nom::character::complete::none_of;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_nonempty_list;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

pub fn parse_expression<'a>(code: &'a str) -> IResult<&'a str, Expression<'a>> {
    parse_union(code)
}

fn parse_atomic_expression<'a>(code: &'a str) -> IResult<&'a str, Expression<'a>> {
    parse_string_literal(code)
}

fn parse_string_literal<'a>(code: &'a str) -> IResult<&'a str, Expression<'a>> {
    map(
        tuple((
            char('"'),
            escaped(none_of("\"\\\r\n"), '\\', none_of("\r\n")),
            char('"'),
        )),
        |(_, value, _)| Expression::StringLiteral { value },
    )(code)
}

fn parse_union<'a>(code: &'a str) -> IResult<&'a str, Expression<'a>> {
    map(
        preceded(
            opt(char('|')),
            separated_nonempty_list(ws(char('|')), parse_atomic_expression),
        ),
        |mut exprs| {
            if exprs.len() >= 1 {
                Expression::Union { exprs }
            } else {
                exprs.pop().unwrap()
            }
        },
    )(code)
}
