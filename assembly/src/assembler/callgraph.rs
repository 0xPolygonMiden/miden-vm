use alloc::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    vec::Vec,
};

use super::{GlobalProcedureIndex, ModuleIndex};

/// Represents the inability to construct a topological ordering of the nodes in a [CallGraph]
/// due to a cycle in the graph, which can happen due to recursion.
#[derive(Debug)]
pub struct CycleError(BTreeSet<GlobalProcedureIndex>);

impl CycleError {
    pub fn into_node_ids(self) -> impl ExactSizeIterator<Item = GlobalProcedureIndex> {
        self.0.into_iter()
    }
}

/// A [CallGraph] is a directed, acyclic graph which represents all of the edges between
/// procedures formed by a caller/callee relationship.
///
/// More precisely, this graph can be used to perform the following analyses:
///
/// * What is the maximum call stack depth for a program?
/// * Are there any recursive procedure calls?
/// * Are there procedures which are unreachable from the program entrypoint?, i.e. dead code
/// * What is the set of procedures which are reachable from a given procedure, and which of
/// those are (un)conditionally called?
///
/// A [CallGraph] is the actual graph underpinning the [ModuleGraph] data structure, and the
/// two are intrinsically linked to one another (i.e. a [CallGraph] is meaningless without
/// the corresponding [ModuleGraph]).
#[derive(Default, Clone)]
pub struct CallGraph {
    /// The adjacency matrix for procedures in the call graph
    nodes: BTreeMap<GlobalProcedureIndex, Vec<GlobalProcedureIndex>>,
}

impl CallGraph {
    /// Get the set of edges from the given caller to its callees in the graph
    pub fn out_edges(&self, gid: GlobalProcedureIndex) -> &[GlobalProcedureIndex] {
        self.nodes.get(&gid).map(|out_edges| out_edges.as_slice()).unwrap_or(&[])
    }

    /// Inserts a node in the graph for `id`, if not already present.
    ///
    /// Returns the set of [ProcedureId] which are the outbound neighbors of `id` in the graph,
    /// i.e. the callees of a call-like instruction.
    pub fn get_or_insert_node(
        &mut self,
        id: GlobalProcedureIndex,
    ) -> &mut Vec<GlobalProcedureIndex> {
        self.nodes.entry(id).or_default()
    }

    /// Add an edge in the call graph from `caller` to `callee`.
    ///
    /// If introducing this edge will cause a cycle in the graph, then `Err` is returned, and
    /// the edge is not added.
    ///
    /// NOTE: This function performs a topological sort of the graph to perform the cycle check,
    /// which can be expensive. If you need to add many edges at once, use [add_edge_unchecked], and
    /// then when ready, call [toposort] to verify that there are no cycles.
    #[allow(unused)]
    pub fn add_edge(
        &mut self,
        caller: GlobalProcedureIndex,
        callee: GlobalProcedureIndex,
    ) -> Result<(), CycleError> {
        if caller == callee {
            return Err(CycleError(BTreeSet::from_iter([caller, callee])));
        }

        // Insert the edge
        self.add_edge_unchecked(caller, callee);

        // Verify the new edge does not introduce cycles in the graph
        let causes_cycle = self.toposort_caller(caller).is_err();
        if causes_cycle {
            // Remove the edge we just inserted
            self.nodes.get_mut(&caller).unwrap().pop();
        }

        Ok(())
    }

    /// Add an edge in the call graph from `caller` to `callee`.
    ///
    /// This version differs from [add_edge], in that no cycle check is performed, which is useful
    /// when constructing the graph in bulk, and validating at the end. However, it is essential
    /// that after adding an edge using this API, that you at some point prior to compilation,
    /// check the validity of the graph using [toposort].
    pub fn add_edge_unchecked(
        &mut self,
        caller: GlobalProcedureIndex,
        callee: GlobalProcedureIndex,
    ) {
        assert_ne!(caller, callee, "a procedure cannot call itself");

        // Make sure the callee is in the graph
        self.get_or_insert_node(callee);
        // Make sure the caller is in the graph
        let callees = self.get_or_insert_node(caller);
        // If the caller already references the callee, we're done
        if callees.contains(&callee) {
            return;
        }

        callees.push(callee);
    }

    /// Removes all edges to/from a procedure in `module`
    ///
    /// NOTE: If a procedure that is removed has predecessors (callers) in the graph, this will
    /// remove those edges, and the graph will be incomplete and not reflect the "true" call graph.
    /// In practice, we are recomputing the graph after making such modifications, so this a
    /// temporary state of affairs - still, it is important to be aware of this behavior.
    pub fn remove_edges_for_module(&mut self, module: ModuleIndex) {
        for (_, out_edges) in self.nodes.iter_mut() {
            out_edges.retain(|gid| gid.module != module);
        }
        self.nodes.retain(|gid, _| gid.module != module);
    }

