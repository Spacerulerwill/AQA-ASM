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
    LabelDefinition,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Operand(operand) => write!(f, "{operand}"),
            TokenKind::Opcode(source_opcode) => write!(f, "{source_opcode}"),
            TokenKind::Newline => write!(f, "newline"),
            TokenKind::Semicolon => write!(f, "semicolon"),
            TokenKind::Comma => write!(f, "comma"),
            TokenKind::LabelDefinition => write!(f, "label definition"),
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
    pub fn new(kind: TokenKind, lexeme: &str, line: usize, col: usize) -> Self {
        Self {
            kind,
            lexeme: lexeme.to_string(),
            line,
            col,
        }
    }
    pub fn get_token_debug_repr(&self) -> String {
        match &self.kind {
            TokenKind::Newline => String::from("'newline'"),
            _ => format!("'{}'", &self.lexeme),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_new() {
        let token = Token::new(TokenKind::Opcode(SourceOpcode::Mov), "MOV", 1, 1);

        assert_eq!(token.kind, TokenKind::Opcode(SourceOpcode::Mov));
        assert_eq!(token.lexeme, "MOV");
        assert_eq!(token.line, 1);
        assert_eq!(token.col, 1);
    }

    #[test]
    fn test_token_kind_display() {
        let operand = TokenKind::Operand(Operand::Register(3));
        let opcode = TokenKind::Opcode(SourceOpcode::Mov);

        assert_eq!(format!("{operand}"), format!("{}", Operand::Register(3)));
        assert_eq!(format!("{opcode}"), format!("{}", SourceOpcode::Mov));
        assert_eq!(format!("{}", TokenKind::Newline), "newline");
        assert_eq!(format!("{}", TokenKind::Semicolon), "semicolon");
        assert_eq!(format!("{}", TokenKind::Comma), "comma");
        assert_eq!(
            format!("{}", TokenKind::LabelDefinition),
            "label definition"
        );
    }

    #[test]
    fn test_token_get_debug_repr() {
        let token_newline = Token::new(TokenKind::Newline, "\n", 1, 1);
        assert_eq!(token_newline.get_token_debug_repr(), "'newline'");
        let token_operand = Token::new(TokenKind::Operand(Operand::Register(3)), "R3", 1, 1);
        assert_eq!(token_operand.get_token_debug_repr(), "'R3'");
    }

    #[test]
    fn test_token_position_default() {
        let pos = TokenPosition::default();
        assert_eq!(pos.idx, 0);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.col, 1);
    }
}
