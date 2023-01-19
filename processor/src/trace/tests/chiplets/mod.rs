use super::{
    super::{utils::build_span_with_respan_ops, Trace, NUM_RAND_ROWS},
    build_trace_from_block, build_trace_from_ops, build_trace_from_ops_with_inputs, rand_array,
    AdviceInputs, ExecutionTrace, Felt, FieldElement, Operation, Word, ONE, ZERO,
};
use rand_utils::rand_value;
use vm_core::{
    chiplets::hasher::HASH_CYCLE_LEN, AUX_TRACE_RAND_ELEMENTS, CHIPLETS_AUX_TRACE_OFFSET,
};

mod bitwise;
mod hasher;
mod memory;
