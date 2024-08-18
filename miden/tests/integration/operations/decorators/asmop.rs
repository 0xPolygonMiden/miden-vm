use processor::{AsmOpInfo, RowIndex, VmStateIterator};
use test_utils::{assert_eq, build_debug_test};
use vm_core::{debuginfo::Location, AssemblyOp, Felt, Operation};

#[test]
fn asmop_one_span_block_test() {
    let source = "begin push.1 push.2 add end";
    let test = build_debug_test!(source);
    let path = test.source.name();
    let push1_loc = Some(Location {
        path: path.clone(),
        start: 6.into(),
        end: (6 + 6).into(),
    });
    let push2_loc = Some(Location {
        path: path.clone(),
        start: 13.into(),
        end: (13 + 6).into(),
    });
    let add_loc = Some(Location {
        path: path.clone(),
        start: 20.into(),
        end: (20 + 3).into(),
    });
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: RowIndex::from(0),
            asmop: None,
            op: None,
        },
        VmStatePartial {
            clk: RowIndex::from(1),
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: RowIndex::from(2),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc.clone(),
                    "#exec::#main".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: RowIndex::from(3),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc,
                    "#exec::#main".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                2,
            )),
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: RowIndex::from(4),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push2_loc,
                    "#exec::#main".to_string(),
                    1,
                    "push.2".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: RowIndex::from(5),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(add_loc, "#exec::#main".to_string(), 1, "add".to_string(), false),
                1,
            )),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: RowIndex::from(6),
            asmop: None,
            op: Some(Operation::End),
        },
    ];
    let vm_state = build_vm_state(vm_state_iterator);
    assert_eq!(expected_vm_state, vm_state);
}

#[test]
fn asmop_with_one_procedure() {
    let source = "proc.foo push.1 push.2 add end begin exec.foo end";
    let test = build_debug_test!(source);
    let path = test.source.name();
    let push1_loc = Some(Location {
        path: path.clone(),
        start: 9.into(),
        end: (9 + 6).into(),
    });
    let push2_loc = Some(Location {
        path: path.clone(),
        start: 16.into(),
        end: (16 + 6).into(),
    });
    let add_loc = Some(Location {
        path: path.clone(),
        start: 23.into(),
        end: (23 + 3).into(),
    });
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: RowIndex::from(0),
            asmop: None,
            op: None,
        },
        VmStatePartial {
            clk: RowIndex::from(1),
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: RowIndex::from(2),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc.clone(),
                    "#exec::foo".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: RowIndex::from(3),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc,
                    "#exec::foo".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                2,
            )),
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: RowIndex::from(4),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push2_loc,
                    "#exec::foo".to_string(),
                    1,
                    "push.2".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: RowIndex::from(5),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(add_loc, "#exec::foo".to_string(), 1, "add".to_string(), false),
                1,
            )),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: RowIndex::from(6),
            asmop: None,
            op: Some(Operation::End),
        },
    ];
    let vm_state = build_vm_state(vm_state_iterator);
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
    let path = test.source.name();
    let push1_loc = Some(Location {
        path: path.clone(),
        start: 43.into(),
        end: (43 + 6).into(),
    });
    let push2_loc = Some(Location {
        path: path.clone(),
        start: 50.into(),
        end: (50 + 6).into(),
    });
    let add_loc = Some(Location {
        path: path.clone(),
        start: 57.into(),
        end: (57 + 3).into(),
    });
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: RowIndex::from(0),
            asmop: None,
            op: None,
        },
        VmStatePartial {
            clk: RowIndex::from(1),
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: RowIndex::from(2),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc.clone(),
                    "#exec::#main".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: RowIndex::from(3),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc.clone(),
                    "#exec::#main".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                2,
            )),
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: RowIndex::from(4),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push2_loc.clone(),
                    "#exec::#main".to_string(),
                    1,
                    "push.2".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: RowIndex::from(5),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    add_loc.clone(),
                    "#exec::#main".to_string(),
                    1,
                    "add".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Add),
        },
        // End first Span
        VmStatePartial {
            clk: RowIndex::from(6),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc.clone(),
                    "#exec::#main".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: RowIndex::from(7),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc.clone(),
                    "#exec::#main".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                2,
            )),
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: RowIndex::from(8),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push2_loc.clone(),
                    "#exec::#main".to_string(),
                    1,
                    "push.2".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: RowIndex::from(9),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    add_loc.clone(),
                    "#exec::#main".to_string(),
                    1,
                    "add".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Add),
        },
        // End second Span
        VmStatePartial {
            clk: RowIndex::from(10),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc.clone(),
                    "#exec::#main".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: RowIndex::from(11),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc,
                    "#exec::#main".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                2,
            )),
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: RowIndex::from(12),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push2_loc,
                    "#exec::#main".to_string(),
                    1,
                    "push.2".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: RowIndex::from(13),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(add_loc, "#exec::#main".to_string(), 1, "add".to_string(), false),
                1,
            )),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: RowIndex::from(14),
            asmop: None,
            op: Some(Operation::Noop),
        },
        VmStatePartial {
            clk: RowIndex::from(15),
            asmop: None,
            op: Some(Operation::Noop),
        },
        VmStatePartial {
            clk: RowIndex::from(16),
            asmop: None,
            op: Some(Operation::Noop),
        },
        VmStatePartial {
            clk: RowIndex::from(17),
            asmop: None,
            op: Some(Operation::End),
        },
    ];
    let vm_state = build_vm_state(vm_state_iterator);
    assert_eq!(expected_vm_state, vm_state);
}

