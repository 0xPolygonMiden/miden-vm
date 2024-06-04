use super::{
    super::sum_check::Proof as SumCheckProof, compute_input_layer_wires_at_main_trace_query,
    error::ProverError, BeforeFinalLayerProof, CircuitWire, FinalLayerProof, GkrCircuitProof,
    GkrClaim, GkrComposition, GkrCompositionMerge, NUM_WIRES_PER_TRACE_ROW,
};
use crate::trace::virtual_bus::{
    multilinear::{EqFunction, MultiLinearPoly},
    sum_check::{FinalClaimBuilder, FinalOpeningClaim, RoundClaim, RoundProof},
    SumCheckProver,
};
use alloc::vec::Vec;
use core::marker::PhantomData;
use miden_air::trace::main_trace::MainTrace;
use vm_core::{Felt, FieldElement};
use winter_prover::crypto::{ElementHasher, RandomCoin};

/// Evaluation of a layered circuit for computing a sum of fractions.
///
/// The circuit computes a sum of fractions based on the formula a / c + b / d = (a * d + b * c) /
/// (c * d) which defines a "gate" ((a, b), (c, d)) --> (a * d + b * c, c * d) upon which the
/// [`EvaluatedCircuit`] is built. Due to the uniformity of the circuit, each of the circuit
/// layers collect all the:
///
/// 1. `a`'s into a [`MultiLinearPoly`] called `left_numerators`.
/// 2. `b`'s into a [`MultiLinearPoly`] called `right_numerators`.
/// 3. `c`'s into a [`MultiLinearPoly`] called `left_denominators`.
/// 4. `d`'s into a [`MultiLinearPoly`] called `right_denominators`.
///
/// The relation between two subsequent layers is given by the formula
///
/// p_0[layer + 1](x_0, x_1, ..., x_{ν - 2}) = p_0[layer](x_0, x_1, ..., x_{ν - 2}, 0) *
/// q_1[layer](x_0, x_1, ..., x_{ν - 2}, 0)
///                                  + p_1[layer](x_0, x_1, ..., x_{ν - 2}, 0) * q_0[layer](x_0,
///                                    x_1, ..., x_{ν - 2}, 0)
///
/// p_1[layer + 1](x_0, x_1, ..., x_{ν - 2}) = p_0[layer](x_0, x_1, ..., x_{ν - 2}, 1) *
/// q_1[layer](x_0, x_1, ..., x_{ν - 2}, 1)
///                                  + p_1[layer](x_0, x_1, ..., x_{ν - 2}, 1) * q_0[layer](x_0,
///                                    x_1, ..., x_{ν - 2}, 1)
///
/// and
///
/// q_0[layer + 1](x_0, x_1, ..., x_{ν - 2}) = q_0[layer](x_0, x_1, ..., x_{ν - 2}, 0) *
/// q_1[layer](x_0, x_1, ..., x_{ν - 1}, 0)                                  
/// q_1[layer + 1](x_0, x_1, ..., x_{ν - 2}) = q_0[layer](x_0, x_1, ..., x_{ν - 2}, 1) *
/// q_1[layer](x_0, x_1, ..., x_{ν - 1}, 1)
///
/// This logic is encoded in [`CircuitWire`].
///
/// This means that layer ν will be the output layer and will consist of four values
/// (p_0[ν - 1], p_1[ν - 1], p_0[ν - 1], p_1[ν - 1]) ∈ 𝔽^ν.
pub struct EvaluatedCircuit<E: FieldElement> {
    layer_polys: Vec<CircuitLayerPolys<E>>,
}

impl<E: FieldElement> EvaluatedCircuit<E> {
    /// Creates a new [`EvaluatedCircuit`] by evaluating the circuit where the input layer is
    /// defined from the main trace columns.
    pub fn new(
        main_trace_columns: &[MultiLinearPoly<E>],
        log_up_randomness: &[E],
    ) -> Result<Self, ProverError> {
        let mut layer_polys = Vec::new();

        let mut current_layer = Self::generate_input_layer(main_trace_columns, log_up_randomness);
        while current_layer.num_wires() > 1 {
            let next_layer = Self::compute_next_layer(&current_layer);

            layer_polys.push(CircuitLayerPolys::from_circuit_layer(current_layer));

            current_layer = next_layer;
        }

        Ok(Self { layer_polys })
    }

