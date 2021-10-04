use air::{ProcessorAir, PublicInputs};

// EXPORTS
// ================================================================================================

pub use assembly;
pub use winterfell::{StarkProof, VerifierError};

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
    program_hash: [u8; 32],
    public_inputs: &[u128],
    outputs: &[u128],
    proof: StarkProof,
) -> Result<(), VerifierError> {
    let pub_inputs = PublicInputs::new(program_hash, public_inputs, outputs);
    winterfell::verify::<ProcessorAir>(proof, pub_inputs)
}
