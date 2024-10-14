mod error;
pub use error::*;

use crate::{
    interpreter::instruction::{
        operand::Operand, signature::SIGNATURE_TREE, source_opcode::SourceOpcode,
    },
    tokenizer::{LabelDefinition, Token, TokenKind},
};

use std::{collections::HashMap, iter::Peekable, slice::IterMut, vec::IntoIter};

#[derive(Debug)]
pub struct Parser<'a> {
    token_iter: Peekable<IntoIter<Token>>,
    labels: HashMap<String, LabelDefinition>,
    memory_iter: IterMut<'a, u8>,
}

impl<'a> Parser<'a> {
    /// parse parses the tokens sequence to ensure that the tokens are in a valid order, and loads the instructions
    /// into the memory space. It will return an error if parsing fails and is also responsible for resolving label operands
    /// to their corresponding label. If there is a label operand without an associated label, an error will be returned.
    pub fn parse(
        tokens: Vec<Token>,
        labels: HashMap<String, LabelDefinition>,
    ) -> Result<[u8; 256], ParserError> {
        let mut memory = [0; 256];
        let mut parser = Parser {
            token_iter: tokens.into_iter().peekable(),
            labels: labels,
            memory_iter: memory.iter_mut(),
        };
        parser.internal_parse()?;
        Ok(memory)
    }

    fn internal_parse(&mut self) -> Result<(), ParserError> {
        // Parser loop
        while let Some(token) = self.token_iter.next() {
            /*
            Parsed based on the tokens type. On matching an opcode, try and parse it.
            If we see a newline or semicolon we can ignore them as we don't mind
            erroneous line delimeters. If we find any other token, there has been an error.
            */
            match &token.kind {
                TokenKind::Opcode(opcode) => {
                    self.parse_opcode(*opcode)?;
                }
                TokenKind::Newline | TokenKind::Semicolon => {}
                _ => {
                    return Err(ParserError::ExpectedOpcode(Box::new(ExpectedOpcode {
                        got: Some(token),
                    })))
                }
            }
        }
        Ok(())
    }

    fn consume_operand(&mut self) -> Result<Operand, ParserError> {
        if let Some(token) = self.token_iter.peek() {
            match token.kind {
                TokenKind::Operand(operand) => {
                    self.token_iter.next();
                    Ok(operand)
                }
                _ => Err(ParserError::ExpectedOperand(Box::new(ExpectedOperand {
                    got: Some(token.clone()),
                }))),
            }
        } else {
            return Err(ParserError::ExpectedOperand(Box::new(ExpectedOperand {
                got: None,
            })));
        }
    }

    fn write_memory(&mut self, val: u8) {
        let current = self.memory_iter.next().expect("Fatal Error: Provided program is too large to load into memory. This should have been prevented earlier, please report as a bug.");
        *current = val;
    }

    fn parse_opcode(&mut self, source_opcode: SourceOpcode) -> Result<(), ParserError> {
        let mut operands_and_tokens = Vec::new();

        // Consume first operand, doesn't need a comma before it
        if let Some(TokenKind::Operand(operand)) = self.token_iter.peek().map(|token| token.kind) {
            let token = self.token_iter.next().unwrap();
            operands_and_tokens.push((operand, token));
        }

        // Consume comma seperated operands
        while let Some(TokenKind::Comma) = self.token_iter.peek().map(|token| token.kind) {
            let token = self.token_iter.next().unwrap();
            operands_and_tokens.push((self.consume_operand()?, token));
        }

        // Consume the line delimeter, if anything else found return appropriate errors
        if let Some(token) = self.token_iter.peek() {
            match token.kind {
                // Line delimeter - just consume
                TokenKind::Newline | TokenKind::Semicolon => {
                    self.token_iter.next();
                }
                // Operand, we must be missing a comma between them!
                TokenKind::Operand(_) => {
                    return Err(ParserError::ExpectedTokenKind(Box::new(
                        ExpectedTokenKind {
                            candidates: vec![TokenKind::Comma],
                            got: self.token_iter.next(),
                        },
                    )))
                }
                // Anything else - we must insert a line delimeter between them
                _ => {
                    return Err(ParserError::ExpectedTokenKind(Box::new(
                        ExpectedTokenKind {
                            candidates: vec![TokenKind::Semicolon, TokenKind::Newline],
                            got: self.token_iter.next(),
                        },
                    )))
                }
            }
        } else {
            // Missing line delimeter!s
            return Err(ParserError::ExpectedTokenKind(Box::new(
                ExpectedTokenKind {
                    candidates: vec![TokenKind::Semicolon, TokenKind::Newline],
                    got: None,
                },
            )));
        }

        // Ensure the operands match an operand format for this instruction
        let operands: Vec<Operand> = operands_and_tokens.iter().map(|x| x.0).collect();
        if let Some(runtime_opcode) = SIGNATURE_TREE.matches_signature(source_opcode, &operands) {
            // write opcode
            self.write_memory(runtime_opcode as u8);
            // write all operands
            for (operand, token) in operands_and_tokens {
                match operand {
                    Operand::Literal(val) => self.write_memory(val),
                    Operand::Register(val) => self.write_memory(val),
                    Operand::MemoryRef(val) => self.write_memory(val),
                    // resolve labels
                    Operand::Label => match self.labels.get(&token.lexeme) {
                        Some(label_definition) => self.write_memory(label_definition.byte),
                        _ => {
                            return Err(ParserError::InvalidLabel(Box::new(InvalidLabel { token })))
                        }
                    },
                }
            }
        } else {
            return Err(ParserError::InvalidInstructionSignature(Box::new(
                InvalidInstructionSignature {
                    source_opcode,
                    received: operands,
                },
            )));
        }
        Ok(())
    }
}
