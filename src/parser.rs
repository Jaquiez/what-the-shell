use crate::scanner::Scanner;
use crate::token::{WTSType, Token, TokenType};
use crate::symbol::Symbol;
use crate::error;
pub struct AST{
    cmds: Vec<Node>
}


#[derive(Debug)]
struct Node{
    shell_type: WTSType,
    flags: Vec<String>,
    symbol: Option<Symbol>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn parse_string(){

}



fn parse_expression(mut toks: Vec<Token>) -> Vec<Node>{
    let mut cmds: Vec<Node> = Vec::new();
    cmds.push(Node{
        shell_type: WTSType::NONE,
        flags: Vec::new(),
        symbol: None,
        left:None,
        right: None
    });
    let mut counter = 0;
    while toks.len() > 0{
        let tok = toks.remove(0);
        match tok.t_type {
            TokenType::LEFT_PAREN => {
            },
            TokenType::RIGHT_PAREN => {
                return cmds;
            },
            TokenType::SHORT_FLAG => {
            },
            TokenType::LONG_FLAG =>{
            },
            TokenType::SEMICOLON => {
                cmds.push(Node{
                    shell_type: WTSType::NONE,
                    flags: Vec::new(),
                    symbol: None,
                    left:None,
                    right: None
                });
                counter+=1;
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
            },
            TokenType::WORD =>{
                cmds[counter].symbol = Some(Symbol::CMD);
                cmds[counter].
            },
            _=>{}
        }
    }
    return cmds;
}


pub fn parse_program(lexer: Scanner){
    //println!("Entering parser\n{:#?}",lexer.tokens);
    parse_expression(lexer.tokens);
}