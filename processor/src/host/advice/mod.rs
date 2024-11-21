use alloc::vec::Vec;
use core::borrow::Borrow;

use vm_core::{
    crypto::{
        hash::RpoDigest,
        merkle::{InnerNodeInfo, MerklePath, MerkleStore, NodeIndex, StoreNode},
    },
    AdviceInjector, SignatureKind,
};

use super::HostResponse;
use crate::{ExecutionError, Felt, InputError, ProcessState, Word};

mod extractors;
pub use extractors::AdviceExtractor;

mod inputs;
pub use inputs::AdviceInputs;

mod injectors;

mod providers;
pub use providers::{MemAdviceProvider, RecAdviceProvider};

mod source;
pub use source::AdviceSource;

// ADVICE PROVIDER
// ================================================================================================

/// Defines behavior of an advice provider.
///
/// An advice provider is a component through which the host can interact with the advice provider.
/// The host can request nondeterministic inputs from the advice provider (i.e., result of a
/// computation performed outside of the VM), as well as insert new data into the advice provider.
///
/// An advice provider consists of the following components:
/// 1. Advice stack, which is a LIFO data structure. The processor can move the elements from the
///    advice stack onto the operand stack, as well as push new elements onto the advice stack.
/// 2. Advice map, which is a key-value map where keys are words (4 field elements) and values are
///    vectors of field elements. The processor can push the values from the map onto the advice
///    stack, as well as insert new values into the map.
/// 3. Merkle store, which contains structured data reducible to Merkle paths. The VM can request
///    Merkle paths from the store, as well as mutate it by updating or merging nodes contained in
///    the store.
pub trait AdviceProvider: Sized {
    // ADVICE HANDLERS
    // --------------------------------------------------------------------------------------------

    /// Handles the specified advice injector request.
    fn set_advice(
        &mut self,
        process: ProcessState,
        advice_injector: &AdviceInjector,
    ) -> Result<HostResponse, ExecutionError> {
        match advice_injector {
            AdviceInjector::MerkleNodeMerge => self.merge_merkle_nodes(process),
            AdviceInjector::MerkleNodeToStack => self.copy_merkle_node_to_adv_stack(process),
            AdviceInjector::MapValueToStack { include_len, key_offset } => {
                self.copy_map_value_to_adv_stack(process, *include_len, *key_offset)
            },
            AdviceInjector::UpdateMerkleNode => self.update_operand_stack_merkle_node(process),
            AdviceInjector::U64Div => self.push_u64_div_result(process),
            AdviceInjector::Ext2Inv => self.push_ext2_inv_result(process),
            AdviceInjector::Ext2Intt => self.push_ext2_intt_result(process),
            AdviceInjector::SmtGet => self.push_smtget_inputs(process),
            AdviceInjector::SmtSet => self.push_smtset_inputs(process),
            AdviceInjector::SmtPeek => self.push_smtpeek_result(process),
            AdviceInjector::U32Clz => self.push_leading_zeros(process),
            AdviceInjector::U32Ctz => self.push_trailing_zeros(process),
            AdviceInjector::U32Clo => self.push_leading_ones(process),
            AdviceInjector::U32Cto => self.push_trailing_ones(process),
            AdviceInjector::ILog2 => self.push_ilog2(process),

            AdviceInjector::MemToMap => self.insert_mem_values_into_adv_map(process),
            AdviceInjector::HdwordToMap { domain } => {
                self.insert_hdword_into_adv_map(process, *domain)
            },
            AdviceInjector::HpermToMap => self.insert_hperm_into_adv_map(process),
            AdviceInjector::SigToStack { kind } => self.push_signature(process, *kind),
        }
    }

    /// Handles the specified advice extractor request.
    fn get_advice(
        &mut self,
        process: ProcessState,
        advice_extractor: &AdviceExtractor,
    ) -> Result<HostResponse, ExecutionError> {
        match advice_extractor {
            AdviceExtractor::PopStack => self.pop_stack(process).map(HostResponse::Element),
            AdviceExtractor::PopStackDWord => {
                self.pop_stack_dword(process).map(HostResponse::DoubleWord)
            },
            AdviceExtractor::PopStackWord => self.pop_stack_word(process).map(HostResponse::Word),
            AdviceExtractor::GetMerklePath => self.get_operand_stack_merkle_path(process),
        }
    }

