use super::{
    format::*,
    imports::ModuleImports,
    parsers::{parse_constants, ParserContext},
    serde::AstSerdeOptions,
    sort_procs_into_vec, LocalProcMap, ProcReExport, ProcedureAst, ReExportedProcMap, MAX_DOCS_LEN,
    MAX_LOCAL_PROCS, MAX_REEXPORTED_PROCS,
    {
        ByteReader, ByteWriter, Deserializable, DeserializationError, ParsingError, SliceReader,
        Token, TokenStream,
    },
};
use crate::utils::{collections::*, string::*};

use core::{fmt, str::from_utf8};
use vm_core::utils::Serializable;

// MODULE AST
// ================================================================================================

/// An abstract syntax tree of a Miden module.
///
/// A module AST consists of a list of procedure ASTs, a list of re-exported procedures, a list of
/// imports, and module documentation. Local procedures could be internal or exported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleAst {
    pub(super) local_procs: Vec<ProcedureAst>,
    pub(super) reexported_procs: Vec<ProcReExport>,
    pub(super) import_info: ModuleImports,
    pub(super) docs: Option<String>,
}

impl ModuleAst {
    // AST
    // --------------------------------------------------------------------------------------------
    /// Returns a new [ModuleAst].
    ///
    /// A module consists of internal and exported procedures but does not contain a body.
    pub fn new(
        local_procs: Vec<ProcedureAst>,
        reexported_procs: Vec<ProcReExport>,
        docs: Option<String>,
    ) -> Result<Self, ParsingError> {
        if local_procs.len() > MAX_LOCAL_PROCS {
            return Err(ParsingError::too_many_module_procs(local_procs.len(), MAX_LOCAL_PROCS));
        }
        if reexported_procs.len() > MAX_REEXPORTED_PROCS {
            return Err(ParsingError::too_many_module_procs(
                reexported_procs.len(),
                MAX_REEXPORTED_PROCS,
            ));
        }
        if let Some(ref docs) = docs {
            if docs.len() > MAX_DOCS_LEN {
                return Err(ParsingError::module_docs_too_long(docs.len(), MAX_DOCS_LEN));
            }
        }
        Ok(Self {
            local_procs,
            reexported_procs,
            import_info: Default::default(),
            docs,
        })
    }

    /// Adds the provided import information to the module.
    ///
    /// # Panics
    /// Panics if import information has already been added.
    pub fn with_import_info(mut self, import_info: ModuleImports) -> Self {
        assert!(self.import_info.is_empty(), "module imports have already been added");
        self.import_info = import_info;
        self
    }

