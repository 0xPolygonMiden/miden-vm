use super::{BTreeMap, CodeBlock, ProcMap, Procedure, MODULE_PATH_DELIM};
use crate::ProcedureId;

// ASSEMBLY CONTEXT
// ================================================================================================

/// Context for a compilation of a given program.
///
/// An assembly context contains a set of procedures which can be called from the parsed code.
/// The procedures are divided into 3 groups:
/// 1. Local procedures, which are procedures parsed from the body of a program.
/// 2. Imported procedures, which are procedures imported from external libraries.
/// 3. Kernel procedures, which are procedures provided by the kernel specified for the program.
///
/// Local procedures are owned by the context, while imported and kernel procedures are stored by
/// reference.
pub struct AssemblyContext<'a> {
    local_procs: BTreeMap<u16, Procedure>,
    imported_procs: BTreeMap<ProcedureId, &'a Procedure>,
    kernel_procs: Option<&'a BTreeMap<ProcedureId, Procedure>>,
    in_debug_mode: bool,
}

impl<'a> AssemblyContext<'a> {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [AssemblyContext] with the specified set of kernel procedures. If kernel
    /// procedures are not provided, it is assumed that the context is created for compiling kernel
    /// procedures. That implies that the context can be instantiated in two modes:
    /// - A mode for compiling a kernel (when kernel_procs parameter is set to None). In this mode
    ///   in_kernel() method returns true.
    /// - A mode for compiling regular programs (when kernel_procs parameter contains a set of
    ///   kernel procedures). In this mode, in_kernel() method returns false.
    pub fn new(kernel_procs: Option<&'a ProcMap>, in_debug_mode: bool) -> Self {
        Self {
            kernel_procs,
            local_procs: BTreeMap::new(),
            imported_procs: BTreeMap::new(),
            in_debug_mode,
        }
    }

    // STATE ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns true if the assembly context was instantiated in debug mode.
    pub fn in_debug_mode(&self) -> bool {
        self.in_debug_mode
    }

    /// Returns true if a procedure with the specified label exists in this context.
    pub fn contains_proc(&self, procedure_id: &ProcedureId) -> bool {
        self.imported_procs.contains_key(procedure_id)
    }

    /// Returns a code root of a imported procedure for the specified label from this context.
    pub fn get_imported_proc_code(&self, procedure_id: &ProcedureId) -> Option<&CodeBlock> {
        // `expect()`'s are OK here because we first check if a given map contains the key
        if self.imported_procs.contains_key(procedure_id) {
            let proc = self
                .imported_procs
                .get(procedure_id)
                .expect("no procedure after contains");
            Some(proc.code_root())
        } else {
            None
        }
    }

    pub fn get_local_proc_code(&self, index: u16) -> Option<&CodeBlock> {
        // `expect()`'s are OK here because we first check if a given map contains the key
        if self.local_procs.contains_key(&index) {
            let proc = &*self
                .local_procs
                .get(&index)
                .expect("no procedure after contains");
            Some(proc.code_root())
        } else {
            None
        }
    }

    /// Returns true if this context is used for compiling kernel module.
    pub fn in_kernel(&self) -> bool {
        self.kernel_procs.is_none()
    }

    /// Returns a code root of a kernel procedure for the specified label in this context.
    pub fn get_kernel_proc_code(&self, procedure_id: &ProcedureId) -> Option<&CodeBlock> {
        // `expect()` is OK here because we first check if the kernel is set
        self.kernel_procs
            .expect("no kernel")
            .get(procedure_id)
            .map(|c| c.code_root())
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Adds a local procedure to this context.
    ///
    /// A label for a local procedure is set simply `proc.label`.
    ///
    /// # Panics
    /// Panics if a procedure with the specified label already exists in this context.
    pub fn add_local_proc(&mut self, proc: Procedure) {
        assert!(
            !self.local_procs.contains_key(&proc.index()),
            "duplicate procedure: {}",
            proc.label()
        );
        self.local_procs.insert(proc.index(), proc);
    }

    /// Adds an imported procedure to this context.
    ///
    /// A label for an imported procedure is set to  `prefix::proc.label`.
    ///
    /// # Panics
    /// Panics if a procedure with the specified label already exists in this context.
    pub fn add_imported_proc(&mut self, prefix: &str, proc: &'a Procedure) {
        let label = format!("{prefix}{MODULE_PATH_DELIM}{}", proc.label());
        let procedure_id = ProcedureId::new(label.clone());
        assert!(
            !self.contains_proc(&procedure_id),
            "duplicate procedure: {label}"
        );
        self.imported_procs.insert(procedure_id, proc);
    }

    /// Extracts exported procedures from this context.
    pub fn into_exported_procs(self) -> ProcMap {
        self.local_procs
            .values()
            .filter(|x| x.is_export())
            .map(|x| {
                let procedure_id =
                    ProcedureId::new(format!("{}{MODULE_PATH_DELIM}{}", x.prefix(), x.label()));
                (procedure_id, x.clone())
            })
            .collect::<ProcMap>()
    }
}
