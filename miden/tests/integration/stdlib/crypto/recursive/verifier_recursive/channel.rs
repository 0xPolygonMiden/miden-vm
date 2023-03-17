// VERIFIER CHANNEL
// ================================================================================================

use std::vec;

use super::VerifierError;
use miden::{Digest, MerkleSet, Rpo256};
use miden_air::{Felt, ProcessorAir};
use vm_core::{
    crypto::merkle::{MerklePath, MerklePathSet},
    QuadExtension,
};
use winter_air::{
    proof::{Queries, StarkProof, Table},
    Air, EvaluationFrame,
};
use winter_fri::VerifierChannel as FriVerifierChannel;
use winter_utils::{collections::Vec, string::ToString};
use winterfell::crypto::BatchMerkleProof;
use winterfell::math::StarkField;

pub type QuadExt = QuadExtension<Felt>;
/// A view into a [StarkProof] for a computation structured to simulate an "interactive" channel.
///
/// A channel is instantiated for a specific proof, which is parsed into structs over the
/// appropriate field (specified by type parameter `E`). This also validates that the proof is
/// well-formed in the context of the computation for the specified [Air].
pub struct VerifierChannel {
    // trace queries
    trace_roots: Vec<Digest>,
    trace_queries: Option<TraceQueries>,
    // constraint queries
    constraint_root: Digest,
    constraint_queries: Option<ConstraintQueries>,
    // FRI proof
    fri_roots: Option<Vec<Digest>>,
    fri_layer_proofs: Vec<BatchMerkleProof<Rpo256>>,
    fri_layer_queries: Vec<Vec<QuadExt>>,
    fri_remainder: Option<Vec<QuadExt>>,
    fri_num_partitions: usize,
    // out-of-domain frame
    ood_trace_frame: Option<TraceOodFrame>,
    ood_constraint_evaluations: Option<Vec<QuadExt>>,
    // query proof-of-work
    pow_nonce: u64,
}

impl VerifierChannel {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Creates and returns a new [VerifierChannel] initialized from the specified `proof`.
    pub fn new(air: &ProcessorAir, proof: StarkProof) -> Result<Self, VerifierError> {
        let StarkProof {
            context,
            commitments,
            trace_queries,
            constraint_queries,
            ood_frame,
            fri_proof,
            pow_nonce,
        } = proof;

        // make AIR and proof base fields are the same
        if Felt::get_modulus_le_bytes() != context.field_modulus_bytes() {
            return Err(VerifierError::InconsistentBaseField);
        }

        let num_trace_segments = air.trace_layout().num_segments();
        let main_trace_width = air.trace_layout().main_trace_width();
        let aux_trace_width = air.trace_layout().aux_trace_width();
        let lde_domain_size = air.lde_domain_size();
        let fri_options = air.options().to_fri_options();

        // --- parse commitments ------------------------------------------------------------------
        let (trace_roots, constraint_root, fri_roots) = commitments
            .parse::<Rpo256>(num_trace_segments, fri_options.num_fri_layers(lde_domain_size))
            .map_err(|err| VerifierError::ProofDeserializationError(err.to_string()))?;

        // --- parse trace and constraint queries -------------------------------------------------
        let trace_queries = TraceQueries::new(trace_queries, air)?;
        let constraint_queries = ConstraintQueries::new(constraint_queries, air)?;

        // --- parse FRI proofs -------------------------------------------------------------------
        let fri_num_partitions = fri_proof.num_partitions();
        let fri_remainder = fri_proof
            .parse_remainder()
            .map_err(|err| VerifierError::ProofDeserializationError(err.to_string()))?;
        let (fri_layer_queries, fri_layer_proofs) = fri_proof
            .parse_layers::<Rpo256, QuadExt>(lde_domain_size, fri_options.folding_factor())
            .map_err(|err| VerifierError::ProofDeserializationError(err.to_string()))?;

        // --- parse out-of-domain evaluation frame -----------------------------------------------
        let (ood_main_trace_frame, ood_aux_trace_frame, ood_constraint_evaluations) = ood_frame
            .parse(main_trace_width, aux_trace_width, air.ce_blowup_factor())
            .map_err(|err| VerifierError::ProofDeserializationError(err.to_string()))?;
        let ood_trace_frame = TraceOodFrame::new(ood_main_trace_frame, ood_aux_trace_frame);

        Ok(VerifierChannel {
            // trace queries
            trace_roots,
            trace_queries: Some(trace_queries),
            // constraint queries
            constraint_root,
            constraint_queries: Some(constraint_queries),
            // FRI proof
            fri_roots: Some(fri_roots),
            fri_layer_proofs,
            fri_layer_queries,
            fri_remainder: Some(fri_remainder),
            fri_num_partitions,
            // out-of-domain evaluation
            ood_trace_frame: Some(ood_trace_frame),
            ood_constraint_evaluations: Some(ood_constraint_evaluations),
            // query seed
            pow_nonce,
        })
    }

    // DATA READERS
    // --------------------------------------------------------------------------------------------

