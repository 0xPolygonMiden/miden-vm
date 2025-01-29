use alloc::vec::Vec;

use miden_air::{
    trace::chiplets::{
        bitwise::TRACE_WIDTH as BITWISE_TRACE_WIDTH,
        hasher::{Digest, HasherState, TRACE_WIDTH as HASHER_TRACE_WIDTH},
        kernel_rom::TRACE_WIDTH as KERNEL_ROM_TRACE_WIDTH,
        memory::TRACE_WIDTH as MEMORY_TRACE_WIDTH,
    },
    RowIndex,
};
use vm_core::{mast::OpBatch, Kernel};

use super::{
    crypto::MerklePath, utils, ChipletsTrace, ExecutionError, Felt, FieldElement, RangeChecker,
    TraceFragment, Word, CHIPLETS_WIDTH, EMPTY_WORD, ONE, ZERO,
};

mod bitwise;
use bitwise::Bitwise;

mod hasher;
#[cfg(test)]
pub(crate) use hasher::init_state_from_words;
use hasher::Hasher;

mod memory;
use memory::Memory;

mod kernel_rom;
use kernel_rom::KernelRom;

mod aux_trace;

pub(crate) use aux_trace::AuxTraceBuilder;

#[cfg(test)]
mod tests;

// CHIPLETS MODULE OF HASHER, BITWISE, MEMORY, AND KERNEL ROM CHIPLETS
// ================================================================================================

