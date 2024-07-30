use assembly::{
    ast::AstSerdeOptions,
    diagnostics::{IntoDiagnostic, Report},
    library::CompiledLibrary,
    LibraryNamespace, Version,
};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[clap(
    name = "Compile Library",
    about = "Bundles .masm files into a single .masl library"
)]
pub struct BundleCmd {
    /// Path to a directory containing the `.masm` files which are part of the library.
    #[clap(value_parser)]
    dir: PathBuf,
    /// Defines the top-level namespace, e.g. `mylib`, otherwise the directory name is used.
    #[clap(short, long)]
    namespace: Option<String>,
    /// Version of the library, defaults to `0.1.0`.
    #[clap(short, long, default_value = "0.1.0")]
    version: String,
}

impl BundleCmd {
    pub fn execute(&self) -> Result<(), Report> {
        println!("============================================================");
        println!("Build library");
        println!("============================================================");

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
        // TODO: Add version to `Library`
        let _version = self.version.parse::<Version>().expect("invalid cargo version");
        let stdlib = CompiledLibrary::from_dir(&self.dir, library_namespace)?;

        // write the masl output
        let options = AstSerdeOptions::new(false, false);
        stdlib.write_to_dir(self.dir.clone(), options).into_diagnostic()?;

        println!("Built library {}", namespace);

        Ok(())
    }
}
