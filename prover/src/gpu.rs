//! This module contains GPU acceleration logic for Apple Silicon devices. For now the
//! logic is limited to GPU accelerating RPO 256 trace commitments.

use super::{
    crypto::{RandomCoin, Rpo256, RpoDigest},
    event,
    math::fft,
    ExecutionProver, ExecutionTrace, Felt, FieldElement, Level, ProcessorAir, PublicInputs,
    WinterProofOptions,
};
use elsa::FrozenVec;
use ministark_gpu::{
    plan::{gen_rpo_merkle_tree, GpuRpo256RowMajor},
    utils::page_aligned_uninit_vector,
};
use pollster::block_on;
use processor::ONE;
use std::time::Instant;
use winter_prover::{
    crypto::MerkleTree,
    matrix::{build_segments, get_evaluation_offsets, ColMatrix, RowMatrix, Segment},
    proof::Queries,
    AuxTraceRandElements, CompositionPoly, CompositionPolyTrace, ConstraintCommitment,
    ConstraintCompositionCoefficients, DefaultConstraintEvaluator, EvaluationFrame, Prover,
    StarkDomain, TraceInfo, TraceLayout, TraceLde, TracePolyTable,
};

// CONSTANTS
// ================================================================================================

const RPO_RATE: usize = Rpo256::RATE_RANGE.end - Rpo256::RATE_RANGE.start;

// METAL RPO PROVER
// ================================================================================================

/// Wraps an [ExecutionProver] and provides GPU acceleration for building Rpo256 trace commitments.
pub(crate) struct MetalRpoExecutionProver<R>(pub ExecutionProver<Rpo256, R>)
where
    R: RandomCoin<BaseField = Felt, Hasher = Rpo256>;

impl<R> Prover for MetalRpoExecutionProver<R>
where
    R: RandomCoin<BaseField = Felt, Hasher = Rpo256>,
{
    type BaseField = Felt;
    type Air = ProcessorAir;
    type Trace = ExecutionTrace;
    type HashFn = Rpo256;
    type RandomCoin = R;
    type TraceLde<E: FieldElement<BaseField = Felt>> = MetalRpoTraceLde<E>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField = Felt>> =
        DefaultConstraintEvaluator<'a, ProcessorAir, E>;

    fn options(&self) -> &WinterProofOptions {
        self.0.options()
    }

    fn get_pub_inputs(&self, trace: &ExecutionTrace) -> PublicInputs {
        self.0.get_pub_inputs(trace)
    }

    fn new_trace_lde<E: FieldElement<BaseField = Felt>>(
        &self,
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Felt>,
        domain: &StarkDomain<Felt>,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        MetalRpoTraceLde::new(trace_info, main_trace, domain)
    }

    fn new_evaluator<'a, E: FieldElement<BaseField = Felt>>(
        &self,
        air: &'a ProcessorAir,
        aux_rand_elements: AuxTraceRandElements<E>,
        composition_coefficients: ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        self.0.new_evaluator(air, aux_rand_elements, composition_coefficients)
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
    fn build_constraint_commitment<E: FieldElement<BaseField = Felt>>(
        &self,
        composition_poly_trace: CompositionPolyTrace<E>,
        num_trace_poly_columns: usize,
        domain: &StarkDomain<Felt>,
    ) -> (ConstraintCommitment<E, Rpo256>, CompositionPoly<E>) {
        // evaluate composition polynomial columns over the LDE domain
        let now = Instant::now();
        let composition_poly =
            CompositionPoly::new(composition_poly_trace, domain, num_trace_poly_columns);
        let blowup = domain.trace_to_lde_blowup();
        let offsets =
            get_evaluation_offsets::<E>(composition_poly.column_len(), blowup, domain.offset());
        let segments = build_segments(composition_poly.data(), domain.trace_twiddles(), &offsets);
        event!(
            Level::INFO,
            "Evaluated {} composition polynomial columns over LDE domain (2^{} elements) in {} ms",
            composition_poly.num_columns(),
            offsets.len().ilog2(),
            now.elapsed().as_millis()
        );

        // build constraint evaluation commitment
        let now = Instant::now();
        let lde_domain_size = domain.lde_domain_size();
        let num_base_columns =
            composition_poly.num_columns() * <E as FieldElement>::EXTENSION_DEGREE;
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
                rpo_padded_segment.iter_mut().for_each(|row| row[rpo_pad_column] = ONE);
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
        event!(
            Level::INFO,
            "Computed constraint evaluation commitment on the GPU (Merkle tree of depth {}) in {} ms",
            constraint_commitment.tree_depth(),
            now.elapsed().as_millis()
        );
        (constraint_commitment, composition_poly)
    }
}

