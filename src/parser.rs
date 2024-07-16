use crate::tokenizer::{AssemblyOpcode, LabelDefinition, OperandType, Token, TokenType};
use std::{
    collections::HashMap,
    iter::Peekable,
    slice::{Iter, IterMut},
};

#[derive(Debug)]
pub enum ParserError {
    ExpectedLineDelimeter {
        got: Token,
    },
    ExpectedOpcode {
        got: Token,
    },
    UnexpectedToken {
        expected: TokenType,
        got: Token,
    },
    ExpectedOperand {
        expected: Vec<OperandType>,
        got: Token,
    },
    InvalidLabel {
        token: Token,
    },
}

#[derive(Debug)]
pub struct Parser<'a> {
    token_iter: Peekable<Iter<'a, Token>>,
    labels: &'a HashMap<String, LabelDefinition>,
    memory_iter: IterMut<'a, u8>,
}

impl<'a> Parser<'a> {
    /// parse parses the tokens sequence to ensure that the tokens are in a valid order, and loads the instructions
    /// into the memory space. It will return an error if parsing fails and is also responsible for resolving label operands
    /// to their corresponding label. If there is a label operand without an associated label, an error will be returned.
    pub fn parse(
        tokens: &'a Vec<Token>,
        labels: &'a HashMap<String, LabelDefinition>,
    ) -> Result<[u8; 256], ParserError> {
        let mut memory = [0; 256];
        let mut parser = Parser {
            token_iter: tokens.iter().peekable(),
            labels: &labels,
            memory_iter: memory.iter_mut(),
        };
        parser.internal_parse()?;
        Ok(memory)
    }

    fn internal_parse(&mut self) -> Result<(), ParserError> {
        // Parser loop
        loop {
            /*
            We can safely unwrap, as the parser will stop when its reaches the EOF token,
            not when it reaches the end of of the Vec. As long as the token stream ends
            with an EOF token, this will never fail.
            */
            let token = self
                .token_iter
                .next()
                .expect("Fatal error: Most likely cause is token stream without EOF token");
            if token.ty == TokenType::EOF {
                break;
            }
            /*
            Parsed based on the tokens type. On matching an opcode, try and parse it.
            If we see a newline or semicolon we can ignore them as we don't mind
            erroneous line delimeters. If we find any other token, there has been an error.
            */
            match &token.ty {
                TokenType::Opcode(opcode) => {
                    self.parse_opcode(*opcode)?;
                    self.consume_line_delimeter()?;
                }
                TokenType::Newline | TokenType::Semicolon => {}
                _ => return Err(ParserError::ExpectedOpcode { got: token.clone() }),
            }
        }
        Ok(())
    }

