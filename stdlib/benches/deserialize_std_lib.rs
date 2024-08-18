use std::time::Duration;

use assembly::{utils::Deserializable, Library};
use criterion::{criterion_group, criterion_main, Criterion};

fn deserialize_std_lib(c: &mut Criterion) {
    // Load the standard library from the compiled assets.
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/assets/std.masl"));

    // Setup and run the benchmarks.
    let mut group = c.benchmark_group("deserialize_std_lib");
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("read_from_bytes", |bench| {
        bench.iter(|| {
            let _ = Library::read_from_bytes(bytes).expect("failed to read std masl!");
        });
    });

    group.finish();
}

criterion_group!(std_lib_group, deserialize_std_lib);
criterion_main!(std_lib_group);
