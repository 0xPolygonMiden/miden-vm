use alloc::collections::BTreeMap;

use miden_air::RowIndex;
use vm_core::{Felt, Word, EMPTY_WORD, WORD_SIZE, ZERO};

use crate::{ContextId, ExecutionError};

#[derive(Debug, Default)]
pub struct Memory {
    /// A map from (context_id, word_address) to the word stored starting at that memory location.
    memory: BTreeMap<(ContextId, u32), [Felt; WORD_SIZE]>,
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads an element from memory at the provided address in the provided context.
    pub fn read_element(&mut self, ctx: ContextId, addr: Felt) -> Result<Felt, ExecutionError> {
        let (word_addr, idx) = split_addr(addr)?;
        let word = self.memory.get(&(ctx, word_addr)).copied().unwrap_or([ZERO; WORD_SIZE]);

        Ok(word[idx as usize])
    }

    /// Reads a word from memory starting at the provided address in the provided context.
    pub fn read_word(
        &self,
        ctx: ContextId,
        addr: Felt,
        clk: RowIndex,
    ) -> Result<&Word, ExecutionError> {
        let addr = enforce_word_aligned_addr(ctx, addr, clk)?;
        let word = self.memory.get(&(ctx, addr)).unwrap_or(&EMPTY_WORD);

        Ok(word)
    }

    /// Writes an element to memory at the provided address in the provided context.
    pub fn write_element(
        &mut self,
        ctx: ContextId,
        addr: Felt,
        element: Felt,
    ) -> Result<(), ExecutionError> {
        let (word_addr, idx) = split_addr(addr)?;

        self.memory
            .entry((ctx, word_addr))
            .and_modify(|word| {
                word[idx as usize] = element;
            })
            .or_insert_with(|| {
                let mut word = [ZERO; WORD_SIZE];
                word[idx as usize] = element;
                word
            });

        Ok(())
    }

    /// Writes a word to memory starting at the provided address in the provided context.
    pub fn write_word(
        &mut self,
        ctx: ContextId,
        addr: Felt,
        clk: RowIndex,
        word: Word,
    ) -> Result<(), ExecutionError> {
        let addr = enforce_word_aligned_addr(ctx, addr, clk)?;
        self.memory.insert((ctx, addr), word);

        Ok(())
    }
}

// HELPERS
// -------------------------------------------------------------------------------------------

/// Splits the provided address into the word address and the index within the word.
///
/// Returns a tuple of the word address and the index within the word.
///
/// # Errors
/// - Returns an error if the provided address is out of bounds.
fn split_addr(addr: Felt) -> Result<(u32, u32), ExecutionError> {
    let addr = addr.as_int();
    let addr: u32 = addr.try_into().map_err(|_| ExecutionError::MemoryAddressOutOfBounds(addr))?;

    let idx = addr % WORD_SIZE as u32;

    Ok((addr - idx, idx))
}

/// Enforces that the provided address is word-aligned; that is, that it be divisible by 4 (in
/// the integer sense).
///
/// Returns the address as a `u32` if it is word-aligned.
///
/// # Errors
/// - Returns an error if the provided address is not word-aligned.
/// - Returns an error if the provided address is out of bounds.
fn enforce_word_aligned_addr(
    ctx: ContextId,
    addr: Felt,
    clk: RowIndex,
) -> Result<u32, ExecutionError> {
    let addr: u64 = addr.as_int();
    let addr: u32 = addr.try_into().map_err(|_| ExecutionError::MemoryAddressOutOfBounds(addr))?;

    if addr % WORD_SIZE as u32 != 0 {
        return Err(ExecutionError::MemoryUnalignedWordAccess { addr, ctx, clk: clk.into() });
    }

    Ok(addr)
}
