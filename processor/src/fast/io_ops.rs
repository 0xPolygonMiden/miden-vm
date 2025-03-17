use vm_core::{utils::range, Word};

use super::{ExecutionError, FastProcessor, Felt, WORD_SIZE};
use crate::{AdviceProvider, Host, ProcessState};

impl FastProcessor {
    pub fn op_push(&mut self, element: Felt) {
        self.stack[self.stack_top_idx] = element;
        self.increment_stack_size();
    }

    pub fn op_advpop(&mut self, op_idx: usize, host: &mut impl Host) -> Result<(), ExecutionError> {
        let value = host.advice_provider_mut().pop_stack(ProcessState::new_fast(self, op_idx))?;
        self.stack[self.stack_top_idx] = value;
        self.increment_stack_size();
        Ok(())
    }

    pub fn op_advpopw(
        &mut self,
        op_idx: usize,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        let word: Word = host
            .advice_provider_mut()
            .pop_stack_word(ProcessState::new_fast(self, op_idx))?;
        self.stack[range(self.stack_top_idx - WORD_SIZE, WORD_SIZE)].copy_from_slice(&word);

        Ok(())
    }

    pub fn op_mloadw(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        {
            let addr = self.stack[self.stack_top_idx - 1];
            let word = self.memory.read_word(self.ctx, addr, self.clk + op_idx)?;

            self.stack[range(self.stack_top_idx - 1 - WORD_SIZE, WORD_SIZE)].copy_from_slice(word);
        }

        self.decrement_stack_size();
        Ok(())
    }

    pub fn op_mstorew(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        {
            let addr = self.stack[self.stack_top_idx - 1];
            let word: [Felt; WORD_SIZE] = self.stack
                [range(self.stack_top_idx - 1 - WORD_SIZE, WORD_SIZE)]
            .try_into()
            .unwrap();

            self.memory.write_word(self.ctx, addr, self.clk + op_idx, word)?;
        }

        self.decrement_stack_size();
        Ok(())
    }

    pub fn op_mload(&mut self) -> Result<(), ExecutionError> {
        let element = {
            let addr = self.stack[self.stack_top_idx - 1];
            self.memory.read_element(self.ctx, addr)?
        };

        self.stack[self.stack_top_idx - 1] = element;

        Ok(())
    }

    pub fn op_mstore(&mut self) -> Result<(), ExecutionError> {
        let addr = self.stack[self.stack_top_idx - 1];
        let value = self.stack[self.stack_top_idx - 2];

        self.memory.write_element(self.ctx, addr, value)?;

        self.decrement_stack_size();
        Ok(())
    }

    pub fn op_mstream(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        // The stack index where the memory address to load the words from is stored.
        let mem_addr_stack_idx: usize = self.stack_top_idx - 1 - 12;

        let addr_first_word = self.stack[mem_addr_stack_idx];
        let addr_second_word = Felt::new(addr_first_word.as_int() + WORD_SIZE as u64);

        // load two words from memory
        let words = [
            self.memory.read_word(self.ctx, addr_first_word, self.clk + op_idx)?,
            self.memory.read_word(self.ctx, addr_second_word, self.clk + op_idx)?,
        ];

        // Replace the stack elements with the elements from memory (in stack order). The word at
        // address `addr + 4` is at the top of the stack.
        {
            let word0_offset = self.stack_top_idx - 2 * WORD_SIZE;
            let word1_offset = self.stack_top_idx - WORD_SIZE;

            self.stack[range(word0_offset, WORD_SIZE)].copy_from_slice(words[0]);
            self.stack[range(word1_offset, WORD_SIZE)].copy_from_slice(words[1]);
        }

        // increment the address by 8 (2 words)
        self.stack[mem_addr_stack_idx] = addr_first_word + Felt::from(2 * WORD_SIZE as u32);

        Ok(())
    }

    pub fn op_pipe(&mut self, op_idx: usize, host: &mut impl Host) -> Result<(), ExecutionError> {
        // The stack index where the memory address to load the words from is stored.
        let mem_addr_stack_idx: usize = self.stack_top_idx - 1 - 12;

        let addr_first_word = self.stack[mem_addr_stack_idx];
        let addr_second_word = Felt::new(addr_first_word.as_int() + WORD_SIZE as u64);

        // pop two words from the advice stack
        let words = host
            .advice_provider_mut()
            .pop_stack_dword(ProcessState::new_fast(self, op_idx))?;

        // write the words to memory
        self.memory.write_word(self.ctx, addr_first_word, self.clk + op_idx, words[0])?;
        self.memory
            .write_word(self.ctx, addr_second_word, self.clk + op_idx, words[1])?;

        // replace the elements on the stack with the word elements (in stack order)
        {
            let word0_offset = self.stack_top_idx - 2 * WORD_SIZE;
            let word1_offset = self.stack_top_idx - WORD_SIZE;

            self.stack[range(word0_offset, WORD_SIZE)].copy_from_slice(&words[0]);
            self.stack[range(word1_offset, WORD_SIZE)].copy_from_slice(&words[1]);
        }

        // increment the address by 8 (2 words)
        self.stack[mem_addr_stack_idx] = addr_first_word + Felt::from(2 * WORD_SIZE as u32);

        Ok(())
    }

    pub fn enforce_word_aligned_addr(
        &self,
        addr: Felt,
        op_idx: usize,
    ) -> Result<u32, ExecutionError> {
        let addr: u64 = addr.as_int();
        let addr: u32 =
            addr.try_into().map_err(|_| ExecutionError::MemoryAddressOutOfBounds(addr))?;

        if addr % WORD_SIZE as u32 != 0 {
            return Err(ExecutionError::MemoryUnalignedWordAccess {
                addr,
                ctx: self.ctx,
                clk: Felt::from(self.clk + op_idx),
            });
        }

        Ok(addr)
    }
}
