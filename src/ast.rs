#[derive(Debug)]
pub enum Statement<'a> {
    Let { name: &'a str },
}

pub type Program<'a> = Vec<Statement<'a>>;
