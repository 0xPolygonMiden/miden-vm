use std::{env, path::Path, sync::Arc};

use assembly::{
    ast::AstSerdeOptions,
    diagnostics::{IntoDiagnostic, Result},
    library::CompiledLibrary,
    LibraryNamespace, Version,
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

    let source_manager = Arc::new(assembly::DefaultSourceManager::default());
    let namespace = "std".parse::<LibraryNamespace>().expect("invalid base namespace");
    // TODO: Add version to `Library`
    let _version = env!("CARGO_PKG_VERSION").parse::<Version>().expect("invalid cargo version");
    let stdlib = CompiledLibrary::from_dir(asm_dir, namespace, source_manager)?;

    // write the masl output
    let build_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(&build_dir);
    let options = AstSerdeOptions::new(false, false);
    let output_file = build_dir
        .join(ASL_DIR_PATH)
        .join("std")
        .with_extension(CompiledLibrary::LIBRARY_EXTENSION);
    stdlib.write_to_file(output_file, options).into_diagnostic()?;

    Ok(())
}