    // PARSER
    // --------------------------------------------------------------------------------------------
    /// Parses the provided source into a [ModuleAst].
    ///
    /// A module consists of internal and exported procedures but does not contain a body.
    pub fn parse(source: &str) -> Result<Self, ParsingError> {
        let mut tokens = TokenStream::new(source)?;
        let mut import_info = ModuleImports::parse(&mut tokens)?;
        let local_constants = parse_constants(&mut tokens)?;
        let mut context = ParserContext {
            import_info: &mut import_info,
            local_procs: LocalProcMap::default(),
            reexported_procs: ReExportedProcMap::default(),
            local_constants,
            num_proc_locals: 0,
        };
        context.parse_procedures(&mut tokens, true)?;

        // make sure program body is absent and there are no more instructions.
        if let Some(token) = tokens.read() {
            if token.parts()[0] == Token::BEGIN {
                return Err(ParsingError::not_a_library_module(token));
            } else {
                return Err(ParsingError::dangling_ops_after_module(token));
            }
        }

        // build a list of local procs sorted by their declaration order
        let local_procs = sort_procs_into_vec(context.local_procs);

        // build a list of re-exported procedures sorted by procedure name
        let reexported_procs = context.reexported_procs.into_values().collect();

        // get module docs and make sure the size is within the limit
        let docs = tokens.take_module_comments();

        context.import_info.check_unused_imports();

        Ok(Self::new(local_procs, reexported_procs, docs)?.with_import_info(import_info))
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a list of procedures in this module.
    pub fn procs(&self) -> &[ProcedureAst] {
        &self.local_procs
    }

    /// Returns a list of re-exported procedures in this module.
    pub fn reexported_procs(&self) -> &[ProcReExport] {
        &self.reexported_procs
    }

    /// Returns doc comments for this module.
    pub fn docs(&self) -> Option<&String> {
        self.docs.as_ref()
    }

    /// Returns a reference to the import information for this module
    pub fn import_info(&self) -> &ModuleImports {
        &self.import_info
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Clears the source locations from this module.
    pub fn clear_locations(&mut self) {
        self.local_procs.iter_mut().for_each(|p| p.clear_locations())
    }

    // SERIALIZATION / DESERIALIZATION
    // --------------------------------------------------------------------------------------------

    /// Returns byte representation of this [ModuleAst].
    ///
    /// The serde options are NOT serialized - the caller must keep track of the serialization
    /// options used.
    pub fn write_into<R: ByteWriter>(&self, target: &mut R, options: AstSerdeOptions) {
        // asserts below are OK because we enforce limits on the number of procedure and length of
        // module docs in the module parser

        // serialize docs
        match &self.docs {
            Some(docs) => {
                assert!(docs.len() <= u16::MAX as usize, "docs too long");
                target.write_u16(docs.len() as u16);
                target.write_bytes(docs.as_bytes());
            }
            None => {
                target.write_u16(0);
            }
        }

        // serialize imports if required
        if options.serialize_imports {
            self.import_info.write_into(target);
        }

        // serialize procedures
        assert!(self.local_procs.len() <= u16::MAX as usize, "too many local procs");
        assert!(
            self.reexported_procs.len() <= MAX_REEXPORTED_PROCS,
            "too many re-exported procs"
        );
        target.write_u16((self.reexported_procs.len()) as u16);
        target.write_many(&self.reexported_procs);
        target.write_u16(self.local_procs.len() as u16);
        target.write_many(&self.local_procs);
    }

    /// Returns a [ModuleAst] struct deserialized from the provided source.
    ///
    /// The serde options must correspond to the options used for serialization.
    pub fn read_from<R: ByteReader>(
        source: &mut R,
        options: AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        // deserialize docs
        let docs_len = source.read_u16()? as usize;
        let docs = if docs_len != 0 {
            let str = source.read_vec(docs_len)?;
            let str =
                from_utf8(&str).map_err(|e| DeserializationError::InvalidValue(e.to_string()))?;
            Some(str.to_string())
        } else {
            None
        };

        // deserialize imports if required
        let import_info = if options.serialize_imports {
            ModuleImports::read_from(source)?
        } else {
            ModuleImports::default()
        };

        // deserialize re-exports
        let num_reexported_procs = source.read_u16()? as usize;
        let reexported_procs = source.read_many::<ProcReExport>(num_reexported_procs)?;

        // deserialize local procs
        let num_local_procs = source.read_u16()? as usize;
        let local_procs = source.read_many::<ProcedureAst>(num_local_procs)?;

        match Self::new(local_procs, reexported_procs, docs) {
            Err(err) => Err(DeserializationError::UnknownError(err.message().clone())),
            Ok(res) => Ok(res.with_import_info(import_info)),
        }
    }

    /// Returns byte representation of this [ModuleAst].
    ///
    /// The serde options are serialized as header information for the purposes of deserialization.
    pub fn to_bytes(&self, options: AstSerdeOptions) -> Vec<u8> {
        let mut target = Vec::<u8>::default();

        // serialize the options, so that deserialization knows what to do
        options.write_into(&mut target);

        self.write_into(&mut target, options);
        target
    }

    /// Returns a [ModuleAst] struct deserialized from the provided bytes.
    ///
    /// This function assumes that the byte array contains a serialized [AstSerdeOptions] struct as
    /// a header.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializationError> {
        let mut source = SliceReader::new(bytes);

        // Deserialize the serialization options used when serializing
        let options = AstSerdeOptions::read_from(&mut source)?;

        Self::read_from(&mut source, options)
    }

    /// Loads the [SourceLocation] of the procedures via [ProcedureAst::load_source_locations].
    ///
    /// The local procedures are expected to have deterministic order from parse. This way, the
    /// serialization can be simplified into a contiguous sequence of locations.
    pub fn load_source_locations<R: ByteReader>(
        &mut self,
        source: &mut R,
    ) -> Result<(), DeserializationError> {
        self.local_procs.iter_mut().try_for_each(|p| p.load_source_locations(source))
    }

    /// Writes the [SourceLocation] of the procedures via [ProcedureAst::write_source_locations].
    ///
    /// The local procedures are expected to have deterministic order from parse. This way, the
    /// serialization can be simplified into a contiguous sequence of locations.
    pub fn write_source_locations<W: ByteWriter>(&self, target: &mut W) {
        self.local_procs.iter().for_each(|p| p.write_source_locations(target))
    }

    // DESTRUCTURING
    // --------------------------------------------------------------------------------------------

    /// Clear import info from the module
    pub fn clear_imports(&mut self) {
        self.import_info.clear();
    }
}

impl fmt::Display for ModuleAst {
    /// Writes this [ModuleAst] as formatted MASM code into the formatter.
    ///
    /// The formatted code puts each instruction on a separate line and preserves correct indentation
    /// for instruction blocks.
    ///
    /// # Panics
    /// Panics if import info is not associated with this module.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Docs
        if let Some(ref doc) = self.docs {
            writeln!(f, "#! {doc}")?;
            writeln!(f)?;
        }

        // Imports
        let paths = self.import_info.import_paths();
        for path in paths.iter() {
            writeln!(f, "use.{path}")?;
        }
        if !paths.is_empty() {
            writeln!(f)?;
        }

        // Re-exports
        for proc in self.reexported_procs.iter() {
            writeln!(f, "export.{}", proc.name())?;
            writeln!(f)?;
        }

        // Local procedures
        let invoked_procs = self.import_info.invoked_procs();
        let context = AstFormatterContext::new(&self.local_procs, invoked_procs);

        for proc in self.local_procs.iter() {
            writeln!(f, "{}", FormattableProcedureAst::new(proc, &context))?;
        }
        Ok(())
    }
}
