// TODOP: Cleanup this file
use core::ops::{Add, Index};

use alloc::vec::Vec;
use static_assertions::const_assert;
use vm_core::{polynom, Felt, FieldElement};
use winter_air::{GkrRandElements, GkrVerifier};
use winter_prover::{
    crypto::{ElementHasher, RandomCoin},
    Deserializable, Serializable,
};

use crate::{
    decoder::{DECODER_OP_BITS_OFFSET, DECODER_USER_OP_HELPERS_OFFSET},
    trace::{
        chiplets::{MEMORY_D0_COL_IDX, MEMORY_D1_COL_IDX},
        range::{M_COL_IDX, V_COL_IDX},
    },
    CHIPLETS_OFFSET, TRACE_WIDTH,
};

pub struct GkrCircuitVerifier {}

impl GkrVerifier for GkrCircuitVerifier {
    type GkrProof<E: FieldElement> = GkrCircuitProof<E>;

    // TODOP: Use proper Error type
    type Error = alloc::string::String;

    fn verify<E, Hasher>(
        &self,
        _gkr_proof: Self::GkrProof<E>,
        _public_coin: &mut impl RandomCoin<BaseField = E::BaseField, Hasher = Hasher>,
    ) -> Result<GkrRandElements<E>, Self::Error>
    where
        E: FieldElement,
        Hasher: ElementHasher<BaseField = E::BaseField>,
    {
        todo!()
    }
}

/// A GKR proof for the correct evaluation of the sum of fractions circuit.
#[derive(Debug)]
pub struct GkrCircuitProof<E: FieldElement> {
    pub circuit_outputs: CircuitLayerPolys<E>,
    pub before_final_layer_proofs: BeforeFinalLayerProof<E>,
    pub final_layer_proof: FinalLayerProof<E>,
}

impl<E: FieldElement> GkrCircuitProof<E> {
    pub fn get_final_opening_claim(&self) -> FinalOpeningClaim<E> {
        self.final_layer_proof.after_merge_proof.openings_claim.clone()
    }
}

impl<E> Serializable for GkrCircuitProof<E>
where
    E: FieldElement,
{
    fn write_into<W: winter_prover::ByteWriter>(&self, _target: &mut W) {
        todo!()
    }
}

impl<E> Deserializable for GkrCircuitProof<E>
where
    E: FieldElement,
{
    fn read_from<R: winter_prover::ByteReader>(
        _source: &mut R,
    ) -> Result<Self, winter_prover::DeserializationError> {
        todo!()
    }
}

/// A set of sum-check proofs for all GKR layers but for the input circuit layer.
#[derive(Debug)]
pub struct BeforeFinalLayerProof<E: FieldElement> {
    pub proof: Vec<SumCheckProof<E>>,
}

/// A proof for the input circuit layer i.e., the final layer in the GKR protocol.
#[derive(Debug)]
pub struct FinalLayerProof<E: FieldElement> {
    pub before_merge_proof: Vec<RoundProof<E>>,
    pub after_merge_proof: SumCheckProof<E>,
}

/// Holds a layer of an [`EvaluatedCircuit`] in a representation amenable to proving circuit
/// evaluation using GKR.
#[derive(Clone, Debug)]
pub struct CircuitLayerPolys<E: FieldElement> {
    pub numerators: MultiLinearPoly<E>,
    pub denominators: MultiLinearPoly<E>,
}

