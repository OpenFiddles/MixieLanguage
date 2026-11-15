use std::{env,fs,io::{self,Read},path::PathBuf,process::ExitCode};
fn run_cli()->Result<(),String>{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.first().is_some_and(|a|a=="setup"||a=="--setup"){return mixieland::setup::run_setup();}
    if args.first().is_some_and(|a|a=="repl"||a=="shell"){return mixieland::repl::run();}
    if args.iter().any(|a|a=="--help"||a=="-h"){println!("MixieLang — MixieLand Rust toolchain\nUsage:\n  mixie repl\n  mixie -f <file.mx>\n  mixie compile <file.mx> [-o output]\n  mixie setup");return Ok(())}
    if args.iter().any(|a|a=="--version"||a=="-V"){println!("MixieLang {}",env!("CARGO_PKG_VERSION"));return Ok(())}
    if args.first().is_some_and(|a|a=="compile"){
        if args.len()<2{return Err("usage: mixie compile <file.mx> [-o output]".into())}
        let input=&args[1];let mut output=PathBuf::from(input);output.set_extension("");
        let mut i=2;while i<args.len(){if args[i]=="-o"&&i+1<args.len(){output=PathBuf::from(&args[i+1]);i+=2}else{return Err(format!("unknown compile option '{}'",args[i]))}}
        let source=fs::read_to_string(input).map_err(|e|format!("cannot read {input}: {e}"))?;
        mixieland::compiler::compile_file(&source,&output)?;
        println!("Built native executable: {}",output.display());return Ok(())
    }
    let source=if args.len()==2&&args[0]=="-f"{let p=&args[1];fs::read_to_string(p).map_err(|e|format!("cannot read {p}: {e}"))?}
    else if args.is_empty(){let mut s=String::new();io::stdin().read_to_string(&mut s).map_err(|e|format!("stdin: {e}"))?;s}
    else{return Err("usage: mixie repl | mixie -f <file.mx> | mixie compile <file.mx> [-o output]".into())};
    print!("{}",mixieland::run(&source)?);Ok(())
}
fn main()->ExitCode{match run_cli(){Ok(())=>ExitCode::SUCCESS,Err(e)=>{eprintln!("MixieLang error: {e}");ExitCode::FAILURE}}}
