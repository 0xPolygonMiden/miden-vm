#![cfg_attr(not(feature = "std"), no_std)]

use air::{ProcessorAir, PublicInputs};
use processor::ExecutionTrace;
use prover::Prover;
use vm_core::{utils::collections::Vec, Felt};

#[cfg(feature = "std")]
use log::debug;
#[cfg(feature = "std")]
use prover::Trace;
#[cfg(feature = "std")]
use std::time::Instant;

// EXPORTS
// ================================================================================================

pub use air::{FieldExtension, HashFunction, ProofOptions};
pub use processor::ExecutionError;
pub use prover::StarkProof;
pub use vm_core::{
    chiplets::hasher::Digest,
    errors::{AdviceSetError, InputError},
    AdviceSet, Program, ProgramInputs, ProgramOutputs, Word,
};

pub mod math {
    pub use vm_core::{Felt, FieldElement, StarkField};
}

pub mod utils {
    pub use vm_core::utils::collections;
}

// PROVER
// ================================================================================================

/// Executes and proves the specified `program` and returns the result together with a STARK-based proof of
/// the program's execution.
///
/// * `inputs` specifies the initial state of the stack as well as non-deterministic (secret)
///   inputs for the VM.
/// * `options` defines parameters for STARK proof generation.
///
/// # Errors
/// Returns an error if program execution or STARK proof generation fails for any reason.
pub fn prove(
    program: &Program,
    inputs: &ProgramInputs,
    options: &ProofOptions,
) -> Result<(ProgramOutputs, StarkProof), ExecutionError> {
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

    let outputs = trace.program_outputs();

    // generate STARK proof
    let prover = ExecutionProver::new(
        options.clone(),
        inputs.stack_init().to_vec(),
        outputs.clone(),
    );
    let proof = prover.prove(trace).map_err(ExecutionError::ProverError)?;

    Ok((outputs, proof))
}

// PROVER
// ================================================================================================

struct ExecutionProver {
    options: ProofOptions,
    stack_inputs: Vec<Felt>,
    outputs: ProgramOutputs,
}

impl ExecutionProver {
    pub fn new(options: ProofOptions, stack_inputs: Vec<Felt>, outputs: ProgramOutputs) -> Self {
        Self {
            options,
            stack_inputs,
            outputs,
        }
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    /// Validates the stack inputs against the provided execution trace and returns true if valid.
    fn are_inputs_valid(&self, trace: &ExecutionTrace) -> bool {
        for (input_element, trace_element) in self
            .stack_inputs
            .iter()
            .zip(trace.init_stack_state().iter())
        {
            if *input_element != *trace_element {
                return false;
            }
        }

        true
    }

    /// Validates the program outputs against the provided execution trace and returns true if valid.
    fn are_outputs_valid(&self, trace: &ExecutionTrace) -> bool {
        for (output_element, trace_element) in self
            .outputs
            .stack_top()
            .iter()
            .zip(trace.last_stack_state().iter())
        {
            if *output_element != *trace_element {
                return false;
            }
        }

        true
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
        // ensure inputs and outputs are consistent with the execution trace.
        debug_assert!(
            self.are_inputs_valid(trace),
            "provided inputs do not match the execution trace"
        );
        debug_assert!(
            self.are_outputs_valid(trace),
            "provided outputs do not match the execution trace"
        );

        PublicInputs::new(
            trace.program_hash(),
            self.stack_inputs.clone(),
            self.outputs.clone(),
        )
    }
}
