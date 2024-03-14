mod context;
mod errors;
mod passes;

pub use self::context::AnalysisContext;
pub use self::errors::{SemanticAnalysisError, SyntaxError};

use self::passes::{ConstEvalVisitor, VerifyInvokeTargets};

use crate::{ast::*, diagnostics::SourceFile, LibraryNamespace, LibraryPath, Span, Spanned};
use alloc::collections::BTreeSet;
use alloc::{boxed::Box, collections::VecDeque, sync::Arc, vec::Vec};

/// Constructs and validates a [Module], given the forms constituting the module body.
///
/// As part of this process, the following is also done:
///
/// * Documentation comments are attached to items they decorate
/// * Import table is constructed
/// * Symbol resolution is performed:
///   * Constants referenced by name are replaced with the value of that constant.
///   * Calls to imported procedures are resolved concretely
/// * Semantic analysis is performed on the module to validate it
pub fn analyze(
    source: Arc<SourceFile>,
    kind: ModuleKind,
    path: LibraryPath,
    forms: Vec<Form>,
) -> Result<Box<Module>, SyntaxError> {
    let mut analyzer = AnalysisContext::new(source, kind, path);
    let mut forms = VecDeque::from(forms);
    let mut docs = None;
    while let Some(form) = forms.pop_front() {
        match form {
            Form::ModuleDoc(docstring) => {
                assert!(docs.is_none());
                analyzer.module_mut().set_docs(Some(docstring));
            }
            Form::Doc(docstring) => {
                if let Some(unused) = docs.replace(docstring) {
                    analyzer.error(SemanticAnalysisError::UnusedDocstring {
                        span: unused.span(),
                    });
                }
            }
            Form::Constant(constant) => {
                analyzer.define_constant(constant.with_docs(docs.take()))?;
            }
            Form::Import(import) => {
                if let Some(docs) = docs.take() {
                    analyzer.error(SemanticAnalysisError::ImportDocstring { span: docs.span() });
                }
                if matches!(kind, ModuleKind::Kernel) {
                    analyzer.error(SemanticAnalysisError::ImportToKernel {
                        span: import.span(),
                    });
                } else {
                    analyzer.define_import(import)?;
                }
            }
            Form::Procedure(export @ Export::Alias(_)) => match kind {
                ModuleKind::Kernel => {
                    docs.take();
                    analyzer.error(SemanticAnalysisError::ImportToKernel {
                        span: export.span(),
                    });
                }
                ModuleKind::Executable => {
                    docs.take();
                    analyzer.error(SemanticAnalysisError::UnexpectedExport {
                        span: export.span(),
                    });
                }
                ModuleKind::Library => {
                    analyzer.define_procedure(export.with_docs(docs.take()))?;
                }
            },
            Form::Procedure(export) => match kind {
                ModuleKind::Executable
                    if export.visibility().is_exported() && !export.is_main() =>
                {
                    docs.take();
                    analyzer.error(SemanticAnalysisError::UnexpectedExport {
                        span: export.span(),
                    });
                }
                _ => {
                    analyzer.define_procedure(export.with_docs(docs.take()))?;
                }
            },
            Form::Begin(body) if matches!(kind, ModuleKind::Executable) => {
                let docs = docs.take();
                let source_file = analyzer.source_file();
                let procedure =
                    Procedure::new(body.span(), Visibility::Public, ProcedureName::main(), 0, body)
                        .with_docs(docs)
                        .with_source_file(Some(source_file));
                analyzer.define_procedure(Export::Procedure(procedure))?;
            }
            Form::Begin(body) => {
                docs.take();
                analyzer.error(SemanticAnalysisError::UnexpectedEntrypoint { span: body.span() });
            }
        }
    }

    if let Some(unused) = docs.take() {
        analyzer.error(SemanticAnalysisError::UnusedDocstring {
            span: unused.span(),
        });
    }

    if matches!(kind, ModuleKind::Executable) && !analyzer.module().has_entrypoint() {
        analyzer.error(SemanticAnalysisError::MissingEntrypoint);
    }

    if analyzer.has_errors() {
        return analyzer.into_result();
    }

    // Run procedure checks
    let module = visit_procedures(&mut analyzer)?;

    // Check unused imports
    for import in module.imports() {
        if !import.is_used() {
            analyzer.error(SemanticAnalysisError::UnusedImport {
                span: import.span(),
            });
        }
    }

    if analyzer.has_errors() {
        analyzer.into_result()
    } else {
        Ok(module)
    }
}

/// Visit all of the procedures of the current analysis context,
/// and apply various transformation and analysis passes.
///
/// When this function returns, all local analysis is complete,
/// and all that remains is construction of a module graph and
/// global program analysis to perform any remaining transformations.
fn visit_procedures(analyzer: &mut AnalysisContext) -> Result<Box<Module>, SyntaxError> {
    let mut module = analyzer.take_module();

    let is_kernel = module.is_kernel();
    let locals = BTreeSet::from_iter(module.procedures().map(|p| p.name().clone()));
    let mut procedures = VecDeque::from(core::mem::take(&mut module.procedures));
    while let Some(procedure) = procedures.pop_front() {
        match procedure {
            Export::Procedure(mut procedure) => {
                // Rewrite visibility for exported kernel procedures
                if is_kernel && matches!(procedure.visibility(), Visibility::Public) {
                    procedure.set_visibility(Visibility::Syscall);
                }

                // Evaluate all named immediates to their concrete values
                {
                    let mut visitor = ConstEvalVisitor::new(analyzer);
                    visitor.visit_mut_procedure(&mut procedure);
                }

                // Next, verify invoke targets:
                //
                // * Kernel procedures cannot use `syscall` or `call`
                // * Mark imports as used if they have at least one call to a procedure defined in
                //   that module
                // * Verify that all external callees have a matching import
                {
                    let mut visitor = VerifyInvokeTargets::new(
                        analyzer,
                        &mut module,
                        &locals,
                        procedure.name().clone(),
                    );
                    visitor.visit_mut_procedure(&mut procedure);
                }
                module.procedures.push(Export::Procedure(procedure));
            }
            Export::Alias(mut alias) => {
                // Resolve the underlying import, and expand the `target`
                // to its fully-qualified path. This is needed because after
                // parsing, the path only contains the last component,
                // e.g. `u64` of `std::math::u64`.
                let target = &mut alias.target;
                let imported_module = match target.module.namespace() {
                    LibraryNamespace::User(ref ns) => {
                        Ident::new_unchecked(Span::new(target.span(), ns.clone()))
                    }
                    _ => unreachable!(),
                };
                if let Some(import) = module.resolve_import_mut(&imported_module) {
                    target.module = import.path.clone();
                    // Mark the backing import as used
                    import.uses += 1;
                } else {
                    // Missing import
                    analyzer.error(SemanticAnalysisError::MissingImport { span: alias.span() });
                }
                module.procedures.push(Export::Alias(alias));
            }
        }
    }

    Ok(module)
}
