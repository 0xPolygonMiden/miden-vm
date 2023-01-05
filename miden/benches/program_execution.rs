use criterion::{criterion_group, criterion_main, Criterion};
use miden::{execute, Assembler, BaseAdviceProvider};
use vm_core::StackInputs;
use std::time::Duration;
use stdlib::StdLibrary;

fn program_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("program_execution");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("sha256", |bench| {
        let source = "
            use.std::crypto::hashes::sha256

            begin
                exec.sha256::hash
            end";
        let assembler = Assembler::default()
            .with_library(&StdLibrary::default())
            .expect("failed to load stdlib");
        let program = assembler.compile(source).expect("Failed to compile test source.");
        let inputs = StackInputs::default();
        let advice = BaseAdviceProvider::default();
        // TODO clone here will impact performance
        bench.iter(|| execute(&program, advice.clone(), inputs.clone()));
    });

    group.finish();
}

criterion_group!(sha256_group, program_execution);
criterion_main!(sha256_group);