// TRACE LOW DEGREE EXTENSION (METAL)
// ================================================================================================

/// Contains all segments of the extended execution trace, the commitments to these segments, the
/// LDE blowup factor, and the [TraceInfo].
///
/// Segments are stored in two groups:
/// - Main segment: this is the first trace segment generated by the prover. Values in this segment
///   will always be elements in the base field (even when an extension field is used).
/// - Auxiliary segments: a list of 0 or more segments for traces generated after the prover commits
///   to the first trace segment. Currently, at most 1 auxiliary segment is possible.
pub struct MetalRpoTraceLde<E: FieldElement<BaseField = Felt>> {
    // low-degree extension of the main segment of the trace
    main_segment_lde: RowMatrix<Felt>,
    // commitment to the main segment of the trace
    main_segment_tree: MerkleTree<Rpo256>,
    // low-degree extensions of the auxiliary segments of the trace
    aux_segment_ldes: Vec<RowMatrix<E>>,
    // commitment to the auxiliary segments of the trace
    aux_segment_trees: Vec<MerkleTree<Rpo256>>,
    blowup: usize,
    trace_info: TraceInfo,
}

impl<E: FieldElement<BaseField = Felt>> MetalRpoTraceLde<E> {
    /// Takes the main trace segment columns as input, interpolates them into polynomials in
    /// coefficient form, evaluates the polynomials over the LDE domain, commits to the
    /// polynomial evaluations, and creates a new [DefaultTraceLde] with the LDE of the main trace
    /// segment and the commitment.
    ///
    /// Returns a tuple containing a [TracePolyTable] with the trace polynomials for the main trace
    /// segment and the new [DefaultTraceLde].
    pub fn new(
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Felt>,
        domain: &StarkDomain<Felt>,
    ) -> (Self, TracePolyTable<E>) {
        // extend the main execution trace and build a Merkle tree from the extended trace
        let (main_segment_lde, main_segment_tree, main_segment_polys) =
            build_trace_commitment(main_trace, domain);

        let trace_poly_table = TracePolyTable::new(main_segment_polys);
        let trace_lde = MetalRpoTraceLde {
            main_segment_lde,
            main_segment_tree,
            aux_segment_ldes: Vec::new(),
            aux_segment_trees: Vec::new(),
            blowup: domain.trace_to_lde_blowup(),
            trace_info: trace_info.clone(),
        };

        (trace_lde, trace_poly_table)
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------

    /// Returns number of columns in the main segment of the execution trace.
    #[cfg(test)]
    pub fn main_segment_width(&self) -> usize {
        self.main_segment_lde.num_cols()
    }

    /// Returns a reference to [Matrix] representing the main trace segment.
    #[cfg(test)]
    pub fn get_main_segment(&self) -> &RowMatrix<Felt> {
        &self.main_segment_lde
    }

    /// Returns the entire trace for the column at the specified index.
    #[cfg(test)]
    pub fn get_main_segment_column(&self, col_idx: usize) -> Vec<Felt> {
        (0..self.main_segment_lde.num_rows())
            .map(|row_idx| self.main_segment_lde.get(col_idx, row_idx))
            .collect()
    }
}

impl<E: FieldElement<BaseField = Felt>> TraceLde<E> for MetalRpoTraceLde<E> {
    type HashFn = Rpo256;

