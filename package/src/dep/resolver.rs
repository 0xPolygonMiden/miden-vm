use alloc::{collections::BTreeMap, sync::Arc};

use assembly::Library;

use super::Dependency;
use crate::{Digest, Package};

// DEPENDENCY RESOLUTION
// ================================================================================================

#[derive(Debug, Clone)]
pub enum DependencyResolution {
    Local(LocalResolution),
}

impl From<Arc<Library>> for DependencyResolution {
    fn from(library: Arc<Library>) -> Self {
        Self::Local(LocalResolution::Library(library))
    }
}

impl From<Arc<Package>> for DependencyResolution {
    fn from(package: Arc<Package>) -> Self {
        Self::Local(LocalResolution::Package(package))
    }
}

#[derive(Debug, Clone)]
pub enum LocalResolution {
    Library(Arc<Library>),
    Package(Arc<Package>),
}

impl From<Arc<Library>> for LocalResolution {
    fn from(library: Arc<Library>) -> Self {
        Self::Library(library)
    }
}

impl From<Arc<Package>> for LocalResolution {
    fn from(package: Arc<Package>) -> Self {
        Self::Package(package)
    }
}

// RESOLVER
// ================================================================================================

pub trait DependencyResolver {
    fn resolve(&self, dependency: &Dependency) -> Option<DependencyResolution>;
}

#[derive(Debug, Default)]
pub struct MemDependencyResolverByDigest {
    resolved: BTreeMap<Digest, DependencyResolution>,
}

impl MemDependencyResolverByDigest {
    pub fn add(&mut self, digest: Digest, resolution: DependencyResolution) {
        self.resolved.insert(digest, resolution);
    }
}

impl DependencyResolver for MemDependencyResolverByDigest {
    fn resolve(&self, dependency: &Dependency) -> Option<DependencyResolution> {
        self.resolved.get(&dependency.digest).cloned()
    }
}
