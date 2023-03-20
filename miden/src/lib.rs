#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

// EXPORTS
// ================================================================================================

pub use assembly::{Assembler, AssemblyError, ParsingError};
pub use processor::{
    execute, execute_iter, utils, AdviceInputs, AdviceProvider, AsmOpInfo, Blake3_192,
    ExecutionError, ExecutionTrace, Kernel, MemAdviceProvider, Operation, ProgramInfo, Rpo256,
    StackInputs, VmState, VmStateIterator,
};
pub use prover::{
    math, prove, Digest, ExecutionProof, FieldExtension, HashFunction, InputError, MerkleError,
    Program, ProofOptions, StackOutputs, StarkProof, Word,
};
pub use verifier::{verify, VerificationError};
