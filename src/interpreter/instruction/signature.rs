use std::{collections::HashMap, fmt};

use once_cell::sync::Lazy;

use super::{operand::Operand, runtime_opcode::RuntimeOpcode, source_opcode::SourceOpcode};

pub static SIGNATURE_TREE: Lazy<SignatureTree> = Lazy::new(|| {
    let mut tree = SignatureTree::new();

    tree.add_signature(SourceOpcode::Nop, &[], RuntimeOpcode::Nop);
    tree.add_signature(
        SourceOpcode::Ldr,
        &[SignatureArgument::Register, SignatureArgument::MemoryRef],
        RuntimeOpcode::Ldr,
    );
    tree.add_signature(
        SourceOpcode::Str,
        &[SignatureArgument::Register, SignatureArgument::MemoryRef],
        RuntimeOpcode::Str,
    );
    tree.add_signature(
        SourceOpcode::Add,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::AddLiteral,
    );
    tree.add_signature(
        SourceOpcode::Add,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::AddRegister,
    );
    tree.add_signature(
        SourceOpcode::Sub,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::SubLiteral,
    );
    tree.add_signature(
        SourceOpcode::Sub,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::SubRegister,
    );
    tree.add_signature(
        SourceOpcode::Mov,
        &[SignatureArgument::Register, SignatureArgument::Literal],
        RuntimeOpcode::MovLiteral,
    );
    tree.add_signature(
        SourceOpcode::Mov,
        &[SignatureArgument::Register, SignatureArgument::Register],
        RuntimeOpcode::MovRegister,
    );
    tree.add_signature(
        SourceOpcode::Cmp,
        &[SignatureArgument::Register, SignatureArgument::Literal],
        RuntimeOpcode::CmpLiteral,
    );
    tree.add_signature(
        SourceOpcode::Cmp,
        &[SignatureArgument::Register, SignatureArgument::Register],
        RuntimeOpcode::CmpRegister,
    );
    tree.add_signature(
        SourceOpcode::B,
        &[SignatureArgument::Label],
        RuntimeOpcode::B,
    );
    tree.add_signature(
        SourceOpcode::Beq,
        &[SignatureArgument::Label],
        RuntimeOpcode::Beq,
    );
    tree.add_signature(
        SourceOpcode::Bne,
        &[SignatureArgument::Label],
        RuntimeOpcode::Bne,
    );
    tree.add_signature(
        SourceOpcode::Bgt,
        &[SignatureArgument::Label],
        RuntimeOpcode::Bgt,
    );
    tree.add_signature(
        SourceOpcode::Blt,
        &[SignatureArgument::Label],
        RuntimeOpcode::Blt,
    );
    tree.add_signature(
        SourceOpcode::And,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::AndLiteral,
    );
    tree.add_signature(
        SourceOpcode::And,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::AndRegister,
    );
    tree.add_signature(
        SourceOpcode::Orr,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::OrrLiteral,
    );
    tree.add_signature(
        SourceOpcode::Orr,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::OrrRegister,
    );
    tree.add_signature(
        SourceOpcode::Eor,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::EorLiteral,
    );
    tree.add_signature(
        SourceOpcode::Eor,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::EorRegister,
    );
    tree.add_signature(
        SourceOpcode::Mvn,
        &[SignatureArgument::Register, SignatureArgument::Literal],
        RuntimeOpcode::MvnLiteral,
    );
    tree.add_signature(
        SourceOpcode::Mvn,
        &[SignatureArgument::Register, SignatureArgument::Register],
        RuntimeOpcode::MvnRegister,
    );
    tree.add_signature(
        SourceOpcode::Lsl,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::LslLiteral,
    );
    tree.add_signature(
        SourceOpcode::Lsl,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::LslRegister,
    );
    tree.add_signature(
        SourceOpcode::Lsr,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::LsrLiteral,
    );
    tree.add_signature(
        SourceOpcode::Lsr,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::LsrRegister,
    );
    tree.add_signature(SourceOpcode::Halt, &[], RuntimeOpcode::Halt);
    tree.add_signature(
        SourceOpcode::Print,
        &[SignatureArgument::Register],
        RuntimeOpcode::PrintRegister,
    );
    tree.add_signature(
        SourceOpcode::Print,
        &[SignatureArgument::MemoryRef],
        RuntimeOpcode::PrintMemory,
    );
    tree.add_signature(
        SourceOpcode::Input,
        &[SignatureArgument::Register],
        RuntimeOpcode::InputRegister,
    );
    tree.add_signature(
        SourceOpcode::Input,
        &[SignatureArgument::MemoryRef],
        RuntimeOpcode::InputMemory,
    );

    tree
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SignatureArgument {
    Register,
    MemoryRef,
    Label,
    Literal,
}

impl fmt::Display for SignatureArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignatureArgument::Register => write!(f, "register"),
            SignatureArgument::MemoryRef => write!(f, "memory reference"),
            SignatureArgument::Label => write!(f, "label"),
            SignatureArgument::Literal => write!(f, "literal"),
        }
    }
}