    // DEFAULT ADVICE MAP INJECTORS
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
    ///
    /// # Errors
    /// Returns an error:
    /// - `start_addr` is greater than or equal to 2^32.
    /// - `end_addr` is greater than or equal to 2^32.
    /// - `start_addr` > `end_addr`.
    fn insert_mem_values_into_adv_map(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_map_injectors::insert_mem_values_into_adv_map(self, process)
    }

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
    fn insert_hdword_into_adv_map(
        &mut self,
        process: ProcessState,
        domain: Felt,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_map_injectors::insert_hdword_into_adv_map(self, process, domain)
    }

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
    fn insert_hperm_into_adv_map(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_map_injectors::insert_hperm_into_adv_map(self, process)
    }

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
    ///
    /// It is not checked whether the provided roots exist as Merkle trees in the advide providers.
    fn merge_merkle_nodes(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_map_injectors::merge_merkle_nodes(self, process)
    }

    // DEFAULT ADVICE STACK INJECTORS
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
    ///
    /// # Errors
    /// Returns an error if:
    /// - Merkle tree for the specified root cannot be found in the advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree identified
    ///   by the specified root.
    /// - Value of the node at the specified depth and index is not known to the advice provider.
    fn copy_merkle_node_to_adv_stack(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_stack_injectors::copy_merkle_node_to_adv_stack(self, process)
    }

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
    ///
    /// # Errors
    /// Returns an error if the required key was not found in the key-value map or if stack offset
    /// is greater than 12.
    fn copy_map_value_to_adv_stack(
        &mut self,
        process: ProcessState,
        include_len: bool,
        key_offset: usize,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_stack_injectors::copy_map_value_to_adv_stack(
            self,
            process,
            include_len,
            key_offset,
        )
    }

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
    ///
    /// # Errors
    /// Returns an error if the divisor is ZERO.
    fn push_u64_div_result(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_stack_injectors::push_u64_div_result(self, process)
    }

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
    ///
    /// # Errors
    /// Returns an error if the input is a zero element in the extension field.
    fn push_ext2_inv_result(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_stack_injectors::push_ext2_inv_result(self, process)
    }

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
    /// - `input_size` is the number of evaluations (each evaluation is 2 base field elements). Must
    ///   be a power of 2 and greater 1.
    /// - `output_size` is the number of coefficients in the interpolated polynomial (each
    ///   coefficient is 2 base field elements). Must be smaller than or equal to the number of
    ///   input evaluations.
    /// - `input_start_ptr` is the memory address of the first evaluation.
    /// - `coefficients` are the coefficients of the interpolated polynomial such that lowest degree
    ///   coefficients are located at the top of the advice stack.
    ///
    /// # Errors
    /// Returns an error if:
    /// - `input_size` less than or equal to 1, or is not a power of 2.
    /// - `output_size` is 0 or is greater than the `input_size`.
    /// - `input_ptr` is greater than 2^32.
    /// - `input_ptr + input_size / 2` is greater than 2^32.
    fn push_ext2_intt_result(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_stack_injectors::push_ext2_intt_result(self, process)
    }

    /// Pushes values onto the advice stack which are required for verification of a DSA in Miden
    /// VM.
    ///
    /// Inputs:
    ///   Operand stack: [PK, MSG, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [PK, MSG, ...]
    ///   Advice stack: \[DATA\]
    ///
    /// Where:
    /// - PK is the digest of an expanded public.
    /// - MSG is the digest of the message to be signed.
    /// - DATA is the needed data for signature verification in the VM.
    ///
    /// The advice provider is expected to contain the private key associated to the public key PK.
    fn push_signature(
        &mut self,
        process: ProcessState,
        kind: SignatureKind,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_stack_injectors::push_signature(self, process, kind)
    }

    /// Pushes the number of the leading zeros of the top stack element onto the advice stack.
    ///
    /// Inputs:
    ///   Operand stack: [n, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [n, ...]
    ///   Advice stack: [leading_zeros, ...]
    fn push_leading_zeros(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_stack_injectors::push_leading_zeros(self, process)
    }

    /// Pushes the number of the trailing zeros of the top stack element onto the advice stack.
    ///
    /// Inputs:
    ///   Operand stack: [n, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [n, ...]
    ///   Advice stack: [trailing_zeros, ...]
    fn push_trailing_zeros(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_stack_injectors::push_trailing_zeros(self, process)
    }

