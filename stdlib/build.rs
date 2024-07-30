use assembly::{
    ast::AstSerdeOptions,
    diagnostics::{IntoDiagnostic, Result},
    library::CompiledLibrary,
    LibraryNamespace, MaslLibrary, Version,
};
use std::{env, fs, io, path::Path};

mod md_renderer;
use md_renderer::MarkdownRenderer;

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

    // updates the documentation of these modules
    // TODO(plafer): Reenable this
    // build_stdlib_docs(stdlib, DOC_DIR_PATH).into_diagnostic()?;

    Ok(())
}

// STDLIB DOCUMENTATION
// ================================================================================================

/// A renderer renders a ModuleSourceMap into a particular doc format and index (e.g: markdown, etc)
trait Renderer {
    // Render writes out the document files into the output directory
    fn render(stdlib: &MaslLibrary, output_dir: &str);
}

/// Writes Miden standard library modules documentation markdown files based on the available
/// modules and comments.
pub fn build_stdlib_docs(library: &MaslLibrary, output_dir: &str) -> io::Result<()> {
    // Clean the output folder. This only deletes the folder's content, and not the folder itself,
    // because removing the folder fails on docs.rs
    for entry in fs::read_dir(output_dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            fs::remove_dir_all(entry.path())?;
        } else {
            assert!(metadata.is_file());
            fs::remove_file(entry.path())?;
        }
    }

    // Render the stdlib modules into markdown
    // TODO: Make the renderer choice pluggable.
    MarkdownRenderer::render(library, output_dir);

    Ok(())
}
