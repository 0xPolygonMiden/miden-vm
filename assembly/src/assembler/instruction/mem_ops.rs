use super::{
    push_felt, push_u32_value, validate_param, AssemblerError, AssemblyContext, CodeBlock, Felt,
    Operation::*, SpanBuilder,
};

// INSTRUCTION PARSERS
// ================================================================================================

/// Appends operations to the span needed to execute a memory read instruction. This includes
/// reading a single element or an entire word from either local or global memory. Specifically,
/// this handles mem_load, mem_loadw, loc_load, and loc_loadw instructions.
///
/// VM cycles per operation:
/// - mem_load(w): 1 cycle
/// - mem_load(w).b: 2 cycles
/// - loc_load(w).b:
///    - 4 cycles if b = 1
///    - 3 cycles if b != 1
///
/// # Errors
/// Returns an error if we are reading from local memory and local memory index is greater than
/// the number of procedure locals.
pub fn mem_read(
    span: &mut SpanBuilder,
    context: &AssemblyContext,
    addr: Option<u32>,
    is_local: bool,
    is_single: bool,
) -> Result<Option<CodeBlock>, AssemblerError> {
    // if the address was provided as an immediate value, put it onto the stack
    if let Some(addr) = addr {
        if is_local {
            local_to_absolute_addr(span, addr as u16, context.num_proc_locals())?;
        } else {
            push_u32_value(span, addr);
        }
    } else if is_local {
        unreachable!("local always contains addr value");
    }

    // load from the memory address on top of the stack
    if is_single {
        span.push_op(MLoad);
    } else {
        span.push_op(MLoadW);
    }

    Ok(None)
}

/// Appends operations to the span needed to execute a memory write instruction. This includes
/// writing a single element or an entire word into either local or global memory. Specifically,
/// this handles mem_store, mem_storew, loc_store, and loc_storew instructions.
///
/// VM cycles per operation:
/// - mem_store(w): 1 cycle
/// - mem_store(w).b: 2 cycles
/// - loc_store(w).b:
///    - 4 cycles if b = 1
///    - 3 cycles if b != 1
///
/// # Errors
/// Returns an error if we are writing to local memory and local memory index is greater than
/// the number of procedure locals.
pub fn mem_write(
    span: &mut SpanBuilder,
    context: &AssemblyContext,
    addr: Option<u32>,
    is_local: bool,
    is_single: bool,
) -> Result<Option<CodeBlock>, AssemblerError> {
    // if the address was provided as an immediate value, put it onto the stack
    if let Some(addr) = addr {
        if is_local {
            local_to_absolute_addr(span, addr as u16, context.num_proc_locals())?;
        } else {
            push_u32_value(span, addr);
        }
    } else if is_local {
        unreachable!("local always contains addr value");
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

/// Appends a sequence of operations to the span needed for converting procedure local index to
/// absolute memory address. This consists of putting index onto the stack and then executing
/// LOCADDR operation.
///
/// This operation takes:
/// - 3 VM cycles if index == 1
/// - 2 VM cycles if index != 1
///
/// # Errors
/// Returns an error if index is greater than the number of procedure locals.
pub fn local_to_absolute_addr(
    span: &mut SpanBuilder,
    index: u16,
    num_proc_locals: u16,
) -> Result<(), AssemblerError> {
    let max = num_proc_locals - 1;
    validate_param(index, 0, max)?;

    push_felt(span, -Felt::from(max - index));
    span.push_op(FmpAdd);

    Ok(())
}
