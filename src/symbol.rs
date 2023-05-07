#[derive(Debug,Clone, Copy, PartialEq, Eq)]
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
    FILE,
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
    NONE,
    EOF
}