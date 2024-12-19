use alloc::string::String;

use serde::{Deserialize, Serialize};

use super::{de, se};
use crate::Digest;

pub(crate) mod resolver;

/// A system library identifier
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[repr(u8)]
pub enum SystemLibraryId {
    /// The standard library
    Stdlib,
    /// The base library
    Miden,
}

impl core::str::FromStr for SystemLibraryId {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Compiler uses "std" and "base" to identify the standard and base libraries
            // respectively. We also accept "stdlib" and "miden" as aliases for these libraries.
            "std" | "stdlib" => Ok(Self::Stdlib),
            "base" | "miden" => Ok(Self::Miden),
            _ => Err(()),
        }
    }
}

/// The name of a dependency
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum DependencyName {
    /// The dependency is a system library
    System(SystemLibraryId),
    /// The dependency is a user library with the given name
    User(String),
}

impl From<String> for DependencyName {
    fn from(s: String) -> Self {
        if let Ok(id) = s.parse() {
            DependencyName::System(id)
        } else {
            DependencyName::User(s)
        }
    }
}

/// A package dependency
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Dependency {
    /// The name of the dependency.
    /// Serves as a human-readable identifier for the dependency and a search hint for the resolver
    pub name: DependencyName,
    /// The digest of the dependency.
    /// Serves as an ultimate source of truth for identifying the dependency.
    #[serde(
        serialize_with = "se::serialize_digest",
        deserialize_with = "de::deserialize_digest"
    )]
    #[cfg_attr(test, proptest(value = "Digest::default()"))]
    pub digest: Digest,
}
