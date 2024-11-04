//! This module contains GPU acceleration logic for Nvidia CUDA devices.

use std::marker::PhantomData;

use air::{AuxRandElements, PartitionOptions};
use miden_gpu::{cuda::{merkle::MerkleTree, trace_lde::CudaTraceLde}, HashFn};
use processor::crypto::{ElementHasher, Hasher};
use winter_prover::{
    crypto::Digest,
    matrix::{ColMatrix, RowMatrix},
    CompositionPoly, CompositionPolyTrace, ConstraintCommitment, ConstraintCompositionCoefficients,
    DefaultConstraintEvaluator, Prover, StarkDomain, Trace, TraceInfo, TraceLde, TracePolyTable,
};

use crate::{
    crypto::{RandomCoin, Rpo256},
    ExecutionProver, ExecutionTrace, Felt, FieldElement, ProcessorAir, PublicInputs,
    WinterProofOptions,
};

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

// The Rate for RPO and RPX is the same
const RATE: usize = Rpo256::RATE_RANGE.end - Rpo256::RATE_RANGE.start;
const DIGEST_SIZE: usize = Rpo256::DIGEST_RANGE.end - Rpo256::DIGEST_RANGE.start;

// CUDA RPO/RPX PROVER
// ================================================================================================

/// Wraps an [ExecutionProver] and provides GPU acceleration for building trace commitments.
pub(crate) struct CudaExecutionProver<H, D, R>
where
    H: Hasher<Digest = D> + ElementHasher<BaseField = R::BaseField>,
    D: Digest + From<[Felt; DIGEST_SIZE]>,
    R: RandomCoin<BaseField = Felt, Hasher = H> + Send,
{
    pub execution_prover: ExecutionProver<H, R>,
    pub hash_fn: HashFn,
    phantom_data: PhantomData<D>,
}

impl<H, D, R> CudaExecutionProver<H, D, R>
where
    H: Hasher<Digest = D> + ElementHasher<BaseField = R::BaseField>,
    D: Digest + From<[Felt; DIGEST_SIZE]>,
    R: RandomCoin<BaseField = Felt, Hasher = H> + Send,
{
    pub fn new(execution_prover: ExecutionProver<H, R>, hash_fn: HashFn) -> Self {
        CudaExecutionProver {
            execution_prover,
            hash_fn,
            phantom_data: PhantomData,
        }
    }
}

impl<H, D, R> Prover for CudaExecutionProver<H, D, R>
where
    H: Hasher<Digest = D> + ElementHasher<BaseField = R::BaseField> + Send + Sync,
    D: Digest + From<[Felt; DIGEST_SIZE]>,
    R: RandomCoin<BaseField = Felt, Hasher = H> + Send,
{
    type BaseField = Felt;
    type Air = ProcessorAir;
    type Trace = ExecutionTrace;
    type VC = MerkleTree<Self::HashFn>;
    type HashFn = H;
    type RandomCoin = R;
    type TraceLde<E: FieldElement<BaseField = Felt>> = CudaTraceLde<E, H>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField = Felt>> =
        DefaultConstraintEvaluator<'a, ProcessorAir, E>;

    fn get_pub_inputs(&self, trace: &ExecutionTrace) -> PublicInputs {
        self.execution_prover.get_pub_inputs(trace)
    }

    fn options(&self) -> &WinterProofOptions {
        self.execution_prover.options()
    }

    fn new_trace_lde<E: FieldElement<BaseField = Felt>>(
        &self,
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Felt>,
        domain: &StarkDomain<Felt>,
        _partition_options: PartitionOptions,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        CudaTraceLde::new(trace_info, main_trace, domain, self.hash_fn)
    }

    fn new_evaluator<'a, E: FieldElement<BaseField = Felt>>(
        &self,
        air: &'a ProcessorAir,
        aux_rand_elements: Option<AuxRandElements<E>>,
        composition_coefficients: ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        self.execution_prover
            .new_evaluator(air, aux_rand_elements, composition_coefficients)
    }

    fn build_aux_trace<E>(
        &self,
        main_trace: &Self::Trace,
        aux_rand_elements: &AuxRandElements<E>,
    ) -> ColMatrix<E>
    where
        E: FieldElement<BaseField = Self::BaseField>,
    {
        self.execution_prover.build_aux_trace(main_trace, aux_rand_elements)
    }
}
