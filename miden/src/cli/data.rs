use assembly::Assembler;
use prover::StarkProof;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io::Write};
use vm_core::{hasher::Digest, program::Script, ProgramInputs};
use winter_utils::{Deserializable, SliceReader};

#[derive(Deserialize, Debug)]
pub struct InputsFile {
    pub stack_inputs: Vec<u64>,
    advice_provider_inputs: Option<Vec<u64>>,
}

impl InputsFile {
    pub fn read(inputs_path: &Option<PathBuf>, script_path: &PathBuf) -> Result<Self, String> {
        // If inputs_path has been provided then use this as path.  Alternatively we will
        // replace the script_path extension with `.inputs` as use this as a default.
        let path = match inputs_path {
            Some(path) => path.clone(),
            None => script_path.with_extension("inputs"),
        };

        // read input file to string
        let inputs_file = fs::read_to_string(&path)
            .map_err(|err| format!("Failed to open input file `{}` - {}", path.display(), err))?;

        // deserilaise input data
        let inputs: InputsFile = serde_json::from_str(&inputs_file)
            .map_err(|err| format!("Failed to deserialse input data - {}", err))?;

        Ok(inputs)
    }

    // TODO add handling of advice provider inputs
    pub fn get_program_inputs(&self) -> Result<ProgramInputs, String> {
        ProgramInputs::from_stack_inputs(&self.stack_inputs).map_err(|err| {
            format!(
                "Failed to construct program inputs from input data - {:?}",
                err
            )
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OutputsFile {
    pub outputs: Vec<u64>,
}

impl OutputsFile {
    pub fn read(outputs_path: &Option<PathBuf>, script_path: &PathBuf) -> Result<Self, String> {
        // If outputs_path has been provided then use this as path.  Alternatively we will
        // replace the script_path extension with `.outputs`
        let path = match outputs_path {
            Some(path) => path.clone(),
            None => script_path.with_extension("outputs"),
        };

        // read outputs file to string
        let outputs_file = fs::read_to_string(&path)
            .map_err(|err| format!("Failed to open outputs file `{}` - {}", path.display(), err))?;

        // deserilaise outputs data
        let outputs: OutputsFile = serde_json::from_str(&outputs_file)
            .map_err(|err| format!("Failed to deserialse outputs data - {}", err))?;

        Ok(outputs)
    }

    pub fn write(outputs: Vec<u64>, path: &Option<PathBuf>) -> Result<(), String> {
        match path {
            Some(path) => {
                // if path provided create ouptut file
                let file = fs::File::create(&path).map_err(|err| {
                    format!(
                        "Failed to create output file `{}` - {}",
                        path.display(),
                        err
                    )
                })?;

                // write outputs to output file
                serde_json::to_writer_pretty(file, &Self { outputs })
            }

            // no path provided - write outputs to stdout
            None => serde_json::to_writer_pretty(std::io::stdout(), &Self { outputs }),
        }
        .map_err(|err| format!("Failed to write output data - {}", err))
    }
}

pub fn load_and_compile_script(path: &PathBuf) -> Result<Script, String> {
    // read script file to string
    let script_file = fs::read_to_string(&path)
        .map_err(|err| format!("Failed to open script file `{}` - {}", path.display(), err))?;

    // compile script
    let script = Assembler::new()
        .compile_script(&script_file)
        .map_err(|err| format!("Failed to compile script - {}", err))?;

    Ok(script)
}

pub struct ProofFile;

impl ProofFile {
    pub fn read(proof_path: &Option<PathBuf>, script_path: &PathBuf) -> Result<StarkProof, String> {
        let path = match proof_path {
            Some(path) => path.clone(),
            None => script_path.with_extension("proof"),
        };

        let file = fs::read(&path)
            .map_err(|err| format!("Failed to open proof file `{}` - {}", path.display(), err))?;
        StarkProof::from_bytes(&file)
            .map_err(|err| format!("Failed to decode proof data - {}", err))
    }

    pub fn write(
        proof: StarkProof,
        proof_path: &Option<PathBuf>,
        script_path: &PathBuf,
    ) -> Result<(), String> {
        let path = match proof_path {
            Some(path) => path.clone(),
            None => script_path.with_extension("proof"),
        };
        let mut file = fs::File::create(&path)
            .map_err(|err| format!("Failed to create proof file `{}` - {}", path.display(), err))?;
        file.write(&proof.to_bytes()).unwrap();
        Ok(())
    }
}

pub struct ProgramHash;

impl ProgramHash {
    pub fn read(hash_hex_string: &String) -> Result<Digest, String> {
        let program_hash_bytes = hex::decode(hash_hex_string)
            .map_err(|err| format!("Failed to convert program hash to bytes {}", err))?;
        let mut program_hash_slice = SliceReader::new(&program_hash_bytes);
        let program_hash = Digest::read_from(&mut program_hash_slice)
            .map_err(|err| format!("Failed to deserialise program hash from bytes - {}", err))?;
        Ok(program_hash)
    }
}
