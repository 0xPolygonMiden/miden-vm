//! This module contains GPU acceleration logic for Apple Silicon devices. For now the
//! logic is limited to GPU accelerating RPO 256 trace commitments.
use crate::{ExecutionProver, WinterProofOptions};
use air::{FieldElement, PublicInputs};
use elsa::FrozenVec;
use log::debug;
use ministark_gpu::{
    plan::{gen_rpo_merkle_tree, GpuRpo256RowMajor},
    utils::page_aligned_uninit_vector,
};
use pollster::block_on;
use processor::{
    crypto::{RandomCoin, Rpo256, RpoDigest},
    math::{fft, Felt},
    ExecutionTrace,
};
use std::time::Instant;
use winter_prover::{
    crypto::MerkleTree,
    matrix::{build_segments, get_evaluation_offsets, Segment},
    ColMatrix, CompositionPoly, ConstraintCommitment, Prover, RowMatrix, StarkDomain,
};

const RPO_RATE: usize = Rpo256::RATE_RANGE.end - Rpo256::RATE_RANGE.start;

/// Wraps an [ExecutionProver] and provides GPU acceleration for building Rpo256 trace commitments.
pub(crate) struct GpuRpoExecutionProver<R>(pub ExecutionProver<Rpo256, R>)
where
    R: RandomCoin<BaseField = Felt, Hasher = Rpo256>;

