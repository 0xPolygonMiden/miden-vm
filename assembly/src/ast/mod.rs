//! Abstract syntax tree (AST) components of Miden programs, modules, and procedures.
//!
//! Structs in this module (specifically [ProgramAst] and [ModuleAst]) can be used to parse source
//! code into relevant ASTs. This can be done via their `parse()` methods.

use super::{
    crypto::hash::RpoDigest, BTreeMap, ByteReader, ByteWriter, Deserializable,
    DeserializationError, Felt, LabelError, LibraryPath, ParsingError, ProcedureId, ProcedureName,
    Serializable, SliceReader, StarkField, String, ToString, Token, TokenStream, Vec,
    MAX_LABEL_LEN,
};
use core::{iter, str::from_utf8};
use vm_core::utils::bound_into_included_u64;

pub use super::tokens::SourceLocation;

mod nodes;
pub use nodes::{AdviceInjectorNode, Instruction, Node};

mod code_body;
pub use code_body::CodeBody;

mod imports;
pub use imports::ModuleImports;

mod invocation_target;
pub use invocation_target::InvocationTarget;

mod parsers;
use parsers::{parse_constants, ParserContext};

pub(crate) use parsers::{NAMESPACE_LABEL_PARSER, PROCEDURE_LABEL_PARSER};

mod serde;
pub use serde::AstSerdeOptions;

#[cfg(test)]
pub mod tests;

// CONSTANTS
// ================================================================================================

/// Maximum number of procedures in a module.
const MAX_LOCAL_PROCS: usize = u16::MAX as usize;

/// Maximum number of re-exported procedures in a module.
const MAX_REEXPORTED_PROCS: usize = u16::MAX as usize;

/// Maximum number of bytes for a single documentation comment.
const MAX_DOCS_LEN: usize = u16::MAX as usize;

/// Maximum number of nodes in statement body (e.g., procedure body, loop body etc.).
const MAX_BODY_LEN: usize = u16::MAX as usize;

/// Maximum number of imported libraries in a module or a program
const MAX_IMPORTS: usize = u16::MAX as usize;

/// Maximum number of imported procedures used in a module or a program
const MAX_INVOKED_IMPORTED_PROCS: usize = u16::MAX as usize;

/// Maximum stack index at which a full word can start.
const MAX_STACK_WORD_OFFSET: u8 = 12;

// TYPE ALIASES
// ================================================================================================
type LocalProcMap = BTreeMap<ProcedureName, (u16, ProcedureAst)>;
type LocalConstMap = BTreeMap<String, u64>;
type ReExportedProcMap = BTreeMap<ProcedureName, ProcReExport>;

// EXECUTABLE PROGRAM AST
// ================================================================================================

/// An abstract syntax tree of an executable Miden program.
///
/// A program AST consists of a body of the program, a list of internal procedure ASTs, a list of
/// imported libraries, a map from procedure ids to procedure names for imported procedures used in
/// the module, and the source location of the program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramAst {
    body: CodeBody,
    local_procs: Vec<ProcedureAst>,
    import_info: Option<ModuleImports>,
    start: SourceLocation,
}

impl ProgramAst {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// Returns a new [ProgramAst].
    ///
    /// A program consist of a body and a set of internal (i.e., not exported) procedures.
    pub fn new(body: Vec<Node>, local_procs: Vec<ProcedureAst>) -> Result<Self, ParsingError> {
        if local_procs.len() > MAX_LOCAL_PROCS {
            return Err(ParsingError::too_many_module_procs(local_procs.len(), MAX_LOCAL_PROCS));
        }
        let start = SourceLocation::default();
        let body = CodeBody::new(body);
        Ok(Self {
            body,
            local_procs,
            import_info: None,
            start,
        })
    }

    /// Adds the provided import information to the program.
    ///
    /// # Panics
    /// Panics if import information has already been added.
    pub fn with_import_info(mut self, import_info: ModuleImports) -> Self {
        assert!(self.import_info.is_none(), "module imports have already been added");
        self.import_info = Some(import_info);
        self
    }

