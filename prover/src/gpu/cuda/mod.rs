//! This module contains GPU acceleration logic for Nvidia CUDA devices.

use std::{marker::PhantomData, println};

use air::{AuxRandElements, PartitionOptions};
use miden_gpu::{cuda::{constraint::build_constraint_commitment, merkle::MerkleTree, trace_lde::CudaTraceLde}, HashFn};
use processor::crypto::{ElementHasher, Hasher};
use tracing::info_span;
use winter_prover::{
    crypto::{Digest, VectorCommitment}, matrix::{ColMatrix, RowMatrix}, CompositionPoly, CompositionPolyTrace, ConstraintCommitment, ConstraintCompositionCoefficients, DefaultConstraintEvaluator, Prover, StarkDomain, TraceInfo, TracePolyTable
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

const DIGEST_SIZE: usize = Rpo256::DIGEST_RANGE.end - Rpo256::DIGEST_RANGE.start;

// CUDA RPO/RPX PROVER
// ================================================================================================

/// Wraps an [ExecutionProver] and provides GPU acceleration for building trace commitments.
pub(crate) struct CudaExecutionProver<H, D, R>
where
    H: Hasher<Digest = D> + ElementHasher<BaseField = R::BaseField>,
    D: Digest + From<[Felt; DIGEST_SIZE]> + Into<[Felt; DIGEST_SIZE]>,
    R: RandomCoin<BaseField = Felt, Hasher = H> + Send,
{
    pub execution_prover: ExecutionProver<H, R>,
    pub hash_fn: HashFn,
    phantom_data: PhantomData<D>,
}

impl<H, D, R> CudaExecutionProver<H, D, R>
where
    H: Hasher<Digest = D> + ElementHasher<BaseField = R::BaseField>,
    D: Digest + From<[Felt; DIGEST_SIZE]> + Into<[Felt; DIGEST_SIZE]>,
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
    D: Digest + From<[Felt; DIGEST_SIZE]> + Into<[Felt; DIGEST_SIZE]>,
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
        partition_options: PartitionOptions,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        CudaTraceLde::new(trace_info, main_trace, domain, partition_options, self.hash_fn)
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

    fn build_constraint_commitment<E>(
        &self,
        composition_poly_trace: CompositionPolyTrace<E>,
        num_constraint_composition_columns: usize,
        domain: &StarkDomain<Self::BaseField>,
    ) -> (ConstraintCommitment<E, Self::HashFn, Self::VC>, CompositionPoly<E>)
    where
        E: FieldElement<BaseField = Self::BaseField>,
    {
        // first, build constraint composition polynomial from its trace as follows:
        // - interpolate the trace into a polynomial in coefficient form
        // - "break" the polynomial into a set of column polynomials each of degree equal to
        //   trace_length - 1
        let composition_poly = info_span!(
            "build_composition_poly_columns",
            num_columns = num_constraint_composition_columns
        )
        .in_scope(|| {
            CompositionPoly::new(composition_poly_trace, domain, num_constraint_composition_columns)
        });
        assert_eq!(composition_poly.num_columns(), num_constraint_composition_columns);
        assert_eq!(composition_poly.column_degree(), domain.trace_length() - 1);


        // LDE on GPU
        println!("GPU start");
        let (lde, commitment) = build_constraint_commitment::<E, Self::HashFn, _>(&composition_poly, domain, self.hash_fn, self.options().partition_options());
        println!("GPU end");


        // assert_eq!(polys.num_cols(), composition_poly.num_columns());
        // assert_eq!(polys.num_rows(), composition_poly.column_len());
        // for i in 0..polys.num_cols() {
        //     for j in 0..polys.num_rows() {
        //         assert_eq!(polys.get(i, j), composition_poly.data.get(i, j));
        //     }
        // }

        // then, evaluate composition polynomial columns over the LDE domain
        // let domain_size = domain.lde_domain_size();
        // let composed_evaluations = info_span!("evaluate_composition_poly_columns").in_scope(|| {
        //     RowMatrix::evaluate_polys_over::<8>(composition_poly.data(), domain)
        // });
        // assert_eq!(composed_evaluations.num_cols(), num_constraint_composition_columns);
        // assert_eq!(composed_evaluations.num_rows(), domain_size);

        // println!(
        //     "extended constraints from {} x {} to {} x {}",
        //     composition_poly.num_columns(), 
        //     composition_poly.column_len(), 
        //     composed_evaluations.num_cols(), 
        //     composed_evaluations.num_rows()
        // );

        let gpu_lde = RowMatrix::<E>::new(lde, num_constraint_composition_columns);
        // println!("cpu-lde {composed_evaluations:?}");
        // println!("gpu-lde {gpu_lde:?}");

        // finally, build constraint evaluation commitment
        // let constraint_commitment = info_span!(
        //     "compute_constraint_evaluation_commitment",
        //     log_domain_size = domain_size.ilog2()
        // )
        // .in_scope(|| {
        //     let commitment = composed_evaluations
        //         .commit_to_rows::<Self::HashFn, Self::VC>(self.options().partition_options());

        //         println!("cpu-tree-leaves {:?}", commitment.leaves());
        //         println!("cpu-tree-nodes {:?}", commitment.nodes());
        //         println!("cpu-commitment-root {:?}", commitment.commitment());

        //     ConstraintCommitment::new(composed_evaluations, commitment)
        // });

        // assert_eq!(constraint_commitment.commitment(), tree.root());

        let constraint_commitment = ConstraintCommitment::new(gpu_lde, commitment);
        (constraint_commitment, composition_poly)
    }
}