impl<R> Prover for GpuRpoExecutionProver<R>
where
    R: RandomCoin<BaseField = Felt, Hasher = Rpo256>,
{
    type Air = <ExecutionProver<Rpo256, R> as Prover>::Air;
    type BaseField = Felt;
    type Trace = <ExecutionProver<Rpo256, R> as Prover>::Trace;
    type HashFn = Rpo256;
    type RandomCoin = R;

    fn options(&self) -> &WinterProofOptions {
        self.0.options()
    }

    fn get_pub_inputs(&self, trace: &ExecutionTrace) -> PublicInputs {
        self.0.get_pub_inputs(trace)
    }

    /// Computes a low-degree extension (LDE) of the provided execution trace over the specified
    /// domain and builds a commitment to the extended trace.
    ///
    /// The extension is performed by interpolating each column of the execution trace into a
    /// polynomial of degree = trace_length - 1, and then evaluating the polynomial over the LDE
    /// domain.
    ///
    /// Trace commitment is computed by hashing each row of the extended execution trace, and then
    /// building a Merkle tree from the resulting hashes.
    ///
    /// Interpolations and evaluations are computed on the CPU while hashes are simultaneously
    /// computed on the GPU:
    ///
    /// ```text
    ///        ──────────────────────────────────────────────────────
    ///               ┌───┐   ┌────┐   ┌───┐   ┌────┐   ┌───┐
    ///  CPU:   ... ──┤fft├─┬─┤ifft├───┤fft├─┬─┤ifft├───┤fft├─┬─ ...
    ///               └───┘ │ └────┘   └───┘ │ └────┘   └───┘ │
    ///        ╴╴╴╴╴╴╴╴╴╴╴╴╴┼╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴┼╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴┼╴╴╴╴╴╴
    ///                     │ ┌──────────┐   │ ┌──────────┐   │
    ///  GPU:               └─┤   hash   │   └─┤   hash   │   └─ ...
    ///                       └──────────┘     └──────────┘
    ///        ────┼────────┼────────┼────────┼────────┼────────┼────
    ///           t=n     t=n+1    t=n+2     t=n+3   t=n+4    t=n+5
    /// ```
    fn build_trace_commitment<E>(
        &self,
        trace: &ColMatrix<E>,
        domain: &StarkDomain<Felt>,
    ) -> (RowMatrix<E>, MerkleTree<Self::HashFn>, ColMatrix<E>)
    where
        E: air::FieldElement<BaseField = Felt>,
    {
        // interpolate the execution trace
        let now = Instant::now();
        let inv_twiddles = fft::get_inv_twiddles::<E::BaseField>(trace.num_rows());
        let trace_polys = trace.columns().map(|col| {
            let mut poly = col.to_vec();
            fft::interpolate_poly(&mut poly, &inv_twiddles);
            poly
        });

        // extend the execution trace and generate hashes on the gpu
        let lde_segments = FrozenVec::new();
        let lde_domain_size = domain.lde_domain_size();
        let num_base_columns = trace.num_base_cols();
        let rpo_requires_padding = num_base_columns % RPO_RATE != 0;
        let rpo_padded_segment_idx = rpo_requires_padding.then_some(num_base_columns / RPO_RATE);
        let mut row_hasher = GpuRpo256RowMajor::<Felt>::new(lde_domain_size, rpo_requires_padding);
        let mut rpo_padded_segment: Vec<[Felt; RPO_RATE]>;
        let mut lde_segment_generator = SegmentGenerator::new(trace_polys, domain);
        let mut lde_segment_iter = lde_segment_generator.gen_segment_iter().enumerate();
        for (segment_idx, segment) in &mut lde_segment_iter {
            let segment = lde_segments.push_get(Box::new(segment));
            // check if the segment requires padding
            if rpo_padded_segment_idx.map_or(false, |pad_idx| pad_idx == segment_idx) {
                // duplicate and modify the last segment with Rpo256's padding
                // rule ("1" followed by "0"s). Our segments are already
                // padded with "0"s we only need to add the "1"s.
                let rpo_pad_column = num_base_columns % RPO_RATE;
                rpo_padded_segment = unsafe { page_aligned_uninit_vector(lde_domain_size) };
                rpo_padded_segment.copy_from_slice(segment);
                rpo_padded_segment.iter_mut().for_each(|row| row[rpo_pad_column] = Felt::ONE);
                row_hasher.update(&rpo_padded_segment);
                assert!(lde_segment_iter.next().is_none(), "padded segment should be the last");
                break;
            }
            row_hasher.update(segment);
        }
        let row_hashes = block_on(row_hasher.finish());
        let tree_nodes = gen_rpo_merkle_tree(&row_hashes);
        // aggregate segments at the same time as the GPU generates the merkle tree nodes
        let lde_segments = lde_segments.into_vec().into_iter().map(|p| *p).collect();
        let trace_lde = RowMatrix::from_segments(lde_segments, num_base_columns);
        let trace_polys = lde_segment_generator.into_polys().unwrap();
        let nodes = block_on(tree_nodes).into_iter().map(RpoDigest::new).collect();
        let leaves = row_hashes.into_iter().map(RpoDigest::new).collect();
        let trace_tree = MerkleTree::from_raw_parts(nodes, leaves).unwrap();
        debug!(
            "Extended (on CPU) and committed (on GPU) to an execution trace of {} columns from 2^{} to 2^{} steps in {} ms",
            trace_polys.num_cols(),
            trace_polys.num_rows().ilog2(),
            trace_lde.num_rows().ilog2(),
            now.elapsed().as_millis()
        );

        (trace_lde, trace_tree, trace_polys)
    }

    /// Evaluates constraint composition polynomial over the LDE domain and builds a commitment
    /// to these evaluations.
    ///
    /// The evaluation is done by evaluating each composition polynomial column over the LDE
    /// domain.
    ///
    /// The commitment is computed by hashing each row in the evaluation matrix, and then building
    /// a Merkle tree from the resulting hashes.
    ///
    /// The composition polynomial columns are evaluated on the CPU. Afterwards the commitment
    /// is computed on the GPU.
    ///
    /// ```text
    ///        ─────────────────────────────────────────────────────
    ///              ┌───┐ ┌───┐
    ///  CPU:   ... ─┤fft├─┤fft├─┐                           ┌─ ...
    ///              └───┘ └───┘ │                           │
    ///        ╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴┼╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴┼╴╴╴╴╴╴
    ///                          │ ┌──────────┐ ┌──────────┐ │
    ///  GPU:                    └─┤   hash   ├─┤   hash   ├─┘
    ///                            └──────────┘ └──────────┘
    ///        ────┼────────┼────────┼────────┼────────┼────────┼───
    ///           t=n     t=n+1    t=n+2     t=n+3   t=n+4    t=n+5
    /// ```
    // TODO: consider merging build_constraint_commitment and build_trace_commitment in Winterfell
    // * https://github.com/facebook/winterfell/pull/192
    // * https://github.com/0xPolygonMiden/miden-vm/issues/877
    fn build_constraint_commitment<E>(
        &self,
        composition_poly: &CompositionPoly<E>,
        domain: &StarkDomain<Self::BaseField>,
    ) -> ConstraintCommitment<E, Self::HashFn>
    where
        E: FieldElement<BaseField = Self::BaseField>,
    {
        // evaluate composition polynomial columns over the LDE domain
        let now = Instant::now();
        let polys = composition_poly.data();
        let blowup = domain.trace_to_lde_blowup();
        let offsets = get_evaluation_offsets::<E>(polys.num_rows(), blowup, domain.offset());
        let segments = build_segments(composition_poly.data(), domain.trace_twiddles(), &offsets);
        debug!(
            "Evaluated {} composition polynomial columns over LDE domain (2^{} elements) in {} ms",
            polys.num_cols(),
            offsets.len().ilog2(),
            now.elapsed().as_millis()
        );

        // build constraint evaluation commitment
        let now = Instant::now();
        let lde_domain_size = domain.lde_domain_size();
        let num_base_columns = polys.num_base_cols();
        let rpo_requires_padding = num_base_columns % RPO_RATE != 0;
        let rpo_padded_segment_idx = rpo_requires_padding.then_some(num_base_columns / RPO_RATE);
        let mut row_hasher = GpuRpo256RowMajor::<Felt>::new(lde_domain_size, rpo_requires_padding);
        let mut rpo_padded_segment: Vec<[Felt; RPO_RATE]>;
        for (segment_idx, segment) in segments.iter().enumerate() {
            // check if the segment requires padding
            if rpo_padded_segment_idx.map_or(false, |pad_idx| pad_idx == segment_idx) {
                // duplicate and modify the last segment with Rpo256's padding
                // rule ("1" followed by "0"s). Our segments are already
                // padded with "0"s we only need to add the "1"s.
                let rpo_pad_column = num_base_columns % RPO_RATE;
                rpo_padded_segment = unsafe { page_aligned_uninit_vector(lde_domain_size) };
                rpo_padded_segment.copy_from_slice(segment);
                rpo_padded_segment.iter_mut().for_each(|row| row[rpo_pad_column] = Felt::ONE);
                row_hasher.update(&rpo_padded_segment);
                assert_eq!(segments.len() - 1, segment_idx, "padded segment should be the last");
                break;
            }
            row_hasher.update(segment);
        }
        let row_hashes = block_on(row_hasher.finish());
        let tree_nodes = gen_rpo_merkle_tree(&row_hashes);
        // aggregate segments at the same time as the GPU generates the merkle tree nodes
        let composed_evaluations = RowMatrix::<E>::from_segments(segments, num_base_columns);
        let nodes = block_on(tree_nodes).into_iter().map(RpoDigest::new).collect();
        let leaves = row_hashes.into_iter().map(RpoDigest::new).collect();
        let commitment = MerkleTree::<Rpo256>::from_raw_parts(nodes, leaves).unwrap();
        let constraint_commitment = ConstraintCommitment::new(composed_evaluations, commitment);
        debug!(
            "Computed constraint evaluation commitment on the GPU (Merkle tree of depth {}) in {} ms",
            constraint_commitment.tree_depth(),
            now.elapsed().as_millis()
        );
        constraint_commitment
    }
}

