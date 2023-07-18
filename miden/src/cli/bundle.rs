use assembly::{LibraryNamespace, MaslLibrary, Version};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Compile Library",
    about = "Bundles .masm files into a single .masl library"
)]
pub struct BundleCmd {
    /// Path to a directory containing the `.masm` files which are part of the library.
    #[structopt(parse(from_os_str))]
    dir: PathBuf,
    /// Defines the top-level namespace, e.g. `mylib`, otherwise the directory name is used.
    #[structopt(short, long)]
    namespace: Option<String>,
    /// Version of the library, defaults to `0.1.0`.
    #[structopt(short, long, default_value = "0.1.0")]
    version: String,
}

impl BundleCmd {
    pub fn execute(&self) -> Result<(), String> {
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
            LibraryNamespace::try_from(namespace.clone()).expect("invalid base namespace");
        let version = Version::try_from(self.version.as_ref()).expect("invalid cargo version");
        let with_source_locations = true;
        let stdlib = MaslLibrary::read_from_dir(
            self.dir.clone(),
            library_namespace,
            with_source_locations,
            version,
        )
        .map_err(|e| e.to_string())?;

        // write the masl output
        stdlib.write_to_dir(self.dir.clone()).map_err(|e| e.to_string())?;

        println!("Built library {}", namespace);

        Ok(())
    }
}
