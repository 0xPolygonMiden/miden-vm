use alloc::{string::String, sync::Arc};

use super::{FullyQualifiedProcedureName, ProcedureName};
use crate::{
    ast::AstSerdeOptions, diagnostics::SourceFile, ByteReader, ByteWriter, DeserializationError,
    SourceSpan, Span, Spanned,
};

/// Represents a procedure that acts like it is locally-defined, but delegates to an externally-
/// defined procedure.
///
/// These procedure "aliases" do not have a concrete representation in the module, but are instead
/// resolved during compilation to refer directly to the aliased procedure, regardless of whether
/// the caller is in the current module, or in another module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureAlias {
    /// The source file in which this alias was defined, if available
    source_file: Option<Arc<SourceFile>>,
    /// The documentation attached to this procedure
    docs: Option<Span<String>>,
    /// The name of the re-exported procedure.
    name: ProcedureName,
    /// The fully-qualified name of the imported procedure
    ///
    /// NOTE: This is fully-qualified from the perspective of
    /// the containing [Module], but may not be fully-resolved
    /// to the concrete definition until compilation time.
    pub(crate) target: FullyQualifiedProcedureName,
}

impl ProcedureAlias {
    /// Create a new procedure alias called `name`, which resolves to `target`
    pub fn new(name: ProcedureName, target: FullyQualifiedProcedureName) -> Self {
        Self {
            docs: None,
            source_file: None,
            name,
            target,
        }
    }

    /// Add documentation to this procedure alias
    pub fn with_docs(mut self, docs: Option<Span<String>>) -> Self {
        self.docs = docs;
        self
    }

    /// Add source code to this declaration, so that we can render source snippets in diagnostics.
    pub fn with_source_file(mut self, source_file: Option<Arc<SourceFile>>) -> Self {
        self.source_file = source_file;
        self
    }

    /// Get the source file associated with this declaration
    pub fn source_file(&self) -> Option<Arc<SourceFile>> {
        self.source_file.clone()
    }

    /// Get the documentation associated with this declaration
    pub fn docs(&self) -> Option<&Span<String>> {
        self.docs.as_ref()
    }

    /// Get the name of this alias within its containing module.
    ///
    /// If the procedure is simply re-exported with the same name,
    /// this will be equivalent to `self.target().name`
    pub fn name(&self) -> &ProcedureName {
        &self.name
    }

    /// Get the fully-qualified procedure name of the aliased procedure
    pub fn target(&self) -> &FullyQualifiedProcedureName {
        &self.target
    }
}

/// Serialization
impl ProcedureAlias {
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        self.name.write_into_with_options(target, options);
        self.target.write_into_with_options(target, options);
    }

    pub fn read_from_with_options<R: ByteReader>(
        source: &mut R,
        options: AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        let name = ProcedureName::read_from_with_options(source, options)?;
        let target = FullyQualifiedProcedureName::read_from_with_options(source, options)?;
        Ok(Self {
            source_file: None,
            docs: None,
            name,
            target,
        })
    }
}

impl Spanned for ProcedureAlias {
    fn span(&self) -> SourceSpan {
        self.target.span()
    }
}

#[cfg(feature = "formatter")]
impl crate::prettier::PrettyPrint for ProcedureAlias {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let mut doc = Document::Empty;
        if let Some(docs) = self.docs.as_deref() {
            doc = docs
                .lines()
                .map(text)
                .reduce(|acc, line| acc + nl() + text("#! ") + line)
                .unwrap_or_default();
        }

        if self.target.name == self.name {
            doc += display(format_args!("export.{}::{}", self.target.module.last(), &self.name));
        } else {
            doc += display(format_args!(
                "export.{}::{}->{}",
                self.target.module.last(),
                &self.target.name,
                &self.name
            ));
        }
        doc
    }
}
