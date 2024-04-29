use super::*;

use core::fmt;

impl fmt::Debug for ModuleGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ModuleGraph")
            .field("nodes", &DisplayModuleGraphNodes(&self.modules))
            .field("graph", &DisplayModuleGraph(self))
            .finish()
    }
}

#[doc(hidden)]
struct DisplayModuleGraph<'a>(&'a ModuleGraph);

impl<'a> fmt::Debug for DisplayModuleGraph<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set()
            .entries(self.0.modules.iter().enumerate().flat_map(|(index, m)| {
                m.procedures().enumerate().filter_map(move |(i, export)| {
                    if matches!(export, Export::Alias(_)) {
                        None
                    } else {
                        let gid = GlobalProcedureIndex {
                            module: ModuleIndex::new(index),
                            index: ProcedureIndex::new(i),
                        };
                        let out_edges = self.0.callgraph.out_edges(gid);
                        Some(DisplayModuleGraphNodeWithEdges { gid, out_edges })
                    }
                })
            }))
            .finish()
    }
}

#[doc(hidden)]
struct DisplayModuleGraphNodes<'a>(&'a Vec<Arc<Module>>);

impl<'a> fmt::Debug for DisplayModuleGraphNodes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list()
            .entries(self.0.iter().enumerate().flat_map(|(index, m)| {
                m.procedures().enumerate().filter_map(move |(i, export)| {
                    if matches!(export, Export::Alias(_)) {
                        None
                    } else {
                        Some(DisplayModuleGraphNode {
                            module: ModuleIndex::new(index),
                            index: ProcedureIndex::new(i),
                            path: m.path(),
                            proc: export,
                        })
                    }
                })
            }))
            .finish()
    }
}

#[doc(hidden)]
struct DisplayModuleGraphNode<'a> {
    module: ModuleIndex,
    index: ProcedureIndex,
    path: &'a LibraryPath,
    proc: &'a Export,
}

impl<'a> fmt::Debug for DisplayModuleGraphNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
            .field("id", &format_args!("{}:{}", &self.module.as_usize(), &self.index.as_usize()))
            .field("module", &self.path)
            .field("name", &self.proc.name())
            .finish()
    }
}

#[doc(hidden)]
struct DisplayModuleGraphNodeWithEdges<'a> {
    gid: GlobalProcedureIndex,
    out_edges: &'a [GlobalProcedureIndex],
}

impl<'a> fmt::Debug for DisplayModuleGraphNodeWithEdges<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Edge")
            .field(
                "caller",
                &format_args!("{}:{}", self.gid.module.as_usize(), self.gid.index.as_usize()),
            )
            .field(
                "callees",
                &self
                    .out_edges
                    .iter()
                    .map(|gid| format!("{}:{}", gid.module.as_usize(), gid.index.as_usize()))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}
