#![no_std]

#[cfg_attr(all(feature = "metal", target_arch = "aarch64", target_os = "macos"), macro_use)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use core::marker::PhantomData;

use air::{trace::{AUX_TRACE_WIDTH, TRACE_WIDTH}, AuxRandElements, PartitionOptions, ProcessorAir, PublicInputs};
#[cfg(all(target_arch = "x86_64", feature = "cuda"))]
use miden_gpu::cuda::util::CudaStorageOwned;
#[cfg(any(
    all(feature = "metal", target_arch = "aarch64", target_os = "macos"),
    all(feature = "cuda", target_arch = "x86_64")
))]
use miden_gpu::HashFn;
use processor::{
    crypto::{
        Blake3_192, Blake3_256, ElementHasher, RandomCoin, Rpo256, RpoRandomCoin, Rpx256,
        RpxRandomCoin, WinterRandomCoin,
    },
    math::{Felt, FieldElement},
    ExecutionTrace, Program,
};
use tracing::instrument;
use winter_maybe_async::{maybe_async, maybe_await};
use winter_prover::{
    matrix::ColMatrix, CompositionPoly, CompositionPolyTrace, ConstraintCompositionCoefficients, DefaultConstraintCommitment, DefaultConstraintEvaluator, DefaultTraceLde, ProofOptions as WinterProofOptions, Prover as WinterProver, StarkDomain, TraceInfo, TracePolyTable
};
#[cfg(feature = "std")]
use {std::time::Instant, winter_prover::Trace};
mod gpu;

// EXPORTS
// ================================================================================================

pub use air::{DeserializationError, ExecutionProof, FieldExtension, HashFunction, ProvingOptions};
#[cfg(all(target_arch = "x86_64", feature = "cuda"))]
pub use miden_gpu::cuda::get_num_of_gpus;
pub use processor::{
    crypto, math, utils, AdviceInputs, Digest, ExecutionError, Host, InputError, MemAdviceProvider,
    StackInputs, StackOutputs, Word,
};
pub use winter_prover::{crypto::MerkleTree as MerkleTreeVC, Proof};

// PROVER
// ================================================================================================

pub struct Prover {
    #[cfg(all(feature = "cuda", target_arch = "x86_64"))]
    storage: CudaStorageOwned,
}

impl Prover {
    #[instrument("create_prover", skip_all)]
    pub fn new() -> Self {
        // 200000th fib number, cubic extension, blowup 16, single partition
        // TODO: make it slightly larger to eliminate code that depends on exact buffer size
        Self {
            #[cfg(all(feature = "cuda", target_arch = "x86_64"))]
            storage: CudaStorageOwned::new(1382023164 + 508559356 + 562036732),
        }
    }