#[test]
fn asmop_conditional_execution_test() {
    let source = "
        begin
            eq
            if.true
                push.1 push.2 add
            else
                push.3 push.4 add
            end
        end";

    //if branch
    let test = build_debug_test!(source, &[1, 1]);
    let path = test.source.name();
    let eq_loc = Some(Location {
        path: path.clone(),
        start: 18.into(),
        end: (18 + 2).into(),
    });
    let push1_loc = Some(Location {
        path: path.clone(),
        start: 57.into(),
        end: (57 + 6).into(),
    });
    let push2_loc = Some(Location {
        path: path.clone(),
        start: 64.into(),
        end: (64 + 6).into(),
    });
    let add_loc = Some(Location {
        path: path.clone(),
        start: 71.into(),
        end: (71 + 3).into(),
    });
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: RowIndex::from(0),
            asmop: None,
            op: None,
        },
        VmStatePartial {
            clk: RowIndex::from(1),
            asmop: None,
            op: Some(Operation::Join),
        },
        VmStatePartial {
            clk: RowIndex::from(2),
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: RowIndex::from(3),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(eq_loc, "#exec::#main".to_string(), 1, "eq".to_string(), false),
                1,
            )),
            op: Some(Operation::Eq),
        },
        VmStatePartial {
            clk: RowIndex::from(4),
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: RowIndex::from(5),
            asmop: None,
            op: Some(Operation::Split),
        },
        VmStatePartial {
            clk: RowIndex::from(6),
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: RowIndex::from(7),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc.clone(),
                    "#exec::#main".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Pad),
        },
        VmStatePartial {
            clk: RowIndex::from(8),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push1_loc,
                    "#exec::#main".to_string(),
                    2,
                    "push.1".to_string(),
                    false,
                ),
                2,
            )),
            op: Some(Operation::Incr),
        },
        VmStatePartial {
            clk: RowIndex::from(9),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push2_loc,
                    "#exec::#main".to_string(),
                    1,
                    "push.2".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Push(Felt::new(2))),
        },
        VmStatePartial {
            clk: RowIndex::from(10),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(add_loc, "#exec::#main".to_string(), 1, "add".to_string(), false),
                1,
            )),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: RowIndex::from(11),
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: RowIndex::from(12),
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: RowIndex::from(13),
            asmop: None,
            op: Some(Operation::End),
        },
    ];
    let vm_state = build_vm_state(vm_state_iterator);
    assert_eq!(expected_vm_state, vm_state);

    //else branch
    let test = build_debug_test!(source, &[1, 0]);
    let path = test.source.name();
    let eq_loc = Some(Location {
        path: path.clone(),
        start: 18.into(),
        end: (18 + 2).into(),
    });
    let push3_loc = Some(Location {
        path: path.clone(),
        start: 108.into(),
        end: (108 + 6).into(),
    });
    let push4_loc = Some(Location {
        path: path.clone(),
        start: 115.into(),
        end: (115 + 6).into(),
    });
    let add_loc = Some(Location {
        path: path.clone(),
        start: 122.into(),
        end: (122 + 3).into(),
    });
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: RowIndex::from(0),
            asmop: None,
            op: None,
        },
        VmStatePartial {
            clk: RowIndex::from(1),
            asmop: None,
            op: Some(Operation::Join),
        },
        VmStatePartial {
            clk: RowIndex::from(2),
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: RowIndex::from(3),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(eq_loc, "#exec::#main".to_string(), 1, "eq".to_string(), false),
                1,
            )),
            op: Some(Operation::Eq),
        },
        VmStatePartial {
            clk: RowIndex::from(4),
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: RowIndex::from(5),
            asmop: None,
            op: Some(Operation::Split),
        },
        VmStatePartial {
            clk: RowIndex::from(6),
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: RowIndex::from(7),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push3_loc,
                    "#exec::#main".to_string(),
                    1,
                    "push.3".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Push(Felt::new(3))),
        },
        VmStatePartial {
            clk: RowIndex::from(8),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(
                    push4_loc,
                    "#exec::#main".to_string(),
                    1,
                    "push.4".to_string(),
                    false,
                ),
                1,
            )),
            op: Some(Operation::Push(Felt::new(4))),
        },
        VmStatePartial {
            clk: RowIndex::from(9),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(add_loc, "#exec::#main".to_string(), 1, "add".to_string(), false),
                1,
            )),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: RowIndex::from(10),
            asmop: None,
            op: Some(Operation::Noop),
        },
        VmStatePartial {
            clk: RowIndex::from(11),
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: RowIndex::from(12),
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: RowIndex::from(13),
            asmop: None,
            op: Some(Operation::End),
        },
    ];
    let vm_state = build_vm_state(vm_state_iterator);
    assert_eq!(expected_vm_state, vm_state);
}

/// This is a helper function to build a vector of [VmStatePartial] from a specified
/// [VmStateIterator].
fn build_vm_state(vm_state_iterator: VmStateIterator) -> Vec<VmStatePartial> {
    let mut vm_state = Vec::new();
    for state in vm_state_iterator {
        vm_state.push(VmStatePartial {
            clk: state.as_ref().unwrap().clk,
            asmop: state.as_ref().unwrap().asmop.clone(),
            op: state.as_ref().unwrap().op,
        });
    }
    vm_state
}

/// [VmStatePartial] holds the following current process state information at a specific clock cycle
/// * clk: Current clock cycle
/// * asmop: AsmOp decorator at the specific clock cycle
/// * op: Operation executed at the specific clock cycle
#[derive(Clone, Debug, Eq, PartialEq)]
struct VmStatePartial {
    clk: RowIndex,
    asmop: Option<AsmOpInfo>,
    op: Option<Operation>,
}
