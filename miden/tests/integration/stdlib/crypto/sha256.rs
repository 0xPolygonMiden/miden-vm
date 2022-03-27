use super::{build_test, Felt, MIN_STACK_DEPTH};
use sha2::{Digest, Sha256};
use vm_core::utils::IntoBytes;

#[test]
fn sha256_2_to_1_hash() {
    let source = "
    use.std::crypto::hashes::sha256

    begin
        exec.sha256::hash
    end";

    // prepare random input byte array
    let i_digest_0: [u8; 32] = rand_utils::rand_array::<Felt, 4>().into_bytes();
    let i_digest_1: [u8; 32] = rand_utils::rand_array::<Felt, 4>().into_bytes();

    // two digests concatenated to form input to sha256 2-to-1 hash function
    let mut i_digest = [0u8; 64];
    i_digest[..32].copy_from_slice(&i_digest_0);
    i_digest[32..].copy_from_slice(&i_digest_1);

    // allocate space on stack so that bytes can be converted to sha256 words
    let mut i_words = [0u64; MIN_STACK_DEPTH];

    // convert each of four consecutive big endian bytes (of input) to sha256 words
    for i in 0..MIN_STACK_DEPTH {
        i_words[i] = from_be_bytes_to_words(&i_digest[i * 4..(i + 1) * 4]) as u64;
    }
    i_words.reverse();

    let mut hasher = Sha256::new();
    hasher.update(&i_digest);
    let digest = hasher.finalize();

    // prepare digest in desired sha256 word form so that assertion writing becomes easier
    let mut digest_words = [0u64; MIN_STACK_DEPTH >> 1];
    // convert each of four consecutive big endian bytes (of digest) to sha256 words
    for i in 0..(MIN_STACK_DEPTH >> 1) {
        digest_words[i] = from_be_bytes_to_words(&digest[i * 4..(i + 1) * 4]) as u64;
    }

    // finally execute miden program on VM
    let test = build_test!(source, &i_words);
    // first 8 elements of stack top holds sha256 digest,
    // while remaining 8 elements are zeroed
    test.expect_stack(&digest_words);
}

// HELPER FUNCTIONS
// ================================================================================================

/// Takes four consecutive big endian bytes and interprets them as a SHA256 word
fn from_be_bytes_to_words(be_bytes: &[u8]) -> u32 {
    ((be_bytes[0] as u32) << 24)
        | ((be_bytes[1] as u32) << 16)
        | ((be_bytes[2] as u32) << 8)
        | ((be_bytes[3] as u32) << 0)
}
