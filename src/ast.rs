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
    // Atomic Expressions
    StringLiteral {
        value: &'a str,
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
}
