use super::{CodeBlock, Procedure};
use winter_utils::collections::BTreeMap;

// SCRIPT CONTEXT
// ================================================================================================

/// TODO: add docs
pub struct ScriptContext<'a> {
    local_procs: BTreeMap<String, Procedure>,
    imported_procs: BTreeMap<String, &'a Procedure>,
}

impl<'a> ScriptContext<'a> {

    /// TODO: add docs
    pub fn new() -> Self {
        Self {
            local_procs: BTreeMap::new(),
            imported_procs: BTreeMap::new(),
        }
    }

    /// TODO: add docs
    pub fn contains_proc(&self, label: &str) -> bool {
        self.local_procs.contains_key(label) || self.imported_procs.contains_key(label)
    }

    /// TODO: add docs
    pub fn get_proc_code(&self, label: &str) -> Option<&CodeBlock> {
        if self.imported_procs.contains_key(label) {
            let proc = *self.imported_procs.get(label).unwrap();
            Some(proc.code_root())
        } else if self.local_procs.contains_key(label) {
            let proc = self.local_procs.get(label).unwrap();
            Some(proc.code_root())
        } else {
            None
        }
    }

    /// TODO: add docs
    pub fn add_local_proc(&mut self, proc: Procedure) {
        let label = proc.label();
        assert!(!self.contains_proc(label), "duplicate procedure");
        self.local_procs.insert(label.to_string(), proc);
    }

    //pub fn add_imported_proc(&mut self, label: &str, proc: &'a CodeBlock) {
    //    self.imported_procs.insert(label.to_string(), proc);
    //}
}
