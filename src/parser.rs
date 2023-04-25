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

fn parse_string(){

}

fn parse_flag(){

}


fn parse_code(mut toks: Vec<Token>){
    while toks.len() > 0{
        let tok = toks.remove(0);
        match tok.t_type {
            TokenType::LEFT_PAREN => {
                //parse_expression();
            },
            TokenType::RIGHT_PAREN => {
                //end node
            },
            TokenType::SHORT_FLAG => {
                parse_flag();
            },
            TokenType::LONG_FLAG =>{
                parse_flag();
            },
            TokenType::SEMICOLON => {
                //end node!
            },
            TokenType::PIPE =>{

            }
            TokenType::REDIR_LEFT =>{

            },
            TokenType::DOUBLE_REDIR_LEFT=> {
                
            },
            TokenType::DOUBLE_REDIR_RIGHT =>{

            },
            TokenType::REDIR_RIGHT=>{

            },
            TokenType::POUND => {

            },
            TokenType::STRING => {
                //treat like word
            },
            TokenType::WORD =>{

            },
            _=>{}
        }
        println!("{:#?}",tok);
    }
    
}


pub fn parse_program(lexer: Scanner){
    println!("Entering parser\n{:#?}",lexer.tokens);
    parse_code(lexer.tokens);
}