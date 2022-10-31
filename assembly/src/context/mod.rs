use super::{
    BTreeMap, CodeBlock, ProcMap, Procedure, StdLibrary, String, ToString, MODULE_PATH_DELIM,
};

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
    local_procs: BTreeMap<[u8; 24], Procedure>,
    imported_procs: BTreeMap<[u8; 24], &'a Procedure>,
    kernel_procs: Option<&'a ProcMap>,
    stdlib: StdLibrary,
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
            stdlib: StdLibrary::default(),
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
    pub fn contains_proc(&self, label: &str) -> bool {
        self.local_procs.contains_key(label) || self.imported_procs.contains_key(label)
    }

    /// Returns a code root of a procedure for the specified label from this context.
    pub fn get_proc_code(&self, hash: &[u8; 24]) -> Option<&CodeBlock> {
        // `expect()`'s are OK here because we first check if a given map contains the key
        if self.imported_procs.contains_key(label) {
            let proc = *self
                .imported_procs
                .get(label)
                .expect("no procedure after contains");
            Some(proc.code_root())
        } else if self.local_procs.contains_key(label) {
            let proc = self
                .local_procs
                .get(label)
                .expect("no procedure after contains");
            Some(proc.code_root())
        } else {
            None
        }
    }

    pub fn get_local_proc_code(&self, index: u32) -> Option<&CodeBlock> {
        // `expect()`'s are OK here because we first check if a given map contains the key
        for proc in self.local_procs.values() {
            if proc.index() == index {
                return Some(proc.code_root());
            }
        }

        None
    }

    /// Returns true if this context is used for compiling kernel module.
    pub fn in_kernel(&self) -> bool {
        self.kernel_procs.is_none()
    }

    /// Returns a code root of a kernel procedure for the specified label in this context.
    pub fn get_kernel_proc_code(&self, hash: &[u8; 24]) -> Option<&CodeBlock> {
        // `expect()` is OK here because we first check if the kernel is set
        self.kernel_procs
            .expect("no kernel")
            .get(label)
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
        let label = proc.label();
        assert!(!self.contains_proc(label), "duplicate procedure: {}", label);
        self.local_procs.insert(label.to_string(), proc);
    }

    /// Adds an imported procedure to this context.
    ///
    /// A label for an imported procedure is set to  `prefix::proc.label`.
    ///
    /// # Panics
    /// Panics if a procedure with the specified label already exists in this context.
    pub fn add_imported_proc(&mut self, prefix: &str, proc: &'a Procedure) {
        let label = format!("{}{}{}", prefix, MODULE_PATH_DELIM, proc.label());
        assert!(
            !self.contains_proc(&label),
            "duplicate procedure: {}",
            label
        );
        self.imported_procs.insert(label, proc);
    }

    /// Extracts local procedures from this context.
    pub fn into_local_procs(self) -> ProcMap {
        self.local_procs
    }
}
