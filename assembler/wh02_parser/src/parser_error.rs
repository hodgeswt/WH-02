use core::fmt;

use wh02_lexer::position::Position;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    pub position: Position,
    pub message: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError at {} {}", self.position, self.message)
    }
}