use core::ops::Range;
use log::debug;
use std::time::Instant;
use winterfell::{ProofOptions, StarkProof};

// RE-EXPORTS
// ================================================================================================
mod utils;

mod air;

mod processor;
pub use processor::{OpCode, OpHint};

mod programs;
pub use programs::{assembly, blocks, Program, ProgramInputs};

// EXECUTOR
// ================================================================================================

/// Executes the specified `program` and returns the result together with a STARK-based proof of execution.
///
/// * `inputs` specifies the initial stack state and provides secret input tapes;
/// * `num_outputs` specifies the number of elements from the top of the stack to be returned;
pub fn execute(
    program: &Program,
    inputs: &ProgramInputs,
    num_outputs: usize,
    _options: &ProofOptions,
) -> (Vec<u128>, StarkProof) {
    assert!(
        num_outputs <= MAX_OUTPUTS,
        "cannot produce more than {} outputs, but requested {}",
        MAX_OUTPUTS,
        num_outputs
    );

    // execute the program to create an execution trace
    let now = Instant::now();
    let trace = processor::execute(program, inputs);
    debug!(
        "Generated execution trace of {} registers and {} steps in {} ms",
        trace.width(),
        trace.length(),
        now.elapsed().as_millis()
    );

    // TODO: migrate the rest
    unimplemented!()
}

// GLOBAL CONSTANTS
// ================================================================================================

pub const MAX_CONTEXT_DEPTH: usize = 16;
pub const MAX_LOOP_DEPTH: usize = 8;
const MIN_TRACE_LENGTH: usize = 16;
const MAX_REGISTER_COUNT: usize = 128;
const MIN_EXTENSION_FACTOR: usize = 16;
const BASE_CYCLE_LENGTH: usize = 16;

const MIN_STACK_DEPTH: usize = 8;
const MIN_CONTEXT_DEPTH: usize = 1;
const MIN_LOOP_DEPTH: usize = 1;

// PUSH OPERATION
// ------------------------------------------------------------------------------------------------
const PUSH_OP_ALIGNMENT: usize = 8;

// HASH OPERATION
// ------------------------------------------------------------------------------------------------
const HASH_STATE_RATE: usize = 4;
const HASH_STATE_CAPACITY: usize = 2;
const HASH_STATE_WIDTH: usize = HASH_STATE_RATE + HASH_STATE_CAPACITY;
const HASH_NUM_ROUNDS: usize = 10;
const HASH_DIGEST_SIZE: usize = 2;

// OPERATION SPONGE
// ------------------------------------------------------------------------------------------------
const SPONGE_WIDTH: usize = 4;
const PROGRAM_DIGEST_SIZE: usize = 2;
const HACC_NUM_ROUNDS: usize = 14;

// DECODER LAYOUT
// ------------------------------------------------------------------------------------------------
//
//  ctr ╒═════ sponge ══════╕╒═══ cf_ops ══╕╒═══════ ld_ops ═══════╕╒═ hd_ops ╕╒═ ctx ══╕╒═ loop ═╕
//   0    1    2    3    4    5    6    7    8    9    10   11   12   13   14   15   ..   ..   ..
// ├────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┤

const NUM_CF_OP_BITS: usize = 3;
const NUM_LD_OP_BITS: usize = 5;
const NUM_HD_OP_BITS: usize = 2;

const NUM_CF_OPS: usize = 8;
const NUM_LD_OPS: usize = 32;
const NUM_HD_OPS: usize = 4;

const OP_COUNTER_IDX: usize = 0;
const SPONGE_RANGE: Range<usize> = Range { start: 1, end: 5 };
const CF_OP_BITS_RANGE: Range<usize> = Range { start: 5, end: 8 };
const LD_OP_BITS_RANGE: Range<usize> = Range { start: 8, end: 13 };
const HD_OP_BITS_RANGE: Range<usize> = Range { start: 13, end: 15 };

// STACK LAYOUT
// ------------------------------------------------------------------------------------------------
//
// ╒═══════════════════ user registers ════════════════════════╕
//    0      1    2    .................................    31
// ├─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┤

pub const MAX_PUBLIC_INPUTS: usize = 8;
pub const MAX_OUTPUTS: usize = MAX_PUBLIC_INPUTS;
pub const MAX_STACK_DEPTH: usize = 32;
