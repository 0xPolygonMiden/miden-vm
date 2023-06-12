use core::fmt;

// ADVICE INJECTORS
// ================================================================================================

/// Defines a set of actions which can be initiated from the VM to inject new data into the advice
/// provider.
///
/// These actions can affect all 3 components of the advice provider: Merkle store, advice stack,
/// and advice map.
///
/// Most of these actions are exposed to the user via Miden assembly instructions, but some actions
/// are used only within higher-level instructions.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AdviceInjector {
    // MERKLE STORE INJECTORS
    // --------------------------------------------------------------------------------------------
    /// Creates a new Merkle tree in the advice provider by combining Merkle trees with the
    /// specified roots. The root of the new tree is defined as `Hash(LEFT_ROOT, RIGHT_ROOT)`.
    ///
    /// The operand stack is expected to be arranged as follows:
    ///
    /// [RIGHT_ROOT, LEFT_ROOT, ...]
    ///
    /// After the operation, both the original trees and the new tree remains in the advice
    /// provider (i.e., the input trees are not removed).
    MerkleTreeMerge,

    // ADVICE STACK INJECTORS
    // --------------------------------------------------------------------------------------------
    /// Pushes a node of the Merkle tree specified by the values on the top of the operand stack
    /// onto the advice stack.
    ///
    /// The operand stack is expected to be arranged as follows:
    ///
    /// [depth, index, TREE_ROOT, ...]
    MerkleNodeToStack,

    /// Pushes a list of field elements onto the advice stack. The list is looked up in the
    /// key-value map maintained by the advice provider using the top 4 elements of the operand
    /// stack as key.
    MapValueToStack,

    /// Pushes the result of [u64] division (both the quotient and the remainder) onto the advice
    /// stack.
    ///
    /// The operand stack is expected to be arranged as follows (from the top):
    /// - divisor split into two 32-bit elements
    /// - dividend split into two 32-bit elements
    ///
    /// The result is pushed onto the advice stack as follows: first the remainder is pushed,
    /// then the quotient.
    DivU64,

    /// Given an element in a quadratic extension field on the top of the stack (i.e., a0, b1),
    /// computes its multiplicative inverse and push the result onto the advice stack.
    ///
    /// The operand stack is expected to be arranged as follows:
    ///
    /// [a1, a0, ...]
    ///
    /// The result (i.e., b0, b1) is pushed onto the advice stack as follows: first b1 is pushed,
    /// then b0 is pushed. Thus, when the VM reads data from the advice stack, it will first read
    /// b0, and then b1.
    Ext2Inv,

    /// Given evaluations of a polynomial over some specified domain, this routine interpolates
    /// the evaluations into a polynomial in coefficient form and pushes the result into the advice
    /// stack.
    ///
    /// The interpolation is performed using the iNTT algorithm. The evaluations are expected to be
    /// in the quadratic extension.
    ///
    /// The operand stack is expected to be arranged as follows:
    ///
    /// [output_size, input_size, input_start_ptr, ...]
    ///
    /// - `input_size` is the number of evaluations (each evaluation is 2 base field elements).
    ///   Must be a power of 2 and greater 1.
    /// - `output_size` is the number of coefficients in the interpolated polynomial (each
    ///   coefficient is 2 base field elements). Must be smaller than or equal to the number of
    ///   input evaluations.
    /// - Memory address of the first evaluation.
    ///
    /// The result is pushed onto the advice stack such that the high-degree coefficients are
    /// pushed first, and the zero-degree coefficient is pushed last.
    Ext2Intt,

    /// Pushes the value and depth flags of a leaf indexed by `KEY` on a Sparse Merkle tree with
    /// the provided `ROOT`.
    ///
    /// The Sparse Merkle tree is tiered, meaning it will have leaf depths in `{16, 32, 48, 64}`.
    /// The depth flags define the tier on which the leaf is located.
    ///
    /// The operand stack is expected to be arranged as follows:
    ///
    /// [KEY, ROOT, ...]
    ///
    /// After a successful operation, the advice stack will look as follows:
    /// - boolean flag set to `1` if the depth is `16` or `48`.
    /// - boolean flag set to `1` if the depth is `16` or `32`.
    /// - remaining key word; will be zeroed if the tree don't contain a mapped value for the key.
    /// - value word; will be zeroed if the tree don't contain a mapped value for the key.
    /// - boolean flag set to `1` if a remaining key is not zero.
    SmtGet,

    // ADVICE MAP INJECTORS
    // --------------------------------------------------------------------------------------------
    /// Reads words from memory range `start_addr .. end_addr` and insert into the advice map
    /// under the key `KEY`.
    ///
    /// The operand stack is expected to be arranged as follows:
    ///
    /// [KEY, start_addr, end_addr, ...]
    MemToMap,
}

impl fmt::Display for AdviceInjector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MerkleTreeMerge => write!(f, "merkle_tree_merge"),
            Self::MerkleNodeToStack => write!(f, "merkle_node_to_stack"),
            Self::MapValueToStack => write!(f, "map_value_to_stack"),
            Self::DivU64 => write!(f, "div_u64"),
            Self::Ext2Inv => write!(f, "ext2_inv"),
            Self::Ext2Intt => write!(f, "ext2_intt"),
            Self::SmtGet => write!(f, "smt_get"),
            Self::MemToMap => write!(f, "mem_to_map"),
        }
    }
}
