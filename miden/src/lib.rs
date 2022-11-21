#![cfg_attr(not(feature = "std"), no_std)]

// EXPORTS
// ================================================================================================

pub use air::{FieldExtension, HashFunction, ProofOptions};
pub use assembly::{Assembler, AssemblerError, ParsingError};
pub use processor::{
    execute, execute_iter, AsmOpInfo, ExecutionError, ExecutionTrace, VmState, VmStateIterator,
};
pub use prover::{prove, StarkProof};
pub use verifier::{verify, VerificationError};
pub use vm_core::{
    chiplets::hasher::Digest,
    errors::{AdviceSetError, InputError},
    AdviceSet, Program, ProgramInputs,
};
