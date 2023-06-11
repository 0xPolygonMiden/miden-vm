use super::{AdviceProvider, ExecutionError, Process};
use vm_core::{StarkField, WORD_SIZE, ZERO};

// ADVICE INJECTORS
// ================================================================================================

impl<A> Process<A>
where
    A: AdviceProvider,
{
    /// Reads the specified number of words from the memory starting at the given start address and
    /// writes the vector of field elements to the advice map with the top 4 elements on the stack
    /// as the key. This operation does not affect the state of the Memory chiplet and the VM in
    /// general.
    ///
    /// # Errors
    /// Returns an error if the key is already present in the advice map.
    pub(super) fn inject_mem_values(&mut self) -> Result<(), ExecutionError> {
        let (start_addr, end_addr) = self.get_mem_addr_range();
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

    //pub(super) fn inject_stack_dword(&mut self, domain: Felt) -> Result<(), ExecutionError> {
    //    // get the top two words from the stack and hash them to compute the key value
    //    let word1 = self.stack.get_word(1);
    //    let word2 = self.stack.get_word(0);
    //    let key = Rpo256::merge_in_domain(&[word1.into(), word2.into()], domain);

    //    // build a vector of values from the two word and insert into the advice map under the
    //    // computed key
    //    let mut values = Vec::with_capacity(2 * WORD_SIZE);
    //    values.extend_from_slice(&word1);
    //    values.extend_from_slice(&word2);
    //    self.advice_provider.insert_into_map(key.into(), values)
    //}

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    ///
    fn get_mem_addr_range(&self) -> (u64, u64) {
        let start_addr = self.stack.get(4).as_int();
        let end_addr = self.stack.get(5).as_int();

        (start_addr, end_addr)
    }
}
