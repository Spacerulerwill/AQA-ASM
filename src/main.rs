use std::collections::HashMap;
use std::{env, fs, io};
use std::str::FromStr;
use maplit::hashmap;
use std::error::Error;

// We get a predefined amount of registers allocated
const REGISTER_COUNT: u8 = 13;

/*
These are the opcodes for the AQA Assembly Language. They are the ones found in
the input source files that the user writes, for example:

MOV R0 #5
 */
#[derive(Debug, Clone)]
enum AssemblyOpcode {
    Ldr,
    Str,
    Add,
    Sub,
    Mov,
    Cmp,
    B,
    Beq,
    Bne,
    Bgt,
    Blt,
    And,
    Orr,
    Eor,
    Mvn,
    Lsl,
    Lsr,
    Halt,
    Print,
    Input,
}

impl FromStr for AssemblyOpcode {
    type Err = ();
    fn from_str(input: &str) -> Result<AssemblyOpcode, Self::Err> {
        match input {
            "LDR"  => Ok(AssemblyOpcode::Ldr),
            "STR" => Ok(AssemblyOpcode::Str),
            "ADD" => Ok(AssemblyOpcode::Add),
            "SUB" => Ok(AssemblyOpcode::Sub),
            "MOV" => Ok(AssemblyOpcode::Mov),
            "CMP" => Ok(AssemblyOpcode::Cmp),
            "B" => Ok(AssemblyOpcode::B),
            "BEQ" => Ok(AssemblyOpcode::Beq),
            "BNE" => Ok(AssemblyOpcode::Bne),
            "BGT" => Ok(AssemblyOpcode::Bgt),
            "BLT" => Ok(AssemblyOpcode::Blt),
            "AND" => Ok(AssemblyOpcode::And),
            "ORR" => Ok(AssemblyOpcode::Orr),
            "EOR" => Ok(AssemblyOpcode::Eor),
            "MVN" => Ok(AssemblyOpcode::Mvn),
            "LSL" => Ok(AssemblyOpcode::Lsl),
            "LSR" => Ok(AssemblyOpcode::Lsr),
            "HALT" => Ok(AssemblyOpcode::Halt),   
            "PRINT" => Ok(AssemblyOpcode::Print),
            "INPUT" => Ok(AssemblyOpcode::Input),
            _      => Err(()),
        }
    }
}

/*
These are the opcodes that the assembly converts too. They are the ones that
the machine code interpreter reads. They are different because some instructions
can have multiple argument signatures:

MOV R0 #5 <- this moves a literal
MOV R0 R1 <- this copies from a register

These are the same instructions in the assembly, but need to be different
instructions for the machine code as they treat the data in the second 
operand differently - one moves a literal value and one copies from a
register.

So here the instruction "MOV" is seperated into MovR and MovL - MOV register and MOV literal
 */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BinaryOpcode {
    Ldr,
    Str,
    Addr,
    Addl,
    Subr,
    Subl,
    Movr,
    Movl,
    Cmpr,
    Cmpl,
    B,
    Beq,
    Bne,
    Bgt,
    Blt,
    Andr,
    Andl,
    Orrr,
    Orrl,
    Eorr,
    Eorl,
    Mvnr,
    Mvnl,
    Lslr,
    Lsll,
    Lsrr,
    Lsrl,
    Halt,
    Printr,
    Printm,
    Inputr,
    Inputm,
}

