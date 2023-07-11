use super::{
    BTreeMap, ByteReader, ByteWriter, Deserializable, DeserializationError, LibraryPath,
    ParsingError, ProcedureId, ProcedureName, Serializable, String, ToString, Token, TokenStream,
    Vec, MAX_IMPORTS, MAX_INVOKED_IMPORTED_PROCS,
};

// TYPE ALIASES
// ================================================================================================

type ImportedModulesMap = BTreeMap<String, LibraryPath>;
type InvokedProcsMap = BTreeMap<ProcedureId, (ProcedureName, LibraryPath)>;

// MODULE IMPORTS
// ================================================================================================

/// Information about imports stored in the AST
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ModuleImports {
    /// Imported libraries.
    imports: ImportedModulesMap,
    /// Imported procedures that are called from somewhere in the AST.
    invoked_procs: InvokedProcsMap,
}

impl ModuleImports {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Create a new ModuleImports instance
    ///
    /// # Panics
    /// Panics if the number of imports is greater than MAX_IMPORTS, or if the number of invoked
    /// procedures is greater than MAX_INVOKED_IMPORTED_PROCS
    pub fn new(imports: ImportedModulesMap, invoked_procs: InvokedProcsMap) -> Self {
        assert!(imports.len() <= MAX_IMPORTS, "too many imports");
        assert!(
            invoked_procs.len() <= MAX_INVOKED_IMPORTED_PROCS,
            "too many imported procedures invoked"
        );
        Self {
            imports,
            invoked_procs,
        }
    }

    // PARSER
    // --------------------------------------------------------------------------------------------
    /// Parses all `use` statements into a map of imports which maps a module name (e.g., "u64") to
    /// its fully-qualified path (e.g., "std::math::u64").
    pub fn parse(tokens: &mut TokenStream) -> Result<Self, ParsingError> {
        let mut imports = BTreeMap::<String, LibraryPath>::new();
        // read tokens from the token stream until all `use` tokens are consumed
        while let Some(token) = tokens.read() {
            match token.parts()[0] {
                Token::USE => {
                    let module_path = token.parse_use()?;
                    let module_name = module_path.last();
                    if imports.contains_key(module_name) {
                        return Err(ParsingError::duplicate_module_import(token, &module_path));
                    }

                    imports.insert(module_name.to_string(), module_path);

                    // consume the `use` token
                    tokens.advance();
                }
                _ => break,
            }
        }

        if imports.len() > MAX_IMPORTS {
            return Err(ParsingError::too_many_imports(imports.len(), MAX_IMPORTS));
        }
        Ok(Self {
            imports,
            invoked_procs: BTreeMap::new(),
        })
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Look up the path of the imported module with the given name.
    pub fn get_module_path(&self, module_name: &str) -> Option<&LibraryPath> {
        self.imports.get(&module_name.to_string())
    }

    /// Return the paths of all imported module
    pub fn import_paths(&self) -> Vec<&LibraryPath> {
        self.imports.values().collect()
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Adds the specified procedure to the set of procedures invoked from imported modules and
    /// returns the ID of the invoked procedure.
    ///
    /// # Errors
    /// Return an error if
    /// - The module with the specified name has not been imported via the `use` statement.
    /// - The total number of invoked procedures exceeds 2^{16} - 1.
    pub fn add_invoked_proc(
        &mut self,
        proc_name: &ProcedureName,
        module_name: &str,
        token: &Token,
    ) -> Result<ProcedureId, ParsingError> {
        let module_path = self
            .imports
            .get(module_name)
            .ok_or_else(|| ParsingError::procedure_module_not_imported(token, module_name))?;
        let proc_id = ProcedureId::from_name(proc_name.as_ref(), module_path);
        self.invoked_procs.insert(proc_id, (proc_name.clone(), module_path.clone()));
        if self.invoked_procs.len() > MAX_INVOKED_IMPORTED_PROCS {
            return Err(ParsingError::too_many_imported_procs_invoked(
                token,
                self.invoked_procs.len(),
                MAX_INVOKED_IMPORTED_PROCS,
            ));
        }
        Ok(proc_id)
    }
}

impl Serializable for ModuleImports {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u16(self.imports.len() as u16);
        // We don't need to serialize the library names (the keys), since the libraty paths (the
        // values) contain the library names
        self.imports.values().for_each(|i| i.write_into(target));
        target.write_u16(self.invoked_procs.len() as u16);
        for (proc_id, (proc_name, lib_path)) in self.invoked_procs.iter() {
            proc_id.write_into(target);
            proc_name.write_into(target);
            lib_path.write_into(target);
        }
    }
}

impl Deserializable for ModuleImports {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let mut imports = BTreeMap::<String, LibraryPath>::new();
        let num_imports = source.read_u16()?;
        for _ in 0..num_imports {
            let path = LibraryPath::read_from(source)?;
            imports.insert(path.last().to_string(), path);
        }

        let mut used_imported_procs = InvokedProcsMap::new();
        let num_used_imported_procs = source.read_u16()?;
        for _ in 0..num_used_imported_procs {
            let proc_id = ProcedureId::read_from(source)?;
            let proc_name = ProcedureName::read_from(source)?;
            let lib_path = LibraryPath::read_from(source)?;
            used_imported_procs.insert(proc_id, (proc_name, lib_path));
        }
        Ok(Self::new(imports, used_imported_procs))
    }
}
