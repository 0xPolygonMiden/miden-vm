mod helpers;

mod air;
mod cli;
mod exec_iters;
mod flow_control;
mod operations;
mod stdlib;

// TESTS
// ================================================================================================

#[test]
fn simple_program() {
    build_test!("begin push.1 push.2 add end").expect_stack(&[3]);
}

#[test]
fn multi_output_program() {
    let test = build_test!("begin mul movup.2 drop end", &[1, 2, 3]);
    test.prove_and_verify(vec![1, 2, 3], false);
}

// MACROS TO BUILD TESTS
// ================================================================================================

/// Returns a Test struct in non debug mode from a string of one or more operations and any
/// specified stack and advice inputs.
///
/// Parameters are expected in the following order:
/// `source`, `stack_inputs` (optional), `advice_stack` (optional), `merkle_store` (optional)
///
/// * `source`: a string of one or more operations, e.g. "push.1 push.2".
/// * `stack_inputs` (optional): the initial inputs which must be at the top of the stack before
/// executing the `source`. Stack inputs can be provided independently without any advice inputs.
/// * `advice_stack` (optional): the initial advice stack values. When provided, `stack_inputs` and
/// `merkle_store` are also expected.
/// * `merkle_store` (optional): the initial merkle set values. When provided, `stack_inputs` and
/// `advice_stack` are also expected.
#[macro_export]
macro_rules! build_op_test {
    ($op_str:expr) => {{
        let source = format!("begin {} end", $op_str);
        $crate::build_test!(&source)
    }};
    ($op_str:expr, $($tail:tt)+) => {{
        let source = format!("begin {} end", $op_str);
        $crate::build_test!(&source, $($tail)+)
    }};
}

/// Returns a Test struct in non debug mode from the provided source string, and any specified
/// stack and advice inputs.
///
/// Parameters are expected in the following order:
/// `source`, `stack_inputs` (optional), `advice_stack` (optional), `merkle_store` (optional)
///
/// * `source`: a well-formed source string.
/// * `stack_inputs` (optional): the initial inputs which must be at the top of the stack before
/// executing the `source`. Stack inputs can be provided independently without any advice inputs.
/// * `advice_stack` (optional): the initial advice stack values. When provided, `stack_inputs` and
/// `merkle_store` are also expected.
/// * `merkle_store` (optional): the initial merkle set values. When provided, `stack_inputs` and
/// `advice_stack` are also expected.
#[macro_export]
macro_rules! build_test {
    ($($params:tt)+) => {{
        $crate::build_test_by_mode!(false, $($params)+)
    }}
}

/// Returns a Test struct in debug mode from the provided source string and any specified stack
/// and advice inputs.
///
/// Parameters are expected in the following order:
/// `source`, `stack_inputs` (optional), `advice_stack` (optional), `merkle_store` (optional)
///
/// * `source`: a well-formed source string.
/// * `stack_inputs` (optional): the initial inputs which must be at the top of the stack before
/// executing the `source`. Stack inputs can be provided independently without any advice inputs.
/// * `advice_stack` (optional): the initial advice stack values. When provided, `stack_inputs` and
/// `merkle_store` are also expected.
/// * `merkle_store` (optional): the initial merkle set values. When provided, `stack_inputs` and
/// `advice_stack` are also expected.
#[macro_export]
macro_rules! build_debug_test {
    ($($params:tt)+) => {{
        $crate::build_test_by_mode!(true, $($params)+)
    }}
}

/// Returns a Test struct in the specified debug or non-debug mode using the provided source string
/// and any specified stack and advice inputs.
///
/// Parameters start with a boolean flag, `in_debug_mode`, specifying whether the test is built in
/// debug or non-debug mode. After that, they match the parameters of `build_test` and
///`build_debug_test` macros.
///
/// This macro is an internal test builder, and is not intended to be called directly from tests.
/// Instead, the build_test and build_debug_test wrappers should be used.
#[macro_export]
macro_rules! build_test_by_mode {
    ($in_debug_mode:expr, $source:expr) => {{
        $crate::helpers::Test::new($source, $in_debug_mode)
    }};
    ($in_debug_mode:expr, $source:expr, $stack_inputs:expr) => {{
        let stack_inputs: Vec<u64> = $stack_inputs.to_vec();
        let stack_inputs = $crate::helpers::StackInputs::try_from_values(stack_inputs).unwrap();
        let advice_inputs = $crate::helpers::AdviceInputs::default();

        $crate::helpers::Test {
            source: String::from($source),
            kernel: None,
            stack_inputs,
            advice_inputs,
            in_debug_mode: $in_debug_mode,
        }
    }};
    (
        $in_debug_mode:expr, $source:expr, $stack_inputs:expr, $advice_stack:expr
    ) => {{
        let stack_inputs: Vec<u64> = $stack_inputs.to_vec();
        let stack_inputs = $crate::helpers::StackInputs::try_from_values(stack_inputs).unwrap();
        let stack_values: Vec<u64> = $advice_stack.to_vec();
        let store = $crate::helpers::MerkleStore::new();
        let advice_inputs = $crate::helpers::AdviceInputs::default()
            .with_stack_values(stack_values)
            .unwrap()
            .with_merkle_store(store);

        $crate::helpers::Test {
            source: String::from($source),
            kernel: None,
            stack_inputs,
            advice_inputs,
            in_debug_mode: $in_debug_mode,
        }
    }};
    (
        $in_debug_mode:expr, $source:expr, $stack_inputs:expr, $advice_stack:expr, $advice_merkle_store:expr
    ) => {{
        let stack_inputs: Vec<u64> = $stack_inputs.to_vec();
        let stack_inputs = $crate::helpers::StackInputs::try_from_values(stack_inputs).unwrap();
        let stack_values: Vec<u64> = $advice_stack.to_vec();
        let advice_inputs = $crate::helpers::AdviceInputs::default()
            .with_stack_values(stack_values)
            .unwrap()
            .with_merkle_store($advice_merkle_store);

        $crate::helpers::Test {
            source: String::from($source),
            kernel: None,
            stack_inputs,
            advice_inputs,
            in_debug_mode: $in_debug_mode,
        }
    }};
    ($in_debug_mode:expr, $source:expr, $stack_inputs:expr, $advice_stack:expr, $advice_merkle_store:expr, $advice_map:expr) => {{
        let stack_inputs: Vec<u64> = $stack_inputs.to_vec();
        let stack_inputs = $crate::helpers::StackInputs::try_from_values(stack_inputs).unwrap();
        let stack_values: Vec<u64> = $advice_stack.to_vec();
        let advice_inputs = $crate::helpers::AdviceInputs::default()
            .with_stack_values(stack_values)
            .unwrap()
            .with_merkle_store($advice_merkle_store)
            .with_map($advice_map);

        $crate::helpers::Test {
            source: String::from($source),
            kernel: None,
            stack_inputs,
            advice_inputs,
            in_debug_mode: $in_debug_mode,
        }
    }};
}