// return potential formats for an instruction from its assembly opcode
fn get_opcode_operand_formats(opcode: AssemblyOpcode) -> HashMap<BinaryOpcode, Vec<Operand>> {
    match opcode {
        AssemblyOpcode::Ldr => hashmap! {BinaryOpcode::Ldr => vec![Operand::Register, Operand::MemoryRef]},
        AssemblyOpcode::Str => hashmap! {BinaryOpcode::Str => vec![Operand::Register, Operand::MemoryRef]},
        AssemblyOpcode::Add => hashmap! {
            BinaryOpcode::Addr => vec![Operand::Register, Operand::Register, Operand::Register], 
            BinaryOpcode::Addl => vec![Operand::Register, Operand::Register, Operand::Literal],
        },
        AssemblyOpcode::Sub => hashmap! {
            BinaryOpcode::Subr => vec![Operand::Register, Operand::Register, Operand::Register], 
            BinaryOpcode::Subl => vec![Operand::Register, Operand::Register, Operand::Literal],
        },
        AssemblyOpcode::Mov => hashmap! {
            BinaryOpcode::Movr => vec![Operand::Register, Operand::Register],
            BinaryOpcode::Movl => vec![Operand::Register, Operand::Literal],
        },
        AssemblyOpcode::Cmp => hashmap! {
            BinaryOpcode::Cmpr => vec![Operand::Register, Operand::Register],
            BinaryOpcode::Cmpl => vec![Operand::Register, Operand::Literal],
        },
        AssemblyOpcode::B => hashmap! {BinaryOpcode::B => vec![Operand::Label]},
        AssemblyOpcode::Beq => hashmap! {BinaryOpcode::Beq => vec![Operand::Label]},
        AssemblyOpcode::Bne => hashmap! {BinaryOpcode::Bne => vec![Operand::Label]},
        AssemblyOpcode::Bgt => hashmap! {BinaryOpcode::Bgt => vec![Operand::Label]},
        AssemblyOpcode::Blt => hashmap! {BinaryOpcode::Blt => vec![Operand::Label]},
        AssemblyOpcode::And => hashmap! {
            BinaryOpcode::Andr => vec![Operand::Register, Operand::Register, Operand::Register],
            BinaryOpcode::Andl => vec![Operand::Register, Operand::Register, Operand::Literal],
        },
        AssemblyOpcode::Orr => hashmap! {
            BinaryOpcode::Orrr => vec![Operand::Register, Operand::Register, Operand::Register],
            BinaryOpcode::Orrl => vec![Operand::Register, Operand::Register, Operand::Literal],
        },
        AssemblyOpcode::Eor => hashmap! {
            BinaryOpcode::Eorr => vec![Operand::Register, Operand::Register, Operand::Register],
            BinaryOpcode::Eorl => vec![Operand::Register, Operand::Register, Operand::Literal],
        },
        AssemblyOpcode::Mvn => hashmap! [
            BinaryOpcode::Mvnr => vec![Operand::Register, Operand::Register],
            BinaryOpcode::Mvnl => vec![Operand::Register, Operand::Literal],
        ],
        AssemblyOpcode::Lsl => hashmap! {
            BinaryOpcode::Lslr => vec![Operand::Register, Operand::Register, Operand::Register],
            BinaryOpcode::Lsll => vec![Operand::Register, Operand::Register, Operand::Literal],
        },
        AssemblyOpcode::Lsr => hashmap! {
            BinaryOpcode::Lsrr => vec![Operand::Register, Operand::Register, Operand::Register],
            BinaryOpcode::Lsrl => vec![Operand::Register, Operand::Register, Operand::Literal],
        },
        AssemblyOpcode::Print => hashmap! {
            BinaryOpcode::Printr => vec![Operand::Register],
            BinaryOpcode::Printm => vec![Operand::MemoryRef],
        },
        AssemblyOpcode::Input => hashmap! {
            BinaryOpcode::Inputr => vec![Operand::Register],
            BinaryOpcode::Inputm => vec![Operand::MemoryRef],
        },
        AssemblyOpcode::Halt => hashmap!{
            BinaryOpcode::Halt => vec![]
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Operand {
    Literal,
    Register,
    MemoryRef,
    Label,
}

#[derive(Debug, Clone)]
enum TokenType {
    Operand(Operand, u8),
    Opcode(AssemblyOpcode),
}

#[derive(Debug, Clone)]
struct Token {
    ty: TokenType,
    lexeme: String,
    line: usize,
    col: usize,
}

fn tokenize(input: &String) -> Result<(Vec<Token>, HashMap<String, u8>), Box<dyn Error>> {
    let mut tokens = Vec::new();
    let mut labels = HashMap::new();
    let mut iter = input.chars().peekable();
    let mut line = 1;
    let mut col = 1;

    while let Some(ch) = iter.next() {
        match ch {
            '\t' | '\r' => continue,
            ' ' => {
                col += 1;
                continue;
            }
            '\n' => {
                col = 1;
                line += 1;
                continue;
            }
            'R' => {
                let mut digits = String::new();
                let start_col = col;
                while let Some(&next_char) = iter.peek() {
                    col += 1;
                    if next_char.is_digit(10) {
                        digits.push(next_char);
                        iter.next();
                    } else {
                        break;
                    }
                }

                let lexeme: String = format!("{}{}", ch, digits);
                let val_wrapped = digits.parse::<u8>();
                let val = match val_wrapped {
                    Ok(val) => val,
                    Err(_) => return Err(format!("Line {line}, Col {start_col}: Invalid register {}. Only R0-R{} are supported", lexeme, REGISTER_COUNT-1).into())
                };
                if val > REGISTER_COUNT - 1 {
                    return Err(format!("Line {line}, Col {start_col}: Invalid register {}. Only R0-R{} are supported", lexeme, REGISTER_COUNT-1).into())
                }
                tokens.push(Token{
                    ty: TokenType::Operand(Operand::Register, val), 
                    lexeme: lexeme,
                    line: line,
                    col: start_col
                });
            }
            '#' => {
                let mut digits = String::new();
                let start_col = col;
                while let Some(&next_char) = iter.peek() {
                    col += 1;
                    if next_char.is_digit(10) {
                        digits.push(next_char);
                        iter.next();
                    } else {
                        break;
                    }
                }
                let lexeme: String = format!("{}{}", ch, digits);
                let val_wrapped = digits.parse::<u8>();
                let val = match val_wrapped {
                    Ok(val) => val,
                    Err(_) => return Err(format!("Line {line}, Col {start_col}: Literal {lexeme} too large to fit in 8 bits").into())
                };
                tokens.push(Token{
                    ty: TokenType::Operand(Operand::Literal, val), 
                    lexeme: lexeme,
                    line: line,
                    col: start_col
                });
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut lexeme = String::from(ch);
                let start_col = col;
                while let Some(&next_char) = iter.peek() {
                    col += 1;
                    if next_char.is_alphabetic() || next_char == '_' {
                        lexeme.push(next_char);
                        iter.next();
                    } else {
                        break;
                    }
                }
                
                let is_opcode:bool;
                // if the next char along is a colon - its a label. Otherwise its an opcode
                if let Some(&next_char) = iter.peek() {
                    if next_char == ':' {
                        // label
                        iter.next();
                        col += 1;
                        is_opcode = false;
                    } else {
                        is_opcode = true;
                    }
                } else {
                    is_opcode = true;
                }

                if is_opcode {
                    // potential opcode
                    match AssemblyOpcode::from_str(&lexeme) {
                    Ok(opcode) => {
                        tokens.push(Token{
                            ty: TokenType::Opcode(opcode), 
                            lexeme: lexeme,
                            line: line,
                            col: start_col
                        });
                    },
                    Err(_) => {
                        tokens.push(Token{
                            ty: TokenType::Operand(Operand::Label, tokens.len() as u8), 
                            lexeme: lexeme,
                            line: line,
                            col: start_col
                        });
                        },
                    };
                } else {
                    labels.insert(lexeme, tokens.len() as u8);
                }
            }
            '0'..='9' => {
                let mut digits = String::new();
                let start_col = col;
                while let Some(&next_char) = iter.peek() {
                    col += 1;
                    if next_char.is_digit(10) {
                        digits.push(next_char);
                        iter.next();
                    } else {
                        break;
                    }
                }
                let lexeme: String = format!("{}{}", ch, digits);
                let val_wrapped = lexeme.parse::<u8>();
                let val = match val_wrapped {
                    Ok(val) => val,
                    Err(_) => return Err(format!("Line {line}, Col {start_col}: Memory reference {lexeme} too large to fit in 8 bits!").into())
                };
                tokens.push(Token{
                    ty: TokenType::Operand(Operand::MemoryRef, val), 
                    lexeme: lexeme,
                    line: line,
                    col: start_col
                });
            }
            _ => return Err(format!("Line {line}, Col {col}: Unexpected character: {ch}").into())
        }
    }

    return Ok((tokens, labels));
}

fn load_instructions(memory: &mut [u8; 256], tokens: &Vec<Token>, labels: &HashMap<String, u8>) -> Result<(), Box<dyn Error>>{
    let mut memory_idx = 0;
    let mut token_iter = tokens.iter();
    while let Some(token) = token_iter.next() {
        match &token.ty {
            TokenType::Operand(_, _) => return Err(format!("Line {}, Col: {}: Unexpected identifier \"{}\"", token.line, token.col, token.lexeme).into()),
            TokenType::Opcode(assembly_opcode) => {
                let operand_formats = get_opcode_operand_formats(assembly_opcode.clone());
                let mut found_operand_format = false;
                // iterate over the formats checking each one to see if they match the operands given
                for (binary_opcode, operands) in operand_formats.iter() {
                    let mut does_format_match = true;
                    let save_iter = token_iter.clone();
                    let mut new_iter = token_iter.clone();
                    for operand in operands {
                        let operand_token = match new_iter.next() {
                            Some(operand_token) => operand_token,
                            None => return Err(format!("Line {}, Col {}: Not enough arguments for opcode {}", token.line, token.col, token.lexeme).into()),
                        };

                        match &operand_token.ty {
                            TokenType::Operand(compare_operand, _) => {
                                if compare_operand != operand {
                                    does_format_match = false;
                                    break;
                                }
                            },
                            _ => return Err(format!("Line {}, Col {}: Not enough operands provided for opcode {}", token.line, token.col, token.lexeme).into()),
                        }
                    }

                    if !does_format_match {
                        // the format didnt match, go back to the opcode and try the next format
                        token_iter = save_iter;
                    } else {
                        // TODO - make sure impossible to oveflow memory
                        // it did match, go back to the opcode and this time load the operands into memory
                        token_iter = save_iter;
                        // insert the binary opcode
                        memory[memory_idx] = binary_opcode.clone() as u8;
                        memory_idx+=1;
                        // insert the operands
                        for _ in 0..operands.len() {
                            let token: &Token = token_iter.next().unwrap();
                            match &token.ty {
                                TokenType::Operand(operand, val) => {
                                    // if its a label operand, we must check there is a label for it
                                    match operand {
                                        Operand::Label => {
                                            match labels.get(&token.lexeme) {
                                                Some(address) => memory[memory_idx] = *address,
                                                None => return Err(format!("Line {}, Col {}: Unexpected identifier {}", token.line, token.col, token.lexeme).into()),
                                            }
                                        },
                                        _ => {
                                            memory[memory_idx] = *val;
                                        }
                                    }
                                    memory_idx += 1
                                },
                                _ => panic!("This should never happen! This is a bug!"),
                            }
                        }
                        found_operand_format = true;
                        break;
                    }
                } 

                if !found_operand_format {
                    return Err(format!("Line {}, Col {}: Incorrect combination of operands for opcode {}", token.line, token.col, token.lexeme).into());
                }
            }
        }
    }
    return Ok(())
}

fn run_program(memory: &mut [u8; 256]) -> Result<(), Box<dyn Error>>{
    let mut idx = 0;
    let mut registers: [u8; 13] = [0; REGISTER_COUNT as usize]; // registers 0-12
    let mut comparison_result = 0;
    let mut underflow = false;

    loop {
        let instruction = *memory.get(idx).ok_or("Program read past end of available memory")? as u8;

        match instruction {
            // Load
            instruction if instruction == BinaryOpcode::Ldr as u8 => {
                idx += 1;
                let register_num = *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let memory_address = *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                registers[register_num] = memory[memory_address];
            },
            // Store
            instruction if instruction == BinaryOpcode::Str as u8 => {
                idx += 1;
                let register_num =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let memory_address =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                memory[memory_address] = registers[register_num];
            },
            // Addition
            instruction if instruction == BinaryOpcode::Addr as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let second_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                registers[store_register_idx] = registers[first_operand_register_idx].wrapping_add(registers[second_operand_register_idx]);
            }
            instruction if instruction == BinaryOpcode::Addl as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let literal = memory[idx];
                registers[store_register_idx] = registers[first_operand_register_idx].wrapping_add(literal);
            }
            // Subtraction
            instruction if instruction == BinaryOpcode::Subr as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let second_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                registers[store_register_idx] = registers[first_operand_register_idx].wrapping_sub(registers[second_operand_register_idx]);
            }
            instruction if instruction == BinaryOpcode::Subl as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let literal = memory[idx];
                registers[store_register_idx] = registers[first_operand_register_idx].wrapping_sub(literal);
            }
            // Move
            instruction if instruction == BinaryOpcode::Movr as u8 => {
                idx += 1;
                let register_num =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let second_register_num =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                registers[register_num] = registers[second_register_num];
            },
            instruction if instruction == BinaryOpcode::Movl as u8 => {
                idx += 1;
                let register_num =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let literal = memory[idx];
                registers[register_num] = literal;
            },
            // Bitwise AND
            instruction if instruction == BinaryOpcode::Andr as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let second_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                registers[store_register_idx] = registers[first_operand_register_idx] & registers[second_operand_register_idx];
            }
            instruction if instruction == BinaryOpcode::Andl as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let literal = memory[idx];
                registers[store_register_idx] = registers[first_operand_register_idx] & literal;
            }
            // Bitwise OR
            instruction if instruction == BinaryOpcode::Orrr as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let second_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                registers[store_register_idx] = registers[first_operand_register_idx] | registers[second_operand_register_idx];
            }
            instruction if instruction == BinaryOpcode::Orrl as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let literal = memory[idx];
                registers[store_register_idx] = registers[first_operand_register_idx] | literal;
            }
            // Bitwise XOR
            instruction if instruction == BinaryOpcode::Eorr as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let second_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                registers[store_register_idx] = registers[first_operand_register_idx] ^ registers[second_operand_register_idx];
            }
            instruction if instruction == BinaryOpcode::Eorl as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let literal = memory[idx];
                registers[store_register_idx] = registers[first_operand_register_idx] ^ literal;
            }
            // Bitwise NOT
            instruction if instruction == BinaryOpcode::Mvnr as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                registers[store_register_idx] = !registers[operand_register_idx];
            }
            instruction if instruction == BinaryOpcode::Mvnl as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let literal = memory[idx];
                registers[store_register_idx] = !literal;
            }
            // Left shift
            instruction if instruction == BinaryOpcode::Lslr as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let second_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                registers[store_register_idx] = registers[first_operand_register_idx].wrapping_shl(registers[second_operand_register_idx] as u32);
            }
            instruction if instruction == BinaryOpcode::Lsll as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let literal = memory[idx];
                registers[store_register_idx] = registers[first_operand_register_idx].wrapping_shl(literal as u32);
            }
            // Right shift
            instruction if instruction == BinaryOpcode::Lsrr as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let second_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                registers[store_register_idx] = registers[first_operand_register_idx].wrapping_shr(registers[second_operand_register_idx] as u32);
            }
            instruction if instruction == BinaryOpcode::Lsrl as u8 => {
                idx += 1;
                let store_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let first_operand_register_idx =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let literal = memory[idx];
                registers[store_register_idx] = registers[first_operand_register_idx].wrapping_shr(literal as u32);
            }
            // Print
            instruction if instruction == BinaryOpcode::Printm as u8 => {
                idx += 1;
                let val = memory[idx];
                println!("{}", val);
            },
            instruction if instruction == BinaryOpcode::Printr as u8 => {
                idx += 1;
                let register_num: usize =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                let val = registers[register_num];
                println!("{}", val);
            },
            // Input
            instrution if instruction == BinaryOpcode::Inputm as u8 => {
                idx += 1;
                let memory_address: usize =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                loop {            
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let number: Result<u8, _> = input.trim().parse();
            
                    match number {
                        Ok(num) => {
                            memory[memory_address] = num;
                            break;
                        }
                        Err(_) => {
                            continue; 
                        }
                    }
                }
            }
            instrution if instruction == BinaryOpcode::Inputr as u8 => {
                idx += 1;
                let register_idx: usize =  *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                loop {            
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let number: Result<u8, _> = input.trim().parse();
            
                    match number {
                        Ok(num) => {
                            registers[register_idx] = num;
                            break;
                        }
                        Err(_) => {
                            continue; 
                        }
                    }
                }
            }
            // Comparison
            instruction if instruction == BinaryOpcode::Cmpr as u8 => {
                idx += 1;
                let first_register_idx = *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let second_register_idx = *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                underflow = registers[second_register_idx] > registers[first_register_idx];
                comparison_result = registers[first_register_idx].wrapping_sub(registers[second_register_idx]);
            },
            instruction if instruction == BinaryOpcode::Cmpl as u8 => {
                idx += 1;
                let register_idx = *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx += 1;
                let literal = *memory.get(idx).ok_or("Program read past end of available memory")?;
                underflow = literal > registers[register_idx];
                comparison_result = registers[register_idx].wrapping_sub(literal);
            },
            // Branch
            instruction if instruction == BinaryOpcode::B as u8 => {
                idx += 1;
                let idx_to_branch_too = *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                idx = idx_to_branch_too;
                continue;
            }
            // Branch equal
            instruction if instruction == BinaryOpcode::Beq as u8 => {
                idx += 1;
                if comparison_result == 0 {
                    let idx_to_branch_too = *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                    idx = idx_to_branch_too;
                    continue;
                }
            }
            // Branch not equal
            instruction if instruction == BinaryOpcode::Bne as u8 => {
                idx += 1;
                if comparison_result != 0  {
                    let idx_to_branch_too = *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                    idx = idx_to_branch_too;
                    continue;
                }
            }
            // Branch greater
            instruction if instruction == BinaryOpcode::Bgt as u8 => {
                idx += 1;
                if comparison_result != 0 && !underflow {
                    let idx_to_branch_too = *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                    idx = idx_to_branch_too;
                    continue;
                }
            }
            // Branch less
            instruction if instruction == BinaryOpcode::Blt as u8 => {
                idx += 1;
                if underflow {
                    let idx_to_branch_too = *memory.get(idx).ok_or("Program read past end of available memory")? as usize;
                    idx = idx_to_branch_too;
                    continue;
                }
            }
            // Halt
            instruction if instruction == BinaryOpcode::Halt as u8 => {
                break;
            },
            _ => {}
        }
        idx += 1;
    }
    return Ok(())
}

fn main() {
    // Command line arg handling
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing filename argument");
        return;
    }

    // Load source file as byte vector (who needs utf-8 anyway)
    let filepath = &args[1];
    let source = fs::read_to_string(filepath).expect("Failed to read file");

    let mut memory: [u8; 256] = [0; 256];  // 256 bytes of memory

    // Tokenize source bytes
    let (tokens, labels) = match tokenize(&source) {
        Ok(res) => res,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    // Load instructions into memory
    if let Err(err) = load_instructions(&mut memory, &tokens, &labels) {
        println!("{err}");
        return;
    }
    
    // Run program
    if let Err(err) = run_program(&mut memory){
        println!("{err}");
        return;
    }
}