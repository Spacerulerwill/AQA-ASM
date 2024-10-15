use core::fmt;

use super::signature::SignatureArgument;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Operand {
    Literal(u8),
    Register(u8),
    MemoryRef(u8),
    Label,
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Literal(val) => write!(f, "#{val} (Literal)"),
            Operand::Register(val) => write!(f, "R{val} (Register)"),
            Operand::MemoryRef(val) => write!(f, "{val} (Memory Reference)"),
            Operand::Label => write!(f, "Label"),
        }
    }
}

impl Operand {
    pub fn get_signature_argument(&self) -> SignatureArgument {
        match self {
            Operand::Literal(_) => SignatureArgument::Literal,
            Operand::Register(_) => SignatureArgument::Register,
            Operand::MemoryRef(_) => SignatureArgument::MemoryRef,
            Operand::Label => SignatureArgument::Label,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operand_display() {
        let test_cases = [
            (Operand::Literal(42), "#42 (Literal)"),
            (Operand::Register(1), "R1 (Register)"),
            (Operand::MemoryRef(3), "3 (Memory Reference)"),
            (Operand::Label, "Label"),
        ];

        for (operand, expected) in test_cases.iter() {
            assert_eq!(format!("{}", operand), *expected);
        }
    }

    #[test]
    fn test_operand_get_signature_argument() {
        let test_cases = [
            (Operand::Literal(0), SignatureArgument::Literal),
            (Operand::Register(1), SignatureArgument::Register),
            (Operand::MemoryRef(2), SignatureArgument::MemoryRef),
            (Operand::Label, SignatureArgument::Label),
        ];

        for (operand, expected) in test_cases.iter() {
            assert_eq!(operand.get_signature_argument(), *expected);
        }
    }
}
