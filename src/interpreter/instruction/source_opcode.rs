use std::{fmt, str::FromStr};

/// Source opcodes are the opcode literals found in source files
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceOpcode {
    Nop,
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
    Print,
    Input,
    Halt,
}

impl fmt::Display for SourceOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceOpcode::Nop => write!(f, "NOP"),
            SourceOpcode::Ldr => write!(f, "LDR"),
            SourceOpcode::Str => write!(f, "STR"),
            SourceOpcode::Add => write!(f, "ADD"),
            SourceOpcode::Sub => write!(f, "SUB"),
            SourceOpcode::Mov => write!(f, "MOV"),
            SourceOpcode::Cmp => write!(f, "CMP"),
            SourceOpcode::B => write!(f, "B"),
            SourceOpcode::Beq => write!(f, "BEQ"),
            SourceOpcode::Bne => write!(f, "BNE"),
            SourceOpcode::Bgt => write!(f, "BGT"),
            SourceOpcode::Blt => write!(f, "BLT"),
            SourceOpcode::And => write!(f, "AND"),
            SourceOpcode::Orr => write!(f, "ORR"),
            SourceOpcode::Eor => write!(f, "EOR"),
            SourceOpcode::Mvn => write!(f, "MVN"),
            SourceOpcode::Lsl => write!(f, "LSL"),
            SourceOpcode::Lsr => write!(f, "LSR"),
            SourceOpcode::Print => write!(f, "PRINT"),
            SourceOpcode::Input => write!(f, "INPUT"),
            SourceOpcode::Halt => write!(f, "HALT"),
        }
    }
}

impl FromStr for SourceOpcode {
    type Err = ();
    fn from_str(input: &str) -> Result<SourceOpcode, Self::Err> {
        match input {
            "NOP" => Ok(SourceOpcode::Nop),
            "LDR" => Ok(SourceOpcode::Ldr),
            "STR" => Ok(SourceOpcode::Str),
            "ADD" => Ok(SourceOpcode::Add),
            "SUB" => Ok(SourceOpcode::Sub),
            "MOV" => Ok(SourceOpcode::Mov),
            "CMP" => Ok(SourceOpcode::Cmp),
            "B" => Ok(SourceOpcode::B),
            "BEQ" => Ok(SourceOpcode::Beq),
            "BNE" => Ok(SourceOpcode::Bne),
            "BGT" => Ok(SourceOpcode::Bgt),
            "BLT" => Ok(SourceOpcode::Blt),
            "AND" => Ok(SourceOpcode::And),
            "ORR" => Ok(SourceOpcode::Orr),
            "EOR" => Ok(SourceOpcode::Eor),
            "MVN" => Ok(SourceOpcode::Mvn),
            "LSL" => Ok(SourceOpcode::Lsl),
            "LSR" => Ok(SourceOpcode::Lsr),
            "HALT" => Ok(SourceOpcode::Halt),
            "PRINT" => Ok(SourceOpcode::Print),
            "INPUT" => Ok(SourceOpcode::Input),
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
            ("NOP", Ok(SourceOpcode::Nop)),
            ("LDR", Ok(SourceOpcode::Ldr)),
            ("STR", Ok(SourceOpcode::Str)),
            ("ADD", Ok(SourceOpcode::Add)),
            ("SUB", Ok(SourceOpcode::Sub)),
            ("MOV", Ok(SourceOpcode::Mov)),
            ("CMP", Ok(SourceOpcode::Cmp)),
            ("B", Ok(SourceOpcode::B)),
            ("BEQ", Ok(SourceOpcode::Beq)),
            ("BNE", Ok(SourceOpcode::Bne)),
            ("BGT", Ok(SourceOpcode::Bgt)),
            ("BLT", Ok(SourceOpcode::Blt)),
            ("AND", Ok(SourceOpcode::And)),
            ("ORR", Ok(SourceOpcode::Orr)),
            ("EOR", Ok(SourceOpcode::Eor)),
            ("MVN", Ok(SourceOpcode::Mvn)),
            ("LSL", Ok(SourceOpcode::Lsl)),
            ("LSR", Ok(SourceOpcode::Lsr)),
            ("HALT", Ok(SourceOpcode::Halt)),
            ("PRINT", Ok(SourceOpcode::Print)),
            ("INPUT", Ok(SourceOpcode::Input)),
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

    #[test]
    fn test_source_opcode_display() {
        let test_cases = [
            (SourceOpcode::Nop, "NOP"),
            (SourceOpcode::Ldr, "LDR"),
            (SourceOpcode::Str, "STR"),
            (SourceOpcode::Add, "ADD"),
            (SourceOpcode::Sub, "SUB"),
            (SourceOpcode::Mov, "MOV"),
            (SourceOpcode::Cmp, "CMP"),
            (SourceOpcode::B, "B"),
            (SourceOpcode::Beq, "BEQ"),
            (SourceOpcode::Bne, "BNE"),
            (SourceOpcode::Bgt, "BGT"),
            (SourceOpcode::Blt, "BLT"),
            (SourceOpcode::And, "AND"),
            (SourceOpcode::Orr, "ORR"),
            (SourceOpcode::Eor, "EOR"),
            (SourceOpcode::Mvn, "MVN"),
            (SourceOpcode::Lsl, "LSL"),
            (SourceOpcode::Lsr, "LSR"),
            (SourceOpcode::Print, "PRINT"),
            (SourceOpcode::Input, "INPUT"),
            (SourceOpcode::Halt, "HALT"),
        ];

        for (opcode, expected) in &test_cases {
            assert_eq!(format!("{opcode}"), *expected);
        }
    }
}
