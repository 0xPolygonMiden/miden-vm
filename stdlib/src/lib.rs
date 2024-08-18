#![no_std]

extern crate alloc;

use assembly::{mast::MastForest, utils::Deserializable, Library};

// STANDARD LIBRARY
// ================================================================================================

/// TODO: add docs
pub struct StdLibrary(Library);

impl AsRef<Library> for StdLibrary {
    fn as_ref(&self) -> &Library {
        &self.0
    }
}

impl From<StdLibrary> for Library {
    fn from(value: StdLibrary) -> Self {
        value.0
    }
}

impl From<StdLibrary> for MastForest {
    fn from(value: StdLibrary) -> Self {
        value.0.into()
    }
}

impl StdLibrary {
    pub const SERIALIZED: &'static [u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/assets/std.masl"));
}

impl Default for StdLibrary {
    fn default() -> Self {
        let contents =
            Library::read_from_bytes(Self::SERIALIZED).expect("failed to read std masl!");
        Self(contents)
    }
}

#[cfg(test)]
mod tests {
    use assembly::LibraryPath;

    use super::*;

    #[test]
    fn test_compile() {
        let path = "std::math::u64::overflowing_add".parse::<LibraryPath>().unwrap();
        let stdlib = StdLibrary::default();
        let exists = stdlib.0.module_infos().any(|module| {
            module
                .procedures()
                .any(|(_, proc)| module.path().clone().append(&proc.name).unwrap() == path)
        });

        assert!(exists);
    }
}
