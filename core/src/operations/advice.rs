use core::fmt;

/// TODO: add docs
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AdviceInjector {
    /// Injects a node of the Merkle tree specified by the values on the stack at the head of the
    /// advice tape. The stack is expected to be arranged as follows (from the top):
    /// - depth of the node, 1 element
    /// - index of the node, 1 element
    /// - root of the tree, 4 elements
    MerkleNode,
}

impl fmt::Display for AdviceInjector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MerkleNode => write!(f, "merkle_node"),
        }
    }
}
