use std::fmt;

use wh02_lexer::position::Position;

use crate::parser_error::ParserError;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Keyword {
    MOV,
    HLT,
    NOP,
    DEF,
    START,
    JMP,
}

impl Keyword {
    pub fn from_str(keyword: &str, position: Position) -> Result<Keyword, ParserError> {
        match keyword {
            "MOV" => Ok(Keyword::MOV),
            "HLT" => Ok(Keyword::HLT),
            "NOP" => Ok(Keyword::NOP),
            "DEF" => Ok(Keyword::DEF),
            "START" => Ok(Keyword::START),
            "JMP" => Ok(Keyword::JMP),
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
            Keyword::START => write!(f, "START"),
            Keyword::JMP => write!(f, "JMP"),
        }
    }
}