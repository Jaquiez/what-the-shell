#[derive(Debug)]
pub struct Token {
    pub t_type: TokenType,
    literal: WTSType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(t_type: TokenType, literal: WTSType, lexeme: String, line: usize) -> Self {
        Self {
            t_type,
            literal,
            lexeme,
            line,
        }
    }
}

#[derive(Debug)]
pub enum WTSType {
    String(String),
    Number(i32),
    NONE
}
#[derive(Debug,Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    //LEFT_BRACE,
    //RIGHT_BRACE,
    //COMMA,
    //DOT,
    //MINUS,
    //PLUS,
    SEMICOLON,
    POUND, 
    STAR,
    
    // One or two character tokens.
    SHORT_FLAG,
    LONG_FLAG,
    REDIR_LEFT,
    REDIR_RIGHT,
    DOUBLE_REDIR_RIGHT,
    DOUBLE_REDIR_LEFT,
    DUP_REDIR,
    PIPE,


    // Literals.
    WORD,
    STRING,
    IONUMBER,
    // Keywords.

    //SCRIPT,
    //AND,
    //ELSE,
    //FOR,
    //IF,
    //OR,
    //TRUE,
    //VAR,
    //WHILE,

    EOF,
}