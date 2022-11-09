use super::{
    parse_element_param, parse_u32_param, push_value, validate_operation, AssemblyError, Operation,
    Token, Vec,
};
use vm_core::Felt;

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
pub fn parse_mem_read(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
    is_local: bool,
    is_single: bool,
) -> Result<(), AssemblyError> {
    if is_local {
        validate_operation!(op, "loc_load|loc_loadw", 1);
        // parse the provided local address and push it onto the stack
        push_local_addr(span_ops, op, num_proc_locals)?;
    } else {
        validate_operation!(op, "mem_load|mem_loadw", 0..1);
        if op.num_parts() == 2 {
            // parse the provided memory address and push it onto the stack
            push_mem_addr(span_ops, op)?;
        }
    }

    // load from the memory address on top of the stack
    if is_single {
        span_ops.push(Operation::MLoad);
    } else {
        span_ops.push(Operation::MLoadW);
    }

    Ok(())
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
pub fn parse_mem_write(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
    is_local: bool,
    is_single: bool,
) -> Result<(), AssemblyError> {
    if is_local {
        validate_operation!(op, "loc_store|loc_storew", 1);
        push_local_addr(span_ops, op, num_proc_locals)?;
    } else {
        validate_operation!(op, "mem_store|mem_storew", 0..1);
        if op.num_parts() == 2 {
            push_mem_addr(span_ops, op)?;
        }
    }

    if is_single {
        span_ops.push(Operation::MStore);
    } else {
        span_ops.push(Operation::MStoreW);
    }

    Ok(())
}

/// Appends operations to the span block to execute `mem_stream` instruction. The sequence of
/// operations is: MSTREAM RPPERM.
///
/// This instruction requires 2 VM cycles to execute.
pub fn parse_mem_stream(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    debug_assert_eq!(op.parts()[0], "mem_stream");
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }

    span_ops.push(Operation::MStream);
    span_ops.push(Operation::RpPerm);

    Ok(())
}

// HELPER FUNCTIONS
// ================================================================================================

