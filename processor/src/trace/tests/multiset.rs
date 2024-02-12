#[cfg(test)]
mod tests {
    use crate::trace::AuxColumnBuilder;
    use crate::{Felt, FieldElement};
    use miden_air::trace::main_trace::MainTrace;
    use vm_core::polynom::mul;
    use winter_prover::matrix::ColMatrix;

    struct MultisetTester<E: FieldElement<BaseField = Felt>> {
        multiplicands: Vec<E>,
        divisors: Vec<E>,
        alphas: Vec<E>,
    }

    impl<E: FieldElement<BaseField = Felt>> MultisetTester<E> {
        fn new(multiplicands: Vec<E>, divisors: Vec<E>) -> Self {
            Self {
                multiplicands,
                divisors,
                alphas: vec![Felt::new(83747374).into()],
            }
        }
    }

    impl<E: FieldElement<BaseField = Felt>> AuxColumnBuilder<E> for MultisetTester<E> {
        fn get_requests_at(&self, main_trace: &MainTrace, alphas: &[E], row_idx: usize) -> E {
            self.multiplicands[row_idx]
        }

        fn get_responses_at(&self, main_trace: &MainTrace, alphas: &[E], row_idx: usize) -> E {
            self.divisors[row_idx]
        }
    }

    fn main_trace_with_n_rows(n_rows: usize) -> MainTrace {
        let trace_columns = vec![(0..n_rows).fold(vec![], |mut acc, row| {
            acc.push(Felt::new(row as u64));
            acc
        })];
        MainTrace::new(ColMatrix::new(trace_columns))
    }

    #[test]
    fn multiset_check_valid_permutation() {
        let multiplicands = vec![Felt::new(1), Felt::new(2), Felt::new(3)];
        let divisors = vec![Felt::new(2), Felt::new(1), Felt::new(3)];
        let multiset_tester = MultisetTester::new(multiplicands, divisors);
        let aux_column = multiset_tester.build_aux_column(
            &main_trace_with_n_rows(4),
            multiset_tester.alphas.as_slice()
        );
        assert_eq!(aux_column.first().unwrap(), aux_column.last().unwrap());
    }

    #[test]
    fn multiset_check_invalid_permutation() {
        let multiplicands = vec![Felt::new(1), Felt::new(3), Felt::new(10)];
        let divisors = vec![Felt::new(4), Felt::new(3), Felt::new(10)];
        let multiset_tester = MultisetTester::new(multiplicands, divisors);
        let aux_column = multiset_tester.build_aux_column(
            &main_trace_with_n_rows(4),
            multiset_tester.alphas.as_slice(),
        );
        assert_ne!(aux_column.first().unwrap(), aux_column.last().unwrap());
    }

    #[test]
    fn multiset_check_invalid_permutation_same_grand_product() {
        // ensure that the multiset check is not just the grand product check
        // multiplicands and divisors are not permutations of one another
        // but have the same grand product
        let multiplicands = vec![Felt::new(1), Felt::new(3), Felt::new(5)];
        let divisors = vec![Felt::new(15), Felt::new(1), Felt::new(1)];
        let multiset_tester = MultisetTester::new(multiplicands, divisors);
        let aux_column = multiset_tester.build_aux_column(
            &main_trace_with_n_rows(4),
            multiset_tester.alphas.as_slice(),
        );
        assert_ne!(aux_column.first().unwrap(), aux_column.last().unwrap());
    }
}
