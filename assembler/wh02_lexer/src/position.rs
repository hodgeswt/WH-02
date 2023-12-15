use core::fmt;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub col: u32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(line {}, col {})", self.line, self.col + 1)
    }
}