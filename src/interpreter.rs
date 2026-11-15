use std::collections::HashMap;
use crate::{ast::{Expr,Program,Statement},builgine};
#[derive(Debug,Clone,PartialEq)]
pub enum Value { Number(f64), String(String) }
#[derive(Debug,Default)]
pub struct Environment { values: HashMap<String,Value> }
impl Environment {
    pub fn new()->Self{Self::default()}
    pub fn get(&self,name:&str)->Option<&Value>{self.values.get(name)}
    pub fn clear(&mut self){self.values.clear();}
    pub fn names(&self)->Vec<String>{let mut v:Vec<_>=self.values.keys().cloned().collect();v.sort();v}
}
pub fn evaluate(expr:&Expr,env:&Environment)->Result<Value,String>{
    match expr {
        Expr::Number(n)=>Ok(Value::Number(*n)),
        Expr::String(s)=>Ok(Value::String(s.clone())),
        Expr::Variable(n)=>env.get(n).cloned().ok_or_else(||format!("undefined variable '{n}'")),
        Expr::Unary{op:'-',value}=>match evaluate(value,env)?{Value::Number(n)=>Ok(Value::Number(-n)),_=>Err("unary '-' requires a number".into())},
        Expr::Unary{op,..}=>Err(format!("unsupported unary operator '{op}'")),
        Expr::Binary{left,op,right}=>{
            let a=evaluate(left,env)?;let b=evaluate(right,env)?;
            match (a,b) {
                (Value::Number(x),Value::Number(y))=>match *op {'+'=>Ok(Value::Number(x+y)),'-'=>Ok(Value::Number(x-y)),'*'=>Ok(Value::Number(x*y)),'/'=>if y==0.0{Err("division by zero".into())}else{Ok(Value::Number(x/y))},_=>Err(format!("unsupported operator '{op}'"))},
                (Value::String(x),Value::String(y)) if *op=='+' =>Ok(Value::String(x+&y)),
                _=>Err("operator requires two numbers (or two strings with '+')".into())
            }
        }
    }
}
pub fn execute_in(p:&Program,env:&mut Environment)->Result<String,String>{
    let mut out=String::new();
    for s in &p.statements {match s {
        Statement::Let{name,value}=>{let v=evaluate(value,env)?;env.values.insert(name.clone(),v);}
        Statement::Print(e)=>{match evaluate(e,env)?{Value::Number(n)=>out.push_str(&format_number(n)),Value::String(s)=>out.push_str(&s)}out.push('\n');}
        Statement::Pull(spec)=>out.push_str(&builgine::describe_pull(spec)?),
    }}
    Ok(out)
}
fn format_number(n:f64)->String{if n.fract()==0.0{format!("{n:.0}")}else{n.to_string()}}
pub fn execute(p:&Program)->Result<(),String>{print!("{}",execute_to_string(p)?);Ok(())}
pub fn execute_to_string(p:&Program)->Result<String,String>{execute_in(p,&mut Environment::new())}
