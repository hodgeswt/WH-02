use std::fmt;

use crate::enumerations::TokenType;
use crate::position::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub start_position: Position,
}

impl Token {
    pub fn new(value: String, token_type: TokenType, start_position: Position) -> Token {
        return Token {
            token_type,
            value,
            start_position,
        };
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token at {}: {} ({})", self.start_position, self.value, self.token_type)
    }
}