use processor::ExecutionError;
use test_utils::{build_expected_hash, build_expected_perm, expect_exec_error};

#[test]
fn test_invalid_end_addr() {
    // end_addr can not be smaller than start_addr
    let empty_range = "
    use.std::crypto::hashes::native

    begin
        push.0999 # end address
        push.1000 # start address

        exec.native::hash_memory
    end
    ";
    let test = build_test!(empty_range, &[]);
    expect_exec_error!(
        test,
        ExecutionError::FailedAssertion {
            clk: 18.into(),
            err_code: 0,
            err_msg: None,
        }
    );

    // address range can not contain zero elements
    let empty_range = "
    use.std::crypto::hashes::native

    begin
        push.1000 # end address
        push.1000 # start address

        exec.native::hash_memory
    end
    ";
    let test = build_test!(empty_range, &[]);
    expect_exec_error!(
        test,
        ExecutionError::FailedAssertion {
            clk: 18.into(),
            err_code: 0,
            err_msg: None,
        }
    );
}

#[test]
fn test_hash_empty() {
    // computes the hash for 8 consecutive zeros using mem_stream directly
    let two_zeros_mem_stream = "
    begin
        # mem_stream state
        push.1000 padw padw padw
        mem_stream hperm

        # drop everything except the hash
        dropw swapw.3 dropw dropw dropw
    end
    ";

    #[rustfmt::skip]
    let zero_hash: Vec<u64> = build_expected_hash(&[
        0, 0, 0, 0,
        0, 0, 0, 0,
    ]).into_iter().map(|e| e.as_int()).collect();
    build_test!(two_zeros_mem_stream, &[]).expect_stack(&zero_hash);

    // checks the hash compute from 8 zero elements is the same when using hash_memory
    let two_zeros = "
    use.std::crypto::hashes::native

    begin
        push.1002 # end address
        push.1000 # start address

        exec.native::hash_memory
        swapw dropw
    end
    ";

    build_test!(two_zeros, &[]).expect_stack(&zero_hash);
}

#[test]
fn test_single_iteration() {
    // computes the hash of 1 using mem_stream
    let one_memstream = "
    begin
        # insert 1 to memory
        push.1.1000 mem_store

        # mem_stream state
        push.1000 padw padw padw
        mem_stream hperm

        # drop everything except the hash
        dropw swapw.3 dropw dropw dropw
    end
    ";

    #[rustfmt::skip]
    let one_hash: Vec<u64> = build_expected_hash(&[
        1, 0, 0, 0,
        0, 0, 0, 0,
    ]).into_iter().map(|e| e.as_int()).collect();
    build_test!(one_memstream, &[]).expect_stack(&one_hash);

    // checks the hash of 1 is the same when using hash_memory
    // Note: This is testing the hashing of two words, so no padding is added
    // here
    let one_element = "
    use.std::crypto::hashes::native

    begin
        # insert 1 to memory
        push.1.1000 mem_store

        push.1002 # end address
        push.1000 # start address

        exec.native::hash_memory
        swapw dropw
    end
    ";

    build_test!(one_element, &[]).expect_stack(&one_hash);
}

#[test]
fn test_hash_one_word() {
    // computes the hash of a single 1, the procedure adds padding as required

    // This slice must not have the second word, that will be padded by the hasher with the correct
    // value
    #[rustfmt::skip]
    let one_hash: Vec<u64> = build_expected_hash(&[
        1, 0, 0, 0,
    ]).into_iter().map(|e| e.as_int()).collect();

    // checks the hash of 1 is the same when using hash_memory
    let one_element = "
    use.std::crypto::hashes::native

    begin
        push.1.1000 mem_store # push data to memory

        push.1001 # end address
        push.1000 # start address

        exec.native::hash_memory
        swapw dropw
    end
    ";

    build_test!(one_element, &[]).expect_stack(&one_hash);
}

#[test]
fn test_hash_even_words() {
    // checks the hash of two words
    let even_words = "
    use.std::crypto::hashes::native

    begin
        push.1.0.0.0.1000 mem_storew dropw
        push.0.1.0.0.1001 mem_storew dropw

        push.1002 # end address
        push.1000 # start address

        exec.native::hash_memory
        swapw dropw
    end
    ";

    #[rustfmt::skip]
    let even_hash: Vec<u64> = build_expected_hash(&[
        1, 0, 0, 0,
        0, 1, 0, 0,
    ]).into_iter().map(|e| e.as_int()).collect();
    build_test!(even_words, &[]).expect_stack(&even_hash);
}

#[test]
fn test_hash_odd_words() {
    // checks the hash of three words
    let odd_words = "
    use.std::crypto::hashes::native

    begin
        push.1.0.0.0.1000 mem_storew dropw
        push.0.1.0.0.1001 mem_storew dropw
        push.0.0.1.0.1002 mem_storew dropw

        push.1003 # end address
        push.1000 # start address

        exec.native::hash_memory
        swapw dropw
    end
    ";

    #[rustfmt::skip]
    let odd_hash: Vec<u64> = build_expected_hash(&[
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
    ]).into_iter().map(|e| e.as_int()).collect();
    build_test!(odd_words, &[]).expect_stack(&odd_hash);
}

#[test]
fn test_hash_memory_even() {
    let even_words = "
    use.std::crypto::hashes::native
    use.std::sys

    begin
        push.1.0.0.0.1000 mem_storew dropw
        push.0.1.0.0.1001 mem_storew dropw

        push.1002      # end address
        push.1000      # start address
        padw padw padw # hasher state
        exec.native::hash_memory_even

        exec.sys::truncate_stack
    end
    ";

    #[rustfmt::skip]
    let mut even_hash: Vec<u64> = build_expected_perm(&[
        0, 0, 0, 0, // capacity, no padding required
        1, 0, 0, 0, // first word of the rate
        0, 1, 0, 0, // second word of the rate
    ]).into_iter().map(|e| e.as_int()).collect();

    // start and end addr
    even_hash.push(1002);
    even_hash.push(1002);

    build_test!(even_words, &[]).expect_stack(&even_hash);
}

#[test]
fn test_state_to_digest() {
    let even_words = "
    use.std::crypto::hashes::native

    begin
        push.1.0.0.0.1000 mem_storew dropw
        push.0.1.0.0.1001 mem_storew dropw
        push.0.0.1.0.1002 mem_storew dropw
        push.0.0.0.1.1003 mem_storew dropw

        push.1004      # end address
        push.1000      # start address
        padw padw padw # hasher state
        exec.native::hash_memory_even

        exec.native::state_to_digest
        swapdw dropw dropw
    end
    ";

    #[rustfmt::skip]
    let mut even_hash: Vec<u64> = build_expected_hash(&[
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1,
    ]).into_iter().map(|e| e.as_int()).collect();

    // start and end addr
    even_hash.push(1004);
    even_hash.push(1004);

    build_test!(even_words, &[]).expect_stack(&even_hash);
}
