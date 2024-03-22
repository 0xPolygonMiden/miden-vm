#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// This is an implementation of `std::assert_matches::assert_matches`
/// so it can be removed when that feature stabilizes upstream
#[macro_export]
macro_rules! assert_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                panic!(r#"
assertion failed: `(left matches right)`
    left: `{:?}`,
    right: `{}`"#, left_val, stringify!($($pattern)|+ $(if $guard)?));
            }
        }
    };

    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $msg:literal $(,)?) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                panic!(concat!(r#"
assertion failed: `(left matches right)`
    left: `{:?}`,
    right: `{}`
"#, $msg), left_val, stringify!($($pattern)|+ $(if $guard)?));
            }
        }
    };

    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $msg:literal, $($arg:tt)+) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                panic!(concat!(r#"
assertion failed: `(left matches right)`
    left: `{:?}`,
    right: `{}`
"#, $msg), left_val, stringify!($($pattern)|+ $(if $guard)?), $($arg)+);
            }
        }
    }
}

pub mod chiplets;
pub mod errors;

pub use miden_crypto::{Word, EMPTY_WORD, ONE, WORD_SIZE, ZERO};
pub mod crypto {
    pub mod merkle {
        pub use miden_crypto::merkle::{
            DefaultMerkleStore, EmptySubtreeRoots, InnerNodeInfo, LeafIndex, MerkleError,
            MerklePath, MerkleStore, MerkleTree, Mmr, MmrPeaks, NodeIndex, PartialMerkleTree,
            RecordingMerkleStore, SimpleSmt, Smt, SmtProof, SmtProofError, StoreNode, SMT_DEPTH,
        };
    }

    pub mod hash {
        pub use miden_crypto::hash::{
            blake::{Blake3Digest, Blake3_160, Blake3_192, Blake3_256},
            rpo::{Rpo256, RpoDigest},
            ElementHasher, Hasher,
        };
    }

    pub mod random {
        pub use miden_crypto::rand::{
            RandomCoin, RandomCoinError, RpoRandomCoin, WinterRandomCoin,
        };
    }

    pub mod dsa {
        pub use miden_crypto::dsa::rpo_falcon512;
    }
}

pub use math::{
    fields::{f64::BaseElement as Felt, QuadExtension},
    polynom, ExtensionOf, FieldElement, StarkField, ToElements,
};

#[cfg(feature = "formatter")]
pub mod prettier;

mod program;
pub use program::{blocks as code_blocks, CodeBlockTable, Kernel, Program, ProgramInfo};

mod operations;
pub use operations::{
    AdviceInjector, AssemblyOp, DebugOptions, Decorator, DecoratorIterator, DecoratorList,
    Operation, SignatureKind,
};

pub mod stack;
pub use stack::{StackInputs, StackOutputs};

pub mod utils;

// TYPE ALIASES
// ================================================================================================

pub type StackTopState = [Felt; stack::STACK_TOP_SIZE];
