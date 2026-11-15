use std::{env,fs,io::{self,Read},process::ExitCode};
fn run_cli()->Result<(),String>{
 let args:Vec<String>=env::args().skip(1).collect();
 if args.iter().any(|a|a=="--help"||a=="-h"){println!("MixieLang — MixieLand Rust toolchain\nUsage: mixie -f <file.mx> | mixie [--help] [--version]");return Ok(())}
 if args.iter().any(|a|a=="--version"||a=="-V"){println!("MixieLang {}",env!("CARGO_PKG_VERSION"));return Ok(())}
 let source=if args.len()==2&&args[0]=="-f"{let p=&args[1];fs::read_to_string(p).map_err(|e|format!("cannot read {p}: {e}"))?}else if args.is_empty(){let mut s=String::new();io::stdin().read_to_string(&mut s).map_err(|e|format!("stdin: {e}"))?;s}else{return Err("usage: mixie -f <file.mx> (or pipe source to stdin)".into())};
 print!("{}",mixieland::run(&source)?);Ok(())
}
fn main()->ExitCode{match run_cli(){Ok(())=>ExitCode::SUCCESS,Err(e)=>{eprintln!("MixieLang error: {e}");ExitCode::FAILURE}}}
