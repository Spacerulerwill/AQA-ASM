#![forbid(unsafe_code)]

mod interpreter;
mod parser;
mod tokenizer;

use clap::Parser;
use inline_colorization::{color_green, color_red, color_reset, style_bold, style_reset};
use interpreter::{interpret, RuntimeError};
use parser::{parse, ParserError};
use std::{collections::HashSet, fs};
use strsim::normalized_damerau_levenshtein;
use tokenizer::{tokenize, OperandType};

/// An interpreter for the AQA assembly language
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The name of the file to process
    #[arg(index = 1)]
    filepath: String,

    /// Width of tabs
    #[arg(short, long, default_value_t = 4)]
    tabsize: u8,

    /// Show trace table after program execution
    #[arg(long)]
    trace: bool,
}

/// prints bold and green
macro_rules! good_print {
    ($($arg:tt)*) => {
        {
            println!("{style_bold}{color_green}{}{color_reset}{style_reset}", format!($($arg)*))
        }
    };
}

// prints bold and red
macro_rules! bad_print {
    ($($arg:tt)*) => {
        {
            eprintln!("{style_bold}{color_red}{}{color_reset}{style_reset}", format!($($arg)*))
        }
    };
}

// bad_print with line and column number
macro_rules! print_syntax_error {
    ($arg1:expr, $arg2:expr, $($arg:tt)*) => {
        bad_print!("Syntax Error :: Line {}, Column {} :: {}", $arg1, $arg2, format!($($arg)*))
    };
}

fn main() {
    // Command line arg handling
    let args = Args::parse();
    let filepath = args.filepath;
    let trace = args.trace;
    let tabsize = args.tabsize;

    // Read in source file
    let source = match fs::read_to_string(&filepath) {
        Ok(source) => source,
        Err(err) => {
            bad_print!("Failed to read the file {filepath}: {err}");
            return;
        }
    };

    // Tokenize source code string
    let (tokens, labels, program_bytes) = match tokenize(&source, tabsize) {
        Ok(res) => res,
        Err(err) => {
            match err {
                tokenizer::TokenizerError::ProgramTooLarge { line, col } => print_syntax_error!(
                    line, col, "Program exceeds memory limit (255 bytes)"
                ),
                tokenizer::TokenizerError::LiteralValueTooLarge {
                    value_string,
                    line,
                    col,
                } => print_syntax_error!(
                    line,
                    col,
                    "Literal value '{}' too large (max value of 255)",
                    &value_string
                ),
                tokenizer::TokenizerError::MissingNumberAfterRegisterDenoter { line, col } => print_syntax_error!(
                    line,
                    col,
                    "Missing number after register denoter 'R'"
                ),
                tokenizer::TokenizerError::MissingNumberAfterLiteralDenoter { line, col } => print_syntax_error!(
                    line,
                    col,
                    "Missing number after literal denoter '#'"
                ),
                tokenizer::TokenizerError::InvalidRegisterNumber { value, line, col } => print_syntax_error!(
                    line,
                    col,
                    "Invalid register 'R{value}' (must be in range 0-12 inclusive)",
                ),
                tokenizer::TokenizerError::InvalidLabelDefinitionLocation {
                    label_name,
                    line,
                    col,
                } => print_syntax_error!(
                    line,
                    col,
                    "Invalid label definition location for label '{}', labels may only appear after line delimeters (newline or ';')",
                    &label_name
                ),
                tokenizer::TokenizerError::DuplicateLabelDefinition {
                    label_name,
                    line,
                    col,
                } => print_syntax_error!(
                    line,
                    col,
                    "Definition for label '{}' already exists",
                    &label_name
                ),
                tokenizer::TokenizerError::UnterminatedBlockComment { line, col } => print_syntax_error!(
                    line,
                    col,
                    "Unterminated block comment begins here"
                ),
                tokenizer::TokenizerError::InvalidCommentDenoter { line, col } => print_syntax_error!(
                    line,
                    col,
                    "Expected '//' or '/*' for comment not '/'"
                ),
                tokenizer::TokenizerError::UnexpectedCharacter { char, line, col } => print_syntax_error!(
                    line,
                    col,
                    "Unexpected character: {char}"
                ),
            }
            return;
        }
    };

    // Parse and load the instructions into memory
    let mut memory = match parse(&tokens, &labels) {
        Ok(memory) => memory,
        Err(err) => {
            match err {
                ParserError::ExpectedLineDelimeter { got } => print_syntax_error!(
                    got.line,
                    got.col,
                    "Expected line delimeter (semicolon or newline), found token {}",
                    &got.get_token_debug_repr(),
                ),
                ParserError::ExpectedOpcode { got } => print_syntax_error!(
                    got.line,
                    got.col,
                    "Expected instruction opcode, found token {}",
                    &got.get_token_debug_repr(),
                ),
                ParserError::ExpectedOperand { expected, got } => {
                    assert!(expected.len() > 0);
                    let unique_expected: HashSet<OperandType> =
                        HashSet::from_iter(expected.iter().cloned());
                    if unique_expected.len() == 1 {
                        print_syntax_error!(
                            got.line,
                            got.col,
                            "Unexpected token {}, expected {:?}",
                            &got.get_token_debug_repr(),
                            expected.get(0).unwrap()
                        )
                    } else {
                        let result = unique_expected
                            .iter()
                            .map(|s| format!("\t• {:?}", s))
                            .collect::<Vec<_>>()
                            .join("\n");
                        print_syntax_error!(
                            got.line,
                            got.col,
                            "Unexpected token {}, expected one of the following:\n{}",
                            &got.get_token_debug_repr(),
                            &result
                        )
                    }
                }
                ParserError::UnexpectedToken { expected, got } => print_syntax_error!(
                    got.line,
                    got.col,
                    "Expected {:?}, found {}",
                    expected,
                    &got.get_token_debug_repr(),
                ),
                ParserError::InvalidLabel { token } => {
                    let mut most_similar_label = &token.lexeme;
                    let mut max_similarity = 0.0;
                    for name in labels.keys() {
                        let similitary = normalized_damerau_levenshtein(&token.lexeme, name);
                        if similitary > max_similarity {
                            max_similarity = similitary;
                            most_similar_label = name;
                        }
                    }

                    if max_similarity > 0.5 {
                        print_syntax_error!(
                            token.line,
                            token.col,
                            "No label exists with name: {}, did you mean '{}'?",
                            &token.get_token_debug_repr(),
                            most_similar_label
                        )
                    } else {
                        print_syntax_error!(
                            token.line,
                            token.col,
                            "No label exists with name: {}",
                            &token.get_token_debug_repr()
                        )
                    }
                }
            }
            return;
        }
    };

    // Run the program
    let free_memory = 256 - program_bytes;
    good_print!(
        "Running program '{}' ({}/256 bytes in use, {} bytes free)",
        &filepath,
        program_bytes,
        free_memory
    );
    if let Err(err) = interpret(&mut memory, program_bytes) {
        match err {
            RuntimeError::ReadPastMemory => bad_print!("Runtime Error :: Program read past of available memory (perhaps you forgot the 'HALT' instruction?)"),
            RuntimeError::OutOfBoundsRead(idx) => bad_print!("Runtime Error :: Attempt to read memory location {idx} but max memory location is {}", free_memory - 1),
            RuntimeError::OutOfBoundsWrite(idx) => bad_print!("Runtime Error :: Attempt to write to memory location {idx} but max memory location is {}", free_memory - 1)
        }
    } else {
        good_print!("Program exited successfully");
    }
}
