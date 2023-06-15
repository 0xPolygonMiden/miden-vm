use super::{AdviceProvider, ExecutionError, Process};
use vm_core::{crypto::hash::Rpo256, utils::collections::Vec, Felt, StarkField, WORD_SIZE, ZERO};

// ADVICE INJECTORS
// ================================================================================================

impl<A> Process<A>
where
    A: AdviceProvider,
{
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
    pub(super) fn insert_mem_values_into_adv_map(&mut self) -> Result<(), ExecutionError> {
        let (start_addr, end_addr) = self.get_mem_addr_range(4, 5)?;
        let ctx = self.system.ctx();

        let mut values = Vec::with_capacity(((end_addr - start_addr) as usize) * WORD_SIZE);
        for addr in start_addr..end_addr {
            let mem_value = self.chiplets.get_mem_value(ctx, addr).unwrap_or([ZERO; WORD_SIZE]);
            values.extend_from_slice(&mem_value);
        }

        let key = self.stack.get_word(0);
        self.advice_provider.insert_into_map(key, values)?;

        Ok(())
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
    pub(super) fn insert_hdword_into_adv_map(
        &mut self,
        domain: Felt,
    ) -> Result<(), ExecutionError> {
        // get the top two words from the stack and hash them to compute the key value
        let word0 = self.stack.get_word(0);
        let word1 = self.stack.get_word(1);
        let key = Rpo256::merge_in_domain(&[word1.into(), word0.into()], domain);

        // build a vector of values from the two word and insert it into the advice map under the
        // computed key
        let mut values = Vec::with_capacity(2 * WORD_SIZE);
        values.extend_from_slice(&word1);
        values.extend_from_slice(&word0);
        self.advice_provider.insert_into_map(key.into(), values)
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Reads (start_addr, end_addr) tuple from the specified elements of the operand stack (
    /// without modifying the state of the stack), and verifies that memory range is valid.
    fn get_mem_addr_range(
        &self,
        start_idx: usize,
        end_idx: usize,
    ) -> Result<(u32, u32), ExecutionError> {
        let start_addr = self.stack.get(start_idx).as_int();
        let end_addr = self.stack.get(end_idx).as_int();

        if start_addr > u32::MAX as u64 {
            return Err(ExecutionError::MemoryAddressOutOfBounds(start_addr));
        }
        if end_addr > u32::MAX as u64 {
            return Err(ExecutionError::MemoryAddressOutOfBounds(end_addr));
        }

        if start_addr > end_addr {
            return Err(ExecutionError::InvalidMemoryRange {
                start_addr,
                end_addr,
            });
        }

        Ok((start_addr as u32, end_addr as u32))
    }
}
