use assembly::{assert_diagnostic_lines, regex};
use test_utils::build_test_by_mode;

use super::*;

/// Ensures that the proper `ExecutionError::InvalidStackDepthOnReturn` diagnostic is generated when
/// the stack depth is invalid on return from a call.
#[test]
fn test_diagnostic_invalid_stack_depth_on_return_call() {
    // returning from a function with non-empty overflow table should result in an error
    let source = "
        proc.foo
            push.1
        end

        begin
            call.foo
        end";

    let build_test = build_test_by_mode!(true, source, &[1, 2]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "when returning from a call or dyncall, stack depth must be 16, but was 17",
        regex!(r#",-\[test[\d]+:7:13\]"#),
        " 6 |         begin",
        " 7 |             call.foo",
        "   :             ^^^^|^^^",
        "   :                 `-- when returning from this call site",
        " 8 |         end",
        "   `----"
    );
}

/// Ensures that the proper `ExecutionError::InvalidStackDepthOnReturn` diagnostic is generated when
/// the stack depth is invalid on return from a dyncall.
#[test]
fn test_diagnostic_invalid_stack_depth_on_return_dyncall() {
    // returning from a function with non-empty overflow table should result in an error
    let source = "
        proc.foo
            push.1
        end

        begin
            procref.foo mem_storew.100 dropw push.100 
            dyncall
        end";

    let build_test = build_test_by_mode!(true, source, &[1, 2]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "when returning from a call or dyncall, stack depth must be 16, but was 17",
        regex!(r#",-\[test[\d]+:8:13\]"#),
        " 7 |             procref.foo mem_storew.100 dropw push.100",
        " 8 |             dyncall",
        "   :             ^^^|^^^",
        "   :                `-- when returning from this call site",
        " 9 |         end",
        "   `----"
    );
}