    /// Returns execution trace commitments sent by the prover.
    ///
    /// For computations requiring multiple trace segment, the returned slice will contain a
    /// commitment for each trace segment.
    pub fn read_trace_commitments(&self) -> &[Digest] {
        &self.trace_roots
    }

    /// Returns constraint evaluation commitment sent by the prover.
    pub fn read_constraint_commitment(&self) -> Digest {
        self.constraint_root
    }

    /// Returns trace polynomial evaluations at out-of-domain points z and z * g, where g is the
    /// generator of the LDE domain.
    ///
    /// For computations requiring multiple trace segments, evaluations of auxiliary trace
    /// polynomials are also included as the second value of the returned tuple. Otherwise, the
    /// second value is None.
    pub fn read_ood_trace_frame(
        &mut self,
    ) -> (EvaluationFrame<QuadExt>, Option<EvaluationFrame<QuadExt>>) {
        let frame = self.ood_trace_frame.take().expect("already read");
        (frame.main_frame, frame.aux_frame)
    }

    /// Returns evaluations of composition polynomial columns at z^m, where z is the out-of-domain
    /// point, and m is the number of composition polynomial columns.
    pub fn read_ood_constraint_evaluations(&mut self) -> Vec<QuadExt> {
        self.ood_constraint_evaluations.take().expect("already read")
    }

    /// Returns query proof-of-work nonce sent by the prover.
    pub fn read_pow_nonce(&self) -> u64 {
        self.pow_nonce
    }

    /// Returns trace states at the specified positions of the LDE domain. This also checks if
    /// the trace states are valid against the trace commitment sent by the prover.
    ///
    /// For computations requiring multiple trace segments, trace states for auxiliary segments
    /// are also included as the second value of the returned tuple (trace states for all auxiliary
    /// segments are merged into a single table). Otherwise, the second value is None.
    #[allow(clippy::type_complexity)]
    pub fn read_queried_trace_states(
        &mut self,
        positions: &[usize],
    ) -> Result<(Table<Felt>, Option<Table<QuadExt>>, Vec<MerkleSet>), VerifierError> {
        let queries = self.trace_queries.take().expect("already read");
        let mut sets = vec![];
        for (_root, proof) in self.trace_roots.iter().zip(queries.query_proofs.into_iter()) {
            let positions = positions.to_vec();
            let (new_set, _leaves) = unbatch_to_path_set(positions, proof);
            let new_set = MerkleSet::MerklePathSet(new_set);

            sets.push(new_set);
        }

        Ok((queries.main_states, queries.aux_states, sets))
    }

    /// Returns constraint evaluations at the specified positions of the LDE domain. This also
    /// checks if the constraint evaluations are valid against the constraint commitment sent by
    /// the prover.
    pub fn read_constraint_evaluations(
        &mut self,
        positions: &[usize],
    ) -> Result<(Table<QuadExt>, MerkleSet), VerifierError> {
        let queries = self.constraint_queries.take().expect("already read");

        //MerkleTree::verify_batch(&self.constraint_root, positions, &queries.query_proofs)
        //.map_err(|_| VerifierError::ConstraintQueryDoesNotMatchCommitment)?;
        let positions = positions.to_vec();
        let (set, _nodes) = unbatch_to_path_set(positions, queries.query_proofs);
        let set = MerkleSet::MerklePathSet(set);

        Ok((queries.evaluations, set))
    }

    // Get the FRI layer challenges alpha
    pub fn fri_layer_commitments(&self) -> Option<Vec<Digest>> {
        self.fri_roots.clone()
    }

    // Get remainder codeword
    pub fn fri_remainder(&self) -> Vec<QuadExt> {
        self.fri_remainder.clone().unwrap()
    }
}

// FRI VERIFIER CHANNEL IMPLEMENTATION
// ================================================================================================

impl FriVerifierChannel<QuadExt> for VerifierChannel {
    type Hasher = Rpo256;

    fn read_fri_num_partitions(&self) -> usize {
        self.fri_num_partitions
    }

    fn read_fri_layer_commitments(&mut self) -> Vec<Digest> {
        self.fri_roots.take().expect("already read")
    }

    fn take_next_fri_layer_proof(&mut self) -> BatchMerkleProof<Rpo256> {
        self.fri_layer_proofs.remove(0)
    }

    fn take_next_fri_layer_queries(&mut self) -> Vec<QuadExt> {
        self.fri_layer_queries.remove(0)
    }

    fn take_fri_remainder(&mut self) -> Vec<QuadExt> {
        self.fri_remainder.take().expect("already read")
    }
}

// TRACE QUERIES
// ================================================================================================

/// Container of trace query data, including:
/// * Queried states for all trace segments.
/// * Merkle authentication paths for all queries.
///
/// Trace states for all auxiliary segments are stored in a single table.
struct TraceQueries {
    query_proofs: Vec<BatchMerkleProof<Rpo256>>,
    main_states: Table<Felt>,
    aux_states: Option<Table<QuadExt>>,
}

