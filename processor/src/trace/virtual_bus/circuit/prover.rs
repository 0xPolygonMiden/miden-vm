use super::{
    super::sum_check::Proof as SumCheckProof, error::ProverError,
    gkr_merge_composition_from_composition_polys, BeforeFinalLayerProof, FinalLayerProof,
    GkrCircuitProof, GkrClaim, GkrComposition,
};
use crate::trace::virtual_bus::{
    multilinear::{CompositionPolynomial, EqFunction, MultiLinearPoly},
    sum_check::{FinalClaimBuilder, FinalOpeningClaim, RoundClaim, RoundProof},
    SumCheckProver,
};
use alloc::{borrow::ToOwned, sync::Arc, vec::Vec};
use core::marker::PhantomData;
use vm_core::{Felt, FieldElement};
use winter_prover::crypto::{ElementHasher, RandomCoin};

/// Layered circuit for computing a sum of fractions.
///
/// The circuit computes a sum of fractions based on the formula a / c + b / d = (a * d + b * c) / (c * d)
/// which defines a "gate" ((a, b), (c, d)) --> (a * d + b * c, c * d) upon which the [`FractionalSumCircuit`]
/// is built. Due to the uniformity of the circuit, each of the circuit layers collect all the:
///
/// 1. `a`'s into a [`MultiLinearPoly`] called `p_0`.
/// 2. `b`'s into a [`MultiLinearPoly`] called `p_1`.
/// 3. `c`'s into a [`MultiLinearPoly`] called `q_0`.
/// 4. `d`'s into a [`MultiLinearPoly`] called `q_1`.
///
/// The relation between two subsequent layers is given by the formula
///
/// p_0[layer + 1](x_0, x_1, ..., x_{ν - 2}) = p_0[layer](x_0, x_1, ..., x_{ν - 2}, 0) * q_1[layer](x_0, x_1, ..., x_{ν - 2}, 0)
///                                  + p_1[layer](x_0, x_1, ..., x_{ν - 2}, 0) * q_0[layer](x_0, x_1, ..., x_{ν - 2}, 0)
///
/// p_1[layer + 1](x_0, x_1, ..., x_{ν - 2}) = p_0[layer](x_0, x_1, ..., x_{ν - 2}, 1) * q_1[layer](x_0, x_1, ..., x_{ν - 2}, 1)
///                                  + p_1[layer](x_0, x_1, ..., x_{ν - 2}, 1) * q_0[layer](x_0, x_1, ..., x_{ν - 2}, 1)
///
/// and
///
/// q_0[layer + 1](x_0, x_1, ..., x_{ν - 2}) = q_0[layer](x_0, x_1, ..., x_{ν - 2}, 0) * q_1[layer](x_0, x_1, ..., x_{ν - 1}, 0)
///                                  
/// q_1[layer + 1](x_0, x_1, ..., x_{ν - 2}) = q_0[layer](x_0, x_1, ..., x_{ν - 2}, 1) * q_1[layer](x_0, x_1, ..., x_{ν - 1}, 1)
///
/// This means that layer ν will be the output layer and will consist of four values
/// (p_0[ν - 1], p_1[ν - 1], p_0[ν - 1], p_1[ν - 1]) ∈ 𝔽^ν.
#[derive(Debug)]
pub struct FractionalSumCircuit<E: FieldElement> {
    p_0_vec: Vec<MultiLinearPoly<E>>,
    p_1_vec: Vec<MultiLinearPoly<E>>,
    q_0_vec: Vec<MultiLinearPoly<E>>,
    q_1_vec: Vec<MultiLinearPoly<E>>,
}

