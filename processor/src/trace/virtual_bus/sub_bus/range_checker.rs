use super::BusBuilder;
use crate::trace::virtual_bus::multilinear::CompositionPolynomial;
use alloc::{sync::Arc, vec::Vec};
use core::marker::PhantomData;
use miden_air::trace::{
    chiplets::MEMORY_D0_COL_IDX,
    decoder::{DECODER_OP_BITS_OFFSET, DECODER_USER_OP_HELPERS_OFFSET},
    range::{M_COL_IDX, V_COL_IDX},
    CHIPLETS_OFFSET,
};
use vm_core::FieldElement;

/// Defines the range checker sub-bus.
///
/// Define the following variables:
///
/// - rc_value: the range checker value
/// - rc_multiplicity: the range checker multiplicity value
/// - flag_s: boolean flag indicating a stack operation with range checks. This flag is degree 3.
/// - sv0-sv3: stack value 0-3, the 4 values range-checked from the stack
/// - flag_m: boolean flag indicating the memory chiplet is active (i.e. range checks are required).
///   This flag is degree 3.
/// - mv0-mv1: memory value 0-1, the 2 values range-checked from the memory chiplet
///
/// Let `col(x)` denote the multi-linear extension of trace column `col`. This means that
/// for x \in \{0 , 1\}^{\nu}, where \nu is the log_2 of the trace length, `col(x)` is the value
/// at row index [x] = \sum_{j=0}^{\nu - 1} x_j * 2^j of column `col`.
///
/// Given the above, the range checker is implemented using the expression
///
/// 0 = \sum_{x \in \{0 , 1\}^{\nu}} rc_multiplicity(x) / (alpha - rc_value)(x)
///        - flag_s(x) / (alpha - sv0)(x) - flag_s(x) / (alpha - sv1)(x)
///        - flag_s(x) / (alpha - sv2)(x) - flag_s(x) / (alpha - sv3)(x)
///        - flag_m(x) / (alpha - mv0)(x) - flag_m(x) / (alpha - mv1)(x)
pub struct RangeCheckerBus<E: FieldElement> {
    alphas: Vec<E>,
}

impl<E: FieldElement> RangeCheckerBus<E> {
    pub fn new(alphas: &[E]) -> Self {
        Self {
            alphas: alphas.to_vec(),
        }
    }
}

impl<E: FieldElement + 'static> BusBuilder<E> for RangeCheckerBus<E> {
    fn compute_initial_claim(&self) -> E {
        E::ZERO
    }

    fn build_numerators(&self) -> Vec<Arc<dyn CompositionPolynomial<E>>> {
        vec![
            // rc_multiplicity(x)
            Arc::new(RangeCheckMultiplicity::default()),
            // flag_m(x)
            Arc::new(MemoryFlagChiplet::default()),
            Arc::new(MemoryFlagChiplet::default()),
            // flag_s(x)
            Arc::new(U32RangeCheckFlag::default()),
            Arc::new(U32RangeCheckFlag::default()),
            Arc::new(U32RangeCheckFlag::default()),
            Arc::new(U32RangeCheckFlag::default()),
        ]
    }

    fn build_denominators(&self) -> Vec<Arc<dyn CompositionPolynomial<E>>> {
        vec![
            // (alpha - rc_value)(x)
            Arc::new(TableValue::new(self.alphas.clone())),
            // (alpha - mv0)(x)
            Arc::new(MemoryValue::new(0, self.alphas.clone())),
            // (alpha - mv1)(x)
            Arc::new(MemoryValue::new(1, self.alphas.clone())),
            // (alpha - sv0)(x)
            Arc::new(StackValue::new(0, self.alphas.clone())),
            // (alpha - sv1)(x)
            Arc::new(StackValue::new(1, self.alphas.clone())),
            // (alpha - sv2)(x)
            Arc::new(StackValue::new(2, self.alphas.clone())),
            // (alpha - sv3)(x)
            Arc::new(StackValue::new(3, self.alphas.clone())),
        ]
    }
}

