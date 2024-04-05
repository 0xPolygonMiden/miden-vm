use super::{
    circuit::GkrCircuitProof,
    error::Error,
    generate,
    multilinear::{CompositionPolynomial, MultiLinearPoly},
    prove,
};
use alloc::{sync::Arc, vec::Vec};
use core::marker::PhantomData;
use miden_air::trace::main_trace::MainTrace;
use vm_core::{Felt, FieldElement};
use winter_prover::crypto::{ElementHasher, RandomCoin};

/// A struct which implements the logic for proving the correctness of the global virtual bus
/// relation.
///
/// The global virtual bus relation is encoded as a layered circuit and is proven using the GKR
/// protocol for proving the correct evaluation of a circuit. The correctness of the resulting
/// GKR proof entails, together with that of the final evaluation claim (see further down), that
/// the current instance of the global virtual bus is consistant with overwhelming probability.
/// This last statement is conditioned on the correctness of what is called the final evaluation
/// claim. This is a claim on the openings of the multi-linear extensions of the (main) trace
/// columns involved in the global virtual bus relation at some random point. This random point
/// is the result of the challenges sent by the GKR verifier in the interactive version of the GKR
/// protocol.
/// As Miden VM uses FRI as its polynomial commitment scheme, we use the transformation in
/// section 5 of [1] for transforming a univariate polynomial commitment scheme into a multi-variate
/// one. This means that the (outer) STARK will take the final evaluation claim in
/// `GkrCircuitProof::FinalLayerProof::Proof::FinalOpeningClaim` and use it to construct two columns
/// in the auxiliary trace, one for the Lagrange kernel at `FinalOpeningClaim::eval_point` and the
/// other for computing the evaluation of the (batched) multi-linear extensions of the relevant
/// columns at `FinalOpeningClaim::eval_point` using the aforementioned Lagrange kernel.
/// This means, in a sense, that the opening proof for the final GKR evaluation is part of the STARK
/// proof.
///
/// [1]: https://eprint.iacr.org/2023/1284
pub struct VirtualBusProver<E, C, H>
where
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
{
    composition_polynomials: Vec<Vec<Arc<dyn CompositionPolynomial<E>>>>,
    _challenger: PhantomData<C>,
}

impl<E, C, H> VirtualBusProver<E, C, H>
where
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
{
    /// Constructs a new [VirtualBusProver] given a set of random values for the GKR-LogUp relation.
    ///
    /// The constructor uses the randomness to do two things:
    ///
    /// 1. Compute the claimed value of the GKR-LogUp relation i.e., the generalization of the 0
    /// constant in equation (3) in [1]. This constant can depend on the LogUp randomness as in
    /// e.g., the overflow stack virtual table bus where such a constant represents entries in
    /// the overflow stack which were present before the VM started executing.
    ///
    /// 2. The composition polynomials describing the numerators and denominators of the fractions
    /// appearing on the left hand side of equation (3) in [1]. The denominators depend as well
    /// on the LogUp randomness.
    /// The composition polynomials are grouped into 4 sets that can be thought of as left/right
    /// numerator/denominator. The reason for this can be understood by looking at the original
    /// GKR protocol in [2] where the protocol therein uses the wiring predicates [\tilde{ADD}]
    /// and [\tilde{MUL}] to encode the wiring of the circuit. For our purposes, we can do
    /// away with wiring predicates using the left/right and numerator/denominator distinction.
    ///
    /// [1]: https://eprint.iacr.org/2023/1284
    /// [2]: https://dl.acm.org/doi/10.1145/2699436
    pub fn new(log_up_randomness: Vec<E>) -> Result<Self, Error> {
        let (_claim, composition_polynomials) = generate(log_up_randomness)?;

        Ok(Self {
            composition_polynomials,
            _challenger: PhantomData,
        })
    }

    /// Returns the composition polynomials of the left/right numerators/denominators of
    /// the GKR-LogUp relation.
    fn composition_polynomials(&self) -> Vec<Vec<Arc<dyn CompositionPolynomial<E>>>> {
        self.composition_polynomials.clone()
    }

    /// Proves the GKR-LogUp relation.
    pub fn prove(&self, trace: &MainTrace, transcript: &mut C) -> GkrCircuitProof<E> {
        // TODO: Optimize this so that we can work with base field element directly and thus save
        // on memory usage.
        let trace_len = trace.num_rows();
        let mut mls: Vec<MultiLinearPoly<E>> = trace
            .columns()
            .map(|col| {
                let mut values: Vec<E> = col.iter().map(|value| E::from(*value)).collect();
                values[trace_len - 1] = E::ZERO;
                MultiLinearPoly::from_evaluations(values).unwrap()
            })
            .collect();
        prove(self.composition_polynomials(), &mut mls, transcript)
    }
}
