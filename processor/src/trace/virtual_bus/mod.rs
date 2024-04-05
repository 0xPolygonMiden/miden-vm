//! Represents a global virtual bus for the Miden VM.
//!
//! A global bus is a single bus which encompasses several sub-buses, each representing
//! a communication channel between two or more components of the VM.
//! A bus represents a client-server relationship between some VM components. The server is
//! usually one specific component of the VM, e.g., hasher chiplet, and the client can be one or
//! several other components of the VM, e.g., the decoder. The communication between the clients
//! and the server is composed of `request` messages made by the clients and corresponding
//! `reply` messages by the server.
//! The purpose of the sub-bus then, from the verifiable computation point of view, is to ensure
//! the consistency between the `request` and `reply` messages exchanged by the clients and
//! the server.
//! The global bus uses a per-sub-bus address in order to ensure correct routing of the `request`
//! messages and their matching `reply` messages.
//! Miden VM uses a virtual global bus in the sense that neither the global bus nor the individual
//! sub-buses are fully materialized as part of the (auxiliary) trace. This is replaced by a layered
//! circuit which computes the global bus relation. The correct evaluation of this circuit is then
//! proved using the GKR protocol of Goldwasser, Kalai and Rothblum [1] using the protocol in
//! GKR-LogUp [2].
//!
//! [1]: https://dl.acm.org/doi/10.1145/2699436
//! [2]: https://eprint.iacr.org/2023/1284

use self::{
    error::Error,
    multilinear::CompositionPolynomial,
    sub_bus::{BusBuilder, RangeCheckerBus},
};
use alloc::{sync::Arc, vec::Vec};
use core::marker::PhantomData;
use vm_core::FieldElement;

mod circuit;
mod multilinear;
pub use circuit::{prove, verify};
mod sum_check;
mod univariate;
pub use sum_check::{SumCheckProver, SumCheckVerifier};

mod prover;
pub use prover::VirtualBusProver;
mod verifier;
pub use verifier::VirtualBusVerifier;
mod error;
mod sub_bus;

#[cfg(test)]
mod tests;

/// Generates the composition polynomials describing the virtual bus.
#[allow(clippy::type_complexity)]
fn generate<E: FieldElement + 'static>(
    log_up_randomness: Vec<E>,
) -> Result<(E, Vec<Vec<Arc<dyn CompositionPolynomial<E>>>>), Error> {
    // build each sub-bus of the virtual bus
    // TODO: This implements only the range checker bus and should be generalized to iterate
    // over all the remaining sub-buses
    let range_checker_bus_builder = RangeCheckerBus::new(&log_up_randomness);
    let claim = range_checker_bus_builder.compute_initial_claim();
    let numerators = range_checker_bus_builder.build_numerators();
    let denominators = range_checker_bus_builder.build_denominators();

    // the numerators and denominators should have matching lengths
    if numerators.len() != denominators.len() {
        return Err(Error::NumeratorDenominatorLengthMismatch);
    }

    // should have at least two numerator/denominator
    if numerators.len() < 2 {
        return Err(Error::NumeratorDenominatorLessThanTwo);
    }

    // split the numerators and denominators into two subsets, a left and a right one
    let mut numerators = pad_and_split(numerators, false);
    let denominators = pad_and_split(denominators, true);
    numerators.extend_from_slice(&denominators);

    Ok((claim, numerators))
}

/// Pads a set of composition polynomials to the next power of two and splits the resulting padded
/// into two sub-sets of equal size.
///
/// The padding is done using the `ZeroNumerator` composition polynomial in the case when the set
/// is the set of composition polynomials appearing in the numerators, and is done using
/// the `OneDenominator` composition polynomial in the case of denominators.
fn pad_and_split<E: FieldElement + 'static>(
    vector: Vec<Arc<dyn CompositionPolynomial<E>>>,
    denominator: bool,
) -> Vec<Vec<Arc<dyn CompositionPolynomial<E>>>> {
    let length = vector.len();
    let padded_length = length.next_power_of_two();

    let mut output = Vec::with_capacity(padded_length);
    output.extend_from_slice(&vector);

    if denominator {
        for _i in length..padded_length {
            output.push(Arc::new(OneDenominator::default()));
        }
    } else {
        for _i in length..padded_length {
            output.push(Arc::new(ZeroNumerator::default()));
        }
    }

    let mut left = output;
    let right = left.split_off(left.len() >> 1);

    vec![left, right]
}

/// A polynomial representing the identically zero polynomial.
///
/// This is used for padding to the next power of 2 as the current implementation of the sum-check
/// protocol requires that the number of numerators be a power of two. The same holds for the
/// denominators.
/// It should be noted that this padding has no effect on the sum computed by the GKR circuit as
/// adding a fraction with a zero numerator and a non-zero denominator doesn't change the value of
/// the sum.
#[derive(Default)]
pub struct ZeroNumerator<E>
where
    E: FieldElement,
{
    phantom: PhantomData<E>,
}

impl<E> CompositionPolynomial<E> for ZeroNumerator<E>
where
    E: FieldElement,
{
    fn num_variables(&self) -> u32 {
        1 // TODO: Update
    }

    fn max_degree(&self) -> u32 {
        1
    }

    fn evaluate(&self, _query: &[E]) -> E {
        E::ZERO
    }
}

/// A polynomial representing the identically `E::ONE` polynomial.
///
/// This is used for padding to the next power of 2 as the current implementation of the sum-check
/// protocol requires that the number of denominators be a power of two. The same holds for the
/// numerators.
/// It should be noted that this padding has no effect on the sum computed by the GKR circuit as
/// adding a fraction with a zero numerator and a non-zero denominator doesn't change the value of
/// the sum.
#[derive(Default)]
pub struct OneDenominator<E>
where
    E: FieldElement,
{
    phantom: PhantomData<E>,
}

impl<E> CompositionPolynomial<E> for OneDenominator<E>
where
    E: FieldElement,
{
    fn num_variables(&self) -> u32 {
        1 // TODO: Update
    }

    fn max_degree(&self) -> u32 {
        1
    }

    fn evaluate(&self, _query: &[E]) -> E {
        E::ONE
    }
}
