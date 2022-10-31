#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

use core::ops;
use crypto::{hashers::Blake3_256, Digest, Hasher};
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

// MODULE PROVIDER
// ================================================================================================

/// A procedure identifier computed as a digest truncated to [`Self::LEN`] bytes, product of the
/// label of a procedure
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcedureId(pub [u8; Self::SIZE]);

impl From<[u8; ProcedureId::SIZE]> for ProcedureId {
    fn from(value: [u8; ProcedureId::SIZE]) -> Self {
        Self(value)
    }
}

impl ops::Deref for ProcedureId {
    type Target = [u8; Self::SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ProcedureId {
    /// Truncated length of the id
    pub const SIZE: usize = 24;

    /// Createa a new procedure id from its label, composed by module path + name identifier.
    ///
    /// No validation is performed regarding the consistency of the label structure
    pub fn new<L>(label: L) -> Self
    where
        L: AsRef<str>,
    {
        let mut digest = [0u8; Self::SIZE];
        let hash = Blake3_256::<Felt>::hash(label.as_ref().as_bytes());
        digest.copy_from_slice(&hash.as_bytes()[..Self::SIZE]);
        Self(digest)
    }
}

/// The module provider is now a simplified version of a module cache. It is expected to evolve to
/// a general solution for the module lookup
pub trait ModuleProvider {
    /// Fetch source contents provided a module path
    fn get_source(&self, path: &str) -> Option<&str>;

    /// Fetch a module AST from its ID
    fn get_module(&self, id: &ProcedureId) -> Option<&ModuleAst>;
}

// A default provider that won't resolve modules
impl ModuleProvider for () {
    fn get_source(&self, _path: &str) -> Option<&str> {
        None
    }

    fn get_module(&self, _id: &ProcedureId) -> Option<&ModuleAst> {
        None
    }
}

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
    module_provider: Box<dyn ModuleProvider>,
    parsed_modules: ModuleMap,
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
            module_provider: Box::new(()),
            parsed_modules: BTreeMap::default(),
            kernel_procs: BTreeMap::default(),
            kernel: Kernel::default(),
            in_debug_mode,
        }
    }

    /// Debug related decorators are added to span blocks when debug mode is enabled.
    pub fn with_debug_mode(mut self, in_debug_mode: bool) -> Self {
        self.in_debug_mode = in_debug_mode;
        self
    }

    /// Create a new assembler with a given module provider
    pub fn with_module_provider<P>(mut self, provider: P) -> Self
    where
        P: ModuleProvider + 'static,
    {
        self.module_provider = Box::new(provider);
        self
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

    // IMPORT PARSERS
    // --------------------------------------------------------------------------------------------

    /// Parses `use` instructions from the token stream.
    ///
    /// For each `use` instructions, retrieves exported procedures from the specified module and
    /// inserts them into the provided context.
    ///
    /// If a module specified by `use` instruction hasn't been parsed yet, parses it, and adds
    /// the parsed module to `self.parsed_modules`.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The `use` instruction is malformed.
    /// - A module specified by the `use` instruction could not be found.
    /// - Parsing the specified module results in an error.
    fn parse_imports<'a>(
        &'a self,
        tokens: &mut TokenStream,
        context: &mut AssemblyContext<'a>,
        dep_chain: &mut Vec<String>,
        cb_table: &mut CodeBlockTable,
    ) -> Result<(), AssemblyError> {
        // read tokens from the token stream until all `use` tokens are consumed
        while let Some(token) = tokens.read() {
            match token.parts()[0] {
                Token::USE => {
                    // parse the `use` instruction to extract module path from it
                    let module_path = &token.parse_use()?;

                    // check if a module with the same path is currently being parsed somewhere up
                    // the chain; if it is, then we have a circular dependency.
                    if dep_chain.iter().any(|v| v == module_path) {
                        dep_chain.push(module_path.clone());
                        return Err(AssemblyError::circular_module_dependency(token, dep_chain));
                    }

                    // add the current module to the dependency chain
                    dep_chain.push(module_path.clone());

                    // if the module hasn't been parsed yet, retrieve its source from the library
                    // and attempt to parse it; if the parsing is successful, this will also add
                    // the parsed module to `self.parsed_modules`
                    if !self.parsed_modules.contains_key(module_path) {
                        self.module_provider
                            .get_source(module_path)
                            .ok_or_else(|| AssemblyError::missing_import_source(token, module_path))
                            .and_then(|module_source| {
                                self.parse_module(module_source, module_path, dep_chain, cb_table)
                            })?;
                    }

                    // get procedures from the module at the specified path; we are guaranteed to
                    // not fail here because the above code block ensures that either there is a
                    // parsed module for the specified path, or the function returns with an error
                    let module_procs = self
                        .parsed_modules
                        .get(module_path)
                        .expect("no module procs");

                    // add all procedures to the current context; procedure labels are set to be
                    // `last_part_of_module_path::procedure_name`. For example, `u256::add`.
                    for proc in module_procs.values() {
                        let path_parts = module_path.split(MODULE_PATH_DELIM).collect::<Vec<_>>();
                        let num_parts = path_parts.len();
                        context.add_imported_proc(path_parts[num_parts - 1], proc);
                    }

                    // consume the `use` token and pop the current module of the dependency chain
                    tokens.advance();
                    dep_chain.pop();
                }
                _ => break,
            }
        }

        Ok(())
    }

    /// Parses a set of exported procedures from the specified source code and adds these
    /// procedures to `self.parsed_modules` using the specified path as the key.
    #[allow(clippy::cast_ref_to_mut)]
    fn parse_module(
        &self,
        source: &str,
        path: &str,
        dep_chain: &mut Vec<String>,
        cb_table: &mut CodeBlockTable,
    ) -> Result<(), AssemblyError> {
        let mut tokens = TokenStream::new(source)?;
        let mut context = AssemblyContext::new(Some(&self.kernel_procs), self.in_debug_mode);

        // parse imported modules (if any), and add exported procedures from these modules to
        // the current context
        self.parse_imports(&mut tokens, &mut context, dep_chain, cb_table)?;

        // parse procedures defined in the module, and add these procedures to the current
        // context
        while let Some(token) = tokens.read() {
            let proc = match token.parts()[0] {
                Token::PROC | Token::EXPORT => {
                    Procedure::parse(&mut tokens, &context, cb_table, true)?
                }
                _ => break,
            };
            context.add_local_proc(proc);
        }

        // make sure there are no dangling instructions after all procedures have been read
        if !tokens.eof() {
            let token = tokens.read().expect("no token before eof");
            return Err(AssemblyError::dangling_ops_after_module(token, path));
        }

        // extract the exported local procedures from the context
        let mut module_procs = context.into_local_procs();
        module_procs.retain(|_, p| p.is_export());

        // insert exported procedures into `self.parsed_procedures`
        // TODO: figure out how to do this using interior mutability
        // When the module provider maps index to procedures, it might be implemented with a
        // send/sync friendly approach (maybe std::sync?).
        unsafe {
            let path = path.to_string();
            let mutable_self = &mut *(self as *const _ as *mut Assembler);
            mutable_self.parsed_modules.insert(path, module_procs);
        }

        Ok(())
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