    /// Returns a layer of the evaluated circuit.
    ///
    /// Note that the return type is [`LayerPolys`] as opposed to [`Layer`], since the evaluated
    /// circuit is stored in a representation which can be proved using GKR.
    pub fn get_layer(&self, layer_idx: usize) -> &CircuitLayerPolys<E> {
        &self.layer_polys[layer_idx]
    }

    /// Returns all layers of the evaluated circuit, starting from the input layer.
    ///
    /// Note that the return type is a slice of [`CircuitLayerPolys`] as opposed to
    /// [`CircuitLayer`], since the evaluated layers are stored in a representation which can be
    /// proved using GKR.
    pub fn layers(&self) -> &[CircuitLayerPolys<E>] {
        &self.layer_polys
    }

    /// Returns the numerator/denominator polynomials representing the output layer of the circuit.
    pub fn output_layer(&self) -> &CircuitLayerPolys<E> {
        self.layer_polys.last().expect("circuit has at least one layer")
    }

    /// Evaluates the output layer at `query`, where the numerators of the output layer are treated
    /// as evaluations of a multilinear polynomial, and similarly for the denominators.
    pub fn evaluate_output_layer(&self, query: E) -> (E, E) {
        let CircuitLayerPolys {
            numerators,
            denominators,
        } = self.output_layer();

        (numerators.evaluate(&[query]), denominators.evaluate(&[query]))
    }

    // HELPERS
    // -------------------------------------------------------------------------------------------

    /// Generates the input layer of the circuit from the main trace columns and some randomness
    /// provided by the verifier.
    fn generate_input_layer(
        main_trace_columns: &[MultiLinearPoly<E>],
        log_up_randomness: &[E],
    ) -> CircuitLayer<E> {
        let num_evaluations = main_trace_columns[0].num_evaluations();
        let mut input_layer_nodes = Vec::with_capacity(num_evaluations * NUM_WIRES_PER_TRACE_ROW);

        for i in 0..num_evaluations {
            let nodes_from_trace_row = {
                let query: Vec<E> = main_trace_columns.iter().map(|ml| ml[i]).collect();
                compute_input_layer_wires_at_main_trace_query(&query, log_up_randomness)
            };

            input_layer_nodes.extend(nodes_from_trace_row);
        }

        CircuitLayer::new(input_layer_nodes)
    }

    /// Computes the subsequent layer of the circuit from a given layer.
    fn compute_next_layer(prev_layer: &CircuitLayer<E>) -> CircuitLayer<E> {
        let next_layer_nodes = prev_layer
            .wires()
            .chunks_exact(2)
            .map(|input_wires| {
                let left_input_wire = input_wires[0];
                let right_input_wire = input_wires[1];

                // output wire
                left_input_wire + right_input_wire
            })
            .collect();

        CircuitLayer::new(next_layer_nodes)
    }
}

/// Represents a layer in a [`EvaluatedCircuit`].
///
/// A layer is made up of a set of `n` wires, where `n` is a power of two. This is the natural
/// circuit representation of a layer, where each consecutive pair of wires are summed to yield an
/// wire in the subsequent layer of a [`EvaluatedCircuit`].
///
/// Note that a [`Layer`] needs to be first converted to a [`LayerPolys`] before the evaluation of
/// the layer can be proved using GKR.
struct CircuitLayer<E: FieldElement> {
    wires: Vec<CircuitWire<E>>,
}

impl<E: FieldElement> CircuitLayer<E> {
    /// Creates a new [`Layer`] from a set of projective coordinates.
    ///
    /// Panics if the number of projective coordinates is not a power of two.
    pub fn new(wires: Vec<CircuitWire<E>>) -> Self {
        assert!(wires.len().is_power_of_two());

        Self { wires }
    }

    /// Returns the wires that make up this circuit layer.
    pub fn wires(&self) -> &[CircuitWire<E>] {
        &self.wires
    }

    /// Returns the number of wires in the layer.
    pub fn num_wires(&self) -> usize {
        self.wires.len()
    }
}

