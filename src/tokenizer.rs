use strum::EnumIter;

use crate::interpreter::REGISTER_COUNT;
use std::{
    collections::HashMap,
    iter::Peekable,
    str::{Chars, FromStr},
};

#[derive(Debug)]
pub enum TokenizerError {
    ProgramTooLarge {
        line: usize,
        col: usize,
    },
    LiteralValueTooLarge {
        value_string: String,
        line: usize,
        col: usize,
    },
    MissingNumberAfterRegisterDenoter {
        line: usize,
        col: usize,
    },
    MissingNumberAfterLiteralDenoter {
        line: usize,
        col: usize,
    },
    InvalidRegisterNumber {
        value: u8,
        line: usize,
        col: usize,
    },
    InvalidLabelDefinitionLocation {
        label_name: String,
        line: usize,
        col: usize,
    },
    DuplicateLabelDefinition {
        label_name: String,
        line: usize,
        col: usize,
    },
    UnterminatedBlockComment {
        line: usize,
        col: usize,
    },
    InvalidCommentDenoter {
        line: usize,
        col: usize,
    },
    UnexpectedCharacter {
        char: char,
        line: usize,
        col: usize,
    },
}

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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum AssemblyOpcode {
    NOP,
    LDR,
    STR,
    ADD,
    SUB,
    MOV,
    CMP,
    B,
    BEQ,
    BNE,
    BGT,
    BLT,
    AND,
    ORR,
    EOR,
    MVN,
    LSL,
    LSR,
    PRINT,
    INPUT,
    HALT,
}

