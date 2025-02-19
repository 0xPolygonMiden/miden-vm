use alloc::vec::Vec;
use core::slice;

use miden_air::{trace::main_trace::MainTrace, RowIndex};
#[cfg(test)]
use vm_core::{utils::ToElements, Operation};

use super::{Felt, FieldElement, NUM_RAND_ROWS};
use crate::{chiplets::Chiplets, utils::uninit_vector};

// TRACE FRAGMENT
// ================================================================================================

/// TODO: add docs
pub struct TraceFragment<'a> {
    data: Vec<&'a mut [Felt]>,
}

impl<'a> TraceFragment<'a> {
    /// Creates a new TraceFragment with its data allocated to the specified capacity.
    pub fn new(capacity: usize) -> Self {
        TraceFragment { data: Vec::with_capacity(capacity) }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the number of columns in this execution trace fragment.
    pub fn width(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of rows in this execution trace fragment.
    pub fn len(&self) -> usize {
        self.data[0].len()
    }

    // DATA MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Updates a single cell in this fragment with provided value.
    #[inline(always)]
    pub fn set(&mut self, row_idx: RowIndex, col_idx: usize, value: Felt) {
        self.data[col_idx][row_idx] = value;
    }

    /// Returns a mutable iterator to the columns of this fragment.
    pub fn columns(&mut self) -> slice::IterMut<'_, &'a mut [Felt]> {
        self.data.iter_mut()
    }

    /// Adds a new column to this fragment by pushing a mutable slice with the first `len`
    /// elements of the provided column. Returns the rest of the provided column as a separate
    /// mutable slice.
    pub fn push_column_slice(&mut self, column: &'a mut [Felt], len: usize) -> &'a mut [Felt] {
        let (column_fragment, rest) = column.split_at_mut(len);
        self.data.push(column_fragment);
        rest
    }

    // TEST METHODS
    // --------------------------------------------------------------------------------------------

    #[cfg(test)]
    pub fn trace_to_fragment(trace: &'a mut [Vec<Felt>]) -> Self {
        let mut data = Vec::new();
        for column in trace.iter_mut() {
            data.push(column.as_mut_slice());
        }
        Self { data }
    }
}

// TRACE LENGTH SUMMARY
// ================================================================================================

/// Contains the data about lengths of the trace parts.
///
/// - `main_trace_len` contains the length of the main trace.
/// - `range_trace_len` contains the length of the range checker trace.
/// - `chiplets_trace_len` contains the trace lengths of the all chiplets (hash, bitwise, memory,
///   kernel ROM)
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct TraceLenSummary {
    main_trace_len: usize,
    range_trace_len: usize,
    chiplets_trace_len: ChipletsLengths,
}

impl TraceLenSummary {
    pub fn new(
        main_trace_len: usize,
        range_trace_len: usize,
        chiplets_trace_len: ChipletsLengths,
    ) -> Self {
        TraceLenSummary {
            main_trace_len,
            range_trace_len,
            chiplets_trace_len,
        }
    }

    /// Returns length of the main trace.
    pub fn main_trace_len(&self) -> usize {
        self.main_trace_len
    }

    /// Returns length of the range checker trace.
    pub fn range_trace_len(&self) -> usize {
        self.range_trace_len
    }

    /// Returns [ChipletsLengths] which contains trace lengths of all chilplets.
    pub fn chiplets_trace_len(&self) -> ChipletsLengths {
        self.chiplets_trace_len
    }

    /// Returns the maximum of all component lengths.
    pub fn trace_len(&self) -> usize {
        self.range_trace_len
            .max(self.main_trace_len)
            .max(self.chiplets_trace_len.trace_len())
    }

    /// Returns `trace_len` rounded up to the next power of two.
    pub fn padded_trace_len(&self) -> usize {
        (self.trace_len() + NUM_RAND_ROWS).next_power_of_two()
    }

    /// Returns the percent (0 - 100) of the steps that were added to the trace to pad it to the
    /// next power of tow.
    pub fn padding_percentage(&self) -> usize {
        (self.padded_trace_len() - self.trace_len()) * 100 / self.padded_trace_len()
    }
}

/// Contains trace lengths of all chilplets: hash, bitwise, memory and kernel ROM trace
/// lengths.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChipletsLengths {
    hash_chiplet_len: usize,
    bitwise_chiplet_len: usize,
    memory_chiplet_len: usize,
    kernel_rom_len: usize,
}

impl ChipletsLengths {
    pub fn new(chiplets: &Chiplets) -> Self {
        ChipletsLengths {
            hash_chiplet_len: chiplets.bitwise_start().into(),
            bitwise_chiplet_len: chiplets.memory_start() - chiplets.bitwise_start(),
            memory_chiplet_len: chiplets.kernel_rom_start() - chiplets.memory_start(),
            kernel_rom_len: chiplets.padding_start() - chiplets.kernel_rom_start(),
        }
    }

    pub fn from_parts(
        hash_len: usize,
        bitwise_len: usize,
        memory_len: usize,
        kernel_len: usize,
    ) -> Self {
        ChipletsLengths {
            hash_chiplet_len: hash_len,
            bitwise_chiplet_len: bitwise_len,
            memory_chiplet_len: memory_len,
            kernel_rom_len: kernel_len,
        }
    }

    /// Returns the length of the hash chiplet trace
    pub fn hash_chiplet_len(&self) -> usize {
        self.hash_chiplet_len
    }

    /// Returns the length of the bitwise trace
    pub fn bitwise_chiplet_len(&self) -> usize {
        self.bitwise_chiplet_len
    }

    /// Returns the length of the memory trace
    pub fn memory_chiplet_len(&self) -> usize {
        self.memory_chiplet_len
    }

    /// Returns the length of the kernel ROM trace
    pub fn kernel_rom_len(&self) -> usize {
        self.kernel_rom_len
    }

    /// Returns the length of the trace required to accommodate chiplet components and 1
    /// mandatory padding row required for ensuring sufficient trace length for auxiliary connector
    /// columns that rely on the memory chiplet.
    pub fn trace_len(&self) -> usize {
        self.hash_chiplet_len()
            + self.bitwise_chiplet_len()
            + self.memory_chiplet_len()
            + self.kernel_rom_len()
            + 1
    }
}

// AUXILIARY COLUMN BUILDER
// ================================================================================================

/// Defines a builder responsible for building a single column in an auxiliary segment of the
/// execution trace.
pub trait AuxColumnBuilder<E: FieldElement<BaseField = Felt>> {
    // REQUIRED METHODS
    // --------------------------------------------------------------------------------------------

    fn get_requests_at(
        &self,
        main_trace: &MainTrace,
        alphas: &[E],
        row: RowIndex,
    ) -> E;

    fn get_responses_at(
        &self,
        main_trace: &MainTrace,
        alphas: &[E],
        row: RowIndex,
    ) -> E;

    #[cfg(any(test, feature = "bus-debugger"))]
    fn bus_debugger(&self) -> &crate::debug::BusDebugger<E>;

    // PROVIDED METHODS
    // --------------------------------------------------------------------------------------------

    fn init_requests(
        &self,
        _main_trace: &MainTrace,
        _alphas: &[E],
    ) -> E {
        E::ONE
    }

    fn init_responses(
        &self,
        _main_trace: &MainTrace,
        _alphas: &[E],
    ) -> E {
        E::ONE
    }

    /// Builds the chiplets bus auxiliary trace column.
    fn build_aux_column(&self, main_trace: &MainTrace, alphas: &[E]) -> Vec<E> {
        let mut responses_prod: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
        let mut requests: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };

        responses_prod[0] = self.init_responses(main_trace, alphas);
        requests[0] = self.init_requests(main_trace, alphas);

        let mut requests_running_prod = requests[0];
        for row_idx in 0..main_trace.num_rows() - 1 {
            let row = row_idx.into();
            responses_prod[row_idx + 1] = responses_prod[row_idx]
                * self.get_responses_at(main_trace, alphas, row);
            requests[row_idx + 1] =
                self.get_requests_at(main_trace, alphas, row);
            requests_running_prod *= requests[row_idx + 1];
        }

        let mut requests_running_divisor = requests_running_prod.inv();
        let mut result_aux_column = responses_prod;
        for i in (0..main_trace.num_rows()).rev() {
            result_aux_column[i] *= requests_running_divisor;
            requests_running_divisor *= requests[i];
        }

        #[cfg(any(test, feature = "bus-debugger"))]
        assert!(self.bus_debugger().is_empty(), "{}", self.bus_debugger());

        result_aux_column
    }
}

// TEST HELPERS
// ================================================================================================

#[cfg(test)]
pub fn build_span_with_respan_ops() -> (Vec<Operation>, Vec<Felt>) {
    let iv = [1, 3, 5, 7, 9, 11, 13, 15, 17].to_elements();
    let ops = vec![
        Operation::Push(iv[0]),
        Operation::Push(iv[1]),
        Operation::Push(iv[2]),
        Operation::Push(iv[3]),
        Operation::Push(iv[4]),
        Operation::Push(iv[5]),
        Operation::Push(iv[6]),
        // next batch
        Operation::Push(iv[7]),
        Operation::Push(iv[8]),
        Operation::Add,
        // drops to make sure stack overflow is empty on exit
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
        Operation::Drop,
    ];
    (ops, iv)
}
