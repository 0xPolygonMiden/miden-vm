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
            Arc::new(RangeCheckMultiplicity::default()),
            Arc::new(MemoryFlagChiplet::default()),
            Arc::new(MemoryFlagChiplet::default()),
            Arc::new(U32RangeCheckFlag::default()),
            Arc::new(U32RangeCheckFlag::default()),
            Arc::new(U32RangeCheckFlag::default()),
            Arc::new(U32RangeCheckFlag::default()),
        ]
    }

    fn build_denominators(&self) -> Vec<Arc<dyn CompositionPolynomial<E>>> {
        vec![
            Arc::new(TableValue::new(self.alphas.clone())),
            Arc::new(MemoryValue::new(0, self.alphas.clone())),
            Arc::new(MemoryValue::new(1, self.alphas.clone())),
            Arc::new(StackValue::new(0, self.alphas.clone())),
            Arc::new(StackValue::new(1, self.alphas.clone())),
            Arc::new(StackValue::new(2, self.alphas.clone())),
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
    fn num_variables(&self) -> usize {
        1
    }

    fn max_degree(&self) -> usize {
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
    fn num_variables(&self) -> usize {
        1
    }

    fn max_degree(&self) -> usize {
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
    fn num_variables(&self) -> usize {
        1
    }

    fn max_degree(&self) -> usize {
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
    fn num_variables(&self) -> usize {
        1
    }

    fn max_degree(&self) -> usize {
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
    fn num_variables(&self) -> usize {
        1
    }

    fn max_degree(&self) -> usize {
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
    fn num_variables(&self) -> usize {
        1
    }

    fn max_degree(&self) -> usize {
        1
    }

    fn evaluate(&self, query: &[E]) -> E {
        self.alphas[0] - query[V_COL_IDX]
    }
}
