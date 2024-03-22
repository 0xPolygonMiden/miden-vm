use vm_core::{FieldElement, StarkField};
use winter_prover::math::batch_inversion;
pub struct EvaluationDomainCoeff<B, E>
where
    B: StarkField,
    E: FieldElement<BaseField = B>,
{
    max_degree: usize,
    vandermonde_matrix: Vec<Vec<E>>,
}

impl<B, E> EvaluationDomainCoeff<B, E>
where
    B: StarkField,
    E: FieldElement<BaseField = B>,
{
    pub fn new(max_degree: usize) -> Self {
        let n = max_degree + 1;
        let xs: Vec<E> = (0..n).map(|x| E::from(x as u32)).collect();

        let mut vandermonde_matrix: Vec<Vec<E>> = Vec::with_capacity(n);
        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            let x = xs[i];
            row.push(E::ONE);
            row.push(x);
            for j in 2..n {
                row.push(row[j - 1] * x);
            }
            row.push(E::ZERO);
            vandermonde_matrix.push(row);
        }

        Self {
            max_degree,
            vandermonde_matrix,
        }
    }

    fn interpolate_naive(evals: &[E]) -> Vec<E> {
        let n = evals.len();
        let xs: Vec<E> = (0..n).map(|x| E::from(x as u32)).collect();

        let mut vandermonde: Vec<Vec<E>> = Vec::with_capacity(n);
        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            let x = xs[i];
            row.push(E::ONE);
            row.push(x);
            for j in 2..n {
                row.push(row[j - 1] * x);
            }
            row.push(evals[i]);
            vandermonde.push(row);
        }

        gaussian_elimination(&mut vandermonde)
    }

    fn interpolate(&mut self, evals: &[E]) -> Vec<E> {
        let mut vandermonde = self.vandermonde_matrix.clone();
        for row in 0..(self.max_degree + 1) {
            vandermonde[row][self.max_degree] = evals[row];
        }
        gaussian_elimination(&mut vandermonde)
    }
}

pub fn gaussian_elimination<E: FieldElement>(matrix: &mut [Vec<E>]) -> Vec<E> {
    let size = matrix.len();
    assert_eq!(size, matrix[0].len() - 1);

    for i in 0..size - 1 {
        for j in i..size - 1 {
            echelon(matrix, i, j);
        }
    }

    for i in (1..size).rev() {
        eliminate(matrix, i);
    }

    // Disable cargo clippy warnings about needless range loops.
    // Checking the diagonal like this is simpler than any alternative.
    //#[allow(clippy::needless_range_loop)]
    for i in 0..size {
        if matrix[i][i] == E::ZERO {
            println!("Infinitely many solutions");
        }
    }

    let mut result: Vec<E> = vec![E::ZERO; size];
    for i in 0..size {
        result[i] = matrix[i][size] / matrix[i][i];
    }
    result
}

fn echelon<E: FieldElement>(matrix: &mut [Vec<E>], i: usize, j: usize) {
    let size = matrix.len();
    if matrix[i][i] == E::ZERO {
    } else {
        let factor = matrix[j + 1][i] / matrix[i][i];
        (i..size + 1).for_each(|k| {
            let tmp = matrix[i][k];
            matrix[j + 1][k] -= factor * tmp;
        });
    }
}

fn eliminate<E: FieldElement>(matrix: &mut [Vec<E>], i: usize) {
    let size = matrix.len();
    if matrix[i][i] == E::ZERO {
    } else {
        for j in (1..i + 1).rev() {
            let factor = matrix[j - 1][i] / matrix[i][i];
            for k in (0..size + 1).rev() {
                let tmp = matrix[i][k];
                matrix[j - 1][k] -= factor * tmp;
            }
        }
    }
}

pub struct EvaluationDomain<E>
where
  
    E: FieldElement
{
    max_degree: usize,
    evaluation_points: Vec<E>,
    barycentric_weights: Vec<E>,
}

impl<E> EvaluationDomain<E>
where
    E: FieldElement
{
    pub fn new(max_degree: usize) -> Self {
        let points: Vec<E> = (0..=max_degree).map(|x| E::from(x as u32)).collect();
        let weights = barycentric_weights(&points);

        Self {
            max_degree,
            evaluation_points: points,
            barycentric_weights: weights,
        }
    }

    pub fn evaluate(&self, evaluations: &[E], r: E) -> E {
        evaluate_barycentric(&self.evaluation_points, evaluations, r, &self.barycentric_weights)
    }
}

pub fn evaluate_barycentric<E: FieldElement>(
    x_i: &[E],
    y_i: &[E],
    r: E,
    barycentric_weights: &[E],
) -> E {
    for (&x_i, &y_i) in x_i.iter().zip(y_i.iter()) {
        if x_i == r {
            return y_i;
        }
    }

    let l_x: E = x_i.iter().fold(E::ONE, |acc, &x_i| acc * (r - x_i));

    let sum = (0..x_i.len()).fold(E::ZERO, |acc, i| {
        let w_i = barycentric_weights[i];
        acc + (w_i / (r - x_i[i]) * y_i[i])
    });

    l_x * sum
}

pub fn barycentric_weights<E: FieldElement>(points: &[E]) -> Vec<E> {
    let n = points.len();
    let tmp = (0..n)
        .map(|i| (0..n).filter(|&j| j != i).fold(E::ONE, |acc, j| acc * (points[i] - points[j])))
        .collect::<Vec<_>>();
    batch_inversion(&tmp)
}
