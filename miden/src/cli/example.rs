use super::{examples, Parser};
use prover::{StarkProof, ProofOptions};
use std::time::Instant;

#[derive(Debug, Parser)]
pub struct ExampleCmd {
    #[clap(subcommand)]
    example: ExampleType,
}

#[derive(Debug, Parser)]
pub enum ExampleType {
    // Collatz {
    //     #[clap(short)]
    //     input: usize
    // },
    // Comparison {
    //     #[clap(short)]
    //     input: usize
    // },
    // Conditional {
    //     #[clap(short)]
    //     input: usize
    // },
    Fibonacci {
        #[clap(short)]
        input: usize
    }
}


impl ExampleCmd {
    pub fn execute(&self) -> Result<(), String> {
        let example = match self.example {
            ExampleType::Fibonacci { input } => examples::fibonacci::get_example(input),
            // ExampleType::Collatz { input } => examples::collatz::get_example(input),
            // ExampleType::Comparison { input } => examples::comparison::get_example(input),
            // ExampleType::Conditional { input } => examples::conditional::get_example(input),
        };

        let examples::Example {
            program,
            inputs,
            num_outputs,
            pub_inputs,
            expected_result,
        } = example;

        println!("============================================================");

        // execute the program and generate the proof of execution
        let now = Instant::now();
        let (outputs, proof) =
            prover::prove(&program, &inputs, num_outputs, &ProofOptions::with_96_bit_security()).unwrap(); // TODO parse proof options by cli
        println!("--------------------------------");

        // #[cfg(feature = "std")]
        println!(
            "Executed program in {} ms",
            //hex::encode(program.hash()), // TODO: include into message
            now.elapsed().as_millis()
        );
        println!("Program output: {:?}", outputs);
        assert_eq!(
            expected_result, outputs,
            "Program result was computed incorrectly"
        );

        // serialize the proof to see how big it is
        let proof_bytes = proof.to_bytes();
        println!("Execution proof size: {} KB", proof_bytes.len() / 1024);
        println!(
            "Execution proof security: {} bits",
            proof.security_level(true)
        );
        println!("--------------------------------");

        // verify that executing a program with a given hash and given inputs
        // results in the expected output
        let proof = StarkProof::from_bytes(&proof_bytes).unwrap();
        let now = Instant::now();
        match verifier::verify(program.hash(), &pub_inputs, &outputs, proof) {
            Ok(_) => println!("Execution verified in {} ms", now.elapsed().as_millis()),
            Err(err) => println!("Failed to verify execution: {}", err),
        }
        Ok(())
    }
}
