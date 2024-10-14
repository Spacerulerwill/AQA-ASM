use inline_colorization::{color_red, color_reset, style_bold, style_reset};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    ReadPastMemory,
    OutOfBoundsRead(usize),
    OutOfBoundsWrite(usize),
}

impl std::error::Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{color_red}{style_bold}")?;
        match self {
            RuntimeError::ReadPastMemory => write!(f, "Runtime Error :: Program read past of available memory (perhaps you forgot the 'HALT' instruction?)"),
            RuntimeError::OutOfBoundsRead(idx) => write!(f, "Runtime Error :: Attempt to read memory location {idx}"),
            RuntimeError::OutOfBoundsWrite(idx) => write!(f, "Runtime Error :: Attempt to write to memory location {idx}")
        }?;
        write!(f, "{color_reset}{style_reset}")
    }
}
