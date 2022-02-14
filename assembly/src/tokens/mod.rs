use super::AssemblyError;
use core::fmt;

mod stream;
pub use stream::TokenStream;

// TOKEN
// ================================================================================================
/// TODO: add comments
#[derive(Clone, Debug, Default)]
pub struct Token<'a> {
    parts: Vec<&'a str>,
    pos: usize,
}

impl<'a> Token<'a> {
    // CONTROL TOKENS
    // --------------------------------------------------------------------------------------------

    pub const BEGIN: &'static str = "begin";
    pub const PROC: &'static str = "proc";
    pub const IF: &'static str = "if";
    pub const ELSE: &'static str = "else";
    pub const WHILE: &'static str = "while";
    pub const REPEAT: &'static str = "repeat";
    pub const EXEC: &'static str = "exec";
    pub const END: &'static str = "end";

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new token created from the specified string and position.
    ///
    /// # Panics
    /// Panic is the `token` parameter is an empty string.
    pub fn new(token: &'a str, pos: usize) -> Self {
        assert!(!token.is_empty(), "token cannot be an empty string");
        Self {
            parts: token.split('.').collect(),
            pos,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the position of this token in the source.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns the number of parts in this token.
    pub fn num_parts(&self) -> usize {
        self.parts.len()
    }

    /// Returns a reference to this token's parts.
    pub fn parts(&self) -> &[&str] {
        &self.parts
    }

    /// Returns true if this token represents a flow control token.
    pub fn is_control_token(&self) -> bool {
        matches!(
            self.parts()[0],
            Self::BEGIN
                | Self::PROC
                | Self::IF
                | Self::ELSE
                | Self::WHILE
                | Self::REPEAT
                | Self::EXEC
                | Self::END
        )
    }

    // STATE MUTATOR
    // --------------------------------------------------------------------------------------------
    /// Updates the contents of this token from the specified string and position.
    ///
    /// # Panics
    /// Panic is the `token` parameter is an empty string.
    pub fn update(&mut self, token: &'a str, pos: usize) {
        assert!(!token.is_empty(), "token cannot be an empty string");
        self.parts.clear();
        token.split('.').for_each(|part| self.parts.push(part));
        self.pos = pos;
    }

    // CONTROL TOKEN PARSERS / VALIDATORS
    // --------------------------------------------------------------------------------------------

    pub fn validate_begin(&self) -> Result<(), AssemblyError> {
        assert_eq!(Self::BEGIN, self.parts[0], "not a begin");
        if self.num_parts() > 1 {
            Err(AssemblyError::extra_param(self))
        } else {
            Ok(())
        }
    }

    pub fn parse_proc(&self) -> Result<(String, u32), AssemblyError> {
        assert_eq!(Self::PROC, self.parts[0], "invalid procedure declaration");
        match self.num_parts() {
            1 => Err(AssemblyError::missing_param(self)),
            2 => {
                let label = validate_proc_label(self.parts[1], self)?;
                Ok((label, 0))
            }
            3 => {
                let label = validate_proc_label(self.parts[1], self)?;
                let num_locals = validate_proc_locals(self.parts[2], self)?;
                Ok((label, num_locals))
            }
            _ => Err(AssemblyError::extra_param(self)),
        }
    }

    pub fn validate_if(&self) -> Result<(), AssemblyError> {
        assert_eq!(Self::IF, self.parts[0], "not an if");
        match self.num_parts() {
            1 => Err(AssemblyError::missing_param(self)),
            2 => {
                if self.parts[1] != "true" {
                    Err(AssemblyError::invalid_param(self, 1))
                } else {
                    Ok(())
                }
            }
            _ => Err(AssemblyError::extra_param(self)),
        }
    }

    pub fn validate_else(&self) -> Result<(), AssemblyError> {
        assert_eq!(Self::ELSE, self.parts[0], "not an else");
        if self.num_parts() > 1 {
            Err(AssemblyError::extra_param(self))
        } else {
            Ok(())
        }
    }

    pub fn validate_while(&self) -> Result<(), AssemblyError> {
        assert_eq!(Self::WHILE, self.parts[0], "not a while");
        match self.num_parts() {
            1 => Err(AssemblyError::missing_param(self)),
            2 => {
                if self.parts[1] != "true" {
                    Err(AssemblyError::invalid_param(self, 1))
                } else {
                    Ok(())
                }
            }
            _ => Err(AssemblyError::extra_param(self)),
        }
    }

    pub fn parse_repeat(&self) -> Result<u32, AssemblyError> {
        assert_eq!(Self::REPEAT, self.parts[0], "not a repeat");
        match self.num_parts() {
            1 => Err(AssemblyError::missing_param(self)),
            2 => self.parts[1]
                .parse::<u32>()
                .map_err(|_| AssemblyError::invalid_param(self, 1)),
            _ => Err(AssemblyError::extra_param(self)),
        }
    }

    pub fn parse_exec(&self) -> Result<String, AssemblyError> {
        assert_eq!(Self::EXEC, self.parts[0], "not an exec");
        match self.num_parts() {
            1 => Err(AssemblyError::missing_param(self)),
            2 => validate_proc_label(self.parts[1], self),
            _ => Err(AssemblyError::extra_param(self)),
        }
    }

    pub fn validate_end(&self) -> Result<(), AssemblyError> {
        assert_eq!(Self::END, self.parts[0], "not an end");
        if self.num_parts() > 1 {
            Err(AssemblyError::extra_param(self))
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

fn validate_proc_label(label: &str, token: &Token) -> Result<String, AssemblyError> {
    // a label must start with an alphanumeric character
    if label.is_empty() || !label.chars().next().unwrap().is_ascii_alphabetic() {
        return Err(AssemblyError::invalid_proc_label(token, label));
    }

    // a label can contain only number, letters, underscores, and colons
    if !label
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':')
    {
        return Err(AssemblyError::invalid_proc_label(token, label));
    }

    Ok(label.to_string())
}

fn validate_proc_locals(locals: &str, token: &Token) -> Result<u32, AssemblyError> {
    match locals.parse::<u64>() {
        Ok(num_locals) => {
            if num_locals > u32::MAX as u64 {
                return Err(AssemblyError::invalid_proc_locals(token, locals));
            }
            Ok(num_locals as u32)
        }
        Err(_) => Err(AssemblyError::invalid_proc_locals(token, locals)),
    }
}
