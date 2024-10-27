use crate::{
    interpreter::instruction::{
        operand::Operand, signature::SIGNATURE_TREE, source_opcode::SourceOpcode,
    },
    tokenizer::{Token, TokenKind},
};
use inline_colorization::{color_red, color_reset, style_bold, style_reset};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    /// Expected an opcode but received something else
    ExpectedOpcode(Box<ExpectedOpcode>),
    /// Expected an operand but received something else
    ExpectedOperand(Box<ExpectedOperand>),
    /// Expected a token out of a list of possible choices but received something else
    ExpectedTokenKind(Box<ExpectedTokenKind>),
    /// Expected an operand but received something else
    InvalidLabel(Box<InvalidLabel>),
    /// Signature for an instruction is incorrect
    InvalidInstructionSignature(Box<InvalidInstructionSignature>),
    /// Same label has been defined in multiple places
    LabelDuplicateDefinition(Box<LabelDuplicateDefinition>),
    /// Program exceeds memory limit (256 bytes),
    ProgramTooLarge,
}

impl std::error::Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{color_red}{style_bold}")?;
        match self {
            ParserError::ExpectedOpcode(err) => write!(
                f,
                "Line {}, Column {} :: Expected instruction opcode but found token {}",
                err.got.line,
                err.got.col,
                &err.got.get_token_debug_repr(),
            ),
            ParserError::ExpectedOperand(err) => match &err.got {
                Some(token) => write!(
                    f,
                    "Line {}, Column {} :: Expected operand but found token {}",
                    token.line,
                    token.col,
                    &token.get_token_debug_repr()
                ),
                None => write!(f, "Expected operand but found EOF"),
            },
            ParserError::ExpectedTokenKind(err) => {
                assert!(err.candidates.len() > 0);
                if err.candidates.len() == 1 {
                    match &err.got {
                        Some(token) => write!(
                            f,
                            "Line {}, Column {} :: Expected {} but found token {}",
                            token.line,
                            token.col,
                            err.candidates[0],
                            &token.get_token_debug_repr()
                        ),
                        None => write!(f, "Expected {} but found EOF", err.candidates[0]),
                    }
                } else if err.candidates.len() == 2 {
                    match &err.got {
                        Some(token) => write!(
                            f,
                            "Line {}, Column {} :: Expected {} or {} but found token {}",
                            token.line,
                            token.col,
                            err.candidates[0],
                            err.candidates[1],
                            &token.get_token_debug_repr()
                        ),
                        None => write!(
                            f,
                            "Expected {} or {} but found EOF",
                            err.candidates[0], err.candidates[1]
                        ),
                    }
                } else {
                    let candidates_string: String = err
                        .candidates
                        .iter()
                        .map(|c| format!("• {}\n", c))
                        .collect();
                    match &err.got {
                        Some(token) => write!(f, "Line {}, Column {} :: Expected one of the following:\n{}but found token {}", token.line, token.col, &candidates_string, &token.get_token_debug_repr()),
                        None => write!(f, "Expected one of the following:\n{}but found EOF", &candidates_string)
                    }
                }
            }
            ParserError::InvalidLabel(err) => write!(
                f,
                "Line {}, Column {} :: No label exists with name: {}",
                err.token.line,
                err.token.col,
                &err.token.get_token_debug_repr()
            ),
            ParserError::InvalidInstructionSignature(err) => {
                let operand_type_strings: Vec<String> = err
                    .received
                    .iter()
                    .map(|operand| operand.get_signature_argument().to_string())
                    .collect();

                let potential_signatures = SIGNATURE_TREE
                    .get_all_valid_operand_combinations_for_source_opcode(err.source_opcode);
                // Convert each inner Vec<SignatureArgument> to a comma-separated string
                let potential_signatures_strings: Vec<String> = potential_signatures
                    .iter()
                    .map(|(_, signature)| {
                        signature
                            .iter()
                            .map(|arg| arg.to_string())
                            .collect::<Vec<String>>() // Collect to a Vec<String>
                            .join(", ") // Ensure this does not add unwanted spaces
                    })
                    .collect();

                write!(
                    f,
                    "Line {}, Column {} :: '{} {}' is not a valid signature! Potential signatures are listed below:\n{}",
                    err.opcode_token.line,
                    err.opcode_token.col,
                    err.source_opcode,
                    operand_type_strings.join(", "),
                    potential_signatures_strings
                        .iter()
                        .map(|signature| format!("• {} {signature}", err.source_opcode))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            ParserError::LabelDuplicateDefinition(err) => write!(
                f,
                "Line {}, Column {} :: Label '{}' defined multiple times",
                err.line,
                err.col,
                &err.name
            ),
            ParserError::ProgramTooLarge => write!(
                f,
                "Program exceeds memory limit (256 bytes)"
            ),
        }?;
        write!(f, "{color_reset}{style_reset}")
    }
}

#[derive(Debug, PartialEq)]
pub struct ExpectedOpcode {
    pub got: Token,
}

#[derive(Debug, PartialEq)]
pub struct ExpectedOperand {
    pub got: Option<Token>,
}

#[derive(Debug, PartialEq)]
pub struct ExpectedTokenKind {
    pub candidates: Vec<TokenKind>,
    pub got: Option<Token>,
}