impl<E> CircuitLayerPolys<E>
where
    E: FieldElement,
{
    pub fn from_circuit_layer(layer: CircuitLayer<E>) -> Self {
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

/// Represents a layer in a [`EvaluatedCircuit`].
///
/// A layer is made up of a set of `n` wires, where `n` is a power of two. This is the natural
/// circuit representation of a layer, where each consecutive pair of wires are summed to yield a
/// wire in the subsequent layer of an [`EvaluatedCircuit`].
///
/// Note that a [`Layer`] needs to be first converted to a [`LayerPolys`] before the evaluation of
/// the layer can be proved using GKR.
pub struct CircuitLayer<E: FieldElement> {
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

/// Represents a fraction `numerator / denominator` as a pair `(numerator, denominator)`. This is
/// the type for the gates' inputs in [`prover::EvaluatedCircuit`].
///
/// Hence, addition is defined in the natural way fractions are added together: `a/b + c/d = (ad +
/// bc) / bd`.
#[derive(Debug, Clone, Copy)]
pub struct CircuitWire<E: FieldElement> {
    numerator: E,
    denominator: E,
}

impl<E> CircuitWire<E>
where
    E: FieldElement,
{
    /// Creates new projective coordinates from a numerator and a denominator.
    pub fn new(numerator: E, denominator: E) -> Self {
        assert_ne!(denominator, E::ZERO);

        Self {
            numerator,
            denominator,
        }
    }
}

impl<E> Add for CircuitWire<E>
where
    E: FieldElement,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        let denominator = self.denominator * other.denominator;

        Self::new(numerator, denominator)
    }
}

/// Represents an opening claim at an evaluation point against a batch of oracles.
///
/// After verifying [`Proof`], the verifier is left with a question on the validity of a final
/// claim on a number of oracles open to a given set of values at some given point.
/// This question is answered either using further interaction with the Prover or using
/// a polynomial commitment opening proof in the compiled protocol.
#[derive(Clone, Debug)]
pub struct FinalOpeningClaim<E> {
    pub eval_point: Vec<E>,
    pub openings: Vec<E>,
}

/// A sum-check round proof.
///
/// This represents the partial polynomial sent by the Prover during one of the rounds of the
/// sum-check protocol. The polynomial is in coefficient form and excludes the coefficient for
/// the linear term as the Verifier can recover it from the other coefficients and the current
/// (reduced) claim.
#[derive(Debug, Clone)]
pub struct RoundProof<E: FieldElement> {
    pub round_poly_coefs: UnivariatePolyCoef<E>,
}

/// A sum-check proof.
///
/// Composed of the round proofs i.e., the polynomials sent by the Prover at each round as well as
/// the (claimed) openings of the multi-linear oracles at the evaluation point given by the round
/// challenges.
#[derive(Debug, Clone)]
pub struct SumCheckProof<E: FieldElement> {
    pub openings_claim: FinalOpeningClaim<E>,
    pub round_proofs: Vec<RoundProof<E>>,
}

// MULTI-LINEAR POLYNOMIAL
// ================================================================================================

/// Represents a multi-linear polynomial.
///
/// The representation stores the evaluations of the polynomial over the boolean hyper-cube
/// {0 , 1}^ν.
#[derive(Clone, Debug)]
pub struct MultiLinearPoly<E: FieldElement> {
    num_variables: usize,
    evaluations: Vec<E>,
}

impl<E: FieldElement> MultiLinearPoly<E> {
    /// Constructs a [`MultiLinearPoly`] from its evaluations over the boolean hyper-cube {0 , 1}^ν.
    pub fn from_evaluations(evaluations: Vec<E>) -> Result<Self, MultiLinearPolyError> {
        if !evaluations.len().is_power_of_two() {
            return Err(MultiLinearPolyError::EvaluationsNotPowerOfTwo);
        }
        Ok(Self {
            num_variables: (evaluations.len().ilog2()) as usize,
            evaluations,
        })
    }

    /// Returns the number of variables of the multi-linear polynomial.
    pub fn num_variables(&self) -> usize {
        self.num_variables
    }

    /// Returns the evaluations over the boolean hyper-cube.
    pub fn evaluations(&self) -> &[E] {
        &self.evaluations
    }

    /// Returns the number of evaluations. This is equal to the size of the boolean hyper-cube.
    pub fn num_evaluations(&self) -> usize {
        self.evaluations.len()
    }

    /// Evaluate the multi-linear at some query (r_0, ..., r_{ν - 1}) ∈ 𝔽^ν.
    ///
    /// It first computes the evaluations of the Lagrange basis polynomials over the interpolating
    /// set {0 , 1}^ν at (r_0, ..., r_{ν - 1}) i.e., the Lagrange kernel at (r_0, ..., r_{ν - 1}).
    /// The evaluation then is the inner product, indexed by {0 , 1}^ν, of the vector of
    /// evaluations times the Lagrange kernel.
    pub fn evaluate(&self, query: &[E]) -> E {
        let tensored_query = compute_lagrange_basis_evals_at(query);
        inner_product(&self.evaluations, &tensored_query)
    }

    /// Similar to [`Self::evaluate`], except that the query was already turned into the Lagrange
    /// kernel (i.e. the [`lagrange_ker::EqFunction`] evaluated at every point in the set
    /// `{0 , 1}^ν`).
    ///
    /// This is more efficient than [`Self::evaluate`] when multiple different [`MultiLinearPoly`]
    /// need to be evaluated at the same query point.
    pub fn evaluate_with_lagrange_kernel(&self, lagrange_kernel: &[E]) -> E {
        inner_product(&self.evaluations, lagrange_kernel)
    }

    /// Computes f(r_0, y_1, ..., y_{ν - 1}) using the linear interpolation formula
    /// (1 - r_0) * f(0, y_1, ..., y_{ν - 1}) + r_0 * f(1, y_1, ..., y_{ν - 1}) and assigns
    /// the resulting multi-linear, defined over a domain of half the size, to `self`.
    pub fn bind_least_significant_variable(&mut self, round_challenge: E) {
        let mut result = vec![E::ZERO; 1 << (self.num_variables() - 1)];
        for (i, res) in result.iter_mut().enumerate() {
            *res = self.evaluations[i << 1]
                + round_challenge * (self.evaluations[(i << 1) + 1] - self.evaluations[i << 1]);
        }
        *self = Self::from_evaluations(result)
            .expect("should not fail given that it is a multi-linear");
    }

    /// Given the multilinear polynomial f(y_0, y_1, ..., y_{ν - 1}), returns two polynomials:
    /// f(0, y_1, ..., y_{ν - 1}) and f(1, y_1, ..., y_{ν - 1}).
    pub fn project_least_significant_variable(&self) -> (Self, Self) {
        let mut p0 = Vec::with_capacity(self.num_evaluations() / 2);
        let mut p1 = Vec::with_capacity(self.num_evaluations() / 2);
        for chunk in self.evaluations.chunks_exact(2) {
            p0.push(chunk[0]);
            p1.push(chunk[1]);
        }

        (
            MultiLinearPoly::from_evaluations(p0).unwrap(),
            MultiLinearPoly::from_evaluations(p1).unwrap(),
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MultiLinearPolyError {
    #[error("A multi-linear polynomial should have a power of 2 number of evaluations over the Boolean hyper-cube")]
    EvaluationsNotPowerOfTwo,
}

impl<E: FieldElement> Index<usize> for MultiLinearPoly<E> {
    type Output = E;

    fn index(&self, index: usize) -> &E {
        &(self.evaluations[index])
    }
}

/// The EQ (equality) function is the binary function defined by
///
/// EQ:    {0 , 1}^ν ⛌ {0 , 1}^ν ⇾ {0 , 1}
///   ((x_0, ..., x_{ν - 1}), (y_0, ..., y_{ν - 1})) ↦ \prod_{i = 0}^{ν - 1} (x_i * y_i + (1 - x_i)
/// * (1 - y_i))
///
/// Taking It's multi-linear extension EQ^{~}, we can define a basis for the set of multi-linear
/// polynomials in ν variables by
///         {EQ^{~}(., (y_0, ..., y_{ν - 1})): (y_0, ..., y_{ν - 1}) ∈ {0 , 1}^ν}
/// where each basis function is a function of its first argument. This is called the Lagrange or
/// evaluation basis with evaluation set {0 , 1}^ν.
///
/// Given a function (f: {0 , 1}^ν ⇾ 𝔽), its multi-linear extension (i.e., the unique
/// mult-linear polynomial extending f to (f^{~}: 𝔽^ν ⇾ 𝔽) and agrees with it on {0 , 1}^ν) is
/// defined as the summation of the evaluations of f against the Lagrange basis.
/// More specifically, given (r_0, ..., r_{ν - 1}) ∈ 𝔽^ν, then:
///
///     f^{~}(r_0, ..., r_{ν - 1}) = \sum_{(y_0, ..., y_{ν - 1}) ∈ {0 , 1}^ν}
///                  f(y_0, ..., y_{ν - 1}) EQ^{~}((r_0, ..., r_{ν - 1}), (y_0, ..., y_{ν - 1}))
///
/// We call the Lagrange kernel the evaluation of the EQ^{~} function at
/// ((r_0, ..., r_{ν - 1}), (y_0, ..., y_{ν - 1})) for all (y_0, ..., y_{ν - 1}) ∈ {0 , 1}^ν for
/// a fixed (r_0, ..., r_{ν - 1}) ∈ 𝔽^ν.
///
/// [`EqFunction`] represents EQ^{~} the mult-linear extension of
///
/// ((y_0, ..., y_{ν - 1}) ↦ EQ((r_0, ..., r_{ν - 1}), (y_0, ..., y_{ν - 1})))
///
/// and contains a method to generate the Lagrange kernel for defining evaluations of multi-linear
/// extensions of arbitrary functions (f: {0 , 1}^ν ⇾ 𝔽) at a given point (r_0, ..., r_{ν - 1})
/// as well as a method to evaluate EQ^{~}((r_0, ..., r_{ν - 1}), (t_0, ..., t_{ν - 1}))) for
/// (t_0, ..., t_{ν - 1}) ∈ 𝔽^ν.
pub struct EqFunction<E> {
    r: Vec<E>,
}

impl<E: FieldElement> EqFunction<E> {
    /// Creates a new [EqFunction].
    pub fn new(r: Vec<E>) -> Self {
        let tmp = r.clone();
        EqFunction { r: tmp }
    }

    /// Computes EQ((r_0, ..., r_{ν - 1}), (t_0, ..., t_{ν - 1}))).
    pub fn evaluate(&self, t: &[E]) -> E {
        assert_eq!(self.r.len(), t.len());

        (0..self.r.len())
            .map(|i| self.r[i] * t[i] + (E::ONE - self.r[i]) * (E::ONE - t[i]))
            .fold(E::ONE, |acc, term| acc * term)
    }

    /// Computes EQ((r_0, ..., r_{ν - 1}), (y_0, ..., y_{ν - 1})) for all
    /// (y_0, ..., y_{ν - 1}) ∈ {0 , 1}^ν i.e., the Lagrange kernel at r = (r_0, ..., r_{ν - 1}).
    pub fn evaluations(&self) -> Vec<E> {
        compute_lagrange_basis_evals_at(&self.r)
    }

    /// Returns the evaluations of
    /// ((y_0, ..., y_{ν - 1}) ↦ EQ^{~}((r_0, ..., r_{ν - 1}), (y_0, ..., y_{ν - 1})))
    /// over {0 , 1}^ν.
    pub fn ml_at(evaluation_point: Vec<E>) -> MultiLinearPoly<E> {
        let eq_evals = EqFunction::new(evaluation_point.clone()).evaluations();
        MultiLinearPoly::from_evaluations(eq_evals)
            .expect("should not fail because evaluations is a power of two")
    }
}

/// The coefficients of a univariate polynomial of degree n with the linear term coefficient
/// omitted.
#[derive(Clone, Debug)]
pub struct UnivariatePolyCoef<E: FieldElement> {
    pub coefficients: Vec<E>,
}

impl<E: FieldElement> UnivariatePolyCoef<E> {
    /// Evaluates a polynomial at a challenge point using a round claim.
    ///
    /// The round claim is used to recover the coefficient of the linear term using the relation
    /// 2 * c0 + c1 + ... c_{n - 1} = claim. Using the complete list of coefficients, the polynomial
    /// is then evaluated using Horner's method.
    pub fn evaluate_using_claim(&self, claim: &E, challenge: &E) -> E {
        // recover the coefficient of the linear term
        let c1 = *claim
            - self.coefficients.iter().fold(E::ZERO, |acc, term| acc + *term)
            - self.coefficients[0];

        // construct the full coefficient list
        let mut complete_coefficients = vec![self.coefficients[0], c1];
        complete_coefficients.extend_from_slice(&self.coefficients[1..]);

        // evaluate
        polynom::eval(&complete_coefficients, *challenge)
    }
}

/// A multi-variate polynomial for composing individual multi-linear polynomials.
pub trait CompositionPolynomial<E: FieldElement> {
    /// Maximum degree in all variables.
    fn max_degree(&self) -> u32;

    /// Given a query, of length equal the number of variables, evaluates [Self] at this query.
    fn evaluate(&self, query: &[E]) -> E;
}

/// A composition polynomial used in the GKR protocol for all of its sum-checks except the final
/// one.
#[derive(Clone)]
pub struct GkrComposition<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub combining_randomness: E,
}

impl<E> GkrComposition<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub fn new(combining_randomness: E) -> Self {
        Self {
            combining_randomness,
        }
    }
}

impl<E> CompositionPolynomial<E> for GkrComposition<E>
where
    E: FieldElement<BaseField = Felt>,
{
    fn max_degree(&self) -> u32 {
        3
    }

    fn evaluate(&self, query: &[E]) -> E {
        let eval_left_numerator = query[0];
        let eval_right_numerator = query[1];
        let eval_left_denominator = query[2];
        let eval_right_denominator = query[3];
        let eq_eval = query[4];
        eq_eval
            * ((eval_left_numerator * eval_right_denominator
                + eval_right_numerator * eval_left_denominator)
                + eval_left_denominator * eval_right_denominator * self.combining_randomness)
    }
}

/// A composition polynomial used in the GKR protocol for its final sum-check.
#[derive(Clone)]
pub struct GkrCompositionMerge<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub sum_check_combining_randomness: E,
    pub tensored_merge_randomness: Vec<E>,
    pub log_up_randomness: Vec<E>,
}

impl<E> GkrCompositionMerge<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub fn new(
        combining_randomness: E,
        merge_randomness: Vec<E>,
        log_up_randomness: Vec<E>,
    ) -> Self {
        let tensored_merge_randomness =
            EqFunction::ml_at(merge_randomness.clone()).evaluations().to_vec();

        Self {
            sum_check_combining_randomness: combining_randomness,
            tensored_merge_randomness,
            log_up_randomness,
        }
    }
}

