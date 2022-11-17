use vm_core::{code_blocks::CodeBlock, Felt, FieldElement, Operation::*};

use crate::{todo::span_builder::SpanBuilder, AssemblerError};

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
    imm: Option<&Felt>,
    num_proc_locals: u32,
    is_local: bool,
    is_single: bool,
) -> Result<Option<CodeBlock>, AssemblerError> {
    match imm {
        Some(imm) if is_local => push_local_addr(span, imm, num_proc_locals)?,
        None if is_local => unreachable!("local always contains imm value"),
        Some(imm) => push_mem_addr(span, *imm),
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
    imm: Option<&Felt>,
    num_proc_locals: u32,
    is_local: bool,
    is_single: bool,
) -> Result<Option<CodeBlock>, AssemblerError> {
    match imm {
        Some(imm) if is_local => push_local_addr(span, imm, num_proc_locals)?,
        None if is_local => unreachable!("local always contains imm value"),
        Some(imm) => push_mem_addr(span, *imm),
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

/// Parses a provided memory address and pushes it onto the stack.
///
/// This operation takes 1 VM cycle.
///
/// # Errors
/// This function will return an `AssemblyError` if the address parameter does not exist.
fn push_mem_addr(span: &mut SpanBuilder, addr: Felt) {
    if addr == Felt::ZERO {
        span.push_op(Pad);
    } else if addr == Felt::ONE {
        span.push_op(Pad);
        span.push_op(Incr);
    } else {
        span.push_op(Push(addr));
    }
}

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
    num_proc_locals: u32,
) -> Result<(), AssemblerError> {
    let index = index.inner();
    let max = num_proc_locals as u64 - 1;

    // check that the parameter is within the specified bounds
    if index > max {
        return Err(AssemblerError::imm_out_of_bounds(index, 0, max));
    }

    let value = max - index;
    if value == 0 {
        span.push_op(Pad);
    } else if value == 1 {
        span.push_op(Pad);
        span.push_op(Incr);
    } else {
        span.push_op(Push(value.into()));
    }

    span.push_op(FmpAdd);

    Ok(())
}