    /// Binds the provided `locations` to the nodes of this program's body.
    ///
    /// The `start` location points to the `begin` token which does not have its own node.
    ///
    /// # Panics
    /// Panics if source location information has already been associated with this program.
    pub fn with_source_locations<L>(mut self, locations: L, start: SourceLocation) -> Self
    where
        L: IntoIterator<Item = SourceLocation>,
    {
        assert!(!self.body.has_locations(), "source locations have already been loaded");
        self.start = start;
        self.body = self.body.with_source_locations(locations);
        self
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the [SourceLocation] associated with this program, if present.
    pub fn source_locations(&self) -> impl Iterator<Item = &'_ SourceLocation> {
        iter::once(&self.start).chain(self.body.source_locations().iter())
    }

    /// Returns a slice over the internal procedures of this program.
    pub fn procedures(&self) -> &[ProcedureAst] {
        &self.local_procs
    }

    /// Returns a reference to the body of this program.
    pub fn body(&self) -> &CodeBody {
        &self.body
    }

    // PARSER
    // --------------------------------------------------------------------------------------------
    /// Parses the provided source into a [ProgramAst].
    ///
    /// A program consist of a body and a set of internal (i.e., not exported) procedures.
    pub fn parse(source: &str) -> Result<ProgramAst, ParsingError> {
        let mut tokens = TokenStream::new(source)?;
        let mut import_info = ModuleImports::parse(&mut tokens)?;
        let local_constants = parse_constants(&mut tokens)?;

        let mut context = ParserContext {
            import_info: &mut import_info,
            local_procs: LocalProcMap::default(),
            reexported_procs: ReExportedProcMap::default(),
            local_constants,
        };

        context.parse_procedures(&mut tokens, false)?;

        // make sure program body is present
        let next_token = tokens
            .read()
            .ok_or_else(|| ParsingError::unexpected_eof(*tokens.eof_location()))?;
        if next_token.parts()[0] != Token::BEGIN {
            return Err(ParsingError::unexpected_token(next_token, Token::BEGIN));
        }

        let program_start = tokens.pos();
        // consume the 'begin' token
        let header = tokens.read().expect("missing program header");
        let start = *header.location();
        header.validate_begin()?;
        tokens.advance();

        // make sure there is something to be read
        if tokens.eof() {
            return Err(ParsingError::unexpected_eof(*tokens.eof_location()));
        }

        // parse the sequence of nodes and add each node to the list
        let body = context.parse_body(&mut tokens, false)?;

        // consume the 'end' token
        match tokens.read() {
            None => Err(ParsingError::unmatched_begin(
                tokens.read_at(program_start).expect("no begin token"),
            )),
            Some(token) => match token.parts()[0] {
                Token::END => token.validate_end(),
                Token::ELSE => Err(ParsingError::dangling_else(token)),
                _ => Err(ParsingError::unmatched_begin(
                    tokens.read_at(program_start).expect("no begin token"),
                )),
            },
        }?;
        tokens.advance();

        // make sure there are no instructions after the end
        if let Some(token) = tokens.read() {
            return Err(ParsingError::dangling_ops_after_program(token));
        }

        let local_procs = sort_procs_into_vec(context.local_procs);
        let (nodes, locations) = body.into_parts();

        Ok(Self::new(nodes, local_procs)?
            .with_source_locations(locations, start)
            .with_import_info(import_info))
    }

    // SERIALIZATION / DESERIALIZATION
    // --------------------------------------------------------------------------------------------

    /// Returns byte representation of this [ProgramAst].
    ///
    /// The serde options are serialized as header information for the purposes of deserialization.
    pub fn to_bytes(&self, options: AstSerdeOptions) -> Vec<u8> {
        let mut target = Vec::<u8>::default();

        // serialize the options, so that deserialization knows what to do
        options.write_into(&mut target);

        // asserts below are OK because we enforce limits on the number of procedure and the
        // number of body instructions in relevant parsers

        // serialize imports if required
        if options.serialize_imports {
            match &self.import_info {
                Some(imports) => imports.write_into(&mut target),
                None => panic!("imports not initialized"),
            }
        }

        // serialize procedures
        assert!(self.local_procs.len() <= MAX_LOCAL_PROCS, "too many local procs");
        target.write_u16(self.local_procs.len() as u16);
        self.local_procs.write_into(&mut target);

        // serialize program body
        assert!(self.body.nodes().len() <= MAX_BODY_LEN, "too many body instructions");
        target.write_u16(self.body.nodes().len() as u16);
        self.body.nodes().write_into(&mut target);

        target
    }

