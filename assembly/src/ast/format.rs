use core::fmt;

use super::{
    CodeBody, FormattableNode, InvokedProcsMap, LibraryPath, ProcedureAst, ProcedureId,
    ProcedureName, Vec,
};

const INDENT_STRING: &str = "    ";

/// Context for the Ast formatter
///
/// The context keeps track of the current indentation level, as well as the declared and imported
/// procedures in the program/module being formatted.
pub struct AstFormatterContext<'a> {
    indent_level: usize,
    local_procs: &'a Vec<ProcedureAst>,
    imported_procs: &'a InvokedProcsMap,
}

impl<'a> AstFormatterContext<'a> {
    pub fn new(
        local_procs: &'a Vec<ProcedureAst>,
        imported_procs: &'a InvokedProcsMap,
    ) -> AstFormatterContext<'a> {
        Self {
            indent_level: 0,
            local_procs,
            imported_procs,
        }
    }

    /// Build a context for the inner scope, e.g., the body of a while loop
    pub fn inner_scope_context(&self) -> Self {
        Self {
            indent_level: self.indent_level + 1,
            local_procs: self.local_procs,
            imported_procs: self.imported_procs,
        }
    }

    /// Add indentation to the current line in the formatter
    pub fn indent(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.indent_level {
            write!(f, "{INDENT_STRING}")?;
        }
        Ok(())
    }

    /// Get the name of the local procedure with the given index.
    ///
    /// # Panics
    /// Panics if the index is not associated with a procedure name
    pub fn local_proc(&self, index: usize) -> &ProcedureName {
        assert!(index < self.local_procs.len(), "Local procedure with index {index} not found");
        &self.local_procs[index].name
    }

    /// Get the name of the imported procedure with the given id/hash.
    ///
    /// # Panics
    /// Panics if the id/hash is not associated with an imported procedure
    pub fn imported_proc(&self, id: &ProcedureId) -> &(ProcedureName, LibraryPath) {
        self.imported_procs
            .get(id)
            .expect("Imported procedure with id/hash {id} not found")
    }
}

// FORMATTING OF PROCEDURES
// ================================================================================================
pub struct FormattableProcedureAst<'a> {
    proc: &'a ProcedureAst,
    context: &'a AstFormatterContext<'a>,
}

impl<'a> FormattableProcedureAst<'a> {
    pub fn new(
        proc: &'a ProcedureAst,
        context: &'a AstFormatterContext<'a>,
    ) -> FormattableProcedureAst<'a> {
        Self { proc, context }
    }
}

impl fmt::Display for FormattableProcedureAst<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Docs
        self.context.indent(f)?;
        if let Some(ref doc) = self.proc.docs {
            writeln!(f, "#! {doc}")?;
        }
        // Procedure header
        self.context.indent(f)?;
        if self.proc.is_export {
            write!(f, "export.")?;
        } else {
            write!(f, "proc.")?;
        }
        writeln!(f, "{}.{}", self.proc.name, self.proc.num_locals)?;
        // Body
        write!(
            f,
            "{}",
            FormattableCodeBody::new(&self.proc.body, &self.context.inner_scope_context())
        )?;
        // Procedure footer
        self.context.indent(f)?;
        writeln!(f, "end")
    }
}

// FORMATTING OF CODE BODIES
// ================================================================================================
pub struct FormattableCodeBody<'a> {
    body: &'a CodeBody,
    context: &'a AstFormatterContext<'a>,
}

impl<'a> FormattableCodeBody<'a> {
    pub fn new(
        body: &'a CodeBody,
        context: &'a AstFormatterContext<'a>,
    ) -> FormattableCodeBody<'a> {
        Self { body, context }
    }
}

impl fmt::Display for FormattableCodeBody<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.body.nodes() {
            write!(f, "{}", FormattableNode::new(node, self.context))?;
        }
        Ok(())
    }
}
