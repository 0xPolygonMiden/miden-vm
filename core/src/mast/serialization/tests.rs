use alloc::string::ToString;
use miden_crypto::{hash::rpo::RpoDigest, Felt};

use super::*;
use crate::{
    operations::Operation, AdviceInjector, AssemblyOp, DebugOptions, Decorator, SignatureKind,
};

/// If this test fails to compile, it means that `Operation` or `Decorator` was changed. Make sure
/// that all tests in this file are updated accordingly. For example, if a new `Operation` variant
/// was added, make sure that you add it in the vector of operations in
/// [`serialize_deserialize_all_nodes`].
#[test]
fn confirm_operation_and_decorator_structure() {
    match Operation::Noop {
        Operation::Noop => (),
        Operation::Assert(_) => (),
        Operation::FmpAdd => (),
        Operation::FmpUpdate => (),
        Operation::SDepth => (),
        Operation::Caller => (),
        Operation::Clk => (),
        Operation::Join => (),
        Operation::Split => (),
        Operation::Loop => (),
        Operation::Call => (),
        Operation::Dyn => (),
        Operation::SysCall => (),
        Operation::Span => (),
        Operation::End => (),
        Operation::Repeat => (),
        Operation::Respan => (),
        Operation::Halt => (),
        Operation::Add => (),
        Operation::Neg => (),
        Operation::Mul => (),
        Operation::Inv => (),
        Operation::Incr => (),
        Operation::And => (),
        Operation::Or => (),
        Operation::Not => (),
        Operation::Eq => (),
        Operation::Eqz => (),
        Operation::Expacc => (),
        Operation::Ext2Mul => (),
        Operation::U32split => (),
        Operation::U32add => (),
        Operation::U32assert2(_) => (),
        Operation::U32add3 => (),
        Operation::U32sub => (),
        Operation::U32mul => (),
        Operation::U32madd => (),
        Operation::U32div => (),
        Operation::U32and => (),
        Operation::U32xor => (),
        Operation::Pad => (),
        Operation::Drop => (),
        Operation::Dup0 => (),
        Operation::Dup1 => (),
        Operation::Dup2 => (),
        Operation::Dup3 => (),
        Operation::Dup4 => (),
        Operation::Dup5 => (),
        Operation::Dup6 => (),
        Operation::Dup7 => (),
        Operation::Dup9 => (),
        Operation::Dup11 => (),
        Operation::Dup13 => (),
        Operation::Dup15 => (),
        Operation::Swap => (),
        Operation::SwapW => (),
        Operation::SwapW2 => (),
        Operation::SwapW3 => (),
        Operation::SwapDW => (),
        Operation::MovUp2 => (),
        Operation::MovUp3 => (),
        Operation::MovUp4 => (),
        Operation::MovUp5 => (),
        Operation::MovUp6 => (),
        Operation::MovUp7 => (),
        Operation::MovUp8 => (),
        Operation::MovDn2 => (),
        Operation::MovDn3 => (),
        Operation::MovDn4 => (),
        Operation::MovDn5 => (),
        Operation::MovDn6 => (),
        Operation::MovDn7 => (),
        Operation::MovDn8 => (),
        Operation::CSwap => (),
        Operation::CSwapW => (),
        Operation::Push(_) => (),
        Operation::AdvPop => (),
        Operation::AdvPopW => (),
        Operation::MLoadW => (),
        Operation::MStoreW => (),
        Operation::MLoad => (),
        Operation::MStore => (),
        Operation::MStream => (),
        Operation::Pipe => (),
        Operation::HPerm => (),
        Operation::MpVerify(_) => (),
        Operation::MrUpdate => (),
        Operation::FriE2F4 => (),
        Operation::RCombBase => (),
    };

    match Decorator::Event(0) {
        Decorator::Advice(advice) => match advice {
            AdviceInjector::MerkleNodeMerge => (),
            AdviceInjector::MerkleNodeToStack => (),
            AdviceInjector::UpdateMerkleNode => (),
            AdviceInjector::MapValueToStack {
                include_len: _,
                key_offset: _,
            } => (),
            AdviceInjector::U64Div => (),
            AdviceInjector::Ext2Inv => (),
            AdviceInjector::Ext2Intt => (),
            AdviceInjector::SmtGet => (),
            AdviceInjector::SmtSet => (),
            AdviceInjector::SmtPeek => (),
            AdviceInjector::U32Clz => (),
            AdviceInjector::U32Ctz => (),
            AdviceInjector::U32Clo => (),
            AdviceInjector::U32Cto => (),
            AdviceInjector::ILog2 => (),
            AdviceInjector::MemToMap => (),
            AdviceInjector::HdwordToMap { domain: _ } => (),
            AdviceInjector::HpermToMap => (),
            AdviceInjector::SigToStack { kind: _ } => (),
        },
        Decorator::AsmOp(_) => (),
        Decorator::Debug(debug_options) => match debug_options {
            DebugOptions::StackAll => (),
            DebugOptions::StackTop(_) => (),
            DebugOptions::MemAll => (),
            DebugOptions::MemInterval(_, _) => (),
            DebugOptions::LocalInterval(_, _, _) => (),
        },
        Decorator::Event(_) => (),
        Decorator::Trace(_) => (),
    };
}

