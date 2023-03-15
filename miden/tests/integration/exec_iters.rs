use super::build_debug_test;
use processor::{AsmOpInfo, VmState};
use vm_core::{utils::ToElements, AssemblyOp, Felt, FieldElement, Operation};

// EXEC ITER TESTS
// =================================================================
#[test]
fn test_exec_iter() {
    let source = "proc.foo.1 loc_store.0 end begin mem_storew.1 dropw push.17 exec.foo end";
    let mut init_stack: Vec<u64> = Vec::new();
    (1..=16).for_each(|i| {
        init_stack.push(i);
    });
    let test = build_debug_test!(source, &init_stack);
    let traces = test.execute_iter();
    let fmp = Felt::new(2u64.pow(30));
    let next_fmp = fmp + Felt::ONE;
    let mem = vec![(1_u64, slice_to_word(&[13, 14, 15, 16]))];
    let expected_states = vec![
        VmState {
            clk: 0,
            ctx: 0,
            op: None,
            asmop: None,
            stack: [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_elements(),
            fmp,
            memory: Vec::new(),
        },
        VmState {
            clk: 1,
            ctx: 0,
            op: Some(Operation::Span),
            asmop: None,
            stack: [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1].to_elements(),
            fmp,
            memory: Vec::new(),
        },
        VmState {
            clk: 2,
            ctx: 0,
            op: Some(Operation::Pad),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("#main".to_string(), 3, "mem_storew.1".to_string(), false),
                1,
            )),
            stack: [0, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_elements(),
            fmp,
            memory: Vec::new(),
        },
        VmState {
            clk: 3,
            ctx: 0,
            op: Some(Operation::Incr),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("#main".to_string(), 3, "mem_storew.1".to_string(), false),
                2,
            )),
            stack: [1, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2].to_elements(),
            fmp,
            memory: Vec::new(),
        },
        VmState {
            clk: 4,
            ctx: 0,
            op: Some(Operation::MStoreW),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("#main".to_string(), 3, "mem_storew.1".to_string(), false),
                3,
            )),
            stack: [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_elements(),
            fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 5,
            ctx: 0,
            op: Some(Operation::Drop),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("#main".to_string(), 4, "dropw".to_string(), false),
                1,
            )),
            stack: [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0].to_elements(),
            fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 6,
            ctx: 0,
            op: Some(Operation::Drop),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("#main".to_string(), 4, "dropw".to_string(), false),
                2,
            )),
            stack: [14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0].to_elements(),
            fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 7,
            ctx: 0,
            op: Some(Operation::Drop),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("#main".to_string(), 4, "dropw".to_string(), false),
                3,
            )),
            stack: [13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0].to_elements(),
            fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 8,
            ctx: 0,
            op: Some(Operation::Drop),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("#main".to_string(), 4, "dropw".to_string(), false),
                4,
            )),
            stack: [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0].to_elements(),
            fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 9,
            ctx: 0,
            op: Some(Operation::Push(Felt::new(17))),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("#main".to_string(), 1, "push.17".to_string(), false),
                1,
            )),
            stack: [17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0].to_elements(),
            fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 10,
            ctx: 0,
            op: Some(Operation::Noop),
            asmop: None,
            stack: [17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0].to_elements(),
            fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 11,
            ctx: 0,
            op: Some(Operation::Push(Felt::new(1))),
            asmop: None,
            stack: [1, 17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0].to_elements(),
            fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 12,
            ctx: 0,
            op: Some(Operation::FmpUpdate),
            asmop: None,
            stack: [17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0].to_elements(),
            fmp: next_fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 13,
            ctx: 0,
            op: Some(Operation::Pad),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("foo".to_string(), 4, "loc_store.0".to_string(), false),
                1,
            )),
            stack: [0, 17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0].to_elements(),
            fmp: next_fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 14,
            ctx: 0,
            op: Some(Operation::FmpAdd),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("foo".to_string(), 4, "loc_store.0".to_string(), false),
                2,
            )),
            stack: [2u64.pow(30) + 1, 17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0]
                .to_elements(),
            fmp: next_fmp,
            memory: mem,
        },
        VmState {
            clk: 15,
            ctx: 0,
            op: Some(Operation::MStore),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("foo".to_string(), 4, "loc_store.0".to_string(), false),
                3,
            )),
            stack: [17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0].to_elements(),
            fmp: next_fmp,
            memory: vec![
                (1_u64, slice_to_word(&[13, 14, 15, 16])),
                (2u64.pow(30) + 1, slice_to_word(&[17, 0, 0, 0])),
            ],
        },
        VmState {
            clk: 16,
            ctx: 0,
            op: Some(Operation::Drop),
            asmop: Some(AsmOpInfo::new(
                AssemblyOp::new("foo".to_string(), 4, "loc_store.0".to_string(), false),
                4,
            )),
            stack: [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0].to_elements(),
            fmp: next_fmp,
            memory: vec![
                (1_u64, slice_to_word(&[13, 14, 15, 16])),
                (2u64.pow(30) + 1, slice_to_word(&[17, 0, 0, 0])),
            ],
        },
    ];
    for (expected, t) in expected_states.iter().zip(traces) {
        let state = t.as_ref().unwrap();
        assert_eq!(*expected, *state);
    }
}

// HELPER FUNCTIONS
// =================================================================
fn slice_to_word(values: &[i32]) -> [Felt; 4] {
    [
        Felt::new(values[0] as u64),
        Felt::new(values[1] as u64),
        Felt::new(values[2] as u64),
        Felt::new(values[3] as u64),
    ]
}
