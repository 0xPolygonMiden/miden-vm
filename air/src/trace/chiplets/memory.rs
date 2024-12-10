use super::{create_range, Felt, Range, ONE, ZERO};

// CONSTANTS
// ================================================================================================

/// Number of columns needed to record an execution trace of the memory chiplet.
pub const TRACE_WIDTH: usize = 15;

// TODO(plafer): get rid of all "selector" constants
/// Number of selector columns in the trace.
pub const NUM_SELECTORS: usize = 2;

/// Type for Memory trace selectors.
///
/// These selectors are used to define which operation and memory state update (init & read / copy &
/// read / write) is to be applied at a specific row of the memory execution trace.
pub type Selectors = [Felt; NUM_SELECTORS];

/// Specifies an operation that initializes new memory and then reads it.
pub const MEMORY_INIT_READ: Selectors = [ONE, ZERO];

/// Specifies an operation that copies existing memory and then reads it.
pub const MEMORY_COPY_READ: Selectors = [ONE, ONE];

/// Specifies a memory write operation.
pub const MEMORY_WRITE_SELECTOR: Selectors = [ZERO, ZERO];

// --- OPERATION SELECTORS ------------------------------------------------------------------------

/// Specifies the value of the `READ_WRITE` column when the operation is a write.
pub const MEMORY_WRITE: Felt = ZERO;
/// Specifies the value of the `READ_WRITE` column when the operation is a read.
pub const MEMORY_READ: Felt = ONE;
/// Specifies the value of the `ELEMENT_OR_WORD` column when the operation is over an element.
pub const MEMORY_ACCESS_ELEMENT: Felt = ZERO;
/// Specifies the value of the `ELEMENT_OR_WORD` column when the operation is over a word.
pub const MEMORY_ACCESS_WORD: Felt = ONE;

// TODO(plafer): figure out the new labels

/// Unique label computed as 1 plus the full chiplet selector with the bits reversed.
/// mem_read selector=[1, 1, 0, 1], rev(selector)=[1, 0, 1, 1], +1=[1, 1, 0, 0]
pub const MEMORY_READ_LABEL: u8 = 0b1100;

/// Unique label computed as 1 plus the full chiplet selector with the bits reversed.
/// mem_write selector=[1, 1, 0, 0] rev(selector)=[0, 0, 1, 1] +1=[0, 1, 0, 0]
pub const MEMORY_WRITE_LABEL: u8 = 0b0100;

// --- COLUMN ACCESSOR INDICES WITHIN THE CHIPLET -------------------------------------------------

/// The number of elements accessible in one read or write memory access.
pub const NUM_ELEMENTS_IN_BATCH: usize = 4;

/// Column to hold the whether the operation is a read or write.
pub const READ_WRITE_COL_IDX: usize = 0;
/// Column to hold the whether the operation was over an element or a word.
pub const ELEMENT_OR_WORD_COL_IDX: usize = READ_WRITE_COL_IDX + 1;
/// Column to hold the context ID of the current memory context.
pub const CTX_COL_IDX: usize = ELEMENT_OR_WORD_COL_IDX + 1;
/// Column to hold the memory address.
pub const BATCH_COL_IDX: usize = CTX_COL_IDX + 1;
/// Column to hold the first bit of the index of the address in the batch.
pub const IDX0_COL_IDX: usize = BATCH_COL_IDX + 1;
/// Column to hold the second bit of the index of the address in the batch.
pub const IDX1_COL_IDX: usize = IDX0_COL_IDX + 1;
/// Column for the clock cycle in which the memory operation occurred.
pub const CLK_COL_IDX: usize = IDX1_COL_IDX + 1;
/// Columns to hold the values stored at a given memory context, address, and clock cycle after
/// the memory operation. When reading from a new address, these are initialized to zero.
pub const V_COL_RANGE: Range<usize> = create_range(CLK_COL_IDX + 1, NUM_ELEMENTS_IN_BATCH);
/// Column for the lower 16-bits of the delta between two consecutive context IDs, addresses, or
/// clock cycles.
pub const D0_COL_IDX: usize = V_COL_RANGE.end;
/// Column for the upper 16-bits of the delta between two consecutive context IDs, addresses, or
/// clock cycles.
pub const D1_COL_IDX: usize = D0_COL_IDX + 1;
/// Column for the inverse of the delta between two consecutive context IDs, addresses, or clock
/// cycles, used to enforce that changes are correctly constrained.
pub const D_INV_COL_IDX: usize = D1_COL_IDX + 1;
/// Column to hold the flag indicating whether the current memory operation is in the same batch and
/// same context as the previous operation.
pub const FLAG_SAME_BATCH_AND_CONTEXT: usize = D_INV_COL_IDX + 1;
