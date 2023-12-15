use std::fmt;

use wh02_lexer::position::Position;

use crate::parser_error::ParserError;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Operand {
    pub indicator: char,
    pub value: String,
}

impl Operand {
    pub fn get_indicator(operand: &str, position: Position) -> Result<char, ParserError> {
        let indicator = operand.chars().nth(0);
        match indicator {
            Some(indicator) => {
                Ok(indicator)
            },
            None => Err(ParserError {
                position,
                message: "Expected to find operand with len > 0, found len = 0".to_string(),
            }),
        }
    }

    pub fn from_str(keyword: &str, position: Position) -> Result<Operand, ParserError> {
        let indicator = Self::get_indicator(keyword, position)?;
        let value = keyword[1..].to_string();

        Ok(Operand {
            indicator,
            value,
        })
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.indicator, self.value)
    }
}