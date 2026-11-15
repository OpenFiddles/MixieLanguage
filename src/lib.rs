pub mod ast;
pub mod builgine;
pub mod image;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod setup;
pub mod syntax;
use wasm_bindgen::prelude::*;
pub fn run(source: &str) -> Result<String, String> {
    if source.trim_start().starts_with("(startofimagedata") {
        image::validate(source)?;
        return Ok("Successfully parsed .mxpg file.\n".into());
    }
    let tokens = lexer::lex(source)?;
    let program = parser::parse(tokens)?;
    interpreter::execute_to_string(&program)
}
#[wasm_bindgen]
pub fn run_mixie(source: &str) -> String {
    run(source).unwrap_or_else(|e| format!("MixieLang error: {e}"))
}
