use super::{
    push_felt, push_u16_value, AssemblerError, AssemblyContext, CodeBlock, Felt, Operation::*,
    SpanBuilder, StarkField,
};

// INSTRUCTION PARSERS
// ================================================================================================

/// Appends operations to the span block to execute a memory read operation. This includes reading
/// a single element or an entire word from either local or global memory. Specifically, this
/// handles mem_load, mem_loadw, loc_load, and loc_loadw instructions.
///
/// VM cycles per operation:
/// - mem_load(w): 1 cycle
/// - mem_load(w).b: 2 cycles
/// - loc_load(w).b:
///    - 4 cycles if b = 1
///    - 3 cycles if b != 1
pub fn mem_read(
    span: &mut SpanBuilder,
    context: &AssemblyContext,
    imm: Option<&Felt>,
    is_local: bool,
    is_single: bool,
) -> Result<Option<CodeBlock>, AssemblerError> {
    let num_proc_locals = context.num_proc_locals();
    match imm {
        Some(imm) if is_local => push_local_addr(span, imm, num_proc_locals)?,
        None if is_local => unreachable!("local always contains imm value"),
        Some(imm) => push_felt(span, *imm),
        None => (),
    }

    // load from the memory address on top of the stack
    if is_single {
        span.push_op(MLoad);
    } else {
        span.push_op(MLoadW);
    }

    Ok(None)
}

/// Appends operations to the span block to execute memory write operations. This includes writing
/// a single element or an entire word into either local or global memory. Specifically, this
/// handles mem_store, mem_storew, loc_store, and loc_storew instructions.
///
/// VM cycles per operation:
/// - mem_store(w): 1 cycle
/// - mem_store(w).b: 2 cycles
/// - loc_store(w).b:
///    - 4 cycles if b = 1
///    - 3 cycles if b != 1
pub fn mem_write(
    span: &mut SpanBuilder,
    context: &AssemblyContext,
    imm: Option<&Felt>,
    is_local: bool,
    is_single: bool,
) -> Result<Option<CodeBlock>, AssemblerError> {
    let num_proc_locals = context.num_proc_locals();
    match imm {
        Some(imm) if is_local => push_local_addr(span, imm, num_proc_locals)?,
        None if is_local => unreachable!("local always contains imm value"),
        Some(imm) => push_felt(span, *imm),
        None => (),
    }

    if is_single {
        span.push_op(MStore);
    } else {
        span.push_op(MStoreW);
    }

    Ok(None)
}

// HELPER FUNCTIONS
// ================================================================================================

/// Parses a provided local memory index and pushes the corresponding absolute memory location onto
/// the stack.
///
/// This operation takes:
/// - 3 VM cycles if index == 1
/// - 2 VM cycles if index != 1
///
/// # Errors
/// This function will return an `AssemblyError` if the index parameter is greater than the number
/// of locals declared by the procedure.
fn push_local_addr(
    span: &mut SpanBuilder,
    index: &Felt,
    num_proc_locals: u16,
) -> Result<(), AssemblerError> {
    let index = index.as_int();
    let max = num_proc_locals as u64 - 1;

    // check that the parameter is within the specified bounds
    if index > max {
        return Err(AssemblerError::imm_out_of_bounds(index, 0, max));
    }

    // conversion to u16 is OK here because max < 2^16
    push_u16_value(span, (max - index) as u16);
    span.push_op(FmpAdd);

    Ok(())
}
