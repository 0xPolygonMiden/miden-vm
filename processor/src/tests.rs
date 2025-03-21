use miette::GraphicalTheme;
use test_utils::build_test_by_mode;

use super::*;

/// Ensures that the proper `ExecutionError::InvalidStackDepthOnReturn` diagnostic is generated when
/// the stack depth is invalid on return.
#[test]
fn test_diagnostic_invalid_stack_depth_on_return() {
    // returning from a function with non-empty overflow table should result in an error
    let source = "
        proc.foo
            push.1
        end

        begin
            call.foo
        end";

    // TODO(plafer): use assert_diagnostic once in core
    let build_test = build_test_by_mode!(true, source, &[1, 2]);
    let err = build_test.execute();
    match err {
        Ok(_) => panic!("Expected an error"),
        Err(err) => {
            let mut output = alloc::string::String::new();
            let handler = miette::GraphicalReportHandler::new_themed(GraphicalTheme::none());
            handler.render_report(&mut output, &err).unwrap();
            std::println!("{}", output);
        },
    }

    // TODO(plafer): test dyncall too
}
