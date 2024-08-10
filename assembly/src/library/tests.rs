use alloc::{string::ToString, vec::Vec};

use vm_core::utils::SliceReader;

use super::*;
use crate::{
    ast::{Module, ModuleKind, ProcedureName},
    diagnostics::{IntoDiagnostic, Report},
    testing::TestContext,
    Assembler, Deserializable,
};

macro_rules! parse_module {
    ($context:expr, $path:literal, $source:expr) => {{
        let path = LibraryPath::new($path).into_diagnostic()?;
        let source_file =
            $context.source_manager().load(concat!("test", line!()), $source.to_string());
        Module::parse(path, ModuleKind::Library, source_file)?
    }};
}

#[test]
fn library_serialization() -> Result<(), Report> {
    let context = TestContext::new();
    // declare foo module
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

    // serialize/deserialize the bundle with locations
    let bundle = Assembler::new(context.source_manager())
        .assemble_library(modules.iter().cloned())
        .unwrap();

    let mut bytes = Vec::new();
    bundle.write_into(&mut bytes);
    let deserialized = Library::read_from(&mut SliceReader::new(&bytes)).unwrap();
    assert_eq!(bundle, deserialized);

    Ok(())
}

#[test]
fn get_module_by_path() -> Result<(), Report> {
    let context = TestContext::new();
    // declare foo module
    let foo_source = r#"
        export.foo
            add
        end
    "#;
    let foo = parse_module!(&context, "test::foo", foo_source);
    let modules = [foo];

    // create the bundle with locations
    let bundle = Assembler::new(context.source_manager())
        .assemble_library(modules.iter().cloned())
        .unwrap();

    let foo_module_info = bundle.module_infos().next().unwrap();
    assert_eq!(foo_module_info.path(), &LibraryPath::new("test::foo").unwrap());

    let (_, foo_proc) = foo_module_info.procedures().next().unwrap();
    assert_eq!(foo_proc.name, ProcedureName::new("foo").unwrap());

    Ok(())
}
