use super::{
    super::Assembler, LibraryNamespace, LibraryPath, MaslLibrary, Module, ModuleAst, Version,
};
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

#[test]
fn nested_modules() {
    // declare foo module
    let foo = r#"
        export.foo
            add
        end
        export.foo_mul
            mul
        end
    "#;
    let path = LibraryPath::new("test::test::foo").unwrap();
    let ast = ModuleAst::parse(foo).unwrap();
    let foo = Module::new(path, ast);

    // declare mod module
    let mod_mod = r#"
        use.test::test::foo

        # foo procedures re-exported from mod
        export.foo::foo
        export.foo::foo_mul

        # exported procedures
        export.test_mul_add
            mul
            add
        end
    "#;

    let path = LibraryPath::new("test::test").unwrap();
    let ast = ModuleAst::parse(mod_mod).unwrap();
    let mod_mod = Module::new(path, ast);

    // declare bar module
    let bar = r#"
        use.test::test

        export.test::foo

        export.bar
            mtree_get
        end

        export.bar_mul
            mul
        end
    "#;
    let path = LibraryPath::new("test::test::bar").unwrap();
    let ast = ModuleAst::parse(bar).unwrap();
    let bar = Module::new(path, ast);

    let modules = [foo, bar, mod_mod].to_vec();

    // create the bundle with locations
    let namespace = LibraryNamespace::new("test").unwrap();
    let version = Version::MIN;
    let locations = true;
    let bundle =
        MaslLibrary::new(namespace.clone(), version, locations, modules.clone(), Vec::new())
            .unwrap();

    // serialize/deserialize the bundle
    let mut bytes = Vec::new();
    bundle.write_into(&mut bytes);
    let deserialized = MaslLibrary::read_from(&mut SliceReader::new(&bytes)).unwrap();
    assert_eq!(bundle, deserialized);

    // create the bundle without locations
    let locations = false;
    let mut bundle = MaslLibrary::new(namespace, version, locations, modules, Vec::new()).unwrap();

    // serialize/deserialize the bundle
    let mut bytes = Vec::new();
    bundle.write_into(&mut bytes);
    let deserialized = MaslLibrary::read_from(&mut SliceReader::new(&bytes)).unwrap();

    // bundle with and without locations should not be equal
    assert_ne!(bundle, deserialized, "sanity check");

    // clear locations and check equality
    bundle.clear_locations();
    assert_eq!(bundle, deserialized);

    // use library in a program
    let source = r#"
        use.test::test
        use.test::test::bar
        begin
            # bar procedure
            exec.bar::bar
            
            # foo procedure re-exported from mod further re-exported from bar
            exec.bar::foo

            # foo_mul procedure re-exported from mod
            exec.test::foo_mul

            # testmul procedure exported from test
            exec.test::test_mul_add
        end
    "#;

    let assembler = Assembler::default()
        .with_library(&deserialized)
        .expect("failed to load library");

    assembler.compile(source).expect("Failed to compile test source");
}
