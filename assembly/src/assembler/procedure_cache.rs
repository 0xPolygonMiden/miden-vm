use super::{
    btree_map::Entry, AssemblyError, BTreeMap, NamedProcedure, Procedure, ProcedureId, RpoDigest,
};

// PROCEDURE CACHE
// ================================================================================================

/// The [ProcedureCache] is responsible for caching [Procedure]s. It allows [Procedure]s to be
/// fetched using both procedure ID and procedure hash (i.e., MAST root of the procedure).
#[derive(Debug, Default)]
pub struct ProcedureCache {
    procedures: BTreeMap<RpoDigest, Procedure>,
    proc_id_map: BTreeMap<ProcedureId, RpoDigest>,
    proc_aliases: BTreeMap<ProcedureId, ProcedureId>,
}

impl ProcedureCache {
    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------
    /// Returns a [Procedure] reference corresponding to the [ProcedureId].
    pub fn get_by_id(&self, id: &ProcedureId) -> Option<&Procedure> {
        // first try to map the procedure ID to MAST root, and if a direct map is not found there,
        // try to look it up by its alias
        match self.proc_id_map.get(id) {
            Some(mast_root) => {
                debug_assert!(!self.proc_aliases.contains_key(id), "duplicate procedure ID");
                // if there is an entry in the proc_id_map, there also must be an entry in the
                // procedures map
                Some(self.procedures.get(mast_root).expect("missing procedure"))
            }
            None => self.proc_aliases.get(id).map(|proc_id| {
                // if an alias entry was found, there must be an entry in the proc_id_map and an
                // entry in the procedures map
                let mast_root = self.proc_id_map.get(proc_id).expect("missing MAST root");
                self.procedures.get(mast_root).expect("missing procedure")
            }),
        }
    }

    /// Returns a [Procedure] reference corresponding to the MAST root ([RpoDigest]).
    pub fn get_by_hash(&self, mast_root: &RpoDigest) -> Option<&Procedure> {
        self.procedures.get(mast_root)
    }

    /// Returns true if the [ProcedureCache] contains a [Procedure] for the specified
    /// [ProcedureId].
    pub fn contains_id(&self, id: &ProcedureId) -> bool {
        self.proc_id_map.contains_key(id) || self.proc_aliases.contains_key(id)
    }

    // MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Inserts a [Procedure] into the [ProcedureCache].
    ///
    /// # Errors
    /// Returns an error if:
    /// - A procedure with the same ID is already in the cache.
    /// - A procedure with the same MAST root but conflicting procedure metadata exists in the
    ///   cache.
    pub fn insert(&mut self, proc: NamedProcedure) -> Result<(), AssemblyError> {
        // if a procedure with the same id is already in the cache, return an error
        if self.contains_id(proc.id()) {
            return Err(AssemblyError::duplicate_proc_id(proc.id()));
        }

        // If the entry is `Vacant` then insert the Procedure. If the procedure with the same MAST
        // was inserted previously, make sure it doesn't conflict with the new procedure.
        match self.procedures.entry(proc.mast_root()) {
            Entry::Occupied(cached_proc_entry) => {
                let cached_proc = cached_proc_entry.get();
                if proc.num_locals() != cached_proc.num_locals() {
                    Err(AssemblyError::conflicting_num_locals(proc.name()))
                } else {
                    self.proc_id_map.insert(*proc.id(), proc.mast_root());
                    Ok(())
                }
            }
            Entry::Vacant(entry) => {
                self.proc_id_map.insert(*proc.id(), proc.mast_root());
                entry.insert(proc.into_inner());
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
        if self.contains_id(&alias_proc_id) {
            return Err(AssemblyError::duplicate_proc_id(&alias_proc_id));
        }

        // we expect that the procedure being aliased is in cache; if it is neither in the
        // procedure map, nor in alias map, panic. in case the procedure being aliased is itself
        // an alias, this also flattens the reference chain.
        let proc_id = if self.proc_id_map.contains_key(&ref_proc_id) {
            ref_proc_id
        } else {
            *self.proc_aliases.get(&ref_proc_id).expect("procedure ID not in cache")
        };

        // add an entry to the alias map and get the procedure for this alias
        self.proc_aliases.insert(alias_proc_id, proc_id);
        let mast_root = self.proc_id_map.get(&proc_id).expect("procedure not in cache");

        Ok(*mast_root)
    }

    // TEST HELPERS
    // --------------------------------------------------------------------------------------------

    /// Returns an iterator over the [Procedure]s in the [ProcedureCache].
    #[cfg(test)]
    pub fn values(&self) -> impl Iterator<Item = &Procedure> {
        self.procedures.values()
    }

    /// Returns the number of [Procedure]s in the [ProcedureCache].
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.procedures.len()
    }
}
