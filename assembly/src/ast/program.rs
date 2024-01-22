use crate::ast::MAX_BODY_LEN;

use super::{
    super::tokens::SourceLocation,
    code_body::CodeBody,
    imports::ModuleImports,
    instrument,
    nodes::Node,
    parsers::{parse_constants, ParserContext},
    serde::AstSerdeOptions,
    {
        format::*, sort_procs_into_vec, LocalProcMap, ProcedureAst, ReExportedProcMap,
        MAX_LOCAL_PROCS,
    },
    {
        ByteReader, ByteWriter, Deserializable, DeserializationError, ParsingError, Serializable,
        SliceReader, Token, TokenStream,
    },
};
use crate::utils::collections::*;

use core::{fmt, iter};
#[cfg(feature = "std")]
use std::{fs, io, path::Path};
// PROGRAM AST
// ================================================================================================

/// An abstract syntax tree of an executable Miden program.
///
/// A program AST consists of a body of the program, a list of internal procedure ASTs, a list of
/// imported libraries, a map from procedure ids to procedure names for imported procedures used in
/// the module, and the source location of the program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramAst {
    pub(super) body: CodeBody,
    pub(super) local_procs: Vec<ProcedureAst>,
    pub(super) import_info: ModuleImports,
    pub(super) start: SourceLocation,
}

impl ProgramAst {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// Returns a new [ProgramAst].
    ///
    /// A program consist of a body and a set of internal (i.e., not exported) procedures.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The number of body nodes is greater than or equal to 2^16.
    /// - The number of local procedures is greater than or equal to 2^16.
    pub fn new(body: Vec<Node>, local_procs: Vec<ProcedureAst>) -> Result<Self, ParsingError> {
        // TODO: instead of ParsingError, this should probably return a different error type:
        // e.g., AstError.
        if body.len() > MAX_BODY_LEN {
            return Err(ParsingError::too_many_body_nodes(body.len(), MAX_BODY_LEN));
        }
        if local_procs.len() > MAX_LOCAL_PROCS {
            return Err(ParsingError::too_many_module_procs(local_procs.len(), MAX_LOCAL_PROCS));
        }
        let start = SourceLocation::default();
        let body = CodeBody::new(body);
        Ok(Self {
            body,
            local_procs,
            import_info: Default::default(),
            start,
        })
    }

    /// Adds the provided import information to the program.
    ///
    /// # Panics
    /// Panics if import information has already been added.
    pub fn with_import_info(mut self, import_info: ModuleImports) -> Self {
        assert!(self.import_info.is_empty(), "module imports have already been added");
        self.import_info = import_info;
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

    /// Returns a reference to the import info for this program
    pub fn import_info(&self) -> &ModuleImports {
        &self.import_info
    }

    // PARSER
    // --------------------------------------------------------------------------------------------
    /// Parses the provided source into a [ProgramAst].
    ///
    /// A program consist of a body and a set of internal (i.e., not exported) procedures.
    #[instrument(name = "parse_program", skip_all)]
    pub fn parse(source: &str) -> Result<ProgramAst, ParsingError> {
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

        context.import_info.check_unused_imports();

        let local_procs = sort_procs_into_vec(context.local_procs);
        let (nodes, locations) = body.into_parts();
        Ok(Self::new(nodes, local_procs)?
            .with_source_locations(locations, start)
            .with_import_info(import_info))
    }

    // SERIALIZATION / DESERIALIZATION
    // --------------------------------------------------------------------------------------------

    /// Writes byte representation of this [ProgramAst] into the specified target according with
    /// the specified serde options.
    ///
    /// The serde options are serialized as header information for the purposes of deserialization.
    pub fn write_into<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        // serialize the options, so that deserialization knows what to do
        options.write_into(target);

        // asserts below are OK because we enforce limits on the number of procedure and the
        // number of body instructions in relevant parsers

        // serialize imports if required
        if options.serialize_imports {
            self.import_info.write_into(target);
        }

        // serialize procedures
        assert!(self.local_procs.len() <= MAX_LOCAL_PROCS, "too many local procs");
        target.write_u16(self.local_procs.len() as u16);
        target.write_many(&self.local_procs);

        // serialize program body
        assert!(self.body.nodes().len() <= MAX_BODY_LEN, "too many body instructions");
        target.write_u16(self.body.nodes().len() as u16);
        target.write_many(self.body.nodes());
    }

    /// Returns byte representation of this [ProgramAst].
    ///
    /// The serde options are serialized as header information for the purposes of deserialization.
    pub fn to_bytes(&self, options: AstSerdeOptions) -> Vec<u8> {
        let mut target = Vec::<u8>::default();
        self.write_into(&mut target, options);
        target
    }

    /// Returns a [ProgramAst] struct deserialized from the specified reader.
    ///
    /// This function assumes that the byte array contains a serialized [AstSerdeOptions] struct as
    /// a header.
    pub fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        // Deserialize the serialization options used when serializing
        let options = AstSerdeOptions::read_from(source)?;

        // deserialize imports if required
        let import_info = if options.serialize_imports {
            ModuleImports::read_from(source)?
        } else {
            ModuleImports::default()
        };

        // deserialize local procs
        let num_local_procs = source.read_u16()?.into();
        let local_procs = source.read_many::<ProcedureAst>(num_local_procs)?;

        // deserialize program body
        let body_len = source.read_u16()? as usize;
        let nodes = source.read_many::<Node>(body_len)?;

        match Self::new(nodes, local_procs) {
            Err(err) => Err(DeserializationError::UnknownError(err.message().clone())),
            Ok(res) => Ok(res.with_import_info(import_info)),
        }
    }

    /// Returns a [ProgramAst] struct deserialized from the provided bytes.
    ///
    /// This function assumes that the byte array contains a serialized [AstSerdeOptions] struct as
    /// a header.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializationError> {
        let mut source = SliceReader::new(bytes);
        Self::read_from(&mut source)
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
        self.import_info.clear();
    }

    // WRITE TO FILE
    // --------------------------------------------------------------------------------------------

    /// Writes ProgramAst to provided file path
    #[cfg(feature = "std")]
    pub fn write_to_file<P>(&self, file_path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let path = file_path.as_ref();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }

        let bytes = self.to_bytes(AstSerdeOptions {
            serialize_imports: true,
        });
        fs::write(path, bytes)
    }
}

impl fmt::Display for ProgramAst {
    /// Writes this [ProgramAst] as formatted MASM code into the formatter.
    ///
    /// The formatted code puts each instruction on a separate line and preserves correct indentation
    /// for instruction blocks.
    ///
    /// # Panics
    /// Panics if import info is not associated with this program.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Imports
        let paths = self.import_info.import_paths();
        for path in paths.iter() {
            writeln!(f, "use.{path}")?;
        }
        if !paths.is_empty() {
            writeln!(f)?;
        }

        let invoked_procs = self.import_info.invoked_procs();
        let context = AstFormatterContext::new(&self.local_procs, invoked_procs);

        // Local procedures
        for proc in self.local_procs.iter() {
            writeln!(f, "{}", FormattableProcedureAst::new(proc, &context))?;
        }

        // Main progrma
        writeln!(f, "begin")?;
        write!(f, "{}", FormattableCodeBody::new(&self.body, &context.inner_scope_context()))?;
        writeln!(f, "end")
    }
}