struct SegmentGenerator<'a, E, I, const N: usize>
where
    E: FieldElement<BaseField = Felt>,
    I: IntoIterator<Item = Vec<E>>,
{
    poly_iter: I::IntoIter,
    polys: Option<ColMatrix<E>>,
    poly_offset: usize,
    offsets: Vec<E::BaseField>,
    domain: &'a StarkDomain<E::BaseField>,
}

impl<'a, E, I, const N: usize> SegmentGenerator<'a, E, I, N>
where
    E: FieldElement<BaseField = Felt>,
    I: IntoIterator<Item = Vec<E>>,
{
    fn new(polys: I, domain: &'a StarkDomain<Felt>) -> Self {
        assert!(N > 0, "batch size N must be greater than zero");
        let poly_size = domain.trace_length();
        let lde_blowup = domain.trace_to_lde_blowup();
        let offsets = get_evaluation_offsets::<E>(poly_size, lde_blowup, domain.offset());
        Self {
            poly_iter: polys.into_iter(),
            polys: None,
            poly_offset: 0,
            offsets,
            domain,
        }
    }

    /// Returns the matrix of polynomials used to generate segments.
    fn into_polys(self) -> Option<ColMatrix<E>> {
        self.polys
    }

    /// Returns a segment generating iterator.
    fn gen_segment_iter(&mut self) -> SegmentIterator<'a, '_, E, I, N> {
        SegmentIterator(self)
    }

    /// Generates the next segment if it exists otherwise returns None.
    fn gen_next_segment(&mut self) -> Option<Segment<E::BaseField, N>> {
        // initialize our col matrix
        if self.polys.is_none() {
            self.polys = Some(ColMatrix::new(vec![self.poly_iter.next()?]));
        }

        let offset = self.poly_offset;
        let polys = self.polys.as_mut().unwrap();
        while polys.num_base_cols() < offset + N {
            if let Some(poly) = self.poly_iter.next() {
                polys.merge_column(poly)
            } else {
                break;
            }
        }

        // terminate if there are no more segments to create
        if polys.num_base_cols() <= offset {
            return None;
        }

        let domain_size = self.domain.lde_domain_size();
        let mut data = unsafe { page_aligned_uninit_vector(domain_size) };
        if polys.num_base_cols() < offset + N {
            // the segment will remain unfilled so we pad it with zeros
            data.fill([E::BaseField::ZERO; N]);
        }

        let twiddles = self.domain.trace_twiddles();
        let segment = Segment::new_with_buffer(data, &*polys, offset, &self.offsets, twiddles);
        self.poly_offset += N;
        Some(segment)
    }
}

