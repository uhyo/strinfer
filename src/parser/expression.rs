use crate::ast::Expression;
use crate::parser::ident;
use crate::parser::token;
use crate::parser::util::predicate_map;
use crate::parser::util::string_literal;
use crate::tokenizer::Token;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::separated_list;
use nom::multi::separated_nonempty_list;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

pub fn parse_expression<'a>(code: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], Expression<'a>> {
    parse_union(code)
}

fn parse_union<'a>(code: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], Expression<'a>> {
    let parser = preceded(
        opt(token(Token::Bar)),
        separated_nonempty_list(token(Token::Bar), parse_atomic_expression),
    );
    map(parser, |mut exprs| {
        if exprs.len() >= 1 {
            Expression::Union { exprs }
        } else {
            exprs.pop().unwrap()
        }
    })(code)
}

fn parse_atomic_expression<'a>(code: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], Expression<'a>> {
    let parser = alt((
        parse_string_literal,
        parse_boolean_literal,
        parse_map,
        parse_tuple,
        parse_ident,
    ));
    parser(code)
}

fn parse_ident<'a>(code: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], Expression<'a>> {
    map(ident, |res| Expression::Var { name: res })(code)
}

fn parse_string_literal<'a>(code: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], Expression<'a>> {
    map(string_literal, |value| Expression::StringLiteral { value })(code)
}

fn parse_boolean_literal<'a>(code: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], Expression<'a>> {
    predicate_map(|token| match token {
        Token::Keyword("true") => Some(Expression::BooleanLiteral { value: true }),
        Token::Keyword("false") => Some(Expression::BooleanLiteral { value: false }),
        _ => None,
    })(code)
}

fn parse_map<'a>(code: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], Expression<'a>> {
    let parser = tuple((
        token(Token::OpenBrace),
        many0(tuple((
            string_literal,
            token(Token::Colon),
            parse_expression,
            token(Token::SemiColon),
        ))),
        token(Token::CloseBrace),
    ));
    map(parser, |(_, pairs, _)| Expression::Map {
        pairs: pairs
            .into_iter()
            .map(|(key, _, value, _)| (key, value))
            .collect(),
    })(code)
}

fn parse_tuple<'a>(code: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], Expression<'a>> {
    let parser = tuple((
        token(Token::OpenBracket),
        separated_list(token(Token::Comma), parse_expression),
        token(Token::CloseBracket),
    ));
    map(parser, |(_, values, _)| Expression::Tuple { values })(code)
}
