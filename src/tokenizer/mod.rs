mod error;
pub use error::*;
mod token;
pub use token::*;

use crate::interpreter::{
        instruction::{operand::Operand, source_opcode::SourceOpcode},
        REGISTER_COUNT,
    };
use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

#[derive(Debug)]
pub struct Tokenizer<'a> {
    pub tokens: Vec<Token>,
    input: &'a str,
    iter: Peekable<Chars<'a>>,
    prev_pos: TokenPosition,
    current_pos: TokenPosition,
    tabsize: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(input: &'a str, tabsize: u8) -> Result<Self, TokenizerError> {
        let mut tokenizer = Tokenizer {
            tokens: Vec::new(),
            input,
            iter: input.chars().peekable(),
            prev_pos: TokenPosition::default(),
            current_pos: TokenPosition::default(),
            tabsize: tabsize as usize,
        };
        tokenizer.interal_tokenize()?;
        Ok(tokenizer)
    }

    /// tokenize takes in an input source code string and returns a Vec of tokens and
    /// a `HashMap` of label definitions to their corresponding byte position and the
    /// amount of bytes the resulting progam will use. There are multiple types of token:
    /// * Newlines / Semicolons (both are used a line delimeters)
    /// * Commas
    /// * Memory references (An unsigned 8 bit number e.g 12)
    /// * Literals (A '#' follwed by an unsigned 8 bit number e.g. #12)
    /// * Registers (An 'R' followed by a number in the range 0 to REGISTER_COUNT-1)
    /// * Opcodes (A string of chars that make any of our opcodes)
    /// * Label operands (A string of chars)
    /// * Label definitions (A string of chars followed by a colon)
    /// 
    /// Label checking does not happen in this stage. All label operands are initialised
    /// with the value 0. The next stage, parsing and instruction loading, will verify
    /// that labels are correct and exist when using them.
    /// The tokenzier will terminate early if it detects that too many bytes for the program
    /// have been loaded. Max 255.
    fn interal_tokenize(&mut self) -> Result<(), TokenizerError> {
        // Main tokenization loop
        while let Some(&ch) = self.iter.peek() {
            // Ignore any whitespace characters
            if ch != '\n' && ch.is_whitespace() {
                self.next();
                self.prev_pos = self.current_pos.clone();
                continue;
            }

            match ch {
                '\n' => self.add_single_char_token(TokenKind::Newline),
                ';' => self.add_single_char_token(TokenKind::Semicolon),
                ',' => self.add_single_char_token(TokenKind::Comma),
                '0'..='9' => self.tokenize_memory_reference()?,
                'a'..='z' | 'A'..='Z' | '_' => self.tokenize_identifier()?,
                '#' => self.tokenize_literal()?,
                '/' => self.comment()?,
                ch => {
                    return Err(TokenizerError::UnexpectedCharacter(Box::new(
                        UnexpectedCharacter {
                            char: ch,
                            line: self.prev_pos.line,
                            col: self.prev_pos.col,
                        },
                    )))
                }
            };
        }
        Ok(())
    }

    /// Consume character, adjusting token positions accordingly
    fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.iter.next() {
            match ch {
                '\n' => {
                    self.current_pos.line += 1;
                    self.current_pos.col = 1;
                }
                '\t' => self.current_pos.col += self.tabsize,
                _ => self.current_pos.col += 1,
            }
            self.current_pos.idx += ch.len_utf8();
            return Some(ch);
        }
        None
    }

    /// Consume a string of characters while a condition is met
    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut string = String::new();
        while let Some(&ch) = self.iter.peek() {
            if !condition(ch) {
                break;
            }
            string.push(ch);
            self.next();
        }
        string
    }

    fn add_token(&mut self, kind: TokenKind) {
        let mut lexeme = self.input[self.prev_pos.idx..self.current_pos.idx].to_string();
        if lexeme == "\n" {
            lexeme = String::from("\\n");
        }
        self.tokens.push(Token::new(
            kind,
            &lexeme,
            self.prev_pos.line,
            self.prev_pos.col,
        ));
        self.prev_pos = self.current_pos.clone();
    }

    fn consume_u8(&mut self) -> Option<Result<u8, TokenizerError>> {
        let value_string = self.consume_while(|ch| ch.is_ascii_digit());
        if value_string.is_empty() {
            None
        } else {
            match value_string.parse::<u8>() {
                Ok(value) => Some(Ok(value)),
                Err(_) => {
                    Some(Err(TokenizerError::LiteralValueTooLarge(Box::new(
                        LiteralValueTooLarge {
                            value_string,
                            line: self.prev_pos.line,
                            col: self.prev_pos.col,
                        },
                    ))))
                }
            }
        }
    }

    fn add_single_char_token(&mut self, kind: TokenKind) {
        self.next();
        self.add_token(kind);
    }

    fn tokenize_memory_reference(&mut self) -> Result<(), TokenizerError> {
        let value = self.consume_u8().unwrap()?;
        self.add_token(TokenKind::Operand(Operand::MemoryRef(value)));
        Ok(())
    }

    fn tokenize_literal(&mut self) -> Result<(), TokenizerError> {
        self.next();
        match self.consume_u8() {
            Some(Ok(val)) => {
                self.add_token(TokenKind::Operand(Operand::Literal(val)));
                Ok(())
            },
            Some(Err(err)) => Err(err),
            None => {
                Err(TokenizerError::MissingNumberAfterLiteralDenoter(Box::new(
                    MissingNumberAfterLiteralDenoter {
                        line: self.prev_pos.line,
                        col: self.prev_pos.col,
                    },
                )))
            }
        }
    }

    fn tokenize_identifier(&mut self) -> Result<(), TokenizerError> {
        let identifier = self.consume_while(|ch| ch.is_alphabetic() || ch == '_');
        // Register
        if identifier == "R" {
            return match self.consume_u8() {
                Some(Ok(val)) => {
                    if val >= REGISTER_COUNT {
                        return Err(TokenizerError::InvalidRegisterNumber(Box::new(
                            InvalidRegisterNumber {
                                value: val,
                                line: self.prev_pos.line,
                                col: self.prev_pos.col,
                            },
                        )));
                    }
                    self.add_token(TokenKind::Operand(Operand::Register(val)));
                    Ok(())
                }
                Some(Err(err)) => Err(err),
                None => Err(TokenizerError::MissingNumberAfterRegisterDenoter(Box::new(
                    MissingNumberAfterRegisterDenoter {
                        line: self.prev_pos.line,
                        col: self.prev_pos.col,
                    },
                ))),
            };
        }
        // Is it an opcode?
        if let Ok(source_opcode) = SourceOpcode::from_str(&identifier) {
            self.add_token(TokenKind::Opcode(source_opcode));
            return Ok(());
        }
        // Is it a label definition? (i.e a ':' follows it)
        if let Some(':') = self.iter.peek() {
            // Move past the ':'
            self.next();
            self.add_token(TokenKind::LabelDefinition);
            return Ok(());
        }
        // It's a label operand
        self.add_token(TokenKind::Operand(Operand::Label));
        Ok(())
    }

    fn comment(&mut self) -> Result<(), TokenizerError> {
        self.next();
        match self.iter.peek() {
            // Comments starts with // therefore its a line comment
            Some('/') => {
                self.next();
                while let Some(&ch) = self.iter.peek() {
                    if ch == '\n' {
                        break;
                    } 
                    self.next();
                }
            }
            // Comment starts with a /* so its multiline
            Some('*') => loop {
                match self.next() {
                    Some('*') => {
                        if self.iter.peek() == Some(&'/') {
                            self.next(); // Consume the '/'
                            break; // Exit the loop
                        }
                    }
                    Some(_) => continue, // Continue if it's not '*'
                    None => {
                        return Err(TokenizerError::UnterminatedBlockComment(Box::new(
                            UnterminatedBlockComment {
                                line: self.prev_pos.line,
                                col: self.prev_pos.col,
                            },
                        )))
                    }
                }
            },
            // A single / by itself is dumb, lets tell the user off
            _ => {
                return Err(TokenizerError::InvalidCommentDenoter(Box::new(
                    InvalidCommentDenoter {
                        line: self.prev_pos.line,
                        col: self.prev_pos.col,
                    },
                )))
            }
        }
        self.prev_pos = self.current_pos.clone();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::instruction::source_opcode::SourceOpcode;

    use super::*;

    /// Given a Vec<Token> return a Vec<TokenType> (the token type for each token)
    fn extract_token_types(tokens: Vec<Token>) -> Vec<TokenKind> {
        tokens.into_iter().map(|token| token.kind).collect()
    }

    /// Given a Vec<Token> return vec of tuples of line and column numbers for each token
    fn extract_token_line_column_numbers(tokens: Vec<Token>) -> Vec<(usize, usize)> {
        tokens
            .into_iter()
            .map(|token| (token.line, token.col))
            .collect()
    }

    /// Test tokenizer produce correct token type sequence for given input string
    fn test_token_type_sequence(input: &str, expected: &[TokenKind]) {
        let token_types = extract_token_types(Tokenizer::tokenize(input, 4).unwrap().tokens);
        assert_eq!(token_types, expected);
    }

    #[test]
    fn test_tokens_isolated() {
        for (input, expected_output) in [
            ("\n", TokenKind::Newline),
            (";", TokenKind::Semicolon),
            (",", TokenKind::Comma),
            ("123", TokenKind::Operand(Operand::MemoryRef(123))),
            ("#12", TokenKind::Operand(Operand::Literal(12))),
            ("R3", TokenKind::Operand(Operand::Register(3))),
            ("label_operand", TokenKind::Operand(Operand::Label)),
            ("NOP", TokenKind::Opcode(SourceOpcode::Nop)),
            ("LDR", TokenKind::Opcode(SourceOpcode::Ldr)),
            ("STR", TokenKind::Opcode(SourceOpcode::Str)),
            ("ADD", TokenKind::Opcode(SourceOpcode::Add)),
            ("SUB", TokenKind::Opcode(SourceOpcode::Sub)),
            ("MOV", TokenKind::Opcode(SourceOpcode::Mov)),
            ("CMP", TokenKind::Opcode(SourceOpcode::Cmp)),
            ("B", TokenKind::Opcode(SourceOpcode::B)),
            ("BEQ", TokenKind::Opcode(SourceOpcode::Beq)),
            ("BNE", TokenKind::Opcode(SourceOpcode::Bne)),
            ("BGT", TokenKind::Opcode(SourceOpcode::Bgt)),
            ("BLT", TokenKind::Opcode(SourceOpcode::Blt)),
            ("AND", TokenKind::Opcode(SourceOpcode::And)),
            ("ORR", TokenKind::Opcode(SourceOpcode::Orr)),
            ("EOR", TokenKind::Opcode(SourceOpcode::Eor)),
            ("MVN", TokenKind::Opcode(SourceOpcode::Mvn)),
            ("LSL", TokenKind::Opcode(SourceOpcode::Lsl)),
            ("LSR", TokenKind::Opcode(SourceOpcode::Lsr)),
            ("PRINT", TokenKind::Opcode(SourceOpcode::Print)),
            ("INPUT", TokenKind::Opcode(SourceOpcode::Input)),
            ("HALT", TokenKind::Opcode(SourceOpcode::Halt)),
        ] {
            test_token_type_sequence(input, &[expected_output]);
        }
    }

    #[test]
    fn test_comment_line_single() {
        test_token_type_sequence("NOP // Comment", &[TokenKind::Opcode(SourceOpcode::Nop)]);
    }

    #[test]
    fn test_comment_line_multiline() {
        test_token_type_sequence(
            "NOP /* Multiline \n Comment */",
            &[TokenKind::Opcode(SourceOpcode::Nop)],
        );
    }

    #[test]
    fn test_comment_block_multiline() {
        test_token_type_sequence(
            "NOP /* Multiline \n Comment \n */ NOP",
            &[
                TokenKind::Opcode(SourceOpcode::Nop),
                TokenKind::Opcode(SourceOpcode::Nop),
            ],
        );
    }

    #[test]
    fn test_empty_program() {
        test_token_type_sequence("", &[]);
    }

    #[test]
    fn test_missing_register_number() {
        assert_eq!(
            Tokenizer::tokenize("MOV R #23", 4).unwrap_err(),
            TokenizerError::MissingNumberAfterRegisterDenoter(Box::new(
                MissingNumberAfterRegisterDenoter { line: 1, col: 5 }
            ))
        );
    }

    #[test]
    fn test_missing_literal_number() {
        assert_eq!(
            Tokenizer::tokenize("MOV R5 #", 4).unwrap_err(),
            TokenizerError::MissingNumberAfterLiteralDenoter(Box::new(
                MissingNumberAfterLiteralDenoter { line: 1, col: 8 }
            ))
        );
    }

    #[test]
    fn test_unterminated_block_comment() {
        // No ending delimeter (*/)
        assert_eq!(
            Tokenizer::tokenize("MOV R5 #23 /* this is an unterminated block comment", 4)
                .unwrap_err(),
            TokenizerError::UnterminatedBlockComment(Box::new(UnterminatedBlockComment {
                line: 1,
                col: 12
            }))
        );
        // Half an ending delimeter
        assert_eq!(
            Tokenizer::tokenize("MOV R5 #23 /* this is an unterminated block comment*", 4)
                .unwrap_err(),
            TokenizerError::UnterminatedBlockComment(Box::new(UnterminatedBlockComment {
                line: 1,
                col: 12
            }))
        );
    }

    #[test]
    fn test_invalid_comment_denoter() {
        assert_eq!(
            Tokenizer::tokenize("/", 4).unwrap_err(),
            TokenizerError::InvalidCommentDenoter(Box::new(InvalidCommentDenoter {
                line: 1,
                col: 1
            }))
        );
    }

    #[test]
    fn test_invalid_register_number() {
        assert_eq!(
            Tokenizer::tokenize("R13", 4).unwrap_err(),
            TokenizerError::InvalidRegisterNumber(Box::new(InvalidRegisterNumber {
                value: 13,
                line: 1,
                col: 1
            }))
        );

        assert_eq!(
            Tokenizer::tokenize("R12345", 4).unwrap_err(),
            TokenizerError::LiteralValueTooLarge(Box::new(LiteralValueTooLarge {
                value_string: String::from("12345"),
                line: 1,
                col: 1
            }))
        );
    }

    #[test]
    fn test_memory_reference_too_large() {
        assert_eq!(
            Tokenizer::tokenize("256", 4).unwrap_err(),
            TokenizerError::LiteralValueTooLarge(Box::new(LiteralValueTooLarge {
                value_string: String::from("256"),
                line: 1,
                col: 1
            }))
        );
    }

    #[test]
    fn test_literal_value_too_large() {
        assert_eq!(
            Tokenizer::tokenize("#256", 4).unwrap_err(),
            TokenizerError::LiteralValueTooLarge(Box::new(LiteralValueTooLarge {
                value_string: String::from("256"),
                line: 1,
                col: 1
            }))
        );
    }

    #[test]
    fn test_invalid_characters() {
        assert_eq!(
            Tokenizer::tokenize("NOP; label: ??", 4).unwrap_err(),
            TokenizerError::UnexpectedCharacter(Box::new(UnexpectedCharacter {
                char: '?',
                line: 1,
                col: 13
            }))
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_line_col_calculations() {
        let input = "MOV R5 #3 #2
;23#54 ,\t,
R11 /* bruh */ LDR ;\t// hello!
           /* this is a 
           multiline
           comment
  */ MOV;
#23 98 R10 MOV ; ,
;\tbruh: ; bruh_two: bruh_three
";
        assert_eq!(
            extract_token_line_column_numbers(Tokenizer::tokenize(input, 4).unwrap().tokens),
            vec![
                (1,1), (1,5), (1,8), (1,11), (1,13),
                (2,1), (2,2), (2,4), (2,8), (2,13), (2,14),
                (3,1), (3,16), (3,20), (3,34),
                (7, 6), (7,9), (7, 10),
                (8, 1), (8,5), (8,8), (8,12), (8,16), (8,18), (8,19),
                (9, 1), (9,6), (9,12), (9,14), (9,24), (9,34),
            ]
        );
    }
}
