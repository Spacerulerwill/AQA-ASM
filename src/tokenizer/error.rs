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
                "Line {}, Column {} :: Program exceeds memory limit (255 bytes)",
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
