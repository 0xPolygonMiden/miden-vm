use criterion::{criterion_group, criterion_main, Criterion};
use miden_assembly::{self, Assembler};
use std::time::Duration;

fn script_compilation(c: &mut Criterion) {
    let mut group = c.benchmark_group("script_compilation");
    group.measurement_time(Duration::from_secs(300));

    group.bench_function("sha256", |bench| {
        let source = "
            use.std::crypto::hashes::sha256

            begin
                exec.sha256::hash
            end";
        bench.iter(|| {
            let assembler = Assembler::new();
            assembler
                .compile_script(source)
                .expect("Failed to compile test source.")
        });
    });

    group.finish();
}

criterion_group!(sha256_group, script_compilation);
criterion_main!(sha256_group);
