#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use core::ops::Range;

pub mod chiplets;
pub mod decoder;
pub mod errors;
pub mod range;

pub use ::crypto::{Word, ONE, WORD_SIZE, ZERO};
pub mod crypto {
    pub mod merkle {
        pub use ::crypto::merkle::{
            MerkleError, MerklePath, MerklePathSet, MerkleStore, MerkleTree, NodeIndex, SimpleSmt,
        };
    }

    pub mod hash {
        pub use ::crypto::hash::{
            blake::{Blake3Digest, Blake3_160, Blake3_192, Blake3_256},
            rpo::{Rpo256, RpoDigest},
            ElementHasher, Hasher,
        };
    }

    pub mod random {
        pub use crate::random::*;
    }
}

pub use math::{
    fields::{f64::BaseElement as Felt, QuadExtension},
    polynom, ExtensionOf, FieldElement, StarkField, ToElements,
};

mod program;
pub use program::{blocks as code_blocks, CodeBlockTable, Kernel, Program, ProgramInfo};

mod operations;
pub use operations::{
    AdviceInjector, AssemblyOp, Decorator, DecoratorIterator, DecoratorList, Operation,
};

pub mod stack;
pub use stack::{StackInputs, StackOutputs};

// TODO: this should move to miden-crypto crate
mod random;

pub mod utils;
use utils::range;

// TYPE ALIASES
// ================================================================================================

pub type StackTopState = [Felt; stack::STACK_TOP_SIZE];

// CONSTANTS
// ================================================================================================

/// The minimum length of the execution trace. This is the minimum required to support range checks.
pub const MIN_TRACE_LEN: usize = 1024;

// MAIN TRACE LAYOUT
// ------------------------------------------------------------------------------------------------

//      system          decoder           stack      range checks       chiplets
//    (8 columns)     (23 columns)    (19 columns)    (4 columns)     (18 columns)
// ├───────────────┴───────────────┴───────────────┴───────────────┴─────────────────┤

pub const SYS_TRACE_OFFSET: usize = 0;
pub const SYS_TRACE_WIDTH: usize = 8;
pub const SYS_TRACE_RANGE: Range<usize> = range(SYS_TRACE_OFFSET, SYS_TRACE_WIDTH);

pub const CLK_COL_IDX: usize = SYS_TRACE_OFFSET;
pub const FMP_COL_IDX: usize = SYS_TRACE_OFFSET + 1;
pub const CTX_COL_IDX: usize = SYS_TRACE_OFFSET + 2;
pub const IN_SYSCALL_COL_IDX: usize = SYS_TRACE_OFFSET + 3;
pub const FN_HASH_OFFSET: usize = SYS_TRACE_OFFSET + 4;
pub const FN_HASH_RANGE: Range<usize> = range(FN_HASH_OFFSET, 4);

// decoder trace
pub const DECODER_TRACE_OFFSET: usize = SYS_TRACE_RANGE.end;
pub const DECODER_TRACE_WIDTH: usize = 23;
pub const DECODER_TRACE_RANGE: Range<usize> = range(DECODER_TRACE_OFFSET, DECODER_TRACE_WIDTH);

// Stack trace
pub const STACK_TRACE_OFFSET: usize = DECODER_TRACE_RANGE.end;
pub const STACK_TRACE_WIDTH: usize = 19;
pub const STACK_TRACE_RANGE: Range<usize> = range(STACK_TRACE_OFFSET, STACK_TRACE_WIDTH);

// Range check trace
pub const RANGE_CHECK_TRACE_OFFSET: usize = STACK_TRACE_RANGE.end;
pub const RANGE_CHECK_TRACE_WIDTH: usize = 4;
pub const RANGE_CHECK_TRACE_RANGE: Range<usize> =
    range(RANGE_CHECK_TRACE_OFFSET, RANGE_CHECK_TRACE_WIDTH);

// Chiplets trace
pub const CHIPLETS_OFFSET: usize = RANGE_CHECK_TRACE_RANGE.end;
pub const CHIPLETS_WIDTH: usize = 18;
pub const CHIPLETS_RANGE: Range<usize> = range(CHIPLETS_OFFSET, CHIPLETS_WIDTH);

pub const TRACE_WIDTH: usize = CHIPLETS_OFFSET + CHIPLETS_WIDTH;

// AUXILIARY COLUMNS LAYOUT
// ------------------------------------------------------------------------------------------------

//      decoder         stack       range checks      hasher         chiplets
//    (3 columns)     (1 column)     (3 columns)    (1 column)      (1 column)
// ├───────────────┴──────────────┴──────────────┴───────────────┴───────────────┤

// Decoder auxiliary columns
pub const DECODER_AUX_TRACE_OFFSET: usize = 0;
pub const DECODER_AUX_TRACE_WIDTH: usize = 3;
pub const DECODER_AUX_TRACE_RANGE: Range<usize> =
    range(DECODER_AUX_TRACE_OFFSET, DECODER_AUX_TRACE_WIDTH);

// Stack auxiliary columns
pub const STACK_AUX_TRACE_OFFSET: usize = DECODER_AUX_TRACE_RANGE.end;
pub const STACK_AUX_TRACE_WIDTH: usize = 1;
pub const STACK_AUX_TRACE_RANGE: Range<usize> =
    range(STACK_AUX_TRACE_OFFSET, STACK_AUX_TRACE_WIDTH);

// Range check auxiliary columns
pub const RANGE_CHECK_AUX_TRACE_OFFSET: usize = STACK_AUX_TRACE_RANGE.end;
pub const RANGE_CHECK_AUX_TRACE_WIDTH: usize = 3;
pub const RANGE_CHECK_AUX_TRACE_RANGE: Range<usize> =
    range(RANGE_CHECK_AUX_TRACE_OFFSET, RANGE_CHECK_AUX_TRACE_WIDTH);

// Chiplets auxiliary columns
pub const CHIPLETS_AUX_TRACE_OFFSET: usize = HASHER_AUX_TRACE_RANGE.end;
pub const CHIPLETS_AUX_TRACE_WIDTH: usize = 1;
pub const CHIPLETS_AUX_TRACE_RANGE: Range<usize> =
    range(CHIPLETS_AUX_TRACE_OFFSET, CHIPLETS_AUX_TRACE_WIDTH);

// Hasher auxiliary columns
pub const HASHER_AUX_TRACE_OFFSET: usize = RANGE_CHECK_AUX_TRACE_RANGE.end;
pub const HASHER_AUX_TRACE_WIDTH: usize = 1;
pub const HASHER_AUX_TRACE_RANGE: Range<usize> =
    range(HASHER_AUX_TRACE_OFFSET, HASHER_AUX_TRACE_WIDTH);

pub const AUX_TRACE_WIDTH: usize = CHIPLETS_AUX_TRACE_RANGE.end;

/// Number of random elements available to the prover after the commitment to the main trace
/// segment.
pub const AUX_TRACE_RAND_ELEMENTS: usize = 16;
