use test_utils::{
    crypto::{
        init_merkle_leaf, init_merkle_leaves, MerkleError, MerkleStore, MerkleTree, Mmr, NodeIndex,
        RpoDigest,
    },
    hash_elements, stack_to_ints, Felt, StarkField, WORD_SIZE, ZERO,
};

#[test]
fn test_ilog2() {
    let bit31 = "
    use.std::collections::mmr

    begin
        push.2147483648
        exec.mmr::ilog2_checked
    end
    ";

    let test = build_test!(bit31, &[]);
    test.expect_stack(&[31, 1 << 31]);

    let bit31_and_one = "
    use.std::collections::mmr

    begin
        push.2147483649
        exec.mmr::ilog2_checked
    end
    ";

    let test = build_test!(bit31_and_one, &[]);
    test.expect_stack(&[31, 1 << 31]);

    let bit16 = "
    use.std::collections::mmr

    begin
        push.65536
        exec.mmr::ilog2_checked
    end
    ";

    let test = build_test!(bit16, &[]);
    test.expect_stack(&[16, 1 << 16]);

    let all_bits_from_16 = "
    use.std::collections::mmr

    begin
        push.131071
        exec.mmr::ilog2_checked
    end
    ";

    let test = build_test!(all_bits_from_16, &[]);
    test.expect_stack(&[16, 1 << 16]);

    let one = "
    use.std::collections::mmr

    begin
        push.1
        exec.mmr::ilog2_checked
    end
    ";

    let test = build_test!(one, &[]);
    test.expect_stack(&[0, 1 << 0]);
}

#[test]
fn test_u32unchecked_trailing_ones() {
    let trailing_ones = "
    use.std::collections::mmr

    begin
        exec.mmr::u32unchecked_trailing_ones
    end
    ";

    build_test!(trailing_ones, &[0b0000]).expect_stack(&[0]);
    build_test!(trailing_ones, &[0b0001]).expect_stack(&[1]);
    build_test!(trailing_ones, &[0b0011]).expect_stack(&[2]);
    build_test!(trailing_ones, &[0b0111]).expect_stack(&[3]);
    build_test!(trailing_ones, &[0b1111]).expect_stack(&[4]);
    build_test!(trailing_ones, &[0b1111_1111]).expect_stack(&[8]);
    build_test!(trailing_ones, &[0b1111_1111_1111]).expect_stack(&[12]);
    build_test!(trailing_ones, &[0b1111_1111_1111_1111]).expect_stack(&[16]);
    build_test!(trailing_ones, &[0b1111_1111_1111_1111_1111_1111_1111_1111]).expect_stack(&[32]);

    build_test!(trailing_ones, &[0b0000]).expect_stack(&[0]);
    build_test!(trailing_ones, &[0b0010]).expect_stack(&[0]);
    build_test!(trailing_ones, &[0b0100]).expect_stack(&[0]);
    build_test!(trailing_ones, &[0b1000]).expect_stack(&[0]);
    build_test!(trailing_ones, &[0b1110]).expect_stack(&[0]);
    build_test!(trailing_ones, &[0b1111_0000]).expect_stack(&[0]);
    build_test!(trailing_ones, &[0b1111_1110]).expect_stack(&[0]);
    build_test!(trailing_ones, &[0b1111_1111_1111_1110]).expect_stack(&[0]);
    build_test!(trailing_ones, &[0b1111_1111_0000_0000]).expect_stack(&[0]);
    build_test!(trailing_ones, &[0b1111_1111_1111_1111_1111_1111_1111_1110]).expect_stack(&[0]);
    build_test!(trailing_ones, &[0b1000_0000_0000_0000_0000_0000_0000_0000]).expect_stack(&[0]);
}

