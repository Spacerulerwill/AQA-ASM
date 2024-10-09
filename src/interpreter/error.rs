use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {
    ReadPastMemory,
    OutOfBoundsRead(usize),
    OutOfBoundsWrite(usize),
}

impl std::error::Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::ReadPastMemory => write!(f, "Runtime Error :: Program read past of available memory (perhaps you forgot the 'HALT' instruction?)"),
            RuntimeError::OutOfBoundsRead(idx) => write!(f, "Runtime Error :: Attempt to read memory location {idx}"),
            RuntimeError::OutOfBoundsWrite(idx) => write!(f, "Runtime Error :: Attempt to write to memory location {idx}")
        }
    }
}