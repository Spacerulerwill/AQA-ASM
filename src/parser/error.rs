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
}

impl std::error::Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{color_red}{style_bold}")?;
        match self {
            ParserError::ExpectedOpcode(err) => write!(
                f,
                "Line {}, Column {} :: Expected instruction opcode, found token '{}'",
                err.got.line,
                err.got.col,
                &err.got.get_token_debug_repr(),
            ),
            ParserError::ExpectedOperand(err) => match &err.got {
                Some(token) => write!(
                    f,
                    "Line {}, Column {} :: Expected operand, found token '{}'",
                    token.line,
                    token.col,
                    &token.get_token_debug_repr()
                ),
                None => write!(f, "Expected operand, found EOF"),
            },
            ParserError::ExpectedTokenKind(err) => {
                assert!(err.candidates.len() > 0);
                if err.candidates.len() == 1 {
                    match &err.got {
                        Some(token) => write!(
                            f,
                            "Line {}, Column {} :: Expected {} but found token '{}'",
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
                            "Line {}, Column {} :: Expected {} or {} but found token '{}'",
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
                        Some(token) => write!(f, "Line {}, Column {} :: Expected one of the following:\n{}but found token '{}'", token.line, token.col, &candidates_string, &token.get_token_debug_repr()),
                        None => write!(f, "Expected one of the following:\n{} but found EOF", &candidates_string)
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
                    .map(|signature| {
                        signature
                            .iter()
                            .map(|arg| arg.to_string()) // Convert each SignatureArgument to a string
                            .collect::<Vec<String>>() // Collect to a Vec<String>
                            .join(", ")
                    }) // Join the inner Vec<String> into a comma-separated string
                    .collect();

                write!(
                    f,
                    "'{} {}' is not a valid signature! Potential signatures are listed below:\n{}",
                    err.source_opcode,
                    operand_type_strings.join(", "),
                    potential_signatures_strings
                        .iter()
                        .map(|signature| format!("• {} {signature}", err.source_opcode))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
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
    pub source_opcode: SourceOpcode,
    pub received: Vec<Operand>,
}
