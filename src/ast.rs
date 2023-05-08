#[derive(Debug)]
pub struct AST{
    pub exprs: Vec<Expr>
}


#[derive(Debug)]
pub struct Expr{
    pub kind: Kind,
    pub flags: Vec<String>,
    pub args: Vec<String>,
    pub symbol: Symbol,
    pub value: Option<String>,
    pub left: Option<AST>,
    pub right: Option<AST>,
}

#[derive(Debug,PartialEq,Eq)]
pub enum Kind{
    NONE,
    EXPR,
    VALUE
}

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
    STRING,
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

