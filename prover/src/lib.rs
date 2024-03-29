#![cfg_attr(not(feature = "std"), no_std)]

use air::{ProcessorAir, PublicInputs};
use core::marker::PhantomData;
use processor::{
    crypto::{
        Blake3_192, Blake3_256, ElementHasher, RandomCoin, Rpo256, RpoRandomCoin, WinterRandomCoin,
    },
    math::{Felt, FieldElement},
    ExecutionTrace,
};
use tracing::{event, instrument, Level};
use winter_prover::{
    matrix::ColMatrix, AuxTraceRandElements, ConstraintCompositionCoefficients,
    DefaultConstraintEvaluator, DefaultTraceLde, ProofOptions as WinterProofOptions, Prover,
    StarkDomain, TraceInfo, TracePolyTable,
};

#[cfg(feature = "std")]
use {std::time::Instant, winter_prover::Trace};

#[cfg(all(feature = "metal", target_arch = "aarch64", target_os = "macos"))]
mod gpu;

// EXPORTS
// ================================================================================================

pub use air::{DeserializationError, ExecutionProof, FieldExtension, HashFunction, ProvingOptions};
pub use processor::{
    crypto, math, utils, AdviceInputs, Digest, ExecutionError, Host, InputError, MemAdviceProvider,
    Program, StackInputs, StackOutputs, Word,
};
pub use winter_prover::StarkProof;

// PROVER
// ================================================================================================

/// Executes and proves the specified `program` and returns the result together with a STARK-based
/// proof of the program's execution.
///
/// * `inputs` specifies the initial state of the stack as well as non-deterministic (secret)
///   inputs for the VM.
/// * `options` defines parameters for STARK proof generation.
///
/// # Errors
/// Returns an error if program execution or STARK proof generation fails for any reason.
#[instrument("prove_program", skip_all)]
pub fn prove<H>(
    program: &Program,
    stack_inputs: StackInputs,
    host: H,
    options: ProvingOptions,
) -> Result<(StackOutputs, ExecutionProof), ExecutionError>
where
    H: Host,
{
    // execute the program to create an execution trace
    #[cfg(feature = "std")]
    let now = Instant::now();
    let trace =
        processor::execute(program, stack_inputs.clone(), host, *options.execution_options())?;
    #[cfg(feature = "std")]
    event!(
        Level::INFO,
        "Generated execution trace of {} columns and {} steps ({}% padded) in {} ms",
        trace.layout().main_trace_width(),
        trace.trace_len_summary().padded_trace_len(),
        trace.trace_len_summary().padding_percentage(),
        now.elapsed().as_millis()
    );

    let stack_outputs = trace.stack_outputs().clone();
    let hash_fn = options.hash_fn();

    // generate STARK proof
    let proof = match hash_fn {
        HashFunction::Blake3_192 => ExecutionProver::<Blake3_192, WinterRandomCoin<_>>::new(
            options,
            stack_inputs,
            stack_outputs.clone(),
        )
        .prove(trace),
        HashFunction::Blake3_256 => ExecutionProver::<Blake3_256, WinterRandomCoin<_>>::new(
            options,
            stack_inputs,
            stack_outputs.clone(),
        )
        .prove(trace),
        HashFunction::Rpo256 => {
            let prover = ExecutionProver::<Rpo256, RpoRandomCoin>::new(
                options,
                stack_inputs,
                stack_outputs.clone(),
            );
            #[cfg(all(feature = "metal", target_arch = "aarch64", target_os = "macos"))]
            let prover = gpu::MetalRpoExecutionProver(prover);
            prover.prove(trace)
        }
    }
    .map_err(ExecutionError::ProverError)?;
    let proof = ExecutionProof::new(proof, hash_fn);

    Ok((stack_outputs, proof))
}

// PROVER
// ================================================================================================

struct ExecutionProver<H, R>
where
    H: ElementHasher<BaseField = Felt>,
    R: RandomCoin<BaseField = Felt, Hasher = H>,
{
    random_coin: PhantomData<R>,
    options: WinterProofOptions,
    stack_inputs: StackInputs,
    stack_outputs: StackOutputs,
}

impl<H, R> ExecutionProver<H, R>
where
    H: ElementHasher<BaseField = Felt>,
    R: RandomCoin<BaseField = Felt, Hasher = H>,
{
    pub fn new(
        options: ProvingOptions,
        stack_inputs: StackInputs,
        stack_outputs: StackOutputs,
    ) -> Self {
        Self {
            random_coin: PhantomData,
            options: options.into(),
            stack_inputs,
            stack_outputs,
        }
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    /// Validates the stack inputs against the provided execution trace and returns true if valid.
    fn are_inputs_valid(&self, trace: &ExecutionTrace) -> bool {
        self.stack_inputs
            .values()
            .iter()
            .zip(trace.init_stack_state().iter())
            .all(|(l, r)| l == r)
    }

    /// Validates the stack outputs against the provided execution trace and returns true if valid.
    fn are_outputs_valid(&self, trace: &ExecutionTrace) -> bool {
        self.stack_outputs
            .stack_top()
            .iter()
            .zip(trace.last_stack_state().iter())
            .all(|(l, r)| l == r)
    }
}

impl<H, R> Prover for ExecutionProver<H, R>
where
    H: ElementHasher<BaseField = Felt>,
    R: RandomCoin<BaseField = Felt, Hasher = H>,
{
    type BaseField = Felt;
    type Air = ProcessorAir;
    type Trace = ExecutionTrace;
    type HashFn = H;
    type RandomCoin = R;
    type TraceLde<E: FieldElement<BaseField = Felt>> = DefaultTraceLde<E, H>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField = Felt>> =
        DefaultConstraintEvaluator<'a, ProcessorAir, E>;

    fn options(&self) -> &WinterProofOptions {
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

    fn new_trace_lde<E: FieldElement<BaseField = Felt>>(
        &self,
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Felt>,
        domain: &StarkDomain<Felt>,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        DefaultTraceLde::new(trace_info, main_trace, domain)
    }

    fn new_evaluator<'a, E: FieldElement<BaseField = Felt>>(
        &self,
        air: &'a ProcessorAir,
        aux_rand_elements: AuxTraceRandElements<E>,
        composition_coefficients: ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        DefaultConstraintEvaluator::new(air, aux_rand_elements, composition_coefficients)
    }
}
