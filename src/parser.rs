use crate::ast::Program;
use crate::ast::Statement;
use crate::parser::util::ws;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::error::ParseError;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::AsChar;
use nom::InputTakeAtPosition;
use nom::{bytes::complete::tag, IResult};
use nom::{exact, many0, named};

mod util;

named!(
    pub parse(&str) -> Program,
    exact!(many0!(parse_statement))
);

fn parse_statement<'a>(code: &'a str) -> IResult<&'a str, Statement<'a>> {
    let parser = tuple((
        tuple((multispace0, tag("let"), multispace1)),
        alphanumeric1,
        ws(tag("=")),
    ));
    map(parser, |(_, ident, _)| Statement::Let { name: ident })(code)
}
