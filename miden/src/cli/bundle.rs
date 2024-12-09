use std::path::PathBuf;

use assembly::{
    diagnostics::{IntoDiagnostic, Report},
    Assembler, KernelLibrary, Library, LibraryNamespace,
};
use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(
    name = "Compile Library",
    about = "Bundles .masm files into a single .masl library"
)]
pub struct BundleCmd {
    /// Include debug symbols.
    #[clap(short, long, action)]
    debug: bool,
    /// Path to a directory containing the `.masm` files which are part of the library.
    #[clap(value_parser)]
    dir: PathBuf,
    /// Defines the top-level namespace, e.g. `mylib`, otherwise the directory name is used.
    #[clap(short, long)]
    namespace: Option<String>,
    /// Version of the library, defaults to `0.1.0`.
    #[clap(short, long, default_value = "0.1.0")]
    version: String,
    /// Build a kernel library from module `kernel` and using the library `dir` as kernel
    /// namespace.
    #[clap(short, long)]
    kernel: Option<PathBuf>,
}

impl BundleCmd {
    pub fn execute(&self) -> Result<(), Report> {
        println!("============================================================");
        println!("Build library");
        println!("============================================================");

        let assembler = Assembler::default().with_debug_mode(self.debug);

        // write the masl output
        let output_file = self
            .dir
            .join(self.namespace.as_deref().unwrap_or("out"))
            .with_extension(Library::LIBRARY_EXTENSION);

        match &self.kernel {
            Some(kernel) => {
                assert!(kernel.is_file(), "kernel must be a file");
                let library = KernelLibrary::from_dir(kernel, Some(&self.dir), assembler)?;
                library.write_to_file(output_file).into_diagnostic()?;
                println!(
                    "Built kernel module {} with library {}",
                    kernel.display(),
                    &self.dir.display()
                );
            },
            None => {
                let namespace = match &self.namespace {
                    Some(namespace) => namespace.to_string(),
                    None => self
                        .dir
                        .file_name()
                        .expect("dir must be a folder")
                        .to_string_lossy()
                        .into_owned(),
                };
                let library_namespace =
                    namespace.parse::<LibraryNamespace>().expect("invalid base namespace");
                let library = Library::from_dir(&self.dir, library_namespace, assembler)?;
                library.write_to_file(output_file).into_diagnostic()?;
                println!("Built library {}", namespace);
            },
        }

        Ok(())
    }
}
