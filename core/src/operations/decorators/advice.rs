use crate::Felt;
use core::fmt;

// ADVICE INJECTORS
// ================================================================================================

/// Defines a set of actions which can be initiated from the VM to inject new data into the advice
/// provider.
///
/// These actions can affect all 3 components of the advice provider: Merkle store, advice stack,
/// and advice map.
///
/// All actions, except for `MerkleNodeMerge` and `Ext2Inv`, can be invoked directly from Miden
/// assembly via dedicated instructions.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AdviceInjector {
    // MERKLE STORE INJECTORS
    // --------------------------------------------------------------------------------------------
    /// Creates a new Merkle tree in the advice provider by combining Merkle trees with the
    /// specified roots. The root of the new tree is defined as `Hash(LEFT_ROOT, RIGHT_ROOT)`.
    ///
    /// Inputs:
    ///   Operand stack: [RIGHT_ROOT, LEFT_ROOT, ...]
    ///   Merkle store: {RIGHT_ROOT, LEFT_ROOT}
    ///
    /// Outputs:
    ///   Operand stack: [RIGHT_ROOT, LEFT_ROOT, ...]
    ///   Merkle store: {RIGHT_ROOT, LEFT_ROOT, hash(LEFT_ROOT, RIGHT_ROOT)}
    ///
    /// After the operation, both the original trees and the new tree remains in the advice
    /// provider (i.e., the input trees are not removed).
    MerkleNodeMerge,

    // ADVICE STACK INJECTORS
    // --------------------------------------------------------------------------------------------
    /// Pushes a node of the Merkle tree specified by the values on the top of the operand stack
    /// onto the advice stack.
    ///
    /// Inputs:
    ///   Operand stack: [depth, index, TREE_ROOT, ...]
    ///   Advice stack: [...]
    ///   Merkle store: {TREE_ROOT<-NODE}
    ///
    /// Outputs:
    ///   Operand stack: [depth, index, TREE_ROOT, ...]
    ///   Advice stack: [NODE, ...]
    ///   Merkle store: {TREE_ROOT<-NODE}
    MerkleNodeToStack,

    /// Pushes a list of field elements onto the advice stack. The list is looked up in the advice
    /// map using the specified word from the operand stack as the key. If `include_len` is set to
    /// true, the number of elements in the value is also pushed onto the advice stack.
    ///
    /// Inputs:
    ///   Operand stack: [..., KEY, ...]
    ///   Advice stack: [...]
    ///   Advice map: {KEY: values}
    ///
    /// Outputs:
    ///   Operand stack: [..., KEY, ...]
    ///   Advice stack: [values_len?, values, ...]
    ///   Advice map: {KEY: values}
    ///
    /// The `key_offset` value specifies the location of the `KEY` on the stack. For example,
    /// offset value of 0 indicates that the top word on the stack should be used as the key, the
    /// offset value of 4, indicates that the second word on the stack should be used as the key
    /// etc.
    ///
    /// The valid values of `key_offset` are 0 through 12 (inclusive).
    MapValueToStack {
        include_len: bool,
        key_offset: usize,
    },

    /// Pushes the result of [u64] division (both the quotient and the remainder) onto the advice
    /// stack.
    ///
    /// Inputs:
    ///   Operand stack: [b1, b0, a1, a0, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [b1, b0, a1, a0, ...]
    ///   Advice stack: [q0, q1, r0, r1, ...]
    ///
    /// Where (a0, a1) and (b0, b1) are the 32-bit limbs of the dividend and the divisor
    /// respectively (with a0 representing the 32 lest significant bits and a1 representing the
    /// 32 most significant bits). Similarly, (q0, q1) and (r0, r1) represent the quotient and
    /// the remainder respectively.
    DivU64,

    /// Given an element in a quadratic extension field on the top of the stack (i.e., a0, b1),
    /// computes its multiplicative inverse and push the result onto the advice stack.
    ///
    /// Inputs:
    ///   Operand stack: [a1, a0, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [a1, a0, ...]
    ///   Advice stack: [b0, b1...]
    ///
    /// Where (b0, b1) is the multiplicative inverse of the extension field element (a0, a1) at the
    /// top of the stack.
    Ext2Inv,

    /// Given evaluations of a polynomial over some specified domain, interpolates the evaluations
    ///  into a polynomial in coefficient form and pushes the result into the advice stack.
    ///
    /// The interpolation is performed using the iNTT algorithm. The evaluations are expected to be
    /// in the quadratic extension.
    ///
    /// Inputs:
    ///   Operand stack: [output_size, input_size, input_start_ptr, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [output_size, input_size, input_start_ptr, ...]
    ///   Advice stack: [coefficients...]
    ///
    /// - `input_size` is the number of evaluations (each evaluation is 2 base field elements).
    ///   Must be a power of 2 and greater 1.
    /// - `output_size` is the number of coefficients in the interpolated polynomial (each
    ///   coefficient is 2 base field elements). Must be smaller than or equal to the number of
    ///   input evaluations.
    /// - `input_start_ptr` is the memory address of the first evaluation.
    /// - `coefficients` are the coefficients of the interpolated polynomial such that lowest
    ///   degree coefficients are located at the top of the advice stack.
    Ext2Intt,

    /// Pushes values onto the advice stack which are required for successful retrieval of a
    /// value from a Sparse Merkle Tree data structure.
    ///
    /// The Sparse Merkle Tree is tiered, meaning it will have leaf depths in `{16, 32, 48, 64}`.
    /// The depth flags define the tier on which the leaf is located.
    ///
    /// Inputs:
    ///   Operand stack: [KEY, ROOT, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [KEY, ROOT, ...]
    ///   Advice stack: [f0, f1, K, V, f2]
    ///
    /// Where:
    /// - f0 is a boolean flag set to `1` if the depth is `16` or `48`.
    /// - f1 is a boolean flag set to `1` if the depth is `16` or `32`.
    /// - K is the remaining key word; will be zeroed if the tree don't contain a mapped value
    ///   for the key.
    /// - V is the value word; will be zeroed if the tree don't contain a mapped value for the key.
    /// - f2 is a boolean flag set to `1` if a remaining key is not zero.
    SmtGet,

    /// Pushes values onto the advice stack which are required for successful insertion of a
    /// key-value pair into a Sparse Merkle Tree data structure.
    ///
    /// The Sparse Merkle Tree is tiered, meaning it will have leaf depths in `{16, 32, 48, 64}`.
    ///
    /// Inputs:
    ///   Operand stack: [VALUE, KEY, ROOT, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [OLD_VALUE, NEW_ROOT, ...]
    ///   Advice stack depends on the type of insert operation as follows:
    ///   - Update of an existing leaf: [ZERO (padding), d0, d1, ONE (is_update), OLD_VALUE]
    ///   - Simple insert at depth 16: [d0, d1, ONE (is_simple_insert), ZERO (is_update)]
    ///   - Simple insert at depth 32 or 48: [d0, d1, ONE (is_simple_insert), ZERO (is_update), P_NODE]
    ///   - Complex insert: [f0, f1, ZERO (is_simple_insert), ZERO (is_update), E_KEY, E_VALUE]
    ///   - Delete against an empty subtree: [d0, d1, ZERO (is_leaf), ONE (key_not_set)]
    ///   - Delete against another leaf: [d0, d1, ONE (is_leaf), ONE (key_not_set), KEY, VALUE]
    ///   - Delete against own leaf: [ZERO, ZERO, ZERO, ZERO (key_not_set), NEW_ROOT, OLD_VALUE]
    ///
    /// Where:
    /// - ROOT and NEW_ROOT are the roots of the TSMT before and after the insert respectively.
    /// - VALUE is the value to be inserted.
    /// - OLD_VALUE is the value previously associated with the specified KEY.
    /// - d0 is a boolean flag set to `1` if the depth is `16` or `48`.
    /// - d1 is a boolean flag set to `1` if the depth is `16` or `32`.
    /// - P_NODE is an internal node located at the tier above the insert tier.
    /// - f0 and f1 are boolean flags a combination of which determines the source and the target
    ///   tiers as follows:
    ///   - (0, 0): depth 16 -> 32
    ///   - (0, 1): depth 16 -> 48
    ///   - (1, 0): depth 32 -> 48
    ///   - (1, 1): depth 16, 32, or 48 -> 64
    /// - E_KEY and E_VALUE are the key-value pair for a leaf which is to be replaced by a subtree.
    SmtInsert,

    // ADVICE MAP INJECTORS
    // --------------------------------------------------------------------------------------------
    /// Reads words from memory at the specified range and inserts them into the advice map under
    /// the key `KEY` located at the top of the stack.
    ///
    /// Inputs:
    ///   Operand stack: [KEY, start_addr, end_addr, ...]
    ///   Advice map: {...}
    ///
    /// Outputs:
    ///   Operand stack: [KEY, start_addr, end_addr, ...]
    ///   Advice map: {KEY: values}
    ///
    /// Where `values` are the elements located in memory[start_addr..end_addr].
    MemToMap,

    /// Reads two word from the operand stack and inserts them into the advice map under the key
    /// defined by the hash of these words.
    ///
    /// Inputs:
    ///   Operand stack: [B, A, ...]
    ///   Advice map: {...}
    ///
    /// Outputs:
    ///   Operand stack: [B, A, ...]
    ///   Advice map: {KEY: [a0, a1, a2, a3, b0, b1, b2, b3]}
    ///
    /// Where KEY is computed as hash(A || B, domain), where domain is provided via the immediate
    /// value.
    HdwordToMap { domain: Felt },

    /// Reads three words from the operand stack and inserts the top two words into the advice map
    /// under the key defined by applying an RPO permutation to all three words.
    ///
    /// Inputs:
    ///   Operand stack: [B, A, C, ...]
    ///   Advice map: {...}
    ///
    /// Outputs:
    ///   Operand stack: [B, A, C, ...]
    ///   Advice map: {KEY: [a0, a1, a2, a3, b0, b1, b2, b3]}
    ///
    /// Where KEY is computed by extracting the digest elements from hperm([C, A, B]). For example,
    /// if C is [0, d, 0, 0], KEY will be set as hash(A || B, d).
    HpermToMap,
}

impl fmt::Display for AdviceInjector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MerkleNodeMerge => write!(f, "merkle_node_merge"),
            Self::MerkleNodeToStack => write!(f, "merkle_node_to_stack"),
            Self::MapValueToStack {
                include_len,
                key_offset,
            } => {
                if *include_len {
                    write!(f, "map_value_to_stack_with_len.{key_offset}")
                } else {
                    write!(f, "map_value_to_stack.{key_offset}")
                }
            }
            Self::DivU64 => write!(f, "div_u64"),
            Self::Ext2Inv => write!(f, "ext2_inv"),
            Self::Ext2Intt => write!(f, "ext2_intt"),
            Self::SmtGet => write!(f, "smt_get"),
            Self::SmtInsert => write!(f, "smt_insert"),
            Self::MemToMap => write!(f, "mem_to_map"),
            Self::HdwordToMap { domain } => write!(f, "hdword_to_map.{domain}"),
            Self::HpermToMap => write!(f, "hperm_to_map"),
        }
    }
}
