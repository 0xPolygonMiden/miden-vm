#![cfg_attr(not(feature = "std"), no_std)]

// EXPORTS
// ================================================================================================

pub use assembly::{Assembler, AssemblyError, ParsingError};
pub use processor::{
    execute, execute_iter, AsmOpInfo, ExecutionError, ExecutionTrace, Operation, VmState,
    VmStateIterator,
};
pub use prover::{
    math, prove, utils, AdviceSet, AdviceSetError, Digest, FieldExtension, HashFunction,
    InputError, Program, ProgramInputs, ProgramOutputs, ProofOptions, StarkProof, Word,
};
pub use verifier::{verify, VerificationError};
