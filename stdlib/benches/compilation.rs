use std::{path::Path, time::Duration};

use assembly::{Assembler, Library, LibraryExt, LibraryNamespace};
use criterion::{Criterion, criterion_group, criterion_main};

fn stdlib_compilation(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_stdlib");
    group.measurement_time(Duration::from_secs(10));

    // Compiles the entire standard library
    group.bench_function("all", |bench| {
        bench.iter(|| {
            let assembler = Assembler::default();

            let manifest_dir = env!("CARGO_MANIFEST_DIR");
            let asm_dir = Path::new(manifest_dir).join("asm");
            let namespace = "std".parse::<LibraryNamespace>().expect("invalid base namespace");
            Library::from_dir(asm_dir, namespace, assembler).unwrap();
        });
    });

    group.finish();
}

criterion_group!(compilation_group, stdlib_compilation);
criterion_main!(compilation_group);
