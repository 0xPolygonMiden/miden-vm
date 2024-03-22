use super::{domain::EvaluationDomain, reduce_claim, Proof, RoundClaim, RoundProof};
use crate::trace::virtual_bus::multilinear::{CompositionPolynomial, MultiLinear};
use core::marker::PhantomData;
use vm_core::{FieldElement, StarkField};
use winter_prover::crypto::{ElementHasher, RandomCoin};

/// A struct that contains relevant information for the execution of the multivariate sum-check
/// protocol.
/// The sum-check protocol is an interactive protocol (IP) for proving the following relation:
///
/// v = \sum_{(x_0,\cdots, x_{\nu - 1}) \in \{0 , 1\}^{2^{\nu}}} g(f_0((x_0,\cdots, x_{\nu - 1})), \cdots , f_c((x_0,\cdots, x_{\nu - 1})))
///
/// where:
///
/// 1. v ∈ 𝔽 where 𝔽 is a finite field.
/// 2. f_i are multi-linear polynomials i.e., polynomials in 𝔽[X_i, \cdots ,X_{\nu - 1}] with degree at most one in each variable.
/// 3. g is a multivariate polynomial with degree at most d in each variable.
///
/// The verifier is given commitments to each `f_i` in addition to the claimed sum `v`. The Prover then engages in an IP
/// to convince the Verifier that the above relation holds for the given `f_i` and `v`.
/// More precisely:
///
/// 0. Denote by w(x_0,\cdots, x_{\nu - 1}) := g(f_0((x_0,\cdots, x_{\nu - 1})), \cdots , f_c((x_0,\cdots, x_{\nu - 1}))).
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
///         s_i(X_i) := \sum_{(x_{i + 1},\cdots, x_{\nu - 1})  w(r_0,\cdots, r_{i - 1}, X_i, x_{i + 1}, \cdots, x_{\nu - 1})
///     b. The Verifier checks that s_{i - 1}(r_{i - 1}) = s_{i}(0) + s_{i}(1) rejecting if not.
///     
///     c. The Verifier samples a random challenge `r_i ∈ 𝔽` and sends it to the Prover.
///
/// 5. The Verifier now queries each of the oracles behind the commitments i.e., `f_i` at `(r_0, \cdots , r_{\nu - 1})` to get
/// u_i = f_i(r_0, \cdots , r_{\nu - 1}). The Verifier then accepts if and only if:
///
///         s_{\nu - 1}(r_{\nu - 1}) = g(u_0, \cdots , u_{\nu - 1})
/// 
/// A few remarks:
/// 
/// 1. The degree bound on `g` implies that each of the `s_i` polynomials is a univariate polynomial of degree at most `d`. 
/// Thus, the Prover in each round sends `d + 1` values, either the coefficients or the evaluations of `s_i`.
/// 
/// 2. The Prover has each `f_i` in its evaluation form over the hyper-cube \{0 , 1\}^{2^{\nu}}.
/// 
/// 3. An optimization is for the Prover to not send `s_i(0)` as it can be recoverd from the current reduced claim s_{i - 1}(r_{i - 1})
/// using the relation  s_{i}(0) = s_{i}(1) - s_{i - 1}(r_{i - 1}). This also means that the Verifier can skip point 4.b.
pub struct SumCheckProver<B, E, P, C, H>
where
    B: StarkField,
    E: FieldElement<BaseField = B>,
    C: RandomCoin<Hasher = H, BaseField = B>,
    H: ElementHasher<BaseField = B>,
{
    pub virtual_poly: P,
    pub eval_domain: EvaluationDomain<E>,
    num_rounds: usize,
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
    pub fn new(virtual_poly: P, num_rounds: usize) -> Self {
        let max_degree = virtual_poly.max_degree();
        let eval_domain = EvaluationDomain::new(max_degree);

        Self {
            virtual_poly,
            eval_domain,
            num_rounds,
            _challenger: PhantomData,
            _hasher: PhantomData,
        }
    }

    pub fn prove(
        &self,
        claim: E,
        mls: &mut Vec<MultiLinear<E>>,
        coin: &mut C,
    ) -> (RoundClaim<E>, Proof<E>) {
        // Setup first round
        let mut prev_claim = RoundClaim {
            partial_eval_point: vec![],
            current_claim: claim,
        };
        let mut round_proofs = vec![];

        let mut output = sumcheck_round(&self.virtual_poly, mls);
        round_proofs.push(output);

        for i in 1..self.num_rounds {
            let round_challenge = coin.draw().unwrap();
            let new_claim =
                reduce_claim(&self.eval_domain, &round_proofs[i - 1], prev_claim, round_challenge);
            mls.into_iter().for_each(|ml| ml.bind_assign(round_challenge));

            output = sumcheck_round(&self.virtual_poly, mls);
            round_proofs.push(output);

            prev_claim = new_claim;

            let poly_evals = &round_proofs[i].poly_evals;
            coin.reseed(H::hash_elements(poly_evals));
        }
        let round_challenge = coin.draw().unwrap();
        mls.into_iter().for_each(|ml| ml.bind_assign(round_challenge));

        let final_round_claim = reduce_claim(
            &self.eval_domain,
            &round_proofs[self.num_rounds - 1],
            prev_claim,
            round_challenge,
        );
        let round_proofs = Proof { round_proofs };

        (final_round_claim, round_proofs)
    }
}

fn sumcheck_round<E: FieldElement>(
    composer: &dyn CompositionPolynomial<E>,
    mls: &mut Vec<MultiLinear<E>>,
) -> RoundProof<E> {
    let polynomial = composer;
    let num_ml = mls.len();
    let num_vars = mls[0].num_variables();
    let num_rounds = num_vars - 1;

    let mut evals_zero = vec![E::ZERO; num_ml];
    let mut evals_one = vec![E::ZERO; num_ml];
    let mut deltas = vec![E::ZERO; num_ml];
    let mut evals_x = vec![E::ZERO; num_ml];

    let total_evals = (0..1 << num_rounds).into_iter().map(|i| {
        for (j, ml) in mls.iter().enumerate() {
            evals_zero[j] = ml.evaluations()[(i << 1) as usize];
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
