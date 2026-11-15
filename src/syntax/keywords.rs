#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Keyword{Print,Pull,Push,If,Else,Func,MFunc,Pixie,Asm}
pub fn keyword(w:&str)->Option<Keyword>{Some(match w{"print"=>Keyword::Print,"pull"=>Keyword::Pull,"push"=>Keyword::Push,"if"=>Keyword::If,"else"=>Keyword::Else,"func"=>Keyword::Func,"mFunc"=>Keyword::MFunc,"pixie"=>Keyword::Pixie,"asm"=>Keyword::Asm,_=>return None})}
pub fn is_keyword(w:&str)->bool{keyword(w).is_some()}
