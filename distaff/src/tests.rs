use crate::{
    assembly, BaseElement, ExecutionTrace, FieldElement, ProgramInputs, Serializable,
    TraceMetadata, TraceState,
};
use air::ToElements;

#[test]
fn execute_span() {
    let program = assembly::compile("begin add push.5 mul push.7 end").unwrap();
    let inputs = ProgramInputs::from_public(&[BaseElement::new(1), BaseElement::new(2)]);

    let trace = processor::execute(&program, &inputs);
    let trace_length = trace.length();
    let trace_width = trace.width();

    assert_eq!(64, trace_length);
    assert_eq!(17, trace_width);
    let state = get_trace_state(&trace, trace_length - 1);

    assert_eq!(BaseElement::new(46), state.op_counter());
    assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
    assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
    assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
    assert_eq!([1, 1].to_elements(), state.hd_op_bits());
    assert_eq!([0].to_elements(), state.ctx_stack());
    assert_eq!([7, 15, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
}

#[test]
fn execute_block() {
    let program = assembly::compile("begin add block push.5 mul push.7 end end").unwrap();
    let inputs = ProgramInputs::from_public(&[BaseElement::new(1), BaseElement::new(2)]);

    let trace = processor::execute(&program, &inputs);
    let trace_length = trace.length();
    let trace_width = trace.width();

    assert_eq!(64, trace_length);
    assert_eq!(18, trace_width);
    let state = get_trace_state(&trace, trace_length - 1);

    assert_eq!(BaseElement::new(60), state.op_counter());
    assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
    assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
    assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
    assert_eq!([1, 1].to_elements(), state.hd_op_bits());
    assert_eq!([0].to_elements(), state.ctx_stack());
    assert_eq!([0].to_elements(), state.loop_stack());
    assert_eq!([7, 15, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
}

#[test]
fn execute_if_else() {
    let program =
        assembly::compile("begin read if.true add push.3 else push.7 add push.8 end mul end")
            .unwrap();

    // execute true branch
    let inputs = ProgramInputs::new(
        &[BaseElement::new(5), BaseElement::new(3)],
        &[BaseElement::new(1)],
        &[],
    );
    let trace = processor::execute(&program, &inputs);
    let trace_length = trace.length();
    let trace_width = trace.width();

    assert_eq!(128, trace_length);
    assert_eq!(19, trace_width);
    let state = get_trace_state(&trace, trace_length - 1);

    assert_eq!(BaseElement::new(76), state.op_counter());
    assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
    assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
    assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
    assert_eq!([1, 1].to_elements(), state.hd_op_bits());
    assert_eq!([0].to_elements(), state.ctx_stack());
    assert_eq!([0].to_elements(), state.loop_stack());
    assert_eq!([24, 0, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());

    // execute false branch
    let inputs = ProgramInputs::new(
        &[BaseElement::new(5), BaseElement::new(3)],
        &[BaseElement::new(0)],
        &[],
    );
    let trace = processor::execute(&program, &inputs);
    let trace_length = trace.length();
    let trace_width = trace.width();

    assert_eq!(128, trace_length);
    assert_eq!(19, trace_width);
    let state = get_trace_state(&trace, trace_length - 1);

    assert_eq!(BaseElement::new(92), state.op_counter());
    assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
    assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
    assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
    assert_eq!([1, 1].to_elements(), state.hd_op_bits());
    assert_eq!([0].to_elements(), state.ctx_stack());
    assert_eq!([0].to_elements(), state.loop_stack());
    assert_eq!([96, 3, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
}

#[test]
fn execute_loop() {
    let program = assembly::compile("begin mul read while.true dup mul read end end").unwrap();

    // don't enter the loop
    let inputs = ProgramInputs::new(
        &[BaseElement::new(5), BaseElement::new(3)],
        &[BaseElement::new(0)],
        &[],
    );
    let trace = processor::execute(&program, &inputs);

    assert_eq!(64, trace.length());
    assert_eq!(18, trace.width());
    let state = get_trace_state(&trace, trace.length() - 1);

    assert_eq!(BaseElement::new(60), state.op_counter());
    assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
    assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
    assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
    assert_eq!([1, 1].to_elements(), state.hd_op_bits());
    assert_eq!([0].to_elements(), state.ctx_stack());
    assert_eq!([0].to_elements(), state.loop_stack());
    assert_eq!([15, 0, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());

    // execute one iteration
    let inputs = ProgramInputs::new(
        &[BaseElement::new(5), BaseElement::new(3)],
        &[BaseElement::new(1), BaseElement::new(0)],
        &[],
    );
    let trace = processor::execute(&program, &inputs);

    assert_eq!(128, trace.length());
    assert_eq!(19, trace.width());
    let state = get_trace_state(&trace, trace.length() - 1);

    assert_eq!(BaseElement::new(75), state.op_counter());
    assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
    assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
    assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
    assert_eq!([1, 1].to_elements(), state.hd_op_bits());
    assert_eq!([0].to_elements(), state.ctx_stack());
    assert_eq!([0].to_elements(), state.loop_stack());
    assert_eq!([225, 0, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());

    // execute five iteration
    let inputs = ProgramInputs::new(
        &[BaseElement::new(5), BaseElement::new(3)],
        &[
            BaseElement::new(1),
            BaseElement::new(1),
            BaseElement::new(1),
            BaseElement::new(1),
            BaseElement::new(1),
            BaseElement::new(0),
        ],
        &[],
    );
    let trace = processor::execute(&program, &inputs);

    assert_eq!(256, trace.length());
    assert_eq!(19, trace.width());
    let state = get_trace_state(&trace, trace.length() - 1);

    assert_eq!(BaseElement::new(135), state.op_counter());
    assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
    assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
    assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
    assert_eq!([1, 1].to_elements(), state.hd_op_bits());
    assert_eq!([0].to_elements(), state.ctx_stack());
    assert_eq!([0].to_elements(), state.loop_stack());
    assert_eq!(
        [43143988327398919500410556793212890625, 0, 0, 0, 0, 0, 0, 0].to_elements(),
        state.user_stack()
    );
}

fn get_trace_state(trace: &ExecutionTrace<BaseElement>, step: usize) -> TraceState<BaseElement> {
    let meta = TraceMetadata::from_trace_info(&trace.get_info());
    let mut row = vec![BaseElement::ZERO; trace.width()];
    trace.read_row_into(step, &mut row);
    TraceState::from_slice(meta.ctx_depth, meta.loop_depth, meta.stack_depth, &row)
}
