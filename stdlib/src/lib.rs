#![no_std]

use assembly::{Deserializable, Library, LibraryNamespace, MaslLibrary, Version};

// STANDARD LIBRARY
// ================================================================================================

/// TODO: add docs
pub struct StdLibrary {
    contents: MaslLibrary,
}

impl Default for StdLibrary {
    fn default() -> Self {
        let bytes = include_bytes!("../assets/std.masl");
        let contents = MaslLibrary::read_from_bytes(bytes).expect("failed to read std masl!");
        Self { contents }
    }
}

impl Library for StdLibrary {
    type ModuleIterator<'a> = <MaslLibrary as Library>::ModuleIterator<'a>;

    fn root_ns(&self) -> &LibraryNamespace {
        self.contents.root_ns()
    }

    fn version(&self) -> &Version {
        self.contents.version()
    }

    fn modules(&self) -> Self::ModuleIterator<'_> {
        self.contents.modules()
    }
}

#[test]
fn test_compile() {
    let path = "std::math::u64::overflowing_add";
    let stdlib = StdLibrary::default();
    let exists = stdlib.modules().any(|module| {
        module
            .ast
            .local_procs
            .iter()
            .any(|proc| module.path.concatenate(&proc.name).as_str() == path)
    });

    assert!(exists);
}
