#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

use vm_core::{
    code_blocks::CodeBlock,
    utils::{
        collections::{BTreeMap, Vec},
        string::{String, ToString},
    },
    CodeBlockTable, Kernel, Program,
};
use vm_stdlib::StdLibrary;

mod context;
use context::AssemblyContext;

mod procedures;
use procedures::{parse_proc_blocks, Procedure};

mod parsers;
use parsers::{
    ast::{parse_module, parse_program, ModuleAst, ProcMap as ProcAstMap, ProgramAst},
    combine_blocks, parse_body,
};

mod tokens;
use tokens::{Token, TokenStream};

mod errors;
pub use errors::AssemblyError;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

const MODULE_PATH_DELIM: &str = "::";

// TYPE ALIASES
// ================================================================================================

type ProcMap = BTreeMap<String, Procedure>;
type ModuleMap = BTreeMap<String, ProcMap>;

// ASSEMBLER
// ================================================================================================

/// Miden Assembler which can be used to convert Miden assembly source code into program MAST (
/// represented by the [Program] struct). The assembler can be instantiated in two ways:
/// - Via the `with_kernel()` constructor. In this case, the specified kernel source is compiled
///   into a set of kernel procedures during instantiation. Programs compiled using such assembler
///   can make calls to kernel procedures via `syscall` instruction.
/// - Via the `new()` constructor. In this case, the kernel is assumed to be empty, and the
///   programs compiled using such assembler cannot contain `syscall` instructions.
pub struct Assembler {
    kernel_procs: ProcMap,
    kernel: Kernel,
    in_debug_mode: bool,
}

impl Assembler {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// Returns a new instance of [Assembler] instantiated with empty module map.
    ///
    /// Debug related decorators are added to span blocks when debug mode is enabled.
    pub fn new(in_debug_mode: bool) -> Self {
        Self {
            kernel_procs: BTreeMap::default(),
            kernel: Kernel::default(),
            in_debug_mode,
        }
    }

    /// Returns a new instance of [Assembler] instantiated with the specified kernel.
    ///
    /// Debug related decorators are added to span blocks when debug mode is enabled.
    ///
    /// # Errors
    /// Returns an error if compiling kernel source results in an error.
    pub fn with_kernel(kernel_source: &str, in_debug_mode: bool) -> Result<Self, AssemblyError> {
        let mut assembler = Self::new(in_debug_mode);
        let module_ast = parse_module(kernel_source)?;

        assembler.set_kernel(module_ast)?;
        Ok(assembler)
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns true if this assembler was instantiated in debug mode.
    pub fn in_debug_mode(&self) -> bool {
        self.in_debug_mode
    }

    /// Returns a reference to the kernel for this assembler.
    ///
    /// If the assembler was instantiated without a kernel, the internal kernel will be empty.
    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    // PROGRAM COMPILER
    // --------------------------------------------------------------------------------------------

    /// Compile the a program source to a mast program.
    pub fn compile(&self, source: &str) -> Result<Program, AssemblyError> {
        let program_ast = parse_program(source)?;
        for node in &program_ast.body {
            print!("{:?}", node);
        }
        self.compile_ast(&program_ast)
    }

    /// Compile the provided ast program into a mast program.
    pub fn compile_ast(&self, program_ast: &ProgramAst) -> Result<Program, AssemblyError> {
        let mut context = AssemblyContext::new(Some(&self.kernel_procs), self.in_debug_mode);
        let mut cb_table = CodeBlockTable::default();
        let mut parsed_modules = ModuleMap::new();

        parse_local_procs(
            &program_ast.procedures,
            &mut context,
            &mut cb_table,
            &mut parsed_modules,
        )?;

        // parse the program body
        let root = parse_body(
            &program_ast.body,
            &context,
            &mut cb_table,
            &mut parsed_modules,
            0,
        )?;

        Ok(Program::with_kernel(root, self.kernel.clone(), cb_table))
    }

    /// Parses the specified module ast and sets the set of procedures exported from this module
    /// as the kernel for this assembler.
    fn set_kernel(&mut self, module_ast: ModuleAst) -> Result<(), AssemblyError> {
        let mut context = AssemblyContext::new(None, self.in_debug_mode);
        let mut cb_table = CodeBlockTable::default();
        let mut parsed_modules = ModuleMap::new();

        parse_local_procs(
            &module_ast.procedures,
            &mut context,
            &mut cb_table,
            &mut parsed_modules,
        )?;

        // we might be able to relax this limitation in the future
        assert!(
            cb_table.is_empty(),
            "kernel procedures cannot rely on the code block table"
        );

        // extract the exported local procedures from the context set the kernel of this assembler
        // to these procedures
        let mut module_procs = context.into_local_procs();
        module_procs.retain(|_, p| p.is_export());
        self.kernel_procs = module_procs;

        // build a list of procedure hashes and instantiate a kernel with them
        let kernel_proc_hashes = self
            .kernel_procs
            .values()
            .map(|p| p.code_root().hash())
            .collect::<Vec<_>>();
        self.kernel = Kernel::new(&kernel_proc_hashes);

        Ok(())
    }
}

impl Default for Assembler {
    /// Returns a new instance of [Assembler] instantiated with empty module map in non-debug mode.
    fn default() -> Self {
        Self::new(false)
    }
}

// HELPER FUNCTIONS
// --------------------------------------------------------------------------------------------
fn parse_local_procs(
    procedures: &ProcAstMap,
    context: &mut AssemblyContext,
    cb_table: &mut CodeBlockTable,
    parsed_modules: &mut ModuleMap,
) -> Result<(), AssemblyError> {
    // parse locally defined procedures (if any), and add these procedures to the current
    // context
    for proc in procedures.values() {
        let code_root = parse_proc_blocks(
            &proc.body,
            context,
            cb_table,
            parsed_modules,
            proc.num_locals,
        )?;
        context.add_local_proc(Procedure::new(
            proc.name.clone(),
            proc.is_export,
            proc.num_locals,
            code_root,
            proc.index,
        ))
    }

    Ok(())
}
