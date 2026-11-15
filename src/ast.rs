#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Variable(String),
    Unary { op: char, value: Box<Expr> },
    Binary { left: Box<Expr>, op: char, right: Box<Expr> },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expr },
    Print(Expr),
    Pull(PullSpec),
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PullSpec {
    pub name: String,
    pub source: Option<String>,
    pub binary: Option<String>,
    pub revision: Option<String>,
    pub checksum: Option<String>,
    pub build: Option<String>,
}
#[derive(Debug, Default)]
pub struct Program { pub statements: Vec<Statement> }