    /// Returns a [ProgramAst] struct deserialized from the provided bytes.
    ///
    /// This function assumes that the byte array contains a serialized [AstSerdeOptions] struct as
    /// a header.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializationError> {
        let mut source = SliceReader::new(bytes);

        // Deserialize the serialization options used when serializing
        let options = AstSerdeOptions::read_from(&mut source)?;

        // deserialize imports if required
        let mut import_info = None;
        if options.serialize_imports {
            import_info = Some(ModuleImports::read_from(&mut source)?);
        }

        // deserialize local procs
        let num_local_procs = source.read_u16()?;
        let local_procs = Deserializable::read_batch_from(&mut source, num_local_procs as usize)?;

        // deserialize program body
        let body_len = source.read_u16()? as usize;
        let nodes = Deserializable::read_batch_from(&mut source, body_len)?;

        match Self::new(nodes, local_procs) {
            Err(err) => Err(DeserializationError::UnknownError(err.message().clone())),
            Ok(res) => match import_info {
                Some(info) => Ok(res.with_import_info(info)),
                None => Ok(res),
            },
        }
    }

    /// Loads the [SourceLocation] from the `source`.
    ///
    /// It expects the `start` location at the first position, and will subsequently load the
    /// body via [CodeBody::load_source_locations]. Finally, it will load the local procedures via
    /// [ProcedureAst::load_source_locations].
    pub fn load_source_locations<R: ByteReader>(
        &mut self,
        source: &mut R,
    ) -> Result<(), DeserializationError> {
        self.start = SourceLocation::read_from(source)?;
        self.body.load_source_locations(source)?;
        self.local_procs.iter_mut().try_for_each(|p| p.load_source_locations(source))
    }

    /// Writes the [SourceLocation] into `target`.
    ///
    /// It will write the `start` location, and then execute the body serialization via
    /// [CodeBlock::write_source_locations]. Finally, it will write the local procedures via
    /// [ProcedureAst::write_source_locations].
    pub fn write_source_locations<W: ByteWriter>(&self, target: &mut W) {
        self.start.write_into(target);
        self.body.write_source_locations(target);
        self.local_procs.iter().for_each(|p| p.write_source_locations(target))
    }

    // DESTRUCTURING
    // --------------------------------------------------------------------------------------------

    /// Returns local procedures and body nodes of this program.
    pub fn into_parts(self) -> (Vec<ProcedureAst>, Vec<Node>) {
        (self.local_procs, self.body.into_parts().0)
    }

    /// Clear import info from the program
    pub fn clear_imports(&mut self) {
        self.import_info = None;
    }
}

// MODULE AST
// ================================================================================================

