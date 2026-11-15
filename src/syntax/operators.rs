pub const OPERATORS:&[&str]=&["->","=>","==","!=","<=",">=","=","+","-","*","/",">","<"];
pub fn is_operator(s:&str)->bool{OPERATORS.contains(&s)}
