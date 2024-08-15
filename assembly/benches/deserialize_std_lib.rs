use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use miden_assembly::{
    ast::{Module, ModuleKind},
    diagnostics::{IntoDiagnostic, Report},
    testing::TestContext,
    Assembler, Deserializable, Library, LibraryPath, Serializable,
};
use vm_core::utils::SliceReader;

// TODO(serge): dedupe copy paste from library/tests.rs
macro_rules! parse_module {
    ($context:expr, $path:literal, $source:expr) => {{
        let path = LibraryPath::new($path).into_diagnostic()?;
        let source_file =
            $context.source_manager().load(concat!("test", line!()), $source.to_string());
        Module::parse(path, ModuleKind::Library, source_file)?
    }};
}

// TODO(serge): impl proper benchmark and remove return Result
fn deserialize_std_lib(c: &mut Criterion) -> Result<(), Report> {
    let context = TestContext::new();
    let foo = r#"
        export.foo
            add
        end
        export.foo_mul
            mul
        end
    "#;
    let foo = parse_module!(&context, "test::foo", foo);

    // declare bar module
    let bar = r#"
        export.bar
            mtree_get
        end
        export.bar_mul
            mul
        end
    "#;
    let bar = parse_module!(&context, "test::bar", bar);
    let modules = [foo, bar];

    let mut group = c.benchmark_group("compute_op_flags");
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("deserialize_std_lib", |bench| {
        bench.iter(|| {
            // Serialize
            let bundle = Assembler::new(context.source_manager())
                .assemble_library(modules.iter().cloned())
                .unwrap();

            // Deserialize
            let mut bytes = Vec::new();
            bundle.write_into(&mut bytes);
            let deserialized = Library::read_from(&mut SliceReader::new(&bytes)).unwrap();
            assert_eq!(bundle, deserialized);
        });
    });

    group.finish();
    Ok(())
}

// TODO(serge): fix clippy no use complaint
criterion_group!(std_lib_group, deserialize_std_lib);
criterion_main!(std_lib_group);
