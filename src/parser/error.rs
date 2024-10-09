use crate::tokenizer::{OperandType, Token, TokenKind};
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
            ParserError::ExpectedLineDelimeter(err) => write!(
                f,
                "Line {}, Column {} :: Expected line delimeter (semicolon or newline), found token {}",
                err.got.line,
                err.got.col,
                &err.got.get_token_debug_repr(),
            ),
            ParserError::ExpectedOpcode(err) => write!(
                f,
                "Line {}, Column {} :: Expected instruction opcode, found token {}",
                err.got.line,
                err.got.col,
                &err.got.get_token_debug_repr(),
            ),
            ParserError::ExpectedOperand(err) => {
                assert!(err.expected.len() > 0);
                let unique_expected: HashSet<OperandType> =
                    HashSet::from_iter(err.expected.iter().cloned());
                if unique_expected.len() == 1 {
                    write!(
                        f,
                        "Line {}, Column {} :: Unexpected token {}, expected {:?}",
                        err.got.line,
                        err.got.col,
                        &err.got.get_token_debug_repr(),
                        err.expected[0]
                    )
                } else {
                    let result = unique_expected
                        .iter()
                        .map(|s| format!("\t• {:?}", s))
                        .collect::<Vec<_>>()
                        .join("\n");
                    write!(
                        f,
                        "Line {}, Column {}, Unexpected token {}, expected one of the following:\n{}",
                        err.got.line,
                        err.got.col,
                        &err.got.get_token_debug_repr(),
                        &result
                    )
                }
            }
            ParserError::UnexpectedToken(err) => write!(
                f,
                "Line {}, Column {} :; Expected {:?}, found {}",
                err.got.line,
                err.got.col,
                err.expected,
                &err.got.get_token_debug_repr(),
            ),
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
    pub got: Token,
}

#[derive(Debug)]
pub struct ExpectedOpcode {
    pub got: Token,
}

#[derive(Debug)]
pub struct ExpectedOperand {
    pub expected: Vec<OperandType>,
    pub got: Token,
}

#[derive(Debug)]
pub struct UnexpectedToken {
    pub expected: TokenKind,
    pub got: Token,
}

#[derive(Debug)]
pub struct InvalidLabel {
    pub token: Token,
}
