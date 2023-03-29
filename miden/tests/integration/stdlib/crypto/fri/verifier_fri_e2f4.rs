use core::{marker::PhantomData, mem};
use miden::{math::fft, utils::math::log2, Digest as MidenDigest};
use processor::crypto::{Hasher, RandomCoin, WinterRandomCoin};
use vm_core::{
    chiplets::hasher::Hasher as MidenHasher,
    crypto::merkle::{MerklePath, MerklePathSet, NodeIndex},
    utils::IntoBytes,
    Felt, FieldElement, QuadExtension, StarkField, ZERO,
};
use winter_fri::{
    folding::fold_positions, DefaultProverChannel, FriOptions, FriProof, FriProver, VerifierError,
};
use winter_utils::group_vector_elements;

use super::channel::{MidenFriVerifierChannel, UnBatch};

type QuadExt = QuadExtension<Felt>;

// This function proves and then verifies a FRI proof with the following fixed parameters:
//  1) Max remainder codeword (1 << 6).
//  2) Blow up factor 8.
//  3) Folding factor 4.
//
//  The main purpose of this function is to build the non-deterministic inputs needed to verify
//  a FRI proof inside the Miden VM.
//  The output is organized as follows:
//  1) `(merkle_sets, advice_maps): (Vec<MerkleTree>, Vec<([u8; 32], Vec<Felt>)>)` where
//  merkle_sets contains the Merkle authentication paths used to authenticate the queries.
//  advice_maps is used to unhash Merkle nodes to a sequence of field elements representing
//  the query-values. TODO: Make use of the advice_maps.
//  2) `positions: Vec<u64>` a vector of consecutive quadruples of the form (poe, p, e1, e0)
//  where p is index of the query at the first layer and (e1, e0) is its corresponding
//  evaluation and poe is g^p with g being the initial domain generator.
//  3) `alphas: Vec<u64>` is a vector of tuples representing the folding challenges.
//  4) `commitments: Vec<u64>` is a vector of consecutive quadruples (c3, c2, c1, c0)
//  representing the Merkle tree layer commitments.
//  5)  `remainder: Vec<u64>` is the remainder codeword as consecutive (r0, r1).
//  6)  `num_queries: usize` is the number of queries contained in the current FRI proof.
pub fn fri_prove_verify_fold4_ext2(
    trace_length_e: usize,
) -> Result<
    (
        (Vec<MerklePathSet>, Vec<([u8; 32], Vec<Felt>)>),
        Vec<u64>,
        Vec<u64>,
        Vec<u64>,
        Vec<u64>,
        usize,
    ),
    VerifierError,
> {
    let max_remainder_size_e = 3;
    let folding_factor_e = 2;
    let trace_length = 1 << trace_length_e;
    let lde_blowup = 1 << 3;
    let max_remainder_size = 1 << max_remainder_size_e;
    let folding_factor = 1 << folding_factor_e;

    let options = FriOptions::new(lde_blowup, folding_factor, max_remainder_size);
    let mut channel = build_prover_channel(trace_length, &options);
    let evaluations = build_evaluations(trace_length, lde_blowup);

    // instantiate the prover and generate the proof
    let mut prover = FriProver::new(options.clone());
    prover.build_layers(&mut channel, evaluations.clone());
    let positions = channel.draw_query_positions();
    let proof = prover.build_proof(&positions);

    // make sure the proof can be verified
    let commitments = channel.layer_commitments().to_vec();

    let max_degree = trace_length - 1;
    let result = verify_proof(
        proof.clone(),
        commitments.clone(),
        &evaluations,
        max_degree,
        trace_length * lde_blowup,
        &positions,
        &options,
    );
    let commitments: Vec<u64> = channel
        .layer_commitments()
        .to_vec()
        .iter()
        .map(|digest| digest.as_elements().into_iter().map(|e| e.as_int()))
        .flatten()
        .collect();

    let remainder_poly: Vec<QuadExt> =
        proof.parse_remainder().expect("should return remainder polynomial");
    let twiddles = fft::get_twiddles(remainder_poly.len());
    let remainder = fft::evaluate_poly_with_offset(&remainder_poly, &twiddles, Felt::GENERATOR, 8);

    let remainder: Vec<u64> = QuadExt::slice_as_base_elements(&remainder[..])
        .to_owned()
        .iter()
        .map(|a| a.as_int())
        .collect();

    match result {
        Ok(((merkle_path_set, advice_values), all_position_evaluation, all_alphas)) => {
            return Ok((
                (merkle_path_set, advice_values),
                all_position_evaluation,
                all_alphas,
                commitments,
                remainder,
                positions.len(),
            ));
        }
        Err(err) => return Err(err),
    }
}