#[test]
fn test_trailing_ones() {
    let trailing_ones = "
    use.std::collections::mmr

    begin
        exec.mmr::trailing_ones
    end
    ";

    build_test!(trailing_ones, &[2u64.pow(1)]).expect_stack(&[0]);
    build_test!(trailing_ones, &[2u64.pow(2)]).expect_stack(&[0]);
    build_test!(trailing_ones, &[2u64.pow(32)]).expect_stack(&[0]);
    build_test!(trailing_ones, &[2u64.pow(33)]).expect_stack(&[0]);
    build_test!(trailing_ones, &[2u64.pow(63)]).expect_stack(&[0]);

    build_test!(trailing_ones, &[2u64.pow(1) - 1]).expect_stack(&[1]);
    build_test!(trailing_ones, &[2u64.pow(2) - 1]).expect_stack(&[2]);
    build_test!(trailing_ones, &[2u64.pow(32) - 1]).expect_stack(&[32]);
    build_test!(trailing_ones, &[2u64.pow(33) - 1]).expect_stack(&[33]);
    build_test!(trailing_ones, &[2u64.pow(63) - 1]).expect_stack(&[63]);
}

#[test]
fn test_num_leaves_to_num_peaks() {
    let hash_size = "
    use.std::collections::mmr

    begin
      exec.mmr::num_leaves_to_num_peaks
    end
    ";

    build_test!(hash_size, &[0b0000]).expect_stack(&[0]);
    build_test!(hash_size, &[0b0001]).expect_stack(&[1]);
    build_test!(hash_size, &[0b0011]).expect_stack(&[2]);
    build_test!(hash_size, &[0b0011]).expect_stack(&[2]);
    build_test!(hash_size, &[0b1100]).expect_stack(&[2]);
    build_test!(hash_size, &[0b1000_0000_0000_0000]).expect_stack(&[1]);
    build_test!(hash_size, &[0b1010_1100_0011_1001]).expect_stack(&[8]);
    build_test!(hash_size, &[0b1111_1111_1111_1111]).expect_stack(&[16]);
    build_test!(hash_size, &[0b1111_1111_1111_1111_0000]).expect_stack(&[16]);
    build_test!(hash_size, &[0b0001_1111_1111_1111_1111]).expect_stack(&[17]);
}

#[test]
fn test_num_peaks_to_message_size() {
    let hash_size = "
    use.std::collections::mmr

    begin
      exec.mmr::num_peaks_to_message_size
    end
    ";

    // minimum size is 16
    build_test!(hash_size, &[1]).expect_stack(&[16]);
    build_test!(hash_size, &[2]).expect_stack(&[16]);
    build_test!(hash_size, &[3]).expect_stack(&[16]);
    build_test!(hash_size, &[4]).expect_stack(&[16]);
    build_test!(hash_size, &[7]).expect_stack(&[16]);
    build_test!(hash_size, &[11]).expect_stack(&[16]);
    build_test!(hash_size, &[16]).expect_stack(&[16]);

    // after that, size is round to the next even number
    build_test!(hash_size, &[17]).expect_stack(&[18]);
    build_test!(hash_size, &[18]).expect_stack(&[18]);
    build_test!(hash_size, &[19]).expect_stack(&[20]);
    build_test!(hash_size, &[20]).expect_stack(&[20]);
    build_test!(hash_size, &[21]).expect_stack(&[22]);
    build_test!(hash_size, &[22]).expect_stack(&[22]);
}

