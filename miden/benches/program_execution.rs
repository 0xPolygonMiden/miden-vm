use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use miden_vm::{Assembler, DefaultHost, StackInputs};
use processor::{execute, ExecutionOptions};
use stdlib::StdLibrary;

fn program_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("program_execution");
    group.measurement_time(Duration::from_secs(10));

    let stdlib = StdLibrary::default();
    let mut host = DefaultHost::default();
    host.load_mast_forest(stdlib.as_ref().mast_forest().clone());

    group.bench_function("sha256", |bench| {
        let source = "
            use.std::crypto::hashes::sha256

            begin
                exec.sha256::hash_2to1
            end";
        let mut assembler = Assembler::default();
        assembler.add_library(&stdlib).expect("failed to load stdlib");
        let program = assembler.assemble_program(source).expect("Failed to compile test source.");
        bench.iter(|| {
            execute(&program, StackInputs::default(), &mut host, ExecutionOptions::default())
        });
    });

    group.finish();
}

criterion_group!(sha256_group, program_execution);
criterion_main!(sha256_group);
