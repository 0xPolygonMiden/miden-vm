//! The [Package] containing a [vm_core::Program] or [assembly::Library] and a manifest(exports and
//! dependencies).

#![no_std]

extern crate alloc;

mod dep;
mod package;

#[cfg(test)]
extern crate std;

pub use assembly::{
    Library, LibraryPath,
    ast::{ProcedureName, QualifiedProcedureName},
};
pub use vm_core::{Program, chiplets::hasher::Digest, mast::MastForest};

pub use self::{
    dep::{
        Dependency, DependencyName,
        resolver::{
            DependencyResolver, LocalResolvedDependency, MemDependencyResolverByDigest,
            ResolvedDependency,
        },
    },
    package::{MastArtifact, Package, PackageExport, PackageManifest},
};
