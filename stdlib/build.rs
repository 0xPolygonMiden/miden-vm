use assembly::{
    ast::AstSerdeOptions,
    diagnostics::{IntoDiagnostic, Result},
    library::CompiledLibrary,
    LibraryNamespace, Version,
};
use std::{env, path::Path};

// CONSTANTS
// ================================================================================================

const ASM_DIR_PATH: &str = "./asm";
const ASL_DIR_PATH: &str = "./assets";
const _DOC_DIR_PATH: &str = "./docs";

// PRE-PROCESSING
// ================================================================================================

/// Read and parse the contents from `./asm` into a `LibraryContents` struct, serializing it into
/// `assets` folder under `std` namespace.
fn main() -> Result<()> {
    // re-build the `[OUT_DIR]/assets/std.masl` file iff something in the `./asm` directory
    // or its builder changed:
    println!("cargo:rerun-if-changed=asm");
    println!("cargo:rerun-if-changed=../assembly/src");

    let namespace = "std".parse::<LibraryNamespace>().expect("invalid base namespace");
    // TODO: Add version to `Library`
    let _version = env!("CARGO_PKG_VERSION").parse::<Version>().expect("invalid cargo version");
    let stdlib = CompiledLibrary::from_dir(ASM_DIR_PATH, namespace)?;

    // write the masl output
    let build_dir = env::var("OUT_DIR").unwrap();
    let options = AstSerdeOptions::new(false, false);
    stdlib
        .write_to_dir(Path::new(&build_dir).join(ASL_DIR_PATH), options)
        .into_diagnostic()?;

    Ok(())
}