#[test]
fn test_mmr_get_single_peak() -> Result<(), MerkleError> {
    // This test uses a single merkle tree as the only MMR peak
    let leaves = &[1, 2, 3, 4];
    let merkle_tree = MerkleTree::new(init_merkle_leaves(leaves))?;
    let merkle_root = merkle_tree.root();
    let merkle_store = MerkleStore::from(&merkle_tree);
    let advice_stack: Vec<u64> = merkle_root.iter().map(StarkField::as_int).collect();

    for pos in 0..(leaves.len() as u64) {
        let source = format!(
            "use.std::collections::mmr

            begin
                push.{num_leaves} push.1000 mem_store # leaves count
                adv_push.4 push.1001 mem_storew dropw # MMR single peak

                push.1000 push.{pos} exec.mmr::get
            end",
            num_leaves = leaves.len(),
            pos = pos,
        );

        let test = build_test!(source, &[], advice_stack, merkle_store.clone());
        let leaf = merkle_store.get_node(merkle_root, NodeIndex::new(2, pos)?)?;

        // the stack should be first the leaf followed by the tree root
        let stack: Vec<u64> = leaf.iter().map(StarkField::as_int).rev().collect();
        test.expect_stack(&stack);
    }

    Ok(())
}

#[test]
fn test_mmr_get_two_peaks() -> Result<(), MerkleError> {
    // This test uses two merkle trees for the MMR, one with 8 elements, and one with 2
    let leaves1 = &[1, 2, 3, 4, 5, 6, 7, 8];
    let merkle_tree1 = MerkleTree::new(init_merkle_leaves(leaves1))?;
    let merkle_root1 = merkle_tree1.root();
    let leaves2 = &[9, 10];
    let merkle_tree2 = MerkleTree::new(init_merkle_leaves(leaves2))?;
    let merkle_root2 = merkle_tree2.root();
    let num_leaves = leaves1.len() + leaves2.len();

    let mut merkle_store = MerkleStore::new();
    merkle_store
        .extend(merkle_tree1.inner_nodes())
        .extend(merkle_tree2.inner_nodes());

    let advice_stack: Vec<u64> = merkle_root1
        .iter()
        .map(StarkField::as_int)
        .chain(merkle_root2.iter().map(StarkField::as_int))
        .collect();

    let examples = [
        // absolute_pos, leaf
        (0, merkle_store.get_node(merkle_root1, NodeIndex::new(3u8, 0u64)?)?),
        (1, merkle_store.get_node(merkle_root1, NodeIndex::new(3u8, 1u64)?)?),
        (2, merkle_store.get_node(merkle_root1, NodeIndex::new(3u8, 2u64)?)?),
        (3, merkle_store.get_node(merkle_root1, NodeIndex::new(3u8, 3u64)?)?),
        (7, merkle_store.get_node(merkle_root1, NodeIndex::new(3u8, 7u64)?)?),
        (8, merkle_store.get_node(merkle_root2, NodeIndex::new(1u8, 0u64)?)?),
        (9, merkle_store.get_node(merkle_root2, NodeIndex::new(1u8, 1u64)?)?),
    ];

    for (absolute_pos, leaf) in examples {
        let source = format!(
            "use.std::collections::mmr

            begin
                push.{num_leaves} push.1000 mem_store # leaves count
                adv_push.4 push.1001 mem_storew dropw # MMR first peak
                adv_push.4 push.1002 mem_storew dropw # MMR second peak

                push.1000 push.{pos} exec.mmr::get
            end",
            num_leaves = num_leaves,
            pos = absolute_pos,
        );

        let test = build_test!(source, &[], advice_stack, merkle_store.clone());

        // the stack should be first the leaf element followed by the tree root
        let stack: Vec<u64> = leaf.iter().map(StarkField::as_int).rev().collect();
        test.expect_stack(&stack);
    }

    Ok(())
}

