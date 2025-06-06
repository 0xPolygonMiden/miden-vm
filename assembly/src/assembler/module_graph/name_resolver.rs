use alloc::{borrow::Cow, collections::BTreeSet, vec::Vec};

use super::{ModuleGraph, WrappedModule};
use crate::{
    AssemblyError, RpoDigest, SourceSpan, Span, Spanned,
    assembler::{GlobalProcedureIndex, ModuleIndex},
    ast::{
        Ident, InvocationTarget, InvokeKind, Module, ProcedureName, QualifiedProcedureName,
        ResolvedProcedure,
    },
    diagnostics::RelatedLabel,
    library::{LibraryNamespace, LibraryPath},
};

// HELPER STRUCTS
// ================================================================================================

/// The bare minimum information needed about a module in order to include it in name resolution.
///
/// We use this to represent modules that are not yet in the [ModuleGraph], but that we need to
/// include in name resolution in order to be able to fully resolve all names for a given set of
/// modules.
struct ThinModule {
    path: LibraryPath,
    resolver: crate::ast::LocalNameResolver,
}

/// Represents the context in which names should be resolved.
///
/// A name may be resolved in different ways depending on where it is being called from, and how it
/// is being called.
#[derive(Debug, Clone)]
pub struct CallerInfo {
    /// The source span of the caller
    pub span: SourceSpan,
    /// The "where", i.e. index of the caller's module in the [ModuleGraph].
    pub module: ModuleIndex,
    /// The "how", i.e. how the callee is being invoked.
    ///
    /// This is primarily relevant for syscalls, as "local" names resolve in the kernel module,
    /// _not_ in the caller's module.
    pub kind: InvokeKind,
}

/// Represents the output of the [NameResolver] when it resolves a procedure name.
#[derive(Debug)]
pub enum ResolvedTarget {
    /// The callee was resolved to a known procedure in the [ModuleGraph]
    Exact { gid: GlobalProcedureIndex },
    /// The callee was resolved to a concrete procedure definition, and can be referenced as
    /// `target` by the caller.
    Resolved {
        /// The id of the callee procedure in the [ModuleGraph]
        gid: GlobalProcedureIndex,
        /// The [InvocationTarget] to use in the caller
        target: InvocationTarget,
    },
    /// We know the MAST root of the callee, but the procedure is not available, either in the
    /// procedure cache or in the [ModuleGraph].
    Phantom(RpoDigest),
}

impl ResolvedTarget {
    pub fn into_global_id(self) -> Option<GlobalProcedureIndex> {
        match self {
            ResolvedTarget::Exact { gid } | ResolvedTarget::Resolved { gid, .. } => Some(gid),
            ResolvedTarget::Phantom(_) => None,
        }
    }
}

// NAME RESOLVER
// ================================================================================================

/// A [NameResolver] is used to resolve a procedure invocation target to its concrete definition.
///
/// Because modules can re-export/alias the procedures of modules they import, resolving the name of
/// a procedure can require multiple steps to reach the original concrete definition of the
/// procedure.
///
/// The [NameResolver] encapsulates the tricky details of doing this, so that users of the resolver
/// need only provide a reference to a [ModuleGraph], a name they wish to resolve, and some
/// information about the caller necessary to determine the context in which the name should be
/// resolved.
pub struct NameResolver<'a> {
    /// The graph containing already-compiled and partially-resolved modules.
    graph: &'a ModuleGraph,
    /// The set of modules which are being added to `graph`, but which have not been fully
    /// processed yet.
    pending: Vec<ThinModule>,
}

impl<'a> NameResolver<'a> {
    /// Create a new [NameResolver] for the provided [ModuleGraph].
    pub fn new(graph: &'a ModuleGraph) -> Self {
        Self { graph, pending: vec![] }
    }

    /// Add a module to the set of "pending" modules this resolver will consult when doing
    /// resolution.
    ///
    /// Pending modules are those which are being added to the underlying [ModuleGraph], but which
    /// have not been processed yet. When resolving names we may need to visit those modules to
    /// determine the location of the actual definition, but they do not need to be fully
    /// validated/processed to do so.
    ///
    /// This is typically called when we begin processing the pending modules, by adding those we
    /// have not yet processed to the resolver, as we resolve names for each module in the set.
    pub fn push_pending(&mut self, module: &Module) {
        self.pending.push(ThinModule {
            path: module.path().clone(),
            resolver: module.resolver(),
        });
    }

