use crate::runtime_opcode::RuntimeOpcode;
use std::io;

use super::RuntimeError;

// We get a predefined amount of registers allocated
pub const REGISTER_COUNT: u8 = 13;

#[derive(Debug)]
pub struct Interpreter<'a> {
    memory: &'a mut [u8; 256],
    registers: &'a mut [u8; REGISTER_COUNT as usize],
    program_bytes: usize,
    program_counter: usize,
    comparison_result: u8,
    underflow: bool,
}

impl<'a> Interpreter<'a> {
    // Create a new Tokenizer instance with the given input and tabsize
    pub fn interpret(
        memory: &'a mut [u8; 256],
        registers: &'a mut [u8; REGISTER_COUNT as usize],
        program_bytes: usize,
    ) -> Result<Self, RuntimeError> {
        let mut interpreter = Interpreter {
            memory: memory,
            registers: registers,
            program_bytes: program_bytes,
            program_counter: 0,
            comparison_result: 0,
            underflow: false,
        };
        interpreter.internal_interpret()?;
        Ok(interpreter)
    }

    fn internal_interpret(&mut self) -> Result<(), RuntimeError> {
        loop {
            let instruction = self.read_next_memory_address()?;

            let opcode: RuntimeOpcode = match instruction.try_into() {
                Ok(opcode) => opcode,
                Err(_) => panic!(
                    "Invalid opcode found while running program, please report as bug to author!"
                ),
            };

            match opcode {
                RuntimeOpcode::NOP => {}
                RuntimeOpcode::LDR => self.interpret_ldr()?,
                RuntimeOpcode::STR => self.interpret_str()?,
                RuntimeOpcode::ADD_REGISTER => self.interpret_add_register()?,
                RuntimeOpcode::ADD_LITERAL => self.interpret_add_literal()?,
                RuntimeOpcode::SUB_REGISTER => self.interpret_sub_register()?,
                RuntimeOpcode::SUB_LITERAL => self.interpret_sub_literal()?,
                RuntimeOpcode::MOV_REGISTER => self.interpret_mov_register()?,
                RuntimeOpcode::MOV_LITERAL => self.interpret_mov_literal()?,
                RuntimeOpcode::CMP_REGISTER => self.interpret_cmp_register()?,
                RuntimeOpcode::CMP_LITERAL => self.interpret_cmp_literal()?,
                RuntimeOpcode::B => self.interpret_b()?,
                RuntimeOpcode::BEQ => self.interpret_beq()?,
                RuntimeOpcode::BNE => self.interpret_bne()?,
                RuntimeOpcode::BGT => self.interpret_bgt()?,
                RuntimeOpcode::BLT => self.interpret_blt()?,
                RuntimeOpcode::AND_REGISTER => self.interpret_and_register()?,
                RuntimeOpcode::AND_LITERAL => self.interpret_and_literal()?,
                RuntimeOpcode::ORR_REGISTER => self.interpret_orr_register()?,
                RuntimeOpcode::ORR_LITERAL => self.interpret_orr_literal()?,
                RuntimeOpcode::EOR_REGISTER => self.interpret_eor_register()?,
                RuntimeOpcode::EOR_LITERAL => self.interpret_eor_literal()?,
                RuntimeOpcode::MVN_REGISTER => self.interpret_mvn_register()?,
                RuntimeOpcode::MVN_LITERAL => self.interpret_mvn_literal()?,
                RuntimeOpcode::LSL_REGISTER => self.interpret_lsl_register()?,
                RuntimeOpcode::LSL_LITERAL => self.interpret_lsl_literal()?,
                RuntimeOpcode::LSR_REGISTER => self.interpret_lsr_register()?,
                RuntimeOpcode::LSR_LITERAL => self.interpret_lsr_literal()?,
                RuntimeOpcode::PRINT_REGISTER => self.interpret_print_register()?,
                RuntimeOpcode::PRINT_MEMORY => self.interpret_print_memory()?,
                RuntimeOpcode::INPUT_REGISTER => self.interpret_input_register()?,
                RuntimeOpcode::INPUT_MEMORY => self.interpret_input_memory()?,
                RuntimeOpcode::HALT => break,
            }
        }
        Ok(())
    }

