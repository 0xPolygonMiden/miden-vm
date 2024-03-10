use super::{GlobalProcedureIndex, ModuleIndex, Procedure};
use crate::{
    ast::{FullyQualifiedProcedureName, ProcedureIndex},
    AssemblyError, LibraryPath, RpoDigest,
};
use alloc::{
    collections::{BTreeMap, VecDeque},
    sync::Arc,
    vec::Vec,
};
use core::{fmt, ops::Index};

/// The [ProcedureCache] is responsible for caching the MAST of compiled procedures.
///
/// Once cached, subsequent compilations will use the cached MAST artifacts, rather than
/// recompiling the same procedures again and again.
///
/// # Usage
///
/// The procedure cache is intimately tied to a [ModuleGraph], which effectively acts as a cache
/// for the MASM syntax tree, and associates each procedure with a unique [GlobalProcedureIndex]
/// which acts as the cache key for the corresponding [ProcedureCache].
///
/// This also is how we avoid serving cached artifacts when the syntax tree of a module is modified
/// and recompiled - the old module will be removed from the [ModuleGraph] and the new version will
/// be added as a new module, getting new [GlobalProcedureIndex]s for each of its procedures as a
/// result.
///
/// As a result of this design choice, a unique [ProcedureCache] is associated with each context in
/// play during compilation: the global assembler context has its own cache, and each
/// [AssemblyContext] has its own cache.
#[derive(Default)]
pub struct ProcedureCache {
    cache: Vec<Vec<Option<Arc<Procedure>>>>,
    /// This is always the same length as `cache`
    modules: Vec<Option<LibraryPath>>,
    by_mast_root: BTreeMap<RpoDigest, GlobalProcedureIndex>,
}

/// When indexing by [ModuleIndex], we return the [LibraryPath] of the [Module]
/// to which that cache slot belongs.
impl Index<ModuleIndex> for ProcedureCache {
    type Output = LibraryPath;
    fn index(&self, id: ModuleIndex) -> &Self::Output {
        self.modules[id.as_usize()].as_ref().expect("attempted to index an empty cache")
    }
}

/// When indexing by [GlobalProcedureIndex], we return the cached [Procedure]
impl Index<GlobalProcedureIndex> for ProcedureCache {
    type Output = Arc<Procedure>;
    fn index(&self, id: GlobalProcedureIndex) -> &Self::Output {
        self.cache[id.module.as_usize()][id.index.as_usize()]
            .as_ref()
            .expect("attempted to index an empty cache slot")
    }
}

impl ProcedureCache {
    /// Returns true if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of procedures in the cache
    pub fn len(&self) -> usize {
        self.cache.iter().map(|m| m.iter().filter_map(|p| p.as_deref()).count()).sum()
    }

    /// Search for a procedure in the cache using `predicate`
    #[allow(unused)]
    pub fn find<F>(&self, mut predicate: F) -> Option<Arc<Procedure>>
    where
        F: FnMut(&Procedure) -> bool,
    {
        self.cache.iter().find_map(|m| {
            m.iter().filter_map(|p| p.as_ref()).find_map(|p| {
                if predicate(p) {
                    Some(p.clone())
                } else {
                    None
                }
            })
        })
    }

    /// Search for a procedure in the cache using `predicate`,
    /// starting from procedures with the highest [ModuleIndex]
    /// to lowest.
    #[allow(unused)]
    pub fn rfind<F>(&self, mut predicate: F) -> Option<Arc<Procedure>>
    where
        F: FnMut(&Procedure) -> bool,
    {
        self.cache.iter().rev().find_map(|m| {
            m.iter().filter_map(|p| p.as_ref()).find_map(|p| {
                if predicate(p) {
                    Some(p.clone())
                } else {
                    None
                }
            })
        })
    }

    /// Look up a procedure by its MAST root hash
    pub fn get_by_mast_root(&self, digest: &RpoDigest) -> Option<Arc<Procedure>> {
        self.by_mast_root.get(digest).copied().map(|index| self[index].clone())
    }

    /// Look up a procedure by its fully-qualified name
    ///
    /// NOTE: If a procedure with the same name is cached twice, this will return
    /// the version with the highest [ModuleIndex].
    #[allow(unused)]
    pub fn get_by_name(&self, name: &FullyQualifiedProcedureName) -> Option<Arc<Procedure>> {
        self.rfind(|p| p.fully_qualified_name() == name)
    }

