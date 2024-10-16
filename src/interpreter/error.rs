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
            RuntimeError::OutOfBoundsRead(idx) => write!(f, "Runtime Error :: Attempt to read out of bounds memory location {idx}"),
            RuntimeError::OutOfBoundsWrite(idx) => write!(f, "Runtime Error :: Attempt to write to out of bounds memory location {idx}")
        }?;
        write!(f, "{color_reset}{style_reset}")
    }
}

#[cfg(test)]
mod tests {
    use inline_colorization::{color_red, color_reset, style_bold, style_reset};

    use super::RuntimeError;

    #[test]
    fn test_display_runtime_error() {
        for (input, expected) in [
            (RuntimeError::ReadPastMemory, "Runtime Error :: Program read past of available memory (perhaps you forgot the 'HALT' instruction?)"),
            (RuntimeError::OutOfBoundsRead(12), "Runtime Error :: Attempt to read out of bounds memory location 12"),
            (RuntimeError::OutOfBoundsWrite(127), "Runtime Error :: Attempt to write to out of bounds memory location 127")
        ] {
            assert_eq!(format!("{}", input), format!("{color_red}{style_bold}{expected}{color_reset}{style_reset}"));
        }
    }
}
