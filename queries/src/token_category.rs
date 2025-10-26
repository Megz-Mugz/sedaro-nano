#[derive(Debug, PartialEq)]
pub enum TokenCategory {
    Prev,
    Root,
    Agent,
    LParen,
    RParen,
    Comma,
    Dot,
    Identifier,
    EOF,
}