    /// Returns the procedure with the given [GlobalProcedureIndex], if it is cached
    pub fn get(&self, id: GlobalProcedureIndex) -> Option<Arc<Procedure>> {
        self.cache
            .get(id.module.as_usize())
            .and_then(|m| m.get(id.index.as_usize()).and_then(|p| p.clone()))
    }

    /// Returns true if the procedure with the given [GlobalProcedureIndex] is cached
    #[allow(unused)]
    pub fn contains_key(&self, id: GlobalProcedureIndex) -> bool {
        self.cache
            .get(id.module.as_usize())
            .map(|m| m.get(id.index.as_usize()).is_some())
            .unwrap_or(false)
    }

    /// Returns true if the procedure with the given MAST root is cached
    #[allow(unused)]
    pub fn contains_mast_root(&self, hash: &RpoDigest) -> bool {
        self.by_mast_root.contains_key(hash)
    }

    /// Returns an iterator over the non-empty entries in the cache
    #[cfg(test)]
    pub fn entries(&self) -> impl Iterator<Item = Arc<Procedure>> + '_ {
        self.cache.iter().flat_map(|m| m.iter().filter_map(|p| p.clone()))
    }

    /// Inserts the given [Procedure] into this cache, using the [GlobalProcedureIndex]
    /// as the cache key.
    ///
    /// # Errors
    ///
    /// This operation will fail under the following conditions:
    ///
    /// * The cache slot for the given [GlobalProcedureIndex] is occupied with a
    /// conflicting definition
    ///
    /// * A procedure with the same MAST root is already in the cache, but the
    /// two procedures have differing metadata (such as the number of locals, etc).
    pub fn insert(
        &mut self,
        id: GlobalProcedureIndex,
        procedure: Arc<Procedure>,
    ) -> Result<(), AssemblyError> {
        let mast_root = procedure.mast_root();

        // Make sure we can index to the cache slot for this procedure
        self.ensure_cache_slot_exists(id, procedure.path());

        // Check if an entry is already in this cache slot.
        //
        // If there is already a cache entry, but it conflicts with what
        // we're trying to cache, then raise an error.
        if let Some(cached) = self.get(id) {
            if cached.mast_root() != mast_root || cached.num_locals() != procedure.num_locals() {
                return Err(AssemblyError::ConflictingDefinitions {
                    first: cached.fully_qualified_name().clone(),
                    second: procedure.fully_qualified_name().clone(),
                });
            }

            // The global procedure index and the MAST root resolve to
            // an already cached version of this procedure, nothing to do
            //
            // TODO: We should emit a warning for this, because while it is
            // not an error per se, it does reflect that we're doing work we
            // don't need to be doing. However, emitting a warning only makes
            // sense if this is controllable by the user, and it isn't yet
            // clear whether this edge case will ever happen in practice anyway.
            return Ok(());
        }

        // We don't have a cache entry yet, but we do want to make sure we
        // don't have a conflicting cache entry with the same MAST root:
        if let Some(cached) = self.get_by_mast_root(&mast_root) {
            // Sanity check
            assert_eq!(cached.mast_root(), mast_root);

            if cached.num_locals() != procedure.num_locals() {
                return Err(AssemblyError::ConflictingDefinitions {
                    first: cached.fully_qualified_name().clone(),
                    second: procedure.fully_qualified_name().clone(),
                });
            }

            // We have a previously cached version of an equivalent procedure,
            // just under a different [GlobalProcedureIndex], so insert the
            // cached procedure into the slot for `id`, but skip inserting
            // a record in the MAST root lookup table
            self.cache[id.module.as_usize()][id.index.as_usize()] = Some(procedure);
            return Ok(());
        }

        // This is a new entry, so record both the cache entry and the MAST root mapping
        self.cache[id.module.as_usize()][id.index.as_usize()] = Some(procedure);
        self.by_mast_root.insert(mast_root, id);

        Ok(())
    }

    /// This removes any entries in the cache for procedures in `module`
    pub fn remove_module(&mut self, module: ModuleIndex) {
        let index = module.as_usize();
        if let Some(slots) = self.cache.get_mut(index) {
            slots.clear();
        }
        if let Some(path) = self.modules.get_mut(index) {
            *path = None;
        }
        self.by_mast_root.retain(|_digest, gid| gid.module != module);
    }

    fn ensure_cache_slot_exists(&mut self, id: GlobalProcedureIndex, module: &LibraryPath) {
        let min_cache_len = id.module.as_usize() + 1;
        let min_module_len = id.index.as_usize() + 1;

        if self.cache.len() < min_cache_len {
            self.cache.resize(min_cache_len, Vec::default());
            self.modules.resize(min_cache_len, None);
        }

        // If this is the first entry for this module index, record
        // the path to the module for future queries
        let module_name = &mut self.modules[id.module.as_usize()];
        if module_name.is_none() {
            *module_name = Some(module.clone());
        }

        let module_cache = &mut self.cache[id.module.as_usize()];
        if module_cache.len() < min_module_len {
            module_cache.resize(min_module_len, None);
        }
    }
}

