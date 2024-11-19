//! The [Package] containing a [vm_core::Program] or [assembly::Library], rodata segments,
//! and a manifest(exports and dependencies).
//!
//! Serves as the unit of deployment and distribution in the Miden ecosystem.
//! Contains everything needed to execute a program or link a library.

#![no_std]

extern crate alloc;

mod de;
mod dep;
mod package;
mod se;

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod tests;

pub use self::{
    dep::{
        resolver::{
            DependencyResolution, DependencyResolver, LocalResolution,
            MemDependencyResolverByDigest,
        },
        Dependency, DependencyName, SystemLibraryId,
    },
    package::{MastArtifact, Package, PackageExport, PackageManifest},
};

type Digest = vm_core::chiplets::hasher::Digest;
