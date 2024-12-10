use alloc::{
    collections::{btree_map::Entry, BTreeMap},
    vec::Vec,
};

use miden_air::RowIndex;
use vm_core::WORD_SIZE;

use super::{Felt, Word, INIT_MEM_VALUE};
use crate::{ContextId, ExecutionError};

// MEMORY SEGMENT TRACE
// ================================================================================================

/// Memory access trace for a single segment sorted first by address and then by clock cycle.
///
/// A memory segment is an isolated address space accessible from a specific execution context.
/// Within each segment, the memory is word-addressable. That is, four field elements are located
/// at each memory address, and we can read and write elements to/from memory in batches of four.
#[derive(Debug, Default)]
pub struct MemorySegmentTrace(BTreeMap<u32, Vec<MemorySegmentAccess>>);

impl MemorySegmentTrace {
    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a word located at the specified address, or None if the address hasn't been
    /// accessed previously.
    ///
    /// Unlike read() which modifies the memory access trace, this method returns the value at the
    /// specified address (if one exists) without altering the memory access trace.
    pub fn get_value(&self, addr: u32) -> Option<Word> {
        match self.0.get(&addr) {
            Some(addr_trace) => addr_trace.last().map(|access| access.word()),
            None => None,
        }
    }

    /// Returns the entire memory state at the beginning of the specified cycle.
    pub fn get_state_at(&self, clk: RowIndex) -> Vec<(u64, Word)> {
        let mut result: Vec<(u64, Word)> = Vec::new();

        if clk == 0 {
            return result;
        }

        // since we record memory state at the end of a given cycle, to get memory state at the end
        // of a cycle, we need to look at the previous cycle. that is, memory state at the end of
        // the previous cycle is the same as memory state the the beginning of the current cycle.
        let search_clk: u64 = (clk - 1).into();

        for (&addr, addr_trace) in self.0.iter() {
            match addr_trace.binary_search_by(|access| access.clk().as_int().cmp(&search_clk)) {
                Ok(i) => result.push((addr.into(), addr_trace[i].word())),
                Err(i) => {
                    // Binary search finds the index of the data with the specified clock cycle.
                    // Decrement the index to get the trace from the previously accessed clock
                    // cycle to insert into the results.
                    if i > 0 {
                        result.push((addr.into(), addr_trace[i - 1].word()));
                    }
                },
            }
        }

        result
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Returns the element located at the specified address. The memory access is assumed to happen
    /// at the provided clock cycle.
    ///
    /// If the element at the specified address hasn't been previously written to, ZERO is returned.
    ///
    /// # Errors
    /// - Returns an error if the same address is accessed more than once in the same clock cycle.
    pub fn read(&mut self, ctx: ContextId, addr: u32, clk: Felt) -> Result<Felt, ExecutionError> {
        let addr_idx_in_word = addr % WORD_SIZE as u32;
        let addr_first_ele = addr - addr_idx_in_word;

        let word = self.read_word_impl(
            ctx,
            addr_first_ele,
            clk,
            MemoryAccessType::Element { addr_idx_in_word: addr_idx_in_word as u8 },
        )?;

        Ok(word[addr_idx_in_word as usize])
    }

    /// Returns a word located in memory starting at the specified address, which must be word
    /// aligned. The memory access is assumed to happen at the provided clock cycle.
    ///
    /// If the word starting at the specified address hasn't been previously written to, four ZERO
    /// elements are returned. This effectively implies that memory is initialized to ZERO.
    ///
    /// # Errors
    /// - Returns an error if the same address is accessed more than once in the same clock cycle.
    pub fn read_word(
        &mut self,
        ctx: ContextId,
        addr: u32,
        clk: Felt,
    ) -> Result<Word, ExecutionError> {
        self.read_word_impl(ctx, addr, clk, MemoryAccessType::Word)
    }

    /// Writes the element located at the specified address. The memory access is assumed to happen
    /// at the provided clock cycle.
    ///
    /// If the element at the specified address hasn't been previously written to, ZERO is returned.
    ///
    /// # Errors
    /// - Returns an error if the same address is accessed more than once in the same clock cycle.
    pub fn write(
        &mut self,
        ctx: ContextId,
        addr: u32,
        clk: Felt,
        value: Felt,
    ) -> Result<(), ExecutionError> {
        debug_assert!(addr % 4 == 0, "unaligned memory access: {addr}");
        let addr_idx_in_word = addr % WORD_SIZE as u32;
        let addr_first_ele = addr - addr_idx_in_word;

        match self.0.entry(addr) {
            Entry::Vacant(vacant_entry) => {
                let last_word = {
                    let mut last_word = Word::default();
                    last_word[addr_idx_in_word as usize] = value;
                    last_word
                };

                let access = MemorySegmentAccess::new(
                    clk,
                    MemoryOperation::Write,
                    MemoryAccessType::Element { addr_idx_in_word: addr_idx_in_word as u8 },
                    last_word,
                );
                vacant_entry.insert(vec![access]);
                Ok(())
            },
            Entry::Occupied(mut occupied_entry) => {
                let addr_trace = occupied_entry.get_mut();
                if addr_trace.last().expect("empty address trace").clk() == clk {
                    Err(ExecutionError::DuplicateMemoryAccess { ctx, addr: addr_first_ele, clk })
                } else {
                    let new_word = {
                        let mut last_word = addr_trace.last().expect("empty address trace").word();
                        last_word[addr_idx_in_word as usize] = value;

                        last_word
                    };

                    let access = MemorySegmentAccess::new(
                        clk,
                        MemoryOperation::Read,
                        MemoryAccessType::Word,
                        new_word,
                    );
                    addr_trace.push(access);

                    Ok(())
                }
            },
        }
    }

    /// Writes the provided word starting at the specified address. The memory access is assumed to
    /// happen at the provided clock cycle.
    ///
    /// # Errors
    /// - Returns an error if the same address is accessed more than once in the same clock cycle.
    pub fn write_word(
        &mut self,
        ctx: ContextId,
        addr: u32,
        clk: Felt,
        value: Word,
    ) -> Result<(), ExecutionError> {
        debug_assert!(addr % 4 == 0, "unaligned memory access: {addr}");

        // add a memory access to the appropriate address trace; if this is the first time
        // we access this address, initialize address trace.
        let access =
            MemorySegmentAccess::new(clk, MemoryOperation::Write, MemoryAccessType::Word, value);
        match self.0.entry(addr) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(vec![access]);
                Ok(())
            },
            Entry::Occupied(mut occupied_entry) => {
                let addr_trace = occupied_entry.get_mut();
                if addr_trace.last().expect("empty address trace").clk() == clk {
                    Err(ExecutionError::DuplicateMemoryAccess { ctx, addr, clk })
                } else {
                    addr_trace.push(access);
                    Ok(())
                }
            },
        }
    }

    // INNER VALUE ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a reference to the map underlying this memory segment trace.
    pub(super) fn inner(&self) -> &BTreeMap<u32, Vec<MemorySegmentAccess>> {
        &self.0
    }

    /// Returns a map underlying this memory segment trace while consuming self.
    pub(super) fn into_inner(self) -> BTreeMap<u32, Vec<MemorySegmentAccess>> {
        self.0
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    // TODO(plafer): docs
    pub fn read_word_impl(
        &mut self,
        ctx: ContextId,
        addr_first_ele: u32,
        clk: Felt,
        access_type: MemoryAccessType,
    ) -> Result<Word, ExecutionError> {
        debug_assert!(addr_first_ele % 4 == 0, "unaligned word access: {addr_first_ele}");

        // look up the previous value in the appropriate address trace and add (clk, prev_value)
        // to it; if this is the first time we access this word, create address trace for it
        // with entry (clk, [ZERO, 4]). in both cases, return the last value in the address trace.
        match self.0.entry(addr_first_ele) {
            Entry::Vacant(vacant_entry) => {
                let access = MemorySegmentAccess::new(
                    clk,
                    MemoryOperation::Read,
                    access_type,
                    INIT_MEM_VALUE,
                );
                vacant_entry.insert(vec![access]);
                Ok(INIT_MEM_VALUE)
            },
            Entry::Occupied(mut occupied_entry) => {
                let addr_trace = occupied_entry.get_mut();
                if addr_trace.last().expect("empty address trace").clk() == clk {
                    Err(ExecutionError::DuplicateMemoryAccess { ctx, addr: addr_first_ele, clk })
                } else {
                    let last_word = addr_trace.last().expect("empty address trace").word();
                    let access = MemorySegmentAccess::new(
                        clk,
                        MemoryOperation::Read,
                        access_type,
                        last_word,
                    );
                    addr_trace.push(access);

                    Ok(last_word)
                }
            },
        }
    }

    /// Returns current size (in words) of this memory segment.
    #[cfg(test)]
    pub fn size(&self) -> usize {
        self.0.len()
    }
}

// MEMORY ACCESS
// ================================================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryOperation {
    Read,
    Write,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryAccessType {
    Element { addr_idx_in_word: u8 },
    Word,
}

/// A single memory access representing the specified memory operation with the specified value at
/// the specified clock cycle.
#[derive(Copy, Debug, Clone)]
pub struct MemorySegmentAccess {
    clk: Felt,
    operation: MemoryOperation,
    access_type: MemoryAccessType,
    word: Word,
}

impl MemorySegmentAccess {
    fn new(clk: Felt, op: MemoryOperation, access_type: MemoryAccessType, word: Word) -> Self {
        Self { clk, operation: op, access_type, word }
    }

    /// Returns the clock cycle at which this memory access happened.
    pub(super) fn clk(&self) -> Felt {
        self.clk
    }

    pub(super) fn operation(&self) -> MemoryOperation {
        self.operation
    }

    pub(super) fn access_type(&self) -> MemoryAccessType {
        self.access_type
    }

    /// Returns the word value for this memory access.
    pub(super) fn word(&self) -> Word {
        self.word
    }
}
