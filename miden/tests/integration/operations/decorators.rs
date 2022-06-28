use crate::build_debug_test;
use vm_core::Felt;
use vm_core::{Decorator, Decorator::AsmOp, Operation};

// DECORATOR TESTS
// ================================================================================================

#[test]
fn asmop_one_span_block_test() {
    let source = "begin push.1 push.2 add end";
    let test = build_debug_test!(source);
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: 0,
            decorators: None,
            op: None,
        },
        VmStatePartial {
            clk: 1,
            decorators: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 2,
            decorators: Some(vec![AsmOp("push.1".to_string())]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 3,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 4,
            decorators: Some(vec![AsmOp("push.2".to_string())]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 5,
            decorators: Some(vec![AsmOp("add".to_string())]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 6,
            decorators: None,
            op: Some(Operation::End),
        },
    ];
    let mut vm_state = Vec::new();
    for state in vm_state_iterator {
        vm_state.push(VmStatePartial {
            clk: state.as_ref().unwrap().clk,
            decorators: state.as_ref().unwrap().decorators.clone(),
            op: state.as_ref().unwrap().op,
        });
    }
    assert_eq!(expected_vm_state, vm_state);
}

#[test]
fn asmop_respan_test() {
    let source = "begin 
            push.1 push.2 add
            push.1 push.2 add
            push.1 push.2 add
            push.1 push.2 add
            push.1 push.2 add
            push.1 push.2 add
        end";
    let test = build_debug_test!(source);
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: 0,
            decorators: None,
            op: None,
        },
        VmStatePartial {
            clk: 1,
            decorators: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 2,
            decorators: Some(vec![AsmOp("push.1".to_string())]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 3,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 4,
            decorators: Some(vec![AsmOp("push.2".to_string())]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 5,
            decorators: Some(vec![AsmOp("add".to_string())]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 6,
            decorators: Some(vec![AsmOp("push.1".to_string())]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 7,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 8,
            decorators: Some(vec![AsmOp("push.2".to_string())]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 9,
            decorators: Some(vec![AsmOp("add".to_string())]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 10,
            decorators: Some(vec![AsmOp("push.1".to_string())]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 11,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 12,
            decorators: Some(vec![AsmOp("push.2".to_string())]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 13,
            decorators: Some(vec![AsmOp("add".to_string())]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 14,
            decorators: Some(vec![AsmOp("push.1".to_string())]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 15,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 16,
            decorators: Some(vec![AsmOp("push.2".to_string())]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 17,
            decorators: Some(vec![AsmOp("add".to_string())]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 18,
            decorators: Some(vec![AsmOp("push.1".to_string())]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 19,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 20,
            decorators: Some(vec![AsmOp("push.2".to_string())]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 21,
            decorators: Some(vec![AsmOp("add".to_string())]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 22,
            decorators: Some(vec![AsmOp("push.1".to_string())]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 23,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 24,
            decorators: None,
            op: Some(Operation::Respan),
        },
        VmStatePartial {
            clk: 25,
            decorators: Some(vec![AsmOp("push.2".to_string())]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 26,
            decorators: Some(vec![AsmOp("add".to_string())]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 27,
            decorators: None,
            op: Some(Operation::End),
        },
    ];
    let mut vm_state = Vec::new();
    for state in vm_state_iterator {
        vm_state.push(VmStatePartial {
            clk: state.as_ref().unwrap().clk,
            decorators: state.as_ref().unwrap().decorators.clone(),
            op: state.as_ref().unwrap().op,
        });
    }
    assert_eq!(expected_vm_state, vm_state);
}

// #[test]
// fn asmop_one_procedure_test() {
// }

// #[test]
// fn asmop_multiple_local_procedures_test() {
// }

// #[test]
// fn asmop_one_imported_procedure_test() {
// }

// #[test]
// fn asmop_multiple_imported_procedure_test() {
// }

#[derive(Clone, Debug, Eq, PartialEq)]
struct VmStatePartial {
    clk: usize,
    decorators: Option<Vec<Decorator>>,
    op: Option<Operation>,
}
