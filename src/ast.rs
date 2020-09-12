pub type Program<'a> = Vec<Statement<'a>>;

#[derive(Debug)]
pub enum Statement<'a> {
    Let {
        name: &'a str,
        value: Expression<'a>,
    },
}

#[derive(Debug)]
pub enum Expression<'a> {
    StringLiteral { value: &'a str },
    Union { exprs: Vec<Expression<'a>> },
}
