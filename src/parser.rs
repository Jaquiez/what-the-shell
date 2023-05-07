use crate::scanner::Scanner;
use crate::token::{WTSType, Token, TokenType};
use crate::symbol::Symbol;
use crate::error::{self, error};

#[derive(Debug)]
pub struct AST{
    exprs: Vec<Expr>
}


#[derive(Debug)]
struct Expr{
    kind: Kind,
    flags: Vec<String>,
    args: Vec<String>,
    symbol: Option<Symbol>,
    value: Option<String>,
    left: Option<AST>,
    right: Option<AST>,
}

#[derive(Debug,PartialEq,Eq)]
pub enum Kind{
    NONE,
    CMD,
    EXPR
}


fn parse_expression(toks: &mut Vec<Token>,inParen: bool) -> Option<AST>{
    let mut ast: AST = AST { exprs: vec![] };
    let mut iscomment = false;
    ast.exprs.push(Expr{
        kind: Kind::NONE,
        flags: Vec::new(),
        value: Some(String::new()),
        args: Vec::new(),
        symbol: None,
        left:None,
        right: None
    });
    let mut counter = 0;
    while toks.len() > 0{
        let tok = toks.remove(0);
        if(iscomment){continue;}
        match tok.t_type {
            TokenType::LEFT_PAREN => {
                if ast.exprs[counter].left.is_none() {
                    ast.exprs[counter].left = parse_expression(toks, true);
                }
                else if !(ast.exprs[counter].right.is_none() && ast.exprs[counter].symbol.is_none()){
                    ast.exprs[counter].right = parse_expression(toks, true);
                }
                else{
                    error(tok.line, String::from("Parse error on parsing expression ("));
                }
            },
            TokenType::RIGHT_PAREN => {
                if inParen {
                    return Some(ast);
                }
                error(tok.line,String::from("Parser error on parens"));
            }
            TokenType::SHORT_FLAG => {
                let flag = toks.remove(0);
                let mut left: Option<&mut AST> = ast.exprs[counter].left;
                let mut right: Option<&mut AST>  =  ast.exprs[counter].right.as_mut();
                if(flag.t_type != TokenType::WORD){
                    error(flag.line,String::from("Parser error: Invalid flag"));
                }
                if left.unwrap().exprs[0].kind == Kind::CMD{
                    left.unwrap().exprs[0].flags.push(flag.lexeme);
                }
                else if right.unwrap().exprs[0].kind == Kind::CMD{
                    right.unwrap().exprs[0].flags.push(flag.lexeme);
                }
                else{
                    error(tok.line,String::from("Error when parsing flag"));
                }
            },
            TokenType::LONG_FLAG =>{
                let flag = toks.remove(0);
                if (flag.t_type != TokenType::WORD){
                    error::error(flag.line,String::from("Error"));
                }
                ast.exprs[counter].flags.push(flag.lexeme);
            },
            TokenType::SEMICOLON => {
                ast.exprs.push(Expr{
                    kind: Kind::NONE,
                    value: Some(String::new()),
                    flags: Vec::new(),
                    args: Vec::new(),
                    symbol: Some(Symbol::NONE),
                    left:None,
                    right: None
                });
                counter+=1;
            },
            TokenType::PIPE =>{
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none(){
                    error(tok.line,String::from("Binary expression \"|\" encountered parse error."));
                }
            }
            TokenType::REDIR_LEFT =>{
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none(){
                    error(tok.line,String::from("Binary expression \"<\" encountered parse error."));
                }
            },
            TokenType::DOUBLE_REDIR_LEFT=> {
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none(){
                    error(tok.line,String::from("Binary expression \"<<\" encountered parse error."));
                }
            },
            TokenType::DOUBLE_REDIR_RIGHT =>{
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none(){
                    error(tok.line,String::from("Binary expression \">>\" encountered parse error."));
                }
            },
            TokenType::REDIR_RIGHT=>{
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none(){
                    error(tok.line,String::from("Binary expression \">\" encountered parse error."));
                }
            },
            TokenType::POUND => {
                iscomment = true;
            },
            TokenType::STRING => {
            },
            TokenType::WORD =>{
                if ast.exprs[counter].left.is_none(){
                    ast.exprs[counter].left = Some(AST { exprs: Vec::new() });
                    ast.exprs[counter].left.as_mut().unwrap().exprs.push(
                        Expr { 
                            kind: Kind::CMD, 
                            flags: Vec::new(), 
                            value: Some(String::from(tok.lexeme)),
                            args: Vec::new(), 
                            symbol: Some(Symbol::CMD), 
                            left: None, 
                            right: None 
                        });
                }

            },
            _=>{}
        }
    }
    return Some(ast);
}


pub fn parse_program(mut lexer: Scanner){
    //println!("Entering parser\n{:#?}",lexer.tokens);
    println!("{:#?}",parse_expression(&mut lexer.tokens,false));
}