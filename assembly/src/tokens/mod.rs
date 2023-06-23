use super::{
    ast::InvocationTarget, BTreeMap, ByteReader, ByteWriter, Deserializable, DeserializationError,
    LibraryPath, ParsingError, ProcedureName, Serializable, String, ToString, Vec,
};
use core::fmt;

mod lines;
pub use lines::{LineInfo, LinesStream};

mod location;
pub use location::SourceLocation;

mod stream;
pub use stream::TokenStream;

mod tokenizer;
pub use tokenizer::LineTokenizer;

// TOKEN
// ================================================================================================
/// Token type used to represent a token in the Miden assembly source.
///
/// This struct is intended to be mutated in place by the tokenizer through the `update` method,
/// which updates the token position and splits the token into its composing parts.
#[derive(Clone, Debug, Default)]
pub struct Token<'a> {
    /// The dot-separated parts of a token, e.g. `push.1` is split into `['push', '1']`.
    parts: Vec<&'a str>,
    /// Source location linked to this token.
    location: SourceLocation,
}

impl<'a> Token<'a> {
    // DEFINITION TOKENS
    // --------------------------------------------------------------------------------------------
    pub const BEGIN: &'static str = "begin";
    pub const CONST: &'static str = "const";
    pub const END: &'static str = "end";
    pub const EXPORT: &'static str = "export";
    pub const PROC: &'static str = "proc";
    pub const USE: &'static str = "use";

    // CONTROL FLOW TOKENS
    // --------------------------------------------------------------------------------------------
    pub const CALL: &'static str = "call";
    pub const ELSE: &'static str = "else";
    pub const EXEC: &'static str = "exec";
    pub const IF: &'static str = "if";
    pub const REPEAT: &'static str = "repeat";
    pub const SYSCALL: &'static str = "syscall";
    pub const WHILE: &'static str = "while";