impl IntoIterator for ProcedureCache {
    type Item = (GlobalProcedureIndex, Arc<Procedure>);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let empty = self.is_empty();
        let pos = (0, 0);
        IntoIter {
            empty,
            pos,
            cache: VecDeque::from_iter(self.cache.into_iter().map(VecDeque::from)),
        }
    }
}

pub struct IntoIter {
    cache: VecDeque<VecDeque<Option<Arc<Procedure>>>>,
    pos: (usize, usize),
    empty: bool,
}

impl Iterator for IntoIter {
    type Item = (GlobalProcedureIndex, Arc<Procedure>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.empty {
            return None;
        }

        loop {
            let (module, index) = self.pos;
            if let Some(slot) = self.cache[module].pop_front() {
                self.pos.1 += 1;
                if let Some(procedure) = slot {
                    let gid = GlobalProcedureIndex {
                        module: ModuleIndex::new(module),
                        index: ProcedureIndex::new(index),
                    };
                    break Some((gid, procedure));
                }
                continue;
            }

            // We've reached the end of this module cache
            self.cache.pop_front();
            self.pos.0 += 1;

            // Check if we've reached the end of the overall cache
            if self.cache.is_empty() {
                self.empty = true;
                break None;
            }
        }
    }
}

impl fmt::Debug for ProcedureCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ProcedureCache")
            .field("modules", &DisplayCachedModules(self))
            .finish()
    }
}

#[doc(hidden)]
struct DisplayCachedModules<'a>(&'a ProcedureCache);

impl<'a> fmt::Debug for DisplayCachedModules<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let roots = &self.0.by_mast_root;
        f.debug_map()
            .entries(self.0.modules.iter().enumerate().zip(self.0.cache.iter()).filter_map(
                |((index, path), slots)| {
                    path.as_ref().map(|path| {
                        (
                            ModuleSlot {
                                index,
                                module: path,
                            },
                            DisplayCachedProcedures {
                                roots,
                                module: index,
                                slots: slots.as_slice(),
                            },
                        )
                    })
                },
            ))
            .finish()
    }
}

#[doc(hidden)]
struct DisplayCachedProcedures<'a> {
    roots: &'a BTreeMap<RpoDigest, GlobalProcedureIndex>,
    slots: &'a [Option<Arc<Procedure>>],
    module: usize,
}

impl<'a> fmt::Debug for DisplayCachedProcedures<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set()
            .entries(self.slots.iter().enumerate().filter_map(|(index, p)| {
                p.as_deref().map(|p| ProcedureSlot {
                    roots: self.roots,
                    module: self.module,
                    index,
                    procedure: p,
                })
            }))
            .finish()
    }
}

// NOTE: Clippy thinks these fields are dead because
// it doesn't recognize that they are used by the
// `debug_map` implementation
#[derive(Debug)]
#[allow(dead_code)]
struct ModuleSlot<'a> {
    index: usize,
    module: &'a LibraryPath,
}

#[doc(hidden)]
struct ProcedureSlot<'a> {
    roots: &'a BTreeMap<RpoDigest, GlobalProcedureIndex>,
    module: usize,
    index: usize,
    procedure: &'a Procedure,
}

impl<'a> fmt::Debug for ProcedureSlot<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = GlobalProcedureIndex {
            module: ModuleIndex::new(self.module),
            index: ProcedureIndex::new(self.index),
        };
        let digest = self
            .roots
            .iter()
            .find_map(|(hash, gid)| if gid == &id { Some(hash) } else { None })
            .expect("missing root for cache entry");
        f.debug_struct("CacheEntry")
            .field("index", &self.index)
            .field("key", digest)
            .field("procedure", self.procedure)
            .finish()
    }
}