    /// Resolve `target`, a possibly-resolved callee identifier, to a [ResolvedTarget], using
    /// `caller` as the context.
    pub fn resolve_target(
        &self,
        caller: &CallerInfo,
        target: &InvocationTarget,
    ) -> Result<ResolvedTarget, AssemblyError> {
        match target {
            InvocationTarget::MastRoot(mast_root) => {
                match self.graph.get_procedure_index_by_digest(mast_root) {
                    None => Ok(ResolvedTarget::Phantom(mast_root.into_inner())),
                    Some(gid) => Ok(ResolvedTarget::Exact { gid }),
                }
            },
            InvocationTarget::ProcedureName(callee) => self.resolve(caller, callee),
            InvocationTarget::ProcedurePath { name, module: imported_module } => {
                match self.resolve_import(caller, imported_module) {
                    Some(imported_module) => {
                        let fqn = QualifiedProcedureName {
                            span: target.span(),
                            module: imported_module.into_inner().clone(),
                            name: name.clone(),
                        };
                        let gid = self.find(caller, &fqn)?;
                        let path = self.module_path(gid.module);
                        let pending_offset = self.graph.modules.len();
                        let name = if gid.module.as_usize() >= pending_offset {
                            self.pending[gid.module.as_usize() - pending_offset]
                                .resolver
                                .get_name(gid.index)
                                .clone()
                        } else {
                            self.graph.get_procedure_unsafe(gid).name().clone()
                        };
                        Ok(ResolvedTarget::Resolved {
                            gid,
                            target: InvocationTarget::AbsoluteProcedurePath { name, path },
                        })
                    },
                    None => Err(AssemblyError::UndefinedModule {
                        span: caller.span,
                        source_file: self.graph.source_manager.get(caller.span.source_id()).ok(),
                        path: LibraryPath::new_from_components(
                            LibraryNamespace::User(imported_module.clone().into_inner()),
                            [],
                        ),
                    }),
                }
            },
            InvocationTarget::AbsoluteProcedurePath { name, path } => {
                let fqn = QualifiedProcedureName {
                    span: target.span(),
                    module: path.clone(),
                    name: name.clone(),
                };
                let gid = self.find(caller, &fqn)?;
                Ok(ResolvedTarget::Exact { gid })
            },
        }
    }

    /// Resolver `callee` to a [ResolvedTarget], using `caller` as the context in which `callee`
    /// should be resolved.
    fn resolve(
        &self,
        caller: &CallerInfo,
        callee: &ProcedureName,
    ) -> Result<ResolvedTarget, AssemblyError> {
        match self.resolve_local(caller, callee) {
            Some(ResolvedProcedure::Local(index)) if matches!(caller.kind, InvokeKind::SysCall) => {
                let gid = GlobalProcedureIndex {
                    module: self.graph.kernel_index.unwrap(),
                    index: index.into_inner(),
                };
                Ok(ResolvedTarget::Exact { gid })
            },
            Some(ResolvedProcedure::Local(index)) => {
                let gid = GlobalProcedureIndex {
                    module: caller.module,
                    index: index.into_inner(),
                };
                Ok(ResolvedTarget::Exact { gid })
            },
            Some(ResolvedProcedure::External(ref fqn)) => {
                let gid = self.find(caller, fqn)?;
                let path = self.module_path(gid.module);
                let pending_offset = self.graph.modules.len();
                let name = if gid.module.as_usize() >= pending_offset {
                    self.pending[gid.module.as_usize() - pending_offset]
                        .resolver
                        .get_name(gid.index)
                        .clone()
                } else {
                    self.graph.get_procedure_unsafe(gid).name().clone()
                };
                Ok(ResolvedTarget::Resolved {
                    gid,
                    target: InvocationTarget::AbsoluteProcedurePath { name, path },
                })
            },
            Some(ResolvedProcedure::MastRoot(ref digest)) => {
                match self.graph.get_procedure_index_by_digest(digest) {
                    Some(gid) => Ok(ResolvedTarget::Exact { gid }),
                    None => Ok(ResolvedTarget::Phantom(*digest)),
                }
            },
            None => Err(AssemblyError::Failed {
                labels: vec![
                    RelatedLabel::error("undefined procedure")
                        .with_source_file(
                            self.graph.source_manager.get(caller.span.source_id()).ok(),
                        )
                        .with_labeled_span(caller.span, "unable to resolve this name locally"),
                ]
                .into(),
            }),
        }
    }

