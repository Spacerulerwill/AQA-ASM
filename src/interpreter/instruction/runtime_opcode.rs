/// Runtime opcodes are the opcodes actually interpreted by the interpreter.
/// These are different to source opcodes as each source opcode can be mapped to
/// multiple different instructions based on the combination of its arguments.    
/// For example:    
/// MOV R5, R5 => MOV_REGISTER 5 5    
/// MOP R5, #5 => MOV_LITERAL 5 5    
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RuntimeOpcode {
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

impl TryFrom<u8> for RuntimeOpcode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == RuntimeOpcode::NOP as u8 => Ok(RuntimeOpcode::NOP),
            x if x == RuntimeOpcode::LDR as u8 => Ok(RuntimeOpcode::LDR),
            x if x == RuntimeOpcode::STR as u8 => Ok(RuntimeOpcode::STR),
            x if x == RuntimeOpcode::ADD_REGISTER as u8 => Ok(RuntimeOpcode::ADD_REGISTER),
            x if x == RuntimeOpcode::ADD_LITERAL as u8 => Ok(RuntimeOpcode::ADD_LITERAL),
            x if x == RuntimeOpcode::SUB_REGISTER as u8 => Ok(RuntimeOpcode::SUB_REGISTER),
            x if x == RuntimeOpcode::SUB_LITERAL as u8 => Ok(RuntimeOpcode::SUB_LITERAL),
            x if x == RuntimeOpcode::MOV_REGISTER as u8 => Ok(RuntimeOpcode::MOV_REGISTER),
            x if x == RuntimeOpcode::MOV_LITERAL as u8 => Ok(RuntimeOpcode::MOV_LITERAL),
            x if x == RuntimeOpcode::CMP_REGISTER as u8 => Ok(RuntimeOpcode::CMP_REGISTER),
            x if x == RuntimeOpcode::CMP_LITERAL as u8 => Ok(RuntimeOpcode::CMP_LITERAL),
            x if x == RuntimeOpcode::B as u8 => Ok(RuntimeOpcode::B),
            x if x == RuntimeOpcode::BEQ as u8 => Ok(RuntimeOpcode::BEQ),
            x if x == RuntimeOpcode::BNE as u8 => Ok(RuntimeOpcode::BNE),
            x if x == RuntimeOpcode::BGT as u8 => Ok(RuntimeOpcode::BGT),
            x if x == RuntimeOpcode::BLT as u8 => Ok(RuntimeOpcode::BLT),
            x if x == RuntimeOpcode::AND_REGISTER as u8 => Ok(RuntimeOpcode::AND_REGISTER),
            x if x == RuntimeOpcode::AND_LITERAL as u8 => Ok(RuntimeOpcode::AND_LITERAL),
            x if x == RuntimeOpcode::ORR_REGISTER as u8 => Ok(RuntimeOpcode::ORR_REGISTER),
            x if x == RuntimeOpcode::ORR_LITERAL as u8 => Ok(RuntimeOpcode::ORR_LITERAL),
            x if x == RuntimeOpcode::EOR_REGISTER as u8 => Ok(RuntimeOpcode::EOR_REGISTER),
            x if x == RuntimeOpcode::EOR_LITERAL as u8 => Ok(RuntimeOpcode::EOR_LITERAL),
            x if x == RuntimeOpcode::MVN_REGISTER as u8 => Ok(RuntimeOpcode::MVN_REGISTER),
            x if x == RuntimeOpcode::MVN_LITERAL as u8 => Ok(RuntimeOpcode::MVN_LITERAL),
            x if x == RuntimeOpcode::LSL_REGISTER as u8 => Ok(RuntimeOpcode::LSL_REGISTER),
            x if x == RuntimeOpcode::LSL_LITERAL as u8 => Ok(RuntimeOpcode::LSL_LITERAL),
            x if x == RuntimeOpcode::LSR_REGISTER as u8 => Ok(RuntimeOpcode::LSR_REGISTER),
            x if x == RuntimeOpcode::LSR_LITERAL as u8 => Ok(RuntimeOpcode::LSR_LITERAL),
            x if x == RuntimeOpcode::PRINT_REGISTER as u8 => Ok(RuntimeOpcode::PRINT_REGISTER),
            x if x == RuntimeOpcode::PRINT_MEMORY as u8 => Ok(RuntimeOpcode::PRINT_MEMORY),
            x if x == RuntimeOpcode::INPUT_REGISTER as u8 => Ok(RuntimeOpcode::INPUT_REGISTER),
            x if x == RuntimeOpcode::INPUT_MEMORY as u8 => Ok(RuntimeOpcode::INPUT_MEMORY),
            x if x == RuntimeOpcode::HALT as u8 => Ok(RuntimeOpcode::HALT),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeOpcode;

    #[test]
    fn test_runtime_opcode_from_u8() {
        for (input, expected) in [
            (0, Ok(RuntimeOpcode::NOP)),
            (1, Ok(RuntimeOpcode::LDR)),
            (2, Ok(RuntimeOpcode::STR)),
            (3, Ok(RuntimeOpcode::ADD_REGISTER)),
            (4, Ok(RuntimeOpcode::ADD_LITERAL)),
            (5, Ok(RuntimeOpcode::SUB_REGISTER)),
            (6, Ok(RuntimeOpcode::SUB_LITERAL)),
            (7, Ok(RuntimeOpcode::MOV_REGISTER)),
            (8, Ok(RuntimeOpcode::MOV_LITERAL)),
            (9, Ok(RuntimeOpcode::CMP_REGISTER)),
            (10, Ok(RuntimeOpcode::CMP_LITERAL)),
            (11, Ok(RuntimeOpcode::B)),
            (12, Ok(RuntimeOpcode::BEQ)),
            (13, Ok(RuntimeOpcode::BNE)),
            (14, Ok(RuntimeOpcode::BGT)),
            (15, Ok(RuntimeOpcode::BLT)),
            (16, Ok(RuntimeOpcode::AND_REGISTER)),
            (17, Ok(RuntimeOpcode::AND_LITERAL)),
            (18, Ok(RuntimeOpcode::ORR_REGISTER)),
            (19, Ok(RuntimeOpcode::ORR_LITERAL)),
            (20, Ok(RuntimeOpcode::EOR_REGISTER)),
            (21, Ok(RuntimeOpcode::EOR_LITERAL)),
            (22, Ok(RuntimeOpcode::MVN_REGISTER)),
            (23, Ok(RuntimeOpcode::MVN_LITERAL)),
            (24, Ok(RuntimeOpcode::LSL_REGISTER)),
            (25, Ok(RuntimeOpcode::LSL_LITERAL)),
            (26, Ok(RuntimeOpcode::LSR_REGISTER)),
            (27, Ok(RuntimeOpcode::LSR_LITERAL)),
            (28, Ok(RuntimeOpcode::PRINT_REGISTER)),
            (29, Ok(RuntimeOpcode::PRINT_MEMORY)),
            (30, Ok(RuntimeOpcode::INPUT_REGISTER)),
            (31, Ok(RuntimeOpcode::INPUT_MEMORY)),
            (32, Ok(RuntimeOpcode::HALT)),
        ] {
            assert_eq!(RuntimeOpcode::try_from(input), expected);
        }

        for input in 33..=255 {
            assert_eq!(RuntimeOpcode::try_from(input), Err(()));
        }
    }
}
