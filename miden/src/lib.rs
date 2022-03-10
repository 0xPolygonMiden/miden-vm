use air::{ProcessorAir, PublicInputs};
use processor::{ExecutionError, ExecutionTrace};
use prover::{Prover, Trace};
use vm_core::{Felt, StarkField, MIN_STACK_DEPTH};

#[cfg(feature = "std")]
use log::debug;
#[cfg(feature = "std")]
use std::time::Instant;

//#[cfg(test)]
//mod tests;

// EXPORTS
// ================================================================================================

pub use air::{FieldExtension, HashFunction, ProofOptions};
pub use assembly;
pub use prover::StarkProof;
pub use verifier::{verify, VerificationError};
pub use vm_core::{program::Script, ProgramInputs};

// EXECUTOR
// ================================================================================================

/// Executes the specified `program` and returns the result together with a STARK-based proof of
/// the program's execution.
///
/// * `inputs` specifies the initial state of the stack as well as non-deterministic (secret)
///   inputs for the VM.
/// * `num_outputs` specifies the number of elements from the top of the stack to be returned.
/// * `options` defines parameters for STARK proof generation.
///
/// # Errors
/// Returns an error if program execution or STARK proof generation fails for any reason.
pub fn execute(
    program: &Script,
    inputs: &ProgramInputs,
    num_outputs: usize,
    options: &ProofOptions,
) -> Result<(Vec<u64>, StarkProof), ExecutionError> {
    assert!(
        num_outputs <= MIN_STACK_DEPTH,
        "cannot produce more than {} outputs, but requested {}",
        MIN_STACK_DEPTH,
        num_outputs
    );

    // execute the program to create an execution trace
    #[cfg(feature = "std")]
    let now = Instant::now();
    let trace = processor::execute(program, inputs)?;
    #[cfg(feature = "std")]
    debug!(
        "Generated execution trace of {} columns and {} steps in {} ms",
        trace.width(),
        trace.length(),
        now.elapsed().as_millis()
    );

    // copy the stack state at the last step to return as output
    let outputs = trace.last_stack_state()[..num_outputs]
        .iter()
        .map(|&v| v.as_int())
        .collect::<Vec<_>>();

    // generate STARK proof
    let prover = ExecutionProver::new(options.clone());
    let proof = prover.prove(trace).map_err(ExecutionError::ProverError)?;

    Ok((outputs, proof))
}

// PROVER
// ================================================================================================

struct ExecutionProver {
    options: ProofOptions,
}

impl ExecutionProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }
}

impl Prover for ExecutionProver {
    type BaseField = Felt;
    type Air = ProcessorAir;
    type Trace = ExecutionTrace;

    fn options(&self) -> &prover::ProofOptions {
        &self.options
    }

    fn get_pub_inputs(&self, trace: &ExecutionTrace) -> PublicInputs {
        PublicInputs::new(
            trace.program_hash(),
            trace.init_stack_state(),
            trace.last_stack_state(),
        )
    }
}
