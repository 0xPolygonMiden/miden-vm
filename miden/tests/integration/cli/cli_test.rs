use std::{fs, path::Path};

use assert_cmd::prelude::*;
use predicates::prelude::*;
extern crate escargot;

fn bin_under_test() -> escargot::CargoRun {
    escargot::CargoBuild::new()
        .bin("miden")
        .features("executable internal")
        .current_release()
        .current_target()
        .run()
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            panic!("failed to build `miden`");
        })
}

#[test]
// Tt test might be an overkill to test only that the 'run' cli command
// outputs steps and ms.
fn cli_run() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = bin_under_test().command();

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

use assembly::{mast::MastNode, Library};

#[test]
fn cli_bundle_debug() {
    let mut cmd = bin_under_test().command();
    cmd.arg("bundle").arg("--debug").arg("./tests/integration/cli/data/lib");
    cmd.assert().success();

    let lib = Library::deserialize_from_file("./tests/integration/cli/data/out.masl").unwrap();
    let found_one_decorator = lib.mast_forest().nodes().iter().any(|node| match node {
        MastNode::Block(node) => !node.decorators().is_empty(),
        _ => false,
    });
    assert!(found_one_decorator);
    fs::remove_file("./tests/integration/cli/data/out.masl").unwrap();
}

#[test]
fn cli_bundle_no_exports() {
    let mut cmd = bin_under_test().command();
    cmd.arg("bundle").arg("./tests/integration/cli/data/lib_noexports");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("library must contain at least one exported procedure"));
}

#[test]
fn cli_bundle_kernel() {
    let mut cmd = bin_under_test().command();
    cmd.arg("bundle")
        .arg("./tests/integration/cli/data/lib")
        .arg("--kernel")
        .arg("./tests/integration/cli/data/kernel_main.masm");
    cmd.assert().success();
    fs::remove_file("./tests/integration/cli/data/out.masl").unwrap()
}

/// A kernel can bundle with a library w/o exports.
#[test]
fn cli_bundle_kernel_noexports() {
    let mut cmd = bin_under_test().command();
    cmd.arg("bundle")
        .arg("./tests/integration/cli/data/lib_noexports")
        .arg("--kernel")
        .arg("./tests/integration/cli/data/kernel_main.masm");
    cmd.assert().success();
    fs::remove_file("./tests/integration/cli/data/out.masl").unwrap()
}

#[test]
fn cli_bundle_output() {
    let mut cmd = bin_under_test().command();
    cmd.arg("bundle")
        .arg("./tests/integration/cli/data/lib")
        .arg("--output")
        .arg("test.masl");
    cmd.assert().success();
    assert!(Path::new("test.masl").exists());
    fs::remove_file("test.masl").unwrap()
}
