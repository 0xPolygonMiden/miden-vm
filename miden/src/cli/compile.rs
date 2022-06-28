use super::{load_and_compile_script, Parser};
use crypto::Digest;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CompileCmd {
    #[clap(short, parse(from_os_str))]
    script_file: PathBuf,
}

impl CompileCmd {
    pub fn execute(&self) -> Result<(), String> {
        // load and compile script file
        let script = load_and_compile_script(&self.script_file)?;

        // report script hash to user
        println!("script hash is {}", hex::encode(script.hash().as_bytes()));

        Ok(())
    }
}
