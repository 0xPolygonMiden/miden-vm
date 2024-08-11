use std::{env, path::Path};

use assembly::{
    diagnostics::{IntoDiagnostic, Result},
    Assembler, Library, LibraryNamespace,
};

// CONSTANTS
// ================================================================================================

const ASM_DIR_PATH: &str = "asm";
const ASL_DIR_PATH: &str = "assets";

// PRE-PROCESSING
// ================================================================================================

/// Read and parse the contents from `./asm` into a `LibraryContents` struct, serializing it into
/// `assets` folder under `std` namespace.
fn main() -> Result<()> {
    // re-build the `[OUT_DIR]/assets/std.masl` file iff something in the `./asm` directory
    // or its builder changed:
    println!("cargo:rerun-if-changed=asm");
    println!("cargo:rerun-if-changed=../assembly/src");

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let asm_dir = Path::new(manifest_dir).join(ASM_DIR_PATH);

    let assembler = Assembler::default().with_debug_mode(true);
    let namespace = "std".parse::<LibraryNamespace>().expect("invalid base namespace");
    // TODO: Add version to `Library`
    //let version = env!("CARGO_PKG_VERSION").parse::<Version>().expect("invalid cargo version");
    let stdlib = Library::from_dir(asm_dir, namespace, assembler)?;

    // write the masl output
    let build_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(&build_dir);
    let output_file = build_dir
        .join(ASL_DIR_PATH)
        .join("std")
        .with_extension(Library::LIBRARY_EXTENSION);
    stdlib.write_to_file(output_file).into_diagnostic()?;

    Ok(())
}
