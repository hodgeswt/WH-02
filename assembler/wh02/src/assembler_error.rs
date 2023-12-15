use core::fmt;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AssemblerError {
    pub message: String,
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AssemblerError: {}", self.message)
    }
}