impl<E> CompositionPolynomial<E> for GkrCompositionMerge<E>
where
    E: FieldElement<BaseField = Felt>,
{
    fn max_degree(&self) -> u32 {
        // Computed as:
        // 1 + max(left_numerator_degree + right_denom_degree, right_numerator_degree +
        // left_denom_degree)
        5
    }

    fn evaluate(&self, query: &[E]) -> E {
        let [numerators, denominators] =
            evaluate_fractions_at_main_trace_query(query, &self.log_up_randomness);

        let numerators = MultiLinearPoly::from_evaluations(numerators.to_vec()).unwrap();
        let denominators = MultiLinearPoly::from_evaluations(denominators.to_vec()).unwrap();

        let (left_numerators, right_numerators) = numerators.project_least_significant_variable();
        let (left_denominators, right_denominators) =
            denominators.project_least_significant_variable();

        let eval_left_numerators =
            left_numerators.evaluate_with_lagrange_kernel(&self.tensored_merge_randomness);
        let eval_right_numerators =
            right_numerators.evaluate_with_lagrange_kernel(&self.tensored_merge_randomness);

        let eval_left_denominators =
            left_denominators.evaluate_with_lagrange_kernel(&self.tensored_merge_randomness);
        let eval_right_denominators =
            right_denominators.evaluate_with_lagrange_kernel(&self.tensored_merge_randomness);

        let eq_eval = query[TRACE_WIDTH];

        eq_eval
            * ((eval_left_numerators * eval_right_denominators
                + eval_right_numerators * eval_left_denominators)
                + eval_left_denominators
                    * eval_right_denominators
                    * self.sum_check_combining_randomness)
    }
}