#[test]
fn test_mmr_tree_with_one_element() -> Result<(), MerkleError> {
    // This test uses three merkle trees for the MMR, one with 8 elements, one with 2, and one with
    // a single leaf. The test is ensure the single leaf case is supported, the other two are used
    // for variaty
    let leaves1 = &[1, 2, 3, 4, 5, 6, 7, 8];
    let leaves2 = &[9, 10];
    let leaves3 = &[11];

    let merkle_tree1 = MerkleTree::new(init_merkle_leaves(leaves1))?;
    let merkle_tree2 = MerkleTree::new(init_merkle_leaves(leaves2))?;

    let merkle_root1 = merkle_tree1.root();
    let merkle_root2 = merkle_tree2.root();
    let merkle_root3 = init_merkle_leaves(leaves3)[0];

    let mut merkle_store = MerkleStore::new();
    merkle_store
        .extend(merkle_tree1.inner_nodes())
        .extend(merkle_tree2.inner_nodes());

    // In the case of a single leaf, the leaf is itself also the root
    let stack: Vec<u64> = merkle_root3.iter().map(StarkField::as_int).rev().collect();

    // Test case for single element MMR
    let advice_stack: Vec<u64> = merkle_root3.iter().map(StarkField::as_int).collect();
    let source = format!(
        "use.std::collections::mmr

        begin
            push.{num_leaves} push.1000 mem_store # leaves count
            adv_push.4 push.1001 mem_storew dropw # MMR first peak

            push.1000 push.{pos} exec.mmr::get
        end",
        num_leaves = leaves3.len(),
        pos = 0,
    );
    let test = build_test!(source, &[], advice_stack, merkle_store.clone());
    test.expect_stack(&stack);

    // Test case for the single element tree in a MMR with multiple trees
    let advice_stack: Vec<u64> = merkle_root1
        .iter()
        .map(StarkField::as_int)
        .chain(merkle_root2.iter().map(StarkField::as_int))
        .chain(merkle_root3.iter().map(StarkField::as_int))
        .collect();
    let num_leaves = leaves1.len() + leaves2.len() + leaves3.len();
    let source = format!(
        "use.std::collections::mmr

        begin
            push.{num_leaves} push.1000 mem_store # leaves count
            adv_push.4 push.1001 mem_storew dropw # MMR first peak
            adv_push.4 push.1002 mem_storew dropw # MMR second peak
            adv_push.4 push.1003 mem_storew dropw # MMR third peak

            push.1000 push.{pos} exec.mmr::get
        end",
        num_leaves = num_leaves,
        pos = num_leaves - 1,
    );
    let test = build_test!(source, &[], advice_stack, merkle_store.clone());
    test.expect_stack(&stack);

    Ok(())
}

#[test]
fn test_mmr_unpack() {
    let number_of_leaves: u64 = 0b10101; // 3 peaks, 21 leaves

    // The hash data is not the same as the peaks, it is padded to 16 elements
    let hash_data: [[Felt; 4]; 16] = [
        // 3 peaks. These hashes are invalid, we can't produce data for any of these peaks (only
        // for testing)
        [ZERO, ZERO, ZERO, Felt::new(1)],
        [ZERO, ZERO, ZERO, Felt::new(2)],
        [ZERO, ZERO, ZERO, Felt::new(3)],
        // Padding, the MMR is padded to a minimum length o 16
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
    ];
    let hash = hash_elements(&hash_data.concat());

    // Set up the VM stack with the MMR hash, and its target address
    let mut stack = stack_to_ints(&*hash);
    let mmr_ptr = 1000;
    stack.insert(0, mmr_ptr);

    // both the advice stack and merkle store start empty (data is available in
    // the map and pushed to the advice stack by the MASM code)
    let advice_stack = &[];
    let store = MerkleStore::new();

    let mut map_data: Vec<Felt> = Vec::with_capacity(hash_data.len() + 1);
    map_data.extend_from_slice(&[number_of_leaves.into(), ZERO, ZERO, ZERO]);
    map_data.extend_from_slice(&hash_data.as_slice().concat());

    let advice_map: &[([u8; 32], Vec<Felt>)] = &[
        // Under the MMR key is the number_of_leaves, followed by the MMR peaks, and any padding
        (hash.as_bytes(), map_data),
    ];

    let source = "
        use.std::collections::mmr
        begin exec.mmr::unpack end
    ";
    let test = build_test!(source, &stack, advice_stack, store, advice_map.iter().cloned());

    #[rustfmt::skip]
    let expect_memory = [
        number_of_leaves, 0, 0, 0, // MMR leaves (only one Felt is used)
        0, 0, 0, 1,                // first peak
        0, 0, 0, 2,                // second peak
        0, 0, 0, 3,                // third peak
    ];
    test.expect_stack(&[]);
    test.expect_stack_and_memory(&[], mmr_ptr, &expect_memory);
}

