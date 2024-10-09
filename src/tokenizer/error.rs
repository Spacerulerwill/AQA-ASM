use inline_colorization::{color_red, color_reset, style_bold, style_reset};
use std::fmt;

use super::TokenPosition;

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    ProgramTooLarge(Box<ProgramTooLarge>),
    LiteralValueTooLarge(Box<LiteralValueTooLarge>),
    MissingNumberAfterRegisterDenoter(Box<MissingNumberAfterRegisterDenoter>),
    MissingNumberAfterLiteralDenoter(Box<MissingNumberAfterLiteralDenoter>),
    InvalidRegisterNumber(Box<InvalidRegisterNumber>),
    InvalidLabelDefinitionLocation(Box<InvalidLabelDefinitionLocation>),
    DuplicateLabelDefinition(Box<DuplicateLabelDefinition>),
    UnterminatedBlockComment(Box<UnterminatedBlockComment>),
    InvalidCommentDenoter(Box<InvalidCommentDenoter>),
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
                err.position.line,
                err.position.col,
                &err.value_string
            ),
            TokenizerError::MissingNumberAfterRegisterDenoter(err) => write!(
                f,
                "Line {}, Column {} :: Missing number after register denoter 'R'",
                err.position.line,
                err.position.col,
            ),
            TokenizerError::MissingNumberAfterLiteralDenoter(err) => write!(
                f,
                "Line {}, Column {} :: Missing number after literal denoter '#'",
                err.position.line,
                err.position.col,
            ),
            TokenizerError::InvalidRegisterNumber(err) => write!(
                f,
                "Line {}, Column {} :: Invalid register 'R{}' (must be in range 0-12 inclusive)",
                err.position.line,
                err.position.col,
                err.value,
            ),
            TokenizerError::InvalidLabelDefinitionLocation(err) => write!(
                f,
                "Line {}, Column {} :: Invalid label definition location for label '{}', labels may only appear after line delimiters (newline or ';')",
                err.position.line,
                err.position.col,
                &err.label_name
            ),
            TokenizerError::DuplicateLabelDefinition(err) => write!(
                f,
                "Line {}, Column {} :: Definition for label '{}' already exists",
                err.position.line,
                err.position.col,
                &err.label_name,
            ),
            TokenizerError::UnterminatedBlockComment(err) => write!(
                f,
                "Line {}, Column {} :: Unterminated block comment begins here",
                err.position.line,
                err.position.col,
            ),
            TokenizerError::InvalidCommentDenoter(err) => write!(
                f,
                "Line {}, Column {} :: Expected '//' or '/*' for comment, not '/'",
                err.position.line,
                err.position.col,
            ),
            TokenizerError::UnexpectedCharacter(err) => write!(
                f,
                "Line {}, Column {} :: Unexpected character: '{}'",
                err.position.line,
                err.position.col,
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
    pub position: TokenPosition,
}

#[derive(Debug, PartialEq)]
pub struct MissingNumberAfterRegisterDenoter {
    pub position: TokenPosition,
}

#[derive(Debug, PartialEq)]
pub struct MissingNumberAfterLiteralDenoter {
    pub position: TokenPosition,
}

#[derive(Debug, PartialEq)]
pub struct InvalidRegisterNumber {
    pub value: u8,
    pub position: TokenPosition,
}

#[derive(Debug, PartialEq)]
pub struct InvalidLabelDefinitionLocation {
    pub label_name: String,
    pub position: TokenPosition,
}

#[derive(Debug, PartialEq)]
pub struct DuplicateLabelDefinition {
    pub label_name: String,
    pub position: TokenPosition,
}

#[derive(Debug, PartialEq)]
pub struct UnterminatedBlockComment {
    pub position: TokenPosition,
}

#[derive(Debug, PartialEq)]
pub struct InvalidCommentDenoter {
    pub position: TokenPosition,
}

#[derive(Debug, PartialEq)]
pub struct UnexpectedCharacter {
    pub char: char,
    pub position: TokenPosition,
}
