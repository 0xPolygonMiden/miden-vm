use criterion::{criterion_group, criterion_main, Criterion};
use miden_air::stack::op_flags::{generate_evaluation_frame, OpFlags};
use std::time::Duration;

fn compute_op_flags(c: &mut Criterion) {
    let mut group = c.benchmark_group("compute_op_flags");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("op_flags", |bench| {
        let frame = generate_evaluation_frame(36);
        bench.iter(|| {
            let _flag = OpFlags::new(&frame);
        });
    });

    group.finish();
}

criterion_group!(op_flags_group, compute_op_flags);
criterion_main!(op_flags_group);
