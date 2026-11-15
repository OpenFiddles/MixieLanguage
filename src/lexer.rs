use crate::syntax;
#[derive(Debug,Clone,PartialEq)]
pub enum Kind{Ident(String),String(String),LeftBrace,RightBrace,Comma,Equals,Semi}
#[derive(Debug,Clone,PartialEq)]
pub struct Token{pub kind:Kind,pub line:usize,pub col:usize}
pub fn lex(source:&str)->Result<Vec<Token>,String>{
 let c:Vec<char>=source.chars().collect();let(mut i,mut line,mut col)=(0,1,1);let mut out=Vec::new();
 while i<c.len(){let ch=c[i];if ch=='\n'{line+=1;col=1;i+=1;continue}if ch.is_whitespace(){col+=1;i+=1;continue}
 if ch=='^'{let start=line;if matches!(c.get(i+1),Some('*')|Some('$')){let end=if c[i+1]=='*'{('*','^')}else{('$','^')};i+=2;col+=2;let mut closed=false;while i<c.len(){if c[i]==end.0&&c.get(i+1)==Some(&end.1){i+=2;col+=2;closed=true;break}if c[i]=='\n'{line+=1;col=1}else{col+=1}i+=1}if !closed{return Err(format!("unterminated comment at line {start}"))}}else{while i<c.len()&&c[i]!='\n'{i+=1;col+=1}}continue}
 let(tl,tc)=(line,col);if ch=='"'{i+=1;col+=1;let mut s=String::new();let mut closed=false;while i<c.len(){match c[i]{'"'=>{i+=1;col+=1;closed=true;break},'\\'=>{i+=1;col+=1;let e=*c.get(i).ok_or_else(||format!("bad escape at {line}:{col}"))?;s.push(match e{'n'=>'\n','r'=>'\r','t'=>'\t','"'=>'"','\\'=>'\\',x=>x});i+=1;col+=1},'\n'=>return Err(format!("newline in string at {line}:{col}")),x=>{s.push(x);i+=1;col+=1}}}if !closed{return Err(format!("unterminated string at {tl}:{tc}"))}out.push(Token{kind:Kind::String(s),line:tl,col:tc});continue}
 let punct=match ch{'{'=>Some(Kind::LeftBrace),'}'=>Some(Kind::RightBrace),','=>Some(Kind::Comma),'='=>Some(Kind::Equals),';'=>Some(Kind::Semi),_=>None};if let Some(kind)=punct{out.push(Token{kind,line,col});i+=1;col+=1;continue}
 if ch.is_ascii_alphabetic()||ch=='_'{let start=i;while i<c.len()&&(c[i].is_ascii_alphanumeric()||c[i]=='_'){i+=1;col+=1}let w:String=c[start..i].iter().collect();let _=syntax::is_keyword(&w);out.push(Token{kind:Kind::Ident(w),line:tl,col:tc});continue}
 return Err(format!("unexpected character {ch:?} at {line}:{col}"))}
 Ok(out)
}
