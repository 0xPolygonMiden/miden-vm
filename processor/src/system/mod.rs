use alloc::vec::Vec;
use core::fmt::{self, Display};

use miden_air::{
    trace::{
        CLK_COL_IDX, CTX_COL_IDX, FMP_COL_IDX, FN_HASH_OFFSET, IN_SYSCALL_COL_IDX, SYS_TRACE_WIDTH,
    },
    RowIndex,
};
use vm_core::utils::uninit_vector;

use super::{ExecutionError, Felt, FieldElement, SysTrace, Word, EMPTY_WORD, ONE, ZERO};

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

// We assign the following special meanings to memory segments in each context:
// - First 2^30 addresses (0 to 2^30 - 1) are reserved for global memory.
// - The next 2^30 addresses (2^30 to 2^31 - 1) are reserved for procedure locals.
// - The next 2^30 addresses (2^31 to 3 * 2^30 - 1) are reserved for procedure locals of SYSCALLs.
// - All remaining addresses do not have any special meaning.
//
// Note that the above assignment is purely conventional: a program can read from and write to any
// address in a given context, regardless of which memory segment it belongs to.

/// Memory addresses for procedure locals start at 2^30.
pub const FMP_MIN: u64 = 2_u64.pow(30);
/// Memory address for procedure locals within a SYSCALL starts at 2^31.
pub const SYSCALL_FMP_MIN: u32 = 2_u32.pow(31);
/// Value of FMP register should not exceed 3 * 2^30 - 1.
pub const FMP_MAX: u64 = 3 * 2_u64.pow(30) - 1;

// SYSTEM INFO
// ================================================================================================

#[derive(Debug)]
struct SystemTraceRow {
    ctx: Felt,
    clk: Felt,
    fmp: Felt,
    in_syscall: Felt,
    fn_hash: [Felt; 4],
}

/// System info container for the VM.
///
/// This keeps track of the following system variables:
/// - clock cycle (clk), which starts at 0 and is incremented with every step.
/// - execution context (ctx), which starts at 0 (root context), and changes when CALL or SYSCALL
///   operations are executed by the VM (or when we return from a CALL or SYSCALL).
/// - free memory pointer (fmp), which is initially set to 2^30.
/// - in_syscall flag which indicates whether the execution is currently in a SYSCALL block.
/// - hash of the function which initiated the current execution context. if the context was
///   initiated from the root context, this will be set to ZEROs.
#[derive(Debug)]
pub struct System {
    clk: RowIndex,
    ctx: ContextId,
    fmp: Felt,
    in_syscall: bool,
    fn_hash: Word,
    rows: Vec<SystemTraceRow>,
}

