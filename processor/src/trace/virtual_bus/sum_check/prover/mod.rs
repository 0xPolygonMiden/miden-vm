use self::error::Error;
use super::{domain::EvaluationDomain, reduce_claim, Proof, RoundClaim, RoundProof};
use crate::trace::virtual_bus::multilinear::{CompositionPolynomial, MultiLinear};
use core::marker::PhantomData;
use vm_core::{FieldElement, StarkField};
use winter_prover::crypto::{ElementHasher, RandomCoin};

mod error;

/// A struct that contains relevant information for the execution of the multivariate sum-check
/// protocol prover.
/// The sum-check protocol is an interactive protocol (IP) for proving the following relation:
///
/// v = \sum_{(x_0,\cdots, x_{\nu - 1}) \in \{0 , 1\}^{2^{\nu}}}
///                     g(f_0((x_0,\cdots, x_{\nu - 1})), \cdots , f_c((x_0,\cdots, x_{\nu - 1})))
///
/// where:
///
/// 1. v ∈ 𝔽 where 𝔽 is a finite field.
/// 2. f_i are multi-linear polynomials i.e., polynomials in 𝔽[X_i, \cdots ,X_{\nu - 1}] with degree
/// at most one in each variable.
/// 3. g is a multivariate polynomial with degree at most d in each variable.
///
/// The Verifier is given commitments to each `f_i` in addition to the claimed sum `v`. The Prover
/// then engages in an IP to convince the Verifier that the above relation holds for the given
/// `f_i` and `v`. More precisely:
///
/// 0. Denote by w(x_0,\cdots, x_{\nu - 1}) := g(f_0((x_0,\cdots, x_{\nu - 1})),
///                                                       \cdots , f_c((x_0,\cdots, x_{\nu - 1}))).
///
/// 1. In the first round, the Prover sends the polynomial defined by:
///         s_0(X_0) := \sum_{(x_{1},\cdots, x_{\nu - 1})  w(X_0, x_{1}, \cdots, x_{\nu - 1})
///
/// 2. The Verifier then checks that s_0(0) + s_0(1) = v rejecting if not.
///
/// 3. The Verifier samples a random challenge `r_0 ∈ 𝔽` and sends it to the Prover.
///
/// 4. For each i in 1...(\nu - 1):
///     a. The Prover sends the univariate polynomial defined by:
///
///         s_i(X_i) := \sum_{(x_{i + 1},\cdots, x_{\nu - 1})
///                                  w(r_0,\cdots, r_{i - 1}, X_i, x_{i + 1}, \cdots, x_{\nu - 1}).
///
///     b. The Verifier checks that s_{i - 1}(r_{i - 1}) = s_{i}(0) + s_{i}(1) rejecting if not.
///     
///     c. The Verifier samples a random challenge `r_i ∈ 𝔽` and sends it to the Prover.
///
/// 5. The Verifier now queries each of the oracles behind the commitments i.e., `f_i` at
/// `(r_0, \cdots , r_{\nu - 1})` to get u_i = f_i(r_0, \cdots , r_{\nu - 1}).
/// The Verifier then accepts if and only if:
///
///         s_{\nu - 1}(r_{\nu - 1}) = g(u_0, \cdots , u_{\nu - 1})
///
/// A few remarks:
///
/// 1. The degree bound on `g` implies that each of the `s_i` polynomials is a univariate polynomial
/// of degree at most `d`. Thus, the Prover in each round sends `d + 1` values, either
/// the coefficients or the evaluations of `s_i`.
///
/// 2. The Prover has each `f_i` in its evaluation form over the hyper-cube \{0 , 1\}^{2^{\nu}}.
///
/// 3. An optimization is for the Prover to not send `s_i(0)` as it can be recoverd from the current
/// reduced claim s_{i - 1}(r_{i - 1}) using the relation s_{i}(0) = s_{i}(1) - s_{i - 1}(r_{i - 1}).
/// This also means that the Verifier can skip point 4.b.
pub struct SumCheckProver<B, E, P, C, H>
where
    B: StarkField,
    E: FieldElement<BaseField = B>,
    C: RandomCoin<Hasher = H, BaseField = B>,
    H: ElementHasher<BaseField = B>,
{
    composition_poly: P,
    eval_domain: EvaluationDomain<E>,
    _challenger: PhantomData<C>,
    _hasher: PhantomData<H>,
}