impl<E: FieldElement> FractionalSumCircuit<E> {
    /// Computes The values of the gate outputs for each of the layers of the fractional sum circuit.
    pub fn new(num_den: Vec<Vec<E>>) -> Result<Self, ProverError> {
        let num_evaluations = num_den[0].len();

        if !num_den.iter().all(|t| t.len() == num_evaluations) {
            return Err(ProverError::MismatchingLengthsCircuitInputs);
        }

        if !num_evaluations.is_power_of_two() {
            return Err(ProverError::InputsMustBePowerTwo);
        }

        if num_evaluations < 2 {
            return Err(ProverError::InputsAtLeastTwo);
        }

        let num_layers = num_evaluations.ilog2() as usize;
        let mut p_0_vec: Vec<MultiLinearPoly<E>> = Vec::with_capacity(num_layers);
        let mut p_1_vec: Vec<MultiLinearPoly<E>> = Vec::with_capacity(num_layers);
        let mut q_0_vec: Vec<MultiLinearPoly<E>> = Vec::with_capacity(num_layers);
        let mut q_1_vec: Vec<MultiLinearPoly<E>> = Vec::with_capacity(num_layers);

        let p_0 = MultiLinearPoly::from_evaluations(num_den[0].to_owned())
            .map_err(|_| ProverError::FailedToGenerateML)?;
        let p_1 = MultiLinearPoly::from_evaluations(num_den[1].to_owned())
            .map_err(|_| ProverError::FailedToGenerateML)?;
        let q_0 = MultiLinearPoly::from_evaluations(num_den[2].to_owned())
            .map_err(|_| ProverError::FailedToGenerateML)?;
        let q_1 = MultiLinearPoly::from_evaluations(num_den[3].to_owned())
            .map_err(|_| ProverError::FailedToGenerateML)?;
        p_0_vec.push(p_0);
        p_1_vec.push(p_1);
        q_0_vec.push(q_0);
        q_1_vec.push(q_1);

        for i in 0..num_layers {
            let (output_p_0, output_p_1, output_q_0, output_q_1) =
                FractionalSumCircuit::compute_layer(
                    &p_0_vec[i],
                    &p_1_vec[i],
                    &q_0_vec[i],
                    &q_1_vec[i],
                )?;
            p_0_vec.push(output_p_0);
            p_1_vec.push(output_p_1);
            q_0_vec.push(output_q_0);
            q_1_vec.push(output_q_1);
        }

        Ok(FractionalSumCircuit {
            p_0_vec,
            p_1_vec,
            q_0_vec,
            q_1_vec,
        })
    }

    /// Computes the output values of the layer given a set of input values
    #[allow(clippy::type_complexity)]
    fn compute_layer(
        inp_p_0: &MultiLinearPoly<E>,
        inp_p_1: &MultiLinearPoly<E>,
        inp_q_0: &MultiLinearPoly<E>,
        inp_q_1: &MultiLinearPoly<E>,
    ) -> Result<
        (MultiLinearPoly<E>, MultiLinearPoly<E>, MultiLinearPoly<E>, MultiLinearPoly<E>),
        ProverError,
    > {
        let len = inp_q_0.num_evaluations();
        let outp_p_0 = (0..len / 2)
            .map(|i| inp_p_0[i] * inp_q_1[i] + inp_p_1[i] * inp_q_0[i])
            .collect::<Vec<E>>();
        let outp_p_1 = (len / 2..len)
            .map(|i| inp_p_0[i] * inp_q_1[i] + inp_p_1[i] * inp_q_0[i])
            .collect::<Vec<E>>();
        let outp_q_0 = (0..len / 2).map(|i| inp_q_0[i] * inp_q_1[i]).collect::<Vec<E>>();
        let outp_q_1 = (len / 2..len).map(|i| inp_q_0[i] * inp_q_1[i]).collect::<Vec<E>>();

        Ok((
            MultiLinearPoly::from_evaluations(outp_p_0)
                .map_err(|_| ProverError::FailedToGenerateML)?,
            MultiLinearPoly::from_evaluations(outp_p_1)
                .map_err(|_| ProverError::FailedToGenerateML)?,
            MultiLinearPoly::from_evaluations(outp_q_0)
                .map_err(|_| ProverError::FailedToGenerateML)?,
            MultiLinearPoly::from_evaluations(outp_q_1)
                .map_err(|_| ProverError::FailedToGenerateML)?,
        ))
    }

    /// Given a value r, computes the evaluation of the last layer at r when interpreted as (two)
    /// multilinear polynomials.
    pub fn evaluate_output_layer(&self, r: E) -> (E, E) {
        let len = self.p_0_vec.len();
        assert_eq!(self.p_0_vec[len - 1].num_variables(), 0);
        assert_eq!(self.p_1_vec[len - 1].num_variables(), 0);
        assert_eq!(self.q_0_vec[len - 1].num_variables(), 0);
        assert_eq!(self.q_1_vec[len - 1].num_variables(), 0);

        let mut p = self.p_0_vec[len - 1].clone();
        p.extend(&self.p_1_vec[len - 1]);
        let mut q = self.q_0_vec[len - 1].clone();
        q.extend(&self.q_1_vec[len - 1]);

        (p.evaluate(&[r]), q.evaluate(&[r]))
    }

