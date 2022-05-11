use core::ops::Range;

pub mod bitwise;
pub mod hasher;
pub mod program;
pub use math::{fields::f64::BaseElement as Felt, FieldElement, StarkField};

mod operations;
pub use operations::{AdviceInjector, DebugOptions, Operation};

mod inputs;
pub use inputs::{AdviceSet, ProgramInputs};

pub mod utils;
use utils::range;

pub mod errors;

// TYPE ALIASES
// ================================================================================================

pub type Word = [Felt; 4];

pub type StackTopState = [Felt; MIN_STACK_DEPTH];

// CONSTANTS
// ================================================================================================

/// The minimum length of the execution trace. This is the minimum required to support range checks.
pub const MIN_TRACE_LEN: usize = 1024;

/// The minimum stack depth enforced by the VM. This is also the number of stack registers which can
/// be accessed by the VM directly.
pub const MIN_STACK_DEPTH: usize = 16;

/// Number of bookkeeping and helper columns in the stack trace.
pub const NUM_STACK_HELPER_COLS: usize = 3;

// TRACE LAYOUT
// ------------------------------------------------------------------------------------------------

//      system          decoder           stack      range checks    auxiliary table
//    (2 columns)     (18 columns)    (19 columns)    (4 columns)     (18 columns)
// ├───────────────┴───────────────┴───────────────┴───────────────┴─────────────────┤

pub const SYS_TRACE_OFFSET: usize = 0;
pub const SYS_TRACE_WIDTH: usize = 2;
pub const SYS_TRACE_RANGE: Range<usize> = range(SYS_TRACE_OFFSET, SYS_TRACE_WIDTH);

pub const CLK_COL_IDX: usize = SYS_TRACE_OFFSET;
pub const FMP_COL_IDX: usize = SYS_TRACE_OFFSET + 1;

// decoder trace
pub const DECODER_TRACE_OFFSET: usize = SYS_TRACE_OFFSET + SYS_TRACE_WIDTH;
pub const DECODER_TRACE_WIDTH: usize = 19;
pub const DECODER_TRACE_RANGE: Range<usize> = range(DECODER_TRACE_OFFSET, DECODER_TRACE_WIDTH);

// Stack trace
pub const STACK_TRACE_OFFSET: usize = DECODER_TRACE_OFFSET + DECODER_TRACE_WIDTH;
pub const STACK_TRACE_WIDTH: usize = MIN_STACK_DEPTH + NUM_STACK_HELPER_COLS;
pub const STACK_TRACE_RANGE: Range<usize> = range(STACK_TRACE_OFFSET, STACK_TRACE_WIDTH);

// Range check trace
pub const RANGE_CHECK_TRACE_OFFSET: usize = STACK_TRACE_OFFSET + STACK_TRACE_WIDTH;
pub const RANGE_CHECK_TRACE_WIDTH: usize = 4;
pub const RANGE_CHECK_TRACE_RANGE: Range<usize> =
    range(RANGE_CHECK_TRACE_OFFSET, RANGE_CHECK_TRACE_WIDTH);

// Auxiliary table trace
pub const AUX_TRACE_OFFSET: usize = RANGE_CHECK_TRACE_OFFSET + RANGE_CHECK_TRACE_WIDTH;
pub const AUX_TRACE_WIDTH: usize = 18;
pub const AUX_TRACE_RANGE: Range<usize> = range(AUX_TRACE_OFFSET, AUX_TRACE_WIDTH);

pub const TRACE_WIDTH: usize = AUX_TRACE_OFFSET + AUX_TRACE_WIDTH;
