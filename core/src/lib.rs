#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

pub mod chiplets;
pub mod errors;

pub use miden_crypto::{Word, EMPTY_WORD, ONE, WORD_SIZE, ZERO};
pub mod crypto {
    pub mod merkle {
        pub use miden_crypto::merkle::{
            DefaultMerkleStore, EmptySubtreeRoots, InnerNodeInfo, MerkleError, MerklePath,
            MerkleStore, MerkleTree, Mmr, MmrPeaks, NodeIndex, PartialMerkleTree,
            RecordingMerkleStore, SimpleSmt, StoreNode, TieredSmt,
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
        pub use miden_crypto::rand::RpoRandomCoin;
        pub use winter_crypto::{
            DefaultRandomCoin as WinterRandomCoin, RandomCoin, RandomCoinError,
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
