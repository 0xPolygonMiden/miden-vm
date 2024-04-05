use crate::{
    trace::virtual_bus::{prover::VirtualBusProver, verifier::VirtualBusVerifier},
    DefaultHost, ExecutionTrace, Process,
};
use alloc::vec::Vec;
use miden_air::{trace::main_trace::MainTrace, ExecutionOptions};
use vm_core::crypto::random::RpoRandomCoin;
use vm_core::{
    code_blocks::CodeBlock, CodeBlockTable, Felt, FieldElement, Kernel, Operation, StackInputs,
};

#[test]
fn test_vb_prover_verifier() {
    let s = 6;
    let o = 6;
    let stack: Vec<_> = (0..(1 << s)).into_iter().collect();
    let operations: Vec<_> = (0..(1 << o))
        .flat_map(|_| {
            vec![Operation::U32split, Operation::U32add, Operation::U32xor, Operation::MStoreW]
        })
        .collect();

    let trace = build_full_trace(&stack, operations, Kernel::default());

    // this should be generated using the transcript up to when the prover sends the commitment
    // to the main trace.
    let alphas: Vec<Felt> = vec![test_utils::rand::rand_value()];

    let seed = [Felt::ZERO; 4]; // should be initialized with the appropriate transcript
    let mut transcript = RpoRandomCoin::new(seed.into());
    let vb_prover = VirtualBusProver::new(alphas.clone()).unwrap();
    let proof = vb_prover.prove(&trace, &mut transcript);

    let seed = [Felt::ZERO; 4]; // should be initialized with the appropriate transcript
    let mut transcript = RpoRandomCoin::new(seed.into());
    let vb_verifier = VirtualBusVerifier::new(alphas).unwrap();
    let _final_opening_claim = vb_verifier.verify(proof, &mut transcript);
}

fn build_full_trace(stack_inputs: &[u64], operations: Vec<Operation>, kernel: Kernel) -> MainTrace {
    let stack_inputs: Vec<Felt> = stack_inputs.iter().map(|a| Felt::new(*a)).collect();
    let stack_inputs = StackInputs::new(stack_inputs).unwrap();
    let host = DefaultHost::default();
    let mut process = Process::new(kernel, stack_inputs, host, ExecutionOptions::default());
    let program = CodeBlock::new_span(operations);
    process.execute_code_block(&program, &CodeBlockTable::default()).unwrap();
    let (trace, _, _): (MainTrace, _, _) = ExecutionTrace::test_finalize_trace(process);

    trace
}
