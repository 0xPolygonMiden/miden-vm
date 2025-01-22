//! The [Package] containing a [vm_core::Program] or [assembly::Library] and a manifest(exports and
//! dependencies).

#![no_std]

extern crate alloc;

mod dep;
mod package;

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod tests;

pub use vm_core::{chiplets::hasher::Digest, mast::MastForest, Program};

pub use self::{
    dep::{
        resolver::{
            DependencyResolver, LocalResolvedDependency, MemDependencyResolverByDigest,
            ResolvedDependency,
        },
        Dependency, DependencyName,
    },
    package::{MastArtifact, Package, PackageExport, PackageManifest},
};