    /// Pushes the number of the leading ones of the top stack element onto the advice stack.
    ///
    /// Inputs:
    ///   Operand stack: [n, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [n, ...]
    ///   Advice stack: [leading_ones, ...]
    fn push_leading_ones(&mut self, process: ProcessState) -> Result<HostResponse, ExecutionError> {
        injectors::adv_stack_injectors::push_leading_ones(self, process)
    }

    /// Pushes the number of the trailing ones of the top stack element onto the advice stack.
    ///
    /// Inputs:
    ///   Operand stack: [n, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [n, ...]
    ///   Advice stack: [trailing_ones, ...]
    fn push_trailing_ones(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::adv_stack_injectors::push_trailing_ones(self, process)
    }

    /// Pushes the base 2 logarithm of the top stack element, rounded down.
    /// Inputs:
    ///   Operand stack: [n, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [n, ...]
    ///   Advice stack: [ilog2(n), ...]
    ///
    /// # Errors
    /// Returns an error if the logarithm argument (top stack element) equals ZERO.
    fn push_ilog2(&mut self, process: ProcessState) -> Result<HostResponse, ExecutionError> {
        injectors::adv_stack_injectors::push_ilog2(self, process)
    }

    // DEFAULT MERKLE STORE INJECTORS
    // --------------------------------------------------------------------------------------------

    /// Updates the node of a Merkle tree specified by the values on the top of the operand stack.
    /// Returns the path from the updated node to the new root of the tree to the caller.
    ///
    /// Inputs:
    ///  Operand stack: [OLD_NODE, depth, index, OLD_ROOT, NEW_NODE, ...]
    ///  Advice: [...]
    ///  Merkle store: {...}
    ///
    /// Outputs:
    ///  Operand stack: [OLD_NODE, depth, index, OLD_ROOT, NEW_NODE, ...]
    ///  Advice stack: [...]
    ///  Merkle store: {path, ...}
    ///  Return: \[path\]
    fn update_operand_stack_merkle_node(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::merkle_store_injectors::update_operand_stack_merkle_node(self, process)
    }

    // DEFAULT MERKLE STORE EXTRACTORS
    // --------------------------------------------------------------------------------------------

    /// Extracts a Merkle path for the node specified by the values at the top of the operand stack
    /// and returns it to the caller.
    ///
    /// # Errors
    /// Returns an error if the Merkle store does not contain the specified Merkle path.
    ///
    /// Inputs:
    ///  Operand stack: [WORD, depth, index, ROOT, ...]
    ///  Advice stack: [...]
    ///  Advice map: {...}
    ///  Merkle store: {path, ...}
    ///
    /// Outputs:
    ///  Operand stack: [WORD, depth, index, ROOT, ...]
    ///  Advice stack: [...]
    ///  Advice map: {...}
    ///  Merkle store: {path, ...}
    ///  Return: \[path\]
    fn get_operand_stack_merkle_path(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        let depth = process.get_stack_item(4);
        let index = process.get_stack_item(5);
        let root = [
            process.get_stack_item(9),
            process.get_stack_item(8),
            process.get_stack_item(7),
            process.get_stack_item(6),
        ];
        self.get_merkle_path(root, &depth, &index).map(HostResponse::MerklePath)
    }

    // DEFAULT SMT INJECTORS
    // --------------------------------------------------------------------------------------------

    /// Pushes onto the advice stack the value associated with the specified key in a Sparse
    /// Merkle Tree defined by the specified root.
    ///
    /// If no value was previously associated with the specified key, [ZERO; 4] is pushed onto
    /// the advice stack.
    ///
    /// Inputs:
    ///   Operand stack: [KEY, ROOT, ...]
    ///   Advice stack: [...]
    ///
    /// Outputs:
    ///   Operand stack: [KEY, ROOT, ...]
    ///   Advice stack: [VALUE, ...]
    ///
    /// # Errors
    /// Returns an error if the provided Merkle root doesn't exist on the advice provider.
    ///
    /// # Panics
    /// Will panic as unimplemented if the target depth is `64`.
    fn push_smtpeek_result(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::smt::push_smtpeek_result(self, process)
    }

    /// Currently unimplemented
    fn push_smtget_inputs(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::smt::push_smtget_inputs(self, process)
    }