/// Defines the number of wires in the input layer that are generated from a single main trace row.
pub const NUM_WIRES_PER_TRACE_ROW: usize = 8;
const_assert!(NUM_WIRES_PER_TRACE_ROW.is_power_of_two());

/// Converts a main trace row (or more generally "query") to numerators and denominators of the
/// input layer.
pub fn evaluate_fractions_at_main_trace_query<E>(
    query: &[E],
    log_up_randomness: &[E],
) -> [[E; NUM_WIRES_PER_TRACE_ROW]; 2]
where
    E: FieldElement,
{
    // numerators
    let multiplicity = query[M_COL_IDX];
    let f_m = {
        let mem_selec0 = query[CHIPLETS_OFFSET];
        let mem_selec1 = query[CHIPLETS_OFFSET + 1];
        let mem_selec2 = query[CHIPLETS_OFFSET + 2];
        mem_selec0 * mem_selec1 * (E::ONE - mem_selec2)
    };

    let f_rc = {
        let op_bit_4 = query[DECODER_OP_BITS_OFFSET + 4];
        let op_bit_5 = query[DECODER_OP_BITS_OFFSET + 5];
        let op_bit_6 = query[DECODER_OP_BITS_OFFSET + 6];

        (E::ONE - op_bit_4) * (E::ONE - op_bit_5) * op_bit_6
    };

    // denominators
    let alphas = log_up_randomness;

    let table_denom = alphas[0] - query[V_COL_IDX];
    let memory_denom_0 = -(alphas[0] - query[MEMORY_D0_COL_IDX]);
    let memory_denom_1 = -(alphas[0] - query[MEMORY_D1_COL_IDX]);
    let stack_value_denom_0 = -(alphas[0] - query[DECODER_USER_OP_HELPERS_OFFSET]);
    let stack_value_denom_1 = -(alphas[0] - query[DECODER_USER_OP_HELPERS_OFFSET + 1]);
    let stack_value_denom_2 = -(alphas[0] - query[DECODER_USER_OP_HELPERS_OFFSET + 2]);
    let stack_value_denom_3 = -(alphas[0] - query[DECODER_USER_OP_HELPERS_OFFSET + 3]);

    [
        [multiplicity, f_m, f_m, f_rc, f_rc, f_rc, f_rc, E::ZERO],
        [
            table_denom,
            memory_denom_0,
            memory_denom_1,
            stack_value_denom_0,
            stack_value_denom_1,
            stack_value_denom_2,
            stack_value_denom_3,
            E::ONE,
        ],
    ]
}

