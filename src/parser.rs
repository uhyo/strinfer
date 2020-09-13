use crate::ast::Program;
use crate::ast::Statement;
use crate::parser::expression::parse_expression;
use crate::parser::util::ws;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{bytes::complete::tag, IResult};

mod expression;
mod util;

pub fn parse<'a>(code: &'a str) -> IResult<&'a str, Program<'a>> {
    all_consuming(many0(ws(parse_statement)))(code)
}

fn parse_statement<'a>(code: &'a str) -> IResult<&'a str, Statement<'a>> {
    let parser = tuple((
        tuple((multispace0, tag("let"), multispace1)),
        alphanumeric1,
        ws(tag("=")),
        parse_expression,
        ws(tag(";")),
    ));
    map(parser, |(_, ident, _, value, _)| Statement::Let {
        name: ident,
        value,
    })(code)
}
