use alloc::vec::Vec;
use core::ops::Range;

use miden_air::{
    trace::chiplets::hasher::{NUM_ROUNDS, NUM_SELECTORS},
    RowIndex,
};
use vm_core::chiplets::hasher::apply_round;

use super::{Felt, HasherState, Selectors, TraceFragment, STATE_WIDTH, TRACE_WIDTH, ZERO};

// HASHER TRACE
// ================================================================================================

#[derive(Debug, Clone)]
struct HasherTraceRow {
    selectors: Selectors,
    hasher_state: HasherState,
    node_index: Felt,
}

/// Execution trace of the hasher component.
///
/// The trace consists of 16 columns grouped logically as follows:
/// - 3 selector columns.
/// - 12 columns describing hasher state.
/// - 1 node index column used for Merkle path related computations.
#[derive(Debug, Default)]
pub struct HasherTrace {
    rows: Vec<HasherTraceRow>,
}

impl HasherTrace {
    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns current length of this execution trace.
    pub fn trace_len(&self) -> usize {
        self.rows.len()
    }

    /// Returns the next row address. The address is equal to the current trace length + 1.
    ///
    /// The above means that row addresses start at ONE (rather than ZERO), and are incremented by
    /// ONE at every row. Starting at ONE is needed for the decoder so that the address of the
    /// first code block is a non-zero value.
    pub fn next_row_addr(&self) -> Felt {
        Felt::new(self.trace_len() as u64 + 1)
    }

    // TRACE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Appends 8 rows to the execution trace describing a single permutation of the hash function.
    ///
    /// The initial state of the hasher is provided via the `state` parameter. All subsequent
    /// states are derived by applying a single round of the hash function to the previous state.
    ///
    /// Selector values for the first and last rows are provided via `init_selectors` and
    /// `final_selectors` parameters. Selector values for all other rows are derived from the
    /// selectors of the first row.
    ///
    /// Node index values are provided via `init_index` and `rest_index` parameters. The former is
    /// used for the first row, and the latter for all subsequent rows.
    pub fn append_permutation_with_index(
        &mut self,
        state: &mut HasherState,
        init_selectors: Selectors,
        final_selectors: Selectors,
        init_index: Felt,
        rest_index: Felt,
    ) {
        // append the first row of the permutation cycle
        self.append_row(init_selectors, state, init_index);

        // append the next 6 rows of the permutation cycle. for these rows:
        // - the last two selectors are carried over from row to row; the first selector is set to
        //   ZERO.
        // - hasher state is updated by applying a single round of the hash function for every row.
        let next_selectors = [ZERO, init_selectors[1], init_selectors[2]];
        for i in 0..NUM_ROUNDS - 1 {
            apply_round(state, i);
            self.append_row(next_selectors, state, rest_index);
        }

        // apply the last round and append the last row to the trace
        apply_round(state, NUM_ROUNDS - 1);
        self.append_row(final_selectors, state, rest_index);
    }

    /// Appends 8 rows to the execution trace describing a single permutation of the hash function.
    ///
    /// This function is similar to the append_permutation_with_index() function above, but it sets
    /// init_index and rest_index parameters to ZEROs.
    #[inline(always)]
    pub fn append_permutation(
        &mut self,
        state: &mut HasherState,
        init_selectors: Selectors,
        final_selectors: Selectors,
    ) {
        self.append_permutation_with_index(state, init_selectors, final_selectors, ZERO, ZERO);
    }

    /// Appends a new row to the execution trace based on the supplied parameters.
    fn append_row(&mut self, selectors: Selectors, state: &HasherState, node_index: Felt) {
        self.rows.push(HasherTraceRow {
            selectors,
            hasher_state: *state,
            node_index,
        });
    }

    /// Copies section of trace from the given range of start and end rows at the end of the trace.
    /// The hasher state of the last row is copied to the provided state input.
    pub fn copy_trace(&mut self, state: &mut [Felt; STATE_WIDTH], range: Range<usize>) {
        self.rows.extend_from_within(range.clone());

        // copy the latest hasher state to the provided state slice
        *state = self.rows[range.end - 1].hasher_state;
    }

    // EXECUTION TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    /// Fills the provided trace fragment with trace data from this hasher trace instance.
    pub fn fill_trace(self, trace: &mut TraceFragment) {
        // make sure fragment dimensions are consistent with the dimensions of this trace
        debug_assert_eq!(self.trace_len(), trace.len(), "inconsistent trace lengths");
        debug_assert_eq!(TRACE_WIDTH, trace.width(), "inconsistent trace widths");

        for (row_idx, row) in self.rows.into_iter().enumerate() {
            let row_idx: RowIndex = (row_idx as u32).into();

            // copy selector values
            for (col_idx, selector_val) in row.selectors.into_iter().enumerate() {
                trace.set(row_idx, col_idx, selector_val);
            }

            // copy hasher state values
            for (col_idx, hasher_val) in row.hasher_state.into_iter().enumerate() {
                trace.set(row_idx, NUM_SELECTORS + col_idx, hasher_val);
            }

            // copy node index value
            trace.set(row_idx, TRACE_WIDTH - 1, row.node_index);
        }
    }

    pub fn write_row(&self, row_idx: RowIndex, row_out: &mut [Felt]) {
        let row = &self.rows[row_idx.as_usize()];

        // copy selector values
        for (col_idx, selector_val) in row.selectors.into_iter().enumerate() {
            row_out[col_idx] = selector_val;
        }

        // copy hasher state values
        for (col_idx, hasher_val) in row.hasher_state.into_iter().enumerate() {
            row_out[NUM_SELECTORS + col_idx] = hasher_val;
        }

        // copy node index value
        row_out[TRACE_WIDTH - 1] = row.node_index;
    }
}
