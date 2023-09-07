#[derive(Debug)]
pub struct AST {
    pub exprs: Vec<Expr>,
}

#[derive(Debug)]
pub struct Expr {
    pub kind: Kind,
    pub flags: Vec<String>,
    pub args: Vec<String>,
    pub symbol: Symbol,
    pub value: Option<String>,
    pub left: Option<AST>,
    pub right: Option<AST>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    //None,
    Expr,
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    // Single-character tokens.
    RedirLeft,
    RedirRight,
    DoubleRedirRight,
    DoubleRedirLeft,
    //DupRedir,
    Pipe,
    // Literals.
    Cmd,
    String,
    File,
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
    None,
}
