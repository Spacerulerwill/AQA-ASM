/// Runtime opcodes are the opcodes actually interpreted by the interpreter.
/// These are different to source opcodes as each source opcode can be mapped to
/// multiple different instructions based on the combination of its arguments.    
/// For example:    
/// MOV R5, R5 => `MOV_REGISTER` 5 5    
/// MOP R5, #5 => `MOV_LITERAL` 5 5    
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RuntimeOpcode {
    Nop,
    Ldr,
    Str,
    AddRegister,
    AddLiteral,
    SubRegister,
    SubLiteral,
    MovRegister,
    MovLiteral,
    CmpRegister,
    CmpLiteral,
    B,
    Beq,
    Bne,
    Bgt,
    Blt,
    AndRegister,
    AndLiteral,
    OrrRegister,
    OrrLiteral,
    EorRegister,
    EorLiteral,
    MvnRegister,
    MvnLiteral,
    LslRegister,
    LslLiteral,
    LsrRegister,
    LsrLiteral,
    PrintRegister,
    PrintMemory,
    InputRegister,
    InputMemory,
    Halt,
}

impl TryFrom<u8> for RuntimeOpcode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == RuntimeOpcode::Nop as u8 => Ok(RuntimeOpcode::Nop),
            x if x == RuntimeOpcode::Ldr as u8 => Ok(RuntimeOpcode::Ldr),
            x if x == RuntimeOpcode::Str as u8 => Ok(RuntimeOpcode::Str),
            x if x == RuntimeOpcode::AddRegister as u8 => Ok(RuntimeOpcode::AddRegister),
            x if x == RuntimeOpcode::AddLiteral as u8 => Ok(RuntimeOpcode::AddLiteral),
            x if x == RuntimeOpcode::SubRegister as u8 => Ok(RuntimeOpcode::SubRegister),
            x if x == RuntimeOpcode::SubLiteral as u8 => Ok(RuntimeOpcode::SubLiteral),
            x if x == RuntimeOpcode::MovRegister as u8 => Ok(RuntimeOpcode::MovRegister),
            x if x == RuntimeOpcode::MovLiteral as u8 => Ok(RuntimeOpcode::MovLiteral),
            x if x == RuntimeOpcode::CmpRegister as u8 => Ok(RuntimeOpcode::CmpRegister),
            x if x == RuntimeOpcode::CmpLiteral as u8 => Ok(RuntimeOpcode::CmpLiteral),
            x if x == RuntimeOpcode::B as u8 => Ok(RuntimeOpcode::B),
            x if x == RuntimeOpcode::Beq as u8 => Ok(RuntimeOpcode::Beq),
            x if x == RuntimeOpcode::Bne as u8 => Ok(RuntimeOpcode::Bne),
            x if x == RuntimeOpcode::Bgt as u8 => Ok(RuntimeOpcode::Bgt),
            x if x == RuntimeOpcode::Blt as u8 => Ok(RuntimeOpcode::Blt),
            x if x == RuntimeOpcode::AndRegister as u8 => Ok(RuntimeOpcode::AndRegister),
            x if x == RuntimeOpcode::AndLiteral as u8 => Ok(RuntimeOpcode::AndLiteral),
            x if x == RuntimeOpcode::OrrRegister as u8 => Ok(RuntimeOpcode::OrrRegister),
            x if x == RuntimeOpcode::OrrLiteral as u8 => Ok(RuntimeOpcode::OrrLiteral),
            x if x == RuntimeOpcode::EorRegister as u8 => Ok(RuntimeOpcode::EorRegister),
            x if x == RuntimeOpcode::EorLiteral as u8 => Ok(RuntimeOpcode::EorLiteral),
            x if x == RuntimeOpcode::MvnRegister as u8 => Ok(RuntimeOpcode::MvnRegister),
            x if x == RuntimeOpcode::MvnLiteral as u8 => Ok(RuntimeOpcode::MvnLiteral),
            x if x == RuntimeOpcode::LslRegister as u8 => Ok(RuntimeOpcode::LslRegister),
            x if x == RuntimeOpcode::LslLiteral as u8 => Ok(RuntimeOpcode::LslLiteral),
            x if x == RuntimeOpcode::LsrRegister as u8 => Ok(RuntimeOpcode::LsrRegister),
            x if x == RuntimeOpcode::LsrLiteral as u8 => Ok(RuntimeOpcode::LsrLiteral),
            x if x == RuntimeOpcode::PrintRegister as u8 => Ok(RuntimeOpcode::PrintRegister),
            x if x == RuntimeOpcode::PrintMemory as u8 => Ok(RuntimeOpcode::PrintMemory),
            x if x == RuntimeOpcode::InputRegister as u8 => Ok(RuntimeOpcode::InputRegister),
            x if x == RuntimeOpcode::InputMemory as u8 => Ok(RuntimeOpcode::InputMemory),
            x if x == RuntimeOpcode::Halt as u8 => Ok(RuntimeOpcode::Halt),
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
            (0, Ok(RuntimeOpcode::Nop)),
            (1, Ok(RuntimeOpcode::Ldr)),
            (2, Ok(RuntimeOpcode::Str)),
            (3, Ok(RuntimeOpcode::AddRegister)),
            (4, Ok(RuntimeOpcode::AddLiteral)),
            (5, Ok(RuntimeOpcode::SubRegister)),
            (6, Ok(RuntimeOpcode::SubLiteral)),
            (7, Ok(RuntimeOpcode::MovRegister)),
            (8, Ok(RuntimeOpcode::MovLiteral)),
            (9, Ok(RuntimeOpcode::CmpRegister)),
            (10, Ok(RuntimeOpcode::CmpLiteral)),
            (11, Ok(RuntimeOpcode::B)),
            (12, Ok(RuntimeOpcode::Beq)),
            (13, Ok(RuntimeOpcode::Bne)),
            (14, Ok(RuntimeOpcode::Bgt)),
            (15, Ok(RuntimeOpcode::Blt)),
            (16, Ok(RuntimeOpcode::AndRegister)),
            (17, Ok(RuntimeOpcode::AndLiteral)),
            (18, Ok(RuntimeOpcode::OrrRegister)),
            (19, Ok(RuntimeOpcode::OrrLiteral)),
            (20, Ok(RuntimeOpcode::EorRegister)),
            (21, Ok(RuntimeOpcode::EorLiteral)),
            (22, Ok(RuntimeOpcode::MvnRegister)),
            (23, Ok(RuntimeOpcode::MvnLiteral)),
            (24, Ok(RuntimeOpcode::LslRegister)),
            (25, Ok(RuntimeOpcode::LslLiteral)),
            (26, Ok(RuntimeOpcode::LsrRegister)),
            (27, Ok(RuntimeOpcode::LsrLiteral)),
            (28, Ok(RuntimeOpcode::PrintRegister)),
            (29, Ok(RuntimeOpcode::PrintMemory)),
            (30, Ok(RuntimeOpcode::InputRegister)),
            (31, Ok(RuntimeOpcode::InputMemory)),
            (32, Ok(RuntimeOpcode::Halt)),
        ] {
            assert_eq!(RuntimeOpcode::try_from(input), expected);
        }

        for input in 33..=255 {
            assert_eq!(RuntimeOpcode::try_from(input), Err(()));
        }
    }
}
