use crate::tokenizer::BinaryOpcode;
use std::io;

// We get a predefined amount of registers allocated
pub const REGISTER_COUNT: u8 = 13;

#[derive(Debug)]
pub enum RuntimeError {
    ReadPastMemory,
    OutOfBoundsRead(usize),
    OutOfBoundsWrite(usize),
}

pub fn interpret(memory: &mut [u8; 256], program_bytes: usize) -> Result<(), RuntimeError> {
    let mut idx = 0;
    let mut registers: [u8; REGISTER_COUNT as usize] = [0; REGISTER_COUNT as usize];
    let mut comparison_result = 0;
    let mut underflow = false;

    fn read_next_memory_address(
        idx: &mut usize,
        memory: &[u8; 256],
        program_bytes: usize,
    ) -> Result<u8, RuntimeError> {
        if *idx >= program_bytes {
            return Err(RuntimeError::ReadPastMemory);
        }
        match memory.get(*idx) {
            Some(val) => {
                *idx += 1;
                Ok(*val)
            }
            None => Err(RuntimeError::ReadPastMemory),
        }
    }

    fn read_memory_address(
        idx: usize,
        program_bytes: usize,
        memory: &[u8; 256],
    ) -> Result<u8, RuntimeError> {
        match memory.get(program_bytes + idx) {
            Some(&val) => Ok(val),
            None => Err(RuntimeError::OutOfBoundsRead(idx)),
        }
    }

    fn write_memory_address(
        val: u8,
        idx: usize,
        program_bytes: usize,
        memory: &mut [u8; 256],
    ) -> Result<(), RuntimeError> {
        match memory.get_mut(program_bytes + idx) {
            Some(v) => {
                *v = val;
                Ok(())
            }
            None => Err(RuntimeError::OutOfBoundsWrite(idx)),
        }
    }

    fn read_u8() -> u8 {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(val) = input.trim().parse() {
                return val;
            }
        }
    }

    loop {
        let instruction = read_next_memory_address(&mut idx, &memory, program_bytes)?;

        let opcode: BinaryOpcode = match instruction.try_into() {
            Ok(opcode) => opcode,
            Err(_) => panic!(
                "Invalid opcode found while running program, please report as bug to author!"
            ),
        };

        match opcode {
            BinaryOpcode::NOP => {}
            BinaryOpcode::LDR => {
                let register = read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let memory_ref =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                registers[register] = read_memory_address(memory_ref, program_bytes, memory)?;
            }
            BinaryOpcode::STR => {
                let register = read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let memory_ref =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                write_memory_address(registers[register], memory_ref, program_bytes, memory)?;
            }
            BinaryOpcode::ADD_REGISTER => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let register_operand_2 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                registers[register_store] = register_operand_1.wrapping_add(register_operand_2);
            }
            BinaryOpcode::ADD_LITERAL => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let literal_operand_2 = read_next_memory_address(&mut idx, memory, program_bytes)?;
                registers[register_store] = register_operand_1.wrapping_add(literal_operand_2);
            }
            BinaryOpcode::SUB_REGISTER => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let register_operand_2 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                registers[register_store] = register_operand_1.wrapping_sub(register_operand_2);
            }
            BinaryOpcode::SUB_LITERAL => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let literal_operand_2 = read_next_memory_address(&mut idx, memory, program_bytes)?;
                registers[register_store] = register_operand_1.wrapping_sub(literal_operand_2);
            }
            BinaryOpcode::MOV_REGISTER => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                registers[register_store] = registers[register_operand];
            }
            BinaryOpcode::MOV_LITERAL => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let literal_operand = read_next_memory_address(&mut idx, memory, program_bytes)?;
                registers[register_store] = literal_operand;
            }
            BinaryOpcode::CMP_REGISTER => {
                let register_operand_1 =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_2 =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                underflow = registers[register_operand_2] > registers[register_operand_1];
                comparison_result =
                    registers[register_operand_1].wrapping_sub(registers[register_operand_2]);
            }
            BinaryOpcode::CMP_LITERAL => {
                let register_idx =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let literal = read_next_memory_address(&mut idx, memory, program_bytes)?;
                underflow = literal > registers[register_idx];
                comparison_result = registers[register_idx].wrapping_sub(literal);
            }
            BinaryOpcode::B => {
                let idx_to_branch_too =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                idx = idx_to_branch_too;
            }
            BinaryOpcode::BEQ => {
                if comparison_result == 0 {
                    let idx_to_branch_too =
                        read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                    idx = idx_to_branch_too;
                } else {
                    idx += 1;
                }
            }
            BinaryOpcode::BNE => {
                if comparison_result != 0 {
                    let idx_to_branch_too: usize =
                        read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                    idx = idx_to_branch_too;
                } else {
                    idx += 1;
                }
            }
            BinaryOpcode::BGT => {
                if comparison_result != 0 && !underflow {
                    let idx_to_branch_too =
                        read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                    idx = idx_to_branch_too;
                } else {
                    idx += 1;
                }
            }
            BinaryOpcode::BLT => {
                if underflow {
                    let idx_to_branch_too =
                        read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                    idx = idx_to_branch_too;
                } else {
                    idx += 1;
                }
            }
            BinaryOpcode::AND_REGISTER => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let register_operand_2 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                registers[register_store] = register_operand_1 & register_operand_2;
            }
            BinaryOpcode::AND_LITERAL => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let literal = read_next_memory_address(&mut idx, memory, program_bytes)?;
                registers[register_store] = register_operand_1 & literal;
            }
            BinaryOpcode::ORR_REGISTER => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let register_operand_2 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                registers[register_store] = register_operand_1 | register_operand_2;
            }
            BinaryOpcode::ORR_LITERAL => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let literal = read_next_memory_address(&mut idx, memory, program_bytes)?;
                registers[register_store] = register_operand_1 | literal;
            }
            BinaryOpcode::EOR_REGISTER => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let register_operand_2 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                registers[register_store] = register_operand_1 ^ register_operand_2;
            }
            BinaryOpcode::EOR_LITERAL => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let literal = read_next_memory_address(&mut idx, memory, program_bytes)?;
                registers[register_store] = register_operand_1 ^ literal;
            }
            BinaryOpcode::MVN_REGISTER => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                registers[register_store] = !register_operand;
            }
            BinaryOpcode::MVN_LITERAL => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let literal = read_next_memory_address(&mut idx, memory, program_bytes)?;
                registers[register_store] = !literal;
            }
            BinaryOpcode::LSL_REGISTER => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let register_operand_2 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                registers[register_store] =
                    register_operand_1.wrapping_shl(register_operand_2 as u32);
            }
            BinaryOpcode::LSL_LITERAL => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let literal_operand_2 = read_next_memory_address(&mut idx, memory, program_bytes)?;
                registers[register_store] =
                    register_operand_1.wrapping_shl(literal_operand_2 as u32);
            }
            BinaryOpcode::LSR_REGISTER => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let register_operand_2 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                registers[register_store] =
                    register_operand_1.wrapping_shr(register_operand_2 as u32);
            }
            BinaryOpcode::LSR_LITERAL => {
                let register_store =
                    read_next_memory_address(&mut idx, memory, program_bytes)? as usize;
                let register_operand_1 =
                    registers[read_next_memory_address(&mut idx, memory, program_bytes)? as usize];
                let literal_operand_2 = read_next_memory_address(&mut idx, memory, program_bytes)?;
                registers[register_store] =
                    register_operand_1.wrapping_shr(literal_operand_2 as u32);
            }
            BinaryOpcode::PRINT_REGISTER => {
                let register = read_next_memory_address(&mut idx, &memory, program_bytes)?;
                println!("{}", registers[register as usize])
            }
            BinaryOpcode::PRINT_MEMORY => {
                let memory_ref = read_next_memory_address(&mut idx, &memory, program_bytes)?;
                println!(
                    "{}",
                    read_memory_address(memory_ref as usize, program_bytes, memory)?
                );
            }
            BinaryOpcode::INPUT_REGISTER => {
                let register = read_next_memory_address(&mut idx, memory, program_bytes)?;
                registers[register as usize] = read_u8();
            }
            BinaryOpcode::INPUT_MEMORY => {
                let memory_ref = read_next_memory_address(&mut idx, memory, program_bytes)?;
                write_memory_address(read_u8(), memory_ref as usize, program_bytes, memory)?;
            }
            BinaryOpcode::HALT => break,
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}