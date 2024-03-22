use crate::{ast::*, diagnostics::SourceFile, Felt, LibraryPath, Span, Spanned};
use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    vec::Vec,
};

use super::{SemanticAnalysisError, SyntaxError};

/// This maintains the state for semantic analysis of a single [Module].
pub struct AnalysisContext {
    source_code: Arc<SourceFile>,
    module: Option<Box<Module>>,
    /// A map of constants to the value of that constant
    constants: BTreeMap<Ident, Constant>,
    procedures: BTreeSet<ProcedureName>,
    errors: Vec<SemanticAnalysisError>,
}

impl AnalysisContext {
    pub fn new(source_code: Arc<SourceFile>, kind: ModuleKind, path: LibraryPath) -> Self {
        let module = Box::new(Module::new(kind, path).with_source_file(Some(source_code.clone())));
        Self {
            source_code,
            module: Some(module),
            constants: Default::default(),
            procedures: Default::default(),
            errors: Default::default(),
        }
    }

    /// Define a new constant `name`, bound to `value`
    ///
    /// Returns `Err` if the symbol is already defined
    pub fn define_constant(&mut self, mut constant: Constant) -> Result<(), SyntaxError> {
        // Handle symbol conflicts before eval to make sure
        // we can catch self-referential expresssions.
        if let Some(value) = self.constants.get(&constant.name) {
            self.errors.push(SemanticAnalysisError::SymbolConflict {
                span: constant.span(),
                prev_span: value.span(),
            });
            return Ok(());
        }

        match self.const_eval(&constant.value) {
            Ok(value) => {
                constant.value = ConstantExpr::Literal(Span::new(constant.span(), value));
                self.constants.insert(constant.name.clone(), constant);
                Ok(())
            }
            Err(err) => {
                self.errors.push(err);
                let errors = core::mem::take(&mut self.errors);
                Err(SyntaxError {
                    input: self.source_code.clone(),
                    errors,
                })
            }
        }
    }

    pub fn define_import(&mut self, import: Import) -> Result<(), SyntaxError> {
        if let Err(err) = self.module.as_mut().unwrap().define_import(import) {
            match err {
                SemanticAnalysisError::ImportConflict { .. } => {
                    // Proceed anyway, to try and capture more errors
                    self.errors.push(err);
                }
                err => {
                    // We can't proceed without producing a bunch of errors
                    self.errors.push(err);
                    let errors = core::mem::take(&mut self.errors);
                    return Err(SyntaxError {
                        input: self.source_code.clone(),
                        errors,
                    });
                }
            }
        }

        Ok(())
    }

    pub fn define_procedure(&mut self, export: Export) -> Result<(), SyntaxError> {
        let name = export.name().clone();
        if let Err(err) = self.module.as_mut().unwrap().define_procedure(export) {
            match err {
                SemanticAnalysisError::SymbolConflict { .. } => {
                    // Proceed anyway, to try and capture more errors
                    self.errors.push(err);
                }
                err => {
                    // We can't proceed without producing a bunch of errors
                    self.errors.push(err);
                    let errors = core::mem::take(&mut self.errors);
                    return Err(SyntaxError {
                        input: self.source_code.clone(),
                        errors,
                    });
                }
            }
        }

        self.procedures.insert(name);

        Ok(())
    }

    fn const_eval(&self, value: &ConstantExpr) -> Result<Felt, SemanticAnalysisError> {
        match value {
            ConstantExpr::Literal(value) => Ok(value.into_inner()),
            ConstantExpr::Var(ref name) => self.get_constant(name),
            ConstantExpr::BinaryOp {
                op,
                ref lhs,
                ref rhs,
                ..
            } => {
                let rhs = self.const_eval(rhs)?;
                let lhs = self.const_eval(lhs)?;
                match op {
                    ConstantOp::Add => Ok(lhs + rhs),
                    ConstantOp::Sub => Ok(lhs - rhs),
                    ConstantOp::Mul => Ok(lhs * rhs),
                    ConstantOp::Div => Ok(lhs / rhs),
                    ConstantOp::IntDiv => Ok(Felt::new(lhs.as_int() / rhs.as_int())),
                }
            }
        }
    }

    /// Get the constant value bound to `name`
    ///
    /// Returns `Err` if the symbol is undefined
    pub fn get_constant(&self, name: &Ident) -> Result<Felt, SemanticAnalysisError> {
        let span = name.span();
        if let Some(expr) = self.constants.get(name) {
            Ok(expr.value.expect_literal())
        } else {
            Err(SemanticAnalysisError::SymbolUndefined { span })
        }
    }

    pub fn error(&mut self, diagnostic: SemanticAnalysisError) {
        self.errors.push(diagnostic);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn source_file(&self) -> Arc<SourceFile> {
        self.source_code.clone()
    }

    pub fn module(&self) -> &Module {
        self.module.as_deref().unwrap()
    }

    pub fn module_mut(&mut self) -> &mut Module {
        self.module.as_deref_mut().unwrap()
    }

    pub(super) fn take_module(&mut self) -> Box<Module> {
        self.module.take().unwrap()
    }

    pub fn into_result(mut self) -> Result<Box<Module>, SyntaxError> {
        if self.errors.is_empty() {
            Ok(self.module.take().unwrap())
        } else {
            Err(SyntaxError {
                input: self.source_code,
                errors: self.errors,
            })
        }
    }
}
