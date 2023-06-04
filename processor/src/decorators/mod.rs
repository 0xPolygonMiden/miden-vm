use super::{
    AdviceInjector, AdviceProvider, AdviceSource, Decorator, ExecutionError, Felt, Process,
    StarkField, Word,
};
use vm_core::{
    crypto::merkle::EmptySubtreeRoots, utils::collections::Vec, FieldElement, QuadExtension, ONE,
    WORD_SIZE, ZERO,
};
use winter_prover::math::fft;

#[cfg(test)]
mod tests;

// TYPE ALIASES
// ================================================================================================
type QuadFelt = QuadExtension<Felt>;

// CONSTANTS
// ================================================================================================

/// Maximum depth of a Sparse Merkle tree
const SMT_MAX_TREE_DEPTH: Felt = Felt::new(64);

/// Lookup table for Sparse Merkle tree depth normalization
const SMT_NORMALIZED_DEPTHS: [u8; 65] = [
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 32, 32, 32, 32, 32, 32, 32,
    32, 32, 32, 32, 32, 32, 32, 32, 32, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
    48, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
];

// DECORATORS
// ================================================================================================

impl<A> Process<A>
where
    A: AdviceProvider,
{
    /// Executes the specified decorator
    pub(super) fn execute_decorator(
        &mut self,
        decorator: &Decorator,
    ) -> Result<(), ExecutionError> {
        match decorator {
            Decorator::Advice(injector) => self.dec_advice(injector)?,
            Decorator::AsmOp(assembly_op) => {
                if self.decoder.in_debug_mode() {
                    self.decoder.append_asmop(self.system.clk(), assembly_op.clone());
                }
            }
        }
        Ok(())
    }

    // ADVICE INJECTION
    // --------------------------------------------------------------------------------------------

    /// Process the specified advice injector.
    pub fn dec_advice(&mut self, injector: &AdviceInjector) -> Result<(), ExecutionError> {
        match injector {
            AdviceInjector::MerkleNode => self.inject_merkle_node(),
            AdviceInjector::MerkleMerge => self.inject_merkle_merge(),
            AdviceInjector::DivResultU64 => self.inject_div_result_u64(),
            AdviceInjector::MapValue => self.inject_map_value(),
            AdviceInjector::Memory => self.inject_mem_values(),
            AdviceInjector::Ext2Inv => self.inject_ext2_inv_result(),
            AdviceInjector::Ext2INTT => self.inject_ext2_intt_result(),
            AdviceInjector::SmtGet => self.inject_smtget(),
        }
    }

    // INJECTOR HELPERS
    // --------------------------------------------------------------------------------------------

    /// Pushes a node of the Merkle tree specified by the word on the top of the operand stack onto
    /// the advice stack. The operand stack is expected to be arranged as follows (from the top):
    /// - depth of the node, 1 element
    /// - index of the node, 1 element
    /// - root of the tree, 4 elements
    ///
    /// # Errors
    /// Returns an error if:
    /// - Merkle tree for the specified root cannot be found in the advice provider.
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Value of the node at the specified depth and index is not known to the advice provider.
    fn inject_merkle_node(&mut self) -> Result<(), ExecutionError> {
        // read node depth, node index, and tree root from the stack
        let depth = self.stack.get(0);
        let index = self.stack.get(1);
        let root = [self.stack.get(5), self.stack.get(4), self.stack.get(3), self.stack.get(2)];

        // look up the node in the advice provider
        let node = self.advice_provider.get_tree_node(root, &depth, &index)?;

        // push the node onto the advice stack with the first element pushed last so that it can
        // be popped first (i.e. stack behavior for word)
        self.advice_provider.push_stack(AdviceSource::Value(node[3]))?;
        self.advice_provider.push_stack(AdviceSource::Value(node[2]))?;
        self.advice_provider.push_stack(AdviceSource::Value(node[1]))?;
        self.advice_provider.push_stack(AdviceSource::Value(node[0]))?;

        Ok(())
    }

    /// Creates a new Merkle tree in the advice provider by combining Merkle trees with the
    /// specified roots. The root of the new tree is defined as `hash(left_root, right_root)`.
    ///
    /// The operand stack is expected to be arranged as follows:
    /// - root of the right tree, 4 elements
    /// - root of the left tree, 4 elements
    ///
    /// After the operation, both the original trees and the new tree remains in the advice
    /// provider (i.e., the input trees are not removed).
    ///
    /// # Errors
    /// Return an error if a Merkle tree for either of the specified roots cannot be found in this
    /// advice provider.
    fn inject_merkle_merge(&mut self) -> Result<(), ExecutionError> {
        // fetch the arguments from the stack
        let lhs = [self.stack.get(7), self.stack.get(6), self.stack.get(5), self.stack.get(4)];
        let rhs = [self.stack.get(3), self.stack.get(2), self.stack.get(1), self.stack.get(0)];

        // perform the merge
        self.advice_provider.merge_roots(lhs, rhs)?;

        Ok(())
    }

    /// Pushes the result of [u64] division (both the quotient and the remainder) onto the advice
    /// stack. The operand stack is expected to be arranged as follows (from the top):
    /// - divisor split into two 32-bit elements
    /// - dividend split into two 32-bit elements
    ///
    /// The result is pushed onto the advice stack as follows: the remainder is pushed first, then
    /// the quotient is pushed. This guarantees that when popping values from the advice stack, the
    /// quotient will be returned first, and the remainder will be returned next.
    ///
    /// # Errors
    /// Returns an error if the divisor is ZERO.
    fn inject_div_result_u64(&mut self) -> Result<(), ExecutionError> {
        let divisor_hi = self.stack.get(0).as_int();
        let divisor_lo = self.stack.get(1).as_int();
        let divisor = (divisor_hi << 32) + divisor_lo;

        if divisor == 0 {
            return Err(ExecutionError::DivideByZero(self.system.clk()));
        }

        let dividend_hi = self.stack.get(2).as_int();
        let dividend_lo = self.stack.get(3).as_int();
        let dividend = (dividend_hi << 32) + dividend_lo;

        let quotient = dividend / divisor;
        let remainder = dividend - quotient * divisor;

        let (q_hi, q_lo) = u64_to_u32_elements(quotient);
        let (r_hi, r_lo) = u64_to_u32_elements(remainder);

        self.advice_provider.push_stack(AdviceSource::Value(r_hi))?;
        self.advice_provider.push_stack(AdviceSource::Value(r_lo))?;
        self.advice_provider.push_stack(AdviceSource::Value(q_hi))?;
        self.advice_provider.push_stack(AdviceSource::Value(q_lo))?;

        Ok(())
    }

    /// Pushes a list of field elements onto the advice stack. The list is looked up in the advice
    /// map using the top 4 elements (i.e. word) from the operand stack as the key.
    ///
    /// # Errors
    /// Returns an error if the required key was not found in the key-value map.
    fn inject_map_value(&mut self) -> Result<(), ExecutionError> {
        let top_word = self.stack.get_word(0);
        self.advice_provider.push_stack(AdviceSource::Map { key: top_word })?;

        Ok(())
    }

    /// Reads the specfied number of words from the memory starting at the given start address and
    /// writes the vector of field elements to the advice map with the top 4 elements on the stack
    /// as the key. This operation does not affect the state of the Memory chiplet and the VM in
    /// general.
    ///
    /// # Errors
    /// Returns an error if the key is already present in the advice map.
    fn inject_mem_values(&mut self) -> Result<(), ExecutionError> {
        let start_addr = self.stack.get(4).as_int();
        let end_addr = self.stack.get(5).as_int();
        let len = end_addr - start_addr;
        let ctx = self.system.ctx();
        let mut values = Vec::with_capacity((len as usize) * WORD_SIZE);
        for i in 0u64..len {
            let mem_value =
                self.chiplets.get_mem_value(ctx, start_addr + i).unwrap_or([ZERO; WORD_SIZE]);
            values.extend_from_slice(&mem_value);
        }
        let top_word = self.stack.get_word(0);
        self.advice_provider.insert_into_map(top_word, values)?;

        Ok(())
    }

    /// Given a quadratic extension field element ( say a ) on stack top, this routine computes
    /// multiplicative inverse of that element ( say b ) s.t.
    ///
    /// a * b = 1 ( mod P ) | b = a ^ -1, P = irreducible polynomial x^2 - x + 2 over F_q, q = 2^64 - 2^32 + 1
    ///
    /// Input on stack expected in following order
    ///
    /// [coeff_1, coeff_0, ...]
    ///
    /// While computed multiplicative inverse is put on advice provider in following order
    ///
    /// [coeff'_0, coeff'_1, ...]
    ///
    /// Meaning when a Miden program is going to read it from advice stack, it'll see
    /// coefficient_0 first and then coefficient_1.
    ///
    /// Note, in case input operand is zero, division by zero error is returned, because
    /// that's a non-invertible element of extension field.
    fn inject_ext2_inv_result(&mut self) -> Result<(), ExecutionError> {
        let coef0 = self.stack.get(1);
        let coef1 = self.stack.get(0);

        let elm = QuadFelt::new(coef0, coef1);
        if elm == QuadFelt::ZERO {
            return Err(ExecutionError::DivideByZero(self.system.clk()));
        }
        let coeffs = elm.inv().to_base_elements();

        self.advice_provider.push_stack(AdviceSource::Value(coeffs[1]))?;
        self.advice_provider.push_stack(AdviceSource::Value(coeffs[0]))?;

        Ok(())
    }

    /// Given evaluations of a polynomial over some specified domain, this routine interpolates the
    /// evaluations into a polynomial in coefficient form, and pushes the results onto the advice
    /// stack.
    ///
    /// The interpolation is performed using the iNTT algorithm. The evaluations are expected to be
    /// in the quadratic extension field | q = 2^64 - 2^32 + 1.
    ///
    /// Input stack state should look like
    ///
    /// `[output_poly_len, input_eval_len, input_eval_begin_address, ...]`
    ///
    /// - `input_eval_len` must be a power of 2 and > 1.
    /// - `output_poly_len <= input_eval_len`
    /// - Only starting memory address of evaluations ( of length `input_eval_len` ) is
    /// provided on the stack, consecutive memory addresses are expected to be holding
    /// remaining `input_eval_len - 2` many evaluations.
    /// - Each memory address holds two evaluations of the polynomial at adjacent points
    ///
    /// Final advice stack should look like
    ///
    /// `[coeff_0, coeff_1, ..., coeff_{n-1}, ...]` | n = output_poly_len
    ///
    /// Program which is requesting this non-deterministic computation should read `coeff0`
    /// first i.e. `coeff{n-1}` should be seen at the very end.
    fn inject_ext2_intt_result(&mut self) -> Result<(), ExecutionError> {
        let out_poly_len = self.stack.get(0).as_int() as usize;
        let in_evaluations_len = self.stack.get(1).as_int() as usize;
        let in_evaluations_addr = self.stack.get(2).as_int();

        if in_evaluations_len <= 1 {
            return Err(ExecutionError::NttDomainSizeTooSmall(in_evaluations_len as u64));
        }
        if !in_evaluations_len.is_power_of_two() {
            return Err(ExecutionError::NttDomainSizeNotPowerOf2(in_evaluations_len as u64));
        }
        if out_poly_len > in_evaluations_len {
            return Err(ExecutionError::InterpolationResultSizeTooBig(
                out_poly_len,
                in_evaluations_len,
            ));
        }

        let mut poly = Vec::with_capacity(in_evaluations_len);
        for i in 0..(in_evaluations_len >> 1) {
            let word = self
                .get_memory_value(self.system.ctx(), in_evaluations_addr + i as u64)
                .ok_or_else(|| {
                    ExecutionError::UninitializedMemoryAddress(in_evaluations_addr + i as u64)
                })?;

            poly.push(QuadFelt::new(word[0], word[1]));
            poly.push(QuadFelt::new(word[2], word[3]));
        }

        let twiddles = fft::get_inv_twiddles::<Felt>(in_evaluations_len);
        fft::interpolate_poly::<Felt, QuadFelt>(&mut poly, &twiddles);

        for i in QuadFelt::slice_as_base_elements(&poly[..out_poly_len]).iter().rev().copied() {
            self.advice_provider.push_stack(AdviceSource::Value(i))?;
        }

        Ok(())
    }

    /// Pushes the value and depth flags of a leaf indexed by `key` on a Sparse Merkle tree with
    /// the provided `root`.
    ///
    /// The Sparse Merkle tree is tiered, meaning it will have leaf depths in `{16, 32, 48, 64}`.
    /// The depth flags define the tier on which the leaf is located.
    ///
    /// The operand stack is expected to be arranged as follows:
    /// - key, 4 elements.
    /// - root of the Sparse Merkle tree, 4 elements.
    ///
    /// After a successful operation, the advice stack will look as follows (from the top):
    /// - boolean flag set to `1` if the depth is `16` or `48`.
    /// - boolean flag set to `1` if the depth is `16` or `32`.
    /// - remaining key word; will be zeroed if the tree don't contain a mapped value for the key.
    /// - value word; will be zeroed if the tree don't contain a mapped value for the key.
    /// - boolean flag set to `1` if a remaining key is not zero.
    ///
    /// # Errors
    /// Will return an error if:
    /// - The provided Merkle root doesn't exist on the advice provider
    ///
    /// # Panics
    /// Will panic as unimplemented if the target depth is `64`.
    fn inject_smtget(&mut self) -> Result<(), ExecutionError> {
        // fetch the arguments from the operand stack
        let key = [self.stack.get(3), self.stack.get(2), self.stack.get(1), self.stack.get(0)];
        let root = [self.stack.get(7), self.stack.get(6), self.stack.get(5), self.stack.get(4)];

        let index = &key[3];
        let depth = self.advice_provider.get_leaf_depth(root, &SMT_MAX_TREE_DEPTH, index)?;
        debug_assert!(depth < 65);

        // normalize the depth into one of the tiers. this is not a simple `next_power_of_two`
        // because of `48`. using a lookup table is far more efficient than if/else if/else.
        let depth = SMT_NORMALIZED_DEPTHS[depth as usize];
        if depth == 64 {
            unimplemented!("the functionality is unimplemented for depth 64 as the bottom tier will have a special treatment to embed multiple key/value pairs onto a single node");
        }

        // fetch the node value
        let index = index.as_int() >> (64 - depth);
        let index = Felt::new(index);
        let node = self.advice_provider.get_tree_node(root, &Felt::new(depth as u64), &index)?;

        // set the node value; zeroed if empty sub-tree
        let empty = EmptySubtreeRoots::empty_hashes(64);
        if Word::from(empty[depth as usize]) == node {
            // push zeroes for remaining key, value & empty remaining key flag
            for _ in 0..9 {
                self.advice_provider.push_stack(AdviceSource::Value(ZERO))?;
            }
        } else {
            // push a flag indicating that a remaining key exists
            self.advice_provider.push_stack(AdviceSource::Value(ONE))?;

            // map is expected to contain `node |-> {K', V}`
            self.advice_provider.push_stack(AdviceSource::Map { key: node })?;
        }

        // set the flags
        let is_16_or_32 = if depth == 16 || depth == 32 { ONE } else { ZERO };
        let is_16_or_48 = if depth == 16 || depth == 48 { ONE } else { ZERO };
        self.advice_provider.push_stack(AdviceSource::Value(is_16_or_32))?;
        self.advice_provider.push_stack(AdviceSource::Value(is_16_or_48))?;

        Ok(())
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn u64_to_u32_elements(value: u64) -> (Felt, Felt) {
    let hi = Felt::new(value >> 32);
    let lo = Felt::new((value as u32) as u64);
    (hi, lo)
}
