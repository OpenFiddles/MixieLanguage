use std::{path::Path,process::{Command,Stdio},io::Write};
use crate::{ast::{Expr,Program,Statement},interpreter,lexer,parser};
fn c_string(s:&str)->String{let mut o=String::from("\"");for c in s.chars(){match c{ '\\'=>o.push_str("\\\\"),'"'=>o.push_str("\\\""),'\n'=>o.push_str("\\n"),'\r'=>o.push_str("\\r"),'\t'=>o.push_str("\\t"),c if c.is_ascii() && !c.is_control()=>o.push(c),c=>o.push_str(&format!("\\x{:02x}",c as u32))}}o.push('"');o}
fn c_expr(e:&Expr)->Result<String,String>{match e{
    Expr::Number(n)=>Ok(format!("{n}")),Expr::Variable(n)=>Ok(format!("v_{n}")),
    Expr::Unary{op:'-',value}=>Ok(format!("(-{})",c_expr(value)?)),
    Expr::Binary{left,op,right}=>Ok(format!("({} {op} {})",c_expr(left)?,c_expr(right)?)),
    Expr::String(_)=>Err("native compiler currently supports strings as direct print values, not numeric expressions".into()),
    Expr::Unary{op,..}=>Err(format!("unsupported unary operator '{op}'")),
}}
pub fn emit_c(p:&Program)->Result<String,String>{
    let mut c=String::from("#include <stdio.h>\nint main(void) {\n");
    for s in &p.statements {match s {
        Statement::Let{name,value}=>{if !name.chars().all(|x|x.is_ascii_alphanumeric()||x=='_'){return Err("invalid variable name".into())}let v=interpreter::evaluate(value,&interpreter::Environment::new());if matches!(value,Expr::String(_))||v.as_ref().is_ok_and(|x|matches!(x,interpreter::Value::String(_))){return Err("native compiler currently supports numeric variables only".into())}c.push_str(&format!("  double v_{name} = {};\n",c_expr(value)?));}
        Statement::Print(Expr::String(s))=>c.push_str(&format!("  puts({});\n",c_string(s))),
        Statement::Print(e)=>c.push_str(&format!("  printf(\"%.15g\\n\", (double)({}));\n",c_expr(e)?)),
        Statement::Pull(_)=>return Err("pull is a Builgine directive and cannot be compiled into an application yet".into()),
    }}
    c.push_str("  return 0;\n}\n");Ok(c)
}
pub fn compile_file(source:&str,output:&Path)->Result<(),String>{
    let tokens=lexer::lex(source)?;let program=parser::parse(tokens)?;let c=emit_c(&program)?;
    let compiler=std::env::var("CC").unwrap_or_else(|_|"cc".into());
    let mut child=Command::new(&compiler).args(["-std=c11","-O2","-x","c","-o"]).arg(output).arg("-").stdin(Stdio::piped()).stderr(Stdio::piped()).spawn().map_err(|e|format!("could not start C compiler '{compiler}': {e}. Install clang or GCC and set CC if needed."))?;
    child.stdin.as_mut().ok_or("failed to open compiler stdin")?.write_all(c.as_bytes()).map_err(|e|e.to_string())?;
    let result=child.wait_with_output().map_err(|e|e.to_string())?;
    if !result.status.success(){return Err(format!("native compilation failed: {}",String::from_utf8_lossy(&result.stderr)))}
    #[cfg(unix)] {use std::os::unix::fs::PermissionsExt;let mut p=std::fs::metadata(output).map_err(|e|e.to_string())?.permissions();p.set_mode(0o755);std::fs::set_permissions(output,p).map_err(|e|e.to_string())?;}
    Ok(())
}
