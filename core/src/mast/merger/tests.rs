use miden_crypto::{hash::rpo::RpoDigest, ONE};

use super::*;
use crate::{Decorator, Operation};

fn block_foo() -> MastNode {
    MastNode::new_basic_block(vec![Operation::Mul, Operation::Add], None).unwrap()
}

fn block_bar() -> MastNode {
    MastNode::new_basic_block(vec![Operation::And, Operation::Eq], None).unwrap()
}

fn block_qux() -> MastNode {
    MastNode::new_basic_block(vec![Operation::Swap, Operation::Push(ONE)], None).unwrap()
}

fn assert_contains_node_once(forest: &MastForest, digest: RpoDigest) {
    assert_eq!(forest.nodes.iter().filter(|node| node.digest() == digest).count(), 1);
}

/// Tests that Call(bar) still correctly calls the remapped bar block.
///
/// [Block(foo), Call(foo)]
/// +
/// [Block(bar), Call(bar)]
/// =
/// [Block(foo), Call(foo), Block(bar), Call(bar)]
#[test]
fn mast_forest_merge_remap() {
    let mut forest_a = MastForest::new();
    let id_foo = forest_a.add_node(block_foo()).unwrap();
    let id_call_a = forest_a.add_call(id_foo).unwrap();
    forest_a.make_root(id_call_a);

    let mut forest_b = MastForest::new();
    let id_bar = forest_b.add_node(block_bar()).unwrap();
    let id_call_b = forest_b.add_call(id_bar).unwrap();
    forest_b.make_root(id_call_b);

    let merged = forest_a.merge(&forest_b).unwrap();

    assert_eq!(merged.nodes().len(), 4);
    assert_eq!(merged.nodes()[0], block_foo());
    assert_matches!(&merged.nodes()[1], MastNode::Call(call_node) if call_node.callee().as_u32() == 0);
    assert_eq!(merged.nodes()[2], block_bar());
    assert_matches!(&merged.nodes()[3], MastNode::Call(call_node) if call_node.callee().as_u32() == 2);
}

/// Tests that Forest_A + Forest_A = Forest_A (i.e. duplicates are removed).
#[test]
fn mast_forest_merge_duplicate() {
    let mut forest_a = MastForest::new();
    forest_a.add_decorator(Decorator::Debug(crate::DebugOptions::MemAll)).unwrap();
    forest_a.add_decorator(Decorator::Trace(25)).unwrap();

    let id_external = forest_a.add_external(block_bar().digest()).unwrap();
    let id_foo = forest_a.add_node(block_foo()).unwrap();
    let id_call = forest_a.add_call(id_foo).unwrap();
    let id_loop = forest_a.add_loop(id_external).unwrap();
    forest_a.make_root(id_call);
    forest_a.make_root(id_loop);

    let merged = forest_a.merge(&forest_a).unwrap();

    for merged_root in merged.procedure_digests() {
        forest_a.procedure_digests().find(|root| root == &merged_root).unwrap();
    }

    for merged_node in merged.nodes().iter().map(MastNode::digest) {
        forest_a.nodes.iter().find(|node| node.digest() == merged_node).unwrap();
    }

    for merged_decorator in merged.decorators.iter() {
        assert!(forest_a.decorators.contains(merged_decorator));
    }
}

/// Tests that External(foo) is replaced by Block(foo) whether it is in forest A or B, and the
/// duplicate Call is removed.
///
/// [External(foo), Call(foo)]
/// +
/// [Block(foo), Call(foo)]
/// =
/// [Block(foo), Call(foo)]
/// +
/// [External(foo), Call(foo)]
/// =
/// [Block(foo), Call(foo)]
#[test]
fn mast_forest_merge_replace_external() {
    let mut forest_a = MastForest::new();
    let id_foo_a = forest_a.add_external(block_foo().digest()).unwrap();
    let id_call_a = forest_a.add_call(id_foo_a).unwrap();
    forest_a.make_root(id_call_a);

    let mut forest_b = MastForest::new();
    let id_foo_b = forest_b.add_node(block_foo()).unwrap();
    let id_call_b = forest_b.add_call(id_foo_b).unwrap();
    forest_b.make_root(id_call_b);

    let merged_ab = forest_a.merge(&forest_b).unwrap();
    let merged_ba = forest_b.merge(&forest_a).unwrap();

    for merged in [merged_ab, merged_ba] {
        assert_eq!(merged.nodes().len(), 2);
        assert_eq!(merged.nodes()[0], block_foo());
        assert_matches!(&merged.nodes()[1], MastNode::Call(call_node) if call_node.callee().as_u32() == 0);
    }
}

