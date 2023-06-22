use super::{LibraryNamespace, LibraryPath, MaslLibrary, Module, ModuleAst, Version};
use vm_core::utils::{Deserializable, Serializable, SliceReader};

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
