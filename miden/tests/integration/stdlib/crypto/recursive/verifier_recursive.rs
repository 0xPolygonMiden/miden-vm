use miden::{math::fft, utils::RandomCoin, Digest, MerkleSet, Rpo256};
use miden_air::{Felt, ProcessorAir, StarkField};
use vm_core::{ExtensionOf, FieldElement, QuadExtension};
use winter_air::{proof::StarkProof, Air, AuxTraceRandElements};
use winter_fri::FriVerifier;
use winter_utils::collections::Vec;
pub use winter_utils::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, SliceReader,
};

mod channel;
use channel::VerifierChannel;

mod evaluator;
use evaluator::evaluate_constraints;

mod composer;
use composer::DeepComposer;

mod errors;
pub use errors::VerifierError;
use winterfell::math::log2;

pub type QuadExt = QuadExtension<Felt>;

// VERIFIER
// ================================================================================================
/// Verifies that the specified computation was executed correctly against the specified inputs.
///
/// Specifically, for a computation specified by `AIR` and `HashFn` type parameter, verifies that the provided
/// `proof` attests to the correct execution of the computation against public inputs specified
/// by `pub_inputs`. If the verification is successful, `Ok(())` is returned.
///
/// # Errors
/// Returns an error if combination of the provided proof and public inputs does not attest to
/// a correct execution of the computation. This could happen for many various reasons, including:
/// - The specified proof was generated for a different computation.
/// - The specified proof was generated for this computation but for different public inputs.
#[rustfmt::skip]
pub fn verify(
    proof: StarkProof,
    pub_inputs: <ProcessorAir as Air>::PublicInputs,
) -> Result<(Vec<u64>, Vec<MerkleSet>), VerifierError> {
    // build a seed for the public coin; the initial seed is the hash of public inputs and proof
    // context, but as the protocol progresses, the coin will be reseeded with the info received
    // from the prover
    let mut public_coin_seed = Vec::new();
    pub_inputs.write_into(&mut public_coin_seed);
    //println!("public_coin_seed after pub_inputs {:?}", public_coin_seed);
    proof.context.write_into(&mut public_coin_seed);
    //println!("proof.context {:?}", proof.context);
    //println!("public_coin_seed after context {:?}", public_coin_seed);

    // create AIR instance for the computation specified in the proof
    let air = ProcessorAir::new(proof.get_trace_info(), pub_inputs, proof.options().clone());

    let mut public_coin = RandomCoin::new(&public_coin_seed);
    // This will put public_coin.seed = Rpo::hash(public_coin_seed)
    let expected_seed = Rpo256::hash(&public_coin_seed);
    println!("expected_seed {:?}", expected_seed);
    let mut channel = VerifierChannel::new(&air, proof)?;

    let mut tape = vec![]; 
    // 1 ----- trace commitment -------------------------------------------------------------------
    // Read the commitments to evaluations of the trace polynomials over the LDE domain sent by the
    // prover. The commitments are used to update the public coin, and draw sets of random elements
    // from the coin (in the interactive version of the protocol the verifier sends these random
    // elements to the prover after each commitment is made). When there are multiple trace
    // commitments (i.e., the trace consists of more than one segment), each previous commitment is
    // used to draw random elements needed to construct the next trace segment. The last trace
    // commitment is used to draw a set of random coefficients which the prover uses to compute
    // constraint composition polynomial.
    let trace_commitments = channel.read_trace_commitments();

    // reseed the coin with the commitment to the main trace segment
    public_coin.reseed(trace_commitments[0]);
    tape.extend_from_slice(&[0, 0, log2(air.lde_domain_size().try_into().unwrap()).try_into().unwrap(), air.lde_domain_size().try_into().unwrap()]);
    tape.extend_from_slice(&digest_to_int_vec(&trace_commitments));

    println!("trace_commitments {:?}", trace_commitments);

    // process auxiliary trace segments (if any), to build a set of random elements for each segment
    let mut aux_trace_rand_elements = AuxTraceRandElements::<QuadExt>::new();
    for (i, commitment) in trace_commitments.iter().skip(1).enumerate() {
        let rand_elements = air
            .get_aux_trace_segment_random_elements(i, &mut public_coin)
            .map_err(|_| VerifierError::RandomCoinError)?;
    println!("number of rand elem {:?}", rand_elements.len());
    println!("actual rand elem {:?}", rand_elements);
        aux_trace_rand_elements.add_segment_elements(rand_elements);
        public_coin.reseed(*commitment);
    }

    // build random coefficients for the composition polynomial
    let constraint_coeffs = air
        .get_constraint_composition_coefficients(&mut public_coin)
        .map_err(|_| VerifierError::RandomCoinError)?;

    // 2 ----- constraint commitment --------------------------------------------------------------
    // read the commitment to evaluations of the constraint composition polynomial over the LDE
    // domain sent by the prover, use it to update the public coin, and draw an out-of-domain point
    // z from the coin; in the interactive version of the protocol, the verifier sends this point z
    // to the prover, and the prover evaluates trace and constraint composition polynomials at z,
    // and sends the results back to the verifier.
    let constraint_commitment = channel.read_constraint_commitment();
    tape.extend_from_slice(&digest_to_int_vec(&[constraint_commitment]));
    public_coin.reseed(constraint_commitment);
    let z = public_coin.draw::<QuadExt>().map_err(|_| VerifierError::RandomCoinError)?;
    println!("z ood is {:?}",z);
    println!("gz lde ood is {:?}", z.mul_base(air.lde_domain_generator()));
    println!("gz trace ood is {:?}", z.mul_base(air.trace_domain_generator()));
    println!("trace gen {:?}", air.trace_domain_generator());
    println!("lde gen {:?}", air.lde_domain_generator());
    println!("lde^8 gen {:?}", air.lde_domain_generator().exp(8));

    // 3 ----- OOD consistency check --------------------------------------------------------------
    // make sure that evaluations obtained by evaluating constraints over the out-of-domain frame
    // are consistent with the evaluations of composition polynomial columns sent by the prover

    // read the out-of-domain trace frames (the main trace frame and auxiliary trace frame, if
    // provided) sent by the prover and evaluate constraints over them; also, reseed the public
    // coin with the OOD frames received from the prover.
    let (ood_main_trace_frame, ood_aux_trace_frame) = channel.read_ood_trace_frame();
    //println!("ood_main_trace_frame {:?}", ood_main_trace_frame);
    //println!("ood_aux_trace_frame {:?}", ood_aux_trace_frame);
    let ood_constraint_evaluation_1 = evaluate_constraints(
        &air,
        constraint_coeffs,
        &ood_main_trace_frame,
        &ood_aux_trace_frame,
        aux_trace_rand_elements,
        z,
    );

    if let Some(ref aux_trace_frame) = ood_aux_trace_frame {
        // when the trace contains auxiliary segments, append auxiliary trace elements at the
        // end of main trace elements for both current and next rows in the frame. this is
        // needed to be consistent with how the prover writes OOD frame into the channel.

        let mut current = ood_main_trace_frame.current().to_vec();
        println!("hash of current only main is {:?}", Rpo256::hash_elements(&current));
        current.extend_from_slice(aux_trace_frame.current());
        let current_ = to_int_vec(&current);
        println!("length of ood+aux trace frame {:?}", current_.len());
        tape.extend_from_slice(&current_);
        println!("hash of current main + aux is {:?}", Rpo256::hash_elements(&current));
        public_coin.reseed(Rpo256::hash_elements(&current));

        let mut next = ood_main_trace_frame.next().to_vec();
        next.extend_from_slice(aux_trace_frame.next());
        tape.extend_from_slice(&to_int_vec(&next));
                println!("hash of next main + aux is {:?}", Rpo256::hash_elements(&next));

        public_coin.reseed(Rpo256::hash_elements(&next));
    } else {
        unreachable!()
    }

    // read evaluations of composition polynomial columns sent by the prover, and reduce them into
    // a single value by computing sum(z^i * value_i), where value_i is the evaluation of the ith
    // column polynomial at z^m, where m is the total number of column polynomials; also, reseed
    // the public coin with the OOD constraint evaluations received from the prover.
    let ood_constraint_evaluations = channel.read_ood_constraint_evaluations();
    println!("ood_const_eval {:?}", ood_constraint_evaluations);
    tape.extend_from_slice(&to_int_vec(&ood_constraint_evaluations));
    let ood_constraint_evaluation_2 = ood_constraint_evaluations
        .iter()
        .enumerate()
        .fold(QuadExt::ZERO, |result, (i, &value)| result + z.exp_vartime((i as u32).into()) * value);
    public_coin.reseed(Rpo256::hash_elements(&ood_constraint_evaluations));

    // finally, make sure the values are the same
    if ood_constraint_evaluation_1 != ood_constraint_evaluation_2 {
        return Err(VerifierError::InconsistentOodConstraintEvaluations);
    }
    println!("ood_constraint_evaluation_2 is {:?}", ood_constraint_evaluation_2);

    // 4 ----- FRI commitments --------------------------------------------------------------------
    // draw coefficients for computing DEEP composition polynomial from the public coin; in the
    // interactive version of the protocol, the verifier sends these coefficients to the prover
    // and the prover uses them to compute the DEEP composition polynomial. the prover, then
    // applies FRI protocol to the evaluations of the DEEP composition polynomial.
    let deep_coefficients = air
        .get_deep_composition_coefficients::<QuadExt, Rpo256>(&mut public_coin)
        .map_err(|_| VerifierError::RandomCoinError)?;

    // instantiates a FRI verifier with the FRI layer commitments read from the channel. From the
    // verifier's perspective, this is equivalent to executing the commit phase of the FRI protocol.
    // The verifier uses these commitments to update the public coin and draw random points alpha
    // from them; in the interactive version of the protocol, the verifier sends these alphas to
    // the prover, and the prover uses them to compute and commit to the subsequent FRI layers.
    let fri_commitments = channel.fri_layer_commitments().unwrap();
    let mut fri_remainder = channel.fri_remainder();
    println!("first fri commitment {:?}", fri_commitments);
    let fri_commitments: Vec<u64> = digest_to_int_vec(&fri_commitments);
    tape.extend_from_slice(&fri_commitments);
    tape.extend_from_slice(&to_int_vec(&fri_remainder));
    let remainder_poly = {
        let inv_twiddles = fft::get_inv_twiddles(fri_remainder.len());
        fft::interpolate_poly_with_offset(&mut fri_remainder, &inv_twiddles, Felt::GENERATOR);
        let remainder = fri_remainder[..fri_remainder.len() / 8].to_vec();
        remainder

    };
    println!("remainder_poly {:?}", remainder_poly);
    tape.extend_from_slice(&to_int_vec(&remainder_poly));
    let _fri_verifier = FriVerifier::new(
        &mut channel,
        &mut public_coin,
        air.options().to_fri_options(),
        air.trace_poly_degree(),
    )
    .map_err(VerifierError::FriVerificationFailed)?;



    //let fri_verifier = FriVerifierFold4Ext2::new(
        //&mut channel,
        //&mut public_coin,
        //air.options().to_fri_options(),
        //air.trace_poly_degree(),
    //)
    //.map_err(VerifierError::FriVerificationFailed)?;
    // TODO: make sure air.lde_domain_size() == fri_verifier.domain_size()
    let domain_size = (air.trace_poly_degree() + 1) * 8;
    let fri_remainder_size = air.options().to_fri_options().fri_remainder_size(domain_size);
    let num_fri_layer = air.options().to_fri_options().num_fri_layers(domain_size);
    println!("domain_size is {:?}, num_fri_layers {:?}, fri_remainder_size {:?}", domain_size, num_fri_layer, fri_remainder_size,);
    // 5 ----- trace and constraint queries -------------------------------------------------------
    // read proof-of-work nonce sent by the prover and update the public coin with it
    let pow_nonce = channel.read_pow_nonce();
    tape.extend_from_slice(&[pow_nonce]);
    public_coin.reseed_with_int(pow_nonce);
    println!("pow is {:?}", pow_nonce);
    // make sure the proof-of-work specified by the grinding factor is satisfied
    if public_coin.leading_zeros() < air.options().grinding_factor() {
        return Err(VerifierError::QuerySeedProofOfWorkVerificationFailed);
    }

    // draw pseudo-random query positions for the LDE domain from the public coin; in the
    // interactive version of the protocol, the verifier sends these query positions to the prover,
    // and the prover responds with decommitments against these positions for trace and constraint
    // composition polynomial evaluations.
    let query_positions = public_coin
        .draw_integers(air.options().num_queries(), air.lde_domain_size())
        .map_err(|_| VerifierError::RandomCoinError)?;
    println!("query positions {:?}", query_positions);
    // read evaluations of trace and constraint composition polynomials at the queried positions;
    // this also checks that the read values are valid against trace and constraint commitments
    let (queried_main_trace_states, queried_aux_trace_states, m_path_sets_traces) =
        channel.read_queried_trace_states(&query_positions)?;
    let (queried_constraint_evaluations, m_path_set_constraint) = channel.read_constraint_evaluations(&query_positions)?;

    //let pos: Vec<u64> = query_positions.iter().map(|e| *e as u64).collect();
    let queried : Vec<u64> = queried_main_trace_states.rows().flatten().map(|e| e.as_int()).collect();
    let queried_aux : Vec<u64> = queried_aux_trace_states.as_ref().unwrap().rows().flat_map(|e| QuadExt::as_base_elements(e).iter().map(|c| c.as_int())).collect();
    let queried_const : Vec<u64> = queried_constraint_evaluations.rows().flat_map(|e| QuadExt::as_base_elements(e).iter().map(|c| c.as_int())).collect();

    println!("length of queried {:?}", queried.len());
    println!("length of queried_aux {:?}", queried_aux.len());
    println!("length of queried_const {:?}", queried_const.len());
    println!("queried_aux {:?}", &queried_aux[..18]);
    println!("queried const{:?}", &queried_const[..16]);
    //tape.extend_from_slice(&pos);
    //tape.extend_from_slice(&queried[..72]);
    //tape.extend_from_slice(&queried_aux[..18]);
    //tape.extend_from_slice(&queried_const[..16]);

    tape.extend_from_slice(&queried);
    tape.extend_from_slice(&queried_aux);
    tape.extend_from_slice(&queried_const);

    // 6 ----- DEEP composition -------------------------------------------------------------------
    // compute evaluations of the DEEP composition polynomial at the queried positions
    let composer = DeepComposer::new(&air, &query_positions, z, deep_coefficients);
    let t_composition = composer.compose_trace_columns(
        queried_main_trace_states,
        queried_aux_trace_states,
        ood_main_trace_frame,
        ood_aux_trace_frame,
    );
    let c_composition = composer
        .compose_constraint_evaluations(queried_constraint_evaluations, ood_constraint_evaluations);
    let _deep_evaluations = composer.combine_compositions(t_composition, c_composition);

    //// 7 ----- Verify low-degree proof -------------------------------------------------------------
    //// make sure that evaluations of the DEEP composition polynomial we computed in the previous
    //// step are in fact evaluations of a polynomial of degree equal to trace polynomial degree
    //fri_verifier
        //.verify(&mut channel, &deep_evaluations, &query_positions)
        //.map_err(VerifierError::FriVerificationFailed)
    Ok((tape, vec![m_path_sets_traces[0].clone(), m_path_sets_traces[1].clone(), m_path_set_constraint]))
}

// Helper
pub fn digest_to_int_vec(digest: &[Digest]) -> Vec<u64> {
    digest
        .iter()
        .map(|digest| digest.as_elements().into_iter().map(|e| e.as_int()))
        .flatten()
        .collect()
}

pub fn to_int_vec(ext_felts: &[QuadExt]) -> Vec<u64> {
    QuadExt::as_base_elements(ext_felts).into_iter().map(|e| e.as_int()).collect()
}
