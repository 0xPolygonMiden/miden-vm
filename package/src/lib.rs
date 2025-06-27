//! The [Package] containing a [vm_core::Program] or [assembly::Library] and a manifest(exports and
//! dependencies).

#![no_std]

extern crate alloc;

mod artifact;
mod dependency;
mod package;

#[cfg(test)]
extern crate std;

pub use miden_assembly_syntax::{
    Library, LibraryPath,
    ast::{ProcedureName, QualifiedProcedureName},
};
pub use miden_core::{Program, Word, mast::MastForest};

pub use self::{
    artifact::MastArtifact,
    dependency::{
        Dependency, DependencyName,
        resolver::{
            DependencyResolver, LocalResolvedDependency, MemDependencyResolverByDigest,
            ResolvedDependency,
        },
    },
    package::{Package, PackageExport, PackageManifest},
};
