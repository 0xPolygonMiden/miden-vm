use alloc::vec::Vec;

use miden_air::{RowIndex, trace::chiplets::hasher::HasherState};
use vm_core::{Kernel, mast::OpBatch};

use super::{
    CHIPLETS_WIDTH, ChipletsTrace, EMPTY_WORD, ExecutionError, Felt, FieldElement, ONE,
    RangeChecker, TraceFragment, Word, ZERO, crypto::MerklePath, utils,
};

mod bitwise;
use bitwise::Bitwise;

mod hasher;
use hasher::Hasher;
#[cfg(test)]
pub(crate) use hasher::init_state_from_words;

mod memory;
use memory::Memory;
pub use memory::MemoryError;

mod ace;
use ace::AceHints;
pub use ace::{
    Ace, CircuitEvaluation, MAX_NUM_ACE_WIRES, PTR_OFFSET_ELEM, PTR_OFFSET_WORD, eval_circuit,
};

mod kernel_rom;
use kernel_rom::KernelRom;

mod aux_trace;

pub(crate) use aux_trace::AuxTraceBuilder;

#[cfg(test)]
mod tests;

// CHIPLETS MODULE OF HASHER, BITWISE, MEMORY, ACE, AND KERNEL ROM CHIPLETS
// ================================================================================================

/// This module manages the VM's hasher, bitwise, memory, arithmetic circuit evaluation (ACE)
/// and kernel ROM chiplets and is responsible for building a final execution trace from their
/// stacked execution traces and chiplet selectors.
///
/// The module's trace can be thought of as 6 stacked segments in the following form:
///
/// * Hasher segment: contains the trace and selector for the hasher chiplet. This segment fills the
///   first rows of the trace up to the length of the hasher `trace_len`.
///   - column 0: selector column with values set to ZERO
///   - columns 1-16: execution trace of hash chiplet
///   - columns 17-20: unused columns padded with ZERO
///
/// * Bitwise segment: contains the trace and selectors for the bitwise chiplet. This segment begins
///   at the end of the hasher segment and fills the next rows of the trace for the `trace_len` of
///   the bitwise chiplet.
///   - column 0: selector column with values set to ONE
///   - column 1: selector column with values set to ZERO
///   - columns 2-14: execution trace of bitwise chiplet
///   - columns 15-20: unused columns padded with ZERO
///
/// * Memory segment: contains the trace and selectors for the memory chiplet.  This segment begins
///   at the end of the bitwise segment and fills the next rows of the trace for the `trace_len` of
///   the memory chiplet.
///   - column 0-1: selector columns with values set to ONE
///   - column 2: selector column with values set to ZERO
///   - columns 3-17: execution trace of memory chiplet
///   - columns 18-20: unused columns padded with ZERO
///
/// * ACE segment: contains the trace and selectors for the arithmetic circuit evaluation chiplet.
///   This segment begins at the end of the memory segment and fills the next rows of the trace for
///   the `trace_len` of the ACE chiplet.
///   - column 0-2: selector columns with values set to ONE
///   - column 3: selector column with values set to ZERO
///   - columns 4-20: execution trace of ACE chiplet
///
/// * Kernel ROM segment: contains the trace and selectors for the kernel ROM chiplet * This segment
///   begins at the end of the memory segment and fills the next rows of the trace for the
///   `trace_len` of the kernel ROM chiplet.
///   - column 0-3: selector columns with values set to ONE
///   - column 4: selector column with values set to ZERO
///   - columns 5-9: execution trace of kernel ROM chiplet
///   - columns 10-20: unused column padded with ZERO
///
/// * Padding segment: unused. This segment begins at the end of the kernel ROM segment and fills
///   the rest of the execution trace minus the number of random rows. When it finishes, the
///   execution trace should have exactly enough rows remaining for the specified number of random
///   rows.
///   - columns 0-4: selector columns with values set to ONE
///   - columns 5-20: unused columns padded with ZERO
///
///
/// The following is a pictorial representation of the chiplet module:
///
/// ```text
///             +---+--------------------------------------------------------------+------+
///             | 0 |                                                              |------|
///             | . |         Hash chiplet                                         |------|
///             | . |         16 columns                                           |------|
///             | . |       constraint degree 8                                    |------|
///             | 0 |                                                              |------|
///             +---+---+------------------------------------------------------+---+------+
///             | 1 | 0 |                                                      |----------|
///             | . | . |                  Bitwise chiplet                     |----------|
///             | . | . |                    13 columns                        |----------|
///             | . | . |               constraint degree 5                    |----------|
///             | . | . |                                                      |----------|
///             | . | 0 |                                                      |----------|
///             | . +---+---+--------------------------------------------------+-----+----+
///             | . | 1 | 0 |                                                        |----|
///             | . | . | . |            Memory chiplet                              |----|
///             | . | . | . |              15 columns                                |----|
///             | . | . | . |          constraint degree 9                           |----|
///             | . | . | 0 |                                                        |----|
///             | . + . +---+---+----------------------------------------------------+----+
///             | . | . | 1 | 0 |                                                         |
///             | . | . | . | . |          ACE chiplet                                    |
///             | . | . | . | . |            16 columns                                   |   
///             | . | . | . | . |        constraint degree 5                              |
///             | . | . | . | 0 |                                                         |
///             | . + . | . +---+---+---------------------------+-------------------------+
///             | . | . | . | 1 | 0 |                           |-------------------------|
///             | . | . | . | . | . |     Kernel ROM chiplet    |-------------------------|
///             | . | . | . | . | . |     5 columns             |-------------------------|
///             | . | . | . | . | . |     constraint degree 9   |-------------------------|
///             | . | . | . | . | 0 |                           |-------------------------|
///             | . + . | . | . +---+---+-----------------------+-------------------------+
///             | . | . | . | . | 1 | 0 |-------------------------------------------------|
///             | . | . | . | . | . | . |-------------------------------------------------|
///             | . | . | . | . | . | . |-------------------------------------------------|
///             | . | . | . | . | . | . |-------------------------------------------------|
///             | . | . | . | . | . | . |-------------------- Padding --------------------|
///             | . + . | . | . | . | . |-------------------------------------------------|
///             | . | . | . | . | . | . |-------------------------------------------------|
///             | . | . | . | . | . | . |-------------------------------------------------|
///             | . | . | . | . | . | . |-------------------------------------------------|
///             | 1 | 1 | 1 | 1 | 1 | 0 |-------------------------------------------------|
///             +---+---+---+---+---------------------------------------------------------+
/// ```
#[derive(Debug)]
pub struct Chiplets {
    pub hasher: Hasher,
    pub bitwise: Bitwise,
    pub memory: Memory,
    pub ace: Ace,
    pub kernel_rom: KernelRom,
}

