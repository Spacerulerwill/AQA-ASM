use super::OperandKind;
use crate::source_opcode::SourceOpcode;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Operand(OperandKind, u8),
    Opcode(SourceOpcode),
    Newline,
    Semicolon,
    Comma,
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

#[cfg(test)]
mod tests {
    use super::{Token, TokenKind};

    #[test]
    fn test_get_token_debug_repr() {
        for (input, expected) in [
            (
                Token {
                    kind: TokenKind::Newline,
                    lexeme: String::from("\n"),
                    line: 0,
                    col: 0,
                },
                "'newline'",
            ),
            (
                Token {
                    kind: TokenKind::Comma,
                    lexeme: String::from(","),
                    line: 0,
                    col: 0,
                },
                "','",
            ),
        ] {
            assert_eq!(input.get_token_debug_repr(), expected);
        }
    }
}
