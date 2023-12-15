use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Operation,
    Hex,
    Address,
    Location,
    Comma,
    Comment,
    Whitespace,
    Newline,
    EndOfFile,
    Unknown,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            TokenType::Operation => "Operation",
            TokenType::Hex => "Hex",
            TokenType::Address => "Address",
            TokenType::Location => "Location",
            TokenType::Comma => "Comma",
            TokenType::Comment => "Comment",
            TokenType::Whitespace => "Whitespace",
            TokenType::Newline => "Newline",
            TokenType::EndOfFile => "EndOfFile",
            TokenType::Unknown => "Unknown",
        })
    }
}

pub enum Keywords {
    MOV,
    HLT,
}