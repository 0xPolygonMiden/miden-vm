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

    /// Returns the element located at the specified address, or None if the address hasn't been
    /// accessed previously.
    ///
    /// Unlike read() which modifies the memory access trace, this method returns the value at the
    /// specified address (if one exists) without altering the memory access trace.
    pub fn get_value(&self, addr: u32) -> Option<Felt> {
        let (batch, addr_idx_in_word) = addr_to_batch_and_idx(addr);

        match self.0.get(&batch) {
            Some(addr_trace) => {
                addr_trace.last().map(|access| access.batch()[addr_idx_in_word as usize])
            },
            None => None,
        }
    }

    /// Returns the word located in memory starting at the specified address, which must be word
    /// aligned.
    ///
    /// # Errors
    /// - Returns an error if `addr` is not word aligned.
    pub fn get_word(&self, addr: u32) -> Result<Option<Word>, ()> {
        if addr % WORD_SIZE as u32 != 0 {
            return Err(());
        }

        let (batch, _) = addr_to_batch_and_idx(addr);

        match self.0.get(&batch) {
            Some(addr_trace) => Ok(addr_trace.last().map(|access| access.batch())),
            None => Ok(None),
        }
    }

    /// Returns the entire memory state at the beginning of the specified cycle.
    pub fn get_state_at(&self, clk: RowIndex) -> Vec<(u64, Felt)> {
        let mut result: Vec<(u64, Felt)> = Vec::new();

        if clk == 0 {
            return result;
        }

        // since we record memory state at the end of a given cycle, to get memory state at the end
        // of a cycle, we need to look at the previous cycle. that is, memory state at the end of
        // the previous cycle is the same as memory state the the beginning of the current cycle.
        let search_clk: u64 = (clk - 1).into();

        for (&addr, addr_trace) in self.0.iter() {
            match addr_trace.binary_search_by(|access| access.clk().as_int().cmp(&search_clk)) {
                Ok(i) => {
                    let batch = addr_trace[i].batch();
                    let addr: u64 = addr.into();
                    result.extend([
                        (addr, batch[0]),
                        (addr + 1, batch[1]),
                        (addr + 2, batch[2]),
                        (addr + 3, batch[3]),
                    ]);
                },
                Err(i) => {
                    // Binary search finds the index of the data with the specified clock cycle.
                    // Decrement the index to get the trace from the previously accessed clock
                    // cycle to insert into the results.
                    if i > 0 {
                        let batch = addr_trace[i - 1].batch();
                        let addr: u64 = addr.into();
                        result.extend([
                            (addr, batch[0]),
                            (addr + 1, batch[1]),
                            (addr + 2, batch[2]),
                            (addr + 3, batch[3]),
                        ]);
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
        let (batch, addr_idx_in_word) = addr_to_batch_and_idx(addr);

        let batch_values = self.read_batch(
            ctx,
            batch,
            clk,
            MemoryAccessType::Element { addr_idx_in_batch: addr_idx_in_word },
        )?;

        Ok(batch_values[addr_idx_in_word as usize])
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
        debug_assert!(addr % 4 == 0, "unaligned word access: {addr}");

        let (batch, _) = addr_to_batch_and_idx(addr);
        self.read_batch(ctx, batch, clk, MemoryAccessType::Word)
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
        let (batch, addr_idx_in_word) = addr_to_batch_and_idx(addr);

        match self.0.entry(batch) {
            Entry::Vacant(vacant_entry) => {
                // If this is the first access to the ctx/batch pair, then all values in the batch
                // are initialized to 0, except for the address being written.
                let batch = {
                    let mut batch = Word::default();
                    batch[addr_idx_in_word as usize] = value;
                    batch
                };

                let access = MemorySegmentAccess::new(
                    clk,
                    MemoryOperation::Write,
                    MemoryAccessType::Element { addr_idx_in_batch: addr_idx_in_word },
                    batch,
                );
                vacant_entry.insert(vec![access]);
                Ok(())
            },
            Entry::Occupied(mut occupied_entry) => {
                // If the ctx/batch pair has been accessed before, then the values in the batch are
                // the same as the previous access, except for the address being written.
                let addr_trace = occupied_entry.get_mut();
                if addr_trace.last().expect("empty address trace").clk() == clk {
                    Err(ExecutionError::DuplicateMemoryAccess { ctx, addr, clk })
                } else {
                    let batch = {
                        let mut last_batch =
                            addr_trace.last().expect("empty address trace").batch();
                        last_batch[addr_idx_in_word as usize] = value;

                        last_batch
                    };

                    let access = MemorySegmentAccess::new(
                        clk,
                        MemoryOperation::Write,
                        MemoryAccessType::Element { addr_idx_in_batch: addr_idx_in_word },
                        batch,
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
        word: Word,
    ) -> Result<(), ExecutionError> {
        debug_assert!(addr % 4 == 0, "unaligned memory access: {addr}");

        let (batch, _) = addr_to_batch_and_idx(addr);

        let access =
            MemorySegmentAccess::new(clk, MemoryOperation::Write, MemoryAccessType::Word, word);
        match self.0.entry(batch) {
            Entry::Vacant(vacant_entry) => {
                // All values in the batch are set to the word being written.
                vacant_entry.insert(vec![access]);
                Ok(())
            },
            Entry::Occupied(mut occupied_entry) => {
                // All values in the batch are set to the word being written.
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

    /// Records a read operation on the specified batch at the specified clock cycle.
    ///
    /// The access type either specifies the element in batch that was read, or that the entire word
    /// was read.
    fn read_batch(
        &mut self,
        ctx: ContextId,
        batch: u32,
        clk: Felt,
        access_type: MemoryAccessType,
    ) -> Result<Word, ExecutionError> {
        match self.0.entry(batch) {
            Entry::Vacant(vacant_entry) => {
                // If this is the first access to the ctx/batch pair, then all values in the batch
                // are initialized to 0.
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
                // If the ctx/batch pair has been accessed before, then the values in the batch are
                // the same as the previous access.
                let addr_trace = occupied_entry.get_mut();
                if addr_trace.last().expect("empty address trace").clk() == clk {
                    Err(ExecutionError::DuplicateMemoryAccess { ctx, addr: batch, clk })
                } else {
                    let last_batch = addr_trace.last().expect("empty address trace").batch();
                    let access = MemorySegmentAccess::new(
                        clk,
                        MemoryOperation::Read,
                        access_type,
                        last_batch,
                    );
                    addr_trace.push(access);

                    Ok(last_batch)
                }
            },
        }
    }

    /// Returns the number of batches that were accessed at least once.
    #[cfg(test)]
    pub fn num_accessed_batches(&self) -> usize {
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
    Element { addr_idx_in_batch: u8 },
    Word,
}

/// A single memory access representing the specified memory operation with the specified value at
/// the specified clock cycle.
#[derive(Copy, Debug, Clone)]
pub struct MemorySegmentAccess {
    clk: Felt,
    operation: MemoryOperation,
    access_type: MemoryAccessType,
    batch: Word,
}

impl MemorySegmentAccess {
    fn new(clk: Felt, op: MemoryOperation, access_type: MemoryAccessType, batch: Word) -> Self {
        Self { clk, operation: op, access_type, batch }
    }

    /// Returns the clock cycle at which this memory access happened.
    pub(super) fn clk(&self) -> Felt {
        self.clk
    }

    /// Returns the operation associated with this memory access.
    pub(super) fn operation(&self) -> MemoryOperation {
        self.operation
    }

    /// Returns the access type associated with this memory access.
    pub(super) fn access_type(&self) -> MemoryAccessType {
        self.access_type
    }

    /// Returns the batch associated with this memory access.
    ///
    /// For example, if the memory access is an element read of address 42, the batch will contain
    /// the values of addresses 40, 41, 42, and 43.
    pub(super) fn batch(&self) -> Word {
        self.batch
    }
}

// HELPERS
// ================================================================================================

/// Splits an address into two components:
/// 1. a batch, which is the closest value to `addr` that is both smaller and word aligned,  and
/// 2. the index within the batch which `addr` represents.
pub fn addr_to_batch_and_idx(addr: u32) -> (u32, u8) {
    let idx = addr % WORD_SIZE as u32;
    (addr - idx, idx as u8)
}