    /// Currently unimplemented
    fn push_smtset_inputs(
        &mut self,
        process: ProcessState,
    ) -> Result<HostResponse, ExecutionError> {
        injectors::smt::push_smtset_inputs(self, process)
    }

    // ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Creates a "by reference" advice provider for this instance.
    ///
    /// The returned adapter also implements [AdviceProvider] and will simply mutably borrow this
    /// instance.
    fn by_ref(&mut self) -> &mut Self {
        // this trait follows the same model as
        // [io::Read](https://doc.rust-lang.org/std/io/trait.Read.html#method.by_ref).
        //
        // this approach allows the flexibility to take an advice provider either as owned or by
        // mutable reference - both equally compatible with the trait requirements as we implement
        // `AdviceProvider` for mutable references of any type that also implements advice
        // provider.
        self
    }

    // REQUIRED METHODS
    // --------------------------------------------------------------------------------------------

    // ADVICE STACK
    // --------------------------------------------------------------------------------------------

    /// Pops an element from the advice stack and returns it.
    ///
    /// # Errors
    /// Returns an error if the advice stack is empty.
    fn pop_stack(&mut self, process: ProcessState) -> Result<Felt, ExecutionError>;

    /// Pops a word (4 elements) from the advice stack and returns it.
    ///
    /// Note: a word is popped off the stack element-by-element. For example, a `[d, c, b, a, ...]`
    /// stack (i.e., `d` is at the top of the stack) will yield `[d, c, b, a]`.
    ///
    /// # Errors
    /// Returns an error if the advice stack does not contain a full word.
    fn pop_stack_word(&mut self, process: ProcessState) -> Result<Word, ExecutionError>;

    /// Pops a double word (8 elements) from the advice stack and returns them.
    ///
    /// Note: words are popped off the stack element-by-element. For example, a
    /// `[h, g, f, e, d, c, b, a, ...]` stack (i.e., `h` is at the top of the stack) will yield
    /// two words: `[h, g, f,e ], [d, c, b, a]`.
    ///
    /// # Errors
    /// Returns an error if the advice stack does not contain two words.
    fn pop_stack_dword(&mut self, process: ProcessState) -> Result<[Word; 2], ExecutionError>;

    /// Pushes the value(s) specified by the source onto the advice stack.
    ///
    /// # Errors
    /// Returns an error if the value specified by the advice source cannot be obtained.
    fn push_stack(&mut self, source: AdviceSource) -> Result<(), ExecutionError>;

    // ADVICE MAP
    // --------------------------------------------------------------------------------------------

    /// Returns a reference to the value(s) associated with the specified key in the advice map.
    fn get_mapped_values(&self, key: &RpoDigest) -> Option<&[Felt]>;

    /// Inserts the provided value into the advice map under the specified key.
    ///
    /// The values in the advice map can be moved onto the advice stack by invoking
    /// [AdviceProvider::push_stack()] method.
    ///
    /// If the specified key is already present in the advice map, the values under the key
    /// are replaced with the specified values.
    fn insert_into_map(&mut self, key: Word, values: Vec<Felt>);

    /// Returns a signature on a message using a public key.
    fn get_signature(
        &self,
        kind: SignatureKind,
        pub_key: Word,
        msg: Word,
    ) -> Result<Vec<Felt>, ExecutionError>;

    // MERKLE STORE
    // --------------------------------------------------------------------------------------------

    /// Returns a node at the specified depth and index in a Merkle tree with the given root.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree identified
    ///   by the specified root.
    /// - Value of the node at the specified depth and index is not known to this advice provider.
    fn get_tree_node(&self, root: Word, depth: &Felt, index: &Felt)
        -> Result<Word, ExecutionError>;

