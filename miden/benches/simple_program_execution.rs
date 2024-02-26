use criterion::{criterion_group, criterion_main, Criterion};
use miden::{Assembler, DefaultHost, StackInputs};
use processor::{ExecutionOptions, Process};
use std::time::Duration;
use stdlib::StdLibrary;

fn simple_program_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_program_execution");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("simple ops", |bench| {
        let source = "
            begin
                push.0 push.1 push.2 push.3 push.4 push.5 push.6 push.7 
                add mul add mul add mul add mul add mul add mul add mul 
                push.0 push.1 push.2 push.3 push.4 push.5 push.6 push.7 
                add mul add mul add mul add mul add mul add mul add mul 
                push.0 push.1 push.2 push.3 push.4 push.5 push.6 push.7 
                add mul add mul add mul add mul add mul add mul add mul 
                push.0 push.1 push.2 push.3 push.4 push.5 push.6 push.7 
                add mul add mul add mul add mul add mul add mul add mul 
                push.0 push.1 push.2 push.3 push.4 push.5 push.6 push.7 
                add mul add mul add mul add mul add mul add mul add mul 
                push.0 push.1 push.2 push.3 push.4 push.5 push.6 push.7 
                add mul add mul add mul add mul add mul add mul add mul 
            end";
        let assembler = Assembler::default()
            .with_library(&StdLibrary::default())
            .expect("failed to load stdlib");
        let program = assembler.compile(source).expect("Failed to compile test source.");
        bench.iter(|| {
            let mut process = Process::new(
                program.kernel().clone(),
                StackInputs::default(),
                DefaultHost::default(),
                ExecutionOptions::default(),
            );
            let _ = process.execute(&program).unwrap();
        });
    });

    group.finish();
}

criterion_group!(simple_execution_group, simple_program_execution);
criterion_main!(simple_execution_group);
