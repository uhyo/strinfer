use crate::tokenizer::Token;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::error::make_error;
use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::sequence::tuple;
use nom::AsChar;
use nom::IResult;
use nom::InputTakeAtPosition;

pub fn ws<I, O, E: ParseError<I>, F>(f: F) -> impl Fn(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition + Clone,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    E: ParseError<I>,
    F: Fn(I) -> IResult<I, O, E>,
{
    map(tuple((multispace0, f, multispace0)), |(_, res, _)| res)
}

/// Parses one input value that satisfies given predicate.
pub fn predicate<'a, F>(
    pred: F,
) -> impl Fn(&'a [Token<'a>]) -> IResult<&'a [Token<'a>], &'a Token<'a>>
where
    F: Fn(&'a Token<'a>) -> bool,
{
    move |input| match input.split_first() {
        Some((fst, rest)) if pred(fst) => Ok((rest, fst)),
        None => Err(nom::Err::Error(make_error(input, ErrorKind::Tag))),
    }
}

/// Parses one input value that satisfies given predicate.
pub fn predicate_map<'a, F, R>(mapper: F) -> impl Fn(&'a [Token<'a>]) -> IResult<&'a [Token<'a>], R>
where
    F: Fn(&'a Token<'a>) -> Option<R>,
{
    move |input| match input.split_first() {
        Some((fst, rest)) => match mapper(fst) {
            Some(res) => Ok((rest, res)),
            None => Err(nom::Err::Error(make_error(input, ErrorKind::MapOpt))),
        },
        None => Err(nom::Err::Error(make_error(input, ErrorKind::Tag))),
    }
}

/// Parses keyword.
pub fn keyword<'a>(
    name: &'a str,
) -> impl Fn(&'a [Token<'a>]) -> IResult<&'a [Token<'a>], &'a Token<'a>> {
    predicate(|token| matches!(token, &Token::Keyword(name)))
}

/// Parses identifier.
pub fn ident<'a>(input: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], &'a str> {
    predicate_map(|token| match token {
        Token::Ident(name) => Some(*name),
        _ => None,
    })(input)
}

/// Parses string literal.
pub fn string_literal<'a>(input: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], &'a str> {
    predicate_map(|token| match token {
        Token::StringLiteral(value) => Some(*value),
        _ => None,
    })(input)
}

/// Parses static token.
pub fn token<'a>(
    token: Token<'a>,
) -> impl Fn(&'a [Token<'a>]) -> IResult<&'a [Token<'a>], &'a Token<'a>> {
    predicate(move |t| t == &token)
}
