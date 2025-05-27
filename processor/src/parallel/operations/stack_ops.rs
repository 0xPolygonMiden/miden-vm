use vm_core::{ONE, ZERO};

use super::CoreTraceFragmentGenerator;

impl CoreTraceFragmentGenerator {
    /// Pushes 0 onto the stack.
    pub(crate) fn pad(&mut self) {
        self.stack_shift_right(1);
        self.stack_set(0, ZERO);
    }

    /// Pops an element off the stack.
    pub(crate) fn drop(&mut self) {
        self.stack_shift_left(1);
    }

    /// Pushes a copy of the n-th element to the top of the stack.
    pub(crate) fn dup(&mut self, n: usize) {
        let value = self.stack_get(n);
        self.stack_shift_right(1);
        self.stack_set(0, value);
    }

    /// Swaps the top two elements on the stack.
    pub(crate) fn swap(&mut self) {
        let a = self.stack_get(0);
        let b = self.stack_get(1);
        self.stack_set(0, b);
        self.stack_set(1, a);
    }

    /// Swaps the top two words on the stack.
    pub(crate) fn swapw(&mut self) {
        for i in 0..4 {
            let a = self.stack_get(i);
            let b = self.stack_get(i + 4);
            self.stack_set(i, b);
            self.stack_set(i + 4, a);
        }
    }

    /// Swaps the top word with the word starting at position 8.
    pub(crate) fn swapw2(&mut self) {
        for i in 0..4 {
            let a = self.stack_get(i);
            let b = self.stack_get(i + 8);
            self.stack_set(i, b);
            self.stack_set(i + 8, a);
        }
    }

    /// Swaps the top word with the word starting at position 12.
    pub(crate) fn swapw3(&mut self) {
        for i in 0..4 {
            let a = self.stack_get(i);
            let b = self.stack_get(i + 12);
            self.stack_set(i, b);
            self.stack_set(i + 12, a);
        }
    }

    /// Swaps the top two double words on the stack.
    pub(crate) fn swapdw(&mut self) {
        for i in 0..8 {
            let a = self.stack_get(i);
            let b = self.stack_get(i + 8);
            self.stack_set(i, b);
            self.stack_set(i + 8, a);
        }
    }

    /// Moves the n-th element to the top of the stack.
    pub(crate) fn movup(&mut self, n: usize) {
        let value = self.stack_get(n);
        // Shift elements down by 1 position
        for i in (1..=n).rev() {
            self.stack_set(i, self.stack_get(i - 1));
        }
        self.stack_set(0, value);
    }

    /// Moves the top element down to the n-th position.
    pub(crate) fn movdn(&mut self, n: usize) {
        let value = self.stack_get(0);
        // Shift elements up by 1 position
        for i in 0..n {
            self.stack_set(i, self.stack_get(i + 1));
        }
        self.stack_set(n, value);
    }

    /// Conditionally swaps the top two elements based on the third element.
    pub(crate) fn cswap(&mut self) {
        let c = self.stack_get(0);
        let b = self.stack_get(1);
        let a = self.stack_get(2);

        if c == ONE {
            self.stack_set(1, a);
            self.stack_set(2, b);
        }

        self.stack_shift_left(1);
    }

    /// Conditionally swaps the top two words based on the fifth element.
    pub(crate) fn cswapw(&mut self) {
        let c = self.stack_get(0);

        if c == ONE {
            for i in 0..4 {
                let a = self.stack_get(i + 1);
                let b = self.stack_get(i + 5);
                self.stack_set(i + 1, b);
                self.stack_set(i + 5, a);
            }
        }

        self.stack_shift_left(1);
    }
}
