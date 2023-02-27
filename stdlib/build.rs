use assembly::{Library, LibraryNamespace, MaslLibrary, ModuleAst, Version};
use std::{collections::BTreeMap, fs, io};

mod md_renderer;
use md_renderer::MarkdownRenderer;

// CONSTANTS
// ================================================================================================

const ASM_DIR_PATH: &str = "./asm";
const ASL_DIR_PATH: &str = "./assets";
const DOC_DIR_PATH: &str = "./docs";

// TYPE ALIASES and HELPER STRUCTS
// ================================================================================================

type ModuleMap = BTreeMap<String, ModuleAst>;

// PRE-PROCESSING
// ================================================================================================

/// Read and parse the contents from `./asm` into a `LibraryContents` struct, serializing it into
/// `assets` folder under `std` namespace.
#[cfg(not(feature = "docs-rs"))]
fn main() -> io::Result<()> {
    // re-build the `./assets/std.masl` file iff something in the `./asm` directory
    // or its builder changed:
    println!("cargo:rerun-if-changed=asm");
    println!("cargo:rerun-if-changed=../assembly/src");

    let namespace = LibraryNamespace::try_from("std".to_string()).expect("invalid base namespace");
    let version = Version::try_from(env!("CARGO_PKG_VERSION")).expect("invalid cargo version");
    let stdlib = MaslLibrary::read_from_dir(ASM_DIR_PATH, namespace, version)?;
    let docs = stdlib
        .modules()
        .map(|module| (module.path.to_string(), module.ast.clone()))
        .collect();

    // write the masl output
    stdlib.write_to_dir(ASL_DIR_PATH)?;

    // updates the documentation of these modules
    build_stdlib_docs(&docs, DOC_DIR_PATH);

    Ok(())
}

// STDLIB DOCUMENTATION
// ================================================================================================

/// A renderer renders a ModuleSourceMap into a particular doc format and index (e.g: markdown, etc)
trait Renderer {
    // Render writes out the document files into the output directory
    fn render(stdlib: &ModuleMap, output_dir: &str);
}

// Writes Miden standard library modules documentation markdown files based on the available modules and comments.
pub fn build_stdlib_docs(module_map: &ModuleMap, output_dir: &str) {
    // Remove functions folder to re-generate
    fs::remove_dir_all(output_dir).unwrap();
    fs::create_dir(output_dir).unwrap();

    // Render the stdlib struct into markdown
    // TODO: Make the renderer choice pluggable.
    MarkdownRenderer::render(module_map, output_dir);
}
