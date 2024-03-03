use crate::Renderer;
use assembly::{
    ast::{Export, Module, Procedure, ProcedureAlias},
    Library, LibraryPath, MaslLibrary,
};
use std::{
    borrow::Cow,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

// MARKDOWN RENDERER
// ================================================================================================

pub struct MarkdownRenderer {}

impl MarkdownRenderer {
    fn escape_module_docs(s: &str) -> Cow<'_, str> {
        if s.contains('\n') {
            Cow::Owned(s.to_string().replace('\n', "<br />"))
        } else {
            Cow::Borrowed(s)
        }
    }

    fn escape_procedure_docs(s: &str) -> Cow<'_, str> {
        if s.contains(['|', '\n']) {
            Cow::Owned(s.to_string().replace('|', "\\|").replace('\n', "<br />"))
        } else {
            Cow::Borrowed(s)
        }
    }

    fn write_docs_header(mut writer: &File, module: &Module) {
        let ns = module.path();
        let header =
            format!("\n## {ns}\n| Procedure | Description |\n| ----------- | ------------- |\n");
        writer.write_all(header.as_bytes()).expect("unable to write header to writer");
    }

    fn write_docs_procedure(mut writer: &File, proc: &Procedure) {
        if let Some(docs) = proc.docs() {
            let escaped = Self::escape_procedure_docs(docs);
            writer
                .write_fmt(format_args!("| {} | {} |\n", proc.name(), &escaped,))
                .expect("unable to write procedure to writer");
        }
    }

    fn write_docs_reexported_proc(mut writer: &File, proc: &ProcedureAlias) {
        if let Some(docs) = proc.docs() {
            let escaped = Self::escape_procedure_docs(docs);
            writer
                .write_fmt(format_args!("| {} | {} |\n", proc.name(), &escaped,))
                .expect("unable to write procedure alias to writer");
        }
    }

    fn write_docs_module(mut writer: &File, module: &Module) {
        if let Some(docs) = module.docs() {
            let escaped = Self::escape_module_docs(docs.into_inner());
            writer.write_all(escaped.as_bytes()).expect("unable to write module docs");
        }
    }
}

impl Renderer for MarkdownRenderer {
    fn render(stdlib: &MaslLibrary, output_dir: &str) {
        // Write per module markdown file
        for module in stdlib.modules() {
            let (dir_path, file_path) = get_dir_and_file_paths(module.path().clone(), output_dir);

            // Create the directories if they don't exist
            fs::create_dir_all(dir_path).expect("Failed to create directory");

            let f = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_path)
                .expect("unable to open stdlib markdown file");

            Self::write_docs_module(&f, module);
            Self::write_docs_header(&f, module);
            for export in module.procedures() {
                if let Export::Alias(ref reexported) = export {
                    Self::write_docs_reexported_proc(&f, reexported);
                }
            }
            for export in module.procedures() {
                if let Export::Procedure(ref proc) = export {
                    Self::write_docs_procedure(&f, proc);
                }
            }
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn get_dir_and_file_paths(mut path: LibraryPath, output_dir: &str) -> (PathBuf, PathBuf) {
    let file_name = path.pop().unwrap();
    let dir_path = path
        .components()
        .skip(1)
        .fold(PathBuf::from(output_dir), |acc, component| acc.join(component.as_ref()));
    let file_path = dir_path.join(format!("{}.md", file_name));
    (dir_path, file_path)
}