#[test]
fn test_mmr_unpack_invalid_hash() {
    // The hash data is not the same as the peaks, it is padded to 16 elements
    let mut hash_data: [[Felt; 4]; 16] = [
        // 3 peaks. These hashes are invalid, we can't produce data for any of these peaks (only
        // for testing)
        [ZERO, ZERO, ZERO, Felt::new(1)],
        [ZERO, ZERO, ZERO, Felt::new(2)],
        [ZERO, ZERO, ZERO, Felt::new(3)],
        // Padding, the MMR is padded to a minimum length o 16
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
        [ZERO, ZERO, ZERO, ZERO],
    ];
    let hash = hash_elements(&hash_data.concat());

    // Set up the VM stack with the MMR hash, and its target address
    let mut stack = stack_to_ints(&*hash);
    let mmr_ptr = 1000;
    stack.insert(0, mmr_ptr);

    // both the advice stack and merkle store start empty (data is available in
    // the map and pushed to the advice stack by the MASM code)
    let advice_stack = &[];
    let store = MerkleStore::new();

    // corrupt the data, this changes the hash and the commitment check must fail
    hash_data[0][0] = hash_data[0][0] + Felt::new(1);

    let mut map_data: Vec<Felt> = Vec::with_capacity(hash_data.len() + 1);
    map_data.extend_from_slice(&[Felt::new(0b10101), ZERO, ZERO, ZERO]); // 3 peaks, 21 leaves
    map_data.extend_from_slice(&hash_data.as_slice().concat());

    let advice_map: &[([u8; 32], Vec<Felt>)] = &[
        // Under the MMR key is the number_of_leaves, followed by the MMR peaks, and any padding
        (hash.as_bytes(), map_data),
    ];

    let source = "
        use.std::collections::mmr
        begin exec.mmr::unpack end
    ";
    let test = build_test!(source, &stack, advice_stack, store, advice_map.iter().cloned());

    assert!(test.execute().is_err());
}

