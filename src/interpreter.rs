use crate::ast::{Program, Statement};

pub fn execute(program: &Program) -> Result<(), String> {
    for s in &program.statements {
        match s {
            Statement::Print(v) => println!("{v}"),
        }
    }
    Ok(())
}
