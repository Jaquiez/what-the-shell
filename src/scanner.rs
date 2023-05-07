use crate::error::error;
use crate::token::{WTSType, Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}



//This is for scripting functionality later
lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        //("WTScript",TokenType::SCRIPT),
        //("and", TokenType::AND),
        //("else", TokenType::ELSE),
        //("false", TokenType::FALSE),
        //("for", TokenType::FOR),
        //("if", TokenType::IF),
        //("or", TokenType::OR),
        //("true", TokenType::TRUE),
        //("var", TokenType::VAR),
        //("while", TokenType::WHILE)
    ]);
}


impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn advance(&mut self) -> char {
        let ret: char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        return ret;
    }
    fn add_token(&mut self, t_type: TokenType) {
        let lexeme = &self.source.as_mut_str()[self.start..self.current];
        let tok: Token = Token::new(t_type, WTSType::NONE, String::from(lexeme), self.line);
        self.tokens.push(tok);
    }
    fn match_next(&mut self, expected: char) -> bool {
        if !(self.current < self.source.len()) {
            return false;
        }
        if !(self.source.chars().nth(self.current).unwrap() == expected) {
            return false;
        }
        self.current += 1;
        return true;
    }
    fn peek(&mut self) -> char {
        if !(self.current < self.source.len()) {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }
    fn peekNext(&mut self) -> char {
        if !(self.current + 1 < self.source.len()) {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }
    fn string(&mut self) {
        while self.peek() != '"' && self.current < self.source.len() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.current >= self.source.len() {
            error(self.line, String::from("Unterminated String"));
            return;
        }
        self.advance();
        let lexeme = &self.source.as_mut_str()[self.start..self.current];
        let literal = &lexeme[1..lexeme.len() - 1];
        self.tokens.push(Token::new(
            TokenType::STRING,
            WTSType::String(String::from(literal)),
            String::from(lexeme),
            self.line,
        ));
    }
    /*
        fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peekNext().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let lexeme = &self.source.as_mut_str()[self.start..self.current];
        self.tokens.push(Token::new(
            TokenType::IONUMBER,
            WTSType::Number(lexeme.parse::<i32>().unwrap()),
            String::from(lexeme),
            self.line,
        ));
    }
    */

    fn identifier(&mut self) {
        let keys = vec!['(',')','-',';','#','>','<','#'];
        while !self.peek().is_whitespace() && !keys.contains(&self.peek()) {
            self.advance();
        }
        let key = &self.source.as_mut_str()[self.start..self.current];
        let type_of = KEYWORDS.get(key);
        if type_of.is_none() {
            self.add_token(TokenType::WORD);
        } else {
            self.add_token(*type_of.unwrap());
        }
    }
    pub fn scan_tokens(&mut self) {
        while self.current < self.source.len() {
            self.start = self.current;
            let c = self.advance();
            match c {
                '(' => self.add_token(TokenType::LEFT_PAREN),
                ')' => self.add_token(TokenType::RIGHT_PAREN),
                //'{' => self.add_token(TokenType::LEFT_BRACE),
                //'}' => self.add_token(TokenType::RIGHT_BRACE),
                //',' => self.add_token(TokenType::COMMA),
                '-' => {
                    if self.match_next('-') {
                        self.add_token(TokenType::LONG_FLAG);
                    } else {
                        self.add_token(TokenType::SHORT_FLAG);
                    }
                },
                //'+' => self.add_token(TokenType::PLUS),
                ';' => self.add_token(TokenType::SEMICOLON),
                //'*' => self.add_token(TokenType::STAR),
                //'!' => {
                //    if self.match_next('=') {
                //        self.add_token(TokenType::BANG_EQUAL);
                //    } else {
                //        self.add_token(TokenType::BANG);
                //    }
                //}
                //'=' => {
                //    if self.match_next('=') {
                //        self.add_token(TokenType::EQUAL_EQUAL);
                //    } else {
                //        self.add_token(TokenType::EQUAL);
                //    }
                //}
                '|' =>{
                    self.add_token(TokenType::PIPE);
                }
                '<' => {
                    if self.match_next('<') {
                        self.add_token(TokenType::DOUBLE_REDIR_LEFT);
                    } else {
                        self.add_token(TokenType::REDIR_LEFT);
                    }
                }
                '>' => {
                    if self.match_next('>') {
                        self.add_token(TokenType::DOUBLE_REDIR_RIGHT);
                    } else {
                        self.add_token(TokenType::REDIR_RIGHT);
                    }
                }
                '#' => {
                    if self.match_next('#') {
                        while self.peek() != '\n' && self.current < self.source.len() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::POUND);
                    }
                }
                '\n' => self.line += 1,
                ' ' => (),
                '\r' => (),
                '\t' => (),
                '"' => self.string(),
                _ => {
                    //For now treat them the same
                    self.identifier();
                    //error(self.line, format!("Unexpected character: '{c}'."));
                }
            }
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            WTSType::NONE,
            String::new(),
            self.line,
        ));
    }
}
