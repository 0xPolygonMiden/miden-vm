use core::fmt;

/// TODO: add docs
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AdviceInjector {
    /// Pushes a node of the Merkle tree specified by the values on the top of the operand stack
    /// onto the advice stack. The operand stack is expected to be arranged as follows (from the
    /// top):
    /// - depth of the node, 1 element
    /// - index of the node, 1 element
    /// - root of the tree, 4 elements
    MerkleNode,

    /// Creates a new Merkle tree in the advice provider by combining Merkle trees with the
    /// specified roots. The root of the new tree is defined as `Hash(left_root, right_root)`.
    ///
    /// The operand stack is expected to be arranged as follows:
    /// - root of the right tree, 4 elements.
    /// - root of the left tree, 4 elements.
    ///
    /// After the operation, both the original trees and the new tree remains in the advice
    /// provider (i.e., the input trees are not removed).
    MerkleMerge,

    /// Pushes the result of [u64] division (both the quotient and the remainder) onto the advice
    /// stack. The operand stack is expected to be arranged as follows (from the top):
    /// - divisor split into two 32-bit elements
    /// - dividend split into two 32-bit elements
    ///
    /// The result is pushed onto the advice stack as follows: first the remainder is pushed,
    /// then the quotient.
    DivResultU64,

    /// Pushes a list of field elements onto the advice stack. The list is looked up in the
    /// key-value map maintained by the advice provider using the top 4 elements of the operand
    /// stack as key.
    MapValue,

    /// Inserts a list of words that are read from VM memory into the advice map under the key
    /// specified by the word at the top of the operand stack.
    ///
    /// The first internal u32 specifies the starting address of the memory region, and the second
    /// specifies the number of words to be read.
    Memory(u32, u32),

    /// Given an element of quadratic extension field, it computes multiplicative inverse and
    /// push the result into the advice stack.
    Ext2Inv,

    /// Given ( power of 2 many ) evaluations of a polynomial over some specified domain, this
    /// routine interpolates ( using inverse NTT ) the evaluations into a polynomial in
    /// coefficient form and pushes the result into the advice stack.
    Ext2INTT,
}

impl fmt::Display for AdviceInjector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MerkleNode => write!(f, "merkle_node"),
            Self::MerkleMerge => write!(f, "merkle_merge"),
            Self::DivResultU64 => write!(f, "div_result_u64"),
            Self::MapValue => write!(f, "map_value"),
            Self::Memory(start_addr, num_words) => write!(f, "mem({start_addr}, {num_words})"),
            Self::Ext2Inv => write!(f, "ext2_inv"),
            Self::Ext2INTT => write!(f, "ext2_intt"),
        }
    }
}
