use crate::scanner::Scanner;
use crate::token::{WTSType, Token, TokenType};
use crate::ast::{Symbol,AST,Kind,Expr};
use crate::error::{self, error};



fn parse_expression(toks: &mut Vec<Token>,inParen: bool) -> Option<AST>{
    let mut ast: AST = AST { exprs: vec![] };
    let mut iscomment = false;
    ast.exprs.push(Expr{
        kind: Kind::EXPR,
        flags: Vec::new(),
        value: Some(String::new()),
        args: Vec::new(),
        symbol: Symbol::NONE,
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
                else if !(ast.exprs[counter].right.is_none() && ast.exprs[counter].symbol == Symbol::NONE){
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
                if(flag.t_type != TokenType::WORD){
                    error(flag.line,String::from("Parser error: Invalid flag"));
                }
                if ast.exprs[counter].left.as_mut().unwrap().exprs[0].symbol == Symbol::CMD{
                    ast.exprs[counter].left.as_mut().unwrap().exprs[0].flags.push(flag.lexeme);
                }
                else if ast.exprs[counter].right.as_mut().unwrap().exprs[0].symbol == Symbol::CMD{
                    ast.exprs[counter].right.as_mut().unwrap().exprs[0].flags.push(flag.lexeme);
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
                    kind: Kind::EXPR,
                    value: Some(String::new()),
                    flags: Vec::new(),
                    args: Vec::new(),
                    symbol: Symbol::NONE,
                    left:None,
                    right: None
                });
                counter+=1;
            },
            TokenType::PIPE =>{
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none(){
                    error(tok.line,String::from("Binary expression \"|\" encountered parse error."));
                }
                ast.exprs[counter].kind = Kind::EXPR;
                ast.exprs[counter].symbol = Symbol::PIPE;
                ast.exprs[counter].value = Some(tok.lexeme);
            }
            TokenType::REDIR_LEFT =>{
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none(){
                    error(tok.line,String::from("Binary expression \"<\" encountered parse error."));
                }
                ast.exprs[counter].kind = Kind::EXPR;
                ast.exprs[counter].symbol = Symbol::REDIR_LEFT;
                ast.exprs[counter].value = Some(tok.lexeme);
            },
            TokenType::DOUBLE_REDIR_LEFT=> {
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none(){
                    error(tok.line,String::from("Binary expression \"<<\" encountered parse error."));
                }
                ast.exprs[counter].kind = Kind::EXPR;
                ast.exprs[counter].symbol = Symbol::DOUBLE_REDIR_LEFT;
                ast.exprs[counter].value = Some(tok.lexeme);
            },
            TokenType::REDIR_RIGHT=>{
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none(){
                    error(tok.line,String::from("Binary expression \">\" encountered parse error."));
                }
                ast.exprs[counter].kind = Kind::EXPR;
                ast.exprs[counter].symbol = Symbol::REDIR_RIGHT;
                ast.exprs[counter].value = Some(tok.lexeme);
            },
            TokenType::DOUBLE_REDIR_RIGHT =>{
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none(){
                    error(tok.line,String::from("Binary expression \">>\" encountered parse error."));
                }
                ast.exprs[counter].kind = Kind::EXPR;
                ast.exprs[counter].symbol = Symbol::DOUBLE_REDIR_RIGHT;
                ast.exprs[counter].value = Some(tok.lexeme);
            },
            TokenType::POUND => {
                iscomment = true;
            },
            TokenType::WORD | TokenType::STRING =>{
                if ast.exprs[counter].left.is_none(){
                    ast.exprs[counter].left = Some(AST { exprs: Vec::new() });
                    ast.exprs[counter].left.as_mut().unwrap().exprs.push(
                        Expr { 
                            kind: Kind::VALUE, 
                            flags: Vec::new(), 
                            value: Some(String::from(tok.lexeme)),
                            args: Vec::new(), 
                            symbol: Symbol::CMD, 
                            left: None, 
                            right: None 
                        });
                }
                else if ast.exprs[counter].symbol==Symbol::NONE && ast.exprs[counter].left.as_mut().unwrap().exprs[0].symbol == Symbol::CMD{
                    ast.exprs[counter].left.as_mut().unwrap().exprs[0].args.push(tok.lexeme);
                }
                else if ast.exprs[counter].right.is_none(){
                    ast.exprs[counter].right = Some(AST { exprs: Vec::new() });
                    let mut sym = Symbol::NONE;
                    match ast.exprs[counter].symbol {
                        Symbol::REDIR_LEFT =>{
                            sym = Symbol::FILE;
                        },
                        Symbol::DOUBLE_REDIR_LEFT =>{
                            sym = Symbol::STRING;
                        },
                        Symbol::REDIR_RIGHT =>{
                            sym = Symbol::FILE;
                        },
                        Symbol::DOUBLE_REDIR_RIGHT=>{
                            sym = Symbol::FILE;
                        }
                        Symbol::PIPE =>{
                            sym = Symbol::CMD;
                        }
                        _=>{
                            error(tok.line,String::from("Parse error, right hand command without "))
                        }
                    }
                    ast.exprs[counter].right.as_mut().unwrap().exprs.push(
                        Expr { 
                            kind: Kind::VALUE, 
                            flags: Vec::new(), 
                            value: Some(String::from(tok.lexeme)),
                            args: Vec::new(), 
                            symbol: sym, 
                            left: None, 
                            right: None 
                        });
                }
                else if ast.exprs[counter].right.as_mut().unwrap().exprs[0].symbol == Symbol::CMD{
                    ast.exprs[counter].right.as_mut().unwrap().exprs[0].args.push(tok.lexeme);
                }
                else {
                    error(tok.line,String::from("Parse error when handling expression"));
                }

            },
            _=>{}
        }
    }
    return Some(ast);
}


pub fn parse_program(mut lexer: Scanner) -> Option<AST>{
    //println!("Entering parser\n{:#?}",lexer.tokens);
    let ast = parse_expression(&mut lexer.tokens,false);
    println!("{:#?}",ast);
    return ast;
}