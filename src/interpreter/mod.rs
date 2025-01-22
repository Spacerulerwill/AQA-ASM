mod error;
pub use error::*;
use instruction::runtime_opcode::RuntimeOpcode;
pub mod instruction;

use std::io::{BufRead, Write};

// We get a predefined amount of registers allocated
pub const REGISTER_COUNT: u8 = 13;

#[derive(Debug)]
pub struct Interpreter<'a, R: BufRead, W: Write> {
    memory: &'a mut [u8; 256],
    registers: &'a mut [u8; REGISTER_COUNT as usize],
    program_bytes: u8,
    program_counter: u8,
    comparison_result: u8,
    underflow: bool,
    reader: R,
    writer: W,
}

#[cfg(test)]
impl<'a> Interpreter<'a, std::io::BufReader<std::io::Stdin>, std::io::Stdout> {
    pub fn interpret(
        memory: &'a mut [u8; 256],
        registers: &'a mut [u8; REGISTER_COUNT as usize],
        program_bytes: u8,
    ) -> Result<Self, RuntimeError> {
        let stdin = std::io::BufReader::new(std::io::stdin());
        let stdout = std::io::stdout();
        Interpreter::interpret_custom_io(memory, registers, program_bytes, stdin, stdout)
    }
}

impl<'a, R: BufRead, W: Write> Interpreter<'a, R, W> {
    pub fn interpret_custom_io(
        memory: &'a mut [u8; 256],
        registers: &'a mut [u8; REGISTER_COUNT as usize],
        program_bytes: u8,
        reader: R,
        writer: W,
    ) -> Result<Self, RuntimeError> {
        let mut interpreter = Interpreter {
            memory,
            registers,
            program_bytes,
            program_counter: 0,
            comparison_result: 0,
            underflow: false,
            reader,
            writer,
        };

        interpreter.internal_interpret()?;
        Ok(interpreter)
    }

    pub fn read_line(&mut self) -> String {
        let mut input = String::new();
        self.reader
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim_end().to_string()
    }

    pub fn write_line(&mut self, output: &str) {
        self.writer
            .write_all(output.as_bytes())
            .expect("Failed to write line");
        self.writer
            .write_all(b"\n")
            .expect("Failed to write newline");
        self.writer.flush().expect("Failed to flush writer");
    }

    fn internal_interpret(&mut self) -> Result<(), RuntimeError> {
        loop {
            let instruction = self.read_next_memory_address()?;

            let opcode: RuntimeOpcode = match instruction.try_into() {
                Ok(opcode) => opcode,
                Err(()) => panic!(
                    "Invalid opcode found while running program, please report as bug to author!"
                ),
            };

            match opcode {
                RuntimeOpcode::Nop => {}
                RuntimeOpcode::Ldr => self.interpret_ldr()?,
                RuntimeOpcode::Str => self.interpret_str()?,
                RuntimeOpcode::AddRegister => self.interpret_add_register()?,
                RuntimeOpcode::AddLiteral => self.interpret_add_literal()?,
                RuntimeOpcode::SubRegister => self.interpret_sub_register()?,
                RuntimeOpcode::SubLiteral => self.interpret_sub_literal()?,
                RuntimeOpcode::MovRegister => self.interpret_mov_register()?,
                RuntimeOpcode::MovLiteral => self.interpret_mov_literal()?,
                RuntimeOpcode::CmpRegister => self.interpret_cmp_register()?,
                RuntimeOpcode::CmpLiteral => self.interpret_cmp_literal()?,
                RuntimeOpcode::B => self.interpret_b()?,
                RuntimeOpcode::Beq => self.interpret_beq()?,
                RuntimeOpcode::Bne => self.interpret_bne()?,
                RuntimeOpcode::Bgt => self.interpret_bgt()?,
                RuntimeOpcode::Blt => self.interpret_blt()?,
                RuntimeOpcode::AndRegister => self.interpret_and_register()?,
                RuntimeOpcode::AndLiteral => self.interpret_and_literal()?,
                RuntimeOpcode::OrrRegister => self.interpret_orr_register()?,
                RuntimeOpcode::OrrLiteral => self.interpret_orr_literal()?,
                RuntimeOpcode::EorRegister => self.interpret_eor_register()?,
                RuntimeOpcode::EorLiteral => self.interpret_eor_literal()?,
                RuntimeOpcode::MvnRegister => self.interpret_mvn_register()?,
                RuntimeOpcode::MvnLiteral => self.interpret_mvn_literal()?,
                RuntimeOpcode::LslRegister => self.interpret_lsl_register()?,
                RuntimeOpcode::LslLiteral => self.interpret_lsl_literal()?,
                RuntimeOpcode::LsrRegister => self.interpret_lsr_register()?,
                RuntimeOpcode::LsrLiteral => self.interpret_lsr_literal()?,
                RuntimeOpcode::PrintRegister => self.interpret_print_register()?,
                RuntimeOpcode::PrintMemory => self.interpret_print_memory()?,
                RuntimeOpcode::InputRegister => self.interpret_input_register()?,
                RuntimeOpcode::InputMemory => self.interpret_input_memory()?,
                RuntimeOpcode::Halt => break,
            }
        }
        Ok(())
    }

