use super::build_test;
use processor::VmState;
use vm_core::{utils::ToElements, Felt, FieldElement, Operation};

// EXEC ITER TESTS
// =================================================================
#[test]
fn test_exec_iter() {
    let source = "proc.foo.1 pop.local.0 end begin popw.mem.1 push.17 exec.foo end";
    let mut init_stack: Vec<u64> = Vec::new();
    (1..=16).for_each(|i| {
        init_stack.push(i);
    });
    let test = build_test!(source, &init_stack.clone());
    let traces = test.execute_iter();
    let fmp = Felt::new(2u64.pow(30));
    let next_fmp = fmp + Felt::ONE;
    let mem = vec![(1_u64, slice_to_word(&[13, 14, 15, 16]))];
    let expected_states = vec![
        VmState {
            clk: 0,
            op: Operation::Span,
            stack: [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_elements(),
            fmp: fmp,
            memory: Vec::new(),
        },
        VmState {
            clk: 1,
            op: Operation::Push(Felt::new(1)),
            stack: [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1].to_elements(),
            fmp: fmp,
            memory: Vec::new(),
        },
        VmState {
            clk: 2,
            op: Operation::StoreW,
            stack: [1, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2].to_elements(),
            fmp: fmp,
            memory: Vec::new(),
        },
        VmState {
            clk: 3,
            op: Operation::Drop,
            stack: [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_elements(),
            fmp: fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 4,
            op: Operation::Drop,
            stack: [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0].to_elements(),
            fmp: fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 5,
            op: Operation::Drop,
            stack: [14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0].to_elements(),
            fmp: fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 6,
            op: Operation::Drop,
            stack: [13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0].to_elements(),
            fmp: fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 7,
            op: Operation::Push(Felt::new(17)),
            stack: [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0].to_elements(),
            fmp: fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 8,
            op: Operation::Push(Felt::new(1)),
            stack: [17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0].to_elements(),
            fmp: fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 9,
            op: Operation::FmpUpdate,
            stack: [1, 17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0].to_elements(),
            fmp: fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 10,
            op: Operation::Pad,
            stack: [17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0].to_elements(),
            fmp: next_fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 11,
            op: Operation::Pad,
            stack: [0, 17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0].to_elements(),
            fmp: next_fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 12,
            op: Operation::Pad,
            stack: [
                0, 0, 17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0,
            ]
            .to_elements(),
            fmp: next_fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 13,
            op: Operation::Pad,
            stack: [
                0, 0, 0, 17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 0, 0, 0, 0,
            ]
            .to_elements(),
            fmp: next_fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 14,
            op: Operation::FmpAdd,
            stack: [
                0, 0, 0, 0, 17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0,
            ]
            .to_elements(),
            fmp: next_fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 15,
            op: Operation::StoreW,
            stack: [
                2u64.pow(30) + 1,
                0,
                0,
                0,
                17,
                12,
                11,
                10,
                9,
                8,
                7,
                6,
                5,
                4,
                3,
                2,
                0,
                0,
                0,
                0,
            ]
            .to_elements(),
            fmp: next_fmp,
            memory: mem.clone(),
        },
        VmState {
            clk: 16,
            op: Operation::Drop,
            stack: [0, 0, 0, 17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0].to_elements(),
            fmp: next_fmp,
            memory: vec![
                (1_u64, slice_to_word(&[13, 14, 15, 16])),
                (2u64.pow(30) + 1, slice_to_word(&[17, 0, 0, 0])),
            ],
        },
        VmState {
            clk: 17,
            op: Operation::Drop,
            stack: [0, 0, 17, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0].to_elements(),
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
