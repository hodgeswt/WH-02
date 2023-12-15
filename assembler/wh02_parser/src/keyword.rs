use std::fmt;

use wh02_lexer::position::Position;

use crate::parser_error::ParserError;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    MOV,
    HLT,
    NOP,
    DEF,
}

impl Keyword {
    pub fn from_str(keyword: &str, position: Position) -> Result<Keyword, ParserError> {
        match keyword {
            "MOV" => Ok(Keyword::MOV),
            "HLT" => Ok(Keyword::HLT),
            "NOP" => Ok(Keyword::NOP),
            "DEF" => Ok(Keyword::DEF),
            _ => Err(ParserError {
                position,
                message: format!("Invalid keyword: {}", keyword),
            }),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Keyword::MOV => write!(f, "MOV"),
            Keyword::HLT => write!(f, "HLT"),
            Keyword::NOP => write!(f, "NOP"),
            Keyword::DEF => write!(f, "DEF"),
        }
    }
}