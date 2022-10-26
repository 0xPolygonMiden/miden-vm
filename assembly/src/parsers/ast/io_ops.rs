use super::super::{parse_decimal_param, parse_element_param, parse_hex_param};
use super::{parse_param, Instruction, Node, Vec};
use crate::{validate_operation, AssemblyError, Token};
use vm_core::Felt;

// CONSTANTS
// ================================================================================================

/// The maximum number of constant inputs allowed by `push` operation.
const MAX_CONST_INPUTS: usize = 16;

// Push constant

/// The required length of the hexadecimal representation for an input value when more than one hex
/// input is provided to `push` without period separators.
const HEX_CHUNK_SIZE: usize = 16;

/// The maximum number of constant inputs allowed by `push` operation.
const MAX_CONST_INPUTS: usize = 16;

fn parse_push_constants(op: &Token) -> Result<Vec<Felt>, AssemblyError> {
    let mut constants = Vec::<Felt>::new();
    let param_idx = 1;
    let param_count = op.num_parts() - param_idx;
    // for multiple input parameters, parse & push each one onto the stack in order, then return
    if param_count > 1 {
        for param_idx in param_idx..=param_count {
            let value = parse_element_param(op, param_idx)?;
            constants.push(value);
        }
        return Ok(constants);
    }

    // for a single input, there could be one value or there could be a series of many hexadecimal
    // values without separators
    let param_str = op.parts()[param_idx];
    if let Some(param_str) = param_str.strip_prefix("0x") {
        // parse 1 or more hexadecimal values
        let values = parse_hex_params(op, param_idx, param_str)?;
        // push each value onto the stack in order
        for &value in values.iter() {
            constants.push(value);
        }
    } else {
        // parse 1 decimal value and push it onto the stack
        let value = parse_decimal_param(op, param_idx, param_str)?;
        constants.push(value);
    }

    Ok(constants)
}

pub fn parse_push(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "push", 1..MAX_CONST_INPUTS);

    let constants = parse_push_constants(op)?;

    Ok(Node::Instruction(Instruction::PushConstants(constants)))
}

pub fn parse_sdepth(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "sdepth", 0);

    Ok(Node::Instruction(Instruction::Sdepth))
}

pub fn parse_locaddr(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "locaddr", 1);

    let param = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::Locaddr(param)))
}

pub fn parse_adv_push(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "adv_push", 1);

    let param = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::AdvPush(param)))
}

pub fn parse_adv_loadw(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "adv_loadw", 0);
    Ok(Node::Instruction(Instruction::AdvLoadW))
}

pub fn parse_adv_inject(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "adv.u64div");

    let node = match op.parts()[1] {
        "u64div" => Node::Instruction(Instruction::AdvU64Div),
        _ => return Err(AssemblyError::invalid_op(op)),
    };

    Ok(node)
}

pub fn parse_mem_load(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "mem_load", 0..1);
    let node = match op.num_parts() {
        2 => {
            let address = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::MemLoadImm(address))
        }
        _ => Node::Instruction(Instruction::MemLoad),
    };

    Ok(node)
}

pub fn parse_loc_load(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_load", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocLoad(index)))
}

pub fn parse_mem_loadw(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "mem_loadw", 0..1);
    let node = match op.num_parts() {
        2 => {
            let address = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::MemLoadWImm(address))
        }
        _ => Node::Instruction(Instruction::MemLoadW),
    };

    Ok(node)
}

pub fn parse_loc_loadw(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_loadw", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocLoadW(index)))
}

pub fn parse_mem_store(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "mem_store", 0..1);
    let node = match op.num_parts() {
        2 => {
            let address = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::MemStoreImm(address))
        }
        _ => Node::Instruction(Instruction::MemStore),
    };

    Ok(node)
}

pub fn parse_loc_store(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_store", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocStore(index)))
}