impl<B, E, P, C, H> SumCheckProver<B, E, P, C, H>
where
    B: StarkField,
    E: FieldElement<BaseField = B>,
    P: CompositionPolynomial<E>,
    C: RandomCoin<Hasher = H, BaseField = B>,
    H: ElementHasher<BaseField = B>,
{
    /// Constructs a new [SumCheckProver] given a multivariate composition polynomial.
    /// The multivariate composition polynomial corresponds to the `g` polynomial in the
    /// description of the [SumCheckProver] struct.
    pub fn new(composition_poly: P) -> Self {
        let max_degree = composition_poly.max_degree();
        let eval_domain = EvaluationDomain::new(max_degree);

        Self {
            composition_poly,
            eval_domain,
            _challenger: PhantomData,
            _hasher: PhantomData,
        }
    }

    /// Given an initial claim `claim`, a mutable vector of multi-linear polynomials `mls` and
    /// a number of rounds `num_rounds`, computes `num_rounds` iterations of the sum-check protocol
    /// starting from claim `claim`.
    ///
    /// # Errors
    /// Returns an error if:
    /// - No multi-linears were provided.
    /// - Number of rounds is zero or is greater than the number of variables of the multilinears.
    /// - The provided multi-linears have different arities.
    pub fn prove(
        &self,
        claim: E,
        mls: &mut [MultiLinear<E>],
        num_rounds: usize,
        coin: &mut C,
        //) -> Result<(RoundClaim<E>, Proof<E>), Error> {
    ) -> Result<Proof<E>, Error> {
        // there should be at least one multi-linear polynomial provided
        if mls.is_empty() {
            return Err(Error::NoMlsProvided);
        }

        // there should be at least one round to prove
        if num_rounds == 0 {
            return Err(Error::NumRoundsZero);
        }

        // there can not be more rounds than variables of the multi-linears
        let ml_variables = mls[0].num_variables();
        if num_rounds > ml_variables {
            return Err(Error::TooManyRounds);
        }

        // there should at least be one variable for the protocol to be non-trivial
        if ml_variables < 2 {
            return Err(Error::AtLeastOneVariable);
        }

        // all multi-linears should have the same arity
        if !mls.iter().all(|ml| ml.num_variables() == ml_variables) {
            return Err(Error::MlesDifferentArities);
        }

        let mut round_proofs = vec![];

        // setup first round claim
        let mut current_round_claim = RoundClaim {
            eval_point: vec![],
            claim,
        };

        // run the first round of the protocol
        let mut round_proof = sumcheck_round(&self.composition_poly, mls);
        round_proofs.push(round_proof);
        // reseed with the s_0 polynomial
        coin.reseed(H::hash_elements(&round_proofs[0].poly_evals));

        for i in 1..num_rounds {
            // generate random challenge r_i for the i-th round
            let round_challenge = coin.draw().map_err(|_| Error::FailedToGenerateChallenge)?;

            // compute the new reduced round claim
            let new_round_claim = reduce_claim(
                &self.eval_domain,
                &round_proofs[i - 1],
                current_round_claim,
                round_challenge,
            );

            // fold each multi-linear using the round challenge
            mls.iter_mut().for_each(|ml| ml.bind_assign(round_challenge));

            // run the i-th round of the protocol using the folded multi-linears for the new reduced
            // claim. This basically computes the s_i polynomial.
            round_proof = sumcheck_round(&self.composition_poly, mls);
            round_proofs.push(round_proof);

            // update the claim
            current_round_claim = new_round_claim;

            // reseed with the s_i polynomial
            coin.reseed(H::hash_elements(&round_proofs[i].poly_evals));
        }

        // generate the last random challenge
        let round_challenge = coin.draw().map_err(|_| Error::FailedToGenerateChallenge)?;
        // fold each multi-linear using the last random challenge
        mls.iter_mut().for_each(|ml| ml.bind_assign(round_challenge));

        let openings = {
            if mls[0].num_evaluations() != 1 {
                None
            } else {
                Some(mls.iter_mut().map(|ml| ml.evaluations()[0]).collect())
            }
        };

        Ok(Proof {
            openings,
            round_proofs,
        })
    }
}