    // DELIMITERS
    // --------------------------------------------------------------------------------------------
    pub const DOC_COMMENT_PREFIX: &str = "#!";
    pub const COMMENT_PREFIX: char = '#';
    pub const EXPORT_ALIAS_DELIM: &str = "->";

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new token created from the specified string and position.
    ///
    /// # Panics
    /// Panic if the `token` parameter is an empty string.
    pub fn new(token: &'a str, location: SourceLocation) -> Self {
        assert!(!token.is_empty(), "token cannot be an empty string");
        Self {
            parts: token.split('.').collect(),
            location,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the [SourceLocation] linked to this [Token].
    pub const fn location(&self) -> &SourceLocation {
        &self.location
    }

    /// Returns the number of parts in this token.
    pub fn num_parts(&self) -> usize {
        self.parts.len()
    }

    /// Returns a reference to this token's parts.
    pub fn parts(&self) -> &[&str] {
        &self.parts
    }

    // STATE MUTATOR
    // --------------------------------------------------------------------------------------------
    /// Updates the contents of this token from the specified string and position.
    ///
    /// # Panics
    /// Panic is the `token` parameter is an empty string.
    pub fn update(&mut self, token: &'a str, location: SourceLocation) {
        assert!(!token.is_empty(), "token cannot be an empty string");
        self.parts.clear();
        token.split('.').for_each(|part| self.parts.push(part));
        self.location = location;
    }

    // CONTROL TOKEN PARSERS / VALIDATORS
    // --------------------------------------------------------------------------------------------

    pub fn parse_use(&self) -> Result<LibraryPath, ParsingError> {
        assert_eq!(Self::USE, self.parts[0], "not a use");
        match self.num_parts() {
            0 => unreachable!(),
            1 => Err(ParsingError::missing_param(self)),
            2 => validate_import_path(self.parts[1], self),
            _ => Err(ParsingError::extra_param(self)),
        }
    }

    pub fn validate_begin(&self) -> Result<(), ParsingError> {
        assert_eq!(Self::BEGIN, self.parts[0], "not a begin");
        if self.num_parts() > 1 {
            Err(ParsingError::extra_param(self))
        } else {
            Ok(())
        }
    }

    pub fn parse_proc(&self) -> Result<(ProcedureName, u16, bool), ParsingError> {
        assert!(
            self.parts[0] == Self::PROC || self.parts[0] == Self::EXPORT,
            "invalid procedure declaration"
        );
        let is_export = self.parts[0] == Self::EXPORT;
        let (name_str, num_locals) = match self.num_parts() {
            0 => unreachable!(),
            1 => return Err(ParsingError::missing_param(self)),
            2 => (self.parts[1], 0),
            3 => {
                let num_locals = validate_proc_locals(self.parts[2], self)?;
                (self.parts[1], num_locals)
            }
            _ => return Err(ParsingError::extra_param(self)),
        };

        ProcedureName::try_from(name_str.to_string())
            .map(|proc_name| (proc_name, num_locals, is_export))
            .map_err(|err| ParsingError::invalid_proc_name(self, err))
    }

    pub fn parse_reexported_proc(
        &self,
    ) -> Result<(ProcedureName, ProcedureName, &str), ParsingError> {
        assert_eq!(Self::EXPORT, self.parts[0], "not an export");
        match self.num_parts() {
            0 => unreachable!(),
            1 => Err(ParsingError::missing_param(self)),
            2 => {
                if self.parts[1].matches(LibraryPath::PATH_DELIM).count() != 1 {
                    return Err(ParsingError::invalid_reexported_procedure(self, self.parts[1]));
                }
                // get module and proc name
                let (module, proc_name_with_alias) = self.parts()[1]
                    .split_once(LibraryPath::PATH_DELIM)
                    .expect("Invalid procedure export {self.parts[1]}");

                // get the alias name if it exists else export it with the original name
                let (ref_name, proc_name) = proc_name_with_alias
                    .split_once(Self::EXPORT_ALIAS_DELIM)
                    .unwrap_or((proc_name_with_alias, proc_name_with_alias));

                // validate the procedure names
                let ref_name = ProcedureName::try_from(ref_name.to_string())
                    .map_err(|err| ParsingError::invalid_proc_name(self, err))?;
                let proc_name = ProcedureName::try_from(proc_name.to_string())
                    .map_err(|err| ParsingError::invalid_proc_name(self, err))?;

                Ok((proc_name, ref_name, module))
            }
            _ => Err(ParsingError::extra_param(self)),
        }
    }

    pub fn validate_if(&self) -> Result<(), ParsingError> {
        assert_eq!(Self::IF, self.parts[0], "not an if");
        match self.num_parts() {
            0 => unreachable!(),
            1 => Err(ParsingError::missing_param(self)),
            2 => {
                if self.parts[1] != "true" {
                    Err(ParsingError::invalid_param(self, 1))
                } else {
                    Ok(())
                }
            }
            _ => Err(ParsingError::extra_param(self)),
        }
    }

    pub fn validate_else(&self) -> Result<(), ParsingError> {
        assert_eq!(Self::ELSE, self.parts[0], "not an else");
        if self.num_parts() > 1 {
            Err(ParsingError::extra_param(self))
        } else {
            Ok(())
        }
    }

    pub fn validate_while(&self) -> Result<(), ParsingError> {
        assert_eq!(Self::WHILE, self.parts[0], "not a while");
        match self.num_parts() {
            0 => unreachable!(),
            1 => Err(ParsingError::missing_param(self)),
            2 => {
                if self.parts[1] != "true" {
                    Err(ParsingError::invalid_param(self, 1))
                } else {
                    Ok(())
                }
            }
            _ => Err(ParsingError::extra_param(self)),
        }
    }

    pub fn parse_repeat(&self) -> Result<u32, ParsingError> {
        assert_eq!(Self::REPEAT, self.parts[0], "not a repeat");
        match self.num_parts() {
            0 => unreachable!(),
            1 => Err(ParsingError::missing_param(self)),
            2 => self.parts[1].parse::<u32>().map_err(|_| ParsingError::invalid_param(self, 1)),
            _ => Err(ParsingError::extra_param(self)),
        }
    }

    pub fn parse_invocation(
        &self,
        invocation_token: &str,
    ) -> Result<InvocationTarget, ParsingError> {
        assert_eq!(invocation_token, self.parts[0], "not an {invocation_token}");
        match self.num_parts() {
            0 => unreachable!(),
            1 => Err(ParsingError::missing_param(self)),
            2 => InvocationTarget::parse(self.parts[1], self),
            _ => Err(ParsingError::extra_param(self)),
        }
    }

    pub fn validate_end(&self) -> Result<(), ParsingError> {
        assert_eq!(Self::END, self.parts[0], "not an end");
        if self.num_parts() > 1 {
            Err(ParsingError::extra_param(self))
        } else {
            Ok(())
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// A module import path must comply with the following rules:
/// - Path limbs must be separated by double-colons ("::").
/// - Each limb must start with an ASCII letter.
/// - Each limb can contain only ASCII letters, numbers, underscores, or colons.
fn validate_import_path(path: &str, token: &Token) -> Result<LibraryPath, ParsingError> {
    LibraryPath::try_from(path).map_err(|_| ParsingError::invalid_module_path(token, path))
}

/// Procedure locals must be a 16-bit integer.
fn validate_proc_locals(locals: &str, token: &Token) -> Result<u16, ParsingError> {
    match locals.parse::<u64>() {
        Ok(num_locals) => {
            if num_locals > u16::MAX as u64 {
                return Err(ParsingError::too_many_proc_locals(token, num_locals, u16::MAX as u64));
            }
            Ok(num_locals as u16)
        }
        Err(_) => Err(ParsingError::invalid_proc_locals(token, locals)),
    }
}
