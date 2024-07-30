#![no_std]

extern crate alloc;

use assembly::{library::CompiledLibrary, utils::Deserializable};

// STANDARD LIBRARY
// ================================================================================================

/// TODO: add docs
pub struct StdLibrary(CompiledLibrary);

impl From<StdLibrary> for CompiledLibrary {
    fn from(value: StdLibrary) -> Self {
        value.0
    }
}

impl Default for StdLibrary {
    fn default() -> Self {
        let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/assets/std.masl"));
        let contents = CompiledLibrary::read_from_bytes(bytes).expect("failed to read std masl!");
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
        let exists = stdlib.0.into_module_infos().any(|module| {
            module
                .procedure_infos()
                .any(|(_, proc)| module.path().clone().append(&proc.name).unwrap() == path)
        });

        assert!(exists);
    }
}
