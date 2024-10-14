use std::{fmt, str::FromStr};

/// Source opcodes are the opcode literals found in source files
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl fmt::Display for SourceOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceOpcode::NOP => write!(f, "NOP"),
            SourceOpcode::LDR => write!(f, "LDR"),
            SourceOpcode::STR => write!(f, "STR"),
            SourceOpcode::ADD => write!(f, "ADD"),
            SourceOpcode::SUB => write!(f, "SUB"),
            SourceOpcode::MOV => write!(f, "MOV"),
            SourceOpcode::CMP => write!(f, "CMP"),
            SourceOpcode::B => write!(f, "B"),
            SourceOpcode::BEQ => write!(f, "BEQ"),
            SourceOpcode::BNE => write!(f, "BNE"),
            SourceOpcode::BGT => write!(f, "BGT"),
            SourceOpcode::BLT => write!(f, "BLT"),
            SourceOpcode::AND => write!(f, "AND"),
            SourceOpcode::ORR => write!(f, "ORR"),
            SourceOpcode::EOR => write!(f, "EOR"),
            SourceOpcode::MVN => write!(f, "MVN"),
            SourceOpcode::LSL => write!(f, "LSL"),
            SourceOpcode::LSR => write!(f, "LSR"),
            SourceOpcode::PRINT => write!(f, "PRINT"),
            SourceOpcode::INPUT => write!(f, "INPUT"),
            SourceOpcode::HALT => write!(f, "HALT"),
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
