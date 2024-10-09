use crate::{interpreter::REGISTER_COUNT, tokenizer::InvalidLabelDefinitionLocation};
use std::{
    collections::HashMap,
    iter::Peekable,
    str::{Chars, FromStr},
};

use super::{
    DuplicateLabelDefinition, InvalidCommentDenoter, InvalidRegisterNumber, LiteralValueTooLarge,
    MissingNumberAfterLiteralDenoter, MissingNumberAfterRegisterDenoter, ProgramTooLarge, Token,
    TokenKind, TokenizerError, UnexpectedCharacter, UnterminatedBlockComment,
};
use crate::source_opcode::SourceOpcode;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OperandType {
    Literal,
    Register,
    MemoryRef,
    Label,
}

#[derive(Debug, PartialEq)]
pub struct LabelDefinition {
    pub byte: u8,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    pub tokens: Vec<Token>,
    pub labels: HashMap<String, LabelDefinition>,
    iter: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
    pub program_bytes: usize,
    tabsize: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(input: &'a str, tabsize: u8) -> Result<Self, TokenizerError> {
        let mut tokenizer = Tokenizer {
            tokens: Vec::new(),
            labels: HashMap::new(),
            iter: input.chars().peekable(),
            line: 1,
            col: 1,
            program_bytes: 0,
            tabsize: tabsize as usize,
        };
        tokenizer.interal_tokenize()?;
        Ok(tokenizer)
    }

