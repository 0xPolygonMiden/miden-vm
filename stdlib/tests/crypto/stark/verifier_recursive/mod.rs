use miden_air::ProcessorAir;
use processor::crypto::RpoRandomCoin;
use test_utils::{
    collections::Vec,
    crypto::{MerkleStore, RandomCoin, Rpo256, RpoDigest},
    math::{fft, FieldElement, QuadExtension, StarkField, ToElements},
    Felt, VerifierError,
};
use winter_air::{proof::StarkProof, Air, AuxTraceRandElements};

mod channel;
use channel::VerifierChannel;

pub const BLOWUP_FACTOR: usize = 8;
pub type QuadExt = QuadExtension<Felt>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VerifierData {
    pub initial_stack: Vec<u64>,
    pub tape: Vec<u64>,
    pub store: MerkleStore,
    pub advice_map: Vec<([u8; 32], Vec<Felt>)>,
}

pub fn generate_advice_inputs(
    proof: StarkProof,
    pub_inputs: <ProcessorAir as Air>::PublicInputs,
) -> Result<VerifierData, VerifierError> {
    //// build a seed for the public coin; the initial seed is the hash of public inputs and proof
    //// context, but as the protocol progresses, the coin will be reseeded with the info received
    //// from the prover
    let mut public_coin_seed = proof.context.to_elements();
    let trace_len: Felt = public_coin_seed[7];
    let initial_stack = vec![
        public_coin_seed[4].as_int(),
        (public_coin_seed[5].as_int() as usize).ilog2() as u64,
        public_coin_seed[6].as_int(),
        (trace_len.as_int() as usize).ilog2() as u64,
    ];

    let mut tape = vec![];
    public_coin_seed.append(&mut pub_inputs.to_elements());

    let pub_inputs_int: Vec<u64> = pub_inputs.to_elements().iter().map(|a| a.as_int()).collect();
    tape.extend_from_slice(&pub_inputs_int[..]);

    // create AIR instance for the computation specified in the proof
    let air = ProcessorAir::new(proof.get_trace_info(), pub_inputs, proof.options().clone());

    let mut public_coin: RpoRandomCoin = RpoRandomCoin::new(&public_coin_seed);
    let mut channel = VerifierChannel::new(&air, proof)?;

    // 1 ----- trace commitment -------------------------------------------------------------------
    let trace_commitments = channel.read_trace_commitments();

    // reseed the coin with the commitment to the main trace segment
    public_coin.reseed(trace_commitments[0]);
    tape.extend_from_slice(&digest_to_int_vec(&trace_commitments));

    // process auxiliary trace segments, to build a set of random elements for each segment
    let mut aux_trace_rand_elements = AuxTraceRandElements::<QuadExt>::new();
    for (i, commitment) in trace_commitments.iter().skip(1).enumerate() {
        let rand_elements = air
            .get_aux_trace_segment_random_elements(i, &mut public_coin)
            .map_err(|_| VerifierError::RandomCoinError)?;
        aux_trace_rand_elements.add_segment_elements(rand_elements);
        public_coin.reseed(*commitment);
    }
    // build random coefficients for the composition polynomial
    let _constraint_coeffs: winter_air::ConstraintCompositionCoefficients<QuadExt> = air
        .get_constraint_composition_coefficients(&mut public_coin)
        .map_err(|_| VerifierError::RandomCoinError)?;

    // 2 ----- constraint commitment --------------------------------------------------------------
    let constraint_commitment = channel.read_constraint_commitment();
    tape.extend_from_slice(&digest_to_int_vec(&[constraint_commitment]));
    public_coin.reseed(constraint_commitment);

    // 3 ----- OOD frames --------------------------------------------------------------
    let ood_trace_frame = channel.read_ood_trace_frame();
    let _ood_main_trace_frame = ood_trace_frame.main_frame();
    let _ood_aux_trace_frame = ood_trace_frame.aux_frame();

    tape.extend_from_slice(&to_int_vec(&ood_trace_frame.values()));
    public_coin.reseed(Rpo256::hash_elements(ood_trace_frame.values()));

    // read evaluations of composition polynomial columns
    let ood_constraint_evaluations = channel.read_ood_constraint_evaluations();
    tape.extend_from_slice(&to_int_vec(&ood_constraint_evaluations));
    public_coin.reseed(Rpo256::hash_elements(&ood_constraint_evaluations));

    // 4 ----- FRI  --------------------------------------------------------------------
    let fri_commitments_digests = channel.fri_layer_commitments().unwrap();
    let poly = channel.fri_remainder();
    let twiddles = fft::get_twiddles(poly.len());
    let fri_remainder =
        fft::evaluate_poly_with_offset(&poly, &twiddles, Felt::GENERATOR, BLOWUP_FACTOR);

    let fri_commitments: Vec<u64> = digest_to_int_vec(&fri_commitments_digests);
    tape.extend_from_slice(&fri_commitments);

    tape.extend_from_slice(&to_int_vec(&poly));
    tape.extend_from_slice(&to_int_vec(&fri_remainder));

    let _deep_coefficients = air
        .get_deep_composition_coefficients::<QuadExt, RpoRandomCoin>(&mut public_coin)
        .map_err(|_| VerifierError::RandomCoinError)?;
    // Reseed with FRI layer commitments
    let layer_commitments = fri_commitments_digests.clone();
    for commitment in layer_commitments.iter() {
        public_coin.reseed(*commitment);
        let _alpha: QuadExt = public_coin.draw().expect("failed to draw random indices");
    }

    // 5 ----- trace and constraint queries -------------------------------------------------------
    // read proof-of-work nonce sent by the prover and update the public coin with it
    let pow_nonce = channel.read_pow_nonce();
    tape.extend_from_slice(&[pow_nonce]);
    public_coin.reseed_with_int(pow_nonce);

    // make sure the proof-of-work specified by the grinding factor is satisfied
    if public_coin.leading_zeros() < air.options().grinding_factor() {
        return Err(VerifierError::QuerySeedProofOfWorkVerificationFailed);
    }

    // draw pseudo-random query positions for the LDE domain from the public coin.
    // this is needed in order to construct Merkle path sets
    let query_positions = public_coin
        .draw_integers(air.options().num_queries(), air.lde_domain_size())
        .map_err(|_| VerifierError::RandomCoinError)?;

    // read advice maps and Merkle paths related to trace and constraint composition polynomial evaluations
    let (mut advice_map, mut m_path_sets_traces) =
        channel.read_queried_trace_states(&query_positions)?;
    let (mut adv_map_constraint, m_path_set_constraint) =
        channel.read_constraint_evaluations(&query_positions)?;

    let domain_size = (air.trace_poly_degree() + 1) * BLOWUP_FACTOR;
    let mut ress = channel.unbatch::<4, 3>(&query_positions, domain_size, fri_commitments_digests);
    // consolidate advice maps
    advice_map.append(&mut adv_map_constraint);
    advice_map.append(&mut ress.1);
    let mut m_path_sets_fri = ress.0;
    m_path_sets_fri.append(&mut m_path_sets_traces);
    m_path_sets_fri.push(m_path_set_constraint);
    let mut store = MerkleStore::new();
    for path_set in &m_path_sets_fri {
        store.add_merkle_path_set(&path_set).unwrap();
    }
    Ok(VerifierData {
        initial_stack,
        tape,
        store,
        advice_map,
    })
}

// Helpers
pub fn digest_to_int_vec(digest: &[RpoDigest]) -> Vec<u64> {
    digest
        .iter()
        .map(|digest| digest.as_elements().into_iter().map(|e| e.as_int()))
        .flatten()
        .collect()
}

pub fn to_int_vec(ext_felts: &[QuadExt]) -> Vec<u64> {
    QuadExt::slice_as_base_elements(ext_felts)
        .into_iter()
        .map(|e| e.as_int())
        .collect()
}
