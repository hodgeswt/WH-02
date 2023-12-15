use crate::enumerations::TokenType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(value: String, token_type: TokenType) -> Token {
        return Token {
            token_type,
            value,
        };
    }
}