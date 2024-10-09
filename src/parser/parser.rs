use crate::{
    source_opcode,
    tokenizer::{LabelDefinition, OperandType, Token, TokenKind},
};
use source_opcode::SourceOpcode;
use std::{
    collections::HashMap,
    iter::Peekable,
    slice::{Iter, IterMut},
};

use super::{
    ExpectedLineDelimeter, ExpectedOpcode, ExpectedOperand, InvalidLabel, ParserError,
    UnexpectedToken,
};

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
            if token.kind == TokenKind::EOF {
                break;
            }
            /*
            Parsed based on the tokens type. On matching an opcode, try and parse it.
            If we see a newline or semicolon we can ignore them as we don't mind
            erroneous line delimeters. If we find any other token, there has been an error.
            */
            match &token.kind {
                TokenKind::Opcode(opcode) => {
                    self.parse_opcode(*opcode)?;
                    self.consume_line_delimeter()?;
                }
                TokenKind::Newline | TokenKind::Semicolon => {}
                _ => {
                    return Err(ParserError::ExpectedOpcode(Box::new(ExpectedOpcode {
                        got: token.clone(),
                    })))
                }
            }
        }
        Ok(())
    }

    /// Try and consume token type in the token stream
    fn consume_token(&mut self, token_type: TokenKind) -> Result<(), ParserError> {
        let token = self.token_iter.peek().unwrap();
        if token.kind == token_type {
            self.token_iter.next();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(Box::new(UnexpectedToken {
                expected: token_type,
                got: (*token).clone(),
            })))
        }
    }

    /// Try and consume a line delimeter in the token stream
    fn consume_line_delimeter(&mut self) -> Result<(), ParserError> {
        let token = self.token_iter.peek().unwrap();
        match token.kind {
            TokenKind::Newline | TokenKind::Semicolon => return Ok(()),
            _ => {
                return Err(ParserError::ExpectedLineDelimeter(Box::new(
                    ExpectedLineDelimeter {
                        got: (*token).clone(),
                    },
                )))
            }
        }
    }

    /// Try and consume an operand in the token stream
    fn consume_operand(&mut self, expected_operand: OperandType) -> Result<u8, ParserError> {
        let token = self.token_iter.peek().unwrap();
        match &token.kind {
            // When consuming a label operand, we must check it exists
            TokenKind::Operand(OperandType::Label, _) => {
                if let Some(val) = self.labels.get(&token.lexeme) {
                    self.token_iter.next();
                    Ok(val.byte)
                } else {
                    Err(ParserError::InvalidLabel(Box::new(InvalidLabel {
                        token: (*token).clone(),
                    })))
                }
            }
            // Consuming other operands - just check equal type
            TokenKind::Operand(actual_operand, val) if *actual_operand == expected_operand => {
                self.token_iter.next();
                Ok(*val)
            }
            _ => Err(ParserError::ExpectedOperand(Box::new(ExpectedOperand {
                expected: vec![expected_operand],
                got: (*token).clone(),
            }))),
        }
    }

    fn write_memory(&mut self, val: u8) {
        let current = self.memory_iter.next().expect("Fatal Error: Provided program is too large to load into memory. This should have been prevented earlier, please report as a bug.");
        *current = val;
    }

    fn parse_opcode(&mut self, assembly_opcode: SourceOpcode) -> Result<(), ParserError> {
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
                    Err(ParserError::InvalidLabel(err)) => {
                        return Err(ParserError::InvalidLabel(err));
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
                    self.consume_token(TokenKind::Comma)?;
                }
            }
        }

        // No match found, incorrect operand at operand_idx
        let potential_operands: Vec<OperandType> = operand_formats
            .iter()
            .map(|(_, operands)| operands.get(operand_idx).expect("This will only fail if an opcode has multiple operand patterns with different lengths!").clone())
            .collect();
        return Err(ParserError::ExpectedOperand(Box::new(ExpectedOperand {
            expected: potential_operands,
            got: (*self.token_iter.peek().unwrap()).clone(),
        })));
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    fn create_tokens_from_token_type(token_types: &[TokenKind]) -> Vec<Token> {
        token_types
            .into_iter()
            .map(|&ty| Token {
                kind: ty,
                lexeme: String::new(),
                line: 0,
                col: 0,
            })
            .collect()
    }

    #[test]
    fn test_all_valid_instructions() {
        for assembly_opcode in SourceOpcode::iter() {
            let operand_formats = assembly_opcode.got_operand_formats();
            for (binary_opcode, operands) in operand_formats {
                // Create input
                let mut input = vec![TokenKind::Opcode(assembly_opcode)];
                for (i, &operand_type) in operands.iter().enumerate() {
                    input.push(TokenKind::Operand(operand_type, 0));
                    if i < operands.len() - 1 {
                        input.push(TokenKind::Comma);
                    }
                }
                input.push(TokenKind::Semicolon);
                input.push(TokenKind::EOF);
                // Expected result - instruction with all zero operands
                let mut expected_result = vec![binary_opcode as u8];
                for _ in 0..operands.len() {
                    expected_result.push(0);
                }
                // Parse
                let tokens = create_tokens_from_token_type(&input);
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

    #[test]
    fn test_missing_line_delimeter() {
        let program = &[
            TokenKind::Opcode(SourceOpcode::ADD),
            TokenKind::Operand(OperandType::Register, 0),
            TokenKind::Comma,
            TokenKind::Operand(OperandType::Register, 0),
            TokenKind::Comma,
            TokenKind::Operand(OperandType::Literal, 0),
            TokenKind::EOF,
        ];
        let tokens = create_tokens_from_token_type(program);
        assert!(matches!(
            Parser::parse(&tokens, &HashMap::new()),
            Err(ParserError::ExpectedLineDelimeter { .. })
        ));
    }

    #[test]
    fn test_expected_opcode() {
        let program = &[TokenKind::Operand(OperandType::Register, 0)];
        let tokens = create_tokens_from_token_type(program);
        assert!(matches!(
            Parser::parse(&tokens, &HashMap::new()),
            Err(ParserError::ExpectedOpcode { .. })
        ));
    }

    #[test]
    fn test_unexpected_token() {
        let program = &[
            TokenKind::Opcode(SourceOpcode::ADD),
            TokenKind::Operand(OperandType::Register, 0),
            TokenKind::Semicolon,
        ];
        let tokens = create_tokens_from_token_type(program);
        assert!(matches!(
            Parser::parse(&tokens, &HashMap::new()),
            Err(ParserError::UnexpectedToken { .. })
        ));
    }

    #[test]
    fn test_expected_operand() {
        let program = &[TokenKind::Opcode(SourceOpcode::ADD), TokenKind::Comma];
        let tokens = create_tokens_from_token_type(program);
        assert!(matches!(
            Parser::parse(&tokens, &HashMap::new()),
            Err(ParserError::ExpectedOperand { .. })
        ));
    }

    #[test]
    fn test_invalid_label() {
        let program = &[
            TokenKind::Opcode(SourceOpcode::B),
            TokenKind::Operand(OperandType::Label, 0),
        ];
        let tokens = create_tokens_from_token_type(program);
        assert!(matches!(
            Parser::parse(&tokens, &HashMap::new()),
            Err(ParserError::InvalidLabel { .. })
        ));
    }
}
