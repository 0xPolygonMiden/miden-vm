use super::data::ScriptFile;
use crypto::Digest;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Compile", about = "Compile a miden program")]
pub struct CompileCmd {
    /// Path to .masm assembly file
    #[structopt(short = "a", long = "assembly", parse(from_os_str))]
    assembly_file: PathBuf,
}

impl CompileCmd {
    pub fn execute(&self) -> Result<(), String> {
        println!("============================================================");
        println!("Compile script");
        println!("============================================================");

        // load and compile script file
        let script = ScriptFile::read(&self.assembly_file)?;

        // report script hash to user
        println!("script hash is {}", hex::encode(script.hash().as_bytes()));

        Ok(())
    }
}
