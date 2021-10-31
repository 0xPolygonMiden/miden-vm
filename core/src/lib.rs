use core::ops::Range;

// EXPORTS
// ================================================================================================

pub use math::{fields::f128::BaseElement, FieldElement, StarkField};
pub mod hasher;
pub mod op_sponge;
pub mod opcodes;
pub mod program;
pub mod utils;

mod trace_state;
pub use trace_state::TraceState;

pub mod v1;

// GLOBAL CONSTANTS
// ================================================================================================

pub const MAX_CONTEXT_DEPTH: usize = 16;
pub const MAX_LOOP_DEPTH: usize = 8;
pub const MIN_TRACE_LENGTH: usize = 16;
pub const BASE_CYCLE_LENGTH: usize = 16;

pub const MIN_STACK_DEPTH: usize = 8;
pub const MIN_CONTEXT_DEPTH: usize = 1;
pub const MIN_LOOP_DEPTH: usize = 1;

// PUSH OPERATION
// ------------------------------------------------------------------------------------------------
pub const PUSH_OP_ALIGNMENT: usize = 8;

// HASH OPERATION
// ------------------------------------------------------------------------------------------------
const HASHER_STATE_RATE: usize = 4;
const HASHER_STATE_CAPACITY: usize = 2;
const HASHER_NUM_ROUNDS: usize = 10;
const HASHER_DIGEST_SIZE: usize = 2;

// OPERATION SPONGE
// ------------------------------------------------------------------------------------------------
const OP_SPONGE_WIDTH: usize = 4;
const PROGRAM_DIGEST_SIZE: usize = 2;
pub const HACC_NUM_ROUNDS: usize = 14;

// DECODER LAYOUT
// ------------------------------------------------------------------------------------------------
//
//  ctr ╒═════ sponge ══════╕╒═══ cf_ops ══╕╒═══════ ld_ops ═══════╕╒═ hd_ops ╕╒═ ctx ══╕╒═ loop ═╕
//   0    1    2    3    4    5    6    7    8    9    10   11   12   13   14   15   ..   ..   ..
// ├────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┤

pub const NUM_CF_OP_BITS: usize = 3;
pub const NUM_LD_OP_BITS: usize = 5;
pub const NUM_HD_OP_BITS: usize = 2;

pub const NUM_CF_OPS: usize = 8;
pub const NUM_LD_OPS: usize = 32;
pub const NUM_HD_OPS: usize = 4;

pub const OP_COUNTER_IDX: usize = 0;
pub const OP_SPONGE_RANGE: Range<usize> = Range { start: 1, end: 5 };
pub const CF_OP_BITS_RANGE: Range<usize> = Range { start: 5, end: 8 };
pub const LD_OP_BITS_RANGE: Range<usize> = Range { start: 8, end: 13 };
pub const HD_OP_BITS_RANGE: Range<usize> = Range { start: 13, end: 15 };

// STACK LAYOUT
// ------------------------------------------------------------------------------------------------
//
// ╒═══════════════════ user registers ════════════════════════╕
//    0      1    2    .................................    31
// ├─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┤

pub const MAX_PUBLIC_INPUTS: usize = 8;
pub const MAX_OUTPUTS: usize = MAX_PUBLIC_INPUTS;
pub const MAX_STACK_DEPTH: usize = 32;