#[derive(Debug)]
pub struct SignatureTreeNode {
    pub runtime_opcode: Option<RuntimeOpcode>,
    pub children: HashMap<SignatureArgument, SignatureTreeNode>,
}

impl SignatureTreeNode {
    pub fn new() -> Self {
        SignatureTreeNode {
            runtime_opcode: None,
            children: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct SignatureTree {
    pub root: HashMap<SourceOpcode, SignatureTreeNode>,
}

impl SignatureTree {
    pub fn new() -> Self {
        SignatureTree {
            root: HashMap::new(),
        }
    }

    pub fn add_signature(
        &mut self,
        source_opcode: SourceOpcode,
        arguments: &[SignatureArgument],
        runtime_opcode: RuntimeOpcode,
    ) {
        // Get or create the root node for the SourceOpcode
        let root_node = self
            .root
            .entry(source_opcode)
            .or_insert_with(SignatureTreeNode::new);

        let mut current_node = root_node;

        // Traverse through each argument in the signature, adding child nodes as necessary
        for arg in arguments {
            current_node = current_node
                .children
                .entry(*arg)
                .or_insert_with(SignatureTreeNode::new);
        }

        // Once all arguments are processed, set the runtime opcode at the leaf node
        current_node.runtime_opcode = Some(runtime_opcode);
    }

    pub fn matches_signature(
        &self,
        source_opcode: SourceOpcode,
        operands: &[Operand],
    ) -> Option<RuntimeOpcode> {
        let mut current = self.root.get(&source_opcode)?;
        for operand in operands {
            current = current.children.get(&operand.get_signature_argument())?;
        }
        current.runtime_opcode
    }

    /// Traverses tree to find all possible combinations of operands for a source opcode
    pub fn get_all_valid_operand_combinations_for_source_opcode(
        &self,
        source_opcode: SourceOpcode,
    ) -> Vec<(RuntimeOpcode, Vec<SignatureArgument>)> {
        fn dfs(
            node: &SignatureTreeNode,
            current_path: &mut Vec<SignatureArgument>,
            combinations: &mut Vec<(RuntimeOpcode, Vec<SignatureArgument>)>,
        ) {
            // If we reached a node with a runtime opcode, store the current path as a valid combination
            if let Some(runtime_opcode) = node.runtime_opcode {
                combinations.push((runtime_opcode, current_path.clone()));
            }

            // DFS for each child
            for (arg, child_node) in &node.children {
                current_path.push(*arg);
                dfs(child_node, current_path, combinations);
                current_path.pop();
            }
        }

        let mut combinations = Vec::new();
        if let Some(root) = self.root.get(&source_opcode) {
            dfs(root, &mut Vec::new(), &mut combinations);
        }
        combinations.sort();
        combinations
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_signature_argument_display() {
        let test_cases = [
            (SignatureArgument::Register, "register"),
            (SignatureArgument::MemoryRef, "memory reference"),
            (SignatureArgument::Label, "label"),
            (SignatureArgument::Literal, "literal"),
        ];

        for (arg, expected) in &test_cases {
            assert_eq!(format!("{arg}"), *expected);
        }
    }

    #[test]
    fn test_matches_signature_mov_register_register() {
        let mut tree = SignatureTree::new();
        tree.add_signature(
            SourceOpcode::Mov,
            &[SignatureArgument::Register, SignatureArgument::Register],
            RuntimeOpcode::MovRegister,
        );
        let operands = &[Operand::Register(1), Operand::Register(2)];
        assert_eq!(
            tree.matches_signature(SourceOpcode::Mov, operands),
            Some(RuntimeOpcode::MovRegister)
        );
    }

    #[test]
    fn test_matches_signature_mov_register_literal() {
        let mut tree = SignatureTree::new();
        tree.add_signature(
            SourceOpcode::Mov,
            &[SignatureArgument::Register, SignatureArgument::Literal],
            RuntimeOpcode::MovLiteral,
        );
        let operands = &[Operand::Register(1), Operand::Literal(42)];
        assert_eq!(
            tree.matches_signature(SourceOpcode::Mov, operands),
            Some(RuntimeOpcode::MovLiteral)
        );
    }

    #[test]
    fn test_matches_signature_nop() {
        let mut tree = SignatureTree::new();
        tree.add_signature(SourceOpcode::Nop, &[], RuntimeOpcode::Nop);
        assert_eq!(
            tree.matches_signature(SourceOpcode::Nop, &[]),
            Some(RuntimeOpcode::Nop)
        );
    }

    #[test]
    fn test_matches_signature_no_match_multi_argument() {
        let mut tree = SignatureTree::new();
        tree.add_signature(
            SourceOpcode::Mov,
            &[SignatureArgument::Register, SignatureArgument::Register],
            RuntimeOpcode::MovRegister,
        );
        assert_eq!(
            tree.matches_signature(
                SourceOpcode::Mov,
                &[Operand::Register(1), Operand::Literal(42)]
            ),
            None
        );
    }

    #[test]
    fn test_matches_signature_no_match_no_argument() {
        let mut tree = SignatureTree::new();
        tree.add_signature(SourceOpcode::Nop, &[], RuntimeOpcode::Nop);
        assert_eq!(
            tree.matches_signature(SourceOpcode::Nop, &[Operand::Register(3)]),
            None
        );
    }

    #[test]
    fn test_matches_signature_extra_operand() {
        let mut tree = SignatureTree::new();
        tree.add_signature(
            SourceOpcode::Mov,
            &[SignatureArgument::Register, SignatureArgument::Register],
            RuntimeOpcode::MovRegister,
        );
        let operands = &[
            Operand::Register(1),
            Operand::Register(2),
            Operand::Literal(42),
        ];
        assert_eq!(tree.matches_signature(SourceOpcode::Mov, operands), None);
    }

    #[test]
    fn test_matches_signature_unknown_opcode() {
        let tree = SignatureTree::new();
        let operands = &[Operand::Register(2)];
        assert_eq!(tree.matches_signature(SourceOpcode::Nop, operands), None);
    }

    #[test]
    fn test_get_all_valid_operand_combinations_for_source_opcode_empty() {
        let mut tree = SignatureTree::new();
        tree.add_signature(SourceOpcode::Halt, &[], RuntimeOpcode::Halt);
        assert_eq!(
            tree.get_all_valid_operand_combinations_for_source_opcode(SourceOpcode::Halt),
            vec![(RuntimeOpcode::Halt, vec![])]
        );
    }

    #[test]
    fn test_get_all_valid_operand_combinations_for_source_opcode_one_arg() {
        let mut tree = SignatureTree::new();
        tree.add_signature(SourceOpcode::Halt, &[], RuntimeOpcode::Halt);
        tree.add_signature(
            SourceOpcode::Halt,
            &[SignatureArgument::Register],
            RuntimeOpcode::Halt,
        );
        assert_eq!(
            tree.get_all_valid_operand_combinations_for_source_opcode(SourceOpcode::Halt),
            vec![
                (RuntimeOpcode::Halt, vec![]),
                (RuntimeOpcode::Halt, vec![SignatureArgument::Register])
            ]
        );
    }

    #[test]
    fn test_get_all_valid_operand_combinations_for_source_opcode_multiple_args() {
        let mut tree = SignatureTree::new();
        tree.add_signature(SourceOpcode::Halt, &[], RuntimeOpcode::Halt);
        tree.add_signature(
            SourceOpcode::Halt,
            &[SignatureArgument::Register],
            RuntimeOpcode::Halt,
        );
        tree.add_signature(
            SourceOpcode::Halt,
            &[SignatureArgument::Register, SignatureArgument::Register],
            RuntimeOpcode::Halt,
        );
        tree.add_signature(
            SourceOpcode::Halt,
            &[SignatureArgument::Register, SignatureArgument::Label],
            RuntimeOpcode::Halt,
        );
        assert_eq!(
            tree.get_all_valid_operand_combinations_for_source_opcode(SourceOpcode::Halt),
            vec![
                (RuntimeOpcode::Halt, vec![]),
                (RuntimeOpcode::Halt, vec![SignatureArgument::Register]),
                (
                    RuntimeOpcode::Halt,
                    vec![SignatureArgument::Register, SignatureArgument::Register]
                ),
                (
                    RuntimeOpcode::Halt,
                    vec![SignatureArgument::Register, SignatureArgument::Label]
                ),
            ]
        );
    }
}
