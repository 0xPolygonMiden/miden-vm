use core::ops::Add;

use alloc::vec::Vec;
use static_assertions::const_assert;
use vm_core::FieldElement;
use winter_air::{GkrRandElements, GkrVerifier, LagrangeKernelRandElements};
use winter_prover::{
    crypto::{ElementHasher, RandomCoin},
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable,
};

use crate::{
    gkr_verifier::VerifierError,
    trace::{
        chiplets::{MEMORY_D0_COL_IDX, MEMORY_D1_COL_IDX},
        decoder::{DECODER_OP_BITS_OFFSET, DECODER_USER_OP_HELPERS_OFFSET},
        range::{M_COL_IDX, V_COL_IDX},
        CHIPLETS_OFFSET, TRACE_WIDTH,
    },
    verify_virtual_bus,
};

mod multilinear;
pub use multilinear::{EqFunction, MultiLinearPoly, MultiLinearPolyError};

pub mod sumcheck;
use sumcheck::{FinalOpeningClaim, RoundProof, SumCheckProof};

#[derive(Debug, Default)]
pub struct GkrCircuitVerifier {}

impl GkrCircuitVerifier {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GkrVerifier for GkrCircuitVerifier {
    type GkrProof<E: FieldElement> = GkrCircuitProof<E>;
    type Error = VerifierError;

    fn verify<E, Hasher>(
        &self,
        gkr_proof: &GkrCircuitProof<E>,
        public_coin: &mut impl RandomCoin<BaseField = E::BaseField, Hasher = Hasher>,
    ) -> Result<GkrRandElements<E>, Self::Error>
    where
        E: FieldElement,
        Hasher: ElementHasher<BaseField = E::BaseField>,
    {
        let log_up_randomness: E = public_coin.draw().expect("failed to draw logup randomness");
        let final_opening_claim =
            verify_virtual_bus(E::ZERO, gkr_proof, vec![log_up_randomness], public_coin)?;

        // draw openings combining randomness
        let openings_combining_randomness: Vec<E> = {
            let openings_digest = Hasher::hash_elements(&final_opening_claim.openings);

            public_coin.reseed(openings_digest);

            (0..TRACE_WIDTH)
                .map(|_| public_coin.draw().expect("failed to draw openings combining randomness"))
                .collect()
        };

        let gkr_rand_elements = GkrRandElements::new(
            LagrangeKernelRandElements::new(final_opening_claim.eval_point),
            openings_combining_randomness,
        );

        Ok(gkr_rand_elements)
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
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.circuit_outputs.write_into(target);
        self.before_final_layer_proofs.write_into(target);
        self.final_layer_proof.write_into(target);
    }
}

impl<E> Deserializable for GkrCircuitProof<E>
where
    E: FieldElement,
{
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        Ok(Self {
            circuit_outputs: CircuitLayerPolys::read_from(source)?,
            before_final_layer_proofs: BeforeFinalLayerProof::read_from(source)?,
            final_layer_proof: FinalLayerProof::read_from(source)?,
        })
    }
}

/// A set of sum-check proofs for all GKR layers but for the input circuit layer.
#[derive(Debug)]
pub struct BeforeFinalLayerProof<E: FieldElement> {
    pub proof: Vec<SumCheckProof<E>>,
}

impl<E> Serializable for BeforeFinalLayerProof<E>
where
    E: FieldElement,
{
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self { proof } = self;
        proof.write_into(target);
    }
}

impl<E> Deserializable for BeforeFinalLayerProof<E>
where
    E: FieldElement,
{
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        Ok(Self {
            proof: Deserializable::read_from(source)?,
        })
    }
}

/// A proof for the input circuit layer i.e., the final layer in the GKR protocol.
#[derive(Debug)]
pub struct FinalLayerProof<E: FieldElement> {
    pub before_merge_proof: Vec<RoundProof<E>>,
    pub after_merge_proof: SumCheckProof<E>,
}

impl<E> Serializable for FinalLayerProof<E>
where
    E: FieldElement,
{
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self {
            before_merge_proof,
            after_merge_proof,
        } = self;
        before_merge_proof.write_into(target);
        after_merge_proof.write_into(target);
    }
}

impl<E> Deserializable for FinalLayerProof<E>
where
    E: FieldElement,
{
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        Ok(Self {
            before_merge_proof: Deserializable::read_from(source)?,
            after_merge_proof: Deserializable::read_from(source)?,
        })
    }
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

impl<E> Serializable for CircuitLayerPolys<E>
where
    E: FieldElement,
{
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self {
            numerators,
            denominators,
        } = self;
        numerators.write_into(target);
        denominators.write_into(target);
    }
}

impl<E> Deserializable for CircuitLayerPolys<E>
where
    E: FieldElement,
{
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        Ok(Self {
            numerators: MultiLinearPoly::read_from(source)?,
            denominators: MultiLinearPoly::read_from(source)?,
        })
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
    E: FieldElement,
{
    pub combining_randomness: E,
}

impl<E> GkrComposition<E>
where
    E: FieldElement,
{
    pub fn new(combining_randomness: E) -> Self {
        Self {
            combining_randomness,
        }
    }
}

impl<E> CompositionPolynomial<E> for GkrComposition<E>
where
    E: FieldElement,
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
    E: FieldElement,
{
    pub sum_check_combining_randomness: E,
    pub tensored_merge_randomness: Vec<E>,
    pub log_up_randomness: Vec<E>,
}

impl<E> GkrCompositionMerge<E>
where
    E: FieldElement,
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
    E: FieldElement,
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

// HELPER
// ================================================================================================

/// Computes the evaluations of the Lagrange basis polynomials over the interpolating
/// set {0 , 1}^ν at (r_0, ..., r_{ν - 1}) i.e., the Lagrange kernel at (r_0, ..., r_{ν - 1}).
pub(crate) fn compute_lagrange_basis_evals_at<E: FieldElement>(query: &[E]) -> Vec<E> {
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