/// Tests the case of an MMR with more than 16 peaks
#[test]
fn test_mmr_unpack_large_mmr() {
    let number_of_leaves: u64 = 0b11111111111111111; // 17 peaks

    // The hash data is not the same as the peaks, it is padded to 16 elements
    let hash_data: [[Felt; 4]; 18] = [
        // These hashes are invalid, we can't produce data for any of these peaks (only for
        // testing)
        [ZERO, ZERO, ZERO, Felt::new(1)],
        [ZERO, ZERO, ZERO, Felt::new(2)],
        [ZERO, ZERO, ZERO, Felt::new(3)],
        [ZERO, ZERO, ZERO, Felt::new(4)],
        [ZERO, ZERO, ZERO, Felt::new(5)],
        [ZERO, ZERO, ZERO, Felt::new(6)],
        [ZERO, ZERO, ZERO, Felt::new(7)],
        [ZERO, ZERO, ZERO, Felt::new(8)],
        [ZERO, ZERO, ZERO, Felt::new(9)],
        [ZERO, ZERO, ZERO, Felt::new(10)],
        [ZERO, ZERO, ZERO, Felt::new(11)],
        [ZERO, ZERO, ZERO, Felt::new(12)],
        [ZERO, ZERO, ZERO, Felt::new(13)],
        [ZERO, ZERO, ZERO, Felt::new(14)],
        [ZERO, ZERO, ZERO, Felt::new(15)],
        [ZERO, ZERO, ZERO, Felt::new(16)],
        // Padding, peaks greater than 16 are padded to an even number
        [ZERO, ZERO, ZERO, Felt::new(17)],
        [ZERO, ZERO, ZERO, ZERO],
    ];
    let hash = hash_elements(&hash_data.concat());

    // Set up the VM stack with the MMR hash, and its target address
    let mut stack = stack_to_ints(&*hash);
    let mmr_ptr = 1000;
    stack.insert(0, mmr_ptr);

    // both the advice stack and merkle store start empty (data is available in
    // the map and pushed to the advice stack by the MASM code)
    let advice_stack = &[];
    let store = MerkleStore::new();

    let mut map_data: Vec<Felt> = Vec::with_capacity(hash_data.len() + 1);
    map_data.extend_from_slice(&[number_of_leaves.into(), ZERO, ZERO, ZERO]);
    map_data.extend_from_slice(&hash_data.as_slice().concat());

    let advice_map: &[([u8; 32], Vec<Felt>)] = &[
        // Under the MMR key is the number_of_leaves, followed by the MMR peaks, and any padding
        (hash.as_bytes(), map_data),
    ];

    let source = "
        use.std::collections::mmr
        begin exec.mmr::unpack end
    ";
    let test = build_test!(source, &stack, advice_stack, store, advice_map.iter().cloned());

    #[rustfmt::skip]
    let expect_memory = [
        number_of_leaves, 0, 0, 0, // MMR leaves (only one Felt is used)
        0, 0, 0, 1,                // peaks
        0, 0, 0, 2,
        0, 0, 0, 3,
        0, 0, 0, 4,
        0, 0, 0, 5,
        0, 0, 0, 6,
        0, 0, 0, 7,
        0, 0, 0, 8,
        0, 0, 0, 9,
        0, 0, 0, 10,
        0, 0, 0, 11,
        0, 0, 0, 12,
        0, 0, 0, 13,
        0, 0, 0, 14,
        0, 0, 0, 15,
        0, 0, 0, 16,
        0, 0, 0, 17,
    ];
    test.expect_stack(&[]);
    test.expect_stack_and_memory(&[], mmr_ptr, &expect_memory);
}

#[test]
fn test_mmr_pack_roundtrip() {
    let mut mmr = Mmr::new();
    mmr.add(init_merkle_leaf(1));
    mmr.add(init_merkle_leaf(2));
    mmr.add(init_merkle_leaf(3));

    let accumulator = mmr.accumulator();
    let hash = accumulator.hash_peaks();

    // Set up the VM stack with the MMR hash, and its target address
    let mut stack = stack_to_ints(&hash);
    let mmr_ptr = 1000;
    stack.insert(0, mmr_ptr); // first value is used by unpack, to load data to memory
    stack.insert(0, mmr_ptr); // second is used by pack, to load data from memory

    // both the advice stack and merkle store start empty (data is available in
    // the map and pushed to the advice stack by the MASM code)
    let advice_stack = &[];
    let store = MerkleStore::new();

    let mut hash_data = accumulator.peaks.clone();
    hash_data.resize(16, [ZERO; WORD_SIZE]);
    let mut map_data: Vec<Felt> = Vec::with_capacity(hash_data.len() + 1);
    map_data.extend_from_slice(&[
        Felt::new(accumulator.num_leaves.try_into().unwrap()),
        ZERO,
        ZERO,
        ZERO,
    ]);
    map_data.extend_from_slice(&hash_data.as_slice().concat());

    let advice_map: &[([u8; 32], Vec<Felt>)] = &[
        // Under the MMR key is the number_of_leaves, followed by the MMR peaks, and any padding
        (RpoDigest::new(hash).as_bytes(), map_data),
    ];

    let source = "
        use.std::collections::mmr
        begin
            exec.mmr::unpack
            exec.mmr::pack
        end
    ";
    let test = build_test!(source, &stack, advice_stack, store, advice_map.iter().cloned());
    let expected_stack: Vec<u64> = hash.iter().rev().map(|e| e.as_int()).collect();

    let mut expect_memory: Vec<u64> = Vec::new();

    // first the number of leaves
    expect_memory.extend_from_slice(&[accumulator.num_leaves as u64, 0, 0, 0]);
    // followed by the peaks
    expect_memory.extend(accumulator.peaks.iter().flatten().map(|v| v.as_int()));
    // followed by padding data
    let size = 4 + 16 * 4;
    expect_memory.resize(size, 0);

    test.expect_stack_and_memory(&expected_stack, 1000, &expect_memory);
}

