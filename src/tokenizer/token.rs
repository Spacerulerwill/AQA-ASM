use std::fmt;

use crate::interpreter::instruction::{operand::Operand, source_opcode::SourceOpcode};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Operand(Operand),
    Opcode(SourceOpcode),
    Newline,
    Semicolon,
    Comma,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Operand(operand) => write!(f, "{operand}"),
            TokenKind::Opcode(source_opcode) => write!(f, "{source_opcode}"),
            TokenKind::Newline => write!(f, "newline"),
            TokenKind::Semicolon => write!(f, "semicolon"),
            TokenKind::Comma => write!(f, "comma"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenPosition {
    pub idx: usize,
    pub line: usize,
    pub col: usize,
}

impl TokenPosition {
    pub fn default() -> Self {
        Self {
            idx: 0,
            line: 1,
            col: 1,
        }
    }
}

impl Token {
    pub fn get_token_debug_repr(&self) -> String {
        match &self.kind {
            TokenKind::Newline => String::from("'newline'"),
            _ => format!("'{}'", &self.lexeme),
        }
    }
}
