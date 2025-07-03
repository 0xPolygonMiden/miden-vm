use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use miden_vm::{Assembler, DefaultHost, StackInputs, internal::InputFile};
use processor::{AdviceInputs, fast::FastProcessor};
use stdlib::StdLibrary;
use walkdir::WalkDir;

/// Benchmark the execution of all the masm examples in the `masm-examples` directory.
fn program_execution_fast(c: &mut Criterion) {
    let mut group = c.benchmark_group("program_execution_fast");

    let masm_examples_dir = {
        let mut miden_dir = std::env::current_dir().unwrap();
        miden_dir.push("masm-examples");

        miden_dir
    };

    for entry in WalkDir::new(masm_examples_dir) {
        match &entry {
            Ok(entry) => {
                // if it's not a masm file, skip it.
                if !entry.file_type().is_file() || entry.path().extension().unwrap() != "masm" {
                    continue;
                }

                // if there's a `.inputs` file associated with this `.masm` file, use it as the
                // inputs.
                let (stack_inputs, advice_inputs) = match InputFile::read(&None, entry.path()) {
                    Ok(input_data) => {
                        let stack_inputs = input_data.parse_stack_inputs().unwrap();
                        let advice_inputs = input_data.parse_advice_inputs().unwrap();
                        (stack_inputs, advice_inputs)
                    },
                    Err(_) => (StackInputs::default(), AdviceInputs::default()),
                };

                // the name of the file without the extension
                let source = std::fs::read_to_string(entry.path()).unwrap();

                // Create a benchmark for the masm file
                let file_stem = entry.path().file_stem().unwrap().to_string_lossy();
                group.bench_function(file_stem, |bench| {
                    let mut assembler = Assembler::default();
                    assembler
                        .link_dynamic_library(StdLibrary::default())
                        .expect("failed to load stdlib");
                    let program = assembler
                        .assemble_program(&source)
                        .expect("Failed to compile test source.");
                    let stack_inputs: Vec<_> = stack_inputs.iter().rev().copied().collect();
                    bench.iter_batched(
                        || {
                            let mut host = DefaultHost::default();
                            host.load_mast_forest(
                                StdLibrary::default().as_ref().mast_forest().clone(),
                            )
                            .unwrap();
                            host
                        },
                        |mut host| {
                            let speedy = FastProcessor::new_with_advice_inputs(
                                &stack_inputs,
                                advice_inputs.clone(),
                            );
                            speedy.execute(&program, &mut host).unwrap();
                        },
                        BatchSize::SmallInput,
                    );
                });
            },
            // If we can't access the entry, just skip it
            Err(err) => {
                eprintln!("Failed to access file: {entry:?} with error {err:?}");
                continue;
            },
        }
    }

    group.finish();
}

criterion_group!(benchmark, program_execution_fast);
criterion_main!(benchmark);
