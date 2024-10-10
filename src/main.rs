#![forbid(unsafe_code)]

mod interpreter;
mod parser;
mod runtime_opcode;
mod source_opcode;
mod tokenizer;

use clap::Parser as ClapParser;
use inline_colorization::{color_green, color_red, color_reset, style_bold, style_reset};
use interpreter::{Interpreter, REGISTER_COUNT};
use parser::Parser;
use std::fs;
use tokenizer::{OperandType, Tokenizer};

/// An interpreter for the AQA assembly language
#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The name of the file to process
    #[arg(index = 1)]
    filepath: String,

    /// Width of tabs
    #[arg(short, long, default_value_t = 4)]
    tabsize: u8,
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

fn main() {
    // Command line arg handling
    let args = Args::parse();
    let filepath = args.filepath;
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
    let tokenizer = match Tokenizer::tokenize(&source, tabsize) {
        Ok(tokenizer) => tokenizer,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    // Parse and load the instructions into memory
    let mut memory = match Parser::parse(tokenizer.tokens, tokenizer.labels) {
        Ok(memory) => memory,
        Err(err) => {
            println! {"{err}"}
            return;
        }
    };

    // Run the program
    let free_memory = 256 - tokenizer.program_bytes;
    let mut registers = [0; REGISTER_COUNT as usize];
    good_print!(
        "Running program '{}' ({}/256 bytes in use, {} bytes free)",
        &filepath,
        tokenizer.program_bytes,
        free_memory
    );
    if let Err(err) = Interpreter::interpret(&mut memory, &mut registers, tokenizer.program_bytes) {
        println!("{err}")
    } else {
        good_print!("Program exited successfully");
    }
}