// HELPER UTILS
// ================================================================================================

pub fn build_prover_channel(
    trace_length: usize,
    options: &FriOptions,
) -> DefaultProverChannel<QuadExt, MidenHasher, WinterRandomCoin<MidenHasher>> {
    DefaultProverChannel::new(trace_length * options.blowup_factor(), 32)
}

pub fn build_evaluations(trace_length: usize, lde_blowup: usize) -> Vec<QuadExt> {
    let mut p = (0..trace_length as u64)
        .map(|i| (i, i))
        .map(|(i, j)| QuadExt::new(i.into(), j.into()))
        .collect::<Vec<_>>();
    let domain_size = trace_length * lde_blowup;
    p.resize(domain_size, QuadExt::ZERO);

    let twiddles = fft::get_twiddles::<Felt>(domain_size);

    fft::evaluate_poly(&mut p, &twiddles);
    p
}

fn verify_proof(
    proof: FriProof,
    commitments: Vec<<MidenHasher as Hasher>::Digest>,
    evaluations: &[QuadExt],
    max_degree: usize,
    domain_size: usize,
    positions: &[usize],
    options: &FriOptions,
) -> Result<((Vec<MerklePathSet>, Vec<([u8; 32], Vec<Felt>)>), Vec<u64>, Vec<u64>), VerifierError> {
    let mut channel = MidenFriVerifierChannel::<QuadExt, MidenHasher>::new(
        proof,
        commitments.clone(),
        domain_size,
        options.folding_factor(),
    )
    .unwrap();
    let mut coin = WinterRandomCoin::new(&[]);

    let miden_verifier =
        FriVerifierFold4Ext2::new(&mut channel, &mut coin, options.clone(), max_degree)?;

    let queried_evaluations = positions.iter().map(|&p| evaluations[p]).collect::<Vec<_>>();

    let result =
        miden_verifier.verify_fold_4_ext_2(&mut channel, &queried_evaluations, &positions)?;

    Ok(result)
}

/// Partial implementation for verification in the case of folding factor 4

pub struct FriVerifierFold4Ext2 {
    domain_size: usize,
    domain_generator: Felt,
    layer_commitments: Vec<MidenDigest>,
    layer_alphas: Vec<QuadExt>,
    options: FriOptions,
    _channel: PhantomData<MidenFriVerifierChannel<QuadExt, MidenHasher>>,
}