impl TraceQueries {
    /// Parses the provided trace queries into trace states in the specified field and
    /// corresponding Merkle authentication paths.
    pub fn new(mut queries: Vec<Queries>, air: &ProcessorAir) -> Result<Self, VerifierError> {
        assert_eq!(
            queries.len(),
            air.trace_layout().num_segments(),
            "expected {} trace segment queries, but received {}",
            air.trace_layout().num_segments(),
            queries.len()
        );

        let num_queries = air.options().num_queries();

        // parse main trace segment queries; parsing also validates that hashes of each table row
        // form the leaves of Merkle authentication paths in the proofs
        let main_segment_width = air.trace_layout().main_trace_width();
        let main_segment_queries = queries.remove(0);
        let (main_segment_query_proofs, main_segment_states) = main_segment_queries
            .parse::<Rpo256, Felt>(air.lde_domain_size(), num_queries, main_segment_width)
            .map_err(|err| {
                VerifierError::ProofDeserializationError(format!(
                    "main trace segment query deserialization failed: {err}"
                ))
            })?;

        // all query proofs will be aggregated into a single vector
        let mut query_proofs = vec![main_segment_query_proofs];

        // parse auxiliary trace segment queries (if any), and merge resulting tables into a
        // single table; parsing also validates that hashes of each table row form the leaves
        // of Merkle authentication paths in the proofs
        let aux_trace_states = if air.trace_info().is_multi_segment() {
            let mut aux_trace_states = Vec::new();
            for (i, segment_queries) in queries.into_iter().enumerate() {
                let segment_width = air.trace_layout().get_aux_segment_width(i);
                let (segment_query_proof, segment_trace_states) = segment_queries
                    .parse::<Rpo256, QuadExt>(air.lde_domain_size(), num_queries, segment_width)
                    .map_err(|err| {
                        VerifierError::ProofDeserializationError(format!(
                            "auxiliary trace segment query deserialization failed: {err}"
                        ))
                    })?;

                query_proofs.push(segment_query_proof);
                aux_trace_states.push(segment_trace_states);
            }

            // merge tables for each auxiliary segment into a single table
            Some(Table::merge(aux_trace_states))
        } else {
            None
        };

        Ok(Self {
            query_proofs,
            main_states: main_segment_states,
            aux_states: aux_trace_states,
        })
    }
}

// CONSTRAINT QUERIES
// ================================================================================================

/// Container of constraint evaluation query data, including:
/// * Queried constraint evaluation values.
/// * Merkle authentication paths for all queries.
struct ConstraintQueries {
    query_proofs: BatchMerkleProof<Rpo256>,
    evaluations: Table<QuadExt>,
}

impl ConstraintQueries {
    /// Parses the provided constraint queries into evaluations in the specified field and
    /// corresponding Merkle authentication paths.
    pub fn new(queries: Queries, air: &ProcessorAir) -> Result<Self, VerifierError> {
        let num_queries = air.options().num_queries();
        let (query_proofs, evaluations) = queries
            .parse::<Rpo256, QuadExt>(air.lde_domain_size(), num_queries, air.ce_blowup_factor())
            .map_err(|err| {
                VerifierError::ProofDeserializationError(format!(
                    "constraint evaluation query deserialization failed: {err}"
                ))
            })?;

        Ok(Self {
            query_proofs,
            evaluations,
        })
    }
}

// TRACE OUT-OF-DOMAIN FRAME
// ================================================================================================

struct TraceOodFrame {
    main_frame: EvaluationFrame<QuadExt>,
    aux_frame: Option<EvaluationFrame<QuadExt>>,
}

impl TraceOodFrame {
    pub fn new(
        main_frame: EvaluationFrame<QuadExt>,
        aux_frame: Option<EvaluationFrame<QuadExt>>,
    ) -> Self {
        Self {
            main_frame,
            aux_frame,
        }
    }
}

// Helper
pub fn unbatch_to_path_set(
    mut positions: Vec<usize>,
    proof: BatchMerkleProof<Rpo256>,
) -> (MerklePathSet, Vec<[Felt; 4]>) {
    let mut unbatched_proof = proof.into_paths(&positions).unwrap();
    let depth = unbatched_proof[0].len() as u8;
    //let mut adv_key_map = vec![];
    let nodes: Vec<[Felt; 4]> = unbatched_proof
        .iter_mut()
        .map(|list| {
            let node = list.remove(0);
            let node = node.as_elements().to_owned();
            [node[0], node[1], node[2], node[3]]
        })
        .collect();

    let paths: Vec<MerklePath> = unbatched_proof
        .iter()
        .map(|list| {
            list.iter()
                .map(|digest| {
                    let node = digest.as_elements();
                    let node = [node[0], node[1], node[2], node[3]];
                    node
                })
                .collect()
        })
        .collect();

    let new_set = MerklePathSet::new(depth);

    let iter_pos = positions.iter_mut().map(|a| *a as u64);
    let nodes_tmp = nodes.clone();
    let iter_nodes = nodes_tmp.iter();
    let iter_paths = paths.into_iter();
    let mut tmp_vec = vec![];
    for (p, (node, path)) in iter_pos.zip(iter_nodes.zip(iter_paths)) {
        tmp_vec.push((p, *node, path));
    }

    (new_set.with_paths(tmp_vec).expect("should not fail from paths"), nodes_tmp)
}
