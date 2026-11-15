#[derive(Debug)]
pub enum Statement {
    Print(String),
}

#[derive(Debug, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}