    /// Outputs the value of the circuit output layer.
    pub fn output_layer(&self) -> [E; 4] {
        let len = self.p_0_vec.len();
        let poly_a = self.p_0_vec[len - 1][0];
        let poly_b = self.p_1_vec[len - 1][0];
        let poly_c = self.q_0_vec[len - 1][0];
        let poly_d = self.q_1_vec[len - 1][0];
        [poly_a, poly_b, poly_c, poly_d]
    }
}

/// Evaluates and proves a fractional sum circuit given a set of composition polynomials.
///
/// Each individual component of the quadruple [p_0, p_1, q_0, q_1] is of the form:
///
/// m(z_0, ... , z_{μ - 1}, x_0, ... , x_{ν - 1}) =
/// \sum_{y ∈ {0,1}^μ} EQ(z, y) * g_{[y]}(f_0(x_0, ... , x_{ν - 1}), ... , f_{κ - 1}(x_0, ... , x_{ν - 1}))
///
/// where:
///
/// 1. μ is the log_2 of the number of different numerator/denominator expressions divided by two.
/// 2. [y] := \sum_{j = 0}^{μ - 1} y_j * 2^j
/// 3. κ is the number of multi-linears (i.e., main trace columns) involved in the computation
/// of the circuit (i.e., virtual bus).
/// 4. ν is the log_2 of the trace length.
///
/// The above `m` is usually referred to as the merge of the individual composed multi-linear
/// polynomials  g_{[y]}(f_0(x_0, ... , x_{ν - 1}), ... , f_{κ - 1}(x_0, ... , x_{ν - 1})).
///
/// The composition polynomials `g` are provided as inputs and then used in order to compute
/// the evaluations of each of the four merge polynomials over {0, 1}^{μ + ν}. The resulting
/// evaluations are then used in order to evaluate [`FractionalSumCircuit`].
/// At this point, the GKR protocol is used to prove the correctness of circuit evaluation. It
/// should be noted that the input layer, which corresponds to the last layer treated by the GKR
/// protocol, is handled differently from the other layers.
/// More specifically, the sum-check protocol used for the input layer is composed of two sum-check
/// protocols, the first one works directly with the evaluations of the `m`'s over {0, 1}^{μ + ν}
/// and runs for μ rounds.
/// After these μ rounds, and using the resulting [`RoundClaim`], we run the second and final
/// sum-check protocol for ν rounds on the composed multi-linear polynomial given by
///
/// \sum_{y ∈ {0,1}^μ} EQ(ρ', y) * g_{[y]}(f_0(x_0, ... , x_{ν - 1}), ... , f_{κ - 1}(x_0, ... , x_{ν - 1}))
///
/// where ρ' is the randomness sampled during the first sum-check protocol.
///
/// As part of the final sum-check protocol, the openings {f_j(ρ)} are provided as part of
/// a [`FinalOpeningClaim`]. This latter claim will be proven by the STARK prover later on using
/// the auxiliary trace.
pub fn prove<
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    composition_polys: Vec<Vec<Arc<dyn CompositionPolynomial<E>>>>,
    mls: &mut Vec<MultiLinearPoly<E>>,
    transcript: &mut C,
) -> Result<GkrCircuitProof<E>, ProverError> {
    // evaluate the numerators and denominators over the boolean hyper-cube {0, 1}^{μ + ν}
    let input: Vec<Vec<E>> = evaluate_composition_polys(mls, &composition_polys);

    // evaluate the GKR fractional sum circuit
    let mut circuit = FractionalSumCircuit::new(input)?;

    // run the GKR prover for all layers except the input layer
    let (before_final_layer_proofs, gkr_claim) =
        prove_before_final_circuit_layers(&mut circuit, transcript)?;

    // run the GKR prover for the input layer
    let num_rounds_before_merge = composition_polys[0].len().ilog2() as usize;
    let final_layer_proof = prove_final_circuit_layer(
        composition_polys,
        mls,
        num_rounds_before_merge,
        gkr_claim,
        &mut circuit,
        transcript,
    )?;

    // include the circuit output as part of the final proof
    let circuit_outputs = circuit.output_layer();

    Ok(GkrCircuitProof {
        circuit_outputs,
        before_final_layer_proofs,
        final_layer_proof,
    })
}