    /// Returns a path to a node at the specified depth and index in a Merkle tree with the
    /// specified root.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree identified
    ///   by the specified root.
    /// - Path to the node at the specified depth and index is not known to this advice provider.
    fn get_merkle_path(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<MerklePath, ExecutionError>;

    /// Reconstructs a path from the root until a leaf or empty node and returns its depth.
    ///
    /// For more information, check [MerkleStore::get_leaf_depth].
    ///
    /// # Errors
    /// Will return an error if:
    /// - The provided `tree_depth` doesn't fit `u8`.
    /// - The conditions of [MerkleStore::get_leaf_depth] aren't met.
    fn get_leaf_depth(
        &self,
        root: Word,
        tree_depth: &Felt,
        index: &Felt,
    ) -> Result<u8, ExecutionError>;

    /// Updates a node at the specified depth and index in a Merkle tree with the specified root;
    /// returns the Merkle path from the updated node to the new root, together with the new root.
    ///
    /// The tree is cloned prior to the update. Thus, the advice provider retains the original and
    /// the updated tree.
    ///
    /// # Errors
    /// Returns an error if:
    /// - A Merkle tree for the specified root cannot be found in this advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree identified
    ///   by the specified root.
    /// - Path to the leaf at the specified index in the specified Merkle tree is not known to this
    ///   advice provider.
    fn update_merkle_node(
        &mut self,
        root: Word,
        depth: &Felt,
        index: &Felt,
        value: Word,
    ) -> Result<(MerklePath, Word), ExecutionError>;

    /// Creates a new Merkle tree in the advice provider by combining Merkle trees with the
    /// specified roots. The root of the new tree is defined as `hash(left_root, right_root)`.
    ///
    /// After the operation, both the original trees and the new tree remains in the advice
    /// provider (i.e., the input trees are not removed).
    ///
    /// It is not checked whether a Merkle tree for either of the specified roots can be found in
    /// this advice provider.
    fn merge_roots(&mut self, lhs: Word, rhs: Word) -> Result<Word, ExecutionError>;

    /// Returns a subset of this Merkle store such that the returned Merkle store contains all
    /// nodes which are descendants of the specified roots.
    ///
    /// The roots for which no descendants exist in this Merkle store are ignored.
    fn get_store_subset<I, R>(&self, roots: I) -> MerkleStore
    where
        I: Iterator<Item = R>,
        R: Borrow<RpoDigest>;
}

impl<T> AdviceProvider for &mut T
where
    T: AdviceProvider,
{
    fn pop_stack(&mut self, process: ProcessState) -> Result<Felt, ExecutionError> {
        T::pop_stack(self, process)
    }

    fn pop_stack_word(&mut self, process: ProcessState) -> Result<Word, ExecutionError> {
        T::pop_stack_word(self, process)
    }

    fn pop_stack_dword(&mut self, process: ProcessState) -> Result<[Word; 2], ExecutionError> {
        T::pop_stack_dword(self, process)
    }

    fn push_stack(&mut self, source: AdviceSource) -> Result<(), ExecutionError> {
        T::push_stack(self, source)
    }

    fn insert_into_map(&mut self, key: Word, values: Vec<Felt>) {
        T::insert_into_map(self, key, values)
    }

    fn get_signature(
        &self,
        kind: SignatureKind,
        pub_key: Word,
        msg: Word,
    ) -> Result<Vec<Felt>, ExecutionError> {
        T::get_signature(self, kind, pub_key, msg)
    }

    fn get_mapped_values(&self, key: &RpoDigest) -> Option<&[Felt]> {
        T::get_mapped_values(self, key)
    }

    fn get_tree_node(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<Word, ExecutionError> {
        T::get_tree_node(self, root, depth, index)
    }

    fn get_merkle_path(
        &self,
        root: Word,
        depth: &Felt,
        index: &Felt,
    ) -> Result<MerklePath, ExecutionError> {
        T::get_merkle_path(self, root, depth, index)
    }

    fn get_leaf_depth(
        &self,
        root: Word,
        tree_depth: &Felt,
        index: &Felt,
    ) -> Result<u8, ExecutionError> {
        T::get_leaf_depth(self, root, tree_depth, index)
    }

    fn update_merkle_node(
        &mut self,
        root: Word,
        depth: &Felt,
        index: &Felt,
        value: Word,
    ) -> Result<(MerklePath, Word), ExecutionError> {
        T::update_merkle_node(self, root, depth, index, value)
    }

    fn merge_roots(&mut self, lhs: Word, rhs: Word) -> Result<Word, ExecutionError> {
        T::merge_roots(self, lhs, rhs)
    }

    fn get_store_subset<I, R>(&self, roots: I) -> MerkleStore
    where
        I: Iterator<Item = R>,
        R: Borrow<RpoDigest>,
    {
        T::get_store_subset(self, roots)
    }
}
