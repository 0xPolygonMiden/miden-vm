use core::ops::Add;

use alloc::vec::Vec;
use vm_core::FieldElement;
use winter_air::{GkrRandElements, GkrVerifier, LagrangeKernelRandElements};
use winter_prover::{
    crypto::{ElementHasher, RandomCoin},
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable,
};

use crate::{gkr_verifier::VerifierError, trace::TRACE_WIDTH, verify_virtual_bus};

mod composition_polynomials;
pub use composition_polynomials::{
    evaluate_fractions_at_main_trace_query, CompositionPolynomial, GkrComposition,
    GkrCompositionMerge, NUM_WIRES_PER_TRACE_ROW,
};

mod multilinear;
pub use multilinear::{EqFunction, MultiLinearPoly, MultiLinearPolyError};

pub mod sumcheck;
use sumcheck::{FinalOpeningClaim, RoundProof, SumCheckProof};

// GKR CIRCUIT VERIFIER
// ===============================================================================================

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

// GKR CIRCUIT PROOF
// ===============================================================================================

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

// CIRCUIT LAYER POLYS
// ===============================================================================================

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

// CIRCUIT LAYER
// ===============================================================================================

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

// CIRCUIT WIRE
// ===============================================================================================

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