pub fn parse_mem_storew(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "mem_storew", 0..1);
    let node = match op.num_parts() {
        2 => {
            let address = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::MemStoreWImm(address))
        }
        _ => Node::Instruction(Instruction::MemStoreW),
    };

    Ok(node)
}

pub fn parse_loc_storew(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_storew", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocStoreW(index)))
}

// UTILITY FUNCTIONS
// ================================================================================================

fn parse_hex_params(
    op: &Token,
    param_idx: usize,
    param_str: &str,
) -> Result<Vec<Felt>, AssemblyError> {
    // handle error cases where the hex string is poorly formed
    let is_single_element = if param_str.len() <= HEX_CHUNK_SIZE {
        if param_str.len() % 2 != 0 {
            // parameter string is not a valid hex representation
            return Err(AssemblyError::invalid_param(op, param_idx));
        }
        true
    } else {
        if param_str.len() % HEX_CHUNK_SIZE != 0 {
            // hex string doesn't contain a valid number of bytes
            return Err(AssemblyError::invalid_param(op, param_idx));
        }
        false
    };

    // parse the hex string into one or more valid field elements
    if is_single_element {
        // parse a single element in hex representation
        let parsed_param = parse_hex_param(op, param_idx, param_str)?;
        Ok(vec![parsed_param])
    } else {
        // iterate over the multi-value hex string and parse each 8-byte chunk into a valid element
        (0..param_str.len())
            .step_by(HEX_CHUNK_SIZE)
            .map(|i| parse_hex_param(op, param_idx, &param_str[i..i + HEX_CHUNK_SIZE]))
            .collect()
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        parse_adv_loadw, parse_adv_push, parse_loc_load, parse_loc_loadw, parse_loc_store,
        parse_loc_storew, parse_locaddr, parse_mem_load, parse_mem_loadw, parse_mem_store,
        parse_mem_storew, parse_push, parse_push_constants, parse_sdepth, AssemblyError, Felt,
        Instruction, Node, Token,
    };

    type ParserFn = fn(&Token) -> Result<Node, AssemblyError>;

    // TESTS FOR PUSHING VALUES ONTO THE STACK (PUSH)
    // ============================================================================================

    #[test]
    fn push_one() {
        let op_0 = Token::new("push.0", 0);
        let op_1 = Token::new("push.1", 0);
        let op_dec = Token::new("push.135", 0);
        let op_hex = Token::new("push.0x7b", 0);

        assert_eq!(
            parse_push_constants(&op_0).expect("Failed to parse push.0"),
            vec![Felt::new(0)]
        );
        assert_eq!(
            parse_push_constants(&op_1).expect("Failed to parse push.1"),
            vec![Felt::new(1)]
        );
        assert_eq!(
            parse_push_constants(&op_dec).expect("Failed to parse push of decimal element 123"),
            vec![Felt::new(135)]
        );
        assert_eq!(
            parse_push_constants(&op_hex).expect("Failed to parse push of hex element 0x7b"),
            vec![Felt::new(123)]
        );
    }

    #[test]
    fn push_many() {
        // --- push 4 decimal values --------------------------------------------------------------
        let op_4_dec = Token::new("push.4.5.6.7", 0);
        let mut expected = Vec::with_capacity(4);
        for a in 4..8 {
            expected.push(Felt::new(a));
        }
        let constants = parse_push_constants(&op_4_dec).expect("Failed to parse push.4.5.6.7");
        assert_eq!(constants, expected);

        // --- push the maximum number of decimal values (16) -------------------------------------
        let op_16_dec = Token::new("push.16.17.18.19.20.21.22.23.24.25.26.27.28.29.30.31", 0);
        let mut expected = Vec::with_capacity(16);
        for a in 16..32 {
            expected.push(Felt::new(a));
        }
        let constants = parse_push_constants(&op_16_dec)
            .expect("Failed to parse push.16.17.18.19.20.21.22.23.24.25.26.27.28.29.30.31");
        assert_eq!(constants, expected);

        // --- push hexadecimal values with period separators between values ----------------------
        let op_5_hex = Token::new("push.0xA.0x64.0x3E8.0x2710.0x186A0", 0);
        let mut expected = Vec::with_capacity(5);
        for i in 1..=5 {
            expected.push(Felt::new(10_u64.pow(i)));
        }
        let constants = parse_push_constants(&op_5_hex)
            .expect("Failed to parse push.0xA.0x64.0x3EB.0x2710.0x186A0");
        assert_eq!(constants, expected);

        // --- push a mixture of decimal and single-element hexadecimal values --------------------
        let op_8_dec_hex = Token::new("push.2.4.8.0x10.0x20.0x40.128.0x100", 0);
        let mut expected = Vec::with_capacity(8);
        for i in 1_u32..=8 {
            expected.push(Felt::new(2_u64.pow(i)));
        }
        let constants = parse_push_constants(&op_8_dec_hex)
            .expect("Failed to parse push.2.4.8.0x10.0x20.0x40.128.0x100");
        assert_eq!(constants, expected);
    }

    #[test]
    fn push_without_separator() {
        // --- push hexadecimal values with no period separators ----------------------------------
        let op_sep = Token::new("push.0x1234.0xabcd", 0);
        let op_no_sep = Token::new("push.0x0000000000001234000000000000abcd", 0);
        let constants = parse_push_constants(&op_sep).expect("Failed to parse push.0x1234.0xabcd");
        let expected = parse_push_constants(&op_no_sep)
            .expect("Failed to parse push.0x0000000000001234000000000000abcd");
        assert_eq!(constants, expected);

        // --- push the maximum number of hexadecimal values without separators (16) --------------
        let op_16_dec = Token::new("push.0.1.2.3.4.5.6.7.8.9.10.11.12.13.14.15", 0);
        let op_16_no_sep = Token::new("push.0x0000000000000000000000000000000100000000000000020000000000000003000000000000000400000000000000050000000000000006000000000000000700000000000000080000000000000009000000000000000A000000000000000B000000000000000C000000000000000D000000000000000E000000000000000F", 0);
        let expected = parse_push_constants(&op_16_dec)
            .expect("Failed to parse push.0.1.2.3.4.5.6.7.8.9.10.11.12.13.14.15");
        let constants = parse_push_constants(&op_16_no_sep)
            .expect("Failed to parse push.0x0000000000000000000000000000000100000000000000020000000000000003000000000000000400000000000000050000000000000006000000000000000700000000000000080000000000000009000000000000000A000000000000000B000000000000000C000000000000000D000000000000000E000000000000000F");
        assert_eq!(constants, expected);
    }

    #[test]
    fn push_invalid() {
        // fails when immediate value is invalid or missing
        let pos = 0;

        // wrong operation passed to parsing function
        let op_mismatch = Token::new("pushw.0", pos);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "push");
        assert_eq!(parse_push(&op_mismatch).unwrap_err(), expected);

        // missing value or variant
        let op_no_val = Token::new("push", pos);
        let expected = AssemblyError::missing_param(&op_no_val);
        assert_eq!(parse_push(&op_no_val).unwrap_err(), expected);

        // invalid value
        let op_val_invalid = Token::new("push.abc", pos);
        let expected = AssemblyError::invalid_param(&op_val_invalid, 1);
        assert_eq!(parse_push(&op_val_invalid).unwrap_err(), expected);

        // --- separators for single values cannot be combined with multi-element hex inputs ------
        let op_mixed_sep = Token::new("push.4.5.0x0000000000001234000000000000abcd", 0);
        let expected = AssemblyError::invalid_param(&op_mixed_sep, 3);
        assert_eq!(parse_push(&op_mixed_sep).unwrap_err(), expected);

        let op_mixed_sep = Token::new("push.0x0000000000001234000000000000abcd.4.5", 0);
        let expected = AssemblyError::invalid_param(&op_mixed_sep, 1);
        assert_eq!(parse_push(&op_mixed_sep).unwrap_err(), expected);

        // extra value
        let op_extra_val = Token::new("push.0.1.2.3.4.5.6.7.8.9.10.11.12.13.14.15.16", pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(parse_push(&op_extra_val).unwrap_err(), expected);
    }

    #[test]
    fn push_invalid_hex() {
        // --- no separators, and total number of bytes is not a multiple of 8 --------------------
        let op_bad_hex = Token::new("push.0x00000000000012340000abcd", 0);
        let expected = AssemblyError::invalid_param(&op_bad_hex, 1);
        assert_eq!(parse_push(&mut &op_bad_hex).unwrap_err(), expected);

        // --- some period separators, but total number of bytes is not a multiple of 8 --------------------
        let op_bad_hex = Token::new("push.0x00001234000000000000abcd.0x5678.0x9ef0", 0);
        let expected = AssemblyError::invalid_param(&op_bad_hex, 1);
        assert_eq!(parse_push(&op_bad_hex).unwrap_err(), expected);

        // --- too many values provided in hex format without separators --------------
        let op_extra_val = Token::new("push.0x0000000000000000000000000000000100000000000000020000000000000003000000000000000400000000000000050000000000000006000000000000000700000000000000080000000000000009000000000000000A000000000000000B000000000000000C000000000000000D000000000000000E000000000000000F0000000000000010", 0);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(parse_push(&op_extra_val).unwrap_err(), expected);
    }

    #[test]
    fn adv_push() {
        // remove n items from the advice tape and push them onto the stack
        let op = Token::new("adv_push.4", 0);

        let node = parse_adv_push(&op).expect("Failed to parse adv_push.4");
        assert_eq!(node, Node::Instruction(Instruction::AdvPush(4)));
    }

    #[test]
    fn adv_push_invalid() {
        // fails when the instruction is malformed or unrecognized
        let pos = 0;

        // missing value
        let op_no_val = Token::new("adv_push", pos);
        let expected = AssemblyError::missing_param(&op_no_val);
        assert_eq!(parse_adv_push(&op_no_val).unwrap_err(), expected);

        // extra value to push
        let op_extra_val = Token::new("adv_push.2.2", pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(parse_adv_push(&op_extra_val).unwrap_err(), expected);

        // invalid value - char
        let op_invalid_char = Token::new("adv_push.a", pos);
        let expected = AssemblyError::invalid_param(&op_invalid_char, 1);
        assert_eq!(parse_adv_push(&op_invalid_char).unwrap_err(), expected);

        // invalid value - hexadecimal
        let op_invalid_hex = Token::new("adv_push.0x10", pos);
        let expected = AssemblyError::invalid_param(&op_invalid_hex, 1);
        assert_eq!(parse_adv_push(&op_invalid_hex).unwrap_err(), expected);
    }

    // TESTS FOR OVERWRITING VALUES ON THE STACK (LOAD)
    // ============================================================================================
    #[test]
    fn loadw_adv() {
        // replace the top 4 elements of the stack with 4 elements from the advice tape
        let op = Token::new("adv_loadw", 0);
        let node = parse_adv_loadw(&op).expect("Failed to parse adv_loadw");
        assert_eq!(node, Node::Instruction(Instruction::AdvLoadW));
    }

    #[test]
    fn loadw_adv_invalid() {
        // fails when the instruction is malformed or unrecognized
        let pos = 0;

        // extra value to loadw
        let op_extra_val = Token::new("adv_loadw.0", pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(parse_adv_loadw(&op_extra_val).unwrap_err(), expected);
    }

    // TESTS FOR PUSHING VALUES ONTO THE STACK (PUSH)
    // ============================================================================================

    #[test]
    fn locaddr() {
        let asm_op = "locaddr";
        let op_str = format!("{}.{}", asm_op, 1);
        let op = Token::new(&op_str, 0);
        assert!(parse_locaddr(&op).is_ok());
    }

    #[test]
    fn sdepth_invalid() {
        // fails when env op variant has too many immediate values
        let pos = 0;

        // --- extra param ------------------------------------------------------------------------
        let op_extra_val = Token::new("sdepth.0", pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(parse_sdepth(&op_extra_val).unwrap_err(), expected);
    }

    #[test]
    fn locaddr_invalid() {
        let asm_op = "locaddr";
        let pos = 0;

        // --- invalid env var --------------------------------------------------------------------
        let op_val_invalid = Token::new("invalid", pos);
        assert_eq!(
            parse_locaddr(&op_val_invalid).unwrap_err(),
            AssemblyError::unexpected_token(&op_val_invalid, "locaddr")
        );

        // --- missing param ----------------------------------------------------------------------
        let op_missing_param = Token::new(asm_op, pos);
        let expected = AssemblyError::missing_param(&op_missing_param);
        assert_eq!(parse_locaddr(&op_missing_param).unwrap_err(), expected);
    }

    // TESTS FOR READING FROM MEMORY
    // ============================================================================================

    #[test]
    fn mem_load() {
        let expected = vec![
            Node::Instruction(Instruction::MemLoad),
            Node::Instruction(Instruction::MemLoadImm(Felt::new(0))),
            Node::Instruction(Instruction::MemLoadImm(Felt::new(2))),
        ];
        test_parse_mem("mem_load", expected, parse_mem_load);
    }

    #[test]
    fn loc_load() {
        let expected = Node::Instruction(Instruction::LocLoad(0));
        test_parse_local("loc_load", expected, parse_loc_load);
    }

    #[test]
    fn mem_load_invalid() {
        test_parse_mem_invalid("mem_load", parse_mem_load);
    }

    #[test]
    fn loc_load_invalid() {
        test_parse_local_invalid("loc_load", parse_loc_load);
    }

    #[test]
    fn mem_loadw() {
        let expected = vec![
            Node::Instruction(Instruction::MemLoadW),
            Node::Instruction(Instruction::MemLoadWImm(Felt::new(0))),
            Node::Instruction(Instruction::MemLoadWImm(Felt::new(2))),
        ];
        test_parse_mem("mem_loadw", expected, parse_mem_loadw);
    }

    #[test]
    fn loc_loadw() {
        let expected = Node::Instruction(Instruction::LocLoadW(0));
        test_parse_local("loc_loadw", expected, parse_loc_loadw);
    }

    #[test]
    fn mem_loadw_invalid() {
        test_parse_mem_invalid("mem_loadw", parse_mem_loadw);
    }

    #[test]
    fn loc_loadw_invalid() {
        test_parse_local_invalid("loc_loadw", parse_loc_loadw);
    }

    // TESTS FOR WRITING INTO MEMORY
    // ============================================================================================

    #[test]
    fn mem_store() {
        let expected = vec![
            Node::Instruction(Instruction::MemStore),
            Node::Instruction(Instruction::MemStoreImm(Felt::new(0))),
            Node::Instruction(Instruction::MemStoreImm(Felt::new(2))),
        ];
        test_parse_mem("mem_store", expected, parse_mem_store);
    }

    #[test]
    fn loc_store() {
        let expected = Node::Instruction(Instruction::LocStore(0));
        test_parse_local("loc_store", expected, parse_loc_store);
    }

    #[test]
    fn mem_store_invalid() {
        test_parse_mem_invalid("mem_store", parse_mem_store);
    }

    #[test]
    fn loc_store_invalid() {
        test_parse_local_invalid("loc_store", parse_loc_store);
    }

    #[test]
    fn mem_storew() {
        let expected = vec![
            Node::Instruction(Instruction::MemStoreW),
            Node::Instruction(Instruction::MemStoreWImm(Felt::new(0))),
            Node::Instruction(Instruction::MemStoreWImm(Felt::new(2))),
        ];
        test_parse_mem("mem_storew", expected, parse_mem_storew);
    }

    #[test]
    fn loc_storew() {
        let expected = Node::Instruction(Instruction::LocStoreW(0));
        test_parse_local("loc_storew", expected, parse_loc_storew);
    }

    #[test]
    fn mem_storew_invalid() {
        test_parse_mem_invalid("mem_storew", parse_mem_storew);
    }

    #[test]
    fn loc_storew_invalid() {
        test_parse_local_invalid("loc_storew", parse_loc_storew);
    }

    // TEST HELPERS
    // ============================================================================================

    /// Test that an instruction for an absolute memory operation is properly formed. It can be used
    /// to test parameter inputs for `mem_load`, `mem_store`, `mem_loadw`, and `mem_storew`.
    fn test_parse_mem_invalid(base_op: &str, parser: ParserFn) {
        // fails when immediate values to a {mem_load|mem_loadw|mem_store|mem_storew}.{a|} operation
        // are invalid or missing
        let pos = 0;

        // invalid value provided to mem variant
        let op_str = format!("{}.abc", base_op);
        let op_val_invalid = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_param(&op_val_invalid, 1);
        assert_eq!(parser(&op_val_invalid).unwrap_err(), expected);

        // extra value provided to mem variant
        let op_str = format!("{}.0.1", base_op);
        let op_extra_val = Token::new(&op_str, pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(parser(&op_extra_val).unwrap_err(), expected);
    }

    /// Test that an instruction for a local memory operation is properly formed. It can be used to
    /// test parameter inputs for loc_load, loc_store, loc_loadw, and loc_storew.
    fn test_parse_local_invalid(base_op: &str, parser: ParserFn) {
        // fails when immediate values to a {loc_load|loc_store|loc_loadw|loc_storew}.i operation are
        // invalid or missing
        let pos = 0;

        // insufficient values provided
        let op_val_missing = Token::new(base_op, pos);
        let expected = AssemblyError::missing_param(&op_val_missing);
        assert_eq!(parser(&op_val_missing).unwrap_err(), expected);

        // invalid value provided to local variant
        let op_str = format!("{}.abc", base_op);
        let op_val_invalid = Token::new(&op_str, pos);
        let expected = AssemblyError::invalid_param(&op_val_invalid, 1);
        assert_eq!(parser(&op_val_invalid).unwrap_err(), expected);

        // extra value provided to local variant
        let op_str = format!("{}.0.1", base_op);
        let op_extra_val = Token::new(&op_str, pos);
        let expected = AssemblyError::extra_param(&op_extra_val);
        assert_eq!(parser(&op_extra_val).unwrap_err(), expected);
    }

    /// Helper function for optimizing local operations testing. It can be used to
    /// test loc_load, loc_store, loc_loadw and loc_storew operations.
    fn test_parse_local(base_op: &str, expected: Node, parser: ParserFn) {
        let pos = 0;

        let op_str = format!("{}.0", base_op);
        let op = Token::new(&op_str, pos);
        let msg = format!("Failed to parse {}.0 (address provided by op)", base_op);

        let node = parser(&op).expect(&msg);

        assert_eq!(expected, node);
    }

    /// Helper function for optimizing memory operations testing. It can be used to
    /// test mem_load, mem_store, mem_loadw and mem_storew operations.
    fn test_parse_mem(base_op: &str, expected: Vec<Node>, parser: ParserFn) {
        let pos = 0;

        let op = Token::new(base_op, pos);
        let msg = format!("Failed to parse {}", base_op);

        let node = parser(&op).expect(&msg);

        assert_eq!(node, expected[0]);

        // test push with memory address provided directly (address 0)
        let op_str = format!("{}.0", base_op);
        let op = Token::new(&op_str, pos);
        let msg = format!("Failed to parse {}.0", base_op);

        let node = parser(&op).expect(&msg);

        assert_eq!(node, expected[1]);

        // test push with memory address provided directly (address 2)
        let op_str = format!("{}.2", base_op);
        let op = Token::new(&op_str, pos);
        let msg = format!("Failed to parse {}.2", base_op);

        let node = parser(&op).expect(&msg);

        assert_eq!(node, expected[2]);
    }
}
