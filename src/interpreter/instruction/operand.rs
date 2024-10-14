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
            Operand::MemoryRef(val) => write!(f, "{val} (Memory Reference"),
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