    /// Returns the commitment to the low-degree extension of the main trace segment.
    fn get_main_trace_commitment(&self) -> RpoDigest {
        let root_hash = self.main_segment_tree.root();
        *root_hash
    }

    /// Takes auxiliary trace segment columns as input, interpolates them into polynomials in
    /// coefficient form, evaluates the polynomials over the LDE domain, and commits to the
    /// polynomial evaluations.
    ///
    /// Returns a tuple containing the column polynomials in coefficient from and the commitment
    /// to the polynomial evaluations over the LDE domain.
    ///
    /// # Panics
    ///
    /// This function will panic if any of the following are true:
    /// - the number of rows in the provided `aux_trace` does not match the main trace.
    /// - this segment would exceed the number of segments specified by the trace layout.
    fn add_aux_segment(
        &mut self,
        aux_trace: &ColMatrix<E>,
        domain: &StarkDomain<Felt>,
    ) -> (ColMatrix<E>, RpoDigest) {
        // extend the auxiliary trace segment and build a Merkle tree from the extended trace
        let (aux_segment_lde, aux_segment_tree, aux_segment_polys) =
            build_trace_commitment::<E>(aux_trace, domain);

        // check errors
        assert!(
            self.aux_segment_ldes.len() < self.trace_info.layout().num_aux_segments(),
            "the specified number of auxiliary segments has already been added"
        );
        assert_eq!(
            self.main_segment_lde.num_rows(),
            aux_segment_lde.num_rows(),
            "the number of rows in the auxiliary segment must be the same as in the main segment"
        );

        // save the lde and commitment
        self.aux_segment_ldes.push(aux_segment_lde);
        let root_hash = *aux_segment_tree.root();
        self.aux_segment_trees.push(aux_segment_tree);

        (aux_segment_polys, root_hash)
    }

    /// Reads current and next rows from the main trace segment into the specified frame.
    fn read_main_trace_frame_into(&self, lde_step: usize, frame: &mut EvaluationFrame<Felt>) {
        // at the end of the trace, next state wraps around and we read the first step again
        let next_lde_step = (lde_step + self.blowup()) % self.trace_len();

        // copy main trace segment values into the frame
        frame.current_mut().copy_from_slice(self.main_segment_lde.row(lde_step));
        frame.next_mut().copy_from_slice(self.main_segment_lde.row(next_lde_step));
    }

    /// Reads current and next rows from the auxiliary trace segment into the specified frame.
    ///
    /// # Panics
    /// This currently assumes that there is exactly one auxiliary trace segment, and will panic
    /// otherwise.
    fn read_aux_trace_frame_into(&self, lde_step: usize, frame: &mut EvaluationFrame<E>) {
        // at the end of the trace, next state wraps around and we read the first step again
        let next_lde_step = (lde_step + self.blowup()) % self.trace_len();

        // copy auxiliary trace segment values into the frame
        let segment = &self.aux_segment_ldes[0];
        frame.current_mut().copy_from_slice(segment.row(lde_step));
        frame.next_mut().copy_from_slice(segment.row(next_lde_step));
    }

    /// Returns trace table rows at the specified positions along with Merkle authentication paths
    /// from the commitment root to these rows.
    fn query(&self, positions: &[usize]) -> Vec<Queries> {
        // build queries for the main trace segment
        let mut result = vec![build_segment_queries(
            &self.main_segment_lde,
            &self.main_segment_tree,
            positions,
        )];

        // build queries for auxiliary trace segments
        for (i, segment_tree) in self.aux_segment_trees.iter().enumerate() {
            let segment_lde = &self.aux_segment_ldes[i];
            result.push(build_segment_queries(segment_lde, segment_tree, positions));
        }

        result
    }

    /// Returns the number of rows in the execution trace.
    fn trace_len(&self) -> usize {
        self.main_segment_lde.num_rows()
    }

    /// Returns blowup factor which was used to extend original execution trace into trace LDE.
    fn blowup(&self) -> usize {
        self.blowup
    }