    /// tokenize takes in an input source code string and returns a Vec of tokens and
    /// a HashMap of label definitions to their corresponding byte position and the
    /// amount of bytes the resulting progam will use. There are multiple types of token:
    /// * Newlines / Semicolons (both are used a line delimeters)
    /// * Commas
    /// * Memory references (An unsigned 8 bit number e.g 12)
    /// * Literals (A '#' follwed by an unsigned 8 bit number e.g. #12)
    /// * Registers (An 'R' followed by a number in the range 0 to REGISTER_COUNT-1)
    /// * Opcodes (A string of chars that make any of our opcodes)
    /// * Label operands (A string of chars)
    /// * Label definitions (A string of chars followed by a colon)
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
                continue;
            }
            // Tokenizer based on characters
            match ch {
                '\n' => self.tokenize_single_char_token(TokenKind::Newline, ch),
                '/' => self.comment()?,
                ';' => self.tokenize_single_char_token(TokenKind::Semicolon, ch),
                ',' => self.tokenize_single_char_token(TokenKind::Comma, ch),
                '0'..='9' => self.tokenize_memory_ref()?,
                '#' => self.tokenize_literal()?,
                'a'..='z' | 'A'..='Z' | '_' => self.tokenizer_identifier()?,
                _ => {
                    return Err(TokenizerError::UnexpectedCharacter(Box::new(
                        UnexpectedCharacter {
                            char: ch,
                            line: self.line,
                            col: self.col,
                        },
                    )))
                }
            }
        }

        // Append an EOF token
        self.tokens.push(Token {
            kind: TokenKind::EOF,
            lexeme: String::from("EOF"),
            line: self.line,
            col: self.col,
        });
        Ok(())
    }

    /// Consume character
    fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.iter.next() {
            match ch {
                '\n' => {
                    self.line += 1;
                    self.col = 1;
                }
                '\t' => self.col += self.tabsize,
                _ => self.col += 1,
            }
            return Some(ch);
        }
        return None;
    }

    /// Increase program byte count, throwing error if it gets too large
    fn inc_program_byte_count(&mut self) -> Result<(), TokenizerError> {
        if self.program_bytes == 256 {
            let most_recent_token = self
                .tokens
                .last()
                .expect("This should never happen, unless this was called BEFORE adding a token");
            return Err(TokenizerError::ProgramTooLarge(Box::new(ProgramTooLarge {
                line: most_recent_token.line,
                col: most_recent_token.col,
            })));
        }
        self.program_bytes += 1;
        Ok(())
    }

    /// Consume series of digits and try to convert into a u8
    fn consume_u8(&mut self) -> Option<Result<(u8, String), TokenizerError>> {
        let line = self.line;
        let col = self.col;
        // Check first character. If there is no character or no digit character there we return None
        if self.iter.peek().map_or(true, |ch| !ch.is_digit(10)) {
            return None;
        }
        // Collect all digits into string until we find a non-digit
        let mut digit_string = String::new();
        while let Some(&ch) = self.iter.peek() {
            if ch.is_digit(10) {
                digit_string.push(ch);
            } else {
                break;
            }
            self.next();
        }
        // Try and convert into a u8, return error if it fails
        match digit_string.parse() {
            Ok(val) => return Some(Ok((val, digit_string))),
            Err(_) => {
                return Some(Err(TokenizerError::LiteralValueTooLarge(Box::new(
                    LiteralValueTooLarge {
                        value_string: digit_string,
                        line: line,
                        col: col,
                    },
                ))))
            }
        };
    }

    /// Consume a series of lowercase characters, uppercase characters and underscores
    fn consume_identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(&ch) = self.iter.peek() {
            match ch {
                'a'..='z' | 'A'..='Z' | '_' => result.push(ch),
                _ => break,
            }
            self.next();
        }
        result
    }

    /// Tokenize a token consisting of a single char (newlines, semicolons, commas)
    fn tokenize_single_char_token(&mut self, token_type: TokenKind, char: char) {
        let line = self.line;
        let col = self.col;
        self.next();
        self.tokens.push(Token {
            kind: token_type,
            lexeme: String::from(char),
            line: line,
            col: col,
        });
    }

    /// Tokenize a memory reference
    fn tokenize_memory_ref(&mut self) -> Result<(), TokenizerError> {
        let line = self.line;
        let col = self.col;
        let (num, lexeme) = self.consume_u8().unwrap()?;
        self.tokens.push(Token {
            kind: TokenKind::Operand(OperandType::MemoryRef, num),
            lexeme: lexeme,
            line: line,
            col: col,
        });
        self.inc_program_byte_count()?;
        Ok(())
    }

    /// Tokenize a literal value
    fn tokenize_literal(&mut self) -> Result<(), TokenizerError> {
        let line = self.line;
        let col = self.col;
        // Move past hashtag
        self.next();
        // Read number and return error if not found
        let (num, num_lexeme) = match self.consume_u8() {
            Some(num) => num?,
            None => {
                return Err(TokenizerError::MissingNumberAfterLiteralDenoter(Box::new(
                    MissingNumberAfterLiteralDenoter {
                        line: line,
                        col: col,
                    },
                )))
            }
        };
        self.tokens.push(Token {
            kind: TokenKind::Operand(OperandType::Literal, num),
            lexeme: format!("#{}", num_lexeme),
            line: line,
            col: col,
        });
        self.inc_program_byte_count()?;
        Ok(())
    }

    /// Tokenize an identifier, either:
    /// * A register (R0-R12)
    /// * A label operand
    /// * An opcode
    /// * A label definition
    fn tokenizer_identifier(&mut self) -> Result<(), TokenizerError> {
        let line = self.line;
        let col = self.col;
        let identifier = self.consume_identifier();
        // Is it a register?
        if identifier == "R" {
            let (register_num, register_num_lexeme) = match self.consume_u8() {
                Some(num) => num?,
                None => {
                    return Err(TokenizerError::MissingNumberAfterRegisterDenoter(Box::new(
                        MissingNumberAfterRegisterDenoter {
                            line: line,
                            col: col,
                        },
                    )))
                }
            };
            if register_num >= REGISTER_COUNT {
                return Err(TokenizerError::InvalidRegisterNumber(Box::new(
                    InvalidRegisterNumber {
                        value: register_num,
                        line: line,
                        col: col,
                    },
                )));
            }
            self.tokens.push(Token {
                kind: TokenKind::Operand(OperandType::Register, register_num),
                lexeme: format!("R{}", &register_num_lexeme),
                line: line,
                col: col,
            });
            self.inc_program_byte_count()?;
            return Ok(());
        }

        // Is it an opcode
        if let Ok(opcode) = SourceOpcode::from_str(&identifier) {
            self.tokens.push(Token {
                kind: TokenKind::Opcode(opcode),
                lexeme: identifier,
                line: line,
                col: col,
            });
            self.inc_program_byte_count()?;
            return Ok(());
        }

        // Is it a label definition?
        match self.iter.peek() {
            Some(&ch) if ch == ':' => {
                self.next();
                // label definitions can only appear after newlines and semicolons
                match self.tokens.last() {
                    Some(token)
                        if token.kind != TokenKind::Semicolon
                            && token.kind != TokenKind::Newline =>
                    {
                        dbg!(token.kind);
                        return Err(TokenizerError::InvalidLabelDefinitionLocation(Box::new(
                            InvalidLabelDefinitionLocation {
                                label_name: identifier,
                                line: line,
                                col: col,
                            },
                        )));
                    }
                    _ => {}
                }
                // Insert label, returning error if it already exists
                match self.labels.insert(
                    identifier.clone(),
                    LabelDefinition {
                        byte: self.program_bytes as u8,
                        line: line,
                        col: col,
                    },
                ) {
                    Some(_) => {
                        return Err(TokenizerError::DuplicateLabelDefinition(Box::new(
                            DuplicateLabelDefinition {
                                label_name: identifier,
                                line: line,
                                col: col,
                            },
                        )))
                    }
                    None => {}
                };
                return Ok(());
            }
            _ => {}
        }

        // It must be a label operand
        self.tokens.push(Token {
            kind: TokenKind::Operand(OperandType::Label, 0),
            lexeme: identifier,
            line: line,
            col: col,
        });
        self.inc_program_byte_count()?;
        Ok(())
    }

    /// Skip past sections of comments
    fn comment(&mut self) -> Result<(), TokenizerError> {
        let line = self.line;
        let col = self.col;
        self.next();
        match self.iter.peek() {
            // Single line comment - consume until we hit a newline
            Some('/') => {
                self.next();
                while let Some(&ch) = self.iter.peek() {
                    if ch == '\n' {
                        break;
                    } else {
                        self.next();
                    }
                }
            }
            Some('*') => loop {
                if let Some(ch) = self.next() {
                    if ch == '*' {
                        if let Some(&next) = self.iter.peek() {
                            if next == '/' {
                                self.next();
                                break;
                            }
                        } else {
                            return Err(TokenizerError::UnterminatedBlockComment(Box::new(
                                UnterminatedBlockComment {
                                    line: line,
                                    col: col,
                                },
                            )));
                        }
                    }
                } else {
                    return Err(TokenizerError::UnterminatedBlockComment(Box::new(
                        UnterminatedBlockComment {
                            line: line,
                            col: col,
                        },
                    )));
                }
            },
            _ => {
                return Err(TokenizerError::InvalidCommentDenoter(Box::new(
                    InvalidCommentDenoter {
                        line: line,
                        col: col,
                    },
                )))
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime_opcode::RuntimeOpcode;

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

    // Assert tokenizer produces specific error type
    macro_rules! assert_tokenizer_error {
        ($input:expr, $expected_error:pat) => {
            assert!(matches!(
                Tokenizer::tokenize($input, 4),
                Err($expected_error)
            ));
        };
    }

    #[test]
    fn test_get_token_debug_repr() {
        for (input, expected) in [
            (
                Token {
                    kind: TokenKind::Newline,
                    lexeme: String::from("\n"),
                    line: 0,
                    col: 0,
                },
                "'newline'",
            ),
            (
                Token {
                    kind: TokenKind::EOF,
                    lexeme: String::from("EOF"),
                    line: 0,
                    col: 0,
                },
                "'end of file'",
            ),
            (
                Token {
                    kind: TokenKind::Comma,
                    lexeme: String::from(","),
                    line: 0,
                    col: 0,
                },
                "','",
            ),
        ] {
            assert_eq!(input.get_token_debug_repr(), expected);
        }
    }

    #[test]
    fn test_assembly_opcode_from_str() {
        for (input, expected) in [
            // Ensure all
            ("NOP", Ok(SourceOpcode::NOP)),
            ("LDR", Ok(SourceOpcode::LDR)),
            ("STR", Ok(SourceOpcode::STR)),
            ("ADD", Ok(SourceOpcode::ADD)),
            ("SUB", Ok(SourceOpcode::SUB)),
            ("MOV", Ok(SourceOpcode::MOV)),
            ("CMP", Ok(SourceOpcode::CMP)),
            ("B", Ok(SourceOpcode::B)),
            ("BEQ", Ok(SourceOpcode::BEQ)),
            ("BNE", Ok(SourceOpcode::BNE)),
            ("BGT", Ok(SourceOpcode::BGT)),
            ("BLT", Ok(SourceOpcode::BLT)),
            ("AND", Ok(SourceOpcode::AND)),
            ("ORR", Ok(SourceOpcode::ORR)),
            ("EOR", Ok(SourceOpcode::EOR)),
            ("MVN", Ok(SourceOpcode::MVN)),
            ("LSL", Ok(SourceOpcode::LSL)),
            ("LSR", Ok(SourceOpcode::LSR)),
            ("HALT", Ok(SourceOpcode::HALT)),
            ("PRINT", Ok(SourceOpcode::PRINT)),
            ("INPUT", Ok(SourceOpcode::INPUT)),
            // lowercase commands shouldn't work
            ("nop", Err(())),
            ("input", Err(())),
            // whitespace should be important
            ("A N D", Err(())),
            ("AND  ", Err(())),
            ("  AND", Err(())),
            ("  AND  ", Err(())),
            // random words shouldn't work
            ("foo", Err(())),
            ("bar", Err(())),
        ] {
            assert_eq!(SourceOpcode::from_str(input), expected);
        }
    }

    #[test]
    fn test_binary_opcode_from_u8() {
        for (input, expected) in [
            (0, Ok(RuntimeOpcode::NOP)),
            (1, Ok(RuntimeOpcode::LDR)),
            (2, Ok(RuntimeOpcode::STR)),
            (3, Ok(RuntimeOpcode::ADD_REGISTER)),
            (4, Ok(RuntimeOpcode::ADD_LITERAL)),
            (5, Ok(RuntimeOpcode::SUB_REGISTER)),
            (6, Ok(RuntimeOpcode::SUB_LITERAL)),
            (7, Ok(RuntimeOpcode::MOV_REGISTER)),
            (8, Ok(RuntimeOpcode::MOV_LITERAL)),
            (9, Ok(RuntimeOpcode::CMP_REGISTER)),
            (10, Ok(RuntimeOpcode::CMP_LITERAL)),
            (11, Ok(RuntimeOpcode::B)),
            (12, Ok(RuntimeOpcode::BEQ)),
            (13, Ok(RuntimeOpcode::BNE)),
            (14, Ok(RuntimeOpcode::BGT)),
            (15, Ok(RuntimeOpcode::BLT)),
            (16, Ok(RuntimeOpcode::AND_REGISTER)),
            (17, Ok(RuntimeOpcode::AND_LITERAL)),
            (18, Ok(RuntimeOpcode::ORR_REGISTER)),
            (19, Ok(RuntimeOpcode::ORR_LITERAL)),
            (20, Ok(RuntimeOpcode::EOR_REGISTER)),
            (21, Ok(RuntimeOpcode::EOR_LITERAL)),
            (22, Ok(RuntimeOpcode::MVN_REGISTER)),
            (23, Ok(RuntimeOpcode::MVN_LITERAL)),
            (24, Ok(RuntimeOpcode::LSL_REGISTER)),
            (25, Ok(RuntimeOpcode::LSL_LITERAL)),
            (26, Ok(RuntimeOpcode::LSR_REGISTER)),
            (27, Ok(RuntimeOpcode::LSR_LITERAL)),
            (28, Ok(RuntimeOpcode::PRINT_REGISTER)),
            (29, Ok(RuntimeOpcode::PRINT_MEMORY)),
            (30, Ok(RuntimeOpcode::INPUT_REGISTER)),
            (31, Ok(RuntimeOpcode::INPUT_MEMORY)),
            (32, Ok(RuntimeOpcode::HALT)),
        ] {
            assert_eq!(RuntimeOpcode::try_from(input), expected);
        }

        for input in 33..=255 {
            assert_eq!(RuntimeOpcode::try_from(input), Err(()));
        }
    }

    #[test]
    fn test_tokens_isolated() {
        for (input, expected_output) in [
            ("\n", TokenKind::Newline),
            (";", TokenKind::Semicolon),
            (",", TokenKind::Comma),
            ("123", TokenKind::Operand(OperandType::MemoryRef, 123)),
            ("#12", TokenKind::Operand(OperandType::Literal, 12)),
            ("R3", TokenKind::Operand(OperandType::Register, 3)),
            ("label_operand", TokenKind::Operand(OperandType::Label, 0)),
            ("NOP", TokenKind::Opcode(SourceOpcode::NOP)),
            ("LDR", TokenKind::Opcode(SourceOpcode::LDR)),
            ("STR", TokenKind::Opcode(SourceOpcode::STR)),
            ("ADD", TokenKind::Opcode(SourceOpcode::ADD)),
            ("SUB", TokenKind::Opcode(SourceOpcode::SUB)),
            ("MOV", TokenKind::Opcode(SourceOpcode::MOV)),
            ("CMP", TokenKind::Opcode(SourceOpcode::CMP)),
            ("B", TokenKind::Opcode(SourceOpcode::B)),
            ("BEQ", TokenKind::Opcode(SourceOpcode::BEQ)),
            ("BNE", TokenKind::Opcode(SourceOpcode::BNE)),
            ("BGT", TokenKind::Opcode(SourceOpcode::BGT)),
            ("BLT", TokenKind::Opcode(SourceOpcode::BLT)),
            ("AND", TokenKind::Opcode(SourceOpcode::AND)),
            ("ORR", TokenKind::Opcode(SourceOpcode::ORR)),
            ("EOR", TokenKind::Opcode(SourceOpcode::EOR)),
            ("MVN", TokenKind::Opcode(SourceOpcode::MVN)),
            ("LSL", TokenKind::Opcode(SourceOpcode::LSL)),
            ("LSR", TokenKind::Opcode(SourceOpcode::LSR)),
            ("PRINT", TokenKind::Opcode(SourceOpcode::PRINT)),
            ("INPUT", TokenKind::Opcode(SourceOpcode::INPUT)),
            ("HALT", TokenKind::Opcode(SourceOpcode::HALT)),
        ] {
            test_token_type_sequence(input, &[expected_output, TokenKind::EOF]);
        }
    }

    #[test]
    fn test_comment_line_single() {
        test_token_type_sequence(
            "NOP // Comment",
            &[TokenKind::Opcode(SourceOpcode::NOP), TokenKind::EOF],
        );
    }

    #[test]
    fn test_comment_line_multiline() {
        test_token_type_sequence(
            "NOP /* Multiline \n Comment */",
            &[TokenKind::Opcode(SourceOpcode::NOP), TokenKind::EOF],
        );
    }

    #[test]
    fn test_comment_block_multiline() {
        test_token_type_sequence(
            "NOP /* Multiline \n Comment \n */ NOP",
            &[
                TokenKind::Opcode(SourceOpcode::NOP),
                TokenKind::Opcode(SourceOpcode::NOP),
                TokenKind::EOF,
            ],
        );
    }

    #[test]
    fn test_empty_program() {
        test_token_type_sequence("", &[TokenKind::EOF]);
    }

    #[test]
    fn test_missing_register_number() {
        assert_tokenizer_error!(
            "MOV R #23",
            TokenizerError::MissingNumberAfterRegisterDenoter { .. }
        );
    }

    #[test]
    fn test_missing_literal_number() {
        assert_tokenizer_error!(
            "MOV R5 #",
            TokenizerError::MissingNumberAfterLiteralDenoter { .. }
        );
    }

    #[test]
    fn test_unterminated_block_comment() {
        // No ending delimeter (*/)
        assert_tokenizer_error!(
            "MOV R5 #23 /* this is an unterminated block comment",
            TokenizerError::UnterminatedBlockComment { .. }
        );
        // Half an ending delimeter
        assert_tokenizer_error!(
            "MOV R5 #23 /* this is an unterminated block comment*",
            TokenizerError::UnterminatedBlockComment { .. }
        );
    }

    #[test]
    fn test_invalid_comment_denoter() {
        assert_tokenizer_error!("/", TokenizerError::InvalidCommentDenoter { .. });
    }

    #[test]
    fn test_duplicate_label_definitions() {
        assert_tokenizer_error!(
            "label:
        NOP
        label:
        NOP",
            TokenizerError::DuplicateLabelDefinition { .. }
        );
    }

    #[test]
    fn test_invalid_register_number() {
        assert_tokenizer_error!("R13", TokenizerError::InvalidRegisterNumber { .. });
    }

    #[test]
    fn test_too_large_program() {
        assert_tokenizer_error!(&"NOP;".repeat(257), TokenizerError::ProgramTooLarge { .. });
    }

    #[test]
    fn test_label_definition_not_after_line_delimeters() {
        assert_tokenizer_error!(
            "NOP label:",
            TokenizerError::InvalidLabelDefinitionLocation { .. }
        );
    }

    #[test]
    fn test_memory_reference_too_large() {
        assert_tokenizer_error!("256", TokenizerError::LiteralValueTooLarge { .. });
    }

    #[test]
    fn test_literal_value_too_large() {
        assert_tokenizer_error!("#256", TokenizerError::LiteralValueTooLarge { .. });
    }

    #[test]
    fn test_invalid_characters() {
        assert_tokenizer_error!("NOP; label: ??", TokenizerError::UnexpectedCharacter { .. });
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
                (9, 1), (9,12), (9,24), (9,34),
                (10,1)  
            ]
        )
    }
}
