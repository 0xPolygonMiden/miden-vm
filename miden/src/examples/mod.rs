use air::ExecutionOptions;
use miden::{AdviceProvider, ExecutionProof, Program, ProgramInfo, ProvingOptions, StackInputs};
use std::io::Write;
use std::time::Instant;
use structopt::StructOpt;

pub mod fibonacci;

// EXAMPLE
// ================================================================================================

pub struct Example<A>
where
    A: AdviceProvider,
{
    pub program: Program,
    pub stack_inputs: StackInputs,
    pub advice_provider: A,
    pub num_outputs: usize,
    pub expected_result: Vec<u64>,
}

// EXAMPLE OPTIONS
// ================================================================================================

#[derive(StructOpt, Debug)]
#[structopt(name = "Examples", about = "Run an example miden program")]
pub struct ExampleOptions {
    #[structopt(subcommand)]
    pub example: ExampleType,

    /// Security level for execution proofs generated by the VM
    #[structopt(short = "s", long = "security", default_value = "96bits")]
    security: String,

    /// Enable generation of proofs suitable for recursive verification
    #[structopt(short = "r", long = "recursive")]
    recursive: bool,
}

#[derive(StructOpt, Debug)]
//#[structopt(about = "available examples")]
pub enum ExampleType {
    /// Compute a Fibonacci sequence of the specified length
    Fib {
        /// Length of Fibonacci sequence
        #[structopt(short = "n", default_value = "1024")]
        sequence_length: usize,
    },
}

impl ExampleOptions {
    pub fn get_proof_options(&self) -> ProvingOptions {
        match self.security.as_str() {
            "96bits" => {
                ProvingOptions::with_96_bit_security(self.recursive, ExecutionOptions::default())
            }
            "128bits" => {
                ProvingOptions::with_128_bit_security(self.recursive, ExecutionOptions::default())
            }
            other => panic!("{} is not a valid security level", other),
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        println!("============================================================");

        // configure logging
        env_logger::Builder::new()
            .format(|buf, record| writeln!(buf, "{}", record.args()))
            .filter_level(log::LevelFilter::Debug)
            .init();

        let proof_options = self.get_proof_options();

        // instantiate and prepare the example
        let example = match self.example {
            ExampleType::Fib { sequence_length } => fibonacci::get_example(sequence_length),
        };

        let Example {
            program,
            stack_inputs,
            advice_provider,
            num_outputs,
            expected_result,
            ..
        } = example;
        println!("--------------------------------");

        // execute the program and generate the proof of execution
        let now = Instant::now();
        let (stack_outputs, proof) =
            miden::prove(&program, stack_inputs.clone(), advice_provider, proof_options).unwrap();
        println!("--------------------------------");

        println!(
            "Executed program in {} ms",
            //hex::encode(program.hash()), // TODO: include into message
            now.elapsed().as_millis()
        );
        println!("Stack outputs: {:?}", stack_outputs.stack_truncated(num_outputs));
        assert_eq!(
            expected_result,
            stack_outputs.stack_truncated(num_outputs),
            "Program result was computed incorrectly"
        );

        // serialize the proof to see how big it is
        let proof_bytes = proof.to_bytes();
        println!("Execution proof size: {} KB", proof_bytes.len() / 1024);
        println!("Execution proof security: {} bits", proof.security_level());
        println!("--------------------------------");

        // verify that executing a program with a given hash and given inputs
        // results in the expected output
        let proof = ExecutionProof::from_bytes(&proof_bytes).unwrap();
        let now = Instant::now();
        let program_info = ProgramInfo::from(program);

        match miden::verify(program_info, stack_inputs, stack_outputs, proof) {
            Ok(_) => println!("Execution verified in {} ms", now.elapsed().as_millis()),
            Err(err) => println!("Failed to verify execution: {}", err),
        }

        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
pub fn test_example<A>(example: Example<A>, fail: bool)
where
    A: AdviceProvider,
{
    let Example {
        program,
        stack_inputs,
        advice_provider,
        num_outputs,
        expected_result,
    } = example;

    let (mut outputs, proof) =
        miden::prove(&program, stack_inputs.clone(), advice_provider, ProvingOptions::default())
            .unwrap();

    assert_eq!(
        expected_result,
        outputs.stack_truncated(num_outputs),
        "Program result was computed incorrectly"
    );

    let kernel = miden::Kernel::default();
    let program_info = ProgramInfo::new(program.hash(), kernel);

    if fail {
        outputs.stack_mut()[0] += 1;
        assert!(miden::verify(program_info, stack_inputs, outputs, proof).is_err())
    } else {
        assert!(miden::verify(program_info, stack_inputs, outputs, proof).is_ok());
    }
}
