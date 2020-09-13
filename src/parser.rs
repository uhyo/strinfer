use crate::ast::Program;
use crate::ast::Statement;
use crate::parser::expression::parse_expression;
use crate::parser::util::ident;
use crate::parser::util::keyword;
use crate::parser::util::token;
use crate::parser::util::ws;
use crate::tokenizer::Token;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{bytes::complete::tag, IResult};

mod expression;
pub mod util;

pub fn parse<'a, 'b: 'a>(tokens: &'a [Token<'b>]) -> IResult<&'a [Token<'b>], Program<'b>> {
    all_consuming(many0(parse_statement))(tokens)
}

fn parse_statement<'a, 'b: 'a>(code: &'a [Token<'b>]) -> IResult<&'a [Token<'b>], Statement<'b>> {
    let parser = tuple((
        keyword("let"),
        ident,
        token(Token::Equal),
        parse_expression,
        token(Token::SemiColon),
    ));
    let (input, (_, ident, _, value, _)) = parser(code)?;
    Ok((input, Statement::Let { name: ident, value }))
}
