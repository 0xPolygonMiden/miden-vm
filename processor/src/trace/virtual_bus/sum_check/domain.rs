use alloc::vec::Vec;
use vm_core::FieldElement;
use winter_prover::math::batch_inversion;

/// Implements barycentric evaluation where the interpolation domain is `0, 1, ..., max_degree`
/// where `max_degree` is maximal polynomial supported degree.
pub struct EvaluationDomain<E>
where
    E: FieldElement,
{
    interpolation_points: Vec<E>,
    barycentric_weights: Vec<E>,
}

impl<E> EvaluationDomain<E>
where
    E: FieldElement,
{
    pub fn new(max_degree: u32) -> Self {
        let interpolation_points: Vec<E> = (0..=max_degree).map(E::from).collect();
        let barycentric_weights = barycentric_weights_denominators(&interpolation_points);

        Self {
            interpolation_points,
            barycentric_weights,
        }
    }

    pub fn evaluate(&self, evaluations: &[E], r: E) -> E {
        evaluate_barycentric(&self.interpolation_points, evaluations, r, &self.barycentric_weights)
    }
}

/// Computes the barycentric weights for a set of interpolation points.
pub fn barycentric_weights_denominators<E: FieldElement>(points: &[E]) -> Vec<E> {
    let n = points.len();
    let tmp = (0..n)
        .map(|i| (0..n).filter(|&j| j != i).fold(E::ONE, |acc, j| acc * (points[i] - points[j])))
        .collect::<Vec<_>>();
    batch_inversion(&tmp)
}

/// Computes the value of the polynomial with minimal degree interpolating the set `{(x_i, y_i)}`
/// at the point `r` given pre-computed barycentric weights.
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
