use alloc::vec::Vec;

use vm_core::utils::{Deserializable, Serializable, SliceReader};

use super::{Library, LibraryNamespace, LibraryPath, MaslLibrary, Module, ModuleAst, Version};

#[test]
fn masl_locations_serialization() {
    // declare foo module
    let foo = r#"
        export.foo
            add
        end
        export.foo_mul
            mul
        end
    "#;
    let path = LibraryPath::new("test::foo").unwrap();
    let ast = ModuleAst::parse(foo).unwrap();
    let foo = Module::new(path, ast);

    // declare bar module
    let bar = r#"
        export.bar
            mtree_get
        end
        export.bar_mul
            mul
        end
    "#;
    let path = LibraryPath::new("test::bar").unwrap();
    let ast = ModuleAst::parse(bar).unwrap();
    let bar = Module::new(path, ast);
    let modules = [foo, bar].to_vec();

    // create the bundle with locations
    let namespace = LibraryNamespace::new("test").unwrap();
    let version = Version::MIN;
    let locations = true;
    let bundle =
        MaslLibrary::new(namespace, version, locations, modules.clone(), Vec::new()).unwrap();

    // serialize/deserialize the bundle
    let mut bytes = Vec::new();
    bundle.write_into(&mut bytes);
    let deserialized = MaslLibrary::read_from(&mut SliceReader::new(&bytes)).unwrap();
    assert_eq!(bundle, deserialized);

    // create the bundle without locations
    let namespace = LibraryNamespace::new("test").unwrap();
    let locations = false;
    let mut bundle = MaslLibrary::new(namespace, version, locations, modules, Vec::new()).unwrap();

    // serialize/deserialize the bundle
    let mut bytes = Vec::new();
    bundle.write_into(&mut bytes);
    let deserialized = MaslLibrary::read_from(&mut SliceReader::new(&bytes)).unwrap();
    assert_ne!(bundle, deserialized, "sanity check");
    bundle.clear_locations();
    assert_eq!(bundle, deserialized);
}

#[test]
fn get_module_by_path() {
    // declare foo module
    let foo_source = r#"
        export.foo
            add
        end
    "#;
    let path = LibraryPath::new("test::foo").unwrap();
    let ast = ModuleAst::parse(foo_source).unwrap();
    let foo = Module::new(path, ast);

    let modules = [foo].to_vec();

    // create the bundle with locations
    let namespace = LibraryNamespace::new("test").unwrap();
    let version = Version::MIN;
    let locations = true;
    let bundle =
        MaslLibrary::new(namespace, version, locations, modules.clone(), Vec::new()).unwrap();

    // get AST associated with "test::foo" path
    let foo_ast = bundle.get_module_ast(&LibraryPath::new("test::foo").unwrap()).unwrap();
    let foo_ast_str = format!("{foo_ast}");
    let foo_expected = "export.foo.0
    add
end

";
    assert_eq!(foo_ast_str, foo_expected);

    assert!(bundle.get_module_ast(&LibraryPath::new("test::bar").unwrap()).is_none());
}