/// Contains the round challenges sent by the Verifier up to some round as well as the current
/// reduced claim.
#[derive(Debug)]
pub struct SumCheckRoundClaim<E: FieldElement> {
    pub eval_point: Vec<E>,
    pub claim: E,
}

// HELPER
// ================================================================================================

/// Computes the inner product of two vectors of the same length.
///
/// Panics if the vectors have different lengths.
pub fn inner_product<E: FieldElement>(evaluations: &[E], tensored_query: &[E]) -> E {
    assert_eq!(evaluations.len(), tensored_query.len());
    evaluations
        .iter()
        .zip(tensored_query.iter())
        .fold(E::ZERO, |acc, (x_i, y_i)| acc + *x_i * *y_i)
}

/// Computes the evaluations of the Lagrange basis polynomials over the interpolating
/// set {0 , 1}^ν at (r_0, ..., r_{ν - 1}) i.e., the Lagrange kernel at (r_0, ..., r_{ν - 1}).
pub fn compute_lagrange_basis_evals_at<E: FieldElement>(query: &[E]) -> Vec<E> {
    let nu = query.len();
    let n = 1 << nu;

    let mut evals: Vec<E> = vec![E::ONE; n];
    let mut size = 1;
    for r_i in query.iter().rev() {
        size *= 2;
        for i in (0..size).rev().step_by(2) {
            let scalar = evals[i / 2];
            evals[i] = scalar * *r_i;
            evals[i - 1] = scalar - evals[i];
        }
    }
    evals
}
