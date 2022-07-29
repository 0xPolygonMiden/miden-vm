use super::{
    super::{Trace, NUM_RAND_ROWS},
    build_trace_from_ops, rand_array, ExecutionTrace, Felt, FieldElement, Operation, Word, ONE,
    ZERO,
};
use rand_utils::rand_value;
use vm_core::{
    chiplets::hasher::HASH_CYCLE_LEN, AUX_TRACE_RAND_ELEMENTS, CHIPLETS_AUX_TRACE_OFFSET,
};

mod bitwise;
mod memory;
