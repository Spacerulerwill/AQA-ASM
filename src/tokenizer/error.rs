use inline_colorization::{color_red, color_reset, style_bold, style_reset};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    /// Program is too big too fit
    ProgramTooLarge(Box<ProgramTooLarge>),
    // Literal with value > 255
    LiteralValueTooLarge(Box<LiteralValueTooLarge>),
    /// Register denoter 'R' without a number following
    MissingNumberAfterRegisterDenoter(Box<MissingNumberAfterRegisterDenoter>),
    /// Literal value '#' without a number following
    MissingNumberAfterLiteralDenoter(Box<MissingNumberAfterLiteralDenoter>),
    /// Invalid register number (greater than REGISTER_COUNT)
    InvalidRegisterNumber(Box<InvalidRegisterNumber>),
    /// A label definition inserted in an incorrect place. They may appear only after newlines or semicolons.
    InvalidLabelDefinitionLocation(Box<InvalidLabelDefinitionLocation>),
    /// Label definition appearing more than once in seperate places
    DuplicateLabelDefinition(Box<DuplicateLabelDefinition>),
    /// Missing a */ delimeter for a block comment
    UnterminatedBlockComment(Box<UnterminatedBlockComment>),
    /// '/' character is an invalid comment denoter
    InvalidCommentDenoter(Box<InvalidCommentDenoter>),
    /// Any invalid character
    UnexpectedCharacter(Box<UnexpectedCharacter>),
}

impl std::error::Error for TokenizerError {}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{color_red}{style_bold}")?;
        match self {
            TokenizerError::ProgramTooLarge(err) => write!(
                f,
                "Line {}, Column {} :: Program exceeds memory limit (256 bytes)",
                err.line,
                err.col
            ),
            TokenizerError::LiteralValueTooLarge(err) => write!(
                f,
                "Line {}, Column {} :: Literal value '{}' too large (max value of 255)",
                err.line,
                err.col,
                &err.value_string
            ),
            TokenizerError::MissingNumberAfterRegisterDenoter(err) => write!(
                f,
                "Line {}, Column {} :: Missing number after register denoter 'R'",
                err.line,
                err.col,
            ),
            TokenizerError::MissingNumberAfterLiteralDenoter(err) => write!(
                f,
                "Line {}, Column {} :: Missing number after literal denoter '#'",
                err.line,
                err.col,
            ),
            TokenizerError::InvalidRegisterNumber(err) => write!(
                f,
                "Line {}, Column {} :: Invalid register 'R{}' (must be in range 0-12 inclusive)",
                err.line,
                err.col,
                err.value,
            ),
            TokenizerError::InvalidLabelDefinitionLocation(err) => write!(
                f,
                "Line {}, Column {} :: Invalid label definition location for label '{}', labels may only appear after line delimiters (newline or ';')",
                err.line,
                err.col,
                &err.label_name
            ),
            TokenizerError::DuplicateLabelDefinition(err) => write!(
                f,
                "Line {}, Column {} :: Definition for label '{}' already exists",
                err.line,
                err.col,
                &err.label_name,
            ),
            TokenizerError::UnterminatedBlockComment(err) => write!(
                f,
                "Line {}, Column {} :: Unterminated block comment begins here",
                err.line,
                err.col,
            ),
            TokenizerError::InvalidCommentDenoter(err) => write!(
                f,
                "Line {}, Column {} :: Expected '//' or '/*' for comment, not '/'",
                err.line,
                err.col,
            ),
            TokenizerError::UnexpectedCharacter(err) => write!(
                f,
                "Line {}, Column {} :: Unexpected character: '{}'",
                err.line,
                err.col,
                err.char
            ),
        }?;
        write!(f, "{color_reset}{style_reset}")
    }
}

#[derive(Debug, PartialEq)]
pub struct ProgramTooLarge {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct LiteralValueTooLarge {
    pub value_string: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct MissingNumberAfterRegisterDenoter {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct MissingNumberAfterLiteralDenoter {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct InvalidRegisterNumber {
    pub value: u8,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct InvalidLabelDefinitionLocation {
    pub label_name: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct DuplicateLabelDefinition {
    pub label_name: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct UnterminatedBlockComment {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct InvalidCommentDenoter {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct UnexpectedCharacter {
    pub char: char,
    pub line: usize,
    pub col: usize,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_display_tokenizer_error() {
        use super::*;

        for (input, expected) in [
            (
                TokenizerError::ProgramTooLarge(Box::new(ProgramTooLarge { line: 1, col: 2 })),
                "Line 1, Column 2 :: Program exceeds memory limit (256 bytes)",
            ),
            (
                TokenizerError::LiteralValueTooLarge(Box::new(LiteralValueTooLarge {
                    value_string: String::from("12345"),
                    line: 36,
                    col: 7778,
                })),
                "Line 36, Column 7778 :: Literal value '12345' too large (max value of 255)",
            ),
            (
                TokenizerError::MissingNumberAfterRegisterDenoter(Box::new(
                    MissingNumberAfterRegisterDenoter {
                        line: 1919,
                        col: 6969,
                    },
                )),
                "Line 1919, Column 6969 :: Missing number after register denoter 'R'",
            ),
            (
                TokenizerError::MissingNumberAfterLiteralDenoter(Box::new(
                    MissingNumberAfterLiteralDenoter { line: 3, col: 4 },
                )),
                "Line 3, Column 4 :: Missing number after literal denoter '#'",
            ),
            (
                TokenizerError::InvalidRegisterNumber(Box::new(InvalidRegisterNumber {
                    value: 0,
                    line: 13,
                    col: 8,
                })),
                "Line 13, Column 8 :: Invalid register 'R0' (must be in range 0-12 inclusive)",
            ),
            (
                TokenizerError::InvalidLabelDefinitionLocation(Box::new(InvalidLabelDefinitionLocation {
                    label_name: String::from("main"),
                    line: 10,
                    col: 5,
                })),
                "Line 10, Column 5 :: Invalid label definition location for label 'main', labels may only appear after line delimiters (newline or ';')",
            ),
            (
                TokenizerError::DuplicateLabelDefinition(Box::new(DuplicateLabelDefinition {
                    label_name: String::from("loop"),
                    line: 6,
                    col: 14,
                })),
                "Line 6, Column 14 :: Definition for label 'loop' already exists",
            ),
            (
                TokenizerError::UnterminatedBlockComment(Box::new(UnterminatedBlockComment {
                    line: 15,
                    col: 20,
                })),
                "Line 15, Column 20 :: Unterminated block comment begins here",
            ),
            (
                TokenizerError::InvalidCommentDenoter(Box::new(InvalidCommentDenoter {
                    line: 4,
                    col: 9,
                })),
                "Line 4, Column 9 :: Expected '//' or '/*' for comment, not '/'",
            ),
            (
                TokenizerError::UnexpectedCharacter(Box::new(UnexpectedCharacter {
                    char: '@',
                    line: 7,
                    col: 11,
                })),
                "Line 7, Column 11 :: Unexpected character: '@'",
            ),
        ] {
            assert_eq!(
                input.to_string(),
                format!("{color_red}{style_bold}{expected}{color_reset}{style_reset}"),
            );
        }
    }
}
