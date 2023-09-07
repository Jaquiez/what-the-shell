use crate::ast::{Expr, Kind, Symbol, AST};
use crate::error::error;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType, WTSType};

fn parse_expression(toks: &mut Vec<Token>, in_paren: bool) -> Option<AST> {
    let mut ast: AST = AST { exprs: vec![] };
    let mut iscomment = false;
    ast.exprs.push(Expr {
        kind: Kind::Expr,
        flags: Vec::new(),
        value: Some(String::new()),
        args: Vec::new(),
        symbol: Symbol::None,
        left: None,
        right: None,
    });
    let mut counter = 0;
    while toks.len() > 0 {
        let tok = toks.remove(0);
        if iscomment {
            continue;
        }
        match tok.t_type {
            TokenType::LeftParen => {
                if ast.exprs[counter].left.is_none() {
                    ast.exprs[counter].left = parse_expression(toks, true);
                } else if !(ast.exprs[counter].right.is_none()
                    && ast.exprs[counter].symbol == Symbol::None)
                {
                    ast.exprs[counter].right = parse_expression(toks, true);
                } else {
                    error(
                        tok.line,
                        String::from("Parse error on parsing expression ("),
                    );
                }
            }
            TokenType::RightParen => {
                if in_paren {
                    return Some(ast);
                }
                error(tok.line, String::from("Parser error on parens"));
            }
            TokenType::ShortFlag => {
                let flag = toks.remove(0);
                if flag.t_type != TokenType::Word {
                    error(flag.line, String::from("Parser error: Invalid flag"));
                }
                if ast.exprs[counter].right.as_mut().is_none()
                    && ast.exprs[counter].left.as_mut().unwrap().exprs[0].symbol == Symbol::Cmd
                {
                    ast.exprs[counter].left.as_mut().unwrap().exprs[0]
                        .flags
                        .push(String::from("-") + &flag.lexeme);
                } else if ast.exprs[counter].right.as_mut().unwrap().exprs[0].symbol == Symbol::Cmd
                {
                    ast.exprs[counter].right.as_mut().unwrap().exprs[0]
                        .flags
                        .push(String::from("-") + &flag.lexeme);
                } else {
                    error(tok.line, String::from("Error when parsing flag"));
                }
            }
            TokenType::LongFlag => {
                let flag = toks.remove(0);
                if flag.t_type != TokenType::Word {
                    error(flag.line, String::from("Parser error: Invalid flag"));
                }
                if ast.exprs[counter].right.as_mut().is_none()
                    && ast.exprs[counter].left.as_mut().unwrap().exprs[0].symbol == Symbol::Cmd
                {
                    ast.exprs[counter].left.as_mut().unwrap().exprs[0]
                        .flags
                        .push(String::from("--") + &flag.lexeme);
                } else if ast.exprs[counter].right.as_mut().unwrap().exprs[0].symbol == Symbol::Cmd
                {
                    ast.exprs[counter].right.as_mut().unwrap().exprs[0]
                        .flags
                        .push(String::from("--") + &flag.lexeme);
                } else {
                    error(tok.line, String::from("Error when parsing flag"));
                }
            }
            TokenType::Semicolon => {
                ast.exprs.push(Expr {
                    kind: Kind::Expr,
                    value: Some(String::new()),
                    flags: Vec::new(),
                    args: Vec::new(),
                    symbol: Symbol::None,
                    left: None,
                    right: None,
                });
                counter += 1;
            }
            TokenType::Pipe => {
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none() {
                    error(
                        tok.line,
                        String::from("Binary expression \"|\" encountered parse error."),
                    );
                }
                ast.exprs[counter].kind = Kind::Expr;
                ast.exprs[counter].symbol = Symbol::Pipe;
                ast.exprs[counter].value = Some(tok.lexeme);
            }
            TokenType::RedirLeft => {
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none() {
                    error(
                        tok.line,
                        String::from("Binary expression \"<\" encountered parse error."),
                    );
                }
                ast.exprs[counter].kind = Kind::Expr;
                ast.exprs[counter].symbol = Symbol::RedirLeft;
                ast.exprs[counter].value = Some(tok.lexeme);
            }
            TokenType::DoubleRedirLeft => {
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none() {
                    error(
                        tok.line,
                        String::from("Binary expression \"<<\" encountered parse error."),
                    );
                }
                ast.exprs[counter].kind = Kind::Expr;
                ast.exprs[counter].symbol = Symbol::DoubleRedirLeft;
                ast.exprs[counter].value = Some(tok.lexeme);
            }
            TokenType::RedirRight => {
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none() {
                    error(
                        tok.line,
                        String::from("Binary expression \">\" encountered parse error."),
                    );
                }
                ast.exprs[counter].kind = Kind::Expr;
                ast.exprs[counter].symbol = Symbol::RedirRight;
                ast.exprs[counter].value = Some(tok.lexeme);
            }
            TokenType::DoubleRedirRight => {
                if ast.exprs[counter].left.is_none() || !ast.exprs[counter].right.is_none() {
                    error(
                        tok.line,
                        String::from("Binary expression \">>\" encountered parse error."),
                    );
                }
                ast.exprs[counter].kind = Kind::Expr;
                ast.exprs[counter].symbol = Symbol::DoubleRedirRight;
                ast.exprs[counter].value = Some(tok.lexeme);
            }
            TokenType::Pound => {
                iscomment = true;
            }
            TokenType::Word | TokenType::String => {
                if ast.exprs[counter].left.is_none() {
                    ast.exprs[counter].left = Some(AST { exprs: Vec::new() });
                    let val = match tok.literal {
                        WTSType::String(s) => s,
                        _ => tok.lexeme,
                    };
                    ast.exprs[counter].left.as_mut().unwrap().exprs.push(Expr {
                        kind: Kind::Value,
                        flags: Vec::new(),
                        value: Some(val),
                        args: Vec::new(),
                        symbol: Symbol::Cmd,
                        left: None,
                        right: None,
                    });
                } else if ast.exprs[counter].symbol == Symbol::None
                    && ast.exprs[counter].left.as_mut().unwrap().exprs[0].symbol == Symbol::Cmd
                {
                    let val = match tok.literal {
                        WTSType::String(s) => s,
                        _ => tok.lexeme,
                    };
                    ast.exprs[counter].left.as_mut().unwrap().exprs[0]
                        .args
                        .push(val);
                } else if ast.exprs[counter].right.is_none() {
                    ast.exprs[counter].right = Some(AST { exprs: Vec::new() });
                    let mut sym = Symbol::None;
                    match ast.exprs[counter].symbol {
                        Symbol::RedirLeft => {
                            sym = Symbol::File;
                        }
                        Symbol::DoubleRedirLeft => {
                            sym = Symbol::String;
                        }
                        Symbol::RedirRight => {
                            sym = Symbol::File;
                        }
                        Symbol::DoubleRedirRight => {
                            sym = Symbol::File;
                        }
                        Symbol::Pipe => {
                            sym = Symbol::Cmd;
                        }
                        _ => error(
                            tok.line,
                            String::from("Parse error, right hand command without "),
                        ),
                    }
                    ast.exprs[counter].right.as_mut().unwrap().exprs.push(Expr {
                        kind: Kind::Value,
                        flags: Vec::new(),
                        value: Some(String::from(tok.lexeme)),
                        args: Vec::new(),
                        symbol: sym,
                        left: None,
                        right: None,
                    });
                } else if ast.exprs[counter].right.as_mut().unwrap().exprs[0].symbol == Symbol::Cmd
                {
                    ast.exprs[counter].right.as_mut().unwrap().exprs[0]
                        .args
                        .push(tok.lexeme);
                } else {
                    error(
                        tok.line,
                        String::from("Parse error when handling expression"),
                    );
                }
            }
            _ => {}
        }
    }
    if in_paren {
        error(0, String::from("Expected closing \")\""));
    }
    return Some(ast);
}

pub fn parse_program(mut lexer: Scanner) -> Option<AST> {
    //println!("Entering parser\n{:#?}",lexer.tokens);
    let ast = parse_expression(&mut lexer.tokens, false);
    return ast;
}
