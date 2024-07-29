use processor::{AsmOpInfo, VmStateIterator};
use test_utils::{assert_eq, build_debug_test};
use vm_core::{AssemblyOp, Felt, Operation};

#[test]
fn asmop_one_span_block_test() {
    let source = "begin push.1 push.2 add end";
    let test = build_debug_test!(source);
    let source_file = vm_core::SourceFile::new(test.source.name());
    let push1_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 6,
        end: 6 + 6,
    });
    let push2_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 13,
        end: 13 + 6,
    });
    let add_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 20,
        end: 20 + 3,
    });
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: 0,
            asmop: None,
            op: None,
        },
        VmStatePartial {
            clk: 1,
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 2,
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
            clk: 3,
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
            clk: 4,
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
            clk: 5,
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(add_loc, "#exec::#main".to_string(), 1, "add".to_string(), false),
                1,
            )),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 6,
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
    let source_file = vm_core::SourceFile::new(test.source.name());
    let push1_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 9,
        end: 9 + 6,
    });
    let push2_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 16,
        end: 16 + 6,
    });
    let add_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 23,
        end: 23 + 3,
    });
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: 0,
            asmop: None,
            op: None,
        },
        VmStatePartial {
            clk: 1,
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 2,
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
            clk: 3,
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
            clk: 4,
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
            clk: 5,
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(add_loc, "#exec::foo".to_string(), 1, "add".to_string(), false),
                1,
            )),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 6,
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
    let source_file = vm_core::SourceFile::new(test.source.name());
    let push1_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 43,
        end: 43 + 6,
    });
    let push2_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 50,
        end: 50 + 6,
    });
    let add_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 57,
        end: 57 + 3,
    });
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: 0,
            asmop: None,
            op: None,
        },
        VmStatePartial {
            clk: 1,
            asmop: None,
            op: Some(Operation::Join),
        },
        VmStatePartial {
            clk: 2,
            asmop: None,
            op: Some(Operation::Join),
        },
        VmStatePartial {
            clk: 3,
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 4,
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
            clk: 5,
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
            clk: 6,
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
            clk: 7,
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
            clk: 8,
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 9,
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 10,
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
            clk: 11,
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
            clk: 12,
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
            clk: 13,
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
            clk: 14,
            asmop: None,
            op: Some(Operation::End),
        },
        // End first Join
        VmStatePartial {
            clk: 15,
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 16,
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 17,
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
            clk: 18,
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
            clk: 19,
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
            clk: 20,
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(add_loc, "#exec::#main".to_string(), 1, "add".to_string(), false),
                1,
            )),
            op: Some(Operation::Add),
        },
        // End Span
        VmStatePartial {
            clk: 21,
            asmop: None,
            op: Some(Operation::End),
        },
        // End second Join
        VmStatePartial {
            clk: 22,
            asmop: None,
            op: Some(Operation::End),
        },
    ];
    let vm_state = build_vm_state(vm_state_iterator);
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
    let source_file = vm_core::SourceFile::new(test.source.name());
    let eq_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 18,
        end: 18 + 2,
    });
    let push1_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 57,
        end: 57 + 6,
    });
    let push2_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 64,
        end: 64 + 6,
    });
    let add_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 71,
        end: 71 + 3,
    });
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: 0,
            asmop: None,
            op: None,
        },
        VmStatePartial {
            clk: 1,
            asmop: None,
            op: Some(Operation::Join),
        },
        VmStatePartial {
            clk: 2,
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 3,
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(eq_loc, "#exec::#main".to_string(), 1, "eq".to_string(), false),
                1,
            )),
            op: Some(Operation::Eq),
        },
        VmStatePartial {
            clk: 4,
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 5,
            asmop: None,
            op: Some(Operation::Split),
        },
        VmStatePartial {
            clk: 6,
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 7,
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
            clk: 8,
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
            clk: 9,
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
            clk: 10,
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(add_loc, "#exec::#main".to_string(), 1, "add".to_string(), false),
                1,
            )),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 11,
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 12,
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 13,
            asmop: None,
            op: Some(Operation::End),
        },
    ];
    let vm_state = build_vm_state(vm_state_iterator);
    assert_eq!(expected_vm_state, vm_state);

    //else branch
    let test = build_debug_test!(source, &[1, 0]);
    let source_file = vm_core::SourceFile::new(test.source.name());
    let eq_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 18,
        end: 18 + 2,
    });
    let push3_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 108,
        end: 108 + 6,
    });
    let push4_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 115,
        end: 115 + 6,
    });
    let add_loc = Some(vm_core::SourceLocation {
        source_file: source_file.clone(),
        start: 122,
        end: 122 + 3,
    });
    let vm_state_iterator = test.execute_iter();
    let expected_vm_state = vec![
        VmStatePartial {
            clk: 0,
            asmop: None,
            op: None,
        },
        VmStatePartial {
            clk: 1,
            asmop: None,
            op: Some(Operation::Join),
        },
        VmStatePartial {
            clk: 2,
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 3,
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(eq_loc, "#exec::#main".to_string(), 1, "eq".to_string(), false),
                1,
            )),
            op: Some(Operation::Eq),
        },
        VmStatePartial {
            clk: 4,
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 5,
            asmop: None,
            op: Some(Operation::Split),
        },
        VmStatePartial {
            clk: 6,
            asmop: None,
            op: Some(Operation::Span),
        },
        VmStatePartial {
            clk: 7,
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
            clk: 8,
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
            clk: 9,
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new(add_loc, "#exec::#main".to_string(), 1, "add".to_string(), false),
                1,
            )),
            op: Some(Operation::Add),
        },
        VmStatePartial {
            clk: 10,
            asmop: None,
            op: Some(Operation::Noop),
        },
        VmStatePartial {
            clk: 11,
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 12,
            asmop: None,
            op: Some(Operation::End),
        },
        VmStatePartial {
            clk: 13,
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
    clk: u32,
    asmop: Option<AsmOpInfo>,
    op: Option<Operation>,
}
