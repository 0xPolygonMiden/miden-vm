use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::PathBuf,
};
use vm_assembly::{parse_module, ProcedureId};

mod stdlib_docs;

// CONSTANTS
// ================================================================================================

const ASM_DIR_PATH: &str = "./asm";
const ASM_FILE_PATH: &str = "./src/asm.rs";
const DOC_DIR_PATH: &str = "./docs";
const MODULE_SEPARATOR: &str = "::";

// TYPE ALIASES
// ================================================================================================

type ModuleMap = BTreeMap<String, String>;

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
    pub serialized: Vec<u8>,
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
                    let module = parse_module(&source)
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.message().as_str()))?;
                    let serialized = module.to_bytes();

                    parsed.push(Module {
                        label,
                        id,
                        source,
                        serialized,
                    });
                }
            }
        }

        Ok(parsed)
    }
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

    // Docs label-> source mapping
    // TODO it might be preferrable to defer the docs parsing to a step before AST generation
    // instead of using different pipelines
    let mut docs = BTreeMap::new();

    modules.into_iter().try_for_each(
        |Module {
             label,
             id,
             source,
             serialized,
         }| {
            docs.insert(label.clone(), source.clone());

            writeln!(
                output,
                "(\"{label}\",vm_assembly::ProcedureId({:?}),\"{source}\",&{serialized:?}),",
                id.0
            )
        },
    )?;

    writeln!(output, "];")?;

    // updates the documentation of these modules
    stdlib_docs::build_stdlib_docs(&docs, DOC_DIR_PATH);

    Ok(())
}
