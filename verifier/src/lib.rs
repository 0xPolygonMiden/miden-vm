#![cfg_attr(not(feature = "std"), no_std)]

use air::{ProcessorAir, PublicInputs};
use core::fmt;
pub use vm_core::{Kernel, ProgramInfo, StackInputs};
pub use winterfell::VerifierError;

// EXPORTS
// ================================================================================================

pub use vm_core::{chiplets::hasher::Digest, StackOutputs, Word};
pub use winterfell::StarkProof;

pub mod math {
    pub use vm_core::{Felt, FieldElement, StarkField};
}

// VERIFIER
// ================================================================================================
/// Returns Ok(()) if the specified program was executed correctly against the specified inputs
/// and outputs.
///
/// Specifically, verifies that if a program with the specified `program_hash` is executed against
/// the provided `stack_inputs` and some secret inputs, the result is equal to the `stack_outputs`.
///
/// Stack inputs are expected to be ordered as if they would be pushed onto the stack one by one.
/// Thus, their expected order on the stack will be the reverse of the order in which they are
/// provided, and the last value in the `stack_inputs` slice is expected to be the value at the top
/// of the stack.
///
/// Stack outputs are expected to be ordered as if they would be popped off the stack one by one.
/// Thus, the value at the top of the stack is expected to be in the first position of the
/// `stack_outputs` slice, and the order of the rest of the output elements will also match the
/// order on the stack. This is the reverse of the order of the `stack_inputs` slice.
///
/// # Errors
/// Returns an error if the provided proof does not prove a correct execution of the program.
pub fn verify(
    program_info: ProgramInfo,
    stack_inputs: StackInputs,
    stack_outputs: StackOutputs,
    proof: StarkProof,
) -> Result<(), VerificationError> {
    // build public inputs and try to verify the proof
    let pub_inputs = PublicInputs::new(program_info, stack_inputs, stack_outputs);
    winterfell::verify::<ProcessorAir>(proof, pub_inputs).map_err(VerificationError::VerifierError)
}

// ERRORS
// ================================================================================================

/// TODO: add docs, implement Display
#[derive(Debug, PartialEq, Eq)]
pub enum VerificationError {
    VerifierError(VerifierError),
    InputNotFieldElement(u64),
    OutputNotFieldElement(u64),
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: implement friendly messages
        write!(f, "{self:?}")
    }
}
