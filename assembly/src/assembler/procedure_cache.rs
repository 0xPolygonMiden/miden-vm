use super::{btree_map::Entry, AssemblyError, BTreeMap, Procedure, ProcedureId, RpoDigest};

// PROCEDURE CACHE
// ================================================================================================

/// The [ProcedureCache] is responsible for caching [Procedure]s. It allows [Procedure]s to be
/// fetched using both [ProcedureId] and [RpoDigest].
#[derive(Debug, Default)]
pub struct ProcedureCache {
    proc_map: BTreeMap<ProcedureId, Procedure>,
    mast_map: BTreeMap<RpoDigest, ProcedureId>,
    proc_aliases: BTreeMap<ProcedureId, ProcedureId>,
}

impl ProcedureCache {
    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------
    /// Returns a [Procedure] reference corresponding to the [ProcedureId].
    pub fn get_by_id(&self, id: &ProcedureId) -> Option<&Procedure> {
        // first check the procedure map, and if a procedure is not found there, try to look it
        // up by its alias
        match self.proc_map.get(id) {
            Some(proc) => {
                debug_assert!(!self.proc_aliases.contains_key(id), "duplicate procedure ID");
                Some(proc)
            }
            None => self.proc_aliases.get(id).and_then(|proc_id| self.proc_map.get(proc_id)),
        }
    }

    /// Returns a [Procedure] reference corresponding to the MAST root ([RpoDigest]).
    pub fn get_by_hash(&self, root: &RpoDigest) -> Option<&Procedure> {
        self.mast_map.get(root).and_then(|proc_id| self.proc_map.get(proc_id))
    }

    /// Returns true if the [ProcedureCache] contains a [Procedure] for the specified
    /// [ProcedureId].
    pub fn contains_id(&self, id: &ProcedureId) -> bool {
        self.proc_map.contains_key(id) || self.proc_aliases.contains_key(id)
    }

    // MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Inserts a [Procedure] into the [ProcedureCache].
    ///
    /// # Errors
    /// Returns an error if a procedure with the same ID is already in the cache.
    pub fn insert(&mut self, proc: Procedure) -> Result<(), AssemblyError> {
        // if a re-exported procedure with the same id is already in the cache, return an error
        if self.proc_aliases.contains_key(proc.id()) {
            return Err(AssemblyError::duplicate_proc_id(proc.id()));
        }

        // If the entry is `Vacant` then insert the Procedure. If the `ProcedureId` is already in
        // the cache (i.e. it is a duplicate) then return an error.
        match self.proc_map.entry(*proc.id()) {
            Entry::Occupied(_) => Err(AssemblyError::duplicate_proc_id(proc.id())),
            Entry::Vacant(entry) => {
                let mast_root = proc.code_root().hash();
                self.mast_map.entry(mast_root).or_insert(*proc.id());
                entry.insert(proc);
                Ok(())
            }
        }
    }

    /// Associated the provided alias procedure ID with the procedure ID already in the procedure
    /// cache and returns a MAST root of this procedure.
    ///
    /// The procedure being aliased is expected to be in the cache already, either as a regular
    /// procedure or as an alias.
    ///
    /// # Errors
    /// Returns an error if procedure with the provided alias ID is already in the cache.
    ///
    /// # Panics
    /// If the procedure which is being aliased is not in the cache.
    pub fn insert_proc_alias(
        &mut self,
        alias_proc_id: ProcedureId,
        ref_proc_id: ProcedureId,
    ) -> Result<RpoDigest, AssemblyError> {
        // if a procedure with the same id is already in the cache (either as regular procedure or
        // as an alias), return an error
        if self.proc_map.contains_key(&alias_proc_id)
            || self.proc_aliases.contains_key(&alias_proc_id)
        {
            return Err(AssemblyError::duplicate_proc_id(&alias_proc_id));
        }

        // we expect that the procedure being aliased is in cache; if it is neither in the
        // procedure map, nor in alias map, panic. in case the procedure being aliased is itself
        // an alias, this also flattens the reference chain.
        let proc_id = if self.proc_map.contains_key(&ref_proc_id) {
            ref_proc_id
        } else {
            *self.proc_aliases.get(&ref_proc_id).expect("procedure ID not in cache")
        };

        // add an entry to the alias map and get the procedure for this alias
        self.proc_aliases.insert(alias_proc_id, proc_id);
        let proc = self.proc_map.get(&proc_id).expect("procedure not in cache");

        Ok(proc.code_root().hash())
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------

    /// Returns an iterator over the [Procedure]s in the [ProcedureCache].
    #[cfg(test)]
    pub fn values(&self) -> impl Iterator<Item = &Procedure> {
        self.proc_map.values()
    }

    /// Returns the number of [Procedure]s in the [ProcedureCache].
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.mast_map.len()
    }
}
