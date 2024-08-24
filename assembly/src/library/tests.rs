use alloc::string::ToString;
use core::str::FromStr;

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
fn library_exports() -> Result<(), Report> {
    let context = TestContext::new();

    // build the first library
    let baz = r#"
        export.baz1
            push.7 push.8 sub
        end
    "#;
    let baz = parse_module!(&context, "lib1::baz", baz);

    let lib1 = Assembler::new(context.source_manager()).assemble_library([baz])?;

    // build the second library
    let foo = r#"
        proc.foo1
            push.1 add
        end

        export.foo2
            push.2 add
            exec.foo1
        end

        export.foo3
            push.3 mul
            exec.foo1
            exec.foo2
        end
    "#;
    let foo = parse_module!(&context, "lib2::foo", foo);

    // declare bar module
    let bar = r#"
        use.lib1::baz
        use.lib2::foo

        export.baz::baz1->bar1

        export.foo::foo2->bar2

        export.bar3
            exec.foo::foo2
        end

        proc.bar4
            push.1 push.2 mul
        end

        export.bar5
            push.3 sub
            exec.foo::foo2
            exec.bar1
            exec.bar2
            exec.bar4
        end
    "#;
    let bar = parse_module!(&context, "lib2::bar", bar);
    let modules = [foo, bar];

    let lib2 = Assembler::new(context.source_manager())
        .with_library(lib1)?
        .assemble_library(modules.iter().cloned())?;

    let foo2 = QualifiedProcedureName::from_str("lib2::foo::foo2").unwrap();
    let foo3 = QualifiedProcedureName::from_str("lib2::foo::foo3").unwrap();
    let bar1 = QualifiedProcedureName::from_str("lib2::bar::bar1").unwrap();
    let bar2 = QualifiedProcedureName::from_str("lib2::bar::bar2").unwrap();
    let bar3 = QualifiedProcedureName::from_str("lib2::bar::bar3").unwrap();
    let bar5 = QualifiedProcedureName::from_str("lib2::bar::bar5").unwrap();

    // make sure the library exports all exported procedures
    let expected_exports: BTreeSet<_> = [&foo2, &foo3, &bar1, &bar2, &bar3, &bar5].into();
    let actual_exports: BTreeSet<_> = lib2.exports().collect();
    assert_eq!(expected_exports, actual_exports);

    // make sure foo2, bar2, and bar3 map to the same MastNode
    assert_eq!(lib2.get_export_node_id(&foo2), lib2.get_export_node_id(&bar2));
    assert_eq!(lib2.get_export_node_id(&foo2), lib2.get_export_node_id(&bar3));

    // make sure there are 6 roots in the MAST (foo1, foo2, foo3, bar1, bar4, and bar5)
    assert_eq!(lib2.mast_forest.num_procedures(), 6);

    // bar1 should be the only re-export
    assert!(!lib2.is_reexport(&foo2));
    assert!(!lib2.is_reexport(&foo3));
    assert!(lib2.is_reexport(&bar1));
    assert!(!lib2.is_reexport(&bar2));
    assert!(!lib2.is_reexport(&bar3));
    assert!(!lib2.is_reexport(&bar5));

    Ok(())
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
    let bundle =
        Assembler::new(context.source_manager()).assemble_library(modules.iter().cloned())?;

    let bytes = bundle.to_bytes();
    let deserialized = Library::read_from_bytes(&bytes).unwrap();
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