#[test]
fn serialize_deserialize_all_nodes() {
    let mut mast_forest = MastForest::new();

    let basic_block_id = {
        let operations = vec![
            Operation::Noop,
            Operation::Assert(42),
            Operation::FmpAdd,
            Operation::FmpUpdate,
            Operation::SDepth,
            Operation::Caller,
            Operation::Clk,
            Operation::Join,
            Operation::Split,
            Operation::Loop,
            Operation::Call,
            Operation::Dyn,
            Operation::SysCall,
            Operation::Span,
            Operation::End,
            Operation::Repeat,
            Operation::Respan,
            Operation::Halt,
            Operation::Add,
            Operation::Neg,
            Operation::Mul,
            Operation::Inv,
            Operation::Incr,
            Operation::And,
            Operation::Or,
            Operation::Not,
            Operation::Eq,
            Operation::Eqz,
            Operation::Expacc,
            Operation::Ext2Mul,
            Operation::U32split,
            Operation::U32add,
            Operation::U32assert2(222),
            Operation::U32add3,
            Operation::U32sub,
            Operation::U32mul,
            Operation::U32madd,
            Operation::U32div,
            Operation::U32and,
            Operation::U32xor,
            Operation::Pad,
            Operation::Drop,
            Operation::Dup0,
            Operation::Dup1,
            Operation::Dup2,
            Operation::Dup3,
            Operation::Dup4,
            Operation::Dup5,
            Operation::Dup6,
            Operation::Dup7,
            Operation::Dup9,
            Operation::Dup11,
            Operation::Dup13,
            Operation::Dup15,
            Operation::Swap,
            Operation::SwapW,
            Operation::SwapW2,
            Operation::SwapW3,
            Operation::SwapDW,
            Operation::MovUp2,
            Operation::MovUp3,
            Operation::MovUp4,
            Operation::MovUp5,
            Operation::MovUp6,
            Operation::MovUp7,
            Operation::MovUp8,
            Operation::MovDn2,
            Operation::MovDn3,
            Operation::MovDn4,
            Operation::MovDn5,
            Operation::MovDn6,
            Operation::MovDn7,
            Operation::MovDn8,
            Operation::CSwap,
            Operation::CSwapW,
            Operation::Push(Felt::new(45)),
            Operation::AdvPop,
            Operation::AdvPopW,
            Operation::MLoadW,
            Operation::MStoreW,
            Operation::MLoad,
            Operation::MStore,
            Operation::MStream,
            Operation::Pipe,
            Operation::HPerm,
            Operation::MpVerify(1022),
            Operation::MrUpdate,
            Operation::FriE2F4,
            Operation::RCombBase,
        ];

        let num_operations = operations.len();

        let decorators = vec![
            (0, Decorator::Advice(AdviceInjector::MerkleNodeMerge)),
            (0, Decorator::Advice(AdviceInjector::MerkleNodeToStack)),
            (0, Decorator::Advice(AdviceInjector::UpdateMerkleNode)),
            (
                0,
                Decorator::Advice(AdviceInjector::MapValueToStack {
                    include_len: true,
                    key_offset: 1023,
                }),
            ),
            (1, Decorator::Advice(AdviceInjector::U64Div)),
            (3, Decorator::Advice(AdviceInjector::Ext2Inv)),
            (5, Decorator::Advice(AdviceInjector::Ext2Intt)),
            (5, Decorator::Advice(AdviceInjector::SmtGet)),
            (5, Decorator::Advice(AdviceInjector::SmtSet)),
            (5, Decorator::Advice(AdviceInjector::SmtPeek)),
            (5, Decorator::Advice(AdviceInjector::U32Clz)),
            (10, Decorator::Advice(AdviceInjector::U32Ctz)),
            (10, Decorator::Advice(AdviceInjector::U32Clo)),
            (10, Decorator::Advice(AdviceInjector::U32Cto)),
            (10, Decorator::Advice(AdviceInjector::ILog2)),
            (10, Decorator::Advice(AdviceInjector::MemToMap)),
            (
                10,
                Decorator::Advice(AdviceInjector::HdwordToMap {
                    domain: Felt::new(423),
                }),
            ),
            (15, Decorator::Advice(AdviceInjector::HpermToMap)),
            (
                15,
                Decorator::Advice(AdviceInjector::SigToStack {
                    kind: SignatureKind::RpoFalcon512,
                }),
            ),
            (
                15,
                Decorator::AsmOp(AssemblyOp::new(
                    "context".to_string(),
                    15,
                    "op".to_string(),
                    false,
                )),
            ),
            (15, Decorator::Debug(DebugOptions::StackAll)),
            (15, Decorator::Debug(DebugOptions::StackTop(255))),
            (15, Decorator::Debug(DebugOptions::MemAll)),
            (15, Decorator::Debug(DebugOptions::MemInterval(0, 16))),
            (17, Decorator::Debug(DebugOptions::LocalInterval(1, 2, 3))),
            (num_operations, Decorator::Event(45)),
            (num_operations, Decorator::Trace(55)),
        ];

        let basic_block_node = MastNode::new_basic_block_with_decorators(operations, decorators);
        mast_forest.add_node(basic_block_node).unwrap()
    };

    let call_node_id = {
        let node = MastNode::new_call(basic_block_id, &mast_forest);
        mast_forest.add_node(node).unwrap()
    };

    let syscall_node_id = {
        let node = MastNode::new_syscall(basic_block_id, &mast_forest);
        mast_forest.add_node(node).unwrap()
    };

    let loop_node_id = {
        let node = MastNode::new_loop(basic_block_id, &mast_forest);
        mast_forest.add_node(node).unwrap()
    };
    let join_node_id = {
        let node = MastNode::new_join(basic_block_id, call_node_id, &mast_forest);
        mast_forest.add_node(node).unwrap()
    };
    let split_node_id = {
        let node = MastNode::new_split(basic_block_id, call_node_id, &mast_forest);
        mast_forest.add_node(node).unwrap()
    };
    let dyn_node_id = {
        let node = MastNode::new_dynexec();
        mast_forest.add_node(node).unwrap()
    };

    let external_node_id = {
        let node = MastNode::new_external(RpoDigest::default());
        mast_forest.add_node(node).unwrap()
    };

    mast_forest.make_root(join_node_id);
    mast_forest.make_root(syscall_node_id);
    mast_forest.make_root(loop_node_id);
    mast_forest.make_root(split_node_id);
    mast_forest.make_root(dyn_node_id);
    mast_forest.make_root(external_node_id);

    let serialized_mast_forest = mast_forest.to_bytes();
    let deserialized_mast_forest = MastForest::read_from_bytes(&serialized_mast_forest).unwrap();

    assert_eq!(mast_forest, deserialized_mast_forest);
}
