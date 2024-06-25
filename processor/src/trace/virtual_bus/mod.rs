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
use alloc::vec::Vec;
use miden_air::{trace::main_trace::MainTrace, AuxRandElements};
use vm_core::{Felt, FieldElement};

mod circuit;
pub use circuit::prove;

mod sum_check;
pub use sum_check::SumCheckProver;
use winter_prover::LagrangeKernelRandElements;

mod univariate;

#[cfg(test)]
mod tests;

const RAND_ELEMENTS_ERR: &str = "Failed to build the LogUp-GKR auxiliary columns. Does the AirContext specify the Lagrange kernel column index?";

// Builds the Lagrange kernel column and "s" auxiliary columns.
pub fn build_aux_columns<E>(
    main_trace: &MainTrace,
    rand_elements: &AuxRandElements<E>,
) -> Vec<Vec<E>>
where
    E: FieldElement<BaseField = Felt>,
{
    let lagrange_kernel_column = build_lagrange_kernel_column(
        main_trace.num_rows(),
        rand_elements.lagrange().expect(RAND_ELEMENTS_ERR),
    );
    let s_column = build_s_column(
        main_trace,
        &lagrange_kernel_column,
        rand_elements.gkr_openings_combining_randomness().expect(RAND_ELEMENTS_ERR),
    );

    vec![lagrange_kernel_column, s_column]
}

fn build_lagrange_kernel_column<E>(
    trace_len: usize,
    lagrange_kernel_rand_elements: &LagrangeKernelRandElements<E>,
) -> Vec<E>
where
    E: FieldElement<BaseField = Felt>,
{
    let r = lagrange_kernel_rand_elements;

    let mut lagrange_col = Vec::with_capacity(trace_len);

    for row in 0..trace_len {
        let mut row_value = E::ONE;

        for (i, &r_i) in r.iter().enumerate() {
            if row & (1 << i) == 0 {
                row_value *= E::ONE - r_i;
            } else {
                row_value *= r_i;
            }
        }

        lagrange_col.push(row_value);
    }

    lagrange_col
}

fn build_s_column<E>(
    main_trace: &MainTrace,
    lagrange_kernel_col: &[E],
    openings_combining_randomness: &[E],
) -> Vec<E>
where
    E: FieldElement<BaseField = Felt>,
{
    let mut s_col = Vec::with_capacity(main_trace.num_rows());

    let mut main_trace_row = vec![Felt::ZERO; main_trace.num_base_cols()];
    let mut last_row_value = E::ZERO;

    for (row_idx, &lagrange_kernel_row) in lagrange_kernel_col.iter().enumerate() {
        main_trace.read_row_into(row_idx, &mut main_trace_row);
        let row_value = last_row_value
            + lagrange_kernel_row
                * inner_product(
                    openings_combining_randomness.iter(),
                    main_trace_row.iter().map(|ele| E::from(*ele)),
                );

        s_col.push(row_value);
        last_row_value = row_value;
    }

    s_col
}

// TODOP: Don't duplicate the one in lagrange kernel
pub fn inner_product<'a, E: FieldElement + 'a>(
    evaluations: impl IntoIterator<Item = &'a E>,
    tensored_query: impl IntoIterator<Item = E>,
) -> E {
    evaluations
        .into_iter()
        .zip(tensored_query)
        .fold(E::ZERO, |acc, (x_i, y_i)| acc + *x_i * y_i)
}