    /// Try and consume token type in the token stream
    fn consume_token(&mut self, token_type: TokenType) -> Result<(), ParserError> {
        let token = self.token_iter.peek().unwrap();
        if token.ty == token_type {
            self.token_iter.next();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken {
                expected: token_type,
                got: (*token).clone(),
            })
        }
    }

    /// Try and consume a line delimeter in the token stream
    fn consume_line_delimeter(&mut self) -> Result<(), ParserError> {
        let token = self.token_iter.peek().unwrap();
        match token.ty {
            TokenType::Newline | TokenType::Semicolon => return Ok(()),
            _ => {
                return Err(ParserError::ExpectedLineDelimeter {
                    got: (*token).clone(),
                })
            }
        }
    }

    /// Try and consume an operand in the token stream
    fn consume_operand(&mut self, expected_operand: OperandType) -> Result<u8, ParserError> {
        let token = self.token_iter.peek().unwrap();
        match &token.ty {
            // When consuming a label operand, we must check it exists
            TokenType::Operand(OperandType::Label, _) => {
                if let Some(val) = self.labels.get(&token.lexeme) {
                    self.token_iter.next();
                    Ok(val.byte)
                } else {
                    Err(ParserError::InvalidLabel {
                        token: (*token).clone(),
                    })
                }
            }
            // Consuming other operands - just check equal type
            TokenType::Operand(actual_operand, val) if *actual_operand == expected_operand => {
                self.token_iter.next();
                Ok(*val)
            }
            _ => Err(ParserError::ExpectedOperand {
                expected: vec![expected_operand],
                got: (*token).clone(),
            }),
        }
    }

    fn write_memory(&mut self, val: u8) {
        let current = self.memory_iter.next().expect("Fatal Error: Provided program is too large to load into memory. This should have been prevented earlier, please report as a bug.");
        *current = val;
    }

    fn parse_opcode(&mut self, assembly_opcode: AssemblyOpcode) -> Result<(), ParserError> {
        let operand_formats = assembly_opcode.got_operand_formats();
        let mut opcode_bytes = Vec::new();
        let mut operand_idx = 0;
        for (format_idx, (binary_opcode, format)) in operand_formats.iter().enumerate() {
            // edge case - format has no operands - instant match
            if format.len() == 0 {
                self.write_memory(*binary_opcode as u8);
                return Ok(());
            }
            // Save current token iterator, so we can go back if this format doesn't match
            let iter_save = self.token_iter.clone();
            operand_idx = 0;
            for &operand in format {
                // Consume the operand
                match self.consume_operand(operand) {
                    Ok(val) => {
                        opcode_bytes.push(val);
                        operand_idx += 1;
                    }
                    Err(ParserError::InvalidLabel { token }) => {
                        return Err(ParserError::InvalidLabel { token });
                    }
                    Err(_) => {
                        /*
                        Only reset token iterator if not on the last format
                        we need it to remain on the failing token on the last
                        one for proper error reporting
                        */
                        if format_idx != operand_formats.len() - 1 {
                            self.token_iter = iter_save.clone();
                        }
                        opcode_bytes.clear();
                        break;
                    }
                }
                // we found a match, write instruction
                if operand_idx == format.len() {
                    self.write_memory(*binary_opcode as u8);
                    for byte in opcode_bytes {
                        self.write_memory(byte);
                    }
                    return Ok(());
                } else {
                    self.consume_token(TokenType::Comma)?;
                }
            }
        }

        // No match found, incorrect operand at operand_idx
        let potential_operands: Vec<OperandType> = operand_formats
            .iter()
            .map(|(_, operands)| operands.get(operand_idx).expect("This will only fail if an opcode has multiple operand patterns with different lengths!").clone())
            .collect();
        return Err(ParserError::ExpectedOperand {
            expected: potential_operands,
            got: (*self.token_iter.peek().unwrap()).clone(),
        });
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_all_valid_instructions() {
        for assembly_opcode in AssemblyOpcode::iter() {
            let operand_formats = assembly_opcode.got_operand_formats();
            for (binary_opcode, operands) in operand_formats {
                // Create input
                let mut input = vec![TokenType::Opcode(assembly_opcode)];
                for (i, &operand_type) in operands.iter().enumerate() {
                    input.push(TokenType::Operand(operand_type, 0));
                    if i < operands.len() - 1 {
                        input.push(TokenType::Comma);
                    }
                }
                input.push(TokenType::Semicolon);
                input.push(TokenType::EOF);
                // Expected result - instruction with all zero operands
                let mut expected_result = vec![binary_opcode as u8];
                for _ in 0..operands.len() {
                    expected_result.push(0);
                }
                // Parse
                let tokens: Vec<Token> = input
                    .into_iter()
                    .map(|ty| Token {
                        ty: ty,
                        lexeme: String::new(),
                        line: 0,
                        col: 0,
                    })
                    .collect();
                let memory = Parser::parse(
                    &tokens,
                    &HashMap::from([(
                        String::new(),
                        LabelDefinition {
                            byte: 0,
                            line: 0,
                            col: 0,
                        },
                    )]),
                )
                .unwrap();
                assert!(expected_result
                    .iter()
                    .zip(memory.iter())
                    .take(expected_result.len())
                    .all(|(a, b)| a == b));
            }
        }
    }
}