struct SegmentIterator<'a, 'b, E, I, const N: usize>(&'b mut SegmentGenerator<'a, E, I, N>)
where
    E: FieldElement<BaseField = Felt>,
    I: IntoIterator<Item = Vec<E>>;

impl<'a, 'b, E, I, const N: usize> Iterator for SegmentIterator<'a, 'b, E, I, N>
where
    E: FieldElement<BaseField = Felt>,
    I: IntoIterator<Item = Vec<E>>,
{
    type Item = Segment<E::BaseField, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.gen_next_segment()
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use air::{ProofOptions, StarkField};
    use processor::{crypto::RpoRandomCoin, StackInputs, StackOutputs};
    use winter_prover::math::fields::CubeExtension;

    #[test]
    fn build_trace_commitment_on_gpu_with_padding_matches_cpu() {
        let cpu_prover = create_test_prover();
        let gpu_prover = GpuRpoExecutionProver(create_test_prover());
        let num_rows = 1 << 8;
        let trace = gen_random_trace(num_rows, RPO_RATE + 1);
        let domain = StarkDomain::from_twiddles(fft::get_twiddles(num_rows), 8, Felt::GENERATOR);
        let (cpu_lde, cpu_mt, cpu_polys) = cpu_prover.build_trace_commitment(&trace, &domain);

        let (gpu_lde, gpu_mt, gpu_polys) = gpu_prover.build_trace_commitment(&trace, &domain);

        assert_eq!(cpu_lde.data(), gpu_lde.data());
        assert_eq!(cpu_mt.root(), gpu_mt.root());
        assert_eq!(cpu_polys.into_columns(), gpu_polys.into_columns());
    }

    #[test]
    fn build_trace_commitment_on_gpu_without_padding_matches_cpu() {
        let cpu_prover = create_test_prover();
        let gpu_prover = GpuRpoExecutionProver(create_test_prover());
        let num_rows = 1 << 8;
        let trace = gen_random_trace(num_rows, RPO_RATE);
        let domain = StarkDomain::from_twiddles(fft::get_twiddles(num_rows), 8, Felt::GENERATOR);
        let (cpu_lde, cpu_mt, cpu_polys) = cpu_prover.build_trace_commitment(&trace, &domain);

        let (gpu_lde, gpu_mt, gpu_polys) = gpu_prover.build_trace_commitment(&trace, &domain);

        assert_eq!(cpu_lde.data(), gpu_lde.data());
        assert_eq!(cpu_mt.root(), gpu_mt.root());
        assert_eq!(cpu_polys.into_columns(), gpu_polys.into_columns());
    }

    #[test]
    fn build_constraint_commitment_on_gpu_with_padding_matches_cpu() {
        let cpu_prover = create_test_prover();
        let gpu_prover = GpuRpoExecutionProver(create_test_prover());
        let num_rows = 1 << 8;
        let ce_blowup_factor = 2;
        let coeffs = gen_random_coeffs::<CubeExtension<Felt>>(num_rows * ce_blowup_factor);
        let composition_poly = CompositionPoly::new(coeffs, num_rows);
        let domain = StarkDomain::from_twiddles(fft::get_twiddles(num_rows), 8, Felt::GENERATOR);
        let commitment_cpu = cpu_prover.build_constraint_commitment(&composition_poly, &domain);

        let commitment_gpu = gpu_prover.build_constraint_commitment(&composition_poly, &domain);

        assert_eq!(commitment_cpu.root(), commitment_gpu.root());
        assert_ne!(0, composition_poly.data().num_base_cols() % RPO_RATE);
    }

    #[test]
    fn build_constraint_commitment_on_gpu_without_padding_matches_cpu() {
        let cpu_prover = create_test_prover();
        let gpu_prover = GpuRpoExecutionProver(create_test_prover());
        let num_rows = 1 << 8;
        let ce_blowup_factor = 8;
        let coeffs = gen_random_coeffs::<Felt>(num_rows * ce_blowup_factor);
        let composition_poly = CompositionPoly::new(coeffs, num_rows);
        let domain = StarkDomain::from_twiddles(fft::get_twiddles(num_rows), 8, Felt::GENERATOR);
        let commitment_cpu = cpu_prover.build_constraint_commitment(&composition_poly, &domain);

        let commitment_gpu = gpu_prover.build_constraint_commitment(&composition_poly, &domain);

        assert_eq!(commitment_cpu.root(), commitment_gpu.root());
        assert_eq!(0, composition_poly.data().num_base_cols() % RPO_RATE);
    }

    fn gen_random_trace(num_rows: usize, num_cols: usize) -> ColMatrix<Felt> {
        ColMatrix::new((0..num_cols as u64).map(|col| vec![Felt::new(col); num_rows]).collect())
    }

    fn gen_random_coeffs<E: FieldElement>(num_rows: usize) -> Vec<E> {
        (0..num_rows).map(|i| E::from(i as u32)).collect()
    }

    fn create_test_prover() -> ExecutionProver<Rpo256, RpoRandomCoin> {
        ExecutionProver::new(
            ProofOptions::with_128_bit_security(true),
            StackInputs::default(),
            StackOutputs::default(),
        )
    }
}
