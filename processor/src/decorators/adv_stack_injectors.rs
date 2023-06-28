use super::{super::Ext2InttError, AdviceProvider, AdviceSource, ExecutionError, Process};
use vm_core::{
    crypto::merkle::EmptySubtreeRoots, utils::collections::Vec, Felt, FieldElement, QuadExtension,
    StarkField, Word, ONE, ZERO,
};
use winter_prover::math::fft;

// TYPE ALIASES
// ================================================================================================
type QuadFelt = QuadExtension<Felt>;

// CONSTANTS
// ================================================================================================

/// Maximum depth of a Sparse Merkle Tree
const SMT_MAX_TREE_DEPTH: Felt = Felt::new(64);

/// Lookup table for Sparse Merkle Tree depth normalization
const SMT_NORMALIZED_DEPTHS: [u8; 65] = [
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 32, 32, 32, 32, 32, 32, 32,
    32, 32, 32, 32, 32, 32, 32, 32, 32, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
    48, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
];

// ADVICE INJECTORS
// ================================================================================================

impl<A> Process<A>
where
    A: AdviceProvider,
{
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
    /// - The specified depth is either zero or greater than the depth of the Merkle tree
    ///   identified by the specified root.
    /// - Value of the node at the specified depth and index is not known to the advice provider.
    pub(super) fn copy_merkle_node_to_adv_stack(&mut self) -> Result<(), ExecutionError> {
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
    pub(super) fn copy_map_value_to_adv_stack(
        &mut self,
        include_len: bool,
        key_offset: usize,
    ) -> Result<(), ExecutionError> {
        if key_offset > 12 {
            return Err(ExecutionError::InvalidStackWordOffset(key_offset));
        }

        let key = [
            self.stack.get(key_offset + 3),
            self.stack.get(key_offset + 2),
            self.stack.get(key_offset + 1),
            self.stack.get(key_offset),
        ];
        self.advice_provider.push_stack(AdviceSource::Map { key, include_len })
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
    pub(super) fn push_u64_div_result(&mut self) -> Result<(), ExecutionError> {
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
    pub(super) fn push_ext2_inv_result(&mut self) -> Result<(), ExecutionError> {
        let coef0 = self.stack.get(1);
        let coef1 = self.stack.get(0);

        let element = QuadFelt::new(coef0, coef1);
        if element == QuadFelt::ZERO {
            return Err(ExecutionError::DivideByZero(self.system.clk()));
        }
        let result = element.inv().to_base_elements();

        self.advice_provider.push_stack(AdviceSource::Value(result[1]))?;
        self.advice_provider.push_stack(AdviceSource::Value(result[0]))?;

        Ok(())
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
    /// - `input_size` is the number of evaluations (each evaluation is 2 base field elements).
    ///   Must be a power of 2 and greater 1.
    /// - `output_size` is the number of coefficients in the interpolated polynomial (each
    ///   coefficient is 2 base field elements). Must be smaller than or equal to the number of
    ///   input evaluations.
    /// - `input_start_ptr` is the memory address of the first evaluation.
    /// - `coefficients` are the coefficients of the interpolated polynomial such that lowest
    ///   degree coefficients are located at the top of the advice stack.
    ///
    /// # Errors
    /// Returns an error if:
    /// - `input_size` less than or equal to 1, or is not a power of 2.
    /// - `output_size` is 0 or is greater than the `input_size`.
    /// - `input_ptr` is greater than 2^32.
    /// - `input_ptr + input_size / 2` is greater than 2^32.
    pub(super) fn push_ext2_intt_result(&mut self) -> Result<(), ExecutionError> {
        let output_size = self.stack.get(0).as_int() as usize;
        let input_size = self.stack.get(1).as_int() as usize;
        let input_start_ptr = self.stack.get(2).as_int();

        if input_size <= 1 {
            return Err(Ext2InttError::DomainSizeTooSmall(input_size as u64).into());
        }
        if !input_size.is_power_of_two() {
            return Err(Ext2InttError::DomainSizeNotPowerOf2(input_size as u64).into());
        }
        if input_start_ptr >= u32::MAX as u64 {
            return Err(Ext2InttError::InputStartAddressTooBig(input_start_ptr).into());
        }
        if input_size > u32::MAX as usize {
            return Err(Ext2InttError::InputSizeTooBig(input_size as u64).into());
        }

        let input_end_ptr = input_start_ptr + (input_size / 2) as u64;
        if input_end_ptr > u32::MAX as u64 {
            return Err(Ext2InttError::InputEndAddressTooBig(input_end_ptr).into());
        }

        if output_size == 0 {
            return Err(Ext2InttError::OutputSizeIsZero.into());
        }
        if output_size > input_size {
            return Err(Ext2InttError::OutputSizeTooBig(output_size, input_size).into());
        }

        let mut poly = Vec::with_capacity(input_size);
        for addr in (input_start_ptr as u32)..(input_end_ptr as u32) {
            let word = self
                .get_memory_value(self.system.ctx(), addr)
                .ok_or(Ext2InttError::UninitializedMemoryAddress(addr))?;

            poly.push(QuadFelt::new(word[0], word[1]));
            poly.push(QuadFelt::new(word[2], word[3]));
        }

        let twiddles = fft::get_inv_twiddles::<Felt>(input_size);
        fft::interpolate_poly::<Felt, QuadFelt>(&mut poly, &twiddles);

        for element in QuadFelt::slice_as_base_elements(&poly[..output_size]).iter().rev() {
            self.advice_provider.push_stack(AdviceSource::Value(*element))?;
        }

        Ok(())
    }

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
    ///
    /// # Errors
    /// Will return an error if:
    /// - The provided Merkle root doesn't exist on the advice provider
    ///
    /// # Panics
    /// Will panic as unimplemented if the target depth is `64`.
    pub(super) fn push_smtget_inputs(&mut self) -> Result<(), ExecutionError> {
        // fetch the arguments from the operand stack
        let key = self.stack.get_word(0);
        let root = self.stack.get_word(1);

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
            self.advice_provider.push_stack(AdviceSource::Map {
                key: node,
                include_len: false,
            })?;
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
