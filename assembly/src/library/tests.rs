use alloc::{string::ToString, sync::Arc, vec::Vec};

use vm_core::utils::{Deserializable, Serializable, SliceReader};

use super::{Library, LibraryNamespace, LibraryPath, MaslLibrary, Version};
use crate::{
    ast::{AstSerdeOptions, Module, ModuleKind},
    diagnostics::{IntoDiagnostic, Report, SourceFile},
    testing::TestContext,
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
    let namespace = LibraryNamespace::new("test").unwrap();
    let version = Version::min();
    let bundle = MaslLibrary::new(namespace, version, modules.iter().cloned(), Vec::new())?;

    let mut bytes = Vec::new();
    bundle.write_into(&mut bytes);
    let deserialized = MaslLibrary::read_from(&mut SliceReader::new(&bytes)).unwrap();
    assert_eq!(bundle, deserialized);

    // serialize/deserialize the bundle without locations
    let namespace = LibraryNamespace::new("test").unwrap();
    let bundle = MaslLibrary::new(namespace, version, modules, Vec::new())?;

    // serialize/deserialize the bundle
    let mut bytes = Vec::new();
    bundle.write_into_with_options(&mut bytes, AstSerdeOptions::new(true, false));
    let deserialized = MaslLibrary::read_from(&mut SliceReader::new(&bytes)).unwrap();
    assert_eq!(bundle, deserialized);

    Ok(())
}

#[test]
#[cfg(feature = "formatter")]
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
    let namespace = LibraryNamespace::new("test")?;
    let version = Version::min();
    let bundle = MaslLibrary::new(namespace, version, modules, Vec::new())?;

    // get AST associated with "test::foo" path
    let foo_ast = bundle.get_module(&LibraryPath::new("test::foo").unwrap()).unwrap();
    let foo_expected = "export.foo
    add
end

";
    assert_eq!(foo_ast.to_string(), foo_expected);

    assert!(bundle.get_module(&LibraryPath::new("test::bar").unwrap()).is_none());

    Ok(())
}