/// Holds a layer of [`EvaluatedCircuit`] in a representation amenable to proving circuit evaluation
/// using GKR.
///
/// Specifically, each element of a [`CircuitWire`] pair `[(a, b), (c, d)]` in a [`Layer`]'s is
/// added to the definition one of two [`MultiLinearPoly`]:
/// - `numerators`: [a, c]
/// - `denominators`: [b, d]
#[derive(Clone, Debug)]
pub struct CircuitLayerPolys<E: FieldElement> {
    pub numerators: MultiLinearPoly<E>,
    pub denominators: MultiLinearPoly<E>,
}

impl<E> CircuitLayerPolys<E>
where
    E: FieldElement,
{
    fn from_circuit_layer(layer: CircuitLayer<E>) -> Self {
        Self::from_wires(layer.wires)
    }

    pub fn from_wires(wires: Vec<CircuitWire<E>>) -> Self {
        let mut numerators = Vec::new();
        let mut denominators = Vec::new();

        for wire in wires {
            numerators.push(wire.numerator);
            denominators.push(wire.denominator);
        }

        Self {
            numerators: MultiLinearPoly::from_evaluations(numerators)
                .expect("evaluations guaranteed to be a power of two"),
            denominators: MultiLinearPoly::from_evaluations(denominators)
                .expect("evaluations guaranteed to be a power of two"),
        }
    }
}

/// Evaluates and proves a fractional sum circuit given a set of composition polynomials.
///
/// For the input layer of the circuit, each individual component of the quadruple 
/// [p_0, p_1, q_0, q_1] is of the form:
///
/// m(z_0, ... , z_{μ - 1}, x_0, ... , x_{ν - 1}) = \sum_{y ∈ {0,1}^μ} EQ(z, y) * g_{[y]}(f_0(x_0,
/// ... , x_{ν - 1}), ... , f_{κ - 1}(x_0, ... , x_{ν
/// - 1}))
///
/// where:
///
/// 1. μ is the log_2 of the number of different numerator/denominator expressions divided by two.
/// 2. [y] := \sum_{j = 0}^{μ - 1} y_j * 2^j
/// 3. κ is the number of multi-linears (i.e., main trace columns) involved in the computation of
/// the circuit (i.e., virtual bus).
/// 4. ν is the log_2 of the trace length.
///
/// The above `m` is usually referred to as the merge of the individual composed multi-linear
/// polynomials  g_{[y]}(f_0(x_0, ... , x_{ν - 1}), ... , f_{κ - 1}(x_0, ... , x_{ν - 1})).
///
/// The composition polynomials `g` are provided as inputs and then used in order to compute the
/// evaluations of each of the four merge polynomials over {0, 1}^{μ + ν}. The resulting evaluations
/// are then used in order to evaluate the circuit. At this point, the GKR protocol is used to prove
/// the correctness of circuit evaluation. It should be noted that the input layer, which
/// corresponds to the last layer treated by the GKR protocol, is handled differently from the other
/// layers. More specifically, the sum-check protocol used for the input layer is composed of two
/// sum-check protocols, the first one works directly with the evaluations of the `m`'s over {0,
/// 1}^{μ + ν} and runs for μ rounds. After these μ rounds, and using the resulting [`RoundClaim`],
/// we run the second and final sum-check protocol for ν rounds on the composed multi-linear
/// polynomial given by
///
/// \sum_{y ∈ {0,1}^μ} EQ(ρ', y) * g_{[y]}(f_0(x_0, ... , x_{ν - 1}), ... , f_{κ - 1}(x_0, ... ,
/// x_{ν - 1}))
///
/// where ρ' is the randomness sampled during the first sum-check protocol.
///
/// As part of the final sum-check protocol, the openings {f_j(ρ)} are provided as part of a
/// [`FinalOpeningClaim`]. This latter claim will be proven by the STARK prover later on using the
/// auxiliary trace.
pub fn prove<
    E: FieldElement<BaseField = Felt>,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    trace: &MainTrace,
    log_up_randomness: Vec<E>,
    transcript: &mut C,
) -> Result<GkrCircuitProof<E>, ProverError> {
    // TODO: Optimize this so that we can work with base field element directly and thus save
    // on memory usage.
    let main_trace_columns: Vec<MultiLinearPoly<E>> = trace
        .columns()
        .map(|col| {
            let mut values: Vec<E> = col.iter().map(|value| E::from(*value)).collect();
            if let Some(value) = values.last_mut() {
                *value = E::ZERO
            }
            MultiLinearPoly::from_evaluations(values).unwrap()
        })
        .collect();

    // evaluate the GKR fractional sum circuit
    let mut circuit = EvaluatedCircuit::new(&main_trace_columns, &log_up_randomness)?;

    // run the GKR prover for all layers except the input layer
    let (before_final_layer_proofs, gkr_claim) =
        prove_before_final_circuit_layers(&mut circuit, transcript)?;

    // run the GKR prover for the input layer
    let num_rounds_before_merge = (NUM_WIRES_PER_TRACE_ROW / 2).ilog2() as usize;
    let final_layer_proof = prove_final_circuit_layer(
        log_up_randomness,
        main_trace_columns,
        num_rounds_before_merge,
        gkr_claim,
        &mut circuit,
        transcript,
    )?;

    // include the circuit output as part of the final proof
    let circuit_outputs = circuit.output_layer();

    Ok(GkrCircuitProof {
        circuit_outputs: circuit_outputs.clone(),
        before_final_layer_proofs,
        final_layer_proof,
    })
}

