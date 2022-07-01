use criterion::{criterion_group, criterion_main, Criterion};
use miden_assembly::Assembler;
use miden_processor::execute;
use std::time::Duration;
use vm_core::ProgramInputs;

fn script_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("script_execution");
    group.measurement_time(Duration::from_secs(300));

    group.bench_function("sha256", |bench| {
        let source = "
            use.std::crypto::hashes::sha256

            begin
                exec.sha256::hash
            end";
        let assembler = Assembler::new();
        let script = assembler
            .compile_script(source)
            .expect("Failed to compile test source.");
        bench.iter(|| execute(&script, &ProgramInputs::none()));
    });

    group.finish();
}

criterion_group!(sha256_group, script_execution);
criterion_main!(sha256_group);
