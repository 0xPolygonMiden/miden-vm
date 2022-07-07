use crate::build_debug_test;
use vm_core::{AsmOpInfo, Decorator, Decorator::AsmOp, Felt, Operation};

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
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.1".to_string(), 2))]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 3,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 4,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.2".to_string(), 1))]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 5,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("add".to_string(), 1))]),
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
fn asmop_repeat_test() {
    let source = "begin
            repeat.3
                push.1 push.2 add
            end
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
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.1".to_string(), 2))]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 3,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 4,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.2".to_string(), 1))]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 5,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("add".to_string(), 1))]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 6,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.1".to_string(), 2))]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 7,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 8,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.2".to_string(), 1))]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 9,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("add".to_string(), 1))]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 10,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.1".to_string(), 2))]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 11,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 12,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.2".to_string(), 1))]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 13,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("add".to_string(), 1))]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 14,
            decorators: None,
            op: Some(Operation::Noop),
        },
        VmStatePartial {
            clk: 15,
            decorators: None,
            op: Some(Operation::Noop),
        },
        VmStatePartial {
            clk: 16,
            decorators: None,
            op: Some(Operation::Noop),
        },
        VmStatePartial {
            clk: 17,
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
fn asmop_conditional_execution_test() {
    let source = "begin
            eq
            if.true
                push.1 push.2 add
            else
                push.3 push.4 add
            end            
        end";

    //if branch
    let test = build_debug_test!(source, &[1, 1]);
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
            op: Some(Operation::Join),
        },
        VmStatePartial {
            clk: 2,
            decorators: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 3,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("eq".to_string(), 1))]),
            op: Some(Operation::Eq),
        },
        VmStatePartial {
            clk: 4,
            decorators: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 5,
            decorators: None,
            op: Some(Operation::Split),
        },
        VmStatePartial {
            clk: 6,
            decorators: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 7,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.1".to_string(), 2))]),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: 8,
            decorators: None,
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: 9,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.2".to_string(), 1))]),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: 10,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("add".to_string(), 1))]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 11,
            decorators: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 12,
            decorators: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 13,
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

    //else branch
    let test = build_debug_test!(source, &[1, 0]);
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
            op: Some(Operation::Join),
        },
        VmStatePartial {
            clk: 2,
            decorators: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 3,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("eq".to_string(), 1))]),
            op: Some(Operation::Eq),
        },
        VmStatePartial {
            clk: 4,
            decorators: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 5,
            decorators: None,
            op: Some(Operation::Split),
        },
        VmStatePartial {
            clk: 6,
            decorators: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 7,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.3".to_string(), 1))]),
            op: Some(Operation::Push(Felt::new(3))),
        },
        VmStatePartial {
            clk: 8,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("push.4".to_string(), 1))]),
            op: Some(Operation::Push(Felt::new(4))),
        },
        VmStatePartial {
            clk: 9,
            decorators: Some(vec![AsmOp(AsmOpInfo::new("add".to_string(), 1))]),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 10,
            decorators: None,
            op: Some(Operation::Noop),
        },
        VmStatePartial {
            clk: 11,
            decorators: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 12,
            decorators: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 13,
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct VmStatePartial {
    clk: usize,
    decorators: Option<Vec<Decorator>>,
    op: Option<Operation>,
}
