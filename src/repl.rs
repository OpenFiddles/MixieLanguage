use std::io::{self,Write};
use crate::{interpreter::{self,Environment},lexer,parser};
pub fn run()->Result<(),String>{
    println!("MixieLang interactive interpreter — type :help for commands");
    let stdin=io::stdin();let mut env=Environment::new();let mut line=String::new();
    loop {
        print!("mixie> ");io::stdout().flush().map_err(|e|e.to_string())?;
        line.clear();let n=stdin.read_line(&mut line).map_err(|e|e.to_string())?;if n==0{break}
        let input=line.trim();if input.is_empty(){continue}
        match input {
            ":quit"|":q"|":exit"=>break,
            ":help"|":h"=>{println!("Enter MixieLang statements ending in ';'.\n  let answer = 6 * 7;\n  print answer;\nCommands: :help, :vars, :reset, :quit");continue}
            ":vars"=>{for name in env.names(){if let Some(v)=env.get(&name){println!("{name} = {v:?}")}}continue}
            ":reset"=>{env.clear();println!("Environment cleared.");continue}
            _=>{}
        }
        match lexer::lex(input).and_then(parser::parse).and_then(|p|interpreter::execute_in(&p,&mut env)){
            Ok(out)=>print!("{out}"),Err(e)=>eprintln!("error: {e}")
        }
    }
    println!("Leaving MixieLang.");Ok(())
}
