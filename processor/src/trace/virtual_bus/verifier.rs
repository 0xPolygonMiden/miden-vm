use super::{
    circuit::GkrCircuitProof,
    error::{Error, VerifierError},
    generate,
    multilinear::CompositionPolynomial,
    sum_check::FinalOpeningClaim,
    verify,
};
use alloc::{sync::Arc, vec::Vec};
use core::marker::PhantomData;
use vm_core::{Felt, FieldElement};
use winter_prover::crypto::{ElementHasher, RandomCoin};

/// A struct which implements the logic for verification of the correctness of the global virtual
/// bus relation.
pub struct VirtualBusVerifier<E, C, H>
where
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
{
    claim: E,
    composition_polynomials: Vec<Vec<Arc<dyn CompositionPolynomial<E>>>>,
    _challenger: PhantomData<C>,
}

impl<E, C, H> VirtualBusVerifier<E, C, H>
where
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
{
    /// Constructs a new [`VirtualBusVerifier`] given a set of random values for the GKR-LogUp relation.
    pub fn new(log_up_randomness: Vec<E>) -> Result<Self, Error> {
        let (claim, composition_polynomials) = generate(log_up_randomness)?;

        Ok(Self {
            claim,
            composition_polynomials,
            _challenger: PhantomData,
        })
    }

    /// Returns the claim of the GKR-LogUp relation.
    pub fn claim(&self) -> Result<E, Error> {
        Ok(self.claim)
    }

    /// Returns the composition polynomials of the left/right numerators/denominators of
    /// the GKR-LogUp relation.
    pub fn composition_polynomials(&self) -> Vec<Vec<Arc<dyn CompositionPolynomial<E>>>> {
        self.composition_polynomials.clone()
    }

    /// Verifies the GKR-LogUp relation. This output, in the case the proof is accepted,
    /// a [FinalOpeningClaim] which is passed on to the STARK verifier in order to check
    /// the correctness of the claimed openings.
    pub fn verify(
        &self,
        proof: GkrCircuitProof<E>,
        transcript: &mut C,
    ) -> Result<FinalOpeningClaim<E>, VerifierError> {
        verify(self.claim, proof, self.composition_polynomials(), transcript)
            .map_err(|_| VerifierError::FailedToVerifyProof)
    }
}