/// Proves the final GKR layer which corresponds to the input circuit layer.
fn prove_final_circuit_layer<
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    composition_polys: Vec<Vec<Arc<dyn CompositionPolynomial<E>>>>,
    mls: &mut Vec<MultiLinearPoly<E>>,
    num_rounds_merge: usize,
    gkr_claim: GkrClaim<E>,
    circuit: &mut FractionalSumCircuit<E>,
    transcript: &mut C,
) -> Result<FinalLayerProof<E>, ProverError> {
    // parse the [GkrClaim] resulting from the previous GKR layer
    let GkrClaim {
        evaluation_point,
        claimed_evaluation,
    } = gkr_claim;

    // compute the EQ function at the evaluation point
    let poly_x = EqFunction::ml_at(evaluation_point.clone());

    // get the multi-linears of the 4 merge polynomials
    let poly_a = circuit.p_0_vec[0].to_owned();
    let poly_b = circuit.p_1_vec[0].to_owned();
    let poly_c = circuit.q_0_vec[0].to_owned();
    let poly_d = circuit.q_1_vec[0].to_owned();
    let mut merged_mls = vec![poly_a, poly_b, poly_c, poly_d, poly_x];

    // run the first sum-check protocol
    let ((round_claim, before_merge_proof), r_sum_check) = sum_check_prover_plain_partial(
        claimed_evaluation,
        num_rounds_merge,
        &mut merged_mls,
        transcript,
    )?;

    // parse the output of the first sum-check protocol
    let RoundClaim {
        eval_point: rand_merge,
        claim,
    } = round_claim;

    // create the composed multi-linear for the second sum-check protocol using the randomness
    // sampled during the first one
    let gkr_composition =
        gkr_merge_composition_from_composition_polys(&composition_polys, r_sum_check, rand_merge);

    // include the partially evaluated at the first sum-check randomness EQ multi-linear
    mls.push(merged_mls[4].clone());

    // run the second sum-check protocol
    let main_prover = SumCheckProver::new(gkr_composition, SimpleGkrFinalClaimBuilder(PhantomData));
    let after_merge_proof = main_prover
        .prove(claim, mls, transcript)
        .map_err(|_| ProverError::FailedToProveSumCheck)?;

    Ok(FinalLayerProof {
        before_merge_proof,
        after_merge_proof,
    })
}

/// Proves all GKR layers except for input layer.
fn prove_before_final_circuit_layers<
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    circuit: &mut FractionalSumCircuit<E>,
    transcript: &mut C,
) -> Result<(BeforeFinalLayerProof<E>, GkrClaim<E>), ProverError> {
    // absorb the circuit output layer. This corresponds to sending the four values of the output
    // layer to the verifier. The verifier then replies with a challenge `r` in order to evaluate
    // `p` and `q` at `r` as multi-linears.
    let num_layers = circuit.p_0_vec.len();
    let data = vec![
        circuit.p_0_vec[num_layers - 1][0],
        circuit.p_1_vec[num_layers - 1][0],
        circuit.q_0_vec[num_layers - 1][0],
        circuit.q_1_vec[num_layers - 1][0],
    ];
    // generate the challenge `r`
    transcript.reseed(H::hash_elements(&data));

    // generate the challenge and reduce [p0, p1, q0, q1] to [pr, qr]
    let r = transcript.draw().map_err(|_| ProverError::FailedToGenerateChallenge)?;
    let mut claim = circuit.evaluate_output_layer(r);

    let mut proof_layers: Vec<SumCheckProof<E>> = Vec::new();
    let mut rand = vec![r];
    for layer_id in (1..num_layers - 1).rev() {
        // construct the Lagrange kernel evaluated at the previous GKR round randomness
        let poly_x = EqFunction::ml_at(rand.clone());

        // construct the vector of multi-linear polynomials
        // TODO: avoid unnecessary allocation
        let poly_a = circuit.p_0_vec[layer_id].to_owned();
        let poly_b = circuit.p_1_vec[layer_id].to_owned();
        let poly_c = circuit.q_0_vec[layer_id].to_owned();
        let poly_d = circuit.q_1_vec[layer_id].to_owned();
        let mut mls = vec![poly_a, poly_b, poly_c, poly_d, poly_x];

        // run the sumcheck protocol
        let (proof, _) = sum_check_prover_plain_full(claim, &mut mls, transcript)?;

        // sample a random challenge to reduce claims
        transcript.reseed(H::hash_elements(&proof.openings_claim.openings));
        let r_layer = transcript.draw().map_err(|_| ProverError::FailedToGenerateChallenge)?;

        // reduce the claim
        let p0 = proof.openings_claim.openings[0];
        let p1 = proof.openings_claim.openings[1];
        let q0 = proof.openings_claim.openings[2];
        let q1 = proof.openings_claim.openings[3];
        claim = (p0 + r_layer * (p1 - p0), q0 + r_layer * (q1 - q0));

        // collect the randomness used for the current layer
        let mut ext = proof.openings_claim.eval_point.clone();
        ext.push(r_layer);
        rand = ext;

        proof_layers.push(proof);
    }

    Ok((
        BeforeFinalLayerProof {
            proof: proof_layers,
        },
        GkrClaim {
            evaluation_point: rand,
            claimed_evaluation: claim,
        },
    ))
}

