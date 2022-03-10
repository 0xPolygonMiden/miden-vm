use air::{ProcessorAir, PublicInputs};
use vm_core::{Felt, FieldElement, MIN_STACK_DEPTH};
use winterfell::VerifierError;

// EXPORTS
// ================================================================================================

pub use assembly;
pub use vm_core::hasher::Digest;
pub use winterfell::StarkProof;

// VERIFIER
// ================================================================================================
/// Returns Ok(()) if the specified program was executed correctly against the specified inputs
/// and outputs.
///
/// Specifically, verifies that if a program with the specified `program_hash` is executed with the
/// provided `public_inputs` and some secret inputs, and the result is equal to the `outputs`.
///
/// # Errors
/// Returns an error if the provided proof does not prove a correct execution of the program.
pub fn verify(
    program_hash: Digest,
    public_inputs: &[u64],
    outputs: &[u64],
    proof: StarkProof,
) -> Result<(), VerificationError> {
    // build initial stack state from public inputs
    if public_inputs.len() > MIN_STACK_DEPTH {
        return Err(VerificationError::TooManyInputValues(
            MIN_STACK_DEPTH,
            public_inputs.len(),
        ));
    }

    let mut init_stack_state = [Felt::ZERO; MIN_STACK_DEPTH];
    for (element, &value) in init_stack_state.iter_mut().zip(public_inputs.iter().rev()) {
        *element = value
            .try_into()
            .map_err(|_| VerificationError::InputNotFieldElement(value))?;
    }

    // build final stack state from outputs
    if outputs.len() > MIN_STACK_DEPTH {
        return Err(VerificationError::TooManyOutputValues(
            MIN_STACK_DEPTH,
            outputs.len(),
        ));
    }

    let mut last_stack_state = [Felt::ZERO; MIN_STACK_DEPTH];
    for (element, &value) in last_stack_state.iter_mut().zip(outputs.iter().rev()) {
        *element = value
            .try_into()
            .map_err(|_| VerificationError::OutputNotFieldElement(value))?;
    }

    // build public inputs and try to verify the proof
    let pub_inputs = PublicInputs::new(program_hash, init_stack_state, last_stack_state);
    winterfell::verify::<ProcessorAir>(proof, pub_inputs).map_err(VerificationError::VerifierError)
}

// ERRORS
// ================================================================================================

/// TODO: add docs, implement Display
#[derive(Debug, PartialEq)]
pub enum VerificationError {
    VerifierError(VerifierError),
    InputNotFieldElement(u64),
    TooManyInputValues(usize, usize),
    OutputNotFieldElement(u64),
    TooManyOutputValues(usize, usize),
}
