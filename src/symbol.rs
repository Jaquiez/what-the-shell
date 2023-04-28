#[derive(Debug,Clone, Copy)]
pub enum Symbol {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    SEMICOLON,
    POUND, 
    STAR,

    REDIR_LEFT,
    REDIR_RIGHT,
    DOUBLE_REDIR_RIGHT,
    DOUBLE_REDIR_LEFT,
    DUP_REDIR,
    PIPE,
    // Literals.
    CMD,
    ARG,
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

    EOF
}