    fn read_next_memory_address(&mut self) -> Result<u8, RuntimeError> {
        if self.program_counter >= self.program_bytes {
            return Err(RuntimeError::ReadPastMemory);
        }
        match self.memory.get(self.program_counter) {
            Some(val) => {
                self.program_counter += 1;
                Ok(*val)
            }
            None => Err(RuntimeError::ReadPastMemory),
        }
    }

    fn read_memory_address(&self, idx: usize) -> Result<u8, RuntimeError> {
        match self.memory.get(self.program_bytes + idx) {
            Some(&val) => Ok(val),
            None => Err(RuntimeError::OutOfBoundsRead(idx)),
        }
    }

    fn write_memory_address(&mut self, val: u8, idx: usize) -> Result<(), RuntimeError> {
        match self.memory.get_mut(self.program_bytes + idx) {
            Some(v) => {
                *v = val;
                Ok(())
            }
            None => Err(RuntimeError::OutOfBoundsWrite(idx)),
        }
    }

    fn take_u8_input() -> u8 {
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

    fn interpret_ldr(&mut self) -> Result<(), RuntimeError> {
        let register = self.read_next_memory_address()? as usize;
        let memory_ref = self.read_next_memory_address()? as usize;
        self.registers[register] = self.read_memory_address(memory_ref)?;
        Ok(())
    }

    fn interpret_str(&mut self) -> Result<(), RuntimeError> {
        let register = self.read_next_memory_address()? as usize;
        let memory_ref = self.read_next_memory_address()? as usize;
        self.write_memory_address(self.registers[register], memory_ref)?;
        Ok(())
    }

    fn interpret_add_register(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let register_operand_2 = self.registers[self.read_next_memory_address()? as usize];
        self.registers[register_store] = register_operand_1.wrapping_add(register_operand_2);
        Ok(())
    }

    fn interpret_add_literal(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let literal_operand_2 = self.read_next_memory_address()?;
        self.registers[register_store] = register_operand_1.wrapping_add(literal_operand_2);
        Ok(())
    }

    fn interpret_sub_register(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let register_operand_2 = self.registers[self.read_next_memory_address()? as usize];
        self.registers[register_store] = register_operand_1.wrapping_sub(register_operand_2);
        Ok(())
    }

    fn interpret_sub_literal(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let literal_operand_2 = self.read_next_memory_address()?;
        self.registers[register_store] = register_operand_1.wrapping_sub(literal_operand_2);
        Ok(())
    }

    fn interpret_mov_register(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand = self.read_next_memory_address()? as usize;
        self.registers[register_store] = self.registers[register_operand];
        Ok(())
    }

    fn interpret_mov_literal(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let literal_operand = self.read_next_memory_address()?;
        self.registers[register_store] = literal_operand;
        Ok(())
    }

    fn interpret_cmp_register(&mut self) -> Result<(), RuntimeError> {
        let register_operand_1 = self.read_next_memory_address()? as usize;
        let register_operand_2 = self.read_next_memory_address()? as usize;
        self.underflow = self.registers[register_operand_2] > self.registers[register_operand_1];
        self.comparison_result =
            self.registers[register_operand_1].wrapping_sub(self.registers[register_operand_2]);
        Ok(())
    }

    fn interpret_cmp_literal(&mut self) -> Result<(), RuntimeError> {
        let register_idx = self.read_next_memory_address()? as usize;
        let literal = self.read_next_memory_address()?;
        self.underflow = literal > self.registers[register_idx];
        self.comparison_result = self.registers[register_idx].wrapping_sub(literal);
        Ok(())
    }

    fn interpret_b(&mut self) -> Result<(), RuntimeError> {
        let idx_to_branch_too = self.read_next_memory_address()? as usize;
        self.program_counter = idx_to_branch_too;
        Ok(())
    }

    fn interpret_beq(&mut self) -> Result<(), RuntimeError> {
        if self.comparison_result == 0 {
            let idx_to_branch_too = self.read_next_memory_address()? as usize;
            self.program_counter = idx_to_branch_too;
        } else {
            self.program_counter += 1;
        }
        Ok(())
    }

    fn interpret_bne(&mut self) -> Result<(), RuntimeError> {
        if self.comparison_result != 0 {
            let idx_to_branch_too: usize = self.read_next_memory_address()? as usize;
            self.program_counter = idx_to_branch_too;
        } else {
            self.program_counter += 1;
        }
        Ok(())
    }

    fn interpret_bgt(&mut self) -> Result<(), RuntimeError> {
        if self.comparison_result != 0 && !self.underflow {
            let idx_to_branch_too = self.read_next_memory_address()? as usize;
            self.program_counter = idx_to_branch_too;
        } else {
            self.program_counter += 1;
        }
        Ok(())
    }

    fn interpret_blt(&mut self) -> Result<(), RuntimeError> {
        if self.underflow {
            let idx_to_branch_too = self.read_next_memory_address()? as usize;
            self.program_counter = idx_to_branch_too;
        } else {
            self.program_counter += 1;
        }
        Ok(())
    }

    fn interpret_and_register(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let register_operand_2 = self.registers[self.read_next_memory_address()? as usize];
        self.registers[register_store] = register_operand_1 & register_operand_2;
        Ok(())
    }

    fn interpret_and_literal(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let literal = self.read_next_memory_address()?;
        self.registers[register_store] = register_operand_1 & literal;
        Ok(())
    }

    fn interpret_orr_register(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let register_operand_2 = self.registers[self.read_next_memory_address()? as usize];
        self.registers[register_store] = register_operand_1 | register_operand_2;
        Ok(())
    }

    fn interpret_orr_literal(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let literal = self.read_next_memory_address()?;
        self.registers[register_store] = register_operand_1 | literal;
        Ok(())
    }

    fn interpret_eor_register(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let register_operand_2 = self.registers[self.read_next_memory_address()? as usize];
        self.registers[register_store] = register_operand_1 ^ register_operand_2;
        Ok(())
    }

    fn interpret_eor_literal(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let literal = self.read_next_memory_address()?;
        self.registers[register_store] = register_operand_1 ^ literal;
        Ok(())
    }

    fn interpret_mvn_register(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand = self.registers[self.read_next_memory_address()? as usize];
        self.registers[register_store] = !register_operand;
        Ok(())
    }

    fn interpret_mvn_literal(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let literal = self.read_next_memory_address()?;
        self.registers[register_store] = !literal;
        Ok(())
    }

    fn interpret_lsl_register(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let register_operand_2 = self.registers[self.read_next_memory_address()? as usize];
        self.registers[register_store] = register_operand_1.wrapping_shl(register_operand_2 as u32);
        Ok(())
    }

    fn interpret_lsl_literal(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let literal_operand_2 = self.read_next_memory_address()?;
        self.registers[register_store] = register_operand_1.wrapping_shl(literal_operand_2 as u32);
        Ok(())
    }

    fn interpret_lsr_register(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let register_operand_2 = self.registers[self.read_next_memory_address()? as usize];
        self.registers[register_store] = register_operand_1.wrapping_shr(register_operand_2 as u32);
        Ok(())
    }

    fn interpret_lsr_literal(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let literal_operand_2 = self.read_next_memory_address()?;
        self.registers[register_store] = register_operand_1.wrapping_shr(literal_operand_2 as u32);
        Ok(())
    }

    fn interpret_print_register(&mut self) -> Result<(), RuntimeError> {
        let register = self.read_next_memory_address()?;
        println!("{}", self.registers[register as usize]);
        Ok(())
    }

    fn interpret_print_memory(&mut self) -> Result<(), RuntimeError> {
        let memory_ref = self.read_next_memory_address()?;
        println!("{}", self.read_memory_address(memory_ref as usize)?);
        Ok(())
    }

    fn interpret_input_register(&mut self) -> Result<(), RuntimeError> {
        let register = self.read_next_memory_address()?;
        self.registers[register as usize] = Self::take_u8_input();
        Ok(())
    }

    fn interpret_input_memory(&mut self) -> Result<(), RuntimeError> {
        let memory_ref = self.read_next_memory_address()?;
        self.write_memory_address(Self::take_u8_input(), memory_ref as usize)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_test_program(program: &[u8]) -> [u8; 256] {
        let mut memory = [0; 256];
        memory[..program.len()].copy_from_slice(&program);
        memory
    }

    #[test]
    fn test_nop() {
        let program = [RuntimeOpcode::NOP as u8, RuntimeOpcode::HALT as u8];
        let mut memory = load_test_program(&program);
        let memory_copy = memory.clone();
        let mut registers = [0; REGISTER_COUNT as usize];
        let registers_copy = registers.clone();
        Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(memory, memory_copy);
        assert_eq!(registers, registers_copy);
    }

    #[test]
    fn test_ldr() {
        let program = [RuntimeOpcode::LDR as u8, 0, 0, RuntimeOpcode::HALT as u8, 5];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        Interpreter::interpret(&mut memory, &mut registers, program.len() - 1).unwrap();
        assert_eq!(registers[0], 5);
    }

    #[test]
    fn test_str() {
        let program = [RuntimeOpcode::STR as u8, 0, 0, RuntimeOpcode::HALT as u8];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 5;
        Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(memory[program.len()], 5);
    }

    #[test]
    fn test_add() {
        let program = [
            RuntimeOpcode::ADD_LITERAL as u8,
            0,
            0,
            5,
            RuntimeOpcode::ADD_REGISTER as u8,
            1,
            1,
            0,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(registers[0], 5);
        assert_eq!(registers[1], 5);
    }

    #[test]
    fn test_sub() {
        let program = [
            RuntimeOpcode::SUB_LITERAL as u8,
            0,
            0,
            5,
            RuntimeOpcode::SUB_REGISTER as u8,
            1,
            1,
            0,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 255;
        registers[1] = 255;
        Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(registers[0], 250);
        assert_eq!(registers[1], 5);
    }

    #[test]
    fn test_mov() {
        let program = [
            RuntimeOpcode::MOV_LITERAL as u8,
            0,
            5,
            RuntimeOpcode::MOV_REGISTER as u8,
            1,
            0,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(registers[0], 5);
        assert_eq!(registers[1], 5);
    }

    #[test]
    fn test_cmp_equal_numbers() {
        // Comparison of register 0 and value 0
        let program = &[
            RuntimeOpcode::CMP_LITERAL as u8,
            0,
            0,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(interpreter.comparison_result, 0);
        assert_eq!(interpreter.underflow, false);

        // Comparison of contents of register 0 and 1 (both have values of 0)
        let program = &[
            RuntimeOpcode::CMP_REGISTER as u8,
            0,
            1,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(program);
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(interpreter.comparison_result, 0);
        assert_eq!(interpreter.underflow, false);
    }

    #[test]
    fn test_cmp_greater_than() {
        // Comparison of register 0 (value of 5) and value 0
        let program = &[
            RuntimeOpcode::CMP_LITERAL as u8,
            0,
            0,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory: [u8; 256] = load_test_program(program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 5;
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(interpreter.comparison_result, 5);
        assert_eq!(interpreter.underflow, false);

        let program = &[
            RuntimeOpcode::CMP_REGISTER as u8,
            0,
            1,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(program);
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(interpreter.comparison_result, 5);
        assert_eq!(interpreter.underflow, false);
    }

    #[test]
    fn test_cmp_less_than() {
        // Comparison of register 0 (value of 0) and value 5
        let program = &[
            RuntimeOpcode::CMP_LITERAL as u8,
            0,
            5,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory: [u8; 256] = load_test_program(program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[1] = 5;
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(interpreter.comparison_result, 251);
        assert_eq!(interpreter.underflow, true);

        // Comaparison of register 0 (value of 0) and register 1 (value of 5)
        let program = &[
            RuntimeOpcode::CMP_REGISTER as u8,
            0,
            1,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(program);
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(interpreter.comparison_result, 251);
        assert_eq!(interpreter.underflow, true);
    }

    #[test]
    fn test_unconditional_branch() {
        let program = [
            RuntimeOpcode::B as u8,
            3,
            RuntimeOpcode::HALT as u8,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(interpreter.program_counter, 4);
    }

    #[test]
    fn test_beq() {
        // Successful BEQ
        let program = [
            RuntimeOpcode::BEQ as u8,
            3,
            RuntimeOpcode::HALT as u8,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let mut interpreter = Interpreter {
            comparison_result: 0,
            program_bytes: program.len(),
            memory: &mut memory,
            registers: &mut registers,
            program_counter: 0,
            underflow: false,
        };
        interpreter.internal_interpret().unwrap();
        assert_eq!(interpreter.program_counter, 4);

        // Unsuccessful BEQ
        interpreter.comparison_result = 1;
        interpreter.program_counter = 0;
        interpreter.internal_interpret().unwrap();
        assert_eq!(interpreter.program_counter, 3);
    }

    #[test]
    fn test_bne() {
        // Successful BNE
        let program = [
            RuntimeOpcode::BNE as u8,
            3,
            RuntimeOpcode::HALT as u8,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let mut interpreter = Interpreter {
            comparison_result: 1,
            program_bytes: program.len(),
            memory: &mut memory,
            registers: &mut registers,
            program_counter: 0,
            underflow: false,
        };
        interpreter.internal_interpret().unwrap();
        assert_eq!(interpreter.program_counter, 4);

        // Unsuccessful BGT
        interpreter.comparison_result = 0;
        interpreter.program_counter = 0;
        interpreter.internal_interpret().unwrap();
        assert_eq!(interpreter.program_counter, 3);
    }

    #[test]
    fn test_bgt() {
        // Successful BGT - num is greater
        let program = [
            RuntimeOpcode::BGT as u8,
            3,
            RuntimeOpcode::HALT as u8,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let mut interpreter = Interpreter {
            comparison_result: 1,
            program_bytes: program.len(),
            memory: &mut memory,
            registers: &mut registers,
            program_counter: 0,
            underflow: false,
        };
        interpreter.internal_interpret().unwrap();
        assert_eq!(interpreter.program_counter, 4);

        // Unsuccessful BGT - nums are equal
        interpreter.comparison_result = 0;
        interpreter.program_counter = 0;
        interpreter.internal_interpret().unwrap();
        assert_eq!(interpreter.program_counter, 3);

        // Unsuccesfull BGT - nums are less
        interpreter.comparison_result = 10;
        interpreter.underflow = true;
        interpreter.program_counter = 0;
        interpreter.internal_interpret().unwrap();
        assert_eq!(interpreter.program_counter, 3);
    }

    #[test]
    fn test_blt() {
        // Successful BLT - num is greater
        let program = [
            RuntimeOpcode::BLT as u8,
            3,
            RuntimeOpcode::HALT as u8,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let mut interpreter = Interpreter {
            comparison_result: 1,
            program_bytes: program.len(),
            memory: &mut memory,
            registers: &mut registers,
            program_counter: 0,
            underflow: true,
        };
        interpreter.internal_interpret().unwrap();
        assert_eq!(interpreter.program_counter, 4);

        // Unsuccessful BLT - nums are equal
        interpreter.comparison_result = 0;
        interpreter.underflow = false;
        interpreter.program_counter = 0;
        interpreter.internal_interpret().unwrap();
        assert_eq!(interpreter.program_counter, 3);

        // Unsuccesfull BLT - nums are less
        interpreter.comparison_result = 10;
        interpreter.program_counter = 0;
        interpreter.internal_interpret().unwrap();
        assert_eq!(interpreter.program_counter, 3);
    }

    #[test]
    fn test_and() {
        let program = [
            RuntimeOpcode::AND_LITERAL as u8,
            0,
            0,
            0b11110000,
            RuntimeOpcode::AND_REGISTER as u8,
            1,
            1,
            2,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b10101010;
        registers[1] = 0b10101010;
        registers[2] = 0b00001111;
        Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(registers[0], 0b10100000);
        assert_eq!(registers[1], 0b00001010);
    }

    #[test]
    fn test_orr() {
        let program = [
            RuntimeOpcode::ORR_LITERAL as u8,
            0,
            0,
            0b00001111,
            RuntimeOpcode::ORR_REGISTER as u8,
            1,
            1,
            0,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b11001010;
        Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(registers[0], 0b11001111);
        assert_eq!(registers[1], 0b11001111);
    }

    #[test]
    fn test_eor() {
        let program = [
            RuntimeOpcode::EOR_LITERAL as u8,
            0,
            0,
            0b00001111,
            RuntimeOpcode::EOR_REGISTER as u8,
            1,
            1,
            1,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b11001010;
        registers[1] = 0b11111111;
        Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(registers[0], 0b11000101);
        assert_eq!(registers[1], 0b00000000);
    }

    #[test]
    fn test_mvn() {
        let program = [
            RuntimeOpcode::MVN_LITERAL as u8,
            0,
            0b00001111,
            RuntimeOpcode::MVN_REGISTER as u8,
            1,
            0,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b11111111;
        registers[1] = 0b11111111;
        Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(registers[0], 0b11110000);
        assert_eq!(registers[1], 0b00001111);
    }

    #[test]
    fn test_lsl() {
        let program = [
            RuntimeOpcode::LSL_LITERAL as u8,
            0,
            0,
            4,
            RuntimeOpcode::LSL_REGISTER as u8,
            1,
            1,
            2,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b11000011;
        registers[1] = 0b11000011;
        registers[2] = 2;
        Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(registers[0], 0b00110000);
        assert_eq!(registers[1], 0b00001100);
    }

    #[test]
    fn test_lsr() {
        let program = [
            RuntimeOpcode::LSR_LITERAL as u8,
            0,
            0,
            4,
            RuntimeOpcode::LSR_REGISTER as u8,
            1,
            1,
            2,
            RuntimeOpcode::HALT as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b11000011;
        registers[1] = 0b11000011;
        registers[2] = 2;
        Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(registers[0], 0b00001100);
        assert_eq!(registers[1], 0b00110000);
    }

    #[test]
    fn test_halt() {
        let program = [RuntimeOpcode::HALT as u8];
        let mut memory = load_test_program(&program);
        let memory_copy = memory.clone();
        let mut registers = [0; REGISTER_COUNT as usize];
        let registers_copy = registers.clone();
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len()).unwrap();
        assert_eq!(*interpreter.memory, memory_copy);
        assert_eq!(*interpreter.registers, registers_copy);
        assert_eq!(interpreter.program_counter, 1);
    }

    #[test]
    fn test_out_of_bounds_read() {
        let program = [RuntimeOpcode::LDR as u8, 0, 253];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        assert!(matches!(
            Interpreter::interpret(&mut memory, &mut registers, program.len()),
            Err(RuntimeError::OutOfBoundsRead(..))
        ));
    }

    #[test]
    fn test_out_of_bounds_write() {
        let program = [RuntimeOpcode::STR as u8, 0, 253];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        assert!(matches!(
            Interpreter::interpret(&mut memory, &mut registers, program.len()),
            Err(RuntimeError::OutOfBoundsWrite(..))
        ));
    }

    #[test]
    fn test_read_past_of_memory() {
        let program = [];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        assert!(matches!(
            Interpreter::interpret(&mut memory, &mut registers, program.len()),
            Err(RuntimeError::ReadPastMemory)
        ))
    }
}
