#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

// EXPORTS
// ================================================================================================

pub use assembly::{Assembler, AssemblyError, ParsingError};
pub use processor::{
    execute, execute_iter, utils, AdviceInputs, AdviceProvider, AsmOpInfo, ExecutionError,
    ExecutionTrace, Kernel, MemAdviceProvider, Operation, ProgramInfo, StackInputs, VmState,
    VmStateIterator,
};
pub use prover::{
    math, prove, Digest, FieldExtension, HashFunction, InputError, MerkleError, MerkleSet, Program,
    ProofOptions, StackOutputs, StarkProof, Word,
};
pub use verifier::{verify, VerificationError};
