use std::{collections::HashMap, fmt};

use once_cell::sync::Lazy;

use super::{operand::Operand, runtime_opcode::RuntimeOpcode, source_opcode::SourceOpcode};

pub static SIGNATURE_TREE: Lazy<SignatureTree> = Lazy::new(|| {
    let mut tree = SignatureTree::new();

    tree.add_signature(SourceOpcode::NOP, &[], RuntimeOpcode::NOP);
    tree.add_signature(
        SourceOpcode::LDR,
        &[SignatureArgument::Register, SignatureArgument::MemoryRef],
        RuntimeOpcode::LDR,
    );
    tree.add_signature(
        SourceOpcode::STR,
        &[SignatureArgument::Register, SignatureArgument::MemoryRef],
        RuntimeOpcode::STR,
    );
    tree.add_signature(
        SourceOpcode::ADD,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::ADD_LITERAL,
    );
    tree.add_signature(
        SourceOpcode::ADD,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::ADD_REGISTER,
    );
    tree.add_signature(
        SourceOpcode::SUB,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::SUB_LITERAL,
    );
    tree.add_signature(
        SourceOpcode::SUB,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::SUB_REGISTER,
    );
    tree.add_signature(
        SourceOpcode::MOV,
        &[SignatureArgument::Register, SignatureArgument::Literal],
        RuntimeOpcode::MOV_LITERAL,
    );
    tree.add_signature(
        SourceOpcode::MOV,
        &[SignatureArgument::Register, SignatureArgument::Register],
        RuntimeOpcode::MOV_REGISTER,
    );
    tree.add_signature(
        SourceOpcode::CMP,
        &[SignatureArgument::Register, SignatureArgument::Literal],
        RuntimeOpcode::CMP_LITERAL,
    );
    tree.add_signature(
        SourceOpcode::CMP,
        &[SignatureArgument::Register, SignatureArgument::Register],
        RuntimeOpcode::CMP_REGISTER,
    );
    tree.add_signature(
        SourceOpcode::B,
        &[SignatureArgument::Label],
        RuntimeOpcode::B,
    );
    tree.add_signature(
        SourceOpcode::BEQ,
        &[SignatureArgument::Label],
        RuntimeOpcode::BEQ,
    );
    tree.add_signature(
        SourceOpcode::BNE,
        &[SignatureArgument::Label],
        RuntimeOpcode::BNE,
    );
    tree.add_signature(
        SourceOpcode::BGT,
        &[SignatureArgument::Label],
        RuntimeOpcode::BGT,
    );
    tree.add_signature(
        SourceOpcode::BLT,
        &[SignatureArgument::Label],
        RuntimeOpcode::BLT,
    );
    tree.add_signature(
        SourceOpcode::AND,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::AND_LITERAL,
    );
    tree.add_signature(
        SourceOpcode::AND,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::AND_REGISTER,
    );
    tree.add_signature(
        SourceOpcode::ORR,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::ORR_LITERAL,
    );
    tree.add_signature(
        SourceOpcode::ORR,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::ORR_REGISTER,
    );
    tree.add_signature(
        SourceOpcode::EOR,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::EOR_LITERAL,
    );
    tree.add_signature(
        SourceOpcode::EOR,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::EOR_REGISTER,
    );
    tree.add_signature(
        SourceOpcode::MVN,
        &[SignatureArgument::Register, SignatureArgument::Literal],
        RuntimeOpcode::MVN_LITERAL,
    );
    tree.add_signature(
        SourceOpcode::MVN,
        &[SignatureArgument::Register, SignatureArgument::Register],
        RuntimeOpcode::MVN_REGISTER,
    );
    tree.add_signature(
        SourceOpcode::LSL,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::LSL_LITERAL,
    );
    tree.add_signature(
        SourceOpcode::LSL,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::LSL_REGISTER,
    );
    tree.add_signature(
        SourceOpcode::LSR,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Literal,
        ],
        RuntimeOpcode::LSR_LITERAL,
    );
    tree.add_signature(
        SourceOpcode::LSR,
        &[
            SignatureArgument::Register,
            SignatureArgument::Register,
            SignatureArgument::Register,
        ],
        RuntimeOpcode::LSR_REGISTER,
    );
    tree.add_signature(SourceOpcode::HALT, &[], RuntimeOpcode::HALT);
    tree.add_signature(
        SourceOpcode::PRINT,
        &[SignatureArgument::Register],
        RuntimeOpcode::PRINT_REGISTER,
    );
    tree.add_signature(
        SourceOpcode::PRINT,
        &[SignatureArgument::MemoryRef],
        RuntimeOpcode::PRINT_MEMORY,
    );
    tree.add_signature(
        SourceOpcode::INPUT,
        &[SignatureArgument::Register],
        RuntimeOpcode::INPUT_REGISTER,
    );
    tree.add_signature(
        SourceOpcode::INPUT,
        &[SignatureArgument::MemoryRef],
        RuntimeOpcode::INPUT_MEMORY,
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

        for (arg, expected) in test_cases.iter() {
            assert_eq!(format!("{}", arg), *expected);
        }
    }

    #[test]
    fn test_matches_signature_mov_register_register() {
        let mut tree = SignatureTree::new();
        tree.add_signature(
            SourceOpcode::MOV,
            &[SignatureArgument::Register, SignatureArgument::Register],
            RuntimeOpcode::MOV_REGISTER,
        );
        let operands = &[Operand::Register(1), Operand::Register(2)];
        assert_eq!(
            tree.matches_signature(SourceOpcode::MOV, operands),
            Some(RuntimeOpcode::MOV_REGISTER)
        );
    }

    #[test]
    fn test_matches_signature_mov_register_literal() {
        let mut tree = SignatureTree::new();
        tree.add_signature(
            SourceOpcode::MOV,
            &[SignatureArgument::Register, SignatureArgument::Literal],
            RuntimeOpcode::MOV_LITERAL,
        );
        let operands = &[Operand::Register(1), Operand::Literal(42)];
        assert_eq!(
            tree.matches_signature(SourceOpcode::MOV, operands),
            Some(RuntimeOpcode::MOV_LITERAL)
        );
    }

    #[test]
    fn test_matches_signature_nop() {
        let mut tree = SignatureTree::new();
        tree.add_signature(SourceOpcode::NOP, &[], RuntimeOpcode::NOP);
        assert_eq!(
            tree.matches_signature(SourceOpcode::NOP, &[]),
            Some(RuntimeOpcode::NOP)
        );
    }

    #[test]
    fn test_matches_signature_no_match_multi_argument() {
        let mut tree = SignatureTree::new();
        tree.add_signature(
            SourceOpcode::MOV,
            &[SignatureArgument::Register, SignatureArgument::Register],
            RuntimeOpcode::MOV_REGISTER,
        );
        assert_eq!(
            tree.matches_signature(
                SourceOpcode::MOV,
                &[Operand::Register(1), Operand::Literal(42)]
            ),
            None
        );
    }

    #[test]
    fn test_matches_signature_no_match_no_argument() {
        let mut tree = SignatureTree::new();
        tree.add_signature(SourceOpcode::NOP, &[], RuntimeOpcode::NOP);
        assert_eq!(
            tree.matches_signature(SourceOpcode::NOP, &[Operand::Register(3)]),
            None
        );
    }

    #[test]
    fn test_matches_signature_extra_operand() {
        let mut tree = SignatureTree::new();
        tree.add_signature(
            SourceOpcode::MOV,
            &[SignatureArgument::Register, SignatureArgument::Register],
            RuntimeOpcode::MOV_REGISTER,
        );
        let operands = &[
            Operand::Register(1),
            Operand::Register(2),
            Operand::Literal(42),
        ];
        assert_eq!(tree.matches_signature(SourceOpcode::MOV, operands), None);
    }

    #[test]
    fn test_matches_signature_unknown_opcode() {
        let tree = SignatureTree::new();
        let operands = &[Operand::Register(2)];
        assert_eq!(tree.matches_signature(SourceOpcode::NOP, operands), None);
    }

    #[test]
    fn test_get_all_valid_operand_combinations_for_source_opcode_empty() {
        let mut tree = SignatureTree::new();
        tree.add_signature(SourceOpcode::HALT, &[], RuntimeOpcode::HALT);
        assert_eq!(
            tree.get_all_valid_operand_combinations_for_source_opcode(SourceOpcode::HALT),
            vec![(RuntimeOpcode::HALT, vec![])]
        );
    }

    #[test]
    fn test_get_all_valid_operand_combinations_for_source_opcode_one_arg() {
        let mut tree = SignatureTree::new();
        tree.add_signature(SourceOpcode::HALT, &[], RuntimeOpcode::HALT);
        tree.add_signature(
            SourceOpcode::HALT,
            &[SignatureArgument::Register],
            RuntimeOpcode::HALT,
        );
        assert_eq!(
            tree.get_all_valid_operand_combinations_for_source_opcode(SourceOpcode::HALT),
            vec![
                (RuntimeOpcode::HALT, vec![]),
                (RuntimeOpcode::HALT, vec![SignatureArgument::Register])
            ]
        );
    }

    #[test]
    fn test_get_all_valid_operand_combinations_for_source_opcode_multiple_args() {
        let mut tree = SignatureTree::new();
        tree.add_signature(SourceOpcode::HALT, &[], RuntimeOpcode::HALT);
        tree.add_signature(
            SourceOpcode::HALT,
            &[SignatureArgument::Register],
            RuntimeOpcode::HALT,
        );
        tree.add_signature(
            SourceOpcode::HALT,
            &[SignatureArgument::Register, SignatureArgument::Register],
            RuntimeOpcode::HALT,
        );
        tree.add_signature(
            SourceOpcode::HALT,
            &[SignatureArgument::Register, SignatureArgument::Label],
            RuntimeOpcode::HALT,
        );
        assert_eq!(
            tree.get_all_valid_operand_combinations_for_source_opcode(SourceOpcode::HALT),
            vec![
                (RuntimeOpcode::HALT, vec![]),
                (RuntimeOpcode::HALT, vec![SignatureArgument::Register]),
                (
                    RuntimeOpcode::HALT,
                    vec![SignatureArgument::Register, SignatureArgument::Register]
                ),
                (
                    RuntimeOpcode::HALT,
                    vec![SignatureArgument::Register, SignatureArgument::Label]
                ),
            ]
        );
    }
}
