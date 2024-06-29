#![forbid(unsafe_code)]

mod interpreter;
mod parser;
mod tokenizer;

use inline_colorization::{color_green, color_red, color_reset, style_bold, style_reset};
use interpreter::{interpret, RuntimeError};
use parser::{parse, ParserError};
use std::{
    collections::HashSet,
    env,
    fs
};
use strsim::normalized_damerau_levenshtein;
use tokenizer::{tokenize, OperandType};

/// prints bold and green
macro_rules! good_print {
    ($($arg:tt)*) => {
        {
            println!("{style_bold}{color_green}{}{color_reset}{style_reset}", format!($($arg)*));
        }
    };
}

// prints bold and red
macro_rules! bad_print {
    ($($arg:tt)*) => {
        {
            eprintln!("{style_bold}{color_red}{}{color_reset}{style_reset}", format!($($arg)*));
        }
    };
}

fn main() {
    // Command line arg handling
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bad_print!("Argument Error :: Missing argument 1 (filepath)");
        return;
    }

    // Read in source file
    let filepath = &args[1];
    let source = match fs::read_to_string(filepath) {
        Ok(source) => source,
        Err(err) => {
            bad_print!("Failed to read the file {filepath}: {err}");
            return;
        }
    };

    // Tokenize source code string
    let (tokens, labels, program_bytes) = match tokenize(&source) {
        Ok(res) => res,
        Err(err) => {
            bad_print!(
                "Tokenizer Error :: Line {}, Col {} :: {}",
                err.line,
                err.col,
                &err.message
            );
            return;
        }
    };

    // Parse and load the instructions into memory
    let mut memory = match parse(&tokens, &labels) {
        Ok(memory) => memory,
        Err(err) => {
            match err {
                ParserError::ExpectedLineDelimeter { got } => bad_print!(
                    "Syntax Error :: Line {}, Col {} :: Expected line delimeter (semicolon or newline), found token {}",
                    got.line,
                    got.col,
                    &got.get_token_debug_repr(),
                ),
                ParserError::ExpectedOpcode { got } => bad_print!(
                    "Syntax Error :: Line {}, Col {} :: Expected instruction opcode, found token {}",
                    got.line,
                    got.col,
                    &got.get_token_debug_repr(),
                ),
                ParserError::ExpectedOperand { expected, got } => {
                    assert!(expected.len() > 0);
                    let unique_expected: HashSet<OperandType> =   HashSet::from_iter(expected.iter().cloned());
                    if unique_expected.len() == 1 {
                        bad_print!(
                            "Syntax Error :: Line {}, Col {} :: Unexpected token {}, expected {:?}",
                            got.line,
                            got.col,
                            &got.get_token_debug_repr(),
                            expected.get(0).unwrap()
                        )
                    } else {
                        let result = unique_expected
                            .iter()
                            .map(|s| format!("\t• {:?}", s))
                            .collect::<Vec<_>>()
                            .join("\n");
                        bad_print!(
                            "Syntax Error :: Line {}, Col {} :: Unexpected token {}, expected one of the following:\n{}",
                            got.line,
                            got.col,
                            &got.get_token_debug_repr(),
                            &result
                        )
                    }
                },
                ParserError::UnexpectedToken { expected, got } => bad_print!(
                    "Syntax Error :: Line {}, Col {} :: Expected {:?}, found {}",
                    got.line,
                    got.col,
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
                        bad_print!(
                            "Syntax Error :: Line {}, Col {} :: No label exists with name: {}, did you mean '{}'?",
                            token.line,
                            token.col,
                            &token.get_token_debug_repr(),
                            most_similar_label
                        )
                    } else {
                        bad_print!(
                            "Syntax Error :: Line {}, Col {} :: No label exists with name: {}",
                            token.line,
                            token.col,
                            &token.get_token_debug_repr()
                        )
                    }
                }
            }
            return;
        }
    };

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
