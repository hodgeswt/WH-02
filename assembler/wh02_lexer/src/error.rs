use crate::position::Position;

pub struct Error {
    pub message: String,
    pub position: Position,
}