#[test]
fn test_mmr_pack() {
    let source = "
        use.std::collections::mmr

        begin
            push.3.1000 mem_store  # num_leaves, 2 peaks
            push.1.1001 mem_store  # peak1
            push.2.1002 mem_store  # peak2

            push.1000 exec.mmr::pack
        end
    ";

    let mut hash_data: Vec<Felt> = Vec::new();

    #[rustfmt::skip]
    hash_data.extend_from_slice( &[
        Felt::new(1), ZERO, ZERO, ZERO, // peak1
        Felt::new(2), ZERO, ZERO, ZERO, // peak2
    ]);
    hash_data.resize(16 * 4, ZERO); // padding data

    let hash = hash_elements(&hash_data);
    let hash_u8 = hash.as_bytes();

    let mut expect_data: Vec<Felt> = Vec::new();
    expect_data.extend_from_slice(&[Felt::new(3), ZERO, ZERO, ZERO]); // num_leaves
    expect_data.extend_from_slice(&hash_data);

    let process = build_test!(source).execute_process().unwrap();

    let advice_data = process.advice_provider.map.get(&hash_u8).unwrap();
    assert_eq!(stack_to_ints(advice_data), stack_to_ints(&expect_data));
}

#[test]
fn test_mmr_add_single() {
    let mmr_ptr = 1000;
    let source = format!(
        "
        use.std::collections::mmr

        begin
            push.{mmr_ptr} # the address of the mmr
            push.1.2.3.4   # the new peak
            exec.mmr::add  # add the element
        end
    "
    );

    // when there is a single element, there is nothing to merge with, so the data is just in the
    // MMR
    #[rustfmt::skip]
    let expect_data = &[
        1, 0, 0, 0, // num_leaves
        1, 2, 3, 4, // peak
    ];
    build_test!(&source).expect_stack_and_memory(&[], mmr_ptr, expect_data);
}

#[test]
fn test_mmr_two() {
    let mmr_ptr = 1000;
    let source = format!(
        "
        use.std::collections::mmr

        begin
            push.{mmr_ptr} # first peak
            push.1.2.3.4
            exec.mmr::add

            push.{mmr_ptr} # second peak
            push.5.6.7.8
            exec.mmr::add
        end
    "
    );

    let mut mmr = Mmr::new();
    mmr.add([Felt::new(1), Felt::new(2), Felt::new(3), Felt::new(4)]);
    mmr.add([Felt::new(5), Felt::new(6), Felt::new(7), Felt::new(8)]);

    let accumulator = mmr.accumulator();
    let peak = accumulator.peaks[0];

    let num_leaves = accumulator.num_leaves.try_into().unwrap();
    let mut expected_memory = vec![num_leaves, 0, 0, 0];
    expected_memory.extend(peak.iter().map(|v| v.as_int()));

    build_test!(&source).expect_stack_and_memory(&[], mmr_ptr, &expected_memory);
}

