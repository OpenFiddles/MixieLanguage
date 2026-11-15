#[derive(Debug,Clone,PartialEq)]
pub enum Statement{Print(String),Pull(PullSpec)}
#[derive(Debug,Clone,PartialEq,Default)]
pub struct PullSpec{pub name:String,pub source:Option<String>,pub binary:Option<String>,pub revision:Option<String>,pub checksum:Option<String>,pub build:Option<String>}
#[derive(Debug,Default)]
pub struct Program{pub statements:Vec<Statement>}