/// An abstract syntax tree of a Miden module.
///
/// A module AST consists of a list of procedure ASTs, a list of re-exported procedures, a list of
/// imports, a map from procedure ids to procedure names for imported procedures used in the module,
/// and module documentation. Local procedures could be internal or exported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleAst {
    local_procs: Vec<ProcedureAst>,
    reexported_procs: Vec<ProcReExport>,
    import_info: Option<ModuleImports>,
    docs: Option<String>,
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
            import_info: None,
            docs,
        })
    }

    /// Adds the provided import information to the module.
    ///
    /// # Panics
    /// Panics if import information has already been added.
    pub fn with_import_info(mut self, import_info: ModuleImports) -> Self {
        assert!(self.import_info.is_none(), "module imports have already been added");
        self.import_info = Some(import_info);
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

    /// Returns a map of imported modules in this module.
    pub fn import_paths(&self) -> Vec<&LibraryPath> {
        match &self.import_info {
            Some(info) => info.import_paths(),
            None => Vec::<&LibraryPath>::new(),
        }
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
            match &self.import_info {
                Some(imports) => imports.write_into(target),
                None => panic!("imports not initialized"),
            }
        }

        // serialize procedures
        assert!(self.local_procs.len() <= u16::MAX as usize, "too many local procs");
        assert!(
            self.reexported_procs.len() <= MAX_REEXPORTED_PROCS,
            "too many re-exported procs"
        );
        target.write_u16((self.reexported_procs.len()) as u16);
        self.reexported_procs.write_into(target);
        target.write_u16(self.local_procs.len() as u16);
        self.local_procs.write_into(target);
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
        let mut import_info = None;
        if options.serialize_imports {
            import_info = Some(ModuleImports::read_from(source)?);
        }

        // deserialize re-exports
        let num_reexported_procs = source.read_u16()? as usize;
        let reexported_procs = Deserializable::read_batch_from(source, num_reexported_procs)?;

        // deserialize local procs
        let num_local_procs = source.read_u16()? as usize;
        let local_procs = Deserializable::read_batch_from(source, num_local_procs)?;

        match Self::new(local_procs, reexported_procs, docs) {
            Err(err) => Err(DeserializationError::UnknownError(err.message().clone())),
            Ok(res) => match import_info {
                Some(info) => Ok(res.with_import_info(info)),
                None => Ok(res),
            },
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
        self.import_info = None;
    }
}

// PROCEDURE AST
// ================================================================================================

/// An abstract syntax tree of a Miden procedure.
///
/// A procedure AST consists of a list of body nodes and additional metadata about the procedure
/// (e.g., procedure name, number of memory locals used by the procedure, and whether a procedure
/// is exported or internal).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureAst {
    pub name: ProcedureName,
    pub docs: Option<String>,
    pub num_locals: u16,
    pub body: CodeBody,
    pub start: SourceLocation,
    pub is_export: bool,
}

impl ProcedureAst {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// Constructs a [ProcedureAst].
    ///
    /// A procedure consists of a name, a number of locals, a body, and a flag to signal whether
    /// the procedure is exported.
    pub fn new(
        name: ProcedureName,
        num_locals: u16,
        body: Vec<Node>,
        is_export: bool,
        docs: Option<String>,
    ) -> Self {
        let start = SourceLocation::default();
        let body = CodeBody::new(body);
        Self {
            name,
            docs,
            num_locals,
            body,
            is_export,
            start,
        }
    }

    /// Binds the provided `locations` into the ast nodes.
    ///
    /// The `start` location points to the first node of this block.
    pub fn with_source_locations<L>(mut self, locations: L, start: SourceLocation) -> Self
    where
        L: IntoIterator<Item = SourceLocation>,
    {
        self.start = start;
        self.body = self.body.with_source_locations(locations);
        self
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the [SourceLocation] associated with this procedure, if present.
    pub fn source_locations(&self) -> impl Iterator<Item = &'_ SourceLocation> {
        iter::once(&self.start).chain(self.body.source_locations().iter())
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Clears the source locations from this Ast.
    pub fn clear_locations(&mut self) {
        self.start = SourceLocation::default();
        self.body.clear_locations();
    }

    // SERIALIZATION / DESERIALIZATION
    // --------------------------------------------------------------------------------------------

    /// Loads the [SourceLocation] from the `source`.
    ///
    /// It expects the `start` location at the first position, and will subsequently load the
    /// body via [CodeBody::load_source_locations].
    pub fn load_source_locations<R: ByteReader>(
        &mut self,
        source: &mut R,
    ) -> Result<(), DeserializationError> {
        self.start = SourceLocation::read_from(source)?;
        self.body.load_source_locations(source)?;
        Ok(())
    }

    /// Writes the [SourceLocation] into `target`.
    ///
    /// It will write the `start` location, and then execute the body serialization via
    /// [CodeBlock::write_source_locations].
    pub fn write_source_locations<W: ByteWriter>(&self, target: &mut W) {
        self.start.write_into(target);
        self.body.write_source_locations(target);
    }
}

impl Serializable for ProcedureAst {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        // asserts below are OK because we enforce limits on the procedure body size and length of
        // procedure docs in the procedure parser

