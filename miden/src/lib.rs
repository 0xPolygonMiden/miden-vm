use air::{ProcessorAir, PublicInputs};
use processor::{ExecutionError, ExecutionTrace};
use prover::Prover;
use vm_core::{Felt, StarkField, MIN_STACK_DEPTH};

#[cfg(feature = "std")]
use log::debug;
#[cfg(feature = "std")]
use prover::Trace;
#[cfg(feature = "std")]
use std::time::Instant;

// EXPORTS
// ================================================================================================

pub use air::{FieldExtension, HashFunction, ProofOptions};
pub use assembly::{Assembler, AssemblyError};
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
/// * `num_stack_outputs` specifies the number of elements from the top of the stack to be
///   returned.
/// * `options` defines parameters for STARK proof generation.
///
/// # Errors
/// Returns an error if program execution or STARK proof generation fails for any reason.
pub fn execute(
    program: &Script,
    inputs: &ProgramInputs,
    num_stack_outputs: usize,
    options: &ProofOptions,
) -> Result<(Vec<u64>, StarkProof), ExecutionError> {
    if num_stack_outputs > MIN_STACK_DEPTH {
        return Err(ExecutionError::TooManyStackOutputs(num_stack_outputs));
    }

    // execute the program to create an execution trace
    #[cfg(feature = "std")]
    let now = Instant::now();
    let trace = processor::execute(program, inputs)?;
    #[cfg(feature = "std")]
    debug!(
        "Generated execution trace of {} columns and {} steps in {} ms",
        trace.layout().main_trace_width(),
        trace.length(),
        now.elapsed().as_millis()
    );

    // copy the stack state at the last step to return as output
    let outputs = trace.last_stack_state()[..num_stack_outputs]
        .iter()
        .map(|&v| v.as_int())
        .collect::<Vec<_>>();

    // generate STARK proof
    let num_stack_inputs = inputs.stack_init().len();
    let prover = ExecutionProver::new(options.clone(), num_stack_inputs, num_stack_outputs);
    let proof = prover.prove(trace).map_err(ExecutionError::ProverError)?;

    Ok((outputs, proof))
}

// PROVER
// ================================================================================================

struct ExecutionProver {
    options: ProofOptions,
    num_stack_inputs: usize,
    num_stack_outputs: usize,
}

impl ExecutionProver {
    pub fn new(options: ProofOptions, num_stack_inputs: usize, num_stack_outputs: usize) -> Self {
        Self {
            options,
            num_stack_inputs,
            num_stack_outputs,
        }
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
            trace.init_stack_state()[..self.num_stack_inputs].to_vec(),
            trace.last_stack_state()[..self.num_stack_outputs].to_vec(),
        )
    }
}