/// Test that roots are preserved and deduplicated if appropriate.
///
/// Nodes: [Block(foo), Call(foo)]
/// Roots: [Call(foo)]
/// +
/// Nodes: [Block(foo), Block(bar), Call(foo)]
/// Roots: [Block(bar), Call(foo)]
/// =
/// Nodes: [Block(foo), Block(bar), Call(foo)]
/// Roots: [Block(bar), Call(foo)]
#[test]
fn mast_forest_merge_roots() {
    let mut forest_a = MastForest::new();
    let id_foo_a = forest_a.add_node(block_foo()).unwrap();
    let call_a = forest_a.add_call(id_foo_a).unwrap();
    forest_a.make_root(call_a);

    let mut forest_b = MastForest::new();
    let id_foo_b = forest_b.add_node(block_foo()).unwrap();
    let id_bar_b = forest_b.add_node(block_bar()).unwrap();
    let call_b = forest_b.add_call(id_foo_b).unwrap();
    forest_b.make_root(id_bar_b);
    forest_b.make_root(call_b);

    let root_digest_call_a = forest_a.get_node_by_id(call_a).unwrap().digest();
    let root_digest_bar_b = forest_b.get_node_by_id(id_bar_b).unwrap().digest();
    let root_digest_call_b = forest_b.get_node_by_id(call_b).unwrap().digest();

    let merged = forest_a.merge(&forest_b).unwrap();

    // Asserts (together with the other assertions) that the duplicate Call(foo) roots have been
    // deduplicated.
    assert_eq!(merged.procedure_roots().len(), 2);

    // Assert that all root digests from A an B are still roots in the merged forest.
    let root_digests = merged.procedure_digests().collect::<Vec<_>>();
    assert!(root_digests.contains(&root_digest_call_a));
    assert!(root_digests.contains(&root_digest_bar_b));
    assert!(root_digests.contains(&root_digest_call_b));
}

/// Test that multiple trees can be merged when the same merger is reused.
///
/// Nodes: [Block(foo), Call(foo)]
/// Roots: [Call(foo)]
/// +
/// Nodes: [Block(foo), Block(bar), Call(foo)]
/// Roots: [Block(bar), Call(foo)]
/// +
/// Nodes: [Block(foo), Block(qux), Call(foo)]
/// Roots: [Block(qux), Call(foo)]
/// =
/// Nodes: [Block(foo), Block(bar), Block(qux), Call(foo)]
/// Roots: [Block(bar), Block(qux), Call(foo)]
#[test]
fn mast_forest_merge_multiple() {
    let mut forest_a = MastForest::new();
    let id_foo_a = forest_a.add_node(block_foo()).unwrap();
    let call_a = forest_a.add_call(id_foo_a).unwrap();
    forest_a.make_root(call_a);

    let mut forest_b = MastForest::new();
    let id_foo_b = forest_b.add_node(block_foo()).unwrap();
    let id_bar_b = forest_b.add_node(block_bar()).unwrap();
    let call_b = forest_b.add_call(id_foo_b).unwrap();
    forest_b.make_root(id_bar_b);
    forest_b.make_root(call_b);

    let mut forest_c = MastForest::new();
    let id_foo_c = forest_c.add_node(block_foo()).unwrap();
    let id_qux_c = forest_c.add_node(block_qux()).unwrap();
    let call_c = forest_c.add_call(id_foo_c).unwrap();
    forest_c.make_root(id_qux_c);
    forest_c.make_root(call_c);

    let merged = forest_a.merge_multiple(&[&forest_b, &forest_c]).unwrap();

    let block_foo_digest = forest_b.get_node_by_id(id_foo_b).unwrap().digest();
    let block_bar_digest = forest_b.get_node_by_id(id_bar_b).unwrap().digest();
    let call_foo_digest = forest_b.get_node_by_id(call_b).unwrap().digest();
    let block_qux_digest = forest_c.get_node_by_id(id_qux_c).unwrap().digest();

    assert_eq!(merged.procedure_roots().len(), 3);

    let root_digests = merged.procedure_digests().collect::<Vec<_>>();
    assert!(root_digests.contains(&call_foo_digest));
    assert!(root_digests.contains(&block_bar_digest));
    assert!(root_digests.contains(&block_qux_digest));

    assert_contains_node_once(&merged, block_foo_digest);
    assert_contains_node_once(&merged, block_bar_digest);
    assert_contains_node_once(&merged, block_qux_digest);
    assert_contains_node_once(&merged, call_foo_digest);
}