        self.name.write_into(target);
        match &self.docs {
            Some(docs) => {
                assert!(docs.len() <= MAX_DOCS_LEN, "docs too long");
                target.write_u16(docs.len() as u16);
                target.write_bytes(docs.as_bytes());
            }
            None => {
                target.write_u16(0);
            }
        }

        target.write_bool(self.is_export);
        target.write_u16(self.num_locals);
        assert!(self.body.nodes().len() <= MAX_BODY_LEN, "too many body instructions");
        target.write_u16(self.body.nodes().len() as u16);
        self.body.nodes().write_into(target);
    }
}

impl Deserializable for ProcedureAst {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let name = ProcedureName::read_from(source)?;
        let docs_len = source.read_u16()? as usize;
        let docs = if docs_len != 0 {
            let str = source.read_vec(docs_len)?;
            let str =
                from_utf8(&str).map_err(|e| DeserializationError::InvalidValue(e.to_string()))?;
            Some(str.to_string())
        } else {
            None
        };

        let is_export = source.read_bool()?;
        let num_locals = source.read_u16()?;
        let body_len = source.read_u16()? as usize;
        let nodes = Deserializable::read_batch_from(source, body_len)?;
        let body = CodeBody::new(nodes);
        let start = SourceLocation::default();
        Ok(Self {
            name,
            num_locals,
            body,
            start,
            is_export,
            docs,
        })
    }
}

/// Represents a re-exported procedure.
///
/// A re-exported procedure is a procedure that is defined in a different module in the same
/// library or a different library and re-exported with the same or a different name. The
/// re-exported procedure is not copied into the module, but rather a reference to it is added to
/// the [ModuleAST].
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProcReExport {
    proc_id: ProcedureId,
    name: ProcedureName,
    docs: Option<String>,
}

impl ProcReExport {
    /// Creates a new re-exported procedure.
    pub fn new(proc_id: ProcedureId, name: ProcedureName, docs: Option<String>) -> Self {
        Self {
            proc_id,
            name,
            docs,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the ID of the re-exported procedure.
    pub fn proc_id(&self) -> ProcedureId {
        self.proc_id
    }

    /// Returns the name of the re-exported procedure.
    pub fn name(&self) -> &ProcedureName {
        &self.name
    }

    /// Returns the documentation of the re-exported procedure, if present.
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }

    /// Returns the ID of the re-exported procedure using the specified module.
    pub fn get_alias_id(&self, module_path: &LibraryPath) -> ProcedureId {
        ProcedureId::from_name(&self.name, module_path)
    }
}

impl Serializable for ProcReExport {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.proc_id.write_into(target);
        self.name.write_into(target);
        match &self.docs {
            Some(docs) => {
                assert!(docs.len() <= MAX_DOCS_LEN, "docs too long");
                target.write_u16(docs.len() as u16);
                target.write_bytes(docs.as_bytes());
            }
            None => {
                target.write_u16(0);
            }
        }
    }
}

impl Deserializable for ProcReExport {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let proc_id = ProcedureId::read_from(source)?;
        let name = ProcedureName::read_from(source)?;
        let docs_len = source.read_u16()? as usize;
        let docs = if docs_len != 0 {
            let str = source.read_vec(docs_len)?;
            let str =
                from_utf8(&str).map_err(|e| DeserializationError::InvalidValue(e.to_string()))?;
            Some(str.to_string())
        } else {
            None
        };
        Ok(Self {
            proc_id,
            name,
            docs,
        })
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Sort a map of procedures into a vec, respecting the order set in the map
fn sort_procs_into_vec(proc_map: LocalProcMap) -> Vec<ProcedureAst> {
    let mut procedures: Vec<_> = proc_map.into_values().collect();
    procedures.sort_by_key(|(idx, _proc)| *idx);

    procedures.into_iter().map(|(_idx, proc)| proc).collect()
}
