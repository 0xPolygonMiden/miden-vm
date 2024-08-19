use alloc::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    vec::Vec,
};

use vm_core::mast::{MastForest, MastNode, MastNodeId};

/// Returns a `MastForest` where all nodes that are unreachable from all procedures are removed.
///
/// It also returns the map from old node IDs to new node IDs; or `None` if the `MastForest` was
/// unchanged. Any [`MastNodeId`] used in reference to the old [`MastForest`] should be remapped
/// using this map.
pub fn dead_code_elimination(
    mast_forest: MastForest,
) -> (MastForest, Option<BTreeMap<MastNodeId, MastNodeId>>) {
    let live_ids = compute_live_ids(&mast_forest);
    if live_ids.len() == mast_forest.num_nodes() as usize {
        return (mast_forest, None);
    }

    let (old_nodes, old_roots) = mast_forest.into_parts();
    let (live_nodes, id_remappings) = prune_nodes(old_nodes, live_ids);

    (
        build_pruned_mast_forest(live_nodes, old_roots, &id_remappings),
        Some(id_remappings),
    )
}

fn compute_live_ids(mast_forest: &MastForest) -> BTreeSet<MastNodeId> {
    let mut live_ids = BTreeSet::new();

    let mut worklist = VecDeque::from_iter(mast_forest.procedure_roots().iter().copied());
    while let Some(mast_node_id) = worklist.pop_front() {
        if !live_ids.insert(mast_node_id) {
            continue;
        }

        match &mast_forest[mast_node_id] {
            MastNode::Join(node) => {
                worklist.push_back(node.first());
                worklist.push_back(node.second());
            },
            MastNode::Split(node) => {
                worklist.push_back(node.on_true());
                worklist.push_back(node.on_false());
            },
            MastNode::Loop(node) => {
                worklist.push_back(node.body());
            },
            MastNode::Call(node) => {
                worklist.push_back(node.callee());
            },
            MastNode::Block(_) | MastNode::Dyn | MastNode::External(_) => (),
        }
    }

    live_ids
}

/// Returns the set of nodes that are live, as well as the mapping from "old ID" to "new ID" for all
/// live nodes.
fn prune_nodes(
    mast_nodes: Vec<MastNode>,
    live_ids: BTreeSet<MastNodeId>,
) -> (Vec<MastNode>, BTreeMap<MastNodeId, MastNodeId>) {
    // Note: this allows us to safely use `usize as u32`, guaranteeing that it won't wrap around.
    assert!(mast_nodes.len() < u32::MAX as usize);

    let mut pruned_nodes = Vec::with_capacity(mast_nodes.len());
    let mut id_remappings = BTreeMap::new();

    for (old_node_index, old_node) in mast_nodes.into_iter().enumerate() {
        let old_node_id: MastNodeId = (old_node_index as u32).into();

        if live_ids.contains(&old_node_id) {
            let new_node_id: MastNodeId = (pruned_nodes.len() as u32).into();
            id_remappings.insert(old_node_id, new_node_id);

            pruned_nodes.push(old_node);
        }
    }

    (pruned_nodes, id_remappings)
}

/// Rewrites all [`MastNodeId`]s in the live nodes to the correct updated IDs using `id_remappings`,
/// which maps all old node IDs to new IDs.
fn build_pruned_mast_forest(
    live_nodes: Vec<MastNode>,
    old_root_ids: Vec<MastNodeId>,
    id_remappings: &BTreeMap<MastNodeId, MastNodeId>,
) -> MastForest {
    let mut pruned_mast_forest = MastForest::new();

    // Add each live node to the new MAST forest, making sure to rewrite any outdated internal
    // `MastNodeId`s
    for live_node in live_nodes {
        match &live_node {
            MastNode::Join(join_node) => {
                let first_child =
                    id_remappings.get(&join_node.first()).copied().unwrap_or(join_node.first());
                let second_child =
                    id_remappings.get(&join_node.second()).copied().unwrap_or(join_node.second());

                pruned_mast_forest.add_join(first_child, second_child).unwrap();
            },
            MastNode::Split(split_node) => {
                let on_true_child = id_remappings
                    .get(&split_node.on_true())
                    .copied()
                    .unwrap_or(split_node.on_true());
                let on_false_child = id_remappings
                    .get(&split_node.on_false())
                    .copied()
                    .unwrap_or(split_node.on_false());

                pruned_mast_forest.add_split(on_true_child, on_false_child).unwrap();
            },
            MastNode::Loop(loop_node) => {
                let body_id =
                    id_remappings.get(&loop_node.body()).copied().unwrap_or(loop_node.body());

                pruned_mast_forest.add_loop(body_id).unwrap();
            },
            MastNode::Call(call_node) => {
                let callee_id =
                    id_remappings.get(&call_node.callee()).copied().unwrap_or(call_node.callee());

                if call_node.is_syscall() {
                    pruned_mast_forest.add_syscall(callee_id).unwrap();
                } else {
                    pruned_mast_forest.add_call(callee_id).unwrap();
                }
            },
            MastNode::Block(_) | MastNode::Dyn | MastNode::External(_) => {
                pruned_mast_forest.add_node(live_node).unwrap();
            },
        }
    }

    for old_root_id in old_root_ids {
        let new_root_id = id_remappings.get(&old_root_id).copied().unwrap_or(old_root_id);
        pruned_mast_forest.make_root(new_root_id);
    }

    pruned_mast_forest
}
