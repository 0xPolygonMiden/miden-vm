use self::error::Error;
use super::{domain::EvaluationDomain, FinalOpeningClaim, Proof};
use crate::trace::virtual_bus::multilinear::CompositionPolynomial;
use alloc::vec::Vec;
use core::marker::PhantomData;
use vm_core::FieldElement;
use winter_prover::crypto::{ElementHasher, RandomCoin};

mod error;

/// A struct that contains relevant information for the execution of the multivariate sum-check
/// protocol verifier. The protocol is described in [`SumCheckProver`].
/// The sum-check Verifier is composed of two parts:
///
/// 1. Multi-round interaction where it sends challenges and receives polynomials. For each
/// polynomial received it uses the sent randomness to reduce the current claim to a new one.
///
/// 2. A final round where the Verifier queries the multi-linear oracles it received at the outset
/// of the protocol (i.e., commitments) for their evaluations the random point
/// `(r_0, ... , r_{\nu - 1})` where $\nu$ is the number of rounds of the sum-check protocol and
/// `r_i` is the randomness sent by the Verifier at each round.
pub struct SumCheckVerifier<E, P, C, H, V>
where
    E: FieldElement,
    C: RandomCoin<Hasher = H, BaseField = E::BaseField>,
    P: CompositionPolynomial<E>,
    H: ElementHasher<BaseField = E::BaseField>,
    V: CompositionPolyQueryBuilder<E>,
{
    composition_poly: P,
    eval_domain: EvaluationDomain<E>,
    final_query_builder: V,
    _challenger: PhantomData<C>,
}

impl<E, P, C, H, V> SumCheckVerifier<E, P, C, H, V>
where
    E: FieldElement,
    C: RandomCoin<Hasher = H, BaseField = E::BaseField>,
    P: CompositionPolynomial<E>,
    H: ElementHasher<BaseField = E::BaseField>,
    V: CompositionPolyQueryBuilder<E>,
{
    /// Create a new [SumCheckVerifier] from a composition polynomial and final query builder.
    pub fn new(composition_poly: P, final_query_builder: V) -> Self {
        let max_degree = composition_poly.max_degree();
        let eval_domain = EvaluationDomain::new(max_degree);

        Self {
            composition_poly,
            eval_domain,
            final_query_builder,
            _challenger: PhantomData,
        }
    }

    /// Verifies a sum-check proof [Proof] and returns a claim on the openings of the mult-linear
    /// oracles that are part of the statement being proven.
    ///
    /// More precisely, the method:
    ///
    /// 1. Generates a `claimed_evaluation` from the round proof polynomials and the round challenge
    /// randomness.
    ///
    /// 2. Computes a query that is built using the [FinalQueryBuilder] from the multi-linear
    /// openings and the round challenges.
    ///
    /// 3. Evaluates the composition polynomial at the query and checks that it is equal
    /// `claimed_evaluation`.
    ///
    /// 4. Outputs a `FinalOpeningClaim` on the multi-linear oracles.
    ///
    /// Thus, the proof is correct if the method outputs a [FinalOpeningClaim] and this latter is
    /// a valid claim on the multi-linear oracles i.e., each multi-linear oracle opens to the
    /// claimed value at the specified opening point.
    ///
    ///  # Errors
    /// Returns an error if:
    /// - No openings were provided.
    /// - Draw the round challenge fails.
    /// - The final evaluation check fails.
    pub fn verify(
        &self,
        claim: E,
        proof: Proof<E>,
        coin: &mut C,
    ) -> Result<FinalOpeningClaim<E>, Error> {
        let Proof {
            openings: openings_claim,
            round_proofs,
        } = proof;

        let mut round_claim = claim;
        let mut evaluation_point = vec![];
        for round_proof in round_proofs {
            let partial_evals = round_proof.partial_poly_evals.clone();
            coin.reseed(H::hash_elements(&partial_evals));
            let evals = round_proof.to_evals(round_claim);

            let r = coin.draw().map_err(|_| Error::FailedToGenerateChallenge)?;

            round_claim = self.eval_domain.evaluate(&evals, r);
            evaluation_point.push(r);
        }

        if let Some(openings_claim) = openings_claim {
            if openings_claim.evaluation_point != evaluation_point {
                return Err(Error::WrongOpeningPoint);
            }
            let query = self.final_query_builder.build_query(&openings_claim, &evaluation_point);
            if self.composition_poly.evaluate(&query) != round_claim {
                Err(Error::FinalEvaluationCheckFailed)
            } else {
                Ok(openings_claim)
            }
        } else {
            Err(Error::NoOpeningsProvided)
        }
    }
}

/// Contains the logic for building the final query made to the virtual polynomial.
///
/// During the last step of the sum-check protocol, the Verifier must evaluate the composed
/// multilinear polynomials at a random point `(r_0, ... ,r_{\nu - 1})`. To do this, the Verifier
/// asks the Prover for the openings of the mult-linear oracles at `(r_0, ... ,r_{\nu - 1})` i.e.,
/// `v_i = f_i(r_0, ... ,r_{\nu - 1})`. The Verifier then evaluates `g(v_0, ... , v_{\nu - 1})` and
/// compares it to the reduced claim resulting from the round proofs and challenges.
/// At this point, for the Verifier to accept the proof, it needs to check that indeed
/// `v_i = f_i(r_0, ... ,r_{\nu - 1})`, this is the exact content of [`FinalOpeningClaim`], which
/// can be either answered by a direct query to the oracles (i.e., in the compiled protocol this
/// would be answered with an opening proof against the commitment) or through further interaction
/// (as in the case of the GKR protocol).
///
/// The purpose of [`CompositionPolyQueryBuilder`] is to abstract the logic for evaluating the
/// multi-linear polynomials that the Verifier can do by herself. For example, this is the case
/// for periodic columns where given `(r_0, ... ,r_{\nu - 1})` the Verifier can evaluate
/// it at `(r_0, ... ,r_{\nu - 1})` unassisted.
pub trait CompositionPolyQueryBuilder<E: FieldElement> {
    fn build_query(&self, openings_claim: &FinalOpeningClaim<E>, evaluation_point: &[E]) -> Vec<E>;
}
