#![no_std]

use assembly::{
    ast::Module, utils::Deserializable, Library, LibraryNamespace, LibraryPath, MaslLibrary,
    Version,
};

// STANDARD LIBRARY
// ================================================================================================

/// TODO: add docs
pub struct StdLibrary(MaslLibrary);

impl From<StdLibrary> for MaslLibrary {
    fn from(value: StdLibrary) -> Self {
        value.0
    }
}

impl Default for StdLibrary {
    fn default() -> Self {
        let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/assets/std.masl"));
        let contents = MaslLibrary::read_from_bytes(bytes).expect("failed to read std masl!");
        Self(contents)
    }
}

impl Library for StdLibrary {
    fn root_ns(&self) -> &LibraryNamespace {
        self.0.root_ns()
    }

    fn version(&self) -> &Version {
        self.0.version()
    }

    fn modules(&self) -> impl ExactSizeIterator<Item = &Module> + '_ {
        self.0.modules()
    }

    fn dependencies(&self) -> &[assembly::LibraryNamespace] {
        self.0.dependencies()
    }

    fn get_module(&self, path: &LibraryPath) -> Option<&Module> {
        self.0.get_module(path)
    }
}

#[test]
fn test_compile() {
    let path = "std::math::u64::overflowing_add".parse::<LibraryPath>().unwrap();
    let stdlib = StdLibrary::default();
    let exists = stdlib.modules().any(|module| {
        module
            .procedures()
            .any(|proc| module.path().clone().append(proc.name()).unwrap() == path)
    });

    assert!(exists);
}