/// Proves the final GKR layer which corresponds to the input circuit layer.
fn prove_final_circuit_layer<
    E: FieldElement<BaseField = Felt>,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    log_up_randomness: Vec<E>,
    mut mls: Vec<MultiLinearPoly<E>>,
    num_rounds_merge: usize,
    gkr_claim: GkrClaim<E>,
    circuit: &mut EvaluatedCircuit<E>,
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
    let layer = circuit.get_layer(0);
    let (even_numerator, odd_numerator) = layer.numerators.project_least_significant_variable();
    let (even_denominator, odd_denominator) =
        layer.denominators.project_least_significant_variable();
    let mut merged_mls =
        vec![even_numerator, odd_numerator, even_denominator, odd_denominator, poly_x];
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
    let gkr_composition = GkrCompositionMerge::new(r_sum_check, rand_merge, log_up_randomness);

    // include the partially evaluated at the first sum-check randomness EQ multi-linear
    // TODO: Find a better way than to push the evaluation of `EqFunction` here.
    mls.push(merged_mls[4].clone());

    // run the second sum-check protocol
    let main_prover = SumCheckProver::new(gkr_composition, SimpleGkrFinalClaimBuilder(PhantomData));
    let after_merge_proof = main_prover.prove(claim, mls, transcript)?;

    Ok(FinalLayerProof {
        before_merge_proof,
        after_merge_proof,
    })
}

