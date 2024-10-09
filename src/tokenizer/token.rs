use super::OperandType;
use crate::source_opcode::SourceOpcode;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Operand(OperandType, u8),
    Opcode(SourceOpcode),
    Newline,
    Semicolon,
    Comma,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn get_token_debug_repr(&self) -> String {
        match &self.kind {
            TokenKind::Newline => String::from("'newline'"),
            TokenKind::EOF => String::from("'end of file'"),
            _ => format!("'{}'", &self.lexeme),
        }
    }
}
