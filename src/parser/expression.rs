use crate::ast::Expression;
use crate::parser::ident;
use crate::parser::keyword;
use crate::parser::token;
use crate::parser::util::hyphened_keyword;
use crate::parser::util::predicate_map;
use crate::parser::util::string_literal;
use crate::parser::util::template_literal;
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

pub fn parse_expression<'a, 'b: 'a>(
    code: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], Expression<'b>> {
    let parser = alt((parse_distribute, parse_ifmatch, parse_union));
    parser(code)
}

fn parse_distribute<'a, 'b: 'a>(
    input: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], Expression<'b>> {
    let parser = tuple((
        keyword("distribute"),
        ident,
        keyword("in"),
        parse_expression,
    ));
    let (input, (_, id, _, body)) = parser(input)?;
    Ok((
        input,
        Expression::Distribute {
            ident: id,
            body: Box::new(body),
        },
    ))
}

fn parse_ifmatch<'a, 'b: 'a>(input: &'a [Token<'b>]) -> IResult<&'a [Token<'b>], Expression<'b>> {
    let parser = tuple((
        hyphened_keyword("if", "match"),
        parse_expression,
        keyword("with"),
        template_literal,
        keyword("then"),
        parse_expression,
        keyword("else"),
        parse_expression,
    ));
    let (input, (_, target, _, pattern, _, then_expr, _, else_expr)) = parser(input)?;
    Ok((
        input,
        Expression::IfMatch {
            pattern: pattern.to_vec(),
            target: Box::new(target),
            then_expr: Box::new(then_expr),
            else_expr: Box::new(else_expr),
        },
    ))
}

fn parse_union<'a, 'b: 'a>(code: &'a [Token<'b>]) -> IResult<&'a [Token<'b>], Expression<'b>> {
    let parser = preceded(
        opt(token(Token::Bar)),
        separated_nonempty_list(token(Token::Bar), parse_atomic_expression),
    );
    map(parser, |mut exprs| {
        if exprs.len() > 1 {
            Expression::Union { exprs }
        } else {
            exprs.pop().unwrap()
        }
    })(code)
}

fn parse_atomic_expression<'a, 'b: 'a>(
    code: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], Expression<'b>> {
    let parser = alt((
        parse_string_literal,
        parse_template_literal,
        parse_boolean_literal,
        parse_map,
        parse_tuple,
        parse_ident,
    ));
    parser(code)
}

fn parse_ident<'a, 'b: 'a>(code: &'a [Token<'b>]) -> IResult<&'a [Token<'b>], Expression<'b>> {
    map(ident, |res| Expression::Var { name: res })(code)
}

fn parse_string_literal<'a, 'b: 'a>(
    code: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], Expression<'b>> {
    map(string_literal, |value| Expression::StringLiteral { value })(code)
}

fn parse_boolean_literal<'a, 'b: 'a>(
    code: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], Expression<'b>> {
    predicate_map(|token| match token {
        Token::Keyword("true") => Some(Expression::BooleanLiteral { value: true }),
        Token::Keyword("false") => Some(Expression::BooleanLiteral { value: false }),
        _ => None,
    })(code)
}

fn parse_template_literal<'a, 'b: 'a>(
    code: &'a [Token<'b>],
) -> IResult<&'a [Token<'b>], Expression<'b>> {
    map(template_literal, |items| Expression::TemplateLiteral {
        items: items.to_vec(),
    })(code)
}

fn parse_map<'a, 'b: 'a>(code: &'a [Token<'b>]) -> IResult<&'a [Token<'b>], Expression<'b>> {
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

fn parse_tuple<'a, 'b: 'a>(code: &'a [Token<'b>]) -> IResult<&'a [Token<'b>], Expression<'b>> {
    let parser = tuple((
        token(Token::OpenBracket),
        separated_list(token(Token::Comma), parse_expression),
        token(Token::CloseBracket),
    ));
    map(parser, |(_, values, _)| Expression::Tuple { values })(code)
}
