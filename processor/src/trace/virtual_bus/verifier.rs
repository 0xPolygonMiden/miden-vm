use super::{
    circuit::GkrCircuitProof,
    error::{Error, VerifierError},
    sum_check::FinalOpeningClaim,
    verify,
};
use alloc::vec::Vec;
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
    log_up_randomness: Vec<E>,
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
        Ok(Self {
            log_up_randomness,
            _challenger: PhantomData,
        })
    }

    /// Returns the claim of the GKR-LogUp relation.
    pub fn claim(&self) -> E {
        E::ZERO
    }

    /// Verifies the GKR-LogUp relation. This output, in the case the proof is accepted,
    /// a [FinalOpeningClaim] which is passed on to the STARK verifier in order to check
    /// the correctness of the claimed openings.
    pub fn verify(
        &self,
        proof: GkrCircuitProof<E>,
        transcript: &mut C,
    ) -> Result<FinalOpeningClaim<E>, VerifierError> {
        verify(self.claim(), proof, self.log_up_randomness.clone(), transcript)
            .map_err(|_| VerifierError::FailedToVerifyProof)
    }
}
