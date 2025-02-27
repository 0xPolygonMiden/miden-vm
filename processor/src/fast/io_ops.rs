use vm_core::utils::range;

use super::{ExecutionError, Felt, SpeedyGonzales, WORD_SIZE, ZERO};

impl<const N: usize> SpeedyGonzales<N> {
    pub fn op_push(&mut self, element: Felt) {
        self.stack[self.stack_top_idx] = element;
        self.increment_stack_size();
    }

    pub fn adv_pop(&mut self) -> Result<(), ExecutionError> {
        todo!()
    }

    pub fn adv_popw(&mut self) -> Result<(), ExecutionError> {
        todo!()
    }

    pub fn op_mloadw(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        let addr = {
            let addr: u64 = self.stack[self.stack_top_idx - 1].as_int();
            let addr: u32 =
                addr.try_into().map_err(|_| ExecutionError::MemoryAddressOutOfBounds(addr))?;

            if addr % WORD_SIZE as u32 != 0 {
                return Err(ExecutionError::MemoryUnalignedWordAccess {
                    addr,
                    ctx: self.ctx,
                    clk: Felt::from(self.clk + op_idx),
                });
            }
            addr
        };

        let word = self.memory.get(&(self.ctx, addr)).copied().unwrap_or([ZERO; WORD_SIZE]);

        self.stack[range(self.stack_top_idx - 1 - WORD_SIZE, WORD_SIZE)].copy_from_slice(&word);

        self.decrement_stack_size();
        Ok(())
    }

    pub fn op_mstorew(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        let addr = {
            let addr: u64 = self.stack[self.stack_top_idx - 1].as_int();
            let addr: u32 =
                addr.try_into().map_err(|_| ExecutionError::MemoryAddressOutOfBounds(addr))?;

            if addr % WORD_SIZE as u32 != 0 {
                return Err(ExecutionError::MemoryUnalignedWordAccess {
                    addr,
                    ctx: self.ctx,
                    clk: Felt::from(self.clk + op_idx),
                });
            }
            addr
        };

        let word: [Felt; WORD_SIZE] = self.stack
            [range(self.stack_top_idx - 1 - WORD_SIZE, WORD_SIZE)]
        .try_into()
        .unwrap();

        self.memory.insert((self.ctx, addr), word);

        self.decrement_stack_size();
        Ok(())
    }

    // TODO(plafer): test this
    pub fn op_mload(&mut self) -> Result<(), ExecutionError> {
        let (word_addr, idx) = {
            let addr = self.stack[self.stack_top_idx - 1].as_int();
            let addr: u32 =
                addr.try_into().map_err(|_| ExecutionError::MemoryAddressOutOfBounds(addr))?;

            let idx = addr % WORD_SIZE as u32;

            (addr - idx, idx)
        };
        let word = self.memory.get(&(self.ctx, word_addr)).copied().unwrap_or([ZERO; WORD_SIZE]);

        self.stack[self.stack_top_idx - 1] = word[idx as usize];
        Ok(())
    }

    // TODO(plafer): test this
    pub fn op_mstore(&mut self) -> Result<(), ExecutionError> {
        let (word_addr, idx) = {
            let addr = self.stack[self.stack_top_idx - 1].as_int();
            let addr: u32 =
                addr.try_into().map_err(|_| ExecutionError::MemoryAddressOutOfBounds(addr))?;

            let idx = addr % WORD_SIZE as u32;

            (addr - idx, idx)
        };

        let value = self.stack[self.stack_top_idx - 2];

        self.memory
            .entry((self.ctx, word_addr))
            .and_modify(|word| {
                word[idx as usize] = value;
            })
            .or_insert_with(|| {
                let mut word = [ZERO; WORD_SIZE];
                word[idx as usize] = value;
                word
            });

        self.decrement_stack_size();
        Ok(())
    }

    pub fn op_mstream(&mut self) -> Result<(), ExecutionError> {
        todo!()
    }

    pub fn op_pipe(&mut self) -> Result<(), ExecutionError> {
        todo!()
    }
}