impl FriVerifierFold4Ext2 {
    pub fn new(
        channel: &mut MidenFriVerifierChannel<QuadExt, MidenHasher>,
        public_coin: &mut WinterRandomCoin<MidenHasher>,
        options: FriOptions,
        max_poly_degree: usize,
    ) -> Result<Self, VerifierError> {
        assert_eq!(options.blowup_factor(), 8);
        assert_eq!(options.folding_factor(), 4);

        // infer evaluation domain info
        let domain_size = max_poly_degree.next_power_of_two() * options.blowup_factor();
        let domain_generator = Felt::get_root_of_unity(log2(domain_size));

        // read layer commitments from the channel and use them to build a list of alphas
        let layer_commitments = channel.read_fri_layer_commitments();
        let mut layer_alphas = Vec::with_capacity(layer_commitments.len());
        let mut max_degree_plus_1 = max_poly_degree + 1;
        for (depth, commitment) in layer_commitments.iter().enumerate() {
            public_coin.reseed(*commitment);
            let alpha = public_coin.draw().map_err(VerifierError::RandomCoinError)?;
            layer_alphas.push(alpha);

            // make sure the degree can be reduced by the folding factor at all layers
            // but the remainder layer
            if depth != layer_commitments.len() - 1
                && max_degree_plus_1 % options.folding_factor() != 0
            {
                return Err(VerifierError::DegreeTruncation(
                    max_degree_plus_1 - 1,
                    options.folding_factor(),
                    depth,
                ));
            }
            max_degree_plus_1 /= options.folding_factor();
        }

        Ok(FriVerifierFold4Ext2 {
            domain_size,
            domain_generator,
            layer_commitments,
            layer_alphas,
            options,
            _channel: PhantomData,
        })
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns size of the domain over which a polynomial commitment checked by this verifier
    /// has been evaluated.
    ///
    /// The domain size can be computed by rounding `max_poly_degree` to the next power of two
    /// and multiplying the result by the `blowup_factor` from the protocol options.
    pub fn domain_size(&self) -> usize {
        self.domain_size
    }

    /// Verifier in the setting of (folding_factor, blowup_factor, extension_degree) = (4, (1 << 3), 2)
    fn verify_fold_4_ext_2(
        &self,
        channel: &mut MidenFriVerifierChannel<QuadExt, MidenHasher>,
        evaluations: &[QuadExt],
        positions: &[usize],
    ) -> Result<((Vec<MerklePathSet>, Vec<([u8; 32], Vec<Felt>)>), Vec<u64>, Vec<u64>), VerifierError>
    {
        // 1 ----- verify the recursive components of the FRI proof -----------------------------------
        let positions = positions.to_vec();
        let evaluations = evaluations.to_vec();
        let mut final_pos_eval: Vec<(usize, QuadExt)> = vec![];
        let advice_provider =
            channel.unbatch::<4, 3>(&positions, self.domain_size(), self.layer_commitments.clone());

        let mut d_generator;
        let mut all_alphas = vec![];
        let mut all_position_evaluation = vec![];
        for (index, &position) in positions.iter().enumerate() {
            d_generator = self.domain_generator;
            let (cur_pos, evaluation, position_evaluation, alphas) = iterate_query_fold_4_quad_ext(
                &self.layer_alphas,
                &advice_provider.0,
                &advice_provider.1,
                position,
                self.options.num_fri_layers(self.domain_size()),
                self.domain_size(),
                &evaluations[index],
                &mut d_generator,
            )?;
            all_position_evaluation.extend_from_slice(&position_evaluation[..]);
            all_alphas = alphas;

            final_pos_eval.push((cur_pos, evaluation));
        }

        // 2 ----- verify the remainder of the FRI proof ----------------------------------------------

        // read the remainder from the channel and make sure it matches with the columns
        // of the previous layer
        let remainder_commitment = self.layer_commitments.last().unwrap();
        let remainder = channel.read_remainder::<4>(remainder_commitment)?;
        for (pos, eval) in final_pos_eval.iter() {
            if remainder[*pos] != *eval {
                return Err(VerifierError::InvalidRemainderFolding);
            }
        }

        Ok((advice_provider, all_position_evaluation, all_alphas))
    }
}

fn iterate_query_fold_4_quad_ext(
    layer_alphas: &Vec<QuadExt>,
    m_path_sets: &Vec<MerklePathSet>,
    key_val_map: &Vec<([u8; 32], Vec<Felt>)>,
    position: usize,
    number_of_layers: usize,
    initial_domain_size: usize,
    evaluation: &QuadExt,
    domain_generator: &mut Felt,
) -> Result<(usize, QuadExtension<Felt>, Vec<u64>, Vec<u64>), VerifierError> {
    let mut cur_pos = position;
    let mut evaluation = *evaluation;
    let mut domain_size = initial_domain_size;
    let domain_offset = Felt::GENERATOR;

    let initial_domain_generator = *domain_generator;
    let norm_cst = Felt::get_root_of_unity(2).inv();
    let mut init_exp = initial_domain_generator.exp((position as u64).into());

    let arr = vec![evaluation];
    let a = QuadExt::slice_as_base_elements(&arr);

    let position_evaluation =
        vec![a[0].as_int(), a[1].as_int(), (position as u64).into(), init_exp.as_int()];

    let mut alphas = vec![];
    for depth in 0..number_of_layers {
        let target_domain_size = domain_size / 4;

        let folded_pos = cur_pos % target_domain_size;

        // Assumes the num_partitions == 1
        let position_index = folded_pos;

        let tree_depth = log2(target_domain_size);

        let query_nodes = m_path_sets[depth]
            .get_node(NodeIndex::new(tree_depth as u8, position_index as u64))
            .unwrap();
        let query_values = &key_val_map
            .iter()
            .filter(|(k, _)| *k == query_nodes.into_bytes())
            .next()
            .expect("must contain the leaf values")
            .1;

        let query_values = [
            QuadExt::new(query_values[0], query_values[1]),
            QuadExt::new(query_values[2], query_values[3]),
            QuadExt::new(query_values[4], query_values[5]),
            QuadExt::new(query_values[6], query_values[7]),
        ];

        let query_value = query_values[cur_pos / target_domain_size];

        if evaluation != query_value {
            return Err(VerifierError::InvalidLayerFolding(depth));
        }

        let xs_new = match cur_pos / target_domain_size {
            0 => init_exp,
            1 => init_exp * norm_cst,
            2 => init_exp * (norm_cst * norm_cst),
            _ => init_exp * (norm_cst * norm_cst * norm_cst),
        } * domain_offset;

        init_exp = init_exp * init_exp * init_exp * init_exp;

        evaluation = {
            let f_minus_x = query_values[2];
            let f_x = query_values[0];
            let x_star = QuadExt::from(xs_new);
            let alpha = layer_alphas[depth];

            let tmp0 = fri_2(f_x, f_minus_x, x_star, alpha);

            let f_minus_x = query_values[3];
            let f_x = query_values[1];
            let alpha = layer_alphas[depth];

            let tmp1 = fri_2(f_x, f_minus_x, x_star * QuadExt::from(norm_cst.inv()), alpha);

            fri_2(tmp0, tmp1, x_star * x_star, alpha * alpha)
        };

        let arr = vec![layer_alphas[depth]];
        let a = QuadExt::slice_as_base_elements(&arr);
        alphas.push(a[0].as_int());
        alphas.push(a[1].as_int());
        alphas.push(0);
        alphas.push(0);

        *domain_generator = (*domain_generator).exp((4 as u32).into());
        cur_pos = folded_pos;
        domain_size /= 4;
    }

    Ok((cur_pos, evaluation, position_evaluation, alphas))
}

impl UnBatch<QuadExt, MidenHasher> for MidenFriVerifierChannel<QuadExt, MidenHasher> {
    fn unbatch<const N: usize, const W: usize>(
        &mut self,
        positions_: &[usize],
        domain_size: usize,
        layer_commitments: Vec<MidenDigest>,
    ) -> (Vec<MerklePathSet>, Vec<([u8; 32], Vec<Felt>)>) {
        let queries = self.layer_queries().clone();
        let mut current_domain_size = domain_size;
        let mut positions = positions_.to_vec();
        let depth = layer_commitments.len() - 1;

        let mut adv_key_map = vec![];
        let mut sets = vec![];
        let mut layer_proofs = self.layer_proofs();
        for i in 0..depth {
            let mut folded_positions = fold_positions(&positions, current_domain_size, N);

            let layer_proof = layer_proofs.remove(0);

            let mut unbatched_proof = layer_proof.into_paths(&folded_positions).unwrap();
            let x = group_vector_elements::<QuadExt, N>(queries[i].clone());
            assert_eq!(x.len(), unbatched_proof.len());

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

            let new_set = MerklePathSet::new((log2(current_domain_size / N)) as u8);

            let iter_pos = folded_positions.iter_mut().map(|a| *a as u64);
            let nodes_tmp = nodes.clone();
            let iter_nodes = nodes_tmp.iter();
            let iter_paths = paths.into_iter();
            let mut tmp_vec = vec![];
            for (p, (node, path)) in iter_pos.zip(iter_nodes.zip(iter_paths)) {
                tmp_vec.push((p, *node, path));
            }

            let new_set = new_set.with_paths(tmp_vec).expect("should not fail from paths");
            sets.push(new_set);

            let _empty: () = nodes
                .into_iter()
                .zip(x.iter())
                .map(|(a, b)| {
                    let mut value = QuadExt::slice_as_base_elements(b).to_owned();
                    value.extend([ZERO; 4]);
                    adv_key_map.push((a.to_owned().into_bytes(), value));
                })
                .collect();

            mem::swap(&mut positions, &mut folded_positions);
            current_domain_size = current_domain_size / N;
        }

        (sets, adv_key_map)
    }
}

// Helper function
fn fri_2<E, B>(f_x: E, f_minus_x: E, x_star: E, alpha: E) -> E
where
    B: StarkField,
    E: FieldElement<BaseField = B>,
{
    (f_x + f_minus_x + ((f_x - f_minus_x) * alpha / x_star)) / E::ONE.double()
}
