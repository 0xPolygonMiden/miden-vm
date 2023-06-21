//! Information about module imports for program ASTs
//!
//! The information is used to serialize and deserialize the AST.

use super::{ LibraryPath, ProcedureName };

/// Path and procedures of an imported module.
/// Note that we only represent the procedures that are actually
/// called/executed by the AST. Any un-called/un-executed procedures
/// in the imported module are not represented.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleImportInfo {
    pub path: LibraryPath,
    imported_procs: Vec<ProcedureName>,
}

impl ModuleImportInfo {
    pub fn new(path: LibraryPath) -> Self {
        Self {
            path,
            imported_procs: Vec::<ProcedureName>::new(),
        }
    }

    pub fn add_procedure(& mut self, proc: &str) {
        if let Ok(proc_name) = ProcedureName::try_from(proc.to_string()) {
            self.imported_procs.push(proc_name);
        }
    }
}
