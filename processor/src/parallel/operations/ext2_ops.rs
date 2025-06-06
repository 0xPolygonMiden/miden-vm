use super::MainTraceFragmentGenerator;

impl MainTraceFragmentGenerator {
    /// Computes the product of two elements in the extension field of degree 2.
    pub(crate) fn ext2mul(&mut self) {
        // Get elements from stack: [b1, b0, a1, a0, ...]
        let a0 = self.stack_get(0);
        let a1 = self.stack_get(1);
        let b0 = self.stack_get(2);
        let b1 = self.stack_get(3);

        // Compute the quadratic extension multiplication
        // For elements a = a0 + a1 * u and b = b0 + b1 * u where u^2 = -1
        // Result c = a * b = (a0*b0 - a1*b1) + (a0*b1 + a1*b0) * u
        let c0 = a0 * b0 - a1 * b1;
        let c1 = a0 * b1 + a1 * b0;

        // Set result on stack: [c1, c0, ...]
        self.stack_set(0, c0);
        self.stack_set(1, c1);

        // Shift stack to remove consumed elements
        self.stack_shift_left(2);
    }
}
