use crate::ast::{MAX_BODY_LEN, MAX_DOCS_LEN};

use super::{
    super::tokens::SourceLocation, code_body::CodeBody, nodes::Node, ByteReader, ByteWriter,
    Deserializable, DeserializationError, ProcedureId, ProcedureName, Serializable,
};
use crate::utils::{collections::*, string::*};
use core::{iter, str::from_utf8};

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
        target.write_many(self.body.nodes());
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
        let nodes = source.read_many::<Node>(body_len)?;
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

// PROCEDURE RE-EXPORT
// ================================================================================================

/// Represents a re-exported procedure.
///
/// A re-exported procedure is a procedure that is defined in a different module in the same
/// library or a different library and re-exported with the same or a different name. The
/// re-exported procedure is not copied into the module, but rather a reference to it is added to
/// the [ModuleAST].
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProcReExport {
    pub(crate) proc_id: ProcedureId,
    pub(crate) name: ProcedureName,
    pub(crate) docs: Option<String>,
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