/// This module manages the VM's hasher, bitwise, memory, and kernel ROM chiplets and is
/// responsible for building a final execution trace from their stacked execution traces and
/// chiplet selectors.
///
/// The module's trace can be thought of as 5 stacked chiplet segments in the following form:
/// * Hasher segment: contains the trace and selector for the hasher chiplet. This segment fills the
///   first rows of the trace up to the length of the hasher `trace_len`.
///   - column 0: selector column with values set to ZERO
///   - columns 1-16: execution trace of hash chiplet
///   - column 17: unused column padded with ZERO
/// * Bitwise segment: contains the trace and selectors for the bitwise chiplet. This segment begins
///   at the end of the hasher segment and fills the next rows of the trace for the `trace_len` of
///   the bitwise chiplet.
///   - column 0: selector column with values set to ONE
///   - column 1: selector column with values set to ZERO
///   - columns 2-14: execution trace of bitwise chiplet
///   - columns 15-17: unused columns padded with ZERO
/// * Memory segment: contains the trace and selectors for the memory chiplet.  This segment begins
///   at the end of the bitwise segment and fills the next rows of the trace for the `trace_len` of
///   the memory chiplet.
///   - column 0-1: selector columns with values set to ONE
///   - column 2: selector column with values set to ZERO
///   - columns 3-17: execution trace of memory chiplet
/// * Kernel ROM segment: contains the trace and selectors for the kernel ROM chiplet * This segment
///   begins at the end of the memory segment and fills the next rows of the trace for the
///   `trace_len` of the kernel ROM chiplet.
///   - column 0-2: selector columns with values set to ONE
///   - column 3: selector column with values set to ZERO
///   - columns 4-9: execution trace of kernel ROM chiplet
///   - columns 10-17: unused column padded with ZERO
/// * Padding segment: unused. This segment begins at the end of the kernel ROM segment and fills
///   the rest of the execution trace minus the number of random rows. When it finishes, the
///   execution trace should have exactly enough rows remaining for the specified number of random
///   rows.
///   - columns 0-3: selector columns with values set to ONE
///   - columns 3-17: unused columns padded with ZERO
///
/// The following is a pictorial representation of the chiplet module:
/// ```text
///             +---+-------------------------------------------------------+-------------+
///             | 0 |                   |                                   |-------------|
///             | . |  Hash chiplet     |       Hash chiplet                |-------------|
///             | . |  internal         |       16 columns                  |-- Padding --|
///             | . |  selectors        |       constraint degree 8         |-------------|
///             | 0 |                   |                                   |-------------|
///             +---+---+---------------------------------------------------+-------------+
///             | 1 | 0 |               |                                   |-------------|
///             | . | . |   Bitwise     |       Bitwise chiplet             |-------------|
///             | . | . |   chiplet     |       13 columns                  |-- Padding --|
///             | . | . |   internal    |       constraint degree 13        |-------------|
///             | . | . |   selectors   |                                   |-------------|
///             | . | 0 |               |                                   |-------------|
///             | . +---+---+-----------------------------------------------+-------------+
///             | . | 1 | 0 |                                               |-------------|
///             | . | . | . |            Memory chiplet                     |-------------|
///             | . | . | . |              15 columns                       |-- Padding --|
///             | . | . | . |          constraint degree 9                  |-------------|
///             | . | . | 0 |                                               |-------------|
///             | . + . |---+---+-------------------------------------------+-------------+
///             | . | . | 1 | 0 |                   |                       |-------------|
///             | . | . | . | . |  Kernel ROM       |   Kernel ROM chiplet  |-------------|
///             | . | . | . | . |  chiplet internal |   6 columns           |-- Padding --|
///             | . | . | . | . |  selectors        |   constraint degree 9 |-------------|
///             | . | . | . | 0 |                   |                       |-------------|
///             | . + . | . |---+-------------------------------------------+-------------+
///             | . | . | . | 1 |---------------------------------------------------------|
///             | . | . | . | . |---------------------------------------------------------|
///             | . | . | . | . |---------------------------------------------------------|
///             | . | . | . | . |---------------------------------------------------------|
///             | . | . | . | . |----------------------- Padding -------------------------|
///             | . + . | . | . |---------------------------------------------------------|
///             | . | . | . | . |---------------------------------------------------------|
///             | . | . | . | . |---------------------------------------------------------|
///             | . | . | . | . |---------------------------------------------------------|
///             | 1 | 1 | 1 | 1 |---------------------------------------------------------|
///             +---+---+---+---+---------------------------------------------------------+
/// ```
#[derive(Debug)]
pub struct Chiplets {
    pub hasher: Hasher,
    pub bitwise: Bitwise,
    pub memory: Memory,
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
    pub fn kernel_rom_start(&self) -> RowIndex {
        self.memory_start() + self.memory.trace_len()
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
    /// Hasher, Bitwise, and Memory chiplets.
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
        self.fill_trace(&mut trace);

        ChipletsTrace {
            trace,
            aux_builder: AuxTraceBuilder::new(kernel),
        }
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Fills the provided trace for the chiplets module with the stacked execution traces of the
    /// Hasher, Bitwise, and Memory chiplets, along with selector columns to identify each chiplet
    /// trace and padding to fill the rest of the trace.
    ///
    /// It returns the auxiliary trace builders for generating auxiliary trace columns that depend
    /// on data from [Chiplets].
    fn fill_trace(self, trace: &mut [Vec<Felt>; CHIPLETS_WIDTH]) {
        // get the rows where:usize  chiplets begin.
        let bitwise_start: usize = self.bitwise_start().into();
        let memory_start: usize = self.memory_start().into();
        let kernel_rom_start: usize = self.kernel_rom_start().into();
        let padding_start: usize = self.padding_start().into();

        let Chiplets { hasher, bitwise, memory, kernel_rom } = self;

        // populate external selector columns for all chiplets
        trace[0][bitwise_start..].fill(ONE);
        trace[1][memory_start..].fill(ONE);
        trace[2][kernel_rom_start..].fill(ONE);
        trace[3][padding_start..].fill(ONE);

        // allocate fragments to be filled with the respective execution traces of each chiplet
        let mut hasher_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut bitwise_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut memory_fragment = TraceFragment::new(CHIPLETS_WIDTH);
        let mut kernel_rom_fragment = TraceFragment::new(CHIPLETS_WIDTH);

        // add the hasher, bitwise, memory, and kernel ROM segments to their respective fragments
        // so they can be filled with the chiplet traces
        for (column_num, column) in trace.iter_mut().enumerate().skip(1) {
            match column_num {
                1 => {
                    // columns 1 and 15 - 17 are relevant only for the hasher
                    hasher_fragment.push_column_slice(column, hasher.trace_len());
                },
                2 => {
                    // column 2 is relevant to the hasher and to bitwise chiplet
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    bitwise_fragment.push_column_slice(rest, bitwise.trace_len());
                },
                3 | 10..=14 => {
                    // columns 3 and 10 - 14 are relevant for hasher, bitwise, and memory chiplets
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    let rest = bitwise_fragment.push_column_slice(rest, bitwise.trace_len());
                    memory_fragment.push_column_slice(rest, memory.trace_len());
                },
                4..=9 => {
                    // columns 4 - 9 are relevant to all chiplets
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    let rest = bitwise_fragment.push_column_slice(rest, bitwise.trace_len());
                    let rest = memory_fragment.push_column_slice(rest, memory.trace_len());
                    kernel_rom_fragment.push_column_slice(rest, kernel_rom.trace_len());
                },
                15 | 16 => {
                    // columns 15 and 16 are relevant only for the hasher and memory chiplets
                    let rest = hasher_fragment.push_column_slice(column, hasher.trace_len());
                    // skip bitwise chiplet
                    let (_, rest) = rest.split_at_mut(bitwise.trace_len());
                    memory_fragment.push_column_slice(rest, memory.trace_len());
                },
                17 => {
                    // column 17 is relevant only for the memory chiplet
                    // skip the hasher and bitwise chiplets
                    let (_, rest) = column.split_at_mut(hasher.trace_len() + bitwise.trace_len());
                    memory_fragment.push_column_slice(rest, memory.trace_len());
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
    }

    /// Writes the specified row of the chiplet trace to the provided output array.
    ///
    /// For the memory and kernel ROM chiplets, only the chiplet selectors and padding are written
    /// to the output array; the actual contents need to be written by the chiplet separately.
    pub fn write_row(&self, row_idx: usize, row_out: &mut [Felt]) {
        let row_idx = RowIndex::from(row_idx);
        if row_idx < self.bitwise_start() {
            // hasher chiplet
            row_out[0] = ZERO;

            let row_end_idx = 1 + HASHER_TRACE_WIDTH;
            self.hasher.write_row(row_idx, &mut row_out[1..row_end_idx]);
            row_out[row_end_idx..].fill(ZERO);
        } else if row_idx < self.memory_start() {
            // bitwise chiplet
            row_out[0] = ONE;
            row_out[1] = ZERO;

            let row_end_idx = 2 + BITWISE_TRACE_WIDTH;
            // TODO(plafer): Why is the into() needed?
            self.bitwise
                .write_row((row_idx - self.bitwise_start()).into(), &mut row_out[2..row_end_idx]);
            row_out[row_end_idx..].fill(ZERO);
        } else if row_idx < self.kernel_rom_start() {
            // memory chiplet
            // Note: The memory chiplet fills its data all at once in a method that must be called
            // separately.
            row_out[0] = ONE;
            row_out[1] = ONE;
            row_out[2] = ZERO;

            let row_end_idx = 3 + MEMORY_TRACE_WIDTH;
            row_out[row_end_idx..].fill(ZERO);
        } else if row_idx < self.padding_start() {
            // kernel ROM chiplet
            // Note: The kernel ROM chiplet fills its data all at once in a method that must be
            // called separately.
            row_out[0] = ONE;
            row_out[1] = ONE;
            row_out[2] = ONE;
            row_out[3] = ZERO;

            let row_end_idx = 4 + KERNEL_ROM_TRACE_WIDTH;
            row_out[row_end_idx..].fill(ZERO);
        } else {
            // padding: selector columns are set to 1, while the rest of the columns are set to 0.
            row_out[..4].fill(ONE);
            row_out[4..].fill(ZERO);
        }
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