    fn read_next_memory_address(&mut self) -> Result<u8, RuntimeError> {
        if self.program_counter >= self.program_bytes {
            return Err(RuntimeError::ReadPastMemory);
        }
        let result = self.memory[self.program_counter as usize];
        self.program_counter += 1;
        Ok(result)
    }

    fn read_memory_address(&self, idx: u8) -> Result<u8, RuntimeError> {
        let Some(new_address) = self.program_bytes.checked_add(idx) else {
            return Err(RuntimeError::OutOfBoundsRead(idx as usize));
        };
        Ok(self.memory[new_address as usize])
    }

    fn write_memory_address(&mut self, val: u8, idx: u8) -> Result<(), RuntimeError> {
        let Some(new_address) = self.program_bytes.checked_add(idx) else {
            return Err(RuntimeError::OutOfBoundsWrite(idx as usize));
        };
        self.memory[new_address as usize] = val;
        Ok(())
    }

    fn take_u8_input(&mut self) -> u8 {
        loop {
            let input = self.read_line();
            if let Ok(val) = input.trim().parse() {
                return val;
            }
        }
    }

    fn interpret_ldr(&mut self) -> Result<(), RuntimeError> {
        let register = self.read_next_memory_address()?;
        let memory_ref = self.read_next_memory_address()?;
        self.registers[register as usize] = self.read_memory_address(memory_ref)?;
        Ok(())
    }

    fn interpret_str(&mut self) -> Result<(), RuntimeError> {
        let register = self.read_next_memory_address()?;
        let memory_ref = self.read_next_memory_address()?;
        self.write_memory_address(self.registers[register as usize], memory_ref)?;
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
        let idx_to_branch_too = self.read_next_memory_address()?;
        self.program_counter = idx_to_branch_too;
        Ok(())
    }

    fn interpret_beq(&mut self) -> Result<(), RuntimeError> {
        if self.comparison_result == 0 {
            let idx_to_branch_too = self.read_next_memory_address()?;
            self.program_counter = idx_to_branch_too;
        } else {
            self.program_counter += 1;
        }
        Ok(())
    }

    fn interpret_bne(&mut self) -> Result<(), RuntimeError> {
        if self.comparison_result != 0 {
            let idx_to_branch_too = self.read_next_memory_address()?;
            self.program_counter = idx_to_branch_too;
        } else {
            self.program_counter += 1;
        }
        Ok(())
    }

    fn interpret_bgt(&mut self) -> Result<(), RuntimeError> {
        if self.comparison_result != 0 && !self.underflow {
            let idx_to_branch_too = self.read_next_memory_address()?;
            self.program_counter = idx_to_branch_too;
        } else {
            self.program_counter += 1;
        }
        Ok(())
    }

