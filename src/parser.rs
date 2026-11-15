use crate::{
    ast::{Program, Statement},
    lexer::{Kind, Token},
};

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut p = Program::default();
    let mut i = 0;
    while i < tokens.len() {
        let t = &tokens[i];
        let (word, line, col) = match &t.kind {
            Kind::Ident(w) => (w.as_str(), t.line, t.col),
            _ => return Err(format!("expected statement at {}:{}", t.line, t.col)),
        };
        if word != "print" {
            return Err(format!("unknown statement '{word}' at {line}:{col}"));
        }
        i += 1;
        let value = match tokens.get(i).map(|t| &t.kind) {
            Some(Kind::String(s)) => s.clone(),
            _ => return Err(format!("print expects a quoted string at {line}:{col}")),
        };
        i += 1;
        if !matches!(tokens.get(i).map(|t| &t.kind), Some(Kind::Semi)) {
            return Err(format!("expected ';' after print at {line}:{col}"));
        }
        i += 1;
        p.statements.push(Statement::Print(value));
    }
    Ok(p)
}
