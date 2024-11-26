#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    SemiColon,
    Identifier,
    Number,
    Unit,

    // Ends
    End,
    Ends,

    // Simulation Commands
    Tran,
    Dc,
    Ac,
    Op,

    Subckt,

    Plot,
    Wave,

    // Units
    T,
    G,
    MEG,
    K,
    M,
    U,
    N,
    P,
    F,

    Equal,
    LParen,
    RParen,
    Comma,
    Add, // used to connect two lines, must be written at the beginning of the line

    Eof,
    E,
    Minus,
    LBrace,
    RBrace,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) content: String,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize, content: String) -> Self {
        Token {
            token_type,
            line,
            column,
            content,
        }
    }
}
