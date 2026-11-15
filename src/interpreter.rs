use crate::{ast::{Program,Statement},builgine};
pub fn execute(p:&Program)->Result<(),String>{print!("{}",execute_to_string(p)?);Ok(())}
pub fn execute_to_string(p:&Program)->Result<String,String>{let mut out=String::new();for s in &p.statements{match s{Statement::Print(v)=>{out.push_str(v);out.push('\n')},Statement::Pull(spec)=>out.push_str(&builgine::describe_pull(spec)?),}}Ok(out)}