    fn interpret_blt(&mut self) -> Result<(), RuntimeError> {
        if self.underflow {
            let idx_to_branch_too = self.read_next_memory_address()?;
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
        self.registers[register_store] =
            register_operand_1.wrapping_shl(u32::from(register_operand_2));
        Ok(())
    }

    fn interpret_lsl_literal(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let literal_operand_2 = self.read_next_memory_address()?;
        self.registers[register_store] =
            register_operand_1.wrapping_shl(u32::from(literal_operand_2));
        Ok(())
    }

    fn interpret_lsr_register(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let register_operand_2 = self.registers[self.read_next_memory_address()? as usize];
        self.registers[register_store] =
            register_operand_1.wrapping_shr(u32::from(register_operand_2));
        Ok(())
    }

    fn interpret_lsr_literal(&mut self) -> Result<(), RuntimeError> {
        let register_store = self.read_next_memory_address()? as usize;
        let register_operand_1 = self.registers[self.read_next_memory_address()? as usize];
        let literal_operand_2 = self.read_next_memory_address()?;
        self.registers[register_store] =
            register_operand_1.wrapping_shr(u32::from(literal_operand_2));
        Ok(())
    }

    fn interpret_print_register(&mut self) -> Result<(), RuntimeError> {
        let register = self.read_next_memory_address()?;
        self.write_line(&format!("{}", self.registers[register as usize]));
        Ok(())
    }

    fn interpret_print_memory(&mut self) -> Result<(), RuntimeError> {
        let memory_ref = self.read_next_memory_address()?;
        self.write_line(&format!("{}", self.read_memory_address(memory_ref)?));
        Ok(())
    }

    fn interpret_input_register(&mut self) -> Result<(), RuntimeError> {
        let register = self.read_next_memory_address()?;
        self.registers[register as usize] = self.take_u8_input();
        Ok(())
    }

    fn interpret_input_memory(&mut self) -> Result<(), RuntimeError> {
        let memory_ref = self.read_next_memory_address()?;
        let value = self.take_u8_input();
        self.write_memory_address(value, memory_ref)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use instruction::runtime_opcode::RuntimeOpcode;
    use std::io::{self, BufReader, Cursor};

    use super::*;

    fn load_test_program(program: &[u8]) -> [u8; 256] {
        let mut memory = [0; 256];
        memory[..program.len()].copy_from_slice(program);
        memory
    }

    #[test]
    fn test_nop() {
        let program = [RuntimeOpcode::Nop as u8, RuntimeOpcode::Halt as u8];
        let mut memory = load_test_program(&program);
        let memory_copy = memory;
        let mut registers = [0; REGISTER_COUNT as usize];
        let registers_copy = registers;
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(memory, memory_copy);
        assert_eq!(registers, registers_copy);
    }

    #[test]
    fn test_ldr() {
        let program = [RuntimeOpcode::Ldr as u8, 0, 0, RuntimeOpcode::Halt as u8, 5];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8 - 1).unwrap();
        assert_eq!(registers[0], 5);
    }

    #[test]
    fn test_str() {
        let program = [RuntimeOpcode::Str as u8, 0, 0, RuntimeOpcode::Halt as u8];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 5;
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(memory[program.len()], 5);
    }

    #[test]
    fn test_add() {
        let program = [
            RuntimeOpcode::AddLiteral as u8,
            0,
            0,
            5,
            RuntimeOpcode::AddRegister as u8,
            1,
            1,
            0,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(registers[0], 5);
        assert_eq!(registers[1], 5);
    }

    #[test]
    fn test_sub() {
        let program = [
            RuntimeOpcode::SubLiteral as u8,
            0,
            0,
            5,
            RuntimeOpcode::SubRegister as u8,
            1,
            1,
            0,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 255;
        registers[1] = 255;
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(registers[0], 250);
        assert_eq!(registers[1], 5);
    }

    #[test]
    fn test_mov() {
        let program = [
            RuntimeOpcode::MovLiteral as u8,
            0,
            5,
            RuntimeOpcode::MovRegister as u8,
            1,
            0,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(registers[0], 5);
        assert_eq!(registers[1], 5);
    }

    #[test]
    fn test_cmp_equal_numbers() {
        // Comparison of register 0 and value 0
        let program = &[
            RuntimeOpcode::CmpLiteral as u8,
            0,
            0,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(interpreter.comparison_result, 0);
        assert!(!interpreter.underflow);

        // Comparison of contents of register 0 and 1 (both have values of 0)
        let program = &[
            RuntimeOpcode::CmpRegister as u8,
            0,
            1,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(program);
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(interpreter.comparison_result, 0);
        assert!(!interpreter.underflow);
    }

    #[test]
    fn test_cmp_greater_than() {
        // Comparison of register 0 (value of 5) and value 0
        let program = &[
            RuntimeOpcode::CmpLiteral as u8,
            0,
            0,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory: [u8; 256] = load_test_program(program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 5;
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(interpreter.comparison_result, 5);
        assert!(!interpreter.underflow);

        let program = &[
            RuntimeOpcode::CmpRegister as u8,
            0,
            1,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(program);
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(interpreter.comparison_result, 5);
        assert!(!interpreter.underflow);
    }

    #[test]
    fn test_cmp_less_than() {
        // Comparison of register 0 (value of 0) and value 5
        let program = &[
            RuntimeOpcode::CmpLiteral as u8,
            0,
            5,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory: [u8; 256] = load_test_program(program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[1] = 5;
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(interpreter.comparison_result, 251);
        assert!(interpreter.underflow);

        // Comaparison of register 0 (value of 0) and register 1 (value of 5)
        let program = &[
            RuntimeOpcode::CmpRegister as u8,
            0,
            1,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(program);
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(interpreter.comparison_result, 251);
        assert!(interpreter.underflow);
    }

    #[test]
    fn test_unconditional_branch() {
        let program = [
            RuntimeOpcode::B as u8,
            3,
            RuntimeOpcode::Halt as u8,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(interpreter.program_counter, 4);
    }

    #[test]
    fn test_beq() {
        // Successful BEQ
        let program = [
            RuntimeOpcode::Beq as u8,
            3,
            RuntimeOpcode::Halt as u8,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let mut interpreter = Interpreter {
            comparison_result: 0,
            program_bytes: program.len() as u8,
            memory: &mut memory,
            registers: &mut registers,
            program_counter: 0,
            underflow: false,
            reader: BufReader::new(io::stdin()),
            writer: io::stdout(),
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
            RuntimeOpcode::Bne as u8,
            3,
            RuntimeOpcode::Halt as u8,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let mut interpreter = Interpreter {
            comparison_result: 1,
            program_bytes: program.len() as u8,
            memory: &mut memory,
            registers: &mut registers,
            program_counter: 0,
            underflow: false,
            reader: BufReader::new(io::stdin()),
            writer: io::stdout(),
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
            RuntimeOpcode::Bgt as u8,
            3,
            RuntimeOpcode::Halt as u8,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let mut interpreter = Interpreter {
            comparison_result: 1,
            program_bytes: program.len() as u8,
            memory: &mut memory,
            registers: &mut registers,
            program_counter: 0,
            underflow: false,
            reader: BufReader::new(io::stdin()),
            writer: io::stdout(),
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
            RuntimeOpcode::Blt as u8,
            3,
            RuntimeOpcode::Halt as u8,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        let mut interpreter = Interpreter {
            comparison_result: 1,
            program_bytes: program.len() as u8,
            memory: &mut memory,
            registers: &mut registers,
            program_counter: 0,
            underflow: true,
            reader: BufReader::new(io::stdin()),
            writer: io::stdout(),
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
            RuntimeOpcode::AndLiteral as u8,
            0,
            0,
            0b11110000,
            RuntimeOpcode::AndRegister as u8,
            1,
            1,
            2,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b10101010;
        registers[1] = 0b10101010;
        registers[2] = 0b00001111;
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(registers[0], 0b10100000);
        assert_eq!(registers[1], 0b00001010);
    }

    #[test]
    fn test_orr() {
        let program = [
            RuntimeOpcode::OrrLiteral as u8,
            0,
            0,
            0b00001111,
            RuntimeOpcode::OrrRegister as u8,
            1,
            1,
            0,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b11001010;
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(registers[0], 0b11001111);
        assert_eq!(registers[1], 0b11001111);
    }

    #[test]
    fn test_eor() {
        let program = [
            RuntimeOpcode::EorLiteral as u8,
            0,
            0,
            0b00001111,
            RuntimeOpcode::EorRegister as u8,
            1,
            1,
            1,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b11001010;
        registers[1] = 0b11111111;
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(registers[0], 0b11000101);
        assert_eq!(registers[1], 0b00000000);
    }

    #[test]
    fn test_mvn() {
        let program = [
            RuntimeOpcode::MvnLiteral as u8,
            0,
            0b00001111,
            RuntimeOpcode::MvnRegister as u8,
            1,
            0,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b11111111;
        registers[1] = 0b11111111;
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(registers[0], 0b11110000);
        assert_eq!(registers[1], 0b00001111);
    }

    #[test]
    fn test_lsl() {
        let program = [
            RuntimeOpcode::LslLiteral as u8,
            0,
            0,
            4,
            RuntimeOpcode::LslRegister as u8,
            1,
            1,
            2,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b11000011;
        registers[1] = 0b11000011;
        registers[2] = 2;
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(registers[0], 0b00110000);
        assert_eq!(registers[1], 0b00001100);
    }

    #[test]
    fn test_lsr() {
        let program = [
            RuntimeOpcode::LsrLiteral as u8,
            0,
            0,
            4,
            RuntimeOpcode::LsrRegister as u8,
            1,
            1,
            2,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 0b11000011;
        registers[1] = 0b11000011;
        registers[2] = 2;
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(registers[0], 0b00001100);
        assert_eq!(registers[1], 0b00110000);
    }

    #[test]
    fn test_interpret_print_register() {
        // Setup
        let program = [
            RuntimeOpcode::PrintRegister as u8,
            0,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        registers[0] = 42;

        // Create a mock output writer
        let inputs = &[];
        let mut output: Vec<u8> = Vec::new();

        let reader = BufReader::new(Cursor::new(inputs));
        let writer = Cursor::new(&mut output);

        Interpreter::interpret_custom_io(
            &mut memory,
            &mut registers,
            program.len() as u8,
            reader,
            writer,
        )
        .unwrap();

        // Check
        let output_str = String::from_utf8(output).unwrap();
        assert_eq!(output_str.trim(), "42");
    }

    #[test]
    fn test_print_memory() {
        // Setup
        let program = [
            RuntimeOpcode::PrintMemory as u8,
            0,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        memory[program.len()] = 42;
        let mut registers = [0; REGISTER_COUNT as usize];

        // Create a mock output writer
        let inputs = &[];
        let mut output: Vec<u8> = Vec::new();

        let reader = BufReader::new(Cursor::new(inputs));
        let writer = Cursor::new(&mut output);

        Interpreter::interpret_custom_io(
            &mut memory,
            &mut registers,
            program.len() as u8,
            reader,
            writer,
        )
        .unwrap();

        // Check
        let output_str = String::from_utf8(output).unwrap();
        assert_eq!(output_str.trim(), "42");
    }

    #[test]
    fn test_interpret_input_register() {
        // Setup
        let program = [
            RuntimeOpcode::InputRegister as u8,
            0,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];

        let input = String::from("99");
        let inputs = input.as_bytes();
        let mut output: Vec<u8> = Vec::new();

        let reader = BufReader::new(Cursor::new(inputs));
        let writer = Cursor::new(&mut output);

        // Execute the interpreter
        Interpreter::interpret_custom_io(
            &mut memory,
            &mut registers,
            program.len() as u8,
            reader,
            writer,
        )
        .unwrap();

        // Check that the memory has the expected value
        assert_eq!(registers[0], 99); // Ensure memory at address 0 has the input value
    }

    #[test]
    fn test_interpret_input_memory() {
        // Setup
        let program = [
            RuntimeOpcode::InputMemory as u8,
            0,
            RuntimeOpcode::Halt as u8,
        ];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];

        let input = String::from("150");
        let inputs = input.as_bytes();
        let mut output: Vec<u8> = Vec::new();

        let reader = BufReader::new(Cursor::new(inputs));
        let writer = Cursor::new(&mut output);

        // Execute the interpreter
        Interpreter::interpret_custom_io(
            &mut memory,
            &mut registers,
            program.len() as u8,
            reader,
            writer,
        )
        .unwrap();

        // Check that the memory has the expected value
        assert_eq!(memory[program.len()], 150); // Ensure memory at address 0 has the input value
    }

    #[test]
    fn test_halt() {
        let program = [RuntimeOpcode::Halt as u8];
        let mut memory = load_test_program(&program);
        let memory_copy = memory;
        let mut registers = [0; REGISTER_COUNT as usize];
        let registers_copy = registers;
        let interpreter =
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap();
        assert_eq!(*interpreter.memory, memory_copy);
        assert_eq!(*interpreter.registers, registers_copy);
        assert_eq!(interpreter.program_counter, 1);
    }

    #[test]
    fn test_out_of_bounds_read() {
        let program = [RuntimeOpcode::Ldr as u8, 0, 253];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        assert_eq!(
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap_err(),
            RuntimeError::OutOfBoundsRead(253)
        );
    }

    #[test]
    fn test_out_of_bounds_write() {
        let program = [RuntimeOpcode::Str as u8, 0, 253];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        assert_eq!(
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap_err(),
            RuntimeError::OutOfBoundsWrite(253)
        );
    }

    #[test]
    fn test_read_past_of_memory() {
        let program = [];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        assert_eq!(
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap_err(),
            RuntimeError::ReadPastMemory
        );
    }

    #[test]
    fn test_read_past_max_memory() {
        let program = [RuntimeOpcode::Nop as u8; 256];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        assert_eq!(
            Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap_err(),
            RuntimeError::ReadPastMemory
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let program = [45];
        let mut memory = load_test_program(&program);
        let mut registers = [0; REGISTER_COUNT as usize];
        Interpreter::interpret(&mut memory, &mut registers, program.len() as u8).unwrap_err();
    }
}
