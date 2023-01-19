#![cfg_attr(not(feature = "std"), no_std)]

use air::{ProcessorAir, PublicInputs};
use processor::{math::Felt, ExecutionTrace};
use prover::Prover;

#[cfg(feature = "std")]
use log::debug;
#[cfg(feature = "std")]
use prover::Trace;
#[cfg(feature = "std")]
use std::time::Instant;

// EXPORTS
// ================================================================================================

pub use air::{FieldExtension, HashFunction, ProofOptions};
pub use processor::{
    math, utils, AdviceInputs, AdviceProvider, AdviceSet, AdviceSetError, Digest, ExecutionError,
    InputError, MemAdviceProvider, Program, StackInputs, StackOutputs, Word,
};
pub use prover::StarkProof;

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
pub fn prove<A>(
    program: &Program,
    stack_inputs: StackInputs,
    advice_provider: A,
    options: &ProofOptions,
) -> Result<(StackOutputs, StarkProof), ExecutionError>
where
    A: AdviceProvider,
{
    // execute the program to create an execution trace
    #[cfg(feature = "std")]
    let now = Instant::now();
    let trace = processor::execute(program, stack_inputs.clone(), advice_provider)?;
    #[cfg(feature = "std")]
    debug!(
        "Generated execution trace of {} columns and {} steps in {} ms",
        trace.layout().main_trace_width(),
        trace.length(),
        now.elapsed().as_millis()
    );

    let stack_outputs = trace.stack_outputs().clone();

    // generate STARK proof
    let prover = ExecutionProver::new(options.clone(), stack_inputs, stack_outputs.clone());
    let proof = prover.prove(trace).map_err(ExecutionError::ProverError)?;

    Ok((stack_outputs, proof))
}

// PROVER
// ================================================================================================

struct ExecutionProver {
    options: ProofOptions,
    stack_inputs: StackInputs,
    stack_outputs: StackOutputs,
}

impl ExecutionProver {
    pub fn new(
        options: ProofOptions,
        stack_inputs: StackInputs,
        stack_outputs: StackOutputs,
    ) -> Self {
        Self {
            options,
            stack_inputs,
            stack_outputs,
        }
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    /// Validates the stack inputs against the provided execution trace and returns true if valid.
    fn are_inputs_valid(&self, trace: &ExecutionTrace) -> bool {
        for (input_element, trace_element) in
            self.stack_inputs.values().iter().zip(trace.init_stack_state().iter())
        {
            if *input_element != *trace_element {
                return false;
            }
        }

        true
    }

    /// Validates the stack outputs against the provided execution trace and returns true if valid.
    fn are_outputs_valid(&self, trace: &ExecutionTrace) -> bool {
        for (output_element, trace_element) in
            self.stack_outputs.stack_top().iter().zip(trace.last_stack_state().iter())
        {
            if *output_element != *trace_element {
                return false;
            }
        }

        true
    }
}

impl Prover for ExecutionProver {
    type Air = ProcessorAir;
    type BaseField = Felt;
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

        let program_info = trace.program_info().clone();
        PublicInputs::new(program_info, self.stack_inputs.clone(), self.stack_outputs.clone())
    }
}
