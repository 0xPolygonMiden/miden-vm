use core::ops::Index;
use miden_air::trace::TRACE_WIDTH;
use std::sync::Arc; // TODO: change to alloc
use vm_core::{Felt, FieldElement};
use winter_prover::math::log2;

mod lagrange_ker;
pub use lagrange_ker::LagrangeKernel;

#[derive(Clone, Debug)]
pub struct MultiLinear<E: FieldElement> {
    num_variables: usize,
    evaluations: Vec<E>,
}

impl<E: FieldElement> MultiLinear<E> {
    pub fn new(values: Vec<E>) -> Self {
        assert!(values.len().is_power_of_two(), "A multi-linear polynomial should have a power of 2 number of evaluations over the Boolean hyper-cube");
        Self {
            num_variables: log2(values.len()) as usize,
            evaluations: values,
        }
    }

    pub fn from_values(values: &[E]) -> Self {
        assert!(values.len().is_power_of_two(), "A multi-linear polynomial should have a power of 2 number of evaluations over the Boolean hyper-cube");
        Self {
            num_variables: log2(values.len()) as usize,
            evaluations: values.to_owned(),
        }
    }

    pub fn num_variables(&self) -> usize {
        self.num_variables
    }

    pub fn evaluations(&self) -> &[E] {
        &self.evaluations
    }

    pub fn num_evaluations(&self) -> usize {
        self.evaluations.len()
    }

    pub fn evaluate(&self, query: &[E]) -> E {
        let tensored_query = tensorize(query);
        inner_product(&self.evaluations, &tensored_query)
    }

    pub fn bind(&self, round_challenge: E) -> Self {
        let mut result = vec![E::ZERO; 1 << (self.num_variables() - 1)];
        for i in 0..(1 << (self.num_variables() - 1)) {
            result[i] = self.evaluations[i << 1]
                + round_challenge * (self.evaluations[(i << 1) + 1] - self.evaluations[i << 1]);
        }
        Self::from_values(&result)
    }

    pub fn bind_assign(&mut self, round_challenge: E) {
        let mut result = vec![E::ZERO; 1 << (self.num_variables() - 1)];
        for i in 0..(1 << (self.num_variables() - 1)) {
            result[i] = self.evaluations[i << 1]
                + round_challenge * (self.evaluations[(i << 1) + 1] - self.evaluations[i << 1]);
        }
        *self = Self::from_values(&result);
    }

    pub fn extend(&mut self, other: &MultiLinear<E>) {
        let other_vec = other.evaluations.to_vec();
        assert_eq!(other_vec.len(), self.num_evaluations());
        self.evaluations.extend(other_vec);
        self.num_variables += 1;
    }
}

impl<E: FieldElement> Index<usize> for MultiLinear<E> {
    type Output = E;

    fn index(&self, index: usize) -> &E {
        &(self.evaluations[index])
    }
}

/// A multi-variate polynomial for composing individual multi-linear polynomials
pub trait CompositionPolynomial<E: FieldElement>: Sync + Send {
    /// The number of variables when interpreted as a multi-variate polynomial.
    fn num_variables(&self) -> usize;

    /// Maximum degree in all variables.
    fn max_degree(&self) -> usize;

    /// Given a query, of length equal the number of variables, evaluate [Self] at this query.
    fn evaluate(&self, query: &[E]) -> E;
}

#[derive(Clone, Copy, Debug)]
pub struct ProjectionComposition {
    coordinate: usize,
}

impl ProjectionComposition {
    pub fn new(coordinate: usize) -> Self {
        Self { coordinate }
    }
}

impl<E> CompositionPolynomial<E> for ProjectionComposition
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
        query[self.coordinate]
    }
}

pub fn gkr_merge_composition_from_composition_polys<E: FieldElement<BaseField = Felt> + 'static>(
    composition_polys: &Vec<Vec<Arc<dyn CompositionPolynomial<E>>>>,
    sum_check_combining_randomness: E,
    merge_randomness: Vec<E>,
    num_variables: usize,
) -> GkrCompositionMerge<E> {
    let eq_composer = Arc::new(ProjectionComposition::new(TRACE_WIDTH));
    let left_numerator = composition_polys[0].to_owned();
    let right_numerator = composition_polys[1].to_owned();
    let left_denominator = composition_polys[2].to_owned();
    let right_denominator = composition_polys[3].to_owned();
    GkrCompositionMerge::new(
        num_variables,
        sum_check_combining_randomness,
        merge_randomness,
        eq_composer,
        right_numerator,
        left_numerator,
        right_denominator,
        left_denominator,
    )
}

#[derive(Clone)]
pub struct GkrComposition<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub num_variables_ml: usize,
    pub combining_randomness: E,
}

impl<E> GkrComposition<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub fn new(num_variables_ml: usize, combining_randomness: E) -> Self {
        Self {
            num_variables_ml,
            combining_randomness,
        }
    }
}

