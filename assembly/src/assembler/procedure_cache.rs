use super::{btree_map::Entry, AssemblyError, BTreeMap, Procedure, ProcedureId, RpoDigest};

/// The [ProcedureCache] is responsible for caching [Procedure]s. It allows [Procedure]s to be
/// fetched using both [ProcedureId] and [RpoDigest].
#[derive(Debug, Default)]
pub struct ProcedureCache {
    proc_map: BTreeMap<ProcedureId, Procedure>,
    mast_map: BTreeMap<RpoDigest, ProcedureId>,
    proc_aliases: BTreeMap<ProcedureId, ProcedureId>,
}

impl ProcedureCache {
    // ACCESSORS
    // --------------------------------------------------------------------------------------------
    /// Returns a [Procedure] reference corresponding to the [ProcedureId].
    pub fn get_by_id(&self, id: &ProcedureId) -> Result<Option<&Procedure>, AssemblyError> {
        match (self.proc_map.get(id), self.proc_aliases.get(id)) {
            (Some(_), Some(_)) => Err(AssemblyError::duplicate_proc_id(id)),
            (Some(proc), None) => Ok(Some(proc)),
            (None, Some(alias_id)) => Ok(self.proc_map.get(alias_id)),
            (None, None) => Ok(None),
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

    /// Returns true if the [ProcedureCache] contains a [Procedure] for the specified root
    /// ([RpoDigest]).
    pub fn _contains_hash(&self, root: &RpoDigest) -> bool {
        self.mast_map.contains_key(root)
    }

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

    // MUTATORS
    // --------------------------------------------------------------------------------------------
    /// Inserts a [Procedure] into the [ProcedureCache].
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

    /// Inserts a re-exported [ProcedureId] to reference [ProcedureId] map into the
    /// [ProcedureCache].
    pub fn insert_proc_alias(
        &mut self,
        alias_proc_id: ProcedureId,
        proc_id: ProcedureId,
    ) -> Result<(), AssemblyError> {
        // if a procedure with the same id is already in the cache, return an error
        if self.proc_map.contains_key(&alias_proc_id) {
            return Err(AssemblyError::duplicate_proc_id(&alias_proc_id));
        }
        // If the entry is `Vacant` then insert the ProcedureId of the alias procedure. If the
        // `ProcedureId` is already in the cache (i.e. it is a duplicate) then return an error.
        match self.proc_aliases.entry(alias_proc_id) {
            Entry::Occupied(_) => Err(AssemblyError::duplicate_proc_id(&alias_proc_id)),
            Entry::Vacant(entry) => {
                entry.insert(proc_id);
                Ok(())
            }
        }
    }
}