/// Parses a provided memory address and pushes it onto the stack.
///
/// This operation takes 1 VM cycle.
///
/// # Errors
/// This function will return an `AssemblyError` if the address parameter does not exist.
fn push_mem_addr(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    let address = parse_element_param(op, 1)?;
    push_value(span_ops, address);

    Ok(())
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
    span_ops: &mut Vec<Operation>,
    op: &Token,
    num_proc_locals: u32,
) -> Result<(), AssemblyError> {
    if num_proc_locals == 0 {
        // if no procedure locals were declared, then no local mem ops are allowed
        return Err(AssemblyError::invalid_op_with_reason(
            op,
            "no procedure locals were declared",
        ));
    }

    // parse the provided local memory index
    let index = parse_u32_param(op, 1, 0, num_proc_locals - 1)?;

    // put the absolute memory address on the stack; the absolute address is computed by
    // subtracting index of the local from the fmp value. this way, the first local is located at
    // fmp - (num_proc_locals - 1) (i.e., the smallest address) and the last local is located at
    // fmp (i.e., the largest address).
    push_value(span_ops, -Felt::from(num_proc_locals - index - 1));
    span_ops.push(Operation::FmpAdd);

    Ok(())
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{parse_mem_read, parse_mem_write, tests::get_parsing_error, Felt},
        AssemblyError, Operation, Token,
    };

    type ParserFn = fn(&mut Vec<Operation>, &Token, u32, bool, bool) -> Result<(), AssemblyError>;

    // TESTS FOR READING FROM MEMORY
    // ============================================================================================

    #[test]
    fn mem_load() {
        test_parse_mem("mem_load", true, Operation::MLoad, parse_mem_read);
    }

    #[test]
    fn loc_load() {
        test_parse_local("loc_load", true, Operation::MLoad, parse_mem_read);
    }

    #[test]
    fn mem_load_invalid() {
        test_parse_mem_invalid("mem_load");
    }

    #[test]
    fn loc_load_invalid() {
        test_parse_local_invalid("loc_load");
    }

    #[test]
    fn mem_loadw() {
        test_parse_mem("mem_loadw", false, Operation::MLoadW, parse_mem_read);
    }

    #[test]
    fn loc_loadw() {
        test_parse_local("loc_loadw", false, Operation::MLoadW, parse_mem_read);
    }

    #[test]
    fn mem_loadw_invalid() {
        test_parse_mem_invalid("mem_loadw");
    }

    #[test]
    fn loc_loadw_invalid() {
        test_parse_local_invalid("loc_loadw");
    }

    // TESTS FOR WRITING INTO MEMORY
    // ============================================================================================

    #[test]
    fn mem_store() {
        test_parse_mem("mem_store", true, Operation::MStore, parse_mem_write);
    }

    #[test]
    fn loc_store() {
        test_parse_local("loc_store", true, Operation::MStore, parse_mem_write);
    }

    #[test]
    fn mem_store_invalid() {
        test_parse_mem_invalid("mem_store");
    }

    #[test]
    fn loc_store_invalid() {
        test_parse_local_invalid("loc_store");
    }

    #[test]
    fn mem_storew() {
        test_parse_mem("mem_storew", false, Operation::MStoreW, parse_mem_write);
    }

    #[test]
    fn loc_storew() {
        test_parse_local("loc_storew", false, Operation::MStoreW, parse_mem_write);
    }

    #[test]
    fn mem_storew_invalid() {
        test_parse_mem_invalid("mem_storew");
    }

    #[test]
    fn loc_storew_invalid() {
        test_parse_local_invalid("loc_storew");
    }

    // TEST HELPERS
    // ============================================================================================

    /// Test that an instruction for an absolute memory operation is properly formed. It can be used
    /// to test parameter inputs for `mem_load`, `mem_store`, `mem_loadw`, and `mem_storew`.
    fn test_parse_mem_invalid(base_op: &str) {
        let num_proc_locals = 0;

        // fails when immediate values to a {mem_load|mem_loadw|mem_store|mem_storew}.{a|} operation
        // are invalid or missing
        let pos = 0;

        // invalid value provided to mem variant
        let op_str = format!("{}.abc", base_op);
        let op_val_invalid = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_param(&op_val_invalid, 1);
        assert_eq!(
            get_parsing_error(base_op, &op_val_invalid, num_proc_locals),
            expected
        );

        // extra value provided to mem variant
        let op_str = format!("{}.0.1", base_op);
        let op_extra_val = Token::new(&op_str, pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            get_parsing_error(base_op, &op_extra_val, num_proc_locals),
            expected
        );
    }

    /// Test that an instruction for a local memory operation is properly formed. It can be used to
    /// test parameter inputs for loc_load, loc_store, loc_loadw, and loc_storew.
    fn test_parse_local_invalid(base_op: &str) {
        let num_proc_locals = 1;

        // fails when immediate values to a {loc_load|loc_store|loc_loadw|loc_storew}.i operation are
        // invalid or missing
        let pos = 0;

        // insufficient values provided
        let op_val_missing = Token::new(base_op, pos);
        let expected = AssemblyError::missing_param(&op_val_missing);
        assert_eq!(
            get_parsing_error(base_op, &op_val_missing, num_proc_locals),
            expected
        );

        // invalid value provided to local variant
        let op_str = format!("{}.abc", base_op);
        let op_val_invalid = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_param(&op_val_invalid, 1);
        assert_eq!(
            get_parsing_error(base_op, &op_val_invalid, num_proc_locals),
            expected
        );

        // no procedure locals declared
        let op_str = format!("{}.{}", base_op, 1);
        let op_no_locals = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_op_with_reason(
            &op_no_locals,
            "no procedure locals were declared",
        );
        assert_eq!(get_parsing_error(base_op, &op_no_locals, 0), expected);

        // provided local index is outside of the declared bounds of the procedure locals
        let op_str = format!("{}.{}", base_op, num_proc_locals);
        let op_val_invalid = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_param_with_reason(
            &op_val_invalid,
            1,
            format!(
                "parameter value must be greater than or equal to 0 and less than or equal to {}",
                num_proc_locals - 1
            )
            .as_str(),
        );
        assert_eq!(
            get_parsing_error(base_op, &op_val_invalid, num_proc_locals),
            expected
        );

        // extra value provided to local variant
        let op_str = format!("{}.0.1", base_op);
        let op_extra_val = Token::new(&op_str, pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(
            get_parsing_error(base_op, &op_extra_val, num_proc_locals),
            expected
        );
    }

    /// Helper function for optimizing local operations testing. It can be used to
    /// test loc_load, loc_store, loc_loadw and loc_storew operations.
    fn test_parse_local(base_op: &str, is_single: bool, operation: Operation, parser: ParserFn) {
        let num_proc_locals = 1;
        let pos = 0;

        let mut span_ops: Vec<Operation> = Vec::new();
        let op_str = format!("{}.0", base_op);
        let op = Token::new(&op_str, pos);
        let expected = vec![Operation::Pad, Operation::FmpAdd, operation];
        let msg = format!("Failed to parse {}.0 (address provided by op)", base_op);

        parser(&mut span_ops, &op, num_proc_locals, true, is_single).expect(&msg);

        assert_eq!(&span_ops, &expected);
    }

    /// Helper function for optimizing memory operations testing. It can be used to
    /// test mem_load, mem_store, mem_loadw and mem_storew operations.
    fn test_parse_mem(base_op: &str, is_single: bool, operation: Operation, parser: ParserFn) {
        let num_proc_locals = 0;
        let pos = 0;

        // test push with memory address on top of stack
        let mut span_ops: Vec<Operation> = Vec::new();
        let op = Token::new(base_op, pos);
        let expected = vec![operation];
        let msg = format!("Failed to parse {}", base_op);

        parser(&mut span_ops, &op, num_proc_locals, false, is_single).expect(&msg);

        assert_eq!(&span_ops, &expected);

        // test push with memory address provided directly (address 0)
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_str = format!("{}.0", base_op);
        let op = Token::new(&op_str, pos);
        let expected = vec![Operation::Pad, operation];
        let msg = format!("Failed to parse {}.0", base_op);

        parser(&mut span_ops, &op, num_proc_locals, false, is_single).expect(&msg);

        assert_eq!(&span_ops, &expected);

        // test push with memory address provided directly (address 2)
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_str = format!("{}.2", base_op);
        let op = Token::new(&op_str, pos);
        let expected = vec![Operation::Push(Felt::new(2)), operation];
        let msg = format!("Failed to parse {}.2", base_op);

        parser(&mut span_ops, &op, num_proc_locals, false, is_single).expect(&msg);

        assert_eq!(&span_ops, &expected);
    }
}
