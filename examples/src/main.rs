use examples::{Example, ExampleOptions, ExampleType};
use log::debug;
use miden::StarkProof;
use std::{io::Write, time::Instant};
use structopt::StructOpt;

fn main() {
    // configure logging
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter_level(log::LevelFilter::Debug)
        .init();

    // read command-line args
    let options = ExampleOptions::from_args();

    debug!("============================================================");

    let proof_options = options.get_proof_options();

    // instantiate and prepare the example
    let example = match options.example {
        ExampleType::Fib { sequence_length } => examples::fibonacci::get_example(sequence_length),
    };

    let Example {
        program,
        inputs,
        num_outputs,
        pub_inputs,
        expected_result,
    } = example;
    #[cfg(feature = "std")]
    debug!("--------------------------------");

    // execute the program and generate the proof of execution
    #[cfg(feature = "std")]
    let now = Instant::now();
    let (outputs, proof) = miden::execute(&program, &inputs, num_outputs, &proof_options).unwrap();
    debug!("--------------------------------");

    #[cfg(feature = "std")]
    debug!(
        "Executed program in {} ms",
        //hex::encode(program.hash()), // TODO: include into message
        now.elapsed().as_millis()
    );
    debug!("Program output: {:?}", outputs);
    assert_eq!(
        expected_result, outputs,
        "Program result was computed incorrectly"
    );

    // serialize the proof to see how big it is
    let proof_bytes = proof.to_bytes();
    debug!("Execution proof size: {} KB", proof_bytes.len() / 1024);
    debug!(
        "Execution proof security: {} bits",
        proof.security_level(true)
    );
    debug!("--------------------------------");

    // verify that executing a program with a given hash and given inputs
    // results in the expected output
    let proof = StarkProof::from_bytes(&proof_bytes).unwrap();
    let now = Instant::now();
    match miden::verify(program.hash(), &pub_inputs, &outputs, proof) {
        Ok(_) => debug!("Execution verified in {} ms", now.elapsed().as_millis()),
        Err(err) => debug!("Failed to verify execution: {}", err),
    }
}
