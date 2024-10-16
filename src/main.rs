#![forbid(unsafe_code)]

mod interpreter;
mod parser;
mod tokenizer;

use clap::Parser as ClapParser;
use inline_colorization::{color_green, color_red, color_reset, style_bold, style_reset};
use interpreter::{Interpreter, REGISTER_COUNT};
use parser::Parser;
use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
};
use tokenizer::Tokenizer;

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

// Add this to your main.rs or lib.rs
pub fn run_interpreter<R: BufRead, W: Write>(
    filepath: &str,
    tabsize: u8,
    reader: R,
    writer: W,
) -> Result<(Vec<u8>, [u8; 13]), String> {
    // Read in source file
    let source = fs::read_to_string(filepath)
        .map_err(|err| format!("Failed to read the file {}: {}", filepath, err))?;

    // Tokenize source code string
    let tokenizer = Tokenizer::tokenize(&source, tabsize).map_err(|err| err.to_string())?;

    // Parse and load the instructions into memory
    let mut memory =
        Parser::parse(tokenizer.tokens, tokenizer.labels).map_err(|err| err.to_string())?;

    // Run the program
    let free_memory = 256 - tokenizer.program_bytes;
    let mut registers = [0; REGISTER_COUNT as usize];

    // Print the program running message
    good_print!(
        "Running program '{}' ({}/256 bytes in use, {} bytes free)",
        filepath,
        tokenizer.program_bytes,
        free_memory
    );

    // Execute the program and handle errors
    Interpreter::interpret_custom_io(
        &mut memory,
        &mut registers,
        tokenizer.program_bytes,
        reader,
        writer,
    )
    .map_err(|err| err.to_string())?;

    good_print!("Program exited successfully");

    let result_memory = memory[tokenizer.program_bytes..].to_owned();
    Ok((result_memory, registers))
}

fn main() {
    // Command line arg handling
    let args = Args::parse();

    if let Err(err) = run_interpreter(
        &args.filepath,
        args.tabsize,
        BufReader::new(io::stdin()),
        io::stdout(),
    ) {
        bad_print!("{}", err);
    }
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    #[test]
    fn test_example_addition() {
        let input = String::from("105\n25");
        let inputs = input.as_bytes();
        let mut output: Vec<u8> = Vec::new();

        let reader = BufReader::new(Cursor::new(inputs));
        let writer = Cursor::new(&mut output);

        let (_, registers) = run_interpreter("examples/addition.aqasm", 4, reader, writer).unwrap();
        assert_eq!(registers[2], 130);
    }

    #[test]
    fn test_example_division() {
        let input = String::from("35\n6");
        let inputs = input.as_bytes();
        let mut output: Vec<u8> = Vec::new();

        let reader = BufReader::new(Cursor::new(inputs));
        let writer = Cursor::new(&mut output);

        let (_, registers) = run_interpreter("examples/division.aqasm", 4, reader, writer).unwrap();
        assert_eq!(registers[0], 5);
        assert_eq!(registers[1], 5);
    }

    #[test]
    fn test_example_subtraction() {
        let input = String::from("35\n6");
        let inputs = input.as_bytes();
        let mut output: Vec<u8> = Vec::new();

        let reader = BufReader::new(Cursor::new(inputs));
        let writer = Cursor::new(&mut output);

        let (_, registers) = run_interpreter("examples/subtraction.aqasm", 4, reader, writer).unwrap();
        assert_eq!(registers[2], 29);
    }

    #[test]
    fn test_example_multiplication() {
        let input = String::from("5\n5");
        let inputs = input.as_bytes();
        let mut output: Vec<u8> = Vec::new();

        let reader = BufReader::new(Cursor::new(inputs));
        let writer = Cursor::new(&mut output);

        let (_, registers) = run_interpreter("examples/multiplication.aqasm", 4, reader, writer).unwrap();
        assert_eq!(registers[3], 25);
    }

    #[test]
    fn test_example_hamming_weight() {
        let input = String::from("12");
        let inputs = input.as_bytes();
        let mut output: Vec<u8> = Vec::new();

        let reader = BufReader::new(Cursor::new(inputs));
        let writer = Cursor::new(&mut output);

        let (_, registers) = run_interpreter("examples/hamming_weight.aqasm", 4, reader, writer).unwrap();
        assert_eq!(registers[2], 2);
    }

    #[test]
    fn test_example_do_while_loop() {
        let input = String::from("12\n45\n22\n69");
        let inputs = input.as_bytes();
        let mut output: Vec<u8> = Vec::new();

        let reader = BufReader::new(Cursor::new(inputs));
        let writer = Cursor::new(&mut output);

        let (_, registers) = run_interpreter("examples/do_while_loop.aqasm", 4, reader, writer).unwrap();
        assert_eq!(registers[1], 4);
    }

    #[test]
    fn test_example_for_loop() {
        let input = String::new();
        let inputs = input.as_bytes();
        let mut output: Vec<u8> = Vec::new();

        let reader = BufReader::new(Cursor::new(inputs));
        let writer = Cursor::new(&mut output);

        let (_, _) = run_interpreter("examples/for_loop.aqasm", 4, reader, writer).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let expected_output: String = (0..=254).map(|n| format!("{}\n", n)).collect();
        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn test_run_interpreter_file_not_found() {
        // Arrange
        let invalid_file = "invalid_file.asm";
        let tabsize = 4;

        // Act
        let result = run_interpreter(
            invalid_file,
            tabsize,
            BufReader::new(io::stdin()),
            io::stdout(),
        );

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to read the file"));
    }
}
