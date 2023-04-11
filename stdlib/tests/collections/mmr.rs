use test_utils::{
    crypto::{init_merkle_leaves, MerkleError, MerkleStore, NodeIndex},
    hash_elements, stack_to_ints, Felt, StarkField, ZERO,
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
fn test_mmr_get_single_peak() -> Result<(), MerkleError> {
    // This test uses a single merkle tree as the only MMR peak
    let leaves = &[1, 2, 3, 4];
    let mut merkle_store = MerkleStore::new();
    let merkle_root = merkle_store.add_merkle_tree(init_merkle_leaves(leaves))?;
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
    let leaves2 = &[9, 10];
    let num_leaves = leaves1.len() + leaves2.len();

    let mut merkle_store = MerkleStore::new();
    let merkle_root1 = merkle_store.add_merkle_tree(init_merkle_leaves(leaves1))?;
    let merkle_root2 = merkle_store.add_merkle_tree(init_merkle_leaves(leaves2))?;

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

    let mut merkle_store = MerkleStore::new();
    let merkle_root1 = merkle_store.add_merkle_tree(init_merkle_leaves(leaves1))?;
    let merkle_root2 = merkle_store.add_merkle_tree(init_merkle_leaves(leaves2))?;
    let merkle_root3 = init_merkle_leaves(leaves3)[0];

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
    map_data.push(number_of_leaves.into());
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
    map_data.push(Felt::new(0b10101)); // 3 peaks, 21 leaves
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
    map_data.push(number_of_leaves.into());
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
