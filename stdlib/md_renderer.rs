use crate::{ModuleMap, Renderer};
use assembly::{
    ast::{ModuleAst, ProcReExport, ProcedureAst},
    LibraryPath,
};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

// MARKDOWN RENDERER
// ================================================================================================

pub struct MarkdownRenderer {}

impl MarkdownRenderer {
    fn write_docs_header(mut writer: &File, ns: &str) {
        let header =
            format!("\n## {ns}\n| Procedure | Description |\n| ----------- | ------------- |\n");
        writer.write_all(header.as_bytes()).expect("unable to write header to writer");
    }

    fn write_docs_procedure(mut writer: &File, proc: &ProcedureAst) {
        if proc.docs.is_none() {
            return;
        }
        let func_output = format!(
            "| {} | {} |\n",
            proc.name.as_str(),
            proc.docs.clone().unwrap().replace('|', "\\|").replace('\n', "<br /><br />")
        );
        writer
            .write_all(func_output.as_bytes())
            .expect("unable to write func to writer");
    }

    fn write_docs_reexported_proc(mut writer: &File, proc: &ProcReExport) {
        if proc.docs().is_none() {
            return;
        }
        let func_output = format!(
            "| {} | {} |\n",
            proc.name().as_str(),
            proc.docs().unwrap().replace('|', "\\|").replace('\n', "<br /><br />")
        );
        writer
            .write_all(func_output.as_bytes())
            .expect("unable to write func to writer");
    }

    fn write_docs_module(mut writer: &File, module: &ModuleAst) {
        if module.docs().is_none() {
            return;
        }
        writer
            .write_all(module.docs().unwrap().replace('\n', "<br />").as_bytes())
            .expect("unable to write module comments");
    }
}

impl Renderer for MarkdownRenderer {
    fn render(stdlib: &ModuleMap, output_dir: &str) {
        // Write per module markdown file
        for (ns, module) in stdlib.iter() {
            let (dir_path, file_path) = get_dir_and_file_paths(ns, output_dir);

            // Create the directories if they don't exist
            fs::create_dir_all(dir_path).expect("Failed to create directory");

            let f = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_path)
                .expect("unable to open stdlib markdown file");

            Self::write_docs_module(&f, module);
            Self::write_docs_header(&f, ns);
            for reexported_proc in module.reexported_procs().iter() {
                Self::write_docs_reexported_proc(&f, reexported_proc);
            }
            for proc in module.procs().iter() {
                Self::write_docs_procedure(&f, proc);
            }
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn get_dir_and_file_paths(ns: &str, output_dir: &str) -> (PathBuf, PathBuf) {
    let mut dir_parts: Vec<&str> = ns.split(LibraryPath::PATH_DELIM).collect();
    let file_name = dir_parts.pop().unwrap();
    let dir_path = dir_parts
        .iter()
        .skip(1)
        .fold(PathBuf::from(output_dir), |acc, part| acc.join(part));
    let file_path = dir_path.join(format!("{}.md", file_name));
    (dir_path, file_path)
}
