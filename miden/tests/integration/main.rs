mod helpers;

mod air;
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

// MACROS TO BUILD TESTS
// ================================================================================================

/// Returns a Test struct in non debug mode from the provided source string, and any specified
/// stack and advice inputs.
///
/// Parameters are expected in the following order:
/// `source`, `stack_inputs` (optional), `advice_tape` (optional), `advice_sets` (optional)
///
/// * `source`: a well-formed source string.
/// * `stack_inputs` (optional): the initial inputs which must be at the top of the stack before
/// executing the `source`. Stack inputs can be provided independently without any advice inputs.
/// * `advice_tape` (optional): the initial advice tape values. When provided, `stack_inputs` and
/// `advice_sets` are also expected.
/// * `advice_sets` (optional): the initial advice set values. When provided, `stack_inputs` and
/// `advice_tape` are also expected.
#[macro_export]
macro_rules! build_test {
    ($source:expr) => {{
        $crate::build_debug_test!($source, false)
    }};
    ($source:expr, $stack_inputs:expr) => {{
        $crate::build_debug_test!($source, false, $stack_inputs)
    }};
    ($source:expr, $stack_inputs:expr, $advice_tape:expr, $advice_sets:expr) => {{
        $crate::build_debug_test!($source, false, $stack_inputs, $advice_tape, $advice_sets)
    }};
}

/// Returns a Test struct in non debug mode from a string of one or more operations and any
/// specified stack and advice inputs.
///
/// Parameters are expected in the following order:
/// `source`, `stack_inputs` (optional), `advice_tape` (optional), `advice_sets` (optional)
///
/// * `source`: a string of one or more operations, e.g. "push.1 push.2".
/// * `stack_inputs` (optional): the initial inputs which must be at the top of the stack before
/// executing the `source`. Stack inputs can be provided independently without any advice inputs.
/// * `advice_tape` (optional): the initial advice tape values. When provided, `stack_inputs` and
/// `advice_sets` are also expected.
/// * `advice_sets` (optional): the initial advice set values. When provided, `stack_inputs` and
/// `advice_tape` are also expected.
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

/// Returns a Test struct from the provided source string, debug mode boolean flag and any specified
/// stack and advice inputs.
///
/// Parameters are expected in the following order:
/// `source`, `in_debug_mode`, `stack_inputs` (optional), `advice_tape` (optional), `advice_sets` (optional)
///
/// * `source`: a well-formed source string.
/// * `in_debug_mode` : boolean flag indicated whether to build test in debug mode.
/// * `stack_inputs` (optional): the initial inputs which must be at the top of the stack before
/// executing the `source`. Stack inputs can be provided independently without any advice inputs.
/// * `advice_tape` (optional): the initial advice tape values. When provided, `stack_inputs` and
/// `advice_sets` are also expected.
/// * `advice_sets` (optional): the initial advice set values. When provided, `stack_inputs` and
/// `advice_tape` are also expected.
#[macro_export]
macro_rules! build_debug_test {
    ($source:expr, $in_debug_mode:expr) => {{
        $crate::helpers::Test::new($source, $in_debug_mode)
    }};
    ($source:expr, $in_debug_mode:expr, $stack_inputs:expr) => {{
        let inputs = $crate::helpers::ProgramInputs::new($stack_inputs, &[], vec![]).unwrap();

        $crate::helpers::Test {
            source: String::from($source),
            inputs,
            in_debug_mode: $in_debug_mode,
        }
    }};
    ($source:expr, $in_debug_mode:expr, $stack_inputs:expr, $advice_tape:expr, $advice_sets:expr) => {{
        let inputs =
            $crate::helpers::ProgramInputs::new($stack_inputs, $advice_tape, $advice_sets).unwrap();

        $crate::helpers::Test {
            source: String::from($source),
            inputs,
            in_debug_mode: $in_debug_mode,
        }
    }};
}
