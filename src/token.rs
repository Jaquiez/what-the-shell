#[derive(Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub literal: WTSType,
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
    //Number(i32),
    //EXPR,
    NONE,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    //LEFT_BRACE,
    //RIGHT_BRACE,
    //COMMA,
    //DOT,
    //MINUS,
    //PLUS,
    Semicolon,
    Pound,
    //Star,

    // One or two character tokens.
    ShortFlag,
    LongFlag,
    RedirLeft,
    RedirRight,
    DoubleRedirRight,
    DoubleRedirLeft,
    Pipe,

    // Literals.
    Word,
    String,
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