/// Proves all GKR layers except for input layer.
fn prove_before_final_circuit_layers<
    E: FieldElement<BaseField = Felt>,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    circuit: &mut EvaluatedCircuit<E>,
    transcript: &mut C,
) -> Result<(BeforeFinalLayerProof<E>, GkrClaim<E>), ProverError> {
    // absorb the circuit output layer. This corresponds to sending the four values of the output
    // layer to the verifier. The verifier then replies with a challenge `r` in order to evaluate
    // `p` and `q` at `r` as multi-linears.
    let CircuitLayerPolys {
        numerators,
        denominators,
    } = circuit.output_layer();
    let mut evaluations = numerators.evaluations().to_vec();
    evaluations.extend_from_slice(denominators.evaluations());
    transcript.reseed(H::hash_elements(&evaluations));

    // generate the challenge and reduce [p0, p1, q0, q1] to [pr, qr]
    let r = transcript.draw().map_err(|_| ProverError::FailedToGenerateChallenge)?;
    let mut claim = circuit.evaluate_output_layer(r);

    let mut proof_layers: Vec<SumCheckProof<E>> = Vec::new();
    let mut rand = vec![r];

    // Loop over all inner layers, from output to input
    for inner_layer in circuit.layers().iter().skip(1).rev().skip(1) {
        // construct the Lagrange kernel evaluated at the previous GKR round randomness
        let poly_x = EqFunction::ml_at(rand.clone());

        // construct the vector of multi-linear polynomials
        // TODO: avoid unnecessary allocation
        let (left_numerators, right_numerators) =
            inner_layer.numerators.project_least_significant_variable();
        let (left_denominators, right_denominators) =
            inner_layer.denominators.project_least_significant_variable();
        let mls =
            vec![left_numerators, right_numerators, left_denominators, right_denominators, poly_x];

        // run the sumcheck protocol
        let (proof, _) = sum_check_prover_plain_full(claim, mls, transcript)?;

        // sample a random challenge to reduce claims
        transcript.reseed(H::hash_elements(&proof.openings_claim.openings));
        let r_layer = transcript.draw().map_err(|_| ProverError::FailedToGenerateChallenge)?;

        // reduce the claim
        claim = {
            let left_numerators_opening = proof.openings_claim.openings[0];
            let right_numerators_opening = proof.openings_claim.openings[1];
            let left_denominators_opening = proof.openings_claim.openings[2];
            let right_denominators_opening = proof.openings_claim.openings[3];

            reduce_layer_claim(
                left_numerators_opening,
                right_numerators_opening,
                left_denominators_opening,
                right_denominators_opening,
                r_layer,
            )
        };

        // collect the randomness used for the current layer
        let mut ext = vec![r_layer];
        ext.extend_from_slice(&proof.openings_claim.eval_point);
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

/// We receive our 4 multilinear polynomials which were evaluated at a random point:
/// `left_numerators` (or `p0`), `right_numerators` (or `p1`), `left_denominators` (or `q0`), and
/// `right_denominators` (or `q1`). We'll call the 4 evaluations at a random point `p0(r)`, `p1(r)`,
/// `q0(r)`, and `q1(r)`, respectively, where `r` is the random point. Note that `r` is a shorthand
/// for a tuple of random values `(r_0, ... r_{l-1})`, where `2^{l + 1}` is the number of wires in
/// the layer.
///
/// It is important to recall how `p0` and `p1` were constructed (and analogously for `q0` and
/// `q1`). They are the `numerators` layer polynomial (or `p`) evaluations `p(0, r)` and `p(1, r)`,
/// obtained from [`MultiLinearPoly::project_least_significant_variable`]. Hence, `[p0, p1]` form
/// the evaluations of polynomial `p'(x_0) = p(x_0, r)`. Then, the round claim for `numerators`,
/// defined as `p(r_layer, r)`, is simply `p'(r_layer)`.
fn reduce_layer_claim<E>(
    left_numerators_opening: E,
    right_numerators_opening: E,
    left_denominators_opening: E,
    right_denominators_opening: E,
    r_layer: E,
) -> (E, E)
where
    E: FieldElement<BaseField = Felt>,
{
    // This is the `numerators` layer polynomial `f(x_0) = numerators(x_0, rx_0, ..., rx_{l-1})`,
    // where `rx_0, ..., rx_{l-1}` are the random variables that were sampled during the sumcheck
    // round for this layer.
    let numerators_univariate =
        MultiLinearPoly::from_evaluations(vec![left_numerators_opening, right_numerators_opening])
            .unwrap();

    // This is analogous to `numerators_univariate`, but for the `denominators` layer polynomial
    let denominators_univariate = MultiLinearPoly::from_evaluations(vec![
        left_denominators_opening,
        right_denominators_opening,
    ])
    .unwrap();

    (
        numerators_univariate.evaluate(&[r_layer]),
        denominators_univariate.evaluate(&[r_layer]),
    )
}

/// Runs the first sum-check prover for the input layer.
#[allow(clippy::type_complexity)]
fn sum_check_prover_plain_partial<
    E: FieldElement<BaseField = Felt>,
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
    let proof = main_prover.prove_rounds(claim, ml_polys, num_rounds, transcript)?;

    Ok((proof, r_batch))
}

/// Runs the sum-check prover used in all but the input layer.
fn sum_check_prover_plain_full<
    E: FieldElement<BaseField = Felt>,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    claim: (E, E),
    ml_polys: Vec<MultiLinearPoly<E>>,
    transcript: &mut C,
) -> Result<(SumCheckProof<E>, E), ProverError> {
    // generate challenge to batch two sumchecks
    transcript.reseed(H::hash_elements(&[claim.0, claim.1]));
    let r_batch = transcript.draw().map_err(|_| ProverError::FailedToGenerateChallenge)?;
    let claim_ = claim.0 + claim.1 * r_batch;

    // generate the composition polynomial
    let composer = GkrComposition::new(r_batch);

    // run the sum-check protocol
    let main_prover = SumCheckProver::new(composer, SimpleGkrFinalClaimBuilder(PhantomData));
    let proof = main_prover.prove(claim_, ml_polys, transcript)?;

    Ok((proof, r_batch))
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
