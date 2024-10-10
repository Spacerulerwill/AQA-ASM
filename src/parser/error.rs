use crate::tokenizer::{OperandKind, Token, TokenKind};
use inline_colorization::{color_red, color_reset, style_bold, style_reset};
use std::{collections::HashSet, fmt};

#[derive(Debug)]
pub enum ParserError {
    ExpectedLineDelimeter(Box<ExpectedLineDelimeter>),
    ExpectedOpcode(Box<ExpectedOpcode>),
    ExpectedOperand(Box<ExpectedOperand>),
    UnexpectedToken(Box<UnexpectedToken>),
    InvalidLabel(Box<InvalidLabel>),
}

impl std::error::Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{color_red}{style_bold}")?;
        match self {
            ParserError::ExpectedLineDelimeter(err) => match &err.got {
                Some(token) => write!(
                    f,
                    "Line {}, Column {} :: Expected line delimeter (semicolon or newline), found token {}",
                    token.line,
                    token.col,
                    &token.get_token_debug_repr(),
                ),
                None => write!(
                    f,
                    "Expected line delimeter (semicolon or newline), EOF",
                )
            }
            ParserError::ExpectedOpcode(err) => match &err.got {
                Some(token) => write!(
                    f,
                    "Line {}, Column {} :: Expected instruction opcode, found token {}",
                    token.line,
                    token.col,
                    &token.get_token_debug_repr(),
                ),
                None => write!(f, "Expected instruction opcode, found EOF")
            }
            ParserError::ExpectedOperand(err) => {
                assert!(err.expected.len() > 0);
                let unique_expected: HashSet<OperandKind> =
                    HashSet::from_iter(err.expected.iter().cloned());
                if unique_expected.len() == 1 {
                    match &err.got {
                        Some(token) => write!(
                            f,
                            "Line {}, Column {} :: Expected {:?}, found token {}",
                            token.line,
                            token.col,
                            err.expected[0],
                            &token.get_token_debug_repr(),
                        ),
                        None => write!(
                            f,
                            "Expected {:?}, found EOF",
                            err.expected[0],
                        )
                    }
                } else {
                    let result = unique_expected
                        .iter()
                        .map(|s| format!("\t• {:?}", s))
                        .collect::<Vec<_>>()
                        .join("\n");

                    match &err.got {
                        Some(token) => write!(
                            f,
                            "Line {}, Column {}, Expected one of the following:\n{}\nbut found {}",
                            token.line,
                            token.col,
                            &result,
                            &token.get_token_debug_repr(),
                        ),
                        None => write!(
                            f,
                            "Expected one of the following:\n{}\nbut found EOF",
                            &result,
                        ),
                    }
                }
            }
            ParserError::UnexpectedToken(err) => match &err.got {
                Some(token) => write!(
                    f,
                    "Line {}, Column {} :; Expected {:?}, found {}",
                    token.line,
                    token.col,
                    err.expected,
                    &token.get_token_debug_repr(),
                ),
                None => write!(
                    f,
                    "Expected {:?}, found EOF",
                    err.expected
                )
            }
            ParserError::InvalidLabel(err) => write!(
                f,
                "Line {}, Column {} :: No label exists with name: {}",
                err.token.line,
                err.token.col,
                &err.token.get_token_debug_repr()
            )
        }?;
        write!(f, "{color_reset}{style_reset}")
    }
}

#[derive(Debug)]
pub struct ExpectedLineDelimeter {
    pub got: Option<Token>,
}

#[derive(Debug)]
pub struct ExpectedOpcode {
    pub got: Option<Token>,
}

#[derive(Debug)]
pub struct ExpectedOperand {
    pub expected: Vec<OperandKind>,
    pub got: Option<Token>,
}

#[derive(Debug)]
pub struct UnexpectedToken {
    pub expected: TokenKind,
    pub got: Option<Token>,
}

#[derive(Debug)]
pub struct InvalidLabel {
    pub token: Token,
}
