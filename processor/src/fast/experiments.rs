#![allow(clippy::missing_safety_doc)]
//! Temporary module to test "theoretical" performance bounds we can achieve for fibonacci

use alloc::vec::Vec;

use vm_core::{Felt, ONE, ZERO};

/// A reduced set of operations relevant to the fibonacci computation.
#[derive(Clone, Debug)]
pub enum FibOperation {
    Swap,
    Dup1,
    Add,
}

/// This experiment hardcodes the program to be fibonacci, and hardcodes the "stack" to be the 2
/// registers needed for the algorithm. We can think of this benchmark as the "theoretical" maximum
/// performance we can achieve for fibonacci, since we hardcode the program (hence removing any
/// overhead of traversing the MAST), and we hardcode the stack (hence removing any overhead of
/// managing the stack) - let alone having no memory or advice provider. Additionally, we perform
/// the fibonacci computation as a "macro-op" - i.e. we don't break it down into swap, dup.1, add
/// operations.
/// *Correction*: we learned with `fibonacci_with_macro_op_2` that we can achieve even better
/// performance by breaking down the computation into swap, dup.1, add operations.
///
/// To compute the "logical VM MHz" for this, we consider each fibonacci iteration to be 3 "VM
/// cycles" (swap dup.1 add). The VM also has 45 overhead cycles on top of that (for join, end,
/// etc). Therefore, for example, we consider 1000 iterations of fibonacci to be 3045 VM cycles (3
/// cycles/iteration * 1000 iterations + 45 overhead cycles).
///
/// Benchmark result: 2560 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 1.17 ms)
pub fn fibonacci_hardcoded(n: u32) -> Felt {
    let mut a = ZERO;
    let mut b = ONE;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

/// This experiment hardcodes the program to be fibonacci, but manages the stack as a vector of
/// Felt values. This is a more realistic scenario than `fibonacci_hardcoded`, but still removes
/// the overhead of traversing the MAST.
///
/// Note that `Vec` and `SmallVec` have the same performance on this benchmark.
pub struct FibonacciStackAsVec {
    stack: Vec<Felt>,
}

impl FibonacciStackAsVec {
    pub fn new() -> Self {
        Self { stack: vec![ZERO; 16] }
    }

    /// This experiment is similar to `fibonacci_with_macro_op`, but we use a `Vec` to manage the
    /// stack. Even though both *look* similar, using a `Vec` instead of an array results in a 2x
    /// performance drop.
    ///
    /// Benchmark result: 1250 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 2.44 ms)
    pub fn fibonacci(&mut self, n: u32) -> Felt {
        self.stack.push(ZERO);
        self.stack.push(ONE);

        let index_a = self.stack.len() - 2;
        let index_b = self.stack.len() - 1;
        assert!(index_a < self.stack.len() && index_b < self.stack.len(), "stack is too large");

        for _ in 0..n {
            let a = self.stack[index_a];
            let b = self.stack[index_b];
            self.stack[index_b] = a + b;
            self.stack[index_a] = b;
        }
        self.stack[index_b]
    }

    /// We see that we lose ~185 MHz compared to the fastest alternative (i.e. the 571 MHz from
    /// `FibonacciStackFixedLength`) by using `Vec` push/pop compared to direct array access.
    ///
    /// Benchmark result: 387 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 7.75 ms)
    pub fn fibonacci_explicit_ops_no_macro(
        mut self,
        ops: impl Iterator<Item = FibOperation>,
    ) -> Felt {
        let size = self.stack.len();
        self.stack[size - 2] = ZERO;
        self.stack[size - 1] = ONE;

        for op in ops {
            match op {
                FibOperation::Swap => self.swap(),
                FibOperation::Dup1 => self.dup1(),
                FibOperation::Add => self.add(),
            };
        }

        self.stack[self.stack.len() - 1]
    }

    // HELPER FUNCTIONS
    // ------------------------------------------------------------------------------------------

    #[inline(always)]
    fn swap(&mut self) {
        let size = self.stack.len();
        let a = self.stack[size - 1];
        let b = self.stack[size - 2];
        self.stack[size - 1] = b;
        self.stack[size - 2] = a;
    }

    #[inline(always)]
    fn dup1(&mut self) {
        self.stack.push(self.stack[self.stack.len() - 2]);
    }

    #[inline(always)]
    fn add(&mut self) {
        let top = self.stack.pop().unwrap_or(ZERO);
        *self.stack.last_mut().unwrap() += top;
    }
}

impl Default for FibonacciStackAsVec {
    fn default() -> Self {
        Self::new()
    }
}

pub const N: usize = 512;

/// This set of experiments hardcodes the program to be fibonacci, but manages the stack as a fixed
/// length array. This is a more realistic-looking scenario than `fibonacci_hardcoded`, but still
/// removes the overhead of traversing the MAST. Yet, it would only allow us to run programs who
/// don't need a stack size of more than `N`.
pub struct FibonacciStackFixedLength {
    stack: [Felt; N],
    size: usize,
}

impl FibonacciStackFixedLength {
    pub fn new() -> Self {
        Self { stack: [ZERO; N], size: 16 }
    }

    // EXPERIMENTS
    // ------------------------------------------------------------------------------------------

    /// In this experiment, we use a macro-op to perform the fibonacci computation. This means that
    /// we don't break down the fibonacci computation into swap, dup.1, add operations, but rather
    /// perform the entire computation in one go.
    ///
    /// As expected, this yields the same performance as `fibonacci_hardcoded`.
    ///
    /// Benchmark result: 2600 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 1.17 ms)
    pub fn fibonacci_with_macro_op(&mut self, n: u32) -> Felt {
        // Note: this assert is important, as it tells the compiler that all accesses to the stack
        // are within bounds, and therefore it doesn't inject bound checks in the hot loop.
        // Removing this assert causes the performance to *drop by half*.
        let index_a = self.size - 2;
        let index_b = self.size - 1;
        assert!(index_a < N && index_b < N, "stack is too large");

        self.stack[index_a] = ZERO;
        self.stack[index_b] = ONE;

        for _ in 0..n {
            let a = self.stack[index_a];
            let b = self.stack[index_b];
            self.stack[index_b] = a + b;
            self.stack[index_a] = b;
        }
        self.stack[index_b]
    }

    /// Similar to `fibonacci_with_macro_op`, but we use a different way to perform the fibonacci
    /// computation. Note that even though this doesn't look like a macro-op (since we break down
    /// the computation into swap, dup.1, add operations), it really is one, specifically because we
    /// don't update `self.stack_size` in-between operations, which allows the compiler to remove
    /// bounds check.
    ///
    /// Notice that this version is quite faster, despite having more operations than
    /// `fibonacci_with_macro_op`. This is probably because we remove data dependencies between
    /// operations, which allows the CPU to pipeline them.
    ///
    /// Benchmark result: 3300 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 921 us)
    pub fn fibonacci_with_macro_op_2(&mut self, n: u32) -> Felt {
        let index_top = self.size;
        let index_a = self.size - 2;
        let index_b = self.size - 1;
        // Note: this assert results in a 5x improvement in performance.
        assert!(index_a < N && index_b < N && index_top < N, "stack is too large");

        self.stack[index_a] = ZERO;
        self.stack[index_b] = ONE;

        for _ in 0..n {
            // swap
            let a = self.stack[index_b];
            let b = self.stack[index_a];
            self.stack[index_b] = b;
            self.stack[index_a] = a;

            // dup1
            self.stack[index_top] = self.stack[index_a];

            // add
            let a = self.stack[index_top];
            let b = self.stack[index_b];
            self.stack[index_b] = a + b;
        }
        self.stack[index_b]
    }

    /// This experiment is similar to `fibonacci_with_macro_op_2`, but we don't use a macro-op to
    /// perform the fibonacci computation. This means that we break down the fibonacci computation
    /// into swap, dup.1, add operations *without knowing what comes next*, which forces us to
    /// update the `self.size` parameter, and hence the compiler needs to insert bound checks.
    ///
    /// Benchmark results: 590 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 5.13 ms)
    pub fn fibonacci_no_macro_op(&mut self, n: u32) -> Felt {
        self.stack[self.size - 2] = ZERO;
        self.stack[self.size - 1] = ONE;

        for _ in 0..n {
            self.swap();
            self.dup1();
            self.add();
        }
        self.stack[self.size - 1]
    }

    /// This experiment is similar to `fibonacci_no_macro_op`, but we use unchecked stack accesses
    /// to avoid bound checks.
    ///
    /// This ends up being the same performance as `fibonacci_no_macro_op`. This suggests that the
    /// poor performance of `fibonacci_no_macro_op` is not due to bound checks, but rather due to
    /// the fact that we use a different value for `self.size` in-between operations.
    ///
    /// Benchmark results: 590 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 5.13 ms)
    pub unsafe fn fibonacci_no_macro_op_unchecked(&mut self, n: u32) -> Felt {
        *self.stack.get_unchecked_mut(self.size - 2) = ZERO;
        *self.stack.get_unchecked_mut(self.size - 1) = ONE;

        for _ in 0..n {
            self.swap_unchecked();
            self.dup1_unchecked();
            self.add_unchecked();
        }
        *self.stack.get_unchecked(self.size - 1)
    }

    /// This is the same as the one in FibonacciStackFixedLengthNoSize.
    ///
    /// Interestingly, this version is faster than the one in FibonacciStackFixedLength. This is
    /// surprising, since using `self.size` mutably is slower when the program is known in advance,
    /// but it's faster when the program isn't.
    ///
    /// Benchmark results: 571 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 5.25ms)
    pub fn fibonacci_explicit_ops_no_macro(
        mut self,
        ops: impl Iterator<Item = FibOperation>,
    ) -> Felt {
        self.stack[self.size - 2] = ZERO;
        self.stack[self.size - 1] = ONE;

        for op in ops {
            match op {
                FibOperation::Swap => self.swap(),
                FibOperation::Dup1 => self.dup1(),
                FibOperation::Add => self.add(),
            };
        }

        self.stack[self.size - 1]
    }

    /// This is the same as `fibonacci_explicit_ops_no_macro`, but we use unchecked stack accesses.
    ///
    /// Interestingly, this version is slightly *slower*. I am not sure why.
    ///
    /// Benchmark results: 537 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 5.58ms)
    pub unsafe fn fibonacci_explicit_ops_no_macro_unchecked(
        mut self,
        ops: impl Iterator<Item = FibOperation>,
    ) -> Felt {
        *self.stack.get_unchecked_mut(self.size - 2) = ZERO;
        *self.stack.get_unchecked_mut(self.size - 1) = ONE;

        for op in ops {
            match op {
                FibOperation::Swap => self.swap_unchecked(),
                FibOperation::Dup1 => self.dup1_unchecked(),
                FibOperation::Add => self.add_unchecked(),
            };
        }

        *self.stack.get_unchecked(self.size - 1)
    }

    // HELPER FUNCTIONS
    // ------------------------------------------------------------------------------------------

    #[inline(always)]
    fn swap(&mut self) {
        let a = self.stack[self.size - 1];
        let b = self.stack[self.size - 2];
        self.stack[self.size - 1] = b;
        self.stack[self.size - 2] = a;
    }

    unsafe fn swap_unchecked(&mut self) {
        let a = *self.stack.get_unchecked(self.size - 1);
        let b = *self.stack.get_unchecked(self.size - 2);
        *self.stack.get_unchecked_mut(self.size - 1) = b;
        *self.stack.get_unchecked_mut(self.size - 2) = a;
    }

    #[inline(always)]
    fn dup1(&mut self) {
        self.stack[self.size] = self.stack[self.size - 2];
        self.size += 1;
    }

    unsafe fn dup1_unchecked(&mut self) {
        *self.stack.get_unchecked_mut(self.size) = *self.stack.get_unchecked(self.size - 2);
        self.size += 1;
    }

    #[inline(always)]
    fn add(&mut self) {
        let a = self.stack[self.size - 1];
        let b = self.stack[self.size - 2];
        self.stack[self.size - 2] = a + b;
        self.size -= 1;
    }

    unsafe fn add_unchecked(&mut self) {
        let a = *self.stack.get_unchecked(self.size - 1);
        let b = *self.stack.get_unchecked(self.size - 2);
        *self.stack.get_unchecked_mut(self.size - 2) = a + b;
        self.size -= 1;
    }
}

impl Default for FibonacciStackFixedLength {
    fn default() -> Self {
        Self::new()
    }
}

/// This is a version of `FibonacciStackFixedLength` that doesn't maintain the size of the stack
/// inside the struct.
///
/// This is to see if we can achieve the performance of `fibonacci_with_macro_op`, which the
/// compiler is able to optimize better because it sees the relationship between the stack accesses.
/// This is an attempt to fix the performance drop in `FibonacciStackFixed::fibonacci_no_macro_op`.
///
/// Benchmark results confirm that this version achieves the same performance as
/// `fibonacci_with_macro_op`, and hence we should be passing the stack size as a parameter to the
/// internal functions.
#[derive(Clone)]
pub struct FibonacciStackFixedLengthNoSize {
    stack: [Felt; N],
}

impl FibonacciStackFixedLengthNoSize {
    pub fn new() -> Self {
        Self { stack: [ZERO; N] }
    }

    /// This experiment is similar to `FibonacciStackFixedLength::fibonacci_no_macro_op` (experiment
    /// 5), but we maintain the stack size outside the struct. Specifically, `self.stack_size` is no
    /// longer a variable that we mutate; instead, we pass it as a parameter to the internal
    /// functions and return the updated value.
    ///
    /// Benchmark results confirms that not mutating `self.size` results in a 5-6x performance
    /// boost.
    ///
    /// Benchmark results: 3300 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 921 us)
    pub fn fibonacci_no_macro_op(mut self, n: u32) -> Felt {
        let mut size = 16;
        self.stack[size - 2] = ZERO;
        self.stack[size - 1] = ONE;

        for _ in 0..n {
            size = self.swap(size);
            size = self.dup1(size);
            size = self.add(size);
        }
        self.stack[size - 1]
    }

    /// This is the same as `fibonacci_no_macro_op`, but we use unchecked stack accesses to avoid
    /// bound checks.
    ///
    /// This ends up being the same performance as `fibonacci_no_macro_op`. This suggests that the
    /// bounds checks are negligible in this case.
    ///
    /// Benchmark results: 3300 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 921 us)
    pub unsafe fn fibonacci_no_macro_op_unchecked(mut self, n: u32) -> Felt {
        let mut size = 16;
        *self.stack.get_unchecked_mut(size - 2) = ZERO;
        *self.stack.get_unchecked_mut(size - 1) = ONE;

        for _ in 0..n {
            size = self.swap_unchecked(size);
            size = self.dup1_unchecked(size);
            size = self.add_unchecked(size);
        }
        *self.stack.get_unchecked(size - 1)
    }

    /// This is the same as `fibonacci_no_macro_op`, but we pass in the operations explicitly.
    ///
    /// From the benchmark results, we see that this version is drastically slower than the other
    /// versions. This is because at each iteration, we don't know which operation comes next, and
    /// hence the compiler can't optimize the code nearly as much (+ all the added branches).
    ///
    /// Benchmark results: 387 MHz (for n=1_000_000, we have 3_000_000 "cycles" in 7.25ms)
    pub fn fibonacci_explicit_ops_no_macro(
        mut self,
        ops: impl Iterator<Item = FibOperation>,
    ) -> Felt {
        let mut size = 16;
        self.stack[size - 2] = ZERO;
        self.stack[size - 1] = ONE;

        for op in ops {
            size = match op {
                FibOperation::Swap => self.swap(size),
                FibOperation::Dup1 => self.dup1(size),
                FibOperation::Add => self.add(size),
            };
        }

        self.stack[size - 1]
    }

    // HELPER FUNCTIONS
    // ------------------------------------------------------------------------------------------

    #[inline(always)]
    fn swap(&mut self, size: usize) -> usize {
        let a = self.stack[size - 1];
        let b = self.stack[size - 2];
        self.stack[size - 1] = b;
        self.stack[size - 2] = a;

        size
    }

    unsafe fn swap_unchecked(&mut self, size: usize) -> usize {
        let a = *self.stack.get_unchecked(size - 1);
        let b = *self.stack.get_unchecked(size - 2);
        *self.stack.get_unchecked_mut(size - 1) = b;
        *self.stack.get_unchecked_mut(size - 2) = a;

        size
    }

    #[inline(always)]
    fn dup1(&mut self, size: usize) -> usize {
        self.stack[size] = self.stack[size - 2];
        size + 1
    }

    unsafe fn dup1_unchecked(&mut self, size: usize) -> usize {
        *self.stack.get_unchecked_mut(size) = *self.stack.get_unchecked(size - 2);
        size + 1
    }

    #[inline(always)]
    fn add(&mut self, size: usize) -> usize {
        let a = self.stack[size - 1];
        let b = self.stack[size - 2];
        self.stack[size - 2] = a + b;
        size - 1
    }

    unsafe fn add_unchecked(&mut self, size: usize) -> usize {
        let a = *self.stack.get_unchecked(size - 1);
        let b = *self.stack.get_unchecked(size - 2);
        *self.stack.get_unchecked_mut(size - 2) = a + b;
        size - 1
    }
}

impl Default for FibonacciStackFixedLengthNoSize {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
/// Make sure that all the experiments run the same computation.
fn test_fibonacci_consistency() {
    const NUM_FIB_ITERATIONS: u32 = 1000;
    let result_1 = fibonacci_hardcoded(NUM_FIB_ITERATIONS);
    let result_2 = FibonacciStackAsVec::new().fibonacci(NUM_FIB_ITERATIONS);
    let result_2_1 = {
        let ops = (0..NUM_FIB_ITERATIONS)
            .flat_map(|_| [FibOperation::Swap, FibOperation::Dup1, FibOperation::Add]);
        FibonacciStackAsVec::new().fibonacci_explicit_ops_no_macro(ops)
    };
    let result_3 = FibonacciStackFixedLength::new().fibonacci_with_macro_op(NUM_FIB_ITERATIONS);
    let result_4 = FibonacciStackFixedLength::new().fibonacci_with_macro_op_2(NUM_FIB_ITERATIONS);
    let result_5 = unsafe {
        FibonacciStackFixedLength::new().fibonacci_no_macro_op_unchecked(NUM_FIB_ITERATIONS)
    };
    let result_6 = FibonacciStackFixedLength::new().fibonacci_no_macro_op(NUM_FIB_ITERATIONS);
    let result_7 = FibonacciStackFixedLengthNoSize::new().fibonacci_no_macro_op(NUM_FIB_ITERATIONS);
    let result_8 = unsafe {
        FibonacciStackFixedLengthNoSize::new().fibonacci_no_macro_op_unchecked(NUM_FIB_ITERATIONS)
    };

    let result_9 = {
        let ops = (0..NUM_FIB_ITERATIONS)
            .flat_map(|_| [FibOperation::Swap, FibOperation::Dup1, FibOperation::Add]);
        FibonacciStackFixedLengthNoSize::new().fibonacci_explicit_ops_no_macro(ops)
    };

    let result_10 = {
        let ops = (0..NUM_FIB_ITERATIONS)
            .flat_map(|_| [FibOperation::Swap, FibOperation::Dup1, FibOperation::Add]);
        FibonacciStackFixedLength::new().fibonacci_explicit_ops_no_macro(ops)
    };

    let result_11 = {
        let ops = (0..NUM_FIB_ITERATIONS)
            .flat_map(|_| [FibOperation::Swap, FibOperation::Dup1, FibOperation::Add]);
        unsafe { FibonacciStackFixedLength::new().fibonacci_explicit_ops_no_macro_unchecked(ops) }
    };

    assert_eq!(result_1, result_2);
    assert_eq!(result_1, result_2_1);
    assert_eq!(result_1, result_3);
    assert_eq!(result_1, result_4);
    assert_eq!(result_1, result_5);
    assert_eq!(result_1, result_6);
    assert_eq!(result_1, result_7);
    assert_eq!(result_1, result_8);
    assert_eq!(result_1, result_9);
    assert_eq!(result_1, result_10);
    assert_eq!(result_1, result_11);
}