/// Runs the first sum-check prover for the input layer.
#[allow(clippy::type_complexity)]
fn sum_check_prover_plain_partial<
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    claim: (E, E),
    num_rounds: usize,
    ml_polys: &mut [MultiLinearPoly<E>],
    transcript: &mut C,
) -> Result<((RoundClaim<E>, Vec<RoundProof<E>>), E), ProverError> {
    // generate challenge to batch two sumchecks
    let data = vec![claim.0, claim.1];
    transcript.reseed(H::hash_elements(&data));
    let r_batch = transcript.draw().map_err(|_| ProverError::FailedToGenerateChallenge)?;
    let claim = claim.0 + claim.1 * r_batch;

    // generate the composition polynomial
    let composer = GkrComposition::new(r_batch);

    // run the sum-check protocol
    let main_prover = SumCheckProver::new(composer, SimpleGkrFinalClaimBuilder(PhantomData));
    let proof = main_prover
        .prove_rounds(claim, ml_polys, num_rounds, transcript)
        .map_err(|_| ProverError::FailedToProveSumCheck)?;

    Ok((proof, r_batch))
}

/// Runs the sum-check prover used in all but the input layer.
fn sum_check_prover_plain_full<
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    claim: (E, E),
    ml_polys: &mut [MultiLinearPoly<E>],
    transcript: &mut C,
) -> Result<(SumCheckProof<E>, E), ProverError> {
    // generate challenge to batch two sumchecks
    let data = vec![claim.0, claim.1];
    transcript.reseed(H::hash_elements(&data));
    let r_batch = transcript.draw().map_err(|_| ProverError::FailedToGenerateChallenge)?;
    let claim_ = claim.0 + claim.1 * r_batch;

    // generate the composition polynomial
    let composer = GkrComposition::new(r_batch);

    // run the sum-check protocol
    let main_prover = SumCheckProver::new(composer, SimpleGkrFinalClaimBuilder(PhantomData));
    let proof = main_prover
        .prove(claim_, ml_polys, transcript)
        .map_err(|_| ProverError::FailedToProveSumCheck)?;

    Ok((proof, r_batch))
}

/// Computes the evaluations over {0, 1}^{μ + ν} of
///
/// m(z_0, ... , z_{μ - 1}, x_0, ... , x_{ν - 1}) =
/// \sum_{y ∈ {0,1}^μ} EQ(z, y) * g_{[y]}(f_0(x_0, ... , x_{ν - 1}), ... , f_{κ - 1}(x_0, ... , x_{ν - 1}))
fn evaluate_composition_polys<E: FieldElement<BaseField = Felt> + 'static>(
    mls: &[MultiLinearPoly<E>],
    composition_polys: &[Vec<Arc<dyn CompositionPolynomial<E>>>],
) -> Vec<Vec<E>> {
    let num_evaluations = 1 << mls[0].num_variables();
    let mut num_den: Vec<Vec<E>> =
        (0..4).map(|_| Vec::with_capacity(num_evaluations)).collect::<Vec<_>>();

    for i in 0..num_evaluations {
        for j in 0..4 {
            let query: Vec<E> = mls.iter().map(|ml| ml[i]).collect();

            composition_polys[j].iter().for_each(|c| {
                let evaluation = c.as_ref().evaluate(&query);
                num_den[j].push(evaluation);
            });
        }
    }
    num_den
}

/// Constructs [`FinalOpeningClaim`] for the sum-checks used in the GKR protocol.
///
/// TODO: currently, this just removes the EQ evaluation as it can be computed by the verifier.
/// This should be generalized for other "transparent" multi-linears e.g., periodic columns.
struct SimpleGkrFinalClaimBuilder<E: FieldElement>(PhantomData<E>);

impl<E: FieldElement> FinalClaimBuilder for SimpleGkrFinalClaimBuilder<E> {
    type Field = E;

    fn build_claim(
        &self,
        openings: Vec<Self::Field>,
        evaluation_point: &[Self::Field],
    ) -> FinalOpeningClaim<Self::Field> {
        FinalOpeningClaim {
            eval_point: evaluation_point.to_vec(),
            openings: (openings[..openings.len() - 1]).to_vec(),
        }
    }
}
