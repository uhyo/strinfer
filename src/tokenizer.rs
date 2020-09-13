use crate::parser::util::ws;
use nom::branch::alt;
use nom::bytes::complete::escaped;
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::character::complete::none_of;
use nom::character::complete::one_of;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::combinator::map_opt;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TemplateItem<'a> {
    Str(&'a str),
    Interpolate(&'a str),
    Infer(&'a str),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
    Keyword(&'a str),
    HyphenedKeyword(&'a str, &'a str),
    Ident(&'a str),
    StringLiteral(&'a str),
    TemplateLiteral(Vec<TemplateItem<'a>>),
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Equal,
    Colon,
    SemiColon,
    Comma,
    Bar,
}

pub fn tokenize<'a>(code: &'a str) -> IResult<&'a str, Vec<Token<'a>>> {
    let parser1 = alt((
        map_opt(
            pair(alphanumeric1, opt(preceded(char('-'), alphanumeric1))),
            |(chars, suffix)| match suffix {
                None => {
                    if is_keyword(chars) {
                        Some(Token::Keyword(chars))
                    } else {
                        Some(Token::Ident(chars))
                    }
                }
                Some(suffix) => {
                    if is_hyphen_keyword(chars, suffix) {
                        Some(Token::HyphenedKeyword(chars, suffix))
                    } else {
                        None
                    }
                }
            },
        ),
        map(one_of("{}[]=:;,|"), |ch| match ch {
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            '=' => Token::Equal,
            ':' => Token::Colon,
            ';' => Token::SemiColon,
            ',' => Token::Comma,
            '|' => Token::Bar,
            _ => unreachable!(),
        }),
        map(parse_string, |s| Token::StringLiteral(s)),
        map(parse_template_literal, |items| {
            Token::TemplateLiteral(items)
        }),
    ));
    let parser = all_consuming(many0(ws(parser1)));
    parser(code)
}

fn parse_string<'a>(code: &'a str) -> IResult<&'a str, &'a str> {
    let parser = tuple((
        char('"'),
        escaped(none_of("\"\\\r\n"), '\\', none_of("\r\n")),
        char('"'),
    ));
    map(parser, |(_, value, _)| value)(code)
}

fn parse_template_literal<'a>(code: &'a str) -> IResult<&'a str, Vec<TemplateItem<'a>>> {
    let parser = tuple((char('`'), many0(parse_template_item), char('`')));
    map(parser, |(_, items, _)| items)(code)
}

fn parse_template_item<'a>(code: &'a str) -> IResult<&'a str, TemplateItem<'a>> {
    let parser = alt((
        map(
            tuple((char('['), alphanumeric1, char(']'))),
            |(_, name, _)| TemplateItem::Infer(name),
        ),
        map(
            tuple((char('{'), alphanumeric1, char('}'))),
            |(_, name, _)| TemplateItem::Interpolate(name),
        ),
        map(escaped(none_of("`\\"), '\\', none_of("\r\n")), |chars| {
            TemplateItem::Str(chars)
        }),
    ));
    parser(code)
}

fn is_keyword(s: &str) -> bool {
    match s {
        "let" | "true" | "false" | "fn" => true,
        _ => false,
    }
}

fn is_hyphen_keyword(s: &str, t: &str) -> bool {
    match (s, t) {
        ("if", "match") => true,
        _ => false,
    }
}
