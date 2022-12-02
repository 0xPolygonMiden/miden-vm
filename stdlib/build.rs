use assembly::{parse_module, ModuleAst, ProcedureId};
use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::PathBuf,
};

mod md_renderer;
use md_renderer::MarkdownRenderer;

// CONSTANTS
// ================================================================================================

const ASM_DIR_PATH: &str = "./asm";
const ASM_FILE_PATH: &str = "./src/asm.rs";
const DOC_DIR_PATH: &str = "./docs";

// TYPE ALIASES and HELPER STRUCTS
// ================================================================================================

type ModuleMap = BTreeMap<String, ModuleAst>;

struct ModuleDirectory {
    pub module_path: String,
    pub fs_path: PathBuf,
}

struct Module {
    pub path: String,
    pub source: String,
}

// PRE-PROCESSING
// ================================================================================================

/// Reads the contents of the `./asm` directory, and writes these contents into a single `.rs`
/// file under `./src/asm.rs`.
///
/// The `asm.rs` file exports a single static array of string tuples. Each tuple consist of module
/// namespace label and the module source code for this label.
#[cfg(not(feature = "docs-rs"))]
fn main() -> io::Result<()> {
    // re-build the `./src/asm.rs` file only if something in the `./asm` directory has changed
    println!("cargo:rerun-if-changed=asm");

    let modules = load_modules()?;
    let mut output = fs::File::create(ASM_FILE_PATH)?;

    writeln!(output, "//! This module is automatically generated during build time and should not be modified manually.\n")?;
    writeln!(
        output,
        "/// An array of modules defined in Miden standard library."
    )?;
    writeln!(output, "///")?;
    writeln!(output, "/// Entries in the array are tuples containing module namespace and module parsed+serialized.")?;
    writeln!(output, "#[rustfmt::skip]")?;
    writeln!(
        output,
        "pub const MODULES: [(&str, &[u8]); {}] = [",
        modules.len()
    )?;

    let mut docs = BTreeMap::new();

    modules
        .into_iter()
        .try_for_each(|Module { path, source }| {
            let module = parse_module(&source)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message().as_str()))?;
            let serialized = module.to_bytes();

            docs.insert(path.clone(), module);

            writeln!(output, "(\"{path}\",&{serialized:?}),")
        })?;

    writeln!(output, "];")?;

    // updates the documentation of these modules
    build_stdlib_docs(&docs, DOC_DIR_PATH);

    Ok(())
}

// STDLIB DOCUMENTATION
// ================================================================================================

/// A renderer renders a ModuleMap into a particular doc format and index (e.g: markdown, etc)
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

// HELPER FUNCTIONS
// ================================================================================================

fn load_modules() -> io::Result<Vec<Module>> {
    let mut dirs = Vec::new();
    fill_module_directories(&mut dirs, ASM_DIR_PATH.into(), "std".into())?;

    let mut parsed = Vec::new();

    for ModuleDirectory {
        module_path,
        fs_path: path,
    } in dirs
    {
        for entry in path.read_dir()? {
            let entry = entry?.path();

            if entry.is_file() {
                entry
                    .extension()
                    .and_then(|x| x.to_str())
                    .filter(|x| x == &"masm")
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            format!("invalid file extension at {}", entry.display()),
                        )
                    })?;

                let name = entry.file_stem().and_then(|x| x.to_str()).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("invalid file name at {}", entry.display()),
                    )
                })?;

                let path = ProcedureId::path(name, module_path.as_str());
                let source = fs::read_to_string(entry)?;

                parsed.push(Module { path, source });
            }
        }
    }

    // make sure the modules are sorted in a stable way by their path
    parsed.sort_by(|m1, m2| m1.path.cmp(&m2.path));

    Ok(parsed)
}

fn fill_module_directories(
    state: &mut Vec<ModuleDirectory>,
    path: PathBuf,
    module_path: String,
) -> io::Result<()> {
    state.push(ModuleDirectory {
        module_path: module_path.clone(),
        fs_path: path.clone(),
    });

    for entry in fs::read_dir(path)? {
        let entry = entry?.path();

        if entry.is_dir() {
            entry
                .file_name()
                .and_then(|x| x.to_str())
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to read dir name"))
                .map(|name| ProcedureId::path(name, &module_path))
                .and_then(|module_path| fill_module_directories(state, entry, module_path))?;
        }
    }

    Ok(())
}
