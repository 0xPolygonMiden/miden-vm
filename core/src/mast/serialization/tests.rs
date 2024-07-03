use math::FieldElement;

use super::*;
use crate::operations::Operation;

#[test]
fn confirm_assumptions() {
    // TODOP: match against all `Operation` and `Decorator`
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
            Operation::U32assert2(Felt::ONE),
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
        mast_forest.add_node(basic_block_node)
    };

    // TODOP: REMOVE
    mast_forest.make_root(basic_block_id);

    let serialized_mast_forest = mast_forest.to_bytes();
    let deserialized_mast_forest = MastForest::read_from_bytes(&serialized_mast_forest).unwrap();

    assert_eq!(mast_forest, deserialized_mast_forest);
}