/// Tests that decorators are merged and that nodes who are identical except for their
/// decorators are not deduplicated.
///
/// Note in particular that the `Loop` nodes only differ in their decorator which ensures that
/// the merging takes decorators into account.
///
/// Nodes: [Block(foo, [Trace(1), Trace(2)]), Loop(foo, [Trace(0), Trace(2)])]
/// Decorators: [Trace(0), Trace(1), Trace(2)]
/// +
/// Nodes: [Block(foo, [Trace(1), Trace(2)]), Loop(foo, [Trace(1), Trace(3)])]
/// Decorators: [Trace(1), Trace(2), Trace(3)]
/// =
/// Nodes: [
///   Block(foo, [Trace(1), Trace(2)]),
///   Loop(foo, [Trace(0), Trace(2)]),
///   Loop(foo, [Trace(1), Trace(3)]),
/// ]
/// Decorators: [Trace(0), Trace(1), Trace(2), Trace(3)]
#[test]
fn mast_forest_merge_decorators() {
    let mut forest_a = MastForest::new();
    let trace0 = Decorator::Trace(0);
    let trace1 = Decorator::Trace(1);
    let trace2 = Decorator::Trace(2);
    let trace3 = Decorator::Trace(3);

    // Build Forest A
    let deco0_a = forest_a.add_decorator(trace0.clone()).unwrap();
    let deco1_a = forest_a.add_decorator(trace1.clone()).unwrap();
    let deco2_a = forest_a.add_decorator(trace2.clone()).unwrap();

    let mut foo_node_a = block_foo();
    foo_node_a.set_before_enter(vec![deco1_a, deco2_a]);
    let id_foo_a = forest_a.add_node(foo_node_a).unwrap();

    let mut loop_node_a = MastNode::new_loop(id_foo_a, &forest_a).unwrap();
    loop_node_a.set_after_exit(vec![deco0_a, deco2_a]);
    let id_loop_a = forest_a.add_node(loop_node_a).unwrap();

    forest_a.make_root(id_loop_a);

    // Build Forest B
    let mut forest_b = MastForest::new();
    let deco1_b = forest_b.add_decorator(trace1.clone()).unwrap();
    let deco2_b = forest_b.add_decorator(trace2.clone()).unwrap();
    let deco3_b = forest_b.add_decorator(trace3.clone()).unwrap();

    // This foo node is identical to the one in A, including its decorators.
    let mut foo_node_b = block_foo();
    foo_node_b.set_before_enter(vec![deco1_b, deco2_b]);
    let id_foo_b = forest_b.add_node(foo_node_b).unwrap();

    // This loop node's decorators are different from the loop node in a.
    let mut loop_node_b = MastNode::new_loop(id_foo_b, &forest_b).unwrap();
    loop_node_b.set_after_exit(vec![deco1_b, deco3_b]);
    let id_loop_b = forest_b.add_node(loop_node_b).unwrap();

    forest_b.make_root(id_loop_b);

    let merged = forest_a.merge(&forest_b).unwrap();

    // There are 4 unique decorators across both forests.
    assert_eq!(merged.decorators.len(), 4);
    assert!(merged.decorators.contains(&trace0));
    assert!(merged.decorators.contains(&trace1));
    assert!(merged.decorators.contains(&trace2));
    assert!(merged.decorators.contains(&trace3));

    let find_decorator_id = |deco: &Decorator| {
        let idx = merged
            .decorators
            .iter()
            .enumerate()
            .find_map(
                |(deco_id, forest_deco)| if forest_deco == deco { Some(deco_id) } else { None },
            )
            .unwrap();
        DecoratorId::from_u32_safe(idx as u32, &merged).unwrap()
    };

    let merged_deco0 = find_decorator_id(&trace0);
    let merged_deco1 = find_decorator_id(&trace1);
    let merged_deco2 = find_decorator_id(&trace2);
    let merged_deco3 = find_decorator_id(&trace3);

    assert_eq!(merged.nodes.len(), 3);

    let merged_foo_block = merged.nodes.iter().find(|node| node.is_basic_block()).unwrap();
    let MastNode::Block(merged_foo_block) = merged_foo_block else {
        panic!("expected basic block node");
    };

    assert_eq!(
        merged_foo_block.decorators().as_slice(),
        &[(0, merged_deco1), (0, merged_deco2)]
    );

    // Asserts that there exists exactly one Loop Node with the given decorators.
    assert_eq!(
        merged
            .nodes
            .iter()
            .filter(|node| {
                if let MastNode::Loop(loop_node) = node {
                    loop_node.after_exit() == [merged_deco0, merged_deco2]
                } else {
                    false
                }
            })
            .count(),
        1
    );

    // Asserts that there exists exactly one Loop Node with the given decorators.
    assert_eq!(
        merged
            .nodes
            .iter()
            .filter(|node| {
                if let MastNode::Loop(loop_node) = node {
                    loop_node.after_exit() == [merged_deco1, merged_deco3]
                } else {
                    false
                }
            })
            .count(),
        1
    );
}