impl<E> CompositionPolynomial<E> for GkrComposition<E>
where
    E: FieldElement<BaseField = Felt>,
{
    fn num_variables(&self) -> usize {
        self.num_variables_ml // + TODO
    }

    fn max_degree(&self) -> usize {
        3
    }

    fn evaluate(&self, query: &[E]) -> E {
        let eval_left_numerator = query[0];
        let eval_right_numerator = query[1];
        let eval_left_denominator = query[2];
        let eval_right_denominator = query[3];
        let eq_eval = query[4];
        let res = eq_eval
            * ((eval_left_numerator * eval_right_denominator
                + eval_right_numerator * eval_left_denominator)
                + eval_left_denominator * eval_right_denominator * self.combining_randomness);
        res
    }
}

#[derive(Clone)]
pub struct GkrCompositionMerge<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub num_variables_ml: usize,
    pub sum_check_combining_randomness: E,
    pub tensored_merge_randomness: Vec<E>,
    pub degree: usize,

    pub eq_composer: Arc<dyn CompositionPolynomial<E>>,
    pub right_numerator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
    pub left_numerator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
    pub right_denominator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
    pub left_denominator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
}

impl<E> GkrCompositionMerge<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub fn new(
        num_variables_ml: usize,
        combining_randomness: E,
        merge_randomness: Vec<E>,
        eq_composer: Arc<dyn CompositionPolynomial<E>>,
        right_numerator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
        left_numerator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
        right_denominator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
        left_denominator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
    ) -> Self {
        let tensored_merge_randomness = LagrangeKernel::new(merge_randomness.clone()).evaluations();

        let max_left_num = left_numerator_composer.iter().map(|c| c.max_degree()).max().unwrap();
        let max_right_num = right_numerator_composer.iter().map(|c| c.max_degree()).max().unwrap();
        let max_left_denom =
            left_denominator_composer.iter().map(|c| c.max_degree()).max().unwrap();
        let max_right_denom =
            right_denominator_composer.iter().map(|c| c.max_degree()).max().unwrap();
        let degree =
            1 + core::cmp::max(max_left_num + max_right_denom, max_right_num + max_left_denom);

        Self {
            num_variables_ml,
            sum_check_combining_randomness: combining_randomness,
            eq_composer,
            degree,
            right_numerator_composer,
            left_numerator_composer,
            right_denominator_composer,
            left_denominator_composer,
            tensored_merge_randomness,
        }
    }
}

impl<E> CompositionPolynomial<E> for GkrCompositionMerge<E>
where
    E: FieldElement<BaseField = Felt>,
{
    fn num_variables(&self) -> usize {
        self.num_variables_ml // + TODO
    }

    fn max_degree(&self) -> usize {
        self.degree
    }

    fn evaluate(&self, query: &[E]) -> E {
        let eval_right_numerator =
            self.right_numerator_composer.iter().enumerate().fold(E::ZERO, |acc, (i, ml)| {
                acc + ml.evaluate(query) * self.tensored_merge_randomness[i]
            });
        let eval_left_numerator =
            self.left_numerator_composer.iter().enumerate().fold(E::ZERO, |acc, (i, ml)| {
                acc + ml.evaluate(query) * self.tensored_merge_randomness[i]
            });
        let eval_right_denominator = self
            .right_denominator_composer
            .iter()
            .enumerate()
            .fold(E::ZERO, |acc, (i, ml)| {
                acc + ml.evaluate(query) * self.tensored_merge_randomness[i]
            });
        let eval_left_denominator =
            self.left_denominator_composer.iter().enumerate().fold(E::ZERO, |acc, (i, ml)| {
                acc + ml.evaluate(query) * self.tensored_merge_randomness[i]
            });
        let eq_eval = self.eq_composer.evaluate(query);
        let res = eq_eval
            * ((eval_left_numerator * eval_right_denominator
                + eval_right_numerator * eval_left_denominator)
                + eval_left_denominator
                    * eval_right_denominator
                    * self.sum_check_combining_randomness);
        res
    }
}

fn to_index<E: FieldElement<BaseField = Felt>>(index: &[E]) -> usize {
    let res = index.iter().fold(E::ZERO, |acc, term| acc * E::ONE.double() + (*term));
    let res = res.base_element(0);
    res.as_int() as usize
}

fn inner_product<E: FieldElement>(evaluations: &[E], tensored_query: &[E]) -> E {
    assert_eq!(evaluations.len(), tensored_query.len());
    evaluations
        .iter()
        .zip(tensored_query.iter())
        .fold(E::ZERO, |acc, (x_i, y_i)| acc + *x_i * *y_i)
}

pub fn tensorize<E: FieldElement>(query: &[E]) -> Vec<E> {
    let nu = query.len();
    let n = 1 << nu;

    (0..n).map(|i| lagrange_basis_eval(query, i)).collect()
}

fn lagrange_basis_eval<E: FieldElement>(query: &[E], i: usize) -> E {
    query
        .iter()
        .enumerate()
        .map(|(j, x_j)| if i & (1 << j) == 0 { E::ONE - *x_j } else { *x_j })
        .fold(E::ONE, |acc, v| acc * v)
}