#[derive(Debug, PartialEq)]
pub struct InvalidLabel {
    pub token: Token,
}

#[derive(Debug, PartialEq)]
pub struct InvalidInstructionSignature {
    pub opcode_token: Token,
    pub source_opcode: SourceOpcode,
    pub received: Vec<Operand>,
}

#[derive(Debug, PartialEq)]
pub struct LabelDuplicateDefinition {
    pub name: String,
    pub line: usize,
    pub col: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        interpreter::instruction::{operand::Operand, source_opcode::SourceOpcode}, tokenizer::{Token, TokenKind}
    };
    use inline_colorization::{color_red, color_reset, style_bold, style_reset};

    #[test]
    fn test_display_parser_error() {
        for (input, expected) in [
            (
                ParserError::ExpectedOpcode(Box::new(ExpectedOpcode {
                    got: Token::new(TokenKind::Comma, ",", 12, 5),
                })),
                "Line 12, Column 5 :: Expected instruction opcode but found token ','",
            ),
            (
                ParserError::ExpectedOperand(Box::new(ExpectedOperand { got: None })),
                "Expected operand but found EOF",
            ),
            (
                ParserError::ExpectedOperand(Box::new(ExpectedOperand {
                    got: Some(Token::new(TokenKind::Semicolon, ";", 4444, 1)),
                })),
                "Line 4444, Column 1 :: Expected operand but found token ';'",
            ),
            (
                ParserError::ExpectedTokenKind(Box::new(ExpectedTokenKind {
                    candidates: vec![TokenKind::Comma],
                    got: None,
                })),
                "Expected comma but found EOF",
            ),
            (
                ParserError::ExpectedTokenKind(Box::new(ExpectedTokenKind {
                    candidates: vec![TokenKind::Comma],
                    got: Some(Token::new(TokenKind::Semicolon, ";", 123, 22)),
                })),
                "Line 123, Column 22 :: Expected comma but found token ';'",
            ),
            (
                ParserError::ExpectedTokenKind(Box::new(ExpectedTokenKind {
                    candidates: vec![TokenKind::Comma, TokenKind::Semicolon],
                    got: None,
                })),
                "Expected comma or semicolon but found EOF",
            ),
            (
                ParserError::ExpectedTokenKind(Box::new(ExpectedTokenKind {
                    candidates: vec![TokenKind::Comma, TokenKind::Semicolon],
                    got: Some(Token::new(
                        TokenKind::Operand(Operand::Register(0)),
                        "R0",
                        1,
                        100,
                    )),
                })),
                "Line 1, Column 100 :: Expected comma or semicolon but found token 'R0'",
            ),
            (
                ParserError::ExpectedTokenKind(Box::new(ExpectedTokenKind {
                    candidates: vec![TokenKind::Comma, TokenKind::Semicolon, TokenKind::Newline],
                    got: None,
                })),
                "Expected one of the following:
• comma
• semicolon
• newline
but found EOF",
            ),
            (
                ParserError::ExpectedTokenKind(Box::new(ExpectedTokenKind {
                    candidates: vec![TokenKind::Comma, TokenKind::Semicolon, TokenKind::Newline],
                    got: Some(Token::new(
                        TokenKind::Operand(Operand::Label),
                        "label",
                        12764,
                        505,
                    )),
                })),
                "Line 12764, Column 505 :: Expected one of the following:
• comma
• semicolon
• newline
but found token 'label'",
            ),
            (
                ParserError::InvalidLabel(Box::new(InvalidLabel {
                    token: Token::new(TokenKind::Operand(Operand::Label), "label", 104, 0),
                })),
                "Line 104, Column 0 :: No label exists with name: 'label'",
            ),
            (
                ParserError::InvalidInstructionSignature(Box::new(InvalidInstructionSignature {
                    opcode_token: Token::new(TokenKind::Opcode(SourceOpcode::HALT), "HALT", 50, 50),
                    source_opcode: SourceOpcode::HALT,
                    received: vec![Operand::Register(0)],
                })),
                "Line 50, Column 50 :: 'HALT register' is not a valid signature! Potential signatures are listed below:
• HALT ",
            ),
            (
                ParserError::InvalidInstructionSignature(Box::new(InvalidInstructionSignature {
                    opcode_token: Token::new(TokenKind::Opcode(SourceOpcode::MOV), "MOV", 500, 20),
                    source_opcode: SourceOpcode::MOV,
                    received: vec![Operand::Register(0), Operand::Register(0), Operand::Register(0)],
                })),
                "Line 500, Column 20 :: 'MOV register, register, register' is not a valid signature! Potential signatures are listed below:
• MOV register, register
• MOV register, literal",
            ),
            (
                ParserError::LabelDuplicateDefinition(Box::new(LabelDuplicateDefinition {
                    name: String::from("test"),
                    line: 1200,
                    col: 130
                })),
                "Line 1200, Column 130 :: Label 'test' defined multiple times"
            ),
            (
                ParserError::ProgramTooLarge,
                "Program exceeds memory limit (256 bytes)"
            )
        ] {
            assert_eq!(
                input.to_string(),
                format!("{color_red}{style_bold}{expected}{color_reset}{style_reset}")
            );
        }
    }
}
