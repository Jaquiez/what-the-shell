use crate::scanner::Scanner;
use crate::token::{WTSType, Token, TokenType};
use crate::symbol::Symbol;

pub struct AST{
    tree: Node
}

struct Node{
    symbol: Symbol,
    left: Box<Node>,
    right: Box<Node>
}

fn block(mut toks: Vec<Token>){
    while(toks.len() > 0){
        let tok = toks.remove(0);
        match tok.t_type {
            TokenType::LEFT_PAREN => {},
            TokenType::RIGHT_PAREN => {},
            TokenType::SHORT_FLAG => {},
            TokenType::LONG_FLAG =>{},
            TokenType::SEMICOLON => {},
            TokenType::PIPE =>{}
            TokenType::REDIR_LEFT =>{},
            TokenType::DOUBLE_REDIR_LEFT=> {},
            TokenType::DOUBLE_REDIR_RIGHT =>{},
            TokenType::REDIR_RIGHT=>{},
            TokenType::POUND => {}
            TokenType::STRING => {},
            TokenType::WORD =>{},
            _=>{}
        }
        println!("{:#?}",tok);
    }
    
}


pub fn parse_program(lexer: Scanner){
    println!("Entering parser\n{:#?}",lexer.tokens);
    block(lexer.tokens);
}