impl Chiplets {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Chiplets] component instantiated with the provided Kernel.
    pub fn new(kernel: Kernel) -> Self {
        Self {
            hasher: Hasher::default(),
            bitwise: Bitwise::default(),
            memory: Memory::default(),
            kernel_rom: KernelRom::new(kernel),
            ace: Ace::default(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the length of the trace required to accommodate chiplet components and 1
    /// mandatory padding row required for ensuring sufficient trace length for auxiliary connector
    /// columns that rely on the memory chiplet.
    pub fn trace_len(&self) -> usize {
        self.hasher.trace_len()
            + self.bitwise.trace_len()
            + self.memory.trace_len()
            + self.kernel_rom.trace_len()
            + self.ace.trace_len()
            + 1
    }

    /// Returns the index of the first row of [Bitwise] execution trace.
    pub fn bitwise_start(&self) -> RowIndex {
        self.hasher.trace_len().into()
    }

    /// Returns the index of the first row of the [Memory] execution trace.
    pub fn memory_start(&self) -> RowIndex {
        self.bitwise_start() + self.bitwise.trace_len()
    }

    /// Returns the index of the first row of [KernelRom] execution trace.
    pub fn ace_start(&self) -> RowIndex {
        self.memory_start() + self.memory.trace_len()
    }

    /// Returns the index of the first row of [KernelRom] execution trace.
    pub fn kernel_rom_start(&self) -> RowIndex {
        self.ace_start() + self.ace.trace_len()
    }

    /// Returns the index of the first row of the padding section of the execution trace.
    pub fn padding_start(&self) -> RowIndex {
        self.kernel_rom_start() + self.kernel_rom.trace_len()
    }

    // EXECUTION TRACE
    // --------------------------------------------------------------------------------------------

    /// Adds all range checks required by the memory chiplet to the provided [RangeChecker]
    /// instance.
    pub fn append_range_checks(&self, range_checker: &mut RangeChecker) {
        self.memory.append_range_checks(self.memory_start(), range_checker);
    }

    /// Returns an execution trace of the chiplets containing the stacked traces of the
    /// Hasher, Bitwise, ACE, Memory chiplets, and kernel ROM chiplet.
    ///
    /// `num_rand_rows` indicates the number of rows at the end of the trace which will be
    /// overwritten with random values.
    pub fn into_trace(self, trace_len: usize, num_rand_rows: usize) -> ChipletsTrace {
        // make sure that only padding rows will be overwritten by random values
        assert!(self.trace_len() + num_rand_rows <= trace_len, "target trace length too small");

        let kernel = self.kernel_rom.kernel().clone();

        // Allocate columns for the trace of the chiplets.
        let mut trace = (0..CHIPLETS_WIDTH)
            .map(|_| vec![Felt::ZERO; trace_len])
            .collect::<Vec<_>>()
            .try_into()
            .expect("failed to convert vector to array");
        let ace_hint = self.fill_trace(&mut trace);

        ChipletsTrace {
            trace,
            aux_builder: AuxTraceBuilder::new(kernel, ace_hint),
        }
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Fills the provided trace for the chiplets module with the stacked execution traces of the
    /// Hasher, Bitwise, Memory, ACE, and kernel ROM chiplets along with selector columns
    /// to identify each individual chiplet trace in addition to padding to fill the rest of
    /// the trace.
    fn fill_trace(self, trace: &mut [Vec<Felt>; CHIPLETS_WIDTH]) -> AceHints {
        // get the rows where:usize  chiplets begin.
        let bitwise_start: usize = self.bitwise_start().into();
        let memory_start: usize = self.memory_start().into();
        let ace_start: usize = self.ace_start().into();
        let kernel_rom_start: usize = self.kernel_rom_start().into();
        let padding_start: usize = self.padding_start().into();

        let Chiplets { hasher, bitwise, memory, kernel_rom, ace } = self;

        // populate external selector columns for all chiplets
        trace[0][bitwise_start..].fill(ONE);
        trace[1][memory_start..].fill(ONE);
        trace[2][ace_start..].fill(ONE);
        trace[3][kernel_rom_start..].fill(ONE);
        trace[4][padding_start..].fill(ONE);

        // allocate fragments to be filled with the respective execution traces of each chiplet
        let mut hasher_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut bitwise_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut memory_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut ace_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut kernel_rom_fragment = TraceFragment::new(CHIPLETS_WIDTH);

        // add the hasher, bitwise, memory, ACE, and kernel ROM segments to their respective
        // fragments so they can be filled with the chiplet traces
        for (column_num, column) in trace.iter_mut().enumerate().skip(1) {
            match column_num {
                1 => {
                    // column 1 is relevant only for the hasher
                    hasher_fragment.push_column_slice(column, hasher.trace_len());
                },
                2 => {
                    // column 2 is relevant to the hasher and to bitwise chiplet
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    bitwise_fragment.push_column_slice(rest, bitwise.trace_len());
                },
                3 => {
                    // column 3 is relevant for hasher, bitwise, and memory chiplets
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    let rest = bitwise_fragment.push_column_slice(rest, bitwise.trace_len());
                    memory_fragment.push_column_slice(rest, memory.trace_len());
                },
                4 | 10..=14 => {
                    // columns 4 - 10 to 14 are relevant for hasher, bitwise, memory chiplets and
                    // ace chiplet
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    let rest = bitwise_fragment.push_column_slice(rest, bitwise.trace_len());
                    let rest = memory_fragment.push_column_slice(rest, memory.trace_len());
                    ace_fragment.push_column_slice(rest, ace.trace_len());
                },
                5..=9 => {
                    // columns 5 - 9 are relevant to all chiplets
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    let rest = bitwise_fragment.push_column_slice(rest, bitwise.trace_len());
                    let rest = memory_fragment.push_column_slice(rest, memory.trace_len());
                    let rest = ace_fragment.push_column_slice(rest, ace.trace_len());
                    kernel_rom_fragment.push_column_slice(rest, kernel_rom.trace_len());
                },
                15 | 16 => {
                    // columns 15 and 16 are relevant only for the hasher, memory and ace chiplets
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    // skip bitwise chiplet
                    let (_, rest) = rest.split_at_mut(bitwise.trace_len());
                    let rest = memory_fragment.push_column_slice(rest, memory.trace_len());
                    ace_fragment.push_column_slice(rest, ace.trace_len());
                },
                17 => {
                    // column 17 is relevant only for the memory chiplet
                    // skip the hasher and bitwise chiplets
                    let (_, rest) = column.split_at_mut(hasher.trace_len() + bitwise.trace_len());
                    let rest = memory_fragment.push_column_slice(rest, memory.trace_len());
                    ace_fragment.push_column_slice(rest, ace.trace_len());
                },
                18 | 19 => {
                    // column 18 and 19 are relevant only for the ACE chiplet
                    // skip the hasher, bitwise and memory chiplets
                    let (_, rest) = column.split_at_mut(
                        hasher.trace_len() + bitwise.trace_len() + memory.trace_len(),
                    );
                    ace_fragment.push_column_slice(rest, ace.trace_len());
                },
                _ => panic!("invalid column index"),
            }
        }

        // fill the fragments with the execution trace from each chiplet
        // TODO: this can be parallelized to fill the traces in multiple threads
        hasher.fill_trace(&mut hasher_fragment);
        bitwise.fill_trace(&mut bitwise_fragment);
        memory.fill_trace(&mut memory_fragment);
        kernel_rom.fill_trace(&mut kernel_rom_fragment);
        let ace_sections = ace.fill_trace(&mut ace_fragment);
        AceHints::new(ace_start, ace_sections)
    }
}

// HELPER STRUCTS
// ================================================================================================

/// Result of a Merkle tree node update. The result contains the old Merkle_root, which
/// corresponding to the old_value, and the new merkle_root, for the updated value. As well as the
/// row address of the execution trace at which the computation started.
#[derive(Debug, Copy, Clone)]
pub struct MerkleRootUpdate {
    address: Felt,
    old_root: Word,
    new_root: Word,
}

impl MerkleRootUpdate {
    pub fn get_address(&self) -> Felt {
        self.address
    }
    pub fn get_old_root(&self) -> Word {
        self.old_root
    }
    pub fn get_new_root(&self) -> Word {
        self.new_root
    }
}
