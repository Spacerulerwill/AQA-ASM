use crate::interpreter::REGISTER_COUNT;
use std::{
    collections::HashMap,
    iter::Peekable,
    str::{Chars, FromStr}
};
use unicode_width::UnicodeWidthChar;

/// Calculate the visual width of a string
fn string_col_width(str: &str) -> usize {
    let mut width = 0;
    for char in str.chars() {
        width += UnicodeWidthChar::width(char).unwrap_or(0)
    }
    width
}

// Tokenization error information struct
#[derive(Debug)]
pub struct TokenizerError {
    pub message: String,
    pub line: usize,
    pub col: usize,
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
#[derive(Debug, Clone, Copy, PartialEq)]
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
                    vec![OperandType::Register, OperandType::Register, OperandType::Literal],
                ),
                (
                    BinaryOpcode::ADD_REGISTER,
                    vec![OperandType::Register, OperandType::Register, OperandType::Register],
                ),
            ],
            AssemblyOpcode::SUB => vec![
                (
                    BinaryOpcode::SUB_LITERAL,
                    vec![OperandType::Register, OperandType::Register, OperandType::Literal],
                ),
                (
                    BinaryOpcode::SUB_REGISTER,
                    vec![OperandType::Register, OperandType::Register, OperandType::Register],
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
                    vec![OperandType::Register, OperandType::Register, OperandType::Literal],
                ),
                (
                    BinaryOpcode::AND_REGISTER,
                    vec![OperandType::Register, OperandType::Register, OperandType::Register],
                ),
            ],
            AssemblyOpcode::ORR => vec![
                (
                    BinaryOpcode::ORR_LITERAL,
                    vec![OperandType::Register, OperandType::Register, OperandType::Literal],
                ),
                (
                    BinaryOpcode::ORR_REGISTER,
                    vec![OperandType::Register, OperandType::Register, OperandType::Register],
                ),
            ],
            AssemblyOpcode::EOR => vec![
                (
                    BinaryOpcode::EOR_LITERAL,
                    vec![OperandType::Register, OperandType::Register, OperandType::Literal],
                ),
                (
                    BinaryOpcode::EOR_REGISTER,
                    vec![OperandType::Register, OperandType::Register, OperandType::Register],
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
                    vec![OperandType::Register, OperandType::Register, OperandType::Literal],
                ),
                (
                    BinaryOpcode::LSL_REGISTER,
                    vec![OperandType::Register, OperandType::Register, OperandType::Register],
                ),
            ],
            AssemblyOpcode::LSR => vec![
                (
                    BinaryOpcode::LSR_LITERAL,
                    vec![OperandType::Register, OperandType::Register, OperandType::Literal],
                ),
                (
                    BinaryOpcode::LSR_REGISTER,
                    vec![OperandType::Register, OperandType::Register, OperandType::Register],
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
struct TokenizerState<'a> {
    tokens: Vec<Token>,
    labels: HashMap<String, LabelDefinition>,
    iter: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
    program_bytes: usize,
}

impl<'a> TokenizerState<'a> {
    fn new(source: &'a str) -> TokenizerState<'a> {
        TokenizerState {
            tokens: Vec::new(),
            labels: HashMap::new(),
            iter: source.chars().peekable(),
            line: 1,
            col: 1,
            program_bytes: 0,
        }
    }

    /// Consume character
    fn next(&mut self) {
        if let Some(ch) = self.iter.next() {
            match ch {
                '\n' => {
                    self.line += 1;
                    self.col = 1;
                }
                _ => {
                    self.col += UnicodeWidthChar::width(ch).unwrap_or(0);
                }
            }
        }
    }

    /// Consumer characters until we hit delimeter
    fn next_until(&mut self, delimeter: char) {
        while let Some(&ch) = self.iter.peek() {
            if ch == delimeter {
                break;
            } else {
                self.iter.next();
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String) {
        let col_width = string_col_width(&lexeme);
        self.tokens.push(Token {
            ty: token_type,
            lexeme: lexeme,
            line: self.line,
            col: self.col - col_width,
        })
    }

    fn inc_program_byte_count(&mut self) -> Result<(), TokenizerError> {
        if self.program_bytes == 256 {
            let most_recent_token = self
                .tokens
                .last()
                .expect("This should never happen, unless this was called BEFORE adding a token");
            return Err(TokenizerError {
                message: format!("Program is too large to load into memory, max size of 256 bytes"),
                line: most_recent_token.line,
                col: most_recent_token.col,
            });
        }
        self.program_bytes += 1;
        Ok(())
    }
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
/// have been loaded. Max 256.
pub fn tokenize(
    input: &str,
) -> Result<(Vec<Token>, HashMap<String, LabelDefinition>, usize), TokenizerError> {
    let mut state = TokenizerState::new(input);
    /// Consume series of digits and try to convert into a u8
    fn consume_u8(state: &mut TokenizerState) -> Option<Result<(u8, String), TokenizerError>> {
        // Check first character. If there is no digit character there we return None
        if let Some(first_ch) = state.iter.peek() {
            if !first_ch.is_digit(10) {
                return None;
            }
        } else {
            return None;
        }
        // Collect all digits into string until we find a non-digit
        let mut digit_string = String::new();
        while let Some(&ch) = state.iter.peek() {
            if ch.is_digit(10) {
                digit_string.push(ch);
            } else {
                break;
            }
            state.next();
        }
        // Try and convert into a u8, return error if it fails
        match digit_string.parse() {
            Ok(val) => return Some(Ok((val, digit_string))),
            Err(_) => {
                return Some(Err(TokenizerError {
                    message: format!(
                        "Integer value '{}' too large (must be in range 0-255 inclusive)",
                        digit_string
                    ),
                    line: state.line,
                    col: state.col - digit_string.len(),
                }))
            }
        };
    }

    /// Consume a series of lowercase characters, uppercase characters and underscores
    fn consume_identifier(state: &mut TokenizerState) -> String {
        let mut result = String::new();
        while let Some(&ch) = state.iter.peek() {
            match ch {
                'a'..='z' | 'A'..='Z' | '_' => result.push(ch),
                _ => break,
            }
            state.next();
        }
        result
    }

    /// Tokenize a token consisting of a single char
    fn tokenize_single_char_token(state: &mut TokenizerState, token_type: TokenType, char: char) {
        state.next();
        state.add_token(token_type, String::from(char));
    }

    /// Tokenize a memory reference (a basic integer literal)
    fn tokenize_memory_ref(state: &mut TokenizerState) -> Result<(), TokenizerError> {
        let (num, lexeme) = consume_u8(state).unwrap()?;
        state.add_token(TokenType::Operand(OperandType::MemoryRef, num), lexeme);
        state.inc_program_byte_count()?;
        Ok(())
    }

    /// Tokenize a literal value (#000)
    fn tokenize_literal(state: &mut TokenizerState) -> Result<(), TokenizerError> {
        // Move past hashtag
        state.next();
        // Read number and return error if not found
        let (num, num_lexeme) = match consume_u8(state) {
            Some(num) => num?,
            None => {
                return Err(TokenizerError {
                    message: String::from("Expected integer value after literal denoter '#'"),
                    line: state.line,
                    col: state.col - 1,
                })
            }
        };
        state.add_token(
            TokenType::Operand(OperandType::Literal, num),
            format!("#{}", num_lexeme),
        );
        state.inc_program_byte_count()?;
        Ok(())
    }

    /*
    Tokenize an identifier, either:
    * A register (R0-R12)
    * A label operand
    * An opcode
    */
    fn tokenizer_identifier(state: &mut TokenizerState) -> Result<(), TokenizerError> {
        let identifier = consume_identifier(state);

        // If the identifier is an R, it must be a register
        if identifier == "R" {
            // Get register number, return an error if none is found
            let (register_num, register_num_lexeme) = match consume_u8(state) {
                Some(num) => num?,
                None => {
                    return Err(TokenizerError {
                        message: String::from(
                            "Expected register number after register denoter 'R'",
                        ),
                        line: state.line,
                        col: state.col - 1,
                    })
                }
            };
            // Is register number valid?
            if register_num >= REGISTER_COUNT {
                return Err(TokenizerError {
                    message: format!("No register exists with register number '{}'. Must be in range 0-12 inclusive", register_num),
                    line: state.line,
                    col: state.col - register_num_lexeme.len() - 1,
                });
            }
            // Add register token
            state.add_token(
                TokenType::Operand(OperandType::Register, register_num),
                identifier + &register_num_lexeme,
            );
            state.inc_program_byte_count()?;
        } else {
            // check if the identifier is an opcode
            if let Ok(opcode) = AssemblyOpcode::from_str(&identifier) {
                state.add_token(TokenType::Opcode(opcode), identifier);
                state.inc_program_byte_count()?;
            } else {
                // If a colon follows, it is a label definition
                if let Some(&ch) = state.iter.peek() {
                    if ch == ':' {
                        // Label definitions can only be added if the previous token was a line delimeter
                        if let Some(token) = state.tokens.last() {
                            match token.ty {
                                TokenType::Semicolon | TokenType::Newline => {}
                                _ => return Err(TokenizerError {
                                    message: String::from("Label definitions are only permitted after line delimeters"), 
                                    line: token.line,
                                    col: token.col + string_col_width(&token.lexeme),
                                })
                            }
                        }
                        // Insert label - return error if label already is there
                        let label_col = state.col - string_col_width(&identifier);
                        match state.labels.insert(
                            identifier.clone(),
                            LabelDefinition {
                                byte: state.program_bytes as u8,
                                line: state.line,
                                col: label_col,
                            },
                        ) {
                            Some(_) => {
                                return Err(TokenizerError {
                                    message: format!(
                                        "Label with name '{}' already exists",
                                        &identifier
                                    ),
                                    line: state.line,
                                    col: label_col,
                                })
                            }
                            None => {}
                        };
                        state.next();
                        return Ok(());
                    }
                }
                // Otherwise it's a label operand
                state.add_token(TokenType::Operand(OperandType::Label, 0), identifier);
                state.inc_program_byte_count()?;
            }
        }
        Ok(())
    }

    /// Tokenize a single or multiline comment
    fn comment(state: &mut TokenizerState) -> Result<(), TokenizerError> {
        state.next();
        // Line comment
        match state.iter.peek() {
            Some(&ch) => match ch {
                '/' => state.next_until('\n'),
                '*' => loop {
                    if let Some(ch) = state.iter.next() {
                        if ch == '*' {
                            if let Some(&next) = state.iter.peek() {
                                if next == '/' {
                                    state.iter.next();
                                    break;
                                }
                            } else {
                                return Err(TokenizerError {
                                    message: String::from("Unterminated block comment"),
                                    line: state.line,
                                    col: state.col,
                                });
                            }
                        }
                    } else {
                        return Err(TokenizerError {
                            message: String::from("Unterminated block comment"),
                            line: state.line,
                            col: state.col,
                        });
                    }
                },
                _ => {}
            },
            None => {
                return Err(TokenizerError {
                    message: String::from("Expected '//' or '/*' for comment not '/'"),
                    line: state.line,
                    col: state.col,
                })
            }
        }
        Ok(())
    }

    while let Some(&ch) = state.iter.peek() {
        // Ignore any whitespace characters
        if ch != '\n' && ch.is_whitespace() {
            state.next();
            continue;
        }
        // Tokenizer based on characters
        match ch {
            '\n' => tokenize_single_char_token(&mut state, TokenType::Newline, ch),
            '/' => comment(&mut state)?,
            ';' => tokenize_single_char_token(&mut state, TokenType::Semicolon, ch),
            ',' => tokenize_single_char_token(&mut state, TokenType::Comma, ch),
            '0'..='9' => tokenize_memory_ref(&mut state)?,
            '#' => tokenize_literal(&mut state)?,
            'a'..='z' | 'A'..='Z' | '_' => tokenizer_identifier(&mut state)?,
            _ => {
                return Err(TokenizerError {
                    message: format!("Unexpected character: {}", ch),
                    line: state.line,
                    col: state.col,
                })
            }
        }
    }
    // Append an EOF token
    state.add_token(TokenType::EOF, String::from(""));
    Ok((state.tokens, state.labels, state.program_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_token_types(tokens: Vec<Token>) -> Vec<TokenType> {
        tokens.into_iter().map(|token| token.ty).collect()
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
            let (tokens, labels, _) = tokenize(input).unwrap();
            assert_eq!(extract_token_types(tokens), vec![expected_output, TokenType::EOF]);
            assert_eq!(labels, HashMap::new());
        }
    }

    #[test]
    fn test_comment_line_single() {
        let input = "NOP // Comment";
        let tokens = extract_token_types(tokenize(input).unwrap().0);
        let expected = vec![
            TokenType::Opcode(AssemblyOpcode::NOP),
            TokenType::EOF,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comment_line_multiline() {
        let input = "NOP /* Multiline \n Comment */";
        let tokens = extract_token_types(tokenize(input).unwrap().0);
        let expected = vec![
            TokenType::Opcode(AssemblyOpcode::NOP),
            TokenType::EOF,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comment_block_multiline() {
        let input = "NOP /* Multiline \n Comment \n */";
        let tokens = extract_token_types(tokenize(input).unwrap().0);
        let expected = vec![
            TokenType::Opcode(AssemblyOpcode::NOP),
            TokenType::EOF,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_invalid_register_number() {
        let input = "R13";
        let result = tokenize(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_too_large_program() {
        let input = "NOP;".repeat(257);
        let result = tokenize(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_program() {
        let input = "";
        assert_eq!(extract_token_types(tokenize(input).unwrap().0), vec![TokenType::EOF])
    }

    #[test]
    fn test_label_definition_not_after_line_delimeters() {
        let input = "NOP label:";
        let result = tokenize(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_memory_reference_too_large() {
        let input = "256";
        assert!(tokenize(input).is_err());
    }

    #[test]
    fn test_literal_value_too_large() {
        let input = "256";
        assert!(tokenize(input).is_err());
    }

    #[test]
    fn test_invalid_characters() {
        let input = "NOP; label: ??";
        assert!(tokenize(input).is_err());
    }
}