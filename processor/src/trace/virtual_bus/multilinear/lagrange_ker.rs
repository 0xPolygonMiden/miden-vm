
use super::{FieldElement, MultiLinear};

pub struct LagrangeKernel<E> {
    r: Vec<E>,
}

impl<E: FieldElement> LagrangeKernel<E> {
    pub fn new(r: Vec<E>) -> Self {
        let mut tmp = r.clone();
        tmp.reverse();
        LagrangeKernel { r: tmp }
    }

    pub fn evaluate(&self, rho: &[E]) -> E {
        assert_eq!(self.r.len(), rho.len());
        (0..rho.len())
            .map(|i| self.r[i] * rho[i] + (E::ONE - self.r[i]) * (E::ONE - rho[i]))
            .fold(E::ONE, |acc, term| acc * term)
    }

    pub fn evaluations(&self) -> Vec<E> {
        let nu = self.r.len();

        let mut evals: Vec<E> = vec![E::ONE; 1 << nu];
        let mut size = 1;
        for j in 0..nu {
            size *= 2;
            for i in (0..size).rev().step_by(2) {
                let scalar = evals[i / 2];
                evals[i] = scalar * self.r[j];
                evals[i - 1] = scalar - evals[i];
            }
        }
        evals
    }

    pub fn new_ml(evaluation_point: Vec<E>) -> MultiLinear<E> {
        let eq_evals = LagrangeKernel::new(evaluation_point.clone()).evaluations();
        MultiLinear::from_values(&eq_evals)
    }
}