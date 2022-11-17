use crate::ModuleMap;
use crate::Renderer;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use vm_assembly::ProcedureAst;

// MARKDOWN RENDERER
// ================================================================================================

pub struct MarkdownRenderer {}

impl MarkdownRenderer {
    fn write_docs_header(mut writer: &File, ns: &str) {
        let header = format!(
            "\n## {}\n| Procedure | Description |\n| ----------- | ------------- |\n",
            ns
        );
        writer
            .write_all(header.as_bytes())
            .expect("unable to write header to writer");
    }

    fn write_docs_procedure(mut writer: &File, proc: &ProcedureAst) {
        if proc.docs.is_none() {
            return;
        }
        let func_output = format!(
            "| {} | {} |\n",
            proc.name,
            proc.docs
                .clone()
                .unwrap()
                .replace('|', "\\|")
                .replace('\n', "<br /><br />")
        );
        writer
            .write_all(func_output.as_bytes())
            .expect("unable to write func to writer");
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
