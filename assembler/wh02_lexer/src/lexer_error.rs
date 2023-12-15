use std::fmt;

use crate::position::Position;
pub struct LexerError {
    pub message: String,
    pub position: Position,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LexerError at {}: {}", self.position, self.message)
    }
}