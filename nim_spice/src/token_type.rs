#[derive(PartialEq,Debug)]
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
}