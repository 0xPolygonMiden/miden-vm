use std::{path::Path, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};
use miden_assembly::Library;

fn deserialize_std_lib(c: &mut Criterion) {
    let mut group = c.benchmark_group("compute_op_flags");
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("deserialize_std_lib", |bench| {
        bench.iter(|| {
            let asm_dir = Path::new(manifest_dir).join("..").join("stdlib").join("asm");
            let paths = [
                asm_dir.clone().join("collections").join("mmr.masm"),
                // TODO(serge): Figure out how to create .masl instead
            ];
            for path in paths {
                let _ = Library::deserialize_from_file(path).unwrap();
            }
        });
    });

    group.finish();
}

criterion_group!(std_lib_group, deserialize_std_lib);
criterion_main!(std_lib_group);
