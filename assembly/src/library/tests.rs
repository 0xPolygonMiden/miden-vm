use alloc::{string::ToString, sync::Arc, vec::Vec};

use vm_core::utils::SliceReader;

use super::LibraryPath;
use crate::{
    ast::{AstSerdeOptions, Module, ModuleKind, ProcedureName},
    diagnostics::{IntoDiagnostic, Report, SourceFile},
    library::CompiledLibrary,
    testing::TestContext,
    Assembler,
};

macro_rules! parse_module {
    ($path:literal, $source:expr) => {{
        let path = LibraryPath::new($path).into_diagnostic()?;
        let source_file = Arc::from(SourceFile::new(concat!("test", line!()), $source.to_string()));
        Module::parse(path, ModuleKind::Library, source_file)?
    }};
}

#[test]
fn masl_locations_serialization() -> Result<(), Report> {
    let _context = TestContext::new();
    // declare foo module
    let foo = r#"
        export.foo
            add
        end
        export.foo_mul
            mul
        end
    "#;
    let foo = parse_module!("test::foo", foo);

    // declare bar module
    let bar = r#"
        export.bar
            mtree_get
        end
        export.bar_mul
            mul
        end
    "#;
    let bar = parse_module!("test::bar", bar);
    let modules = vec![foo, bar];

    // serialize/deserialize the bundle with locations
    let bundle = Assembler::default().assemble_library(modules.iter().cloned()).unwrap();

    let mut bytes = Vec::new();
    bundle.write_into_with_options(&mut bytes, AstSerdeOptions::new(true, true));
    let deserialized = CompiledLibrary::read_from_with_options(
        &mut SliceReader::new(&bytes),
        AstSerdeOptions::new(true, false),
    )
    .unwrap();
    assert_eq!(bundle, deserialized);

    // serialize/deserialize the bundle without locations
    let bundle = Assembler::default().assemble_library(modules.iter().cloned()).unwrap();

    // serialize/deserialize the bundle
    let mut bytes = Vec::new();
    bundle.write_into_with_options(&mut bytes, AstSerdeOptions::new(true, false));
    let deserialized = CompiledLibrary::read_from_with_options(
        &mut SliceReader::new(&bytes),
        AstSerdeOptions::new(true, false),
    )
    .unwrap();
    assert_eq!(bundle, deserialized);

    Ok(())
}

#[test]
fn get_module_by_path() -> Result<(), Report> {
    let _context = TestContext::new();
    // declare foo module
    let foo_source = r#"
        export.foo
            add
        end
    "#;
    let foo = parse_module!("test::foo", foo_source);
    let modules = vec![foo];

    // create the bundle with locations
    let bundle = Assembler::default().assemble_library(modules.iter().cloned()).unwrap();

    let foo_module_info = bundle.into_module_infos().next().unwrap();
    assert_eq!(foo_module_info.path(), &LibraryPath::new("test::foo").unwrap());

    let (_, foo_proc) = foo_module_info.procedure_infos().next().unwrap();
    assert_eq!(foo_proc.name, ProcedureName::new("foo").unwrap());

    Ok(())
}
