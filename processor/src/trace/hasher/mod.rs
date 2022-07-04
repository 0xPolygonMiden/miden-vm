use super::{AuxColumnBuilder, Felt, FieldElement, Matrix, Vec};
use crate::hasher::AuxTraceHints;

// HASHER AUXILIARY TRACE COLUMNS
// ================================================================================================

/// Builds and returns stack auxiliary trace column p1 describing states of the hasher sibling
/// table (used for Merkle root update computation).
pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
    main_trace: &Matrix<Felt>,
    aux_trace_hints: &AuxTraceHints,
    rand_elements: &[E],
) -> Vec<Vec<E>> {
    let p1 = aux_trace_hints.build_aux_column(main_trace, rand_elements);
    vec![p1]
}