#[test]
fn test_mmr_large() {
    let mmr_ptr = 1000;
    let source = format!(
        "
        use.std::collections::mmr

        begin
            push.{mmr_ptr}.0.0.0.1 exec.mmr::add
            push.{mmr_ptr}.0.0.0.2 exec.mmr::add
            push.{mmr_ptr}.0.0.0.3 exec.mmr::add
            push.{mmr_ptr}.0.0.0.4 exec.mmr::add
            push.{mmr_ptr}.0.0.0.5 exec.mmr::add
            push.{mmr_ptr}.0.0.0.6 exec.mmr::add
            push.{mmr_ptr}.0.0.0.7 exec.mmr::add

            push.{mmr_ptr} exec.mmr::pack
        end
    "
    );

    let mut mmr = Mmr::new();
    mmr.add([Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(1)]);
    mmr.add([Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(2)]);
    mmr.add([Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(3)]);
    mmr.add([Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(4)]);
    mmr.add([Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(5)]);
    mmr.add([Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(6)]);
    mmr.add([Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(7)]);

    let accumulator = mmr.accumulator();

    let num_leaves = accumulator.num_leaves.try_into().unwrap();
    let mut expected_memory = vec![num_leaves, 0, 0, 0];
    expected_memory.extend(accumulator.peaks.iter().flatten().map(|v| v.as_int()));

    let expect_stack: Vec<u64> =
        accumulator.hash_peaks().iter().rev().map(|v| v.as_int()).collect();
    build_test!(&source).expect_stack_and_memory(&expect_stack, mmr_ptr, &expected_memory);
}

#[test]
fn test_mmr_large_add_roundtrip() {
    let mmr_ptr = 1000;

    let mut mmr: Mmr = Mmr::from([
        [Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(1)],
        [Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(2)],
        [Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(3)],
        [Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(4)],
        [Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(5)],
        [Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(6)],
        [Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(7)],
    ]);

    let old_accumulator = mmr.accumulator();
    let hash = old_accumulator.hash_peaks();

    // Set up the VM stack with the MMR hash, and its target address
    let mut stack = stack_to_ints(&hash);
    stack.insert(0, mmr_ptr);

    // both the advice stack and merkle store start empty (data is available in
    // the map and pushed to the advice stack by the MASM code)
    let advice_stack = &[];
    let store = MerkleStore::new();

    let mut hash_data = old_accumulator.peaks.clone();
    hash_data.resize(16, [ZERO, ZERO, ZERO, ZERO]);

    let mut map_data: Vec<Felt> = Vec::with_capacity(hash_data.len() + 1);
    let num_leaves: u64 = old_accumulator.num_leaves as u64;
    map_data.extend_from_slice(&[Felt::from(num_leaves), ZERO, ZERO, ZERO]);
    map_data.extend_from_slice(&hash_data.as_slice().concat());

    let advice_map: &[([u8; 32], Vec<Felt>)] = &[
        // Under the MMR key is the number_of_leaves, followed by the MMR peaks, and any padding
        (RpoDigest::new(hash).as_bytes(), map_data),
    ];

    let source = format!(
        "
        use.std::collections::mmr

        begin
            exec.mmr::unpack
            push.{mmr_ptr}.0.0.0.8 exec.mmr::add
            push.{mmr_ptr} exec.mmr::pack
        end
    "
    );

    mmr.add([ZERO, ZERO, ZERO, Felt::new(8)]);

    let new_accumulator = mmr.accumulator();
    let num_leaves = new_accumulator.num_leaves.try_into().unwrap();
    let mut expected_memory = vec![num_leaves, 0, 0, 0];
    let mut new_peaks = new_accumulator.peaks.clone();
    // make sure the old peaks are zeroed
    new_peaks.resize(16, [ZERO, ZERO, ZERO, ZERO]);
    expected_memory.extend(new_peaks.iter().flatten().map(|v| v.as_int()));

    let expect_stack: Vec<u64> =
        new_accumulator.hash_peaks().iter().rev().map(|v| v.as_int()).collect();

    let test = build_test!(source, &stack, advice_stack, store, advice_map.iter().cloned());
    test.expect_stack_and_memory(&expect_stack, mmr_ptr, &expected_memory);
}