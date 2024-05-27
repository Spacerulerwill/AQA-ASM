use crate::tokenizer::{AssemblyOpcode, LabelDefinition, Operand, Token, TokenType};
use std::{
    collections::HashMap,
    iter::Peekable,
    slice::{Iter, IterMut}
};

#[derive(Debug)]
pub enum ParserError {
    ExpectedLineDelimeter { got: Token },
    ExpectedOpcode { got: Token },
    UnexpectedToken { expected: TokenType, got: Token },
    ExpectedOperand { expected: Vec<Operand>, got: Token },
    InvalidLabel { token: Token },
}

#[derive(Debug)]
struct ParserState<'a> {
    token_iter: Peekable<Iter<'a, Token>>,
    labels: &'a HashMap<String, LabelDefinition>,
    memory_iter: IterMut<'a, u8>,
}

impl<'a> ParserState<'a> {
    fn new(
        tokens: &'a Vec<Token>,
        labels: &'a HashMap<String, LabelDefinition>,
        memory: &'a mut [u8; 256],
    ) -> ParserState<'a> {
        ParserState {
            token_iter: tokens.iter().peekable(),
            labels: labels,
            memory_iter: memory.iter_mut(),
        }
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
    fn consume_operand(&mut self, expected_operand: Operand) -> Result<u8, ParserError> {
        let token = self.token_iter.peek().unwrap();
        match &token.ty {
            // When consuming a label operand, we must check it exists
            TokenType::Operand(Operand::Label, _) => {
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
}

/// parse_load parses the tokens sequence to ensure that the tokens are in a valid order, and loads the instructions
/// into the memory space. It will return an error if parsing fails and is also responsible for resolving label operands
/// to their corresponding label. If there is a label operand without an associated label, an error will be returned.
pub fn parse(
    memory: &mut [u8; 256],
    tokens: &Vec<Token>,
    labels: &HashMap<String, LabelDefinition>,
) -> Result<(), ParserError> {
    let mut state = ParserState::new(tokens, labels, memory);

    fn parse_opcode(
        state: &mut ParserState,
        assembly_opcode: AssemblyOpcode,
    ) -> Result<(), ParserError> {
        let operand_formats = assembly_opcode.got_operand_formats();
        let mut opcode_bytes = Vec::new();
        let mut operand_idx = 0;
        for (format_idx, (binary_opcode, format)) in operand_formats.iter().enumerate() {
            // edge case - format has no operands - instant match
            if format.len() == 0 {
                state.write_memory(*binary_opcode as u8);
                return Ok(());
            }
            // Save current token iterator, so we can go back if this format doesn't match
            let iter_save = state.token_iter.clone();
            operand_idx = 0;
            for &operand in format {
                // Consume the operand
                match state.consume_operand(operand) {
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
                            state.token_iter = iter_save.clone();
                        }
                        opcode_bytes.clear();
                        break;
                    }
                }
                // we found a match, write instruction
                if operand_idx == format.len() {
                    state.write_memory(*binary_opcode as u8);
                    for byte in opcode_bytes {
                        state.write_memory(byte);
                    }
                    return Ok(());
                } else {
                    state.consume_token(TokenType::Comma)?;
                }
            }
        }

        // No match found, incorrect operand at operand_idx
        let potential_operands: Vec<Operand> = operand_formats
            .iter()
            .map(|(_, operands)| operands.get(operand_idx).expect("This will only fail if an opcode has multiple operand patterns with different lengths!").clone())
            .collect();
        return Err(ParserError::ExpectedOperand {
            expected: potential_operands,
            got: (*state.token_iter.peek().unwrap()).clone(),
        });
    }

    // Parser loop
    loop {
        /*
        We can safely unwrap, as the parser will stop when its reaches the EOF token,
        not when it reaches the end of of the Vec. As long as the token stream ends
        with an EOF token, this will never fail.
        */
        let token = state
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
                parse_opcode(&mut state, *opcode)?;
                state.consume_line_delimeter()?;
            }
            TokenType::Newline | TokenType::Semicolon => {}
            _ => return Err(ParserError::ExpectedOpcode { got: token.clone() }),
        }
    }
    Ok(())
}
