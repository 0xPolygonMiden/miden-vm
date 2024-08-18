use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

fn deserialize_std_lib(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize_std_lib");
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("read_from_bytes", |bench| {
        bench.iter(|| {
            // TODO(serge): impl
        });
    });

    group.finish();
}

criterion_group!(std_lib_group, deserialize_std_lib);
criterion_main!(std_lib_group);