impl System {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [System] struct with execution traces instantiated with the specified length.
    ///
    /// Initializes the free memory pointer `fmp` used for local memory offsets to 2^30.
    pub fn new(init_trace_capacity: usize) -> Self {
        let fmp = Felt::new(FMP_MIN);

        let rows = {
            let first_row = SystemTraceRow {
                ctx: Felt::ZERO,
                clk: Felt::ZERO,
                fmp,
                in_syscall: Felt::ZERO,
                fn_hash: [Felt::ZERO; 4],
            };

            let mut rows = Vec::with_capacity(init_trace_capacity);
            rows.push(first_row);
            rows
        };

        Self {
            clk: RowIndex::from(0),
            ctx: ContextId::root(),
            fmp,
            in_syscall: false,
            fn_hash: EMPTY_WORD,
            rows,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the current clock cycle of a process.
    #[inline(always)]
    pub fn clk(&self) -> RowIndex {
        self.clk
    }

    /// Returns the current execution context ID.
    #[inline(always)]
    pub fn ctx(&self) -> ContextId {
        self.ctx
    }

    /// Returns the current value of the free memory pointer for a process.
    #[inline(always)]
    pub fn fmp(&self) -> Felt {
        self.fmp
    }

    /// Returns true if the VM is currently executing a SYSCALL block.
    pub fn in_syscall(&self) -> bool {
        self.in_syscall
    }

    /// Returns hash of the function which initiated the current execution context.
    #[inline(always)]
    pub fn fn_hash(&self) -> Word {
        self.fn_hash
    }

    /// Returns execution trace length for the systems columns of the process.
    ///
    /// Trace length of the system columns is equal to the number of cycles executed by the VM.
    #[inline(always)]
    pub fn trace_len(&self) -> usize {
        self.clk.into()
    }

    /// Returns execution context ID at the specified clock cycle.
    #[inline(always)]
    pub fn get_ctx_at(&self, clk: RowIndex) -> ContextId {
        let ctx = self.rows[clk.as_usize()].ctx.as_int() as u32;
        ctx.into()
    }

    /// Returns free memory pointer at the specified clock cycle.
    #[inline(always)]
    pub fn get_fmp_at(&self, clk: RowIndex) -> Felt {
        self.rows[clk.as_usize()].fmp
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self, max_cycles: u32) -> Result<(), ExecutionError> {
        self.clk += 1;

        // Check that maximum number of cycles is not exceeded.
        // TODO(plafer): do that somewhere else, only once
        if self.clk.as_u32() > max_cycles {
            return Err(ExecutionError::CycleLimitExceeded(max_cycles));
        }

        let row = SystemTraceRow {
            ctx: self.ctx.into(),
            clk: self.clk.into(),
            fmp: self.fmp,
            in_syscall: if self.in_syscall { ONE } else { ZERO },
            fn_hash: self.fn_hash,
        };
        self.rows.push(row);

        Ok(())
    }

    /// Sets the value of free memory pointer for the next clock cycle.
    pub fn set_fmp(&mut self, fmp: Felt) {
        // we set only the current value of fmp here, the trace will be updated with this value
        // when the clock cycle advances.
        self.fmp = fmp;
    }

    /// Updates system registers to mark a new function call.
    ///
    /// Internally, this performs the following updates:
    /// - Set the execution context to the current clock cycle + 1. This ensures that the context is
    ///   globally unique as is never set to 0.
    /// - Sets the free memory pointer to its initial value (FMP_MIN).
    /// - Sets the hash of the function which initiated the current context to the provided value.
    ///
    /// A CALL or DYNCALL cannot be started when the VM is executing a SYSCALL.
    pub fn start_call_or_dyncall(&mut self, fn_hash: Word) {
        debug_assert!(!self.in_syscall, "call in syscall");
        self.ctx = (self.clk + 1).into();
        self.fmp = Felt::new(FMP_MIN);
        self.fn_hash = fn_hash;
    }

    /// Updates system registers to mark a new syscall.
    ///
    /// Internally, this performs the following updates:
    /// - Set the execution context to 0 (the root context).
    /// - Sets the free memory pointer to the initial value of syscalls (SYSCALL_FMP_MIN). This
    ///   ensures that procedure locals within a syscall do not conflict with procedure locals of
    ///   the original root context.
    /// - Sets the in_syscall flag to true.
    ///
    /// A SYSCALL cannot be started when the VM is executing a SYSCALL.
    ///
    /// Note that this does not change the hash of the function which initiated the context:
    /// for SYSCALLs this remains set to the hash of the last invoked function.
    pub fn start_syscall(&mut self) {
        debug_assert!(!self.in_syscall, "already in syscall");
        self.ctx = ContextId::root();
        self.fmp = Felt::from(SYSCALL_FMP_MIN);
        self.in_syscall = true;
    }

    /// Updates system registers to the provided values. These updates are made at the end of a
    /// CALL or a SYSCALL blocks.
    ///
    /// Note that we set in_syscall flag to true regardless of whether we return from a CALL or a
    /// SYSCALL.
    pub fn restore_context(&mut self, ctx: ContextId, fmp: Felt, fn_hash: Word) {
        self.ctx = ctx;
        self.fmp = fmp;
        self.in_syscall = false;
        self.fn_hash = fn_hash;
    }

    // TRACE GENERATIONS
    // --------------------------------------------------------------------------------------------

    /// Returns an execution trace of this system info container.
    ///
    /// If the trace is smaller than the specified `trace_len`, the columns of the trace are
    /// extended to match the specified length as follows:
    /// - the remainder of the `clk` column is filled in with increasing values of `clk`.
    /// - the remainder of the `ctx` column is filled in with ZERO, which should be the last value
    ///   in the column.
    /// - the remainder of the `fmp` column is filled in with the last value in the column.
    /// - the remainder of the `in_syscall` column is filled with ZERO, which should be the last
    ///   value in this colum.
    /// - the remainder of the `fn_hash` columns are filled with ZERO, which should be the last
    ///   values in these columns.
    ///
    /// `num_rand_rows` indicates the number of rows at the end of the trace which will be
    /// overwritten with random values. This parameter is unused because last rows are just
    /// duplicates of the prior rows and thus can be safely overwritten.
    pub fn into_trace(self, trace_len: usize, num_rand_rows: usize) -> SysTrace {
        let clk: usize = self.clk().into();
        // make sure that only the duplicate rows will be overwritten with random values
        assert!(clk + num_rand_rows <= trace_len, "target trace length too small");
        let own_len = self.rows.len();

        let mut trace_columns = unsafe { vec![uninit_vector(trace_len); SYS_TRACE_WIDTH] };

        for (i, row) in self.rows.into_iter().enumerate() {
            trace_columns[CLK_COL_IDX][i] = row.clk;
            trace_columns[FMP_COL_IDX][i] = row.fmp;
            trace_columns[CTX_COL_IDX][i] = row.ctx;
            trace_columns[IN_SYSCALL_COL_IDX][i] = row.in_syscall;
            for (j, hash) in row.fn_hash.iter().enumerate() {
                trace_columns[FN_HASH_OFFSET + j][i] = *hash;
            }
        }

        // padding rows

        // complete the clk column by filling in all values after the last clock cycle, which
        // continue to increment at each row.
        for i in own_len..trace_len {
            trace_columns[CLK_COL_IDX][i] = Felt::from(i as u32);
        }

        // complete the ctx column by filling all values after the last clock cycle with ZEROs as
        // the last context must be zero context.
        trace_columns[CTX_COL_IDX][own_len..trace_len].fill(ZERO);

        // complete the fmp column by filling in all values after the last clock cycle with the
        // value in the column at the last clock cycle.
        trace_columns[FMP_COL_IDX][own_len..trace_len].fill(self.fmp);

        // complete the in_syscall column by filling all values after the last clock cycle with
        // ZEROs as we must end the program in the root context which is not a SYSCALL
        trace_columns[IN_SYSCALL_COL_IDX][own_len..trace_len].fill(ZERO);

        // complete the fn hash columns by filling them with ZEROs as program execution must always
        // end in the root context.
        trace_columns[FN_HASH_OFFSET][own_len..trace_len].fill(ZERO);
        trace_columns[FN_HASH_OFFSET + 1][own_len..trace_len].fill(ZERO);
        trace_columns[FN_HASH_OFFSET + 2][own_len..trace_len].fill(ZERO);
        trace_columns[FN_HASH_OFFSET + 3][own_len..trace_len].fill(ZERO);

        trace_columns.try_into().expect("failed to convert vector to array")
    }
}

// EXECUTION CONTEXT
// ================================================================================================

/// Represents the ID of an execution context
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ContextId(u32);

impl ContextId {
    /// Returns the root context ID
    pub fn root() -> Self {
        Self(0)
    }

    /// Returns true if the context ID represents the root context
    pub fn is_root(&self) -> bool {
        self.0 == 0
    }
}

impl From<RowIndex> for ContextId {
    fn from(value: RowIndex) -> Self {
        Self(value.as_u32())
    }
}

impl From<u32> for ContextId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<ContextId> for u32 {
    fn from(context_id: ContextId) -> Self {
        context_id.0
    }
}

impl From<ContextId> for u64 {
    fn from(context_id: ContextId) -> Self {
        context_id.0.into()
    }
}

impl From<ContextId> for Felt {
    fn from(context_id: ContextId) -> Self {
        context_id.0.into()
    }
}

impl Display for ContextId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
