use crate::{execute, Felt, FieldElement, ProgramInputs, Script, STACK_TOP_SIZE};
use vm_core::utils::IntoBytes;

#[test]
fn blake3_2_to_1_hash() {
    let script = compile(
        "
    use.std::crypto::hashes::blake3

    begin
        exec.blake3::hash
    end
    ",
    );

    // prepare random input byte array
    let i_digest_0: [u8; 32] = rand_utils::rand_array::<Felt, 4>().into_bytes();
    let i_digest_1: [u8; 32] = rand_utils::rand_array::<Felt, 4>().into_bytes();

    let mut i_digest = [0u8; 64];
    i_digest[..32].copy_from_slice(&i_digest_0);
    i_digest[32..].copy_from_slice(&i_digest_1);

    // allocate space on stack so that bytes can be converted to blake3 words
    let mut i_words = [0u64; STACK_TOP_SIZE];

    // convert each of four consecutive little endian bytes (of input) to blake3 words
    for i in 0..STACK_TOP_SIZE {
        i_words[i] = from_le_bytes_to_words(&i_digest[i * 4..(i + 1) * 4]) as u64;
    }
    i_words.reverse();

    // use blake3 crate to compute 2-to-1 digest of byte array
    let digest = blake3::hash(&i_digest);

    // prepare digest in desired blake3 word form so that assertion writing becomes easier
    let digest_bytes = digest.as_bytes();
    let mut digest_words = [0u64; STACK_TOP_SIZE >> 1];

    // convert each of four consecutive little endian bytes (of digest) to blake3 words
    for i in 0..(STACK_TOP_SIZE >> 1) {
        digest_words[i] = from_le_bytes_to_words(&digest_bytes[i * 4..(i + 1) * 4]) as u64;
    }

    // finally execute miden program on VM
    let inputs = ProgramInputs::new(&i_words, &[], Vec::new()).unwrap();
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();

    // first 8 elements of stack top holds blake3 digest, while remaining 8 elements
    // are zeroed
    let digest_on_stack = convert_to_stack(&digest_words);
    assert_eq!(digest_on_stack, last_state);
}

// HELPER FUNCTIONS
// ================================================================================================

fn compile(source: &str) -> Script {
    let assembler = assembly::Assembler::new();
    assembler.compile_script(source).unwrap()
}

/// Takes an array of u64 values and builds a stack, perserving their order and converting them to
/// field elements.
fn convert_to_stack(values: &[u64]) -> [Felt; STACK_TOP_SIZE] {
    let mut result = [Felt::ZERO; STACK_TOP_SIZE];
    for (&value, result) in values.iter().zip(result.iter_mut()) {
        *result = Felt::new(value);
    }
    result
}

/// Given a slice of four consecutive little endian bytes, interprets them as 32 -bit unsigned integer
fn from_le_bytes_to_words(le_bytes: &[u8]) -> u32 {
    ((le_bytes[3] as u32) << 24)
        | ((le_bytes[2] as u32) << 16)
        | ((le_bytes[1] as u32) << 8)
        | ((le_bytes[0] as u32) << 0)
}