    /// Resolve `name`, the name of an imported module, to a [LibraryPath], using `caller` as the
    /// context.
    fn resolve_import(&self, caller: &CallerInfo, name: &Ident) -> Option<Span<&LibraryPath>> {
        let pending_offset = self.graph.modules.len();
        if caller.module.as_usize() >= pending_offset {
            self.pending[caller.module.as_usize() - pending_offset]
                .resolver
                .resolve_import(name)
        } else {
            match &self.graph[caller.module] {
                WrappedModule::Ast(module) => module
                    .resolve_import(name)
                    .map(|import| Span::new(import.span(), import.path())),
                WrappedModule::Info(_) => None,
            }
        }
    }

    fn resolve_local(
        &self,
        caller: &CallerInfo,
        callee: &ProcedureName,
    ) -> Option<ResolvedProcedure> {
        let module = if matches!(caller.kind, InvokeKind::SysCall) {
            // Resolve local names relative to the kernel
            self.graph.kernel_index?
        } else {
            caller.module
        };
        self.resolve_local_with_index(module, callee)
    }

    fn resolve_local_with_index(
        &self,
        module: ModuleIndex,
        callee: &ProcedureName,
    ) -> Option<ResolvedProcedure> {
        let pending_offset = self.graph.modules.len();
        let module_index = module.as_usize();
        if module_index >= pending_offset {
            self.pending[module_index - pending_offset].resolver.resolve(callee)
        } else {
            self.graph[module].resolve(callee)
        }
    }

