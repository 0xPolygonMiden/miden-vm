use crate::{ModuleMap, Renderer};
use assembly::{ModuleAst, ProcedureAst};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
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

    fn write_docs_module(mut writer: &File, module: &ModuleAst) {
        if module.docs.is_none() {
            return;
        }
        writer
            .write_all(module.docs.clone().unwrap().replace('\n', "<br />").as_bytes())
            .expect("unable to write module comments");
    }
}

impl Renderer for MarkdownRenderer {
    fn render(stdlib: &ModuleMap, output_dir: &str) {
        // Write per module markdown file
        for (ns, module) in stdlib.iter() {
            let file_name = markdown_file_name(ns);
            let file_path = Path::new(output_dir).join(file_name);
            let f = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_path)
                .expect("unable to open stdlib markdown file");
            Self::write_docs_module(&f, module);
            Self::write_docs_header(&f, ns);
            for proc in module.local_procs.iter() {
                Self::write_docs_procedure(&f, proc);
            }
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn get_module_name(ns: &str) -> String {
    let parts: Vec<&str> = ns.split("::").collect();
    String::from(parts[parts.len() - 1])
}

fn get_module_section(ns: &str) -> String {
    let parts: Vec<&str> = ns.split("::").collect();
    String::from(parts[parts.len() - 2])
}

fn markdown_file_name(ns: &str) -> String {
    format!("{}_{}.md", get_module_name(ns), get_module_section(ns))
}
