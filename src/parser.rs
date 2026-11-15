use crate::{ast::{Expr, Program, PullSpec, Statement}, lexer::{Kind, Token}};
pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut p = Program::default(); let mut i = 0;
    while i < tokens.len() {
        let (word, line, col) = match &tokens[i].kind { Kind::Ident(w)=>(w.as_str(),tokens[i].line,tokens[i].col), _=>return Err(format!("expected statement at {}:{}",tokens[i].line,tokens[i].col)) };
        i += 1;
        match word {
            "print" => { let v = expression(&tokens, &mut i, 0)?; expect(&tokens,&mut i,|k|matches!(k,Kind::Semi),"expected ';' after print")?; p.statements.push(Statement::Print(v)); }
            "let" => { let name = ident(&tokens,&mut i,"expected variable name")?; expect(&tokens,&mut i,|k|matches!(k,Kind::Equals),"expected '=' after variable name")?; let value=expression(&tokens,&mut i,0)?; expect(&tokens,&mut i,|k|matches!(k,Kind::Semi),"expected ';' after assignment")?; p.statements.push(Statement::Let{name,value}); }
            "pull" => {
                expect(&tokens,&mut i,|k|matches!(k,Kind::LeftBrace),"expected '{' after pull")?;
                let name=ident(&tokens,&mut i,"expected resource name")?;
                let mut s=PullSpec{name,..Default::default()};
                while !matches!(tokens.get(i).map(|t|&t.kind),Some(Kind::RightBrace)) {
                    expect(&tokens,&mut i,|k|matches!(k,Kind::Comma),"expected ',' between pull fields")?;
                    let key=ident(&tokens,&mut i,"expected pull field")?;
                    expect(&tokens,&mut i,|k|matches!(k,Kind::Equals),"expected '=' after pull field")?;
                    let v=string(&tokens,&mut i,"pull values must be quoted strings")?;
                    let slot=match key.as_str(){"source"=>&mut s.source,"binary"=>&mut s.binary,"revision"=>&mut s.revision,"checksum"=>&mut s.checksum,"build"=>&mut s.build,_=>return Err(format!("unknown pull field '{key}'"))};
                    if slot.replace(v).is_some(){return Err(format!("duplicate pull field '{key}'"))}
                }
                i+=1;
                if s.source.is_some()==s.binary.is_some(){return Err("pull requires exactly one of source or binary".into())}
                if s.binary.is_some()&&s.checksum.is_none(){return Err("prebuilt binary pulls require a SHA256 checksum".into())}
                if s.build.as_deref().is_some_and(|v|v!="make"){return Err("initial source builder recognizes build = \"make\" only".into())}
                p.statements.push(Statement::Pull(s));
            }
            x=>return Err(format!("unknown statement '{x}' at {line}:{col}")),
        }
    }
    Ok(p)
}
fn expression(t:&[Token],i:&mut usize,min:u8)->Result<Expr,String>{
    let mut left=match t.get(*i).map(|x|&x.kind){
        Some(Kind::Number(n))=>{let n=*n;*i+=1;Expr::Number(n)}
        Some(Kind::String(s))=>{let s=s.clone();*i+=1;Expr::String(s)}
        Some(Kind::Ident(s))=>{let s=s.clone();*i+=1;Expr::Variable(s)}
        Some(Kind::Operator('-'))=>{*i+=1;Expr::Unary{op:'-',value:Box::new(expression(t,i,3)?)}}
        Some(Kind::LeftParen)=>{*i+=1;let e=expression(t,i,0)?;expect(t,i,|k|matches!(k,Kind::RightParen),"expected ')'")?;e}
        _=>return Err("expected a number, string, variable, or parenthesized expression".into())
    };
    loop {
        let op=match t.get(*i).map(|x|&x.kind){Some(Kind::Operator(op))=>*op,_=>break};
        let prec=if op=='+'||op=='-'{1}else{2}; if prec<min{break}
        *i+=1;let right=expression(t,i,prec+1)?;
        left=Expr::Binary{left:Box::new(left),op,right:Box::new(right)};
    }
    Ok(left)
}
fn expect<F:Fn(&Kind)->bool>(t:&[Token],i:&mut usize,f:F,msg:&str)->Result<(),String>{if t.get(*i).is_some_and(|v|f(&v.kind)){*i+=1;Ok(())}else{Err(msg.into())}}
fn ident(t:&[Token],i:&mut usize,msg:&str)->Result<String,String>{match t.get(*i).map(|v|&v.kind){Some(Kind::Ident(s))=>{*i+=1;Ok(s.clone())},_=>Err(msg.into())}}
fn string(t:&[Token],i:&mut usize,msg:&str)->Result<String,String>{match t.get(*i).map(|v|&v.kind){Some(Kind::String(s))=>{*i+=1;Ok(s.clone())},_=>Err(msg.into())}}