impl AssemblyOpcode {
    pub fn got_operand_formats(&self) -> Vec<(BinaryOpcode, Vec<OperandType>)> {
        match self {
            AssemblyOpcode::NOP => vec![(BinaryOpcode::NOP, vec![])],
            AssemblyOpcode::LDR => vec![(
                BinaryOpcode::LDR,
                vec![OperandType::Register, OperandType::MemoryRef],
            )],
            AssemblyOpcode::STR => vec![(
                BinaryOpcode::STR,
                vec![OperandType::Register, OperandType::MemoryRef],
            )],
            AssemblyOpcode::ADD => vec![
                (
                    BinaryOpcode::ADD_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    BinaryOpcode::ADD_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            AssemblyOpcode::SUB => vec![
                (
                    BinaryOpcode::SUB_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    BinaryOpcode::SUB_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            AssemblyOpcode::MOV => vec![
                (
                    BinaryOpcode::MOV_LITERAL,
                    vec![OperandType::Register, OperandType::Literal],
                ),
                (
                    BinaryOpcode::MOV_REGISTER,
                    vec![OperandType::Register, OperandType::Register],
                ),
            ],
            AssemblyOpcode::CMP => vec![
                (
                    BinaryOpcode::CMP_LITERAL,
                    vec![OperandType::Register, OperandType::Literal],
                ),
                (
                    BinaryOpcode::CMP_REGISTER,
                    vec![OperandType::Register, OperandType::Register],
                ),
            ],
            AssemblyOpcode::B => vec![(BinaryOpcode::B, vec![OperandType::Label])],
            AssemblyOpcode::BEQ => vec![(BinaryOpcode::BEQ, vec![OperandType::Label])],
            AssemblyOpcode::BNE => vec![(BinaryOpcode::BNE, vec![OperandType::Label])],
            AssemblyOpcode::BGT => vec![(BinaryOpcode::BGT, vec![OperandType::Label])],
            AssemblyOpcode::BLT => vec![(BinaryOpcode::BLT, vec![OperandType::Label])],
            AssemblyOpcode::AND => vec![
                (
                    BinaryOpcode::AND_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    BinaryOpcode::AND_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            AssemblyOpcode::ORR => vec![
                (
                    BinaryOpcode::ORR_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    BinaryOpcode::ORR_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            AssemblyOpcode::EOR => vec![
                (
                    BinaryOpcode::EOR_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    BinaryOpcode::EOR_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            AssemblyOpcode::MVN => vec![
                (
                    BinaryOpcode::MVN_LITERAL,
                    vec![OperandType::Register, OperandType::Literal],
                ),
                (
                    BinaryOpcode::MVN_REGISTER,
                    vec![OperandType::Register, OperandType::Register],
                ),
            ],
            AssemblyOpcode::LSL => vec![
                (
                    BinaryOpcode::LSL_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    BinaryOpcode::LSL_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            AssemblyOpcode::LSR => vec![
                (
                    BinaryOpcode::LSR_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    BinaryOpcode::LSR_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            AssemblyOpcode::HALT => vec![(BinaryOpcode::HALT, vec![])],
            AssemblyOpcode::PRINT => vec![
                (BinaryOpcode::PRINT_REGISTER, vec![OperandType::Register]),
                (BinaryOpcode::PRINT_MEMORY, vec![OperandType::MemoryRef]),
            ],
            AssemblyOpcode::INPUT => vec![
                (BinaryOpcode::INPUT_REGISTER, vec![OperandType::Register]),
                (BinaryOpcode::INPUT_MEMORY, vec![OperandType::MemoryRef]),
            ],
        }
    }
}

impl FromStr for AssemblyOpcode {
    type Err = ();
    fn from_str(input: &str) -> Result<AssemblyOpcode, Self::Err> {
        match input {
            "NOP" => Ok(AssemblyOpcode::NOP),
            "LDR" => Ok(AssemblyOpcode::LDR),
            "STR" => Ok(AssemblyOpcode::STR),
            "ADD" => Ok(AssemblyOpcode::ADD),
            "SUB" => Ok(AssemblyOpcode::SUB),
            "MOV" => Ok(AssemblyOpcode::MOV),
            "CMP" => Ok(AssemblyOpcode::CMP),
            "B" => Ok(AssemblyOpcode::B),
            "BEQ" => Ok(AssemblyOpcode::BEQ),
            "BNE" => Ok(AssemblyOpcode::BNE),
            "BGT" => Ok(AssemblyOpcode::BGT),
            "BLT" => Ok(AssemblyOpcode::BLT),
            "AND" => Ok(AssemblyOpcode::AND),
            "ORR" => Ok(AssemblyOpcode::ORR),
            "EOR" => Ok(AssemblyOpcode::EOR),
            "MVN" => Ok(AssemblyOpcode::MVN),
            "LSL" => Ok(AssemblyOpcode::LSL),
            "LSR" => Ok(AssemblyOpcode::LSR),
            "HALT" => Ok(AssemblyOpcode::HALT),
            "PRINT" => Ok(AssemblyOpcode::PRINT),
            "INPUT" => Ok(AssemblyOpcode::INPUT),
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOpcode {
    NOP,
    LDR,
    STR,
    ADD_REGISTER,
    ADD_LITERAL,
    SUB_REGISTER,
    SUB_LITERAL,
    MOV_REGISTER,
    MOV_LITERAL,
    CMP_REGISTER,
    CMP_LITERAL,
    B,
    BEQ,
    BNE,
    BGT,
    BLT,
    AND_REGISTER,
    AND_LITERAL,
    ORR_REGISTER,
    ORR_LITERAL,
    EOR_REGISTER,
    EOR_LITERAL,
    MVN_REGISTER,
    MVN_LITERAL,
    LSL_REGISTER,
    LSL_LITERAL,
    LSR_REGISTER,
    LSR_LITERAL,
    PRINT_REGISTER,
    PRINT_MEMORY,
    INPUT_REGISTER,
    INPUT_MEMORY,
    HALT,
}

impl TryFrom<u8> for BinaryOpcode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == BinaryOpcode::NOP as u8 => Ok(BinaryOpcode::NOP),
            x if x == BinaryOpcode::LDR as u8 => Ok(BinaryOpcode::LDR),
            x if x == BinaryOpcode::STR as u8 => Ok(BinaryOpcode::STR),
            x if x == BinaryOpcode::ADD_REGISTER as u8 => Ok(BinaryOpcode::ADD_REGISTER),
            x if x == BinaryOpcode::ADD_LITERAL as u8 => Ok(BinaryOpcode::ADD_LITERAL),
            x if x == BinaryOpcode::SUB_REGISTER as u8 => Ok(BinaryOpcode::SUB_REGISTER),
            x if x == BinaryOpcode::SUB_LITERAL as u8 => Ok(BinaryOpcode::SUB_LITERAL),
            x if x == BinaryOpcode::MOV_REGISTER as u8 => Ok(BinaryOpcode::MOV_REGISTER),
            x if x == BinaryOpcode::MOV_LITERAL as u8 => Ok(BinaryOpcode::MOV_LITERAL),
            x if x == BinaryOpcode::CMP_REGISTER as u8 => Ok(BinaryOpcode::CMP_REGISTER),
            x if x == BinaryOpcode::CMP_LITERAL as u8 => Ok(BinaryOpcode::CMP_LITERAL),
            x if x == BinaryOpcode::B as u8 => Ok(BinaryOpcode::B),
            x if x == BinaryOpcode::BEQ as u8 => Ok(BinaryOpcode::BEQ),
            x if x == BinaryOpcode::BNE as u8 => Ok(BinaryOpcode::BNE),
            x if x == BinaryOpcode::BGT as u8 => Ok(BinaryOpcode::BGT),
            x if x == BinaryOpcode::BLT as u8 => Ok(BinaryOpcode::BLT),
            x if x == BinaryOpcode::AND_REGISTER as u8 => Ok(BinaryOpcode::AND_REGISTER),
            x if x == BinaryOpcode::AND_LITERAL as u8 => Ok(BinaryOpcode::AND_LITERAL),
            x if x == BinaryOpcode::ORR_REGISTER as u8 => Ok(BinaryOpcode::ORR_REGISTER),
            x if x == BinaryOpcode::ORR_LITERAL as u8 => Ok(BinaryOpcode::ORR_LITERAL),
            x if x == BinaryOpcode::EOR_REGISTER as u8 => Ok(BinaryOpcode::EOR_REGISTER),
            x if x == BinaryOpcode::EOR_LITERAL as u8 => Ok(BinaryOpcode::EOR_LITERAL),
            x if x == BinaryOpcode::MVN_REGISTER as u8 => Ok(BinaryOpcode::MVN_REGISTER),
            x if x == BinaryOpcode::MVN_LITERAL as u8 => Ok(BinaryOpcode::MVN_LITERAL),
            x if x == BinaryOpcode::LSL_REGISTER as u8 => Ok(BinaryOpcode::LSL_REGISTER),
            x if x == BinaryOpcode::LSL_LITERAL as u8 => Ok(BinaryOpcode::LSL_LITERAL),
            x if x == BinaryOpcode::LSR_REGISTER as u8 => Ok(BinaryOpcode::LSR_REGISTER),
            x if x == BinaryOpcode::LSR_LITERAL as u8 => Ok(BinaryOpcode::LSR_LITERAL),
            x if x == BinaryOpcode::PRINT_REGISTER as u8 => Ok(BinaryOpcode::PRINT_REGISTER),
            x if x == BinaryOpcode::PRINT_MEMORY as u8 => Ok(BinaryOpcode::PRINT_MEMORY),
            x if x == BinaryOpcode::INPUT_REGISTER as u8 => Ok(BinaryOpcode::INPUT_REGISTER),
            x if x == BinaryOpcode::INPUT_MEMORY as u8 => Ok(BinaryOpcode::INPUT_MEMORY),
            x if x == BinaryOpcode::HALT as u8 => Ok(BinaryOpcode::HALT),
            _ => Err(()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Operand(OperandType, u8),
    Opcode(AssemblyOpcode),
    Newline,
    Semicolon,
    Comma,
    EOF,
}

impl Token {
    pub fn get_token_debug_repr(&self) -> String {
        match &self.ty {
            TokenType::Newline => String::from("'newline'"),
            TokenType::EOF => String::from("'end of file'"),
            _ => format!("'{}'", &self.lexeme),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
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
                '\n' => self.tokenize_single_char_token(TokenType::Newline, ch),
                '/' => self.comment()?,
                ';' => self.tokenize_single_char_token(TokenType::Semicolon, ch),
                ',' => self.tokenize_single_char_token(TokenType::Comma, ch),
                '0'..='9' => self.tokenize_memory_ref()?,
                '#' => self.tokenize_literal()?,
                'a'..='z' | 'A'..='Z' | '_' => self.tokenizer_identifier()?,
                _ => {
                    return Err(TokenizerError::UnexpectedCharacter {
                        char: ch,
                        line: self.line,
                        col: self.col,
                    })
                }
            }
        }

        // Append an EOF token
        self.tokens.push(Token {
            ty: TokenType::EOF,
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
            return Err(TokenizerError::ProgramTooLarge {
                line: most_recent_token.line,
                col: most_recent_token.col,
            });
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
                return Some(Err(TokenizerError::LiteralValueTooLarge {
                    value_string: digit_string,
                    line: line,
                    col: col,
                }))
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
    fn tokenize_single_char_token(&mut self, token_type: TokenType, char: char) {
        let line = self.line;
        let col = self.col;
        self.next();
        self.tokens.push(Token {
            ty: token_type,
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
            ty: TokenType::Operand(OperandType::MemoryRef, num),
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
                return Err(TokenizerError::MissingNumberAfterLiteralDenoter {
                    line: line,
                    col: col,
                })
            }
        };
        self.tokens.push(Token {
            ty: TokenType::Operand(OperandType::Literal, num),
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
                    return Err(TokenizerError::MissingNumberAfterRegisterDenoter {
                        line: line,
                        col: col,
                    })
                }
            };
            if register_num >= REGISTER_COUNT {
                return Err(TokenizerError::InvalidRegisterNumber {
                    value: register_num,
                    line: line,
                    col: col,
                });
            }
            self.tokens.push(Token {
                ty: TokenType::Operand(OperandType::Register, register_num),
                lexeme: format!("R{}", &register_num_lexeme),
                line: line,
                col: col,
            });
            self.inc_program_byte_count()?;
            return Ok(());
        }

        // Is it an opcode
        if let Ok(opcode) = AssemblyOpcode::from_str(&identifier) {
            self.tokens.push(Token {
                ty: TokenType::Opcode(opcode),
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
                        if token.ty != TokenType::Semicolon && token.ty != TokenType::Newline =>
                    {
                        dbg!(token.ty);
                        return Err(TokenizerError::InvalidLabelDefinitionLocation {
                            label_name: identifier,
                            line: line,
                            col: col,
                        });
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
                        return Err(TokenizerError::DuplicateLabelDefinition {
                            label_name: identifier,
                            line: line,
                            col: col,
                        })
                    }
                    None => {}
                };
                return Ok(());
            }
            _ => {}
        }

        // It must be a label operand
        self.tokens.push(Token {
            ty: TokenType::Operand(OperandType::Label, 0),
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
                            return Err(TokenizerError::UnterminatedBlockComment {
                                line: line,
                                col: col,
                            });
                        }
                    }
                } else {
                    return Err(TokenizerError::UnterminatedBlockComment {
                        line: line,
                        col: col,
                    });
                }
            },
            _ => {
                return Err(TokenizerError::InvalidCommentDenoter {
                    line: line,
                    col: col,
                })
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Given a Vec<Token> return a Vec<TokenType> (the token type for each token)
    fn extract_token_types(tokens: Vec<Token>) -> Vec<TokenType> {
        tokens.into_iter().map(|token| token.ty).collect()
    }

    /// Given a Vec<Token> return vec of tuples of line and column numbers for each token
    fn extract_token_line_column_numbers(tokens: Vec<Token>) -> Vec<(usize, usize)> {
        tokens
            .into_iter()
            .map(|token| (token.line, token.col))
            .collect()
    }

    /// Test tokenizer produce correct token type sequence for given input string
    fn test_token_type_sequence(input: &str, expected: &[TokenType]) {
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
    fn test_tokens_isolated() {
        for (input, expected_output) in [
            ("\n", TokenType::Newline),
            (";", TokenType::Semicolon),
            (",", TokenType::Comma),
            ("123", TokenType::Operand(OperandType::MemoryRef, 123)),
            ("#12", TokenType::Operand(OperandType::Literal, 12)),
            ("R3", TokenType::Operand(OperandType::Register, 3)),
            ("label_operand", TokenType::Operand(OperandType::Label, 0)),
            ("NOP", TokenType::Opcode(AssemblyOpcode::NOP)),
            ("LDR", TokenType::Opcode(AssemblyOpcode::LDR)),
            ("STR", TokenType::Opcode(AssemblyOpcode::STR)),
            ("ADD", TokenType::Opcode(AssemblyOpcode::ADD)),
            ("SUB", TokenType::Opcode(AssemblyOpcode::SUB)),
            ("MOV", TokenType::Opcode(AssemblyOpcode::MOV)),
            ("CMP", TokenType::Opcode(AssemblyOpcode::CMP)),
            ("B", TokenType::Opcode(AssemblyOpcode::B)),
            ("BEQ", TokenType::Opcode(AssemblyOpcode::BEQ)),
            ("BNE", TokenType::Opcode(AssemblyOpcode::BNE)),
            ("BGT", TokenType::Opcode(AssemblyOpcode::BGT)),
            ("BLT", TokenType::Opcode(AssemblyOpcode::BLT)),
            ("AND", TokenType::Opcode(AssemblyOpcode::AND)),
            ("ORR", TokenType::Opcode(AssemblyOpcode::ORR)),
            ("EOR", TokenType::Opcode(AssemblyOpcode::EOR)),
            ("MVN", TokenType::Opcode(AssemblyOpcode::MVN)),
            ("LSL", TokenType::Opcode(AssemblyOpcode::LSL)),
            ("LSR", TokenType::Opcode(AssemblyOpcode::LSR)),
            ("PRINT", TokenType::Opcode(AssemblyOpcode::PRINT)),
            ("INPUT", TokenType::Opcode(AssemblyOpcode::INPUT)),
            ("HALT", TokenType::Opcode(AssemblyOpcode::HALT)),
        ] {
            test_token_type_sequence(input, &[expected_output, TokenType::EOF]);
        }
    }

    #[test]
    fn test_comment_line_single() {
        test_token_type_sequence(
            "NOP // Comment",
            &[TokenType::Opcode(AssemblyOpcode::NOP), TokenType::EOF],
        );
    }

    #[test]
    fn test_comment_line_multiline() {
        test_token_type_sequence(
            "NOP /* Multiline \n Comment */",
            &[TokenType::Opcode(AssemblyOpcode::NOP), TokenType::EOF],
        );
    }

    #[test]
    fn test_comment_block_multiline() {
        test_token_type_sequence(
            "NOP /* Multiline \n Comment \n */ NOP",
            &[
                TokenType::Opcode(AssemblyOpcode::NOP),
                TokenType::Opcode(AssemblyOpcode::NOP),
                TokenType::EOF,
            ],
        );
    }

    #[test]
    fn test_empty_program() {
        test_token_type_sequence("", &[TokenType::EOF]);
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
        assert_tokenizer_error!(
            "MOV R5 #23 /* this is an unterminated block comment",
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
;23#54 ,    ,
R11 /* bruh */ LDR ;    // hello!
           /* this is a 
           multiline
           comment
  */ MOV;
#23 98 R10 MOV ; ,
;    bruh: ; bruh_two: bruh_three
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