/// Computes the polynomial
///
/// s_i(X_i) := \sum_{(x_{i + 1},\cdots, x_{\nu - 1})
///                                  w(r_0,\cdots, r_{i - 1}, X_i, x_{i + 1}, \cdots, x_{\nu - 1}).
/// where
///
/// w(x_0,\cdots, x_{\nu - 1}) := g(f_0((x_0,\cdots, x_{\nu - 1})),
///                                                       \cdots , f_c((x_0,\cdots, x_{\nu - 1}))).
///
/// Given a degree bound `d_max` for all variables, it suffices to compute the evaluations of `s_i`
/// at `d_max + 1` points. Given that `s_{i}(0) = s_{i}(1) - s_{i - 1}(r_{i - 1})` it is sufficient
/// to compute the evaluations on only `d_max` points.
///
/// The algorithm works by iterating over the variables (x_{i + 1}, \cdots, x_{\nu - 1}) in
/// {0, 1}^{\nu - 1 - i}. For each such tuple, we store the evaluations of the (folded)
/// multi-linears at (0, x_{i + 1}, \cdots, x_{\nu - 1}) and
/// (1, x_{i + 1}, \cdots, x_{\nu - 1}) in two arrays, `evals_zero` and `evals_one`.
/// Using `evals_one`, remember that we optimize evaluating at 0 away, we get the first evaluation
/// i.e., `s_i(1)`.
///
/// For the remaining evaluations, we use the fact that the folded `f_i` is multi-linear and hence
/// we can write
///
///     f_i(X_i, x_{i + 1}, \cdots, x_{\nu - 1}) =
///        (1 - X_i) . f_i(0, x_{i + 1}, \cdots, x_{\nu - 1}) + X_i . f_i(1, x_{i + 1}, \cdots, x_{\nu - 1})
///
/// Note that we omitted writing the folding randomness for readability.
/// Since the evaluation domain is {0, 1, ... , d_max}, we can compute the evaluations based on
/// the previous one using only additions. This is the purpose of `deltas`, to hold the increments
/// added to each multi-linear to compute the evaluation at the next point, and `evals_x` to hold
/// the current evaluation at `x` in {2, ... , d_max}.
fn sumcheck_round<E: FieldElement>(
    polynomial: &dyn CompositionPolynomial<E>,
    mls: &mut [MultiLinear<E>],
) -> RoundProof<E> {
    let num_ml = mls.len();
    let num_vars = mls[0].num_variables();
    let num_rounds = num_vars - 1;

    let mut evals_zero = vec![E::ZERO; num_ml];
    let mut evals_one = vec![E::ZERO; num_ml];
    let mut deltas = vec![E::ZERO; num_ml];
    let mut evals_x = vec![E::ZERO; num_ml];

    let total_evals = (0..1 << num_rounds).map(|i| {
        for (j, ml) in mls.iter().enumerate() {
            evals_zero[j] = ml.evaluations()[i << 1];
            evals_one[j] = ml.evaluations()[(i << 1) + 1];
        }

        let mut total_evals = vec![E::ZERO; polynomial.max_degree()];
        total_evals[0] = polynomial.evaluate(&evals_one);

        evals_zero
            .iter()
            .zip(evals_one.iter().zip(deltas.iter_mut().zip(evals_x.iter_mut())))
            .for_each(|(a0, (a1, (delta, evx)))| {
                *delta = *a1 - *a0;
                *evx = *a1;
            });

        total_evals.iter_mut().skip(1).for_each(|e| {
            evals_x.iter_mut().zip(deltas.iter()).for_each(|(evx, delta)| {
                *evx += *delta;
            });
            *e = polynomial.evaluate(&evals_x);
        });
        total_evals
    });

    let evaluations = total_evals.fold(vec![E::ZERO; polynomial.max_degree()], |mut acc, evals| {
        acc.iter_mut().zip(evals.iter()).for_each(|(a, ev)| *a += *ev);
        acc
    });

    RoundProof {
        poly_evals: evaluations,
    }
}
