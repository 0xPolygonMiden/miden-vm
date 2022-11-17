use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::PathBuf,
};
use vm_assembly::{parse_module, ModuleAst, ProcedureId};

mod md_renderer;
use md_renderer::MarkdownRenderer;

// CONSTANTS
// ================================================================================================

const ASM_DIR_PATH: &str = "./asm";
const ASM_FILE_PATH: &str = "./src/asm.rs";
const DOC_DIR_PATH: &str = "./docs";
const MODULE_SEPARATOR: &str = "::";

// TYPE ALIASES
// ================================================================================================

type ModuleMap = BTreeMap<String, ModuleAst>;

// HELPER STRUCTURES
// ================================================================================================

struct ModuleDirectory {
    pub label: String,
    pub path: PathBuf,
}

impl ModuleDirectory {
    fn fill(state: &mut Vec<Self>, path: PathBuf, label: String) -> io::Result<()> {
        state.push(ModuleDirectory {
            label: label.clone(),
            path: path.clone(),
        });

        for entry in fs::read_dir(path)? {
            let entry = entry?.path();

            if entry.is_dir() {
                entry
                    .file_name()
                    .and_then(|x| x.to_str())
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to read dir name"))
                    .map(|name| format!("{label}{MODULE_SEPARATOR}{name}"))
                    .and_then(|label| Self::fill(state, entry, label))?;
            }
        }

        Ok(())
    }
}

struct Module {
    pub label: String,
    pub id: ProcedureId,
    pub source: String,
}

impl Module {
    fn load() -> io::Result<Vec<Self>> {
        let mut dirs = Vec::new();

        ModuleDirectory::fill(&mut dirs, ASM_DIR_PATH.into(), "std".into())?;

        let mut parsed = Vec::new();

        for ModuleDirectory { label, path } in dirs {
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

                    let label = format!("{label}{MODULE_SEPARATOR}{name}");
                    let id = ProcedureId::new(&label);

                    let source = fs::read_to_string(entry)?;

                    parsed.push(Module { label, id, source });
                }
            }
        }

        Ok(parsed)
    }
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

    let modules = Module::load()?;
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
        "pub const MODULES: [(&str, vm_assembly::ProcedureId, &str, &[u8]); {}] = [",
        modules.len()
    )?;

    let mut docs = BTreeMap::new();

    modules
        .into_iter()
        .try_for_each(|Module { label, id, source }| {
            let module = parse_module(&source)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message().as_str()))?;
            let serialized = module.to_bytes();

            docs.insert(label.clone(), module);

            writeln!(
                output,
                "(\"{label}\",vm_assembly::ProcedureId({:?}),\"{source}\",&{serialized:?}),",
                id.0
            )
        })?;

    writeln!(output, "];")?;

    // updates the documentation of these modules
    build_stdlib_docs(&docs, DOC_DIR_PATH);

    Ok(())
}
