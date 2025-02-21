use criterion::{criterion_group, criterion_main, Criterion};
use processor::fast::experiments::{
    fibonacci_hardcoded, FibOperation, FibonacciStackAsVec, FibonacciStackFixedLength,
    FibonacciStackFixedLengthNoSize,
};

const NUM_FIB_ITERATIONS: u32 = 1_000_000;

fn fib_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    group.bench_function("experiment 1: all hardcoded", |bench| {
        bench.iter(|| fibonacci_hardcoded(NUM_FIB_ITERATIONS));
    });
    group.bench_function("experiment 2: program hardcoded, stack as vec", |bench| {
        let mut processor = FibonacciStackAsVec::new();
        bench.iter(|| processor.fibonacci(NUM_FIB_ITERATIONS));
    });
    group.bench_function("experiment 2_1: explicit ops, stack as vec", |bench| {
        let ops = (0..NUM_FIB_ITERATIONS)
            .flat_map(|_| [FibOperation::Swap, FibOperation::Dup1, FibOperation::Add]);
        bench.iter_batched(
            || (FibonacciStackAsVec::new(), ops.clone()),
            |(processor, ops)| processor.fibonacci_explicit_ops_no_macro(ops),
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("experiment 3: stack fixed size WITH macro op", |bench| {
        let mut processor = FibonacciStackFixedLength::new();
        bench.iter(|| processor.fibonacci_with_macro_op(NUM_FIB_ITERATIONS));
    });

    group.bench_function("experiment 4: stack fixed size WITH macro op (version 2)", |bench| {
        let mut processor = FibonacciStackFixedLength::new();
        bench.iter(|| processor.fibonacci_with_macro_op_2(NUM_FIB_ITERATIONS));
    });

    group.bench_function("experiment 5: stack fixed size WITHOUT macro op", |bench| {
        let mut processor = FibonacciStackFixedLength::new();
        bench.iter(|| processor.fibonacci_no_macro_op(NUM_FIB_ITERATIONS));
    });

    group.bench_function("experiment 6: stack fixed size WITHOUT macro op (unchecked)", |bench| {
        let mut processor = FibonacciStackFixedLength::new();
        bench.iter(|| unsafe { processor.fibonacci_no_macro_op_unchecked(NUM_FIB_ITERATIONS) });
    });

    group.bench_function("experiment 7: stack no size WITHOUT macro op", |bench| {
        let processor = FibonacciStackFixedLengthNoSize::new();
        bench.iter_batched(
            || processor.clone(),
            |processor| processor.fibonacci_no_macro_op(NUM_FIB_ITERATIONS),
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("experiment 8: stack no size WITHOUT macro op (unchecked)", |bench| {
        let processor = FibonacciStackFixedLengthNoSize::new();
        bench.iter_batched(
            || processor.clone(),
            |processor| unsafe { processor.fibonacci_no_macro_op_unchecked(NUM_FIB_ITERATIONS) },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("experiment 9: explicit ops, no stack size in struct", |bench| {
        let ops = (0..NUM_FIB_ITERATIONS)
            .flat_map(|_| [FibOperation::Swap, FibOperation::Dup1, FibOperation::Add]);
        let processor = FibonacciStackFixedLengthNoSize::new();
        bench.iter_batched(
            || (processor.clone(), ops.clone()),
            |(processor, ops)| processor.fibonacci_explicit_ops_no_macro(ops),
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("experiment 10: explicit ops, stack size in struct", |bench| {
        let ops = (0..NUM_FIB_ITERATIONS)
            .flat_map(|_| [FibOperation::Swap, FibOperation::Dup1, FibOperation::Add]);
        bench.iter_batched(
            || (FibonacciStackFixedLength::new(), ops.clone()),
            |(processor, ops)| processor.fibonacci_explicit_ops_no_macro(ops),
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("experiment 11: explicit ops, stack size in struct, unchecked", |bench| {
        let ops = (0..NUM_FIB_ITERATIONS)
            .flat_map(|_| [FibOperation::Swap, FibOperation::Dup1, FibOperation::Add]);
        bench.iter_batched(
            || (FibonacciStackFixedLength::new(), ops.clone()),
            |(processor, ops)| unsafe { processor.fibonacci_explicit_ops_no_macro_unchecked(ops) },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(fib_group, fib_benches);
criterion_main!(fib_group);