    /// Resolve `callee` to its concrete definition, returning the corresponding
    /// [GlobalProcedureIndex].
    ///
    /// If an error occurs during resolution, or the name cannot be resolved, `Err` is returned.
    fn find(
        &self,
        caller: &CallerInfo,
        callee: &QualifiedProcedureName,
    ) -> Result<GlobalProcedureIndex, AssemblyError> {
        // If the caller is a syscall, set the invoke kind to `ProcRef` until we have resolved the
        // procedure, then verify that it is in the kernel module. This bypasses validation until
        // after resolution
        let mut current_caller = if matches!(caller.kind, InvokeKind::SysCall) {
            let mut caller = caller.clone();
            caller.kind = InvokeKind::ProcRef;
            Cow::Owned(caller)
        } else {
            Cow::Borrowed(caller)
        };
        let mut current_callee = Cow::Borrowed(callee);
        let mut visited = BTreeSet::default();
        loop {
            let module_index = self.find_module_index(&current_callee.module).ok_or_else(|| {
                AssemblyError::UndefinedModule {
                    span: current_caller.span,
                    source_file: self
                        .graph
                        .source_manager
                        .get(current_caller.span.source_id())
                        .ok(),
                    path: current_callee.module.clone(),
                }
            })?;
            let resolved = self.resolve_local_with_index(module_index, &current_callee.name);
            match resolved {
                Some(ResolvedProcedure::Local(index)) => {
                    let id = GlobalProcedureIndex {
                        module: module_index,
                        index: index.into_inner(),
                    };
                    if matches!(current_caller.kind, InvokeKind::SysCall if self.graph.kernel_index != Some(module_index))
                    {
                        break Err(AssemblyError::InvalidSysCallTarget {
                            span: current_caller.span,
                            source_file: self
                                .graph
                                .source_manager
                                .get(current_caller.span.source_id())
                                .ok(),
                            callee: current_callee.into_owned().into(),
                        });
                    }
                    break Ok(id);
                },
                Some(ResolvedProcedure::External(fqn)) => {
                    // If we see that we're about to enter an infinite
                    // resolver loop because of a recursive alias, return
                    // an error
                    if !visited.insert(fqn.clone()) {
                        break Err(AssemblyError::Failed {
                            labels: vec![
                                RelatedLabel::error("recursive alias")
                                    .with_source_file(self.graph.source_manager.get(fqn.span().source_id()).ok())
                                    .with_labeled_span(fqn.span(), "occurs because this import causes import resolution to loop back on itself"),
                                RelatedLabel::advice("recursive alias")
                                    .with_source_file(self.graph.source_manager.get(caller.span.source_id()).ok())
                                    .with_labeled_span(caller.span, "as a result of resolving this procedure reference"),
                            ].into(),
                        });
                    }
                    current_caller = Cow::Owned(CallerInfo {
                        span: fqn.span(),
                        module: module_index,
                        kind: current_caller.kind,
                    });
                    current_callee = Cow::Owned(fqn);
                },
                Some(ResolvedProcedure::MastRoot(ref digest)) => {
                    if let Some(id) = self.graph.get_procedure_index_by_digest(digest) {
                        break Ok(id);
                    }
                    // This is a phantom procedure - we know its root, but do not have its
                    // definition
                    break Err(AssemblyError::Failed {
                        labels: vec![
                            RelatedLabel::error("undefined procedure")
                                .with_source_file(
                                    self.graph.source_manager.get(caller.span.source_id()).ok(),
                                )
                                .with_labeled_span(
                                    caller.span,
                                    "unable to resolve this reference to its definition",
                                ),
                            RelatedLabel::error("name resolution cannot proceed")
                                .with_source_file(
                                    self.graph
                                        .source_manager
                                        .get(current_callee.span().source_id())
                                        .ok(),
                                )
                                .with_labeled_span(
                                    current_callee.span(),
                                    "this name cannot be resolved",
                                ),
                        ]
                        .into(),
                    });
                },
                None if matches!(current_caller.kind, InvokeKind::SysCall) => {
                    if self.graph.has_nonempty_kernel() {
                        // No kernel, so this invoke is invalid anyway
                        break Err(AssemblyError::Failed {
                            labels: vec![
                                RelatedLabel::error("undefined kernel procedure")
                                    .with_source_file(self.graph.source_manager.get(caller.span.source_id()).ok())
                                    .with_labeled_span(caller.span, "unable to resolve this reference to a procedure in the current kernel"),
                                RelatedLabel::error("invalid syscall")
                                    .with_source_file(self.graph.source_manager.get(current_callee.span().source_id()).ok())
                                    .with_labeled_span(
                                        current_callee.span(),
                                        "this name cannot be resolved, because the assembler has an empty kernel",
                                    ),
                            ].into()
                        });
                    } else {
                        // No such kernel procedure
                        break Err(AssemblyError::Failed {
                            labels: vec![
                                RelatedLabel::error("undefined kernel procedure")
                                    .with_source_file(self.graph.source_manager.get(caller.span.source_id()).ok())
                                    .with_labeled_span(caller.span, "unable to resolve this reference to a procedure in the current kernel"),
                                RelatedLabel::error("name resolution cannot proceed")
                                    .with_source_file(self.graph.source_manager.get(current_callee.span().source_id()).ok())
                                    .with_labeled_span(
                                        current_callee.span(),
                                        "this name cannot be resolved",
                                    ),
                            ].into()
                        });
                    }
                },
                None => {
                    // No such procedure known to `module`
                    break Err(AssemblyError::Failed {
                        labels: vec![
                            RelatedLabel::error("undefined procedure")
                                .with_source_file(
                                    self.graph.source_manager.get(caller.span.source_id()).ok(),
                                )
                                .with_labeled_span(
                                    caller.span,
                                    "unable to resolve this reference to its definition",
                                ),
                            RelatedLabel::error("name resolution cannot proceed")
                                .with_source_file(
                                    self.graph
                                        .source_manager
                                        .get(current_callee.span().source_id())
                                        .ok(),
                                )
                                .with_labeled_span(
                                    current_callee.span(),
                                    "this name cannot be resolved",
                                ),
                        ]
                        .into(),
                    });
                },
            }
        }
    }

    /// Resolve a [LibraryPath] to a [ModuleIndex] in this graph
    fn find_module_index(&self, name: &LibraryPath) -> Option<ModuleIndex> {
        self.graph
            .modules
            .iter()
            .map(|m| m.path())
            .chain(self.pending.iter().map(|m| &m.path))
            .position(|path| path == name)
            .map(ModuleIndex::new)
    }

    fn module_path(&self, module: ModuleIndex) -> LibraryPath {
        let pending_offset = self.graph.modules.len();
        let module_index = module.as_usize();
        if module_index >= pending_offset {
            self.pending[module_index - pending_offset].path.clone()
        } else {
            self.graph[module].path().clone()
        }
    }
}