    /// Executes and proves the specified `program` and returns the result together with a STARK-based
    /// proof of the program's execution.
    ///
    /// - `stack_inputs` specifies the initial state of the stack for the VM.
    /// - `host` specifies the host environment which contain non-deterministic (secret) inputs for the
    ///   prover
    /// - `options` defines parameters for STARK proof generation.
    ///
    /// # Errors
    /// Returns an error if program execution or STARK proof generation fails for any reason.
    #[instrument("prove_program", skip_all)]
    #[maybe_async]
    pub fn prove(
        &mut self,
        program: &Program,
        stack_inputs: StackInputs,
        host: &mut impl Host,
        options: ProvingOptions,
    ) -> Result<(StackOutputs, ExecutionProof), ExecutionError> {
        // execute the program to create an execution trace
        #[cfg(feature = "std")]
        let now = Instant::now();
        let trace =
            processor::execute(program, stack_inputs.clone(), host, *options.execution_options())?;
        #[cfg(feature = "std")]
        tracing::event!(
            tracing::Level::INFO,
            "Generated execution trace of {} columns and {} steps ({}% padded) in {} ms",
            trace.info().main_trace_width(),
            trace.trace_len_summary().padded_trace_len(),
            trace.trace_len_summary().padding_percentage(),
            now.elapsed().as_millis()
        );

        let stack_outputs = trace.stack_outputs().clone();
        let hash_fn = options.hash_fn();

        #[cfg(all(feature = "cuda", target_arch = "x86_64"))]
        let Some((main, aux, ce)) = self.storage.borrow_mut(
                TRACE_WIDTH,
                AUX_TRACE_WIDTH,
                trace.get_trace_len(),
                options.clone().into(),
            )
         else {
            return Err(ExecutionError::MemoryLimitExceeded(self.storage.len()));
        };

        // generate STARK proof
        let proof = match hash_fn {
            HashFunction::Blake3_192 => {
                let prover = ExecutionProver::<Blake3_192, WinterRandomCoin<_>>::new(
                    options,
                    stack_inputs,
                    stack_outputs.clone(),
                );
                maybe_await!(prover.prove(trace))
            },
            HashFunction::Blake3_256 => {
                let prover = ExecutionProver::<Blake3_256, WinterRandomCoin<_>>::new(
                    options,
                    stack_inputs,
                    stack_outputs.clone(),
                );
                maybe_await!(prover.prove(trace))
            },
            HashFunction::Rpo256 => {
                let prover = ExecutionProver::<Rpo256, RpoRandomCoin>::new(
                    options,
                    stack_inputs,
                    stack_outputs.clone(),
                );
                #[cfg(all(feature = "metal", target_arch = "aarch64", target_os = "macos"))]
                let prover = gpu::metal::MetalExecutionProver::new(prover, HashFn::Rpo256);
                #[cfg(all(feature = "cuda", target_arch = "x86_64"))]
                let prover = gpu::cuda::CudaExecutionProver::new(prover, HashFn::Rpo256, main, aux, ce);
                maybe_await!(prover.prove(trace))
            },
            HashFunction::Rpx256 => {
                let prover = ExecutionProver::<Rpx256, RpxRandomCoin>::new(
                    options,
                    stack_inputs,
                    stack_outputs.clone(),
                );
                #[cfg(all(feature = "metal", target_arch = "aarch64", target_os = "macos"))]
                let prover = gpu::metal::MetalExecutionProver::new(prover, HashFn::Rpx256);
                #[cfg(all(feature = "cuda", target_arch = "x86_64"))]
                let prover = gpu::cuda::CudaExecutionProver::new(prover, HashFn::Rpx256, main, aux, ce);
                maybe_await!(prover.prove(trace))
            },
        }
        .map_err(ExecutionError::ProverError)?;
        let proof = ExecutionProof::new(proof, hash_fn);

        Ok((stack_outputs, proof))
    }
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
            .iter()
            .zip(trace.init_stack_state().iter())
            .all(|(l, r)| l == r)
    }

    /// Validates the stack outputs against the provided execution trace and returns true if valid.
    fn are_outputs_valid(&self, trace: &ExecutionTrace) -> bool {
        self.stack_outputs
            .iter()
            .zip(trace.last_stack_state().iter())
            .all(|(l, r)| l == r)
    }
}

impl<H, R> WinterProver for ExecutionProver<H, R>
where
    H: ElementHasher<BaseField = Felt> + Sync,
    R: RandomCoin<BaseField = Felt, Hasher = H> + Send,
{
    type BaseField = Felt;
    type Air = ProcessorAir;
    type Trace = ExecutionTrace;
    type HashFn = H;
    type VC = MerkleTreeVC<Self::HashFn>;
    type RandomCoin = R;
    type TraceLde<E: FieldElement<BaseField = Felt>> = DefaultTraceLde<E, H, Self::VC>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField = Felt>> =
        DefaultConstraintEvaluator<'a, ProcessorAir, E>;
    type ConstraintCommitment<E: FieldElement<BaseField = Felt>> =
        DefaultConstraintCommitment<E, H, Self::VC>;

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

    #[maybe_async]
    fn new_trace_lde<E: FieldElement<BaseField = Felt>>(
        &self,
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Felt>,
        domain: &StarkDomain<Felt>,
        partition_options: PartitionOptions,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        DefaultTraceLde::new(trace_info, main_trace, domain, partition_options)
    }

    #[maybe_async]
    fn new_evaluator<'a, E: FieldElement<BaseField = Felt>>(
        &self,
        air: &'a ProcessorAir,
        aux_rand_elements: Option<AuxRandElements<E>>,
        composition_coefficients: ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        DefaultConstraintEvaluator::new(air, aux_rand_elements, composition_coefficients)
    }

    #[instrument(skip_all)]
    #[maybe_async]
    fn build_aux_trace<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        trace: &Self::Trace,
        aux_rand_elements: &AuxRandElements<E>,
    ) -> ColMatrix<E> {
        trace.build_aux_trace(aux_rand_elements.rand_elements()).unwrap()
    }

    #[maybe_async]
    fn build_constraint_commitment<E: FieldElement<BaseField = Felt>>(
        &self,
        composition_poly_trace: CompositionPolyTrace<E>,
        num_constraint_composition_columns: usize,
        domain: &StarkDomain<Self::BaseField>,
        partition_options: PartitionOptions,
    ) -> (Self::ConstraintCommitment<E>, CompositionPoly<E>) {
        DefaultConstraintCommitment::new(
            composition_poly_trace,
            num_constraint_composition_columns,
            domain,
            partition_options,
        )
    }
}
