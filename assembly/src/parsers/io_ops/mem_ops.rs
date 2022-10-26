use super::{
    super::{check_u32_param, Instruction},
    push_value, AssemblyError, Operation,
};
use vm_core::Felt;

// INSTRUCTION PARSERS
// ================================================================================================

// appends the write mem operation based on flags.
fn append_mem_op(span_ops: &mut Vec<Operation>, is_single: bool, is_load: bool) {
    if is_single {
        if is_load {
            span_ops.push(Operation::MLoad);
        } else {
            span_ops.push(Operation::MStore);
        }
    } else if is_load {
        span_ops.push(Operation::MLoadW);
    } else {
        span_ops.push(Operation::MStoreW);
    }
}

/// Appends operations to the span block to execute a memory read or write operation from a local
/// memory.
/// Specifically, this handles loc_load, loc_loadw, loc_store and loc_storew instructions.
///
/// VM cycles per operation:
/// - loc_load(w).b:
///    - 4 cycles if b = 1
///    - 3 cycles if b != 1
/// - loc_store(w).b:
///    - 4 cycles if b = 1
///    - 3 cycles if b != 1
pub fn parse_mem_local(
    instruction: &Instruction,
    span_ops: &mut Vec<Operation>,
    num_proc_locals: u32,
    is_single: bool,
    is_load: bool,
    index: u32,
) -> Result<(), AssemblyError> {
    // parse the provided local address and push it onto the stack
    push_local_addr(instruction, span_ops, num_proc_locals, index)?;

    append_mem_op(span_ops, is_single, is_load);

    Ok(())
}

/// Appends operations to the span block to execute a memory read or write operation from a
/// global memory address.
/// Specifically, this handles mem_load, mem_loadw instructions.
///
/// VM cycles per operation:
/// - mem_load(w): 1 cycle
/// - mem_load(w).b: 2 cycles
/// - mem_store(w): 1 cycle
/// - mem_store(w).b: 2 cycles
pub fn parse_mem_global(
    span_ops: &mut Vec<Operation>,
    is_single: bool,
    is_load: bool,
    address: Option<Felt>,
) -> Result<(), AssemblyError> {
    if let Some(a) = address {
        // parse the provided memory address and push it onto the stack
        push_value(span_ops, a);
    }

    append_mem_op(span_ops, is_single, is_load);

    Ok(())
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
///
/// This function will return an `AssemblyError` if the index parameter is greater than the number
/// of locals declared by the procedure.
fn push_local_addr(
    instruction: &Instruction,
    span_ops: &mut Vec<Operation>,
    num_proc_locals: u32,
    index: u32,
) -> Result<(), AssemblyError> {
    if num_proc_locals == 0 {
        // if no procedure locals were declared, then no local mem ops are allowed
        return Err(AssemblyError::invalid_instruction_with_reason(
            &instruction.to_string(),
            "no procedure locals were declared",
        ));
    }

    // parse the provided local memory index
    let index = check_u32_param(instruction, index, 0, num_proc_locals - 1)?;

    // put the absolute memory address on the stack; the absolute address is computed by
    // subtracting index of the local from the fmp value. this way, the first local is located at
    // fmp - (num_proc_locals - 1) (i.e., the smallest address) and the last local is located at
    // fmp (i.e., the largest address).
    push_value(span_ops, -Felt::from(num_proc_locals - index - 1));
    span_ops.push(Operation::FmpAdd);

    Ok(())
}