    /// Removes the edge between `caller` and `callee` from the graph
    pub fn remove_edge(&mut self, caller: GlobalProcedureIndex, callee: GlobalProcedureIndex) {
        if let Some(out_edges) = self.nodes.get_mut(&caller) {
            out_edges.retain(|n| *n != callee);
        }
    }

    /// Returns the number of predecessors of `id` in the graph, i.e.
    /// the number of procedures which call `id`.
    pub fn num_predecessors(&self, id: GlobalProcedureIndex) -> usize {
        self.nodes.iter().filter(|(_, out_edges)| out_edges.contains(&id)).count()
    }

    /// Construct the topological ordering of all nodes in the call graph.
    ///
    /// Returns `Err` if a cycle is detected in the graph
    pub fn toposort(&self) -> Result<Vec<GlobalProcedureIndex>, CycleError> {
        if self.nodes.is_empty() {
            return Ok(vec![]);
        }

        let mut output = Vec::with_capacity(self.nodes.len());
        let mut graph = self.clone();

        // Build the set of roots by finding all nodes
        // that have no predecessors
        let mut has_preds = BTreeSet::default();
        for (_node, out_edges) in graph.nodes.iter() {
            for succ in out_edges.iter() {
                has_preds.insert(*succ);
            }
        }
        let mut roots =
            VecDeque::from_iter(graph.nodes.keys().copied().filter(|n| !has_preds.contains(n)));

        // If all nodes have predecessors, there must be a cycle, so
        // just pick a node and let the algorithm find the cycle for
        // that node so we have a useful error. Set a flag so that we
        // can assert that the cycle was actually found as a sanity
        // check
        let mut expect_cycle = false;
        if roots.is_empty() {
            expect_cycle = true;
            roots.extend(graph.nodes.keys().next());
        }

        let mut successors = Vec::with_capacity(4);
        while let Some(id) = roots.pop_front() {
            output.push(id);
            successors.clear();
            successors.extend(graph.nodes[&id].iter().copied());
            for mid in successors.drain(..) {
                graph.remove_edge(id, mid);
                if graph.num_predecessors(mid) == 0 {
                    roots.push_back(mid);
                }
            }
        }

        let has_cycle = graph
            .nodes
            .iter()
            .any(|(n, out_edges)| output.contains(n) && !out_edges.is_empty());
        if has_cycle {
            let mut in_cycle = BTreeSet::default();
            for (n, edges) in graph.nodes.iter() {
                if edges.is_empty() {
                    continue;
                }
                in_cycle.insert(*n);
                in_cycle.extend(edges.as_slice());
            }
            Err(CycleError(in_cycle))
        } else {
            assert!(!expect_cycle, "we expected a cycle to be found, but one was not identified");
            Ok(output)
        }
    }

    /// Get a new graph which is a subgraph of `self` containing all of
    /// the nodes reachable from `root`, and nothing else.
    pub fn subgraph(&self, root: GlobalProcedureIndex) -> Self {
        let mut worklist = VecDeque::from_iter([root]);
        let mut graph = Self::default();
        let mut visited = BTreeSet::default();

        while let Some(gid) = worklist.pop_front() {
            if !visited.insert(gid) {
                continue;
            }

            let new_successors = graph.get_or_insert_node(gid);
            let prev_successors = self.out_edges(gid);
            worklist.extend(prev_successors.iter().cloned());
            new_successors.extend_from_slice(prev_successors);
        }

        graph
    }

    /// Construct the topological ordering of nodes in the call graph,
    /// for which `caller` is an ancestor.
    ///
    /// Returns `Err` if a cycle is detected in the graph
    pub fn toposort_caller(
        &self,
        caller: GlobalProcedureIndex,
    ) -> Result<Vec<GlobalProcedureIndex>, CycleError> {
        let mut output = Vec::with_capacity(self.nodes.len());

        // Build a subgraph of `self` containing only those nodes
        // reachable from `caller`
        let mut graph = self.subgraph(caller);

        // Remove all predecessor edges to `caller`
        graph.nodes.iter_mut().for_each(|(_pred, out_edges)| {
            out_edges.retain(|n| *n != caller);
        });

        let mut roots = VecDeque::from_iter([caller]);
        let mut successors = Vec::with_capacity(4);
        while let Some(id) = roots.pop_front() {
            output.push(id);
            successors.clear();
            successors.extend(graph.nodes[&id].iter().copied());
            for mid in successors.drain(..) {
                graph.remove_edge(id, mid);
                if graph.num_predecessors(mid) == 0 {
                    roots.push_back(mid);
                }
            }
        }

        let has_cycle = graph
            .nodes
            .iter()
            .any(|(n, out_edges)| output.contains(n) && !out_edges.is_empty());
        if has_cycle {
            let mut in_cycle = BTreeSet::default();
            for (n, edges) in graph.nodes.iter() {
                if edges.is_empty() {
                    continue;
                }
                in_cycle.insert(*n);
                in_cycle.extend(edges.as_slice());
            }
            Err(CycleError(in_cycle))
        } else {
            Ok(output)
        }
    }
}
