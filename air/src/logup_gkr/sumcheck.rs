use alloc::vec::Vec;
use vm_core::{polynom, FieldElement};
use winter_prover::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

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

impl<E: FieldElement> Serializable for UnivariatePolyCoef<E> {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self { coefficients } = self;
        coefficients.write_into(target);
    }
}

impl<E> Deserializable for UnivariatePolyCoef<E>
where
    E: FieldElement,
{
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        Ok(Self {
            coefficients: Deserializable::read_from(source)?,
        })
    }
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

impl<E: FieldElement> Serializable for RoundProof<E> {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self { round_poly_coefs } = self;
        round_poly_coefs.write_into(target);
    }
}

impl<E> Deserializable for RoundProof<E>
where
    E: FieldElement,
{
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        Ok(Self {
            round_poly_coefs: Deserializable::read_from(source)?,
        })
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

impl<E: FieldElement> Serializable for FinalOpeningClaim<E> {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self {
            eval_point,
            openings,
        } = self;
        eval_point.write_into(target);
        openings.write_into(target);
    }
}

impl<E> Deserializable for FinalOpeningClaim<E>
where
    E: FieldElement,
{
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        Ok(Self {
            eval_point: Deserializable::read_from(source)?,
            openings: Deserializable::read_from(source)?,
        })
    }
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

impl<E> Serializable for SumCheckProof<E>
where
    E: FieldElement,
{
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.openings_claim.write_into(target);
        self.round_proofs.write_into(target);
    }
}

impl<E> Deserializable for SumCheckProof<E>
where
    E: FieldElement,
{
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        Ok(Self {
            openings_claim: Deserializable::read_from(source)?,
            round_proofs: Deserializable::read_from(source)?,
        })
    }
}

/// Contains the round challenges sent by the Verifier up to some round as well as the current
/// reduced claim.
#[derive(Debug)]
pub struct SumCheckRoundClaim<E: FieldElement> {
    pub eval_point: Vec<E>,
    pub claim: E,
}