#[derive(Default)]
pub struct U32RangeCheckFlag<E>
where
    E: FieldElement,
{
    phantom: PhantomData<E>,
}

impl<E> CompositionPolynomial<E> for U32RangeCheckFlag<E>
where
    E: FieldElement,
{
    fn num_variables(&self) -> u32 {
        1
    }

    fn max_degree(&self) -> u32 {
        3
    }

    fn evaluate(&self, query: &[E]) -> E {
        let op_bit_4 = query[DECODER_OP_BITS_OFFSET + 4];
        let op_bit_5 = query[DECODER_OP_BITS_OFFSET + 5];
        let op_bit_6 = query[DECODER_OP_BITS_OFFSET + 6];

        (E::ONE - op_bit_4) * (E::ONE - op_bit_5) * op_bit_6
    }
}

#[derive(Default)]
pub struct MemoryFlagChiplet<E>
where
    E: FieldElement,
{
    phantom: PhantomData<E>,
}

impl<E> CompositionPolynomial<E> for MemoryFlagChiplet<E>
where
    E: FieldElement,
{
    fn num_variables(&self) -> u32 {
        1
    }

    fn max_degree(&self) -> u32 {
        3
    }

    fn evaluate(&self, query: &[E]) -> E {
        let mem_selec0 = query[CHIPLETS_OFFSET];
        let mem_selec1 = query[CHIPLETS_OFFSET + 1];
        let mem_selec2 = query[CHIPLETS_OFFSET + 2];

        mem_selec0 * mem_selec1 * (E::ONE - mem_selec2)
    }
}

#[derive(Default)]
pub struct RangeCheckMultiplicity<E>
where
    E: FieldElement,
{
    phantom: PhantomData<E>,
}

impl<E> CompositionPolynomial<E> for RangeCheckMultiplicity<E>
where
    E: FieldElement,
{
    fn num_variables(&self) -> u32 {
        1
    }

    fn max_degree(&self) -> u32 {
        1
    }

    fn evaluate(&self, query: &[E]) -> E {
        query[M_COL_IDX]
    }
}

pub struct StackValue<E>
where
    E: FieldElement,
{
    i: usize,
    alphas: Vec<E>,
}

impl<E> StackValue<E>
where
    E: FieldElement,
{
    pub fn new(i: usize, alphas: Vec<E>) -> Self {
        assert!(i < 4);
        Self { i, alphas }
    }
}

impl<E> CompositionPolynomial<E> for StackValue<E>
where
    E: FieldElement,
{
    fn num_variables(&self) -> u32 {
        1
    }

    fn max_degree(&self) -> u32 {
        1
    }

    fn evaluate(&self, query: &[E]) -> E {
        -(self.alphas[0] - query[DECODER_USER_OP_HELPERS_OFFSET + self.i])
    }
}

pub struct MemoryValue<E>
where
    E: FieldElement,
{
    i: usize,
    alphas: Vec<E>,
}

impl<E> MemoryValue<E>
where
    E: FieldElement,
{
    pub fn new(i: usize, alphas: Vec<E>) -> Self {
        assert!(i < 2);
        Self { i, alphas }
    }
}

impl<E> CompositionPolynomial<E> for MemoryValue<E>
where
    E: FieldElement,
{
    fn num_variables(&self) -> u32 {
        1
    }

    fn max_degree(&self) -> u32 {
        1
    }

    fn evaluate(&self, query: &[E]) -> E {
        -(self.alphas[0] - query[MEMORY_D0_COL_IDX + self.i])
    }
}

pub struct TableValue<E>
where
    E: FieldElement,
{
    alphas: Vec<E>,
}

impl<E> TableValue<E>
where
    E: FieldElement,
{
    pub fn new(alphas: Vec<E>) -> Self {
        Self { alphas }
    }
}

impl<E> CompositionPolynomial<E> for TableValue<E>
where
    E: FieldElement,
{
    fn num_variables(&self) -> u32 {
        1
    }

    fn max_degree(&self) -> u32 {
        1
    }

    fn evaluate(&self, query: &[E]) -> E {
        self.alphas[0] - query[V_COL_IDX]
    }
}
