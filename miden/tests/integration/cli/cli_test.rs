use assert_cmd::prelude::*;
use predicates::prelude::*;
extern crate escargot;

#[test]
// Tt test might be an overkill to test only that the 'run' cli command
// outputs steps and ms.
fn cli_run() -> Result<(), Box<dyn std::error::Error>> {
    let bin_under_test = escargot::CargoBuild::new()
        .bin("miden")
        .features("executable internal")
        .current_release()
        .current_target()
        .run()
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            panic!("failed to build `miden`");
        });

    let mut cmd = bin_under_test.command();

    cmd.arg("run")
        .arg("-a")
        .arg("./masm-examples/fib/fib.masm")
        .arg("-n")
        .arg("1")
        .arg("-m")
        .arg("8192")
        .arg("-e")
        .arg("8192");

    let output = cmd.unwrap();

    // This tests what we want. Actually it outputs X steps in Y ms.
    // However we the X and the Y can change in future versions.
    // There is no other 'steps in' in the output
    output.assert().stdout(predicate::str::contains("VM cycles"));

    Ok(())
}
