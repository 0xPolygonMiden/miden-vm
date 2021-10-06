use distaff::{BaseElement, FieldExtension, HashFunction, Program, ProgramInputs, ProofOptions};
use structopt::StructOpt;

pub mod collatz;
pub mod comparison;
pub mod conditional;
pub mod fibonacci;
pub mod merkle;
pub mod range;

// EXAMPLE
// ================================================================================================

pub struct Example {
    pub program: Program,
    pub inputs: ProgramInputs,
    pub num_outputs: usize,
    pub expected_result: Vec<BaseElement>,
}

// EXAMPLE OPTIONS
// ================================================================================================

#[derive(StructOpt, Debug)]
#[structopt(name = "winterfell", about = "Winterfell examples")]
pub struct ExampleOptions {
    #[structopt(subcommand)]
    pub example: ExampleType,

    /// Hash function used in the protocol
    #[structopt(short = "h", long = "hash_fn", default_value = "blake3_256")]
    hash_fn: String,

    /// Number of queries to include in a proof
    #[structopt(short = "q", long = "queries", default_value = "28")]
    num_queries: usize,

    /// Blowup factor for low degree extension
    #[structopt(short = "b", long = "blowup", default_value = "8")]
    blowup_factor: usize,

    /// Grinding factor for query seed
    #[structopt(short = "g", long = "grinding", default_value = "16")]
    grinding_factor: u32,

    /// Whether to use field extension for composition polynomial
    #[structopt(short = "e", long = "extension")]
    field_extension: bool,
}

impl ExampleOptions {
    pub fn get_proof_options(&self) -> ProofOptions {
        let field_extension = if self.field_extension {
            FieldExtension::Quadratic
        } else {
            FieldExtension::None
        };
        let hash_fn = match self.hash_fn.as_str() {
            "blake3_192" => HashFunction::Blake3_192,
            "blake3_256" => HashFunction::Blake3_256,
            "sha3_256" => HashFunction::Sha3_256,
            val => panic!("'{}' is not a valid hash function option", val),
        };

        ProofOptions::new(
            self.num_queries,
            self.blowup_factor,
            self.grinding_factor,
            hash_fn,
            field_extension,
            8,
            256,
        )
    }
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
    /// Compute a Collatz sequence from the specified starting value
    Collatz {
        /// Starting value of the Collatz sequence
        #[structopt(short = "n", default_value = "511")]
        start_value: usize,
    },
    /// If provided value is less than 9, multiplies it by 9; otherwise add 9 to it
    Comparison {
        /// Value to compare to 9
        #[structopt(short = "n", default_value = "11")]
        value: usize,
    },
    /// If provided value is 0, outputs 15; if provided value is 1, outputs 8
    Conditional {
        /// Value to compare to 9
        #[structopt(short = "n", default_value = "1")]
        value: usize,
    },
    /// Computes a root of a randomly generated Merkle branch of the specified depth
    Merkle {
        /// Depth of the Merkle tree
        #[structopt(short = "n", default_value = "20")]
        tree_depth: usize,
    },
    /// Determines how many of the randomly generated values are less than 2^63
    Range {
        /// Number of randomly generated 64-bit values
        #[structopt(short = "n", default_value = "100")]
        num_values: usize,
    },
}
