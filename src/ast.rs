use crate::tokenizer::TemplateItem;

pub type Program<'a> = Vec<Statement<'a>>;

#[derive(Debug)]
pub enum Statement<'a> {
    Let {
        name: &'a str,
        value: Expression<'a>,
    },
    Fn {
        name: &'a str,
        args: Vec<&'a str>,
        body: Expression<'a>,
    },
}

#[derive(Debug)]
pub enum Expression<'a> {
    // Atomic Expressions
    StringLiteral {
        value: &'a str,
    },
    TemplateLiteral {
        items: Vec<TemplateItem<'a>>,
    },
    BooleanLiteral {
        value: bool,
    },
    Var {
        name: &'a str,
    },
    Map {
        pairs: Vec<(&'a str, Expression<'a>)>,
    },
    Tuple {
        values: Vec<Expression<'a>>,
    },
    // Union Expression
    Union {
        exprs: Vec<Expression<'a>>,
    },
    // Keyword Expressions
    Distribute {
        ident: &'a str,
        body: Box<Expression<'a>>,
    },
    IfMatch {
        target: Box<Expression<'a>>,
        pattern: Vec<TemplateItem<'a>>,
        then_expr: Box<Expression<'a>>,
        else_expr: Box<Expression<'a>>,
    },
}
