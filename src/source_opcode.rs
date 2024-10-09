use super::OperandType;
use crate::runtime_opcode::RuntimeOpcode;
use std::str::FromStr;
use strum::EnumIter;

/// Source opcodes are the opcode literals found in source files
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum SourceOpcode {
    NOP,
    LDR,
    STR,
    ADD,
    SUB,
    MOV,
    CMP,
    B,
    BEQ,
    BNE,
    BGT,
    BLT,
    AND,
    ORR,
    EOR,
    MVN,
    LSL,
    LSR,
    PRINT,
    INPUT,
    HALT,
}

impl SourceOpcode {
    pub fn got_operand_formats(&self) -> Vec<(RuntimeOpcode, Vec<OperandType>)> {
        match self {
            SourceOpcode::NOP => vec![(RuntimeOpcode::NOP, vec![])],
            SourceOpcode::LDR => vec![(
                RuntimeOpcode::LDR,
                vec![OperandType::Register, OperandType::MemoryRef],
            )],
            SourceOpcode::STR => vec![(
                RuntimeOpcode::STR,
                vec![OperandType::Register, OperandType::MemoryRef],
            )],
            SourceOpcode::ADD => vec![
                (
                    RuntimeOpcode::ADD_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    RuntimeOpcode::ADD_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            SourceOpcode::SUB => vec![
                (
                    RuntimeOpcode::SUB_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    RuntimeOpcode::SUB_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            SourceOpcode::MOV => vec![
                (
                    RuntimeOpcode::MOV_LITERAL,
                    vec![OperandType::Register, OperandType::Literal],
                ),
                (
                    RuntimeOpcode::MOV_REGISTER,
                    vec![OperandType::Register, OperandType::Register],
                ),
            ],
            SourceOpcode::CMP => vec![
                (
                    RuntimeOpcode::CMP_LITERAL,
                    vec![OperandType::Register, OperandType::Literal],
                ),
                (
                    RuntimeOpcode::CMP_REGISTER,
                    vec![OperandType::Register, OperandType::Register],
                ),
            ],
            SourceOpcode::B => vec![(RuntimeOpcode::B, vec![OperandType::Label])],
            SourceOpcode::BEQ => vec![(RuntimeOpcode::BEQ, vec![OperandType::Label])],
            SourceOpcode::BNE => vec![(RuntimeOpcode::BNE, vec![OperandType::Label])],
            SourceOpcode::BGT => vec![(RuntimeOpcode::BGT, vec![OperandType::Label])],
            SourceOpcode::BLT => vec![(RuntimeOpcode::BLT, vec![OperandType::Label])],
            SourceOpcode::AND => vec![
                (
                    RuntimeOpcode::AND_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    RuntimeOpcode::AND_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            SourceOpcode::ORR => vec![
                (
                    RuntimeOpcode::ORR_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    RuntimeOpcode::ORR_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            SourceOpcode::EOR => vec![
                (
                    RuntimeOpcode::EOR_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    RuntimeOpcode::EOR_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            SourceOpcode::MVN => vec![
                (
                    RuntimeOpcode::MVN_LITERAL,
                    vec![OperandType::Register, OperandType::Literal],
                ),
                (
                    RuntimeOpcode::MVN_REGISTER,
                    vec![OperandType::Register, OperandType::Register],
                ),
            ],
            SourceOpcode::LSL => vec![
                (
                    RuntimeOpcode::LSL_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    RuntimeOpcode::LSL_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            SourceOpcode::LSR => vec![
                (
                    RuntimeOpcode::LSR_LITERAL,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Literal,
                    ],
                ),
                (
                    RuntimeOpcode::LSR_REGISTER,
                    vec![
                        OperandType::Register,
                        OperandType::Register,
                        OperandType::Register,
                    ],
                ),
            ],
            SourceOpcode::HALT => vec![(RuntimeOpcode::HALT, vec![])],
            SourceOpcode::PRINT => vec![
                (RuntimeOpcode::PRINT_REGISTER, vec![OperandType::Register]),
                (RuntimeOpcode::PRINT_MEMORY, vec![OperandType::MemoryRef]),
            ],
            SourceOpcode::INPUT => vec![
                (RuntimeOpcode::INPUT_REGISTER, vec![OperandType::Register]),
                (RuntimeOpcode::INPUT_MEMORY, vec![OperandType::MemoryRef]),
            ],
        }
    }
}

impl FromStr for SourceOpcode {
    type Err = ();
    fn from_str(input: &str) -> Result<SourceOpcode, Self::Err> {
        match input {
            "NOP" => Ok(SourceOpcode::NOP),
            "LDR" => Ok(SourceOpcode::LDR),
            "STR" => Ok(SourceOpcode::STR),
            "ADD" => Ok(SourceOpcode::ADD),
            "SUB" => Ok(SourceOpcode::SUB),
            "MOV" => Ok(SourceOpcode::MOV),
            "CMP" => Ok(SourceOpcode::CMP),
            "B" => Ok(SourceOpcode::B),
            "BEQ" => Ok(SourceOpcode::BEQ),
            "BNE" => Ok(SourceOpcode::BNE),
            "BGT" => Ok(SourceOpcode::BGT),
            "BLT" => Ok(SourceOpcode::BLT),
            "AND" => Ok(SourceOpcode::AND),
            "ORR" => Ok(SourceOpcode::ORR),
            "EOR" => Ok(SourceOpcode::EOR),
            "MVN" => Ok(SourceOpcode::MVN),
            "LSL" => Ok(SourceOpcode::LSL),
            "LSR" => Ok(SourceOpcode::LSR),
            "HALT" => Ok(SourceOpcode::HALT),
            "PRINT" => Ok(SourceOpcode::PRINT),
            "INPUT" => Ok(SourceOpcode::INPUT),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::SourceOpcode;

    #[test]
    fn test_source_opcode_from_str() {
        for (input, expected) in [
            // Ensure all source opcodes convert from string
            ("NOP", Ok(SourceOpcode::NOP)),
            ("LDR", Ok(SourceOpcode::LDR)),
            ("STR", Ok(SourceOpcode::STR)),
            ("ADD", Ok(SourceOpcode::ADD)),
            ("SUB", Ok(SourceOpcode::SUB)),
            ("MOV", Ok(SourceOpcode::MOV)),
            ("CMP", Ok(SourceOpcode::CMP)),
            ("B", Ok(SourceOpcode::B)),
            ("BEQ", Ok(SourceOpcode::BEQ)),
            ("BNE", Ok(SourceOpcode::BNE)),
            ("BGT", Ok(SourceOpcode::BGT)),
            ("BLT", Ok(SourceOpcode::BLT)),
            ("AND", Ok(SourceOpcode::AND)),
            ("ORR", Ok(SourceOpcode::ORR)),
            ("EOR", Ok(SourceOpcode::EOR)),
            ("MVN", Ok(SourceOpcode::MVN)),
            ("LSL", Ok(SourceOpcode::LSL)),
            ("LSR", Ok(SourceOpcode::LSR)),
            ("HALT", Ok(SourceOpcode::HALT)),
            ("PRINT", Ok(SourceOpcode::PRINT)),
            ("INPUT", Ok(SourceOpcode::INPUT)),
            // lowercase commands shouldn't work
            ("nop", Err(())),
            ("input", Err(())),
            // whitespace should be important
            ("A N D", Err(())),
            ("AND  ", Err(())),
            ("  AND", Err(())),
            ("  AND  ", Err(())),
            // random words shouldn't work
            ("foo", Err(())),
            ("bar", Err(())),
        ] {
            assert_eq!(SourceOpcode::from_str(input), expected);
        }
    }

}