    /// Returns the trace layout of the execution trace.
    fn trace_layout(&self) -> &TraceLayout {
        self.trace_info.layout()
    }
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
fn build_trace_commitment<E: FieldElement<BaseField = Felt>>(
    trace: &ColMatrix<E>,
    domain: &StarkDomain<Felt>,
) -> (RowMatrix<E>, MerkleTree<Rpo256>, ColMatrix<E>) {
    // interpolate the execution trace
    let now = Instant::now();
    let inv_twiddles = fft::get_inv_twiddles::<Felt>(trace.num_rows());
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
            rpo_padded_segment.iter_mut().for_each(|row| row[rpo_pad_column] = ONE);
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
    event!(
            Level::INFO,
            "Extended (on CPU) and committed (on GPU) to an execution trace of {} columns from 2^{} to 2^{} steps in {} ms",
            trace_polys.num_cols(),
            trace_polys.num_rows().ilog2(),
            trace_lde.num_rows().ilog2(),
            now.elapsed().as_millis()
        );

    (trace_lde, trace_tree, trace_polys)
}

// SEGMENT GENERATOR
// ================================================================================================

struct SegmentGenerator<'a, E, I, const N: usize>
where
    E: FieldElement<BaseField = Felt>,
    I: IntoIterator<Item = Vec<E>>,
{
    poly_iter: I::IntoIter,
    polys: Option<ColMatrix<E>>,
    poly_offset: usize,
    offsets: Vec<Felt>,
    domain: &'a StarkDomain<Felt>,
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
    fn gen_next_segment(&mut self) -> Option<Segment<Felt, N>> {
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
            data.fill([Felt::ZERO; N]);
        }

        let twiddles = self.domain.trace_twiddles();
        let segment = Segment::new_with_buffer(data, &*polys, offset, &self.offsets, twiddles);
        self.poly_offset += N;
        Some(segment)
    }
}

fn build_segment_queries<E: FieldElement<BaseField = Felt>>(
    segment_lde: &RowMatrix<E>,
    segment_tree: &MerkleTree<Rpo256>,
    positions: &[usize],
) -> Queries {
    // for each position, get the corresponding row from the trace segment LDE and put all these
    // rows into a single vector
    let trace_states =
        positions.iter().map(|&pos| segment_lde.row(pos).to_vec()).collect::<Vec<_>>();

    // build Merkle authentication paths to the leaves specified by positions
    let trace_proof = segment_tree
        .prove_batch(positions)
        .expect("failed to generate a Merkle proof for trace queries");

    Queries::new(trace_proof, trace_states)
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
    type Item = Segment<Felt, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.gen_next_segment()
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use air::{ProvingOptions, StarkField};
    use processor::{crypto::RpoRandomCoin, StackInputs, StackOutputs};
    use winter_prover::math::fields::CubeExtension;

    type CubeFelt = CubeExtension<Felt>;

    #[test]
    fn build_trace_commitment_on_gpu_with_padding_matches_cpu() {
        let cpu_prover = create_test_prover();
        let gpu_prover = MetalRpoExecutionProver(create_test_prover());
        let num_rows = 1 << 8;
        let trace_info = get_trace_info(1, num_rows);
        let trace = gen_random_trace(num_rows, RPO_RATE + 1);
        let domain = StarkDomain::from_twiddles(fft::get_twiddles(num_rows), 8, Felt::GENERATOR);

        let (cpu_trace_lde, cpu_polys) =
            cpu_prover.new_trace_lde::<CubeFelt>(&trace_info, &trace, &domain);
        let (gpu_trace_lde, gpu_polys) =
            gpu_prover.new_trace_lde::<CubeFelt>(&trace_info, &trace, &domain);

        assert_eq!(
            cpu_trace_lde.get_main_trace_commitment(),
            gpu_trace_lde.get_main_trace_commitment()
        );
        assert_eq!(
            cpu_polys.main_trace_polys().collect::<Vec<_>>(),
            gpu_polys.main_trace_polys().collect::<Vec<_>>()
        );
    }

    #[test]
    fn build_trace_commitment_on_gpu_without_padding_matches_cpu() {
        let cpu_prover = create_test_prover();
        let gpu_prover = MetalRpoExecutionProver(create_test_prover());
        let num_rows = 1 << 8;
        let trace_info = get_trace_info(1, num_rows);
        let trace = gen_random_trace(num_rows, RPO_RATE);
        let domain = StarkDomain::from_twiddles(fft::get_twiddles(num_rows), 8, Felt::GENERATOR);

        let (cpu_trace_lde, cpu_polys) =
            cpu_prover.new_trace_lde::<CubeFelt>(&trace_info, &trace, &domain);
        let (gpu_trace_lde, gpu_polys) =
            gpu_prover.new_trace_lde::<CubeFelt>(&trace_info, &trace, &domain);

        assert_eq!(
            cpu_trace_lde.get_main_trace_commitment(),
            gpu_trace_lde.get_main_trace_commitment()
        );
        assert_eq!(
            cpu_polys.main_trace_polys().collect::<Vec<_>>(),
            gpu_polys.main_trace_polys().collect::<Vec<_>>()
        );
    }

    #[test]
    fn build_constraint_commitment_on_gpu_with_padding_matches_cpu() {
        let cpu_prover = create_test_prover();
        let gpu_prover = MetalRpoExecutionProver(create_test_prover());
        let num_rows = 1 << 8;
        let ce_blowup_factor = 2;
        let values = get_random_values::<CubeFelt>(num_rows * ce_blowup_factor);
        let domain = StarkDomain::from_twiddles(fft::get_twiddles(num_rows), 8, Felt::GENERATOR);

        let (commitment_cpu, composition_poly_cpu) = cpu_prover.build_constraint_commitment(
            CompositionPolyTrace::new(values.clone()),
            2,
            &domain,
        );
        let (commitment_gpu, composition_poly_gpu) =
            gpu_prover.build_constraint_commitment(CompositionPolyTrace::new(values), 2, &domain);

        assert_eq!(commitment_cpu.root(), commitment_gpu.root());
        assert_ne!(0, composition_poly_cpu.data().num_base_cols() % RPO_RATE);
        assert_eq!(composition_poly_cpu.into_columns(), composition_poly_gpu.into_columns());
    }

    #[test]
    fn build_constraint_commitment_on_gpu_without_padding_matches_cpu() {
        let cpu_prover = create_test_prover();
        let gpu_prover = MetalRpoExecutionProver(create_test_prover());
        let num_rows = 1 << 8;
        let ce_blowup_factor = 8;
        let values = get_random_values::<Felt>(num_rows * ce_blowup_factor);
        let domain = StarkDomain::from_twiddles(fft::get_twiddles(num_rows), 8, Felt::GENERATOR);

        let (commitment_cpu, composition_poly_cpu) = cpu_prover.build_constraint_commitment(
            CompositionPolyTrace::new(values.clone()),
            8,
            &domain,
        );
        let (commitment_gpu, composition_poly_gpu) =
            gpu_prover.build_constraint_commitment(CompositionPolyTrace::new(values), 8, &domain);

        assert_eq!(commitment_cpu.root(), commitment_gpu.root());
        assert_eq!(0, composition_poly_cpu.data().num_base_cols() % RPO_RATE);
        assert_eq!(composition_poly_cpu.into_columns(), composition_poly_gpu.into_columns());
    }

    fn gen_random_trace(num_rows: usize, num_cols: usize) -> ColMatrix<Felt> {
        ColMatrix::new((0..num_cols as u64).map(|col| vec![Felt::new(col); num_rows]).collect())
    }

    fn get_random_values<E: FieldElement>(num_rows: usize) -> Vec<E> {
        (0..num_rows).map(|i| E::from(i as u32)).collect()
    }

    fn get_trace_info(num_cols: usize, num_rows: usize) -> TraceInfo {
        TraceInfo::new(num_cols, num_rows)
    }

    fn create_test_prover() -> ExecutionProver<Rpo256, RpoRandomCoin> {
        ExecutionProver::new(
            ProvingOptions::with_128_bit_security(true),
            StackInputs::default(),
            StackOutputs::default(),
        )
    }
}
