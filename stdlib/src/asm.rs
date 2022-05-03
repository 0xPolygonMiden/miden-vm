//! This module is automatically generated during build time and should not be modified manually.

/// An array of modules defined in Miden standard library.
///
/// Entries in the array are tuples containing module namespace and module source code.
#[rustfmt::skip]
pub const MODULES: [(&str, &str); 5] = [
// ----- std::crypto::hashes::blake3 --------------------------------------------------------------
("std::crypto::hashes::blake3", "proc.from_mem_to_stack.1
    storew.local.0
    drop
    drop
    drop
    pushw.mem # = d #

    pushw.local.0
    drop
    drop
    swap
    drop
    pushw.mem # = c #

    pushw.local.0
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem # = b #

    pushw.local.0
    repeat.3
        swap
        drop
    end
    pushw.mem # = a #
end

# initial hash state of blake3 when computing 2-to-1 hash i.e. two blake3 digests are being merged into single digest of 32 -bytes #
# see https://github.com/itzmeanjan/blake3/blob/f07d32ec10cbc8a10663b7e6539e0b1dab3e453b/include/blake3.hpp#L1709-L1713 #
proc.initialize_hash_state.1
    popw.local.0

    # blake3 initial values #
    # see https://github.com/BLAKE3-team/BLAKE3/blob/da4c792d8094f35c05c41c9aeb5dfe4aa67ca1ac/reference_impl/reference_impl.rs#L36-L38 #
    push.0xA54FF53A.0x3C6EF372.0xBB67AE85.0x6A09E667

    pushw.local.0
    repeat.3
        swap
        drop
    end

    popw.mem

    push.0x5BE0CD19.0x1F83D9AB.0x9B05688C.0x510E527F

    pushw.local.0
    drop
    repeat.2
        swap
        drop
    end

    popw.mem

    push.0xA54FF53A.0x3C6EF372.0xBB67AE85.0x6A09E667

    pushw.local.0
    drop
    drop
    swap
    drop

    popw.mem

    # blake3 hash constants https://github.com/itzmeanjan/blake3/blob/1c58f6a343baee52ba1fe7fc98bfb280b6d567da/include/blake3_consts.hpp#L16-L20 #
    push.11.64.0.0

    pushw.local.0
    drop
    drop
    drop

    popw.mem
end

# permutes ordered message words, kept on stack top ( = sixteen 32 -bit BLAKE3 words ) #
# such that next round of mixing can be applied #
# after completion of permutation, message words are transferred back to stack top, in ordered form #
# see https://github.com/itzmeanjan/blake3/blob/f07d32ec10cbc8a10663b7e6539e0b1dab3e453b/include/blake3.hpp#L1623-L1639 #
proc.blake3_msg_words_permute.3
    movup.2
    movup.6
    swap
    movup.4
    movup.10
    swap
    movup.3
    movup.3

    push.env.locaddr.0
    popw.mem

    movup.4
    movup.3
    movup.9
    swap
    movup.3
    movup.3

    push.env.locaddr.1
    popw.mem

    movup.4
    swap
    movup.5
    movdn.2

    push.env.locaddr.2
    popw.mem

    movdn.3

    # bring message words back to stack, from local memory #
    push.env.locaddr.2
    pushw.mem
    push.env.locaddr.1
    pushw.mem
    push.env.locaddr.0
    pushw.mem
end

# this function computes final 32 -bytes digest from first 8 blake3 words of hash state, #
# which is here represented as stack top of Miden VM i.e. top 8 elements of stack #
# ( read top two words) are to be manipulated in this function so that after completion of #
# execution of this function, first 8 elements of stack should hold desired blake3 hash #
# #
# see https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#L116-L119 #
# you'll notice I've skipped executing second statement in loop body of above hyperlinked implementation, #
# that's because it doesn't dictate what output of 2-to-1 hash will be #
proc.prepare_digest.0
    dup.8
    u32xor

    dup.9
    movup.2
    u32xor
    swap

    dup.10
    movup.3
    u32xor
    movdn.2

    dup.11
    movup.4
    u32xor
    movdn.3

    dup.12
    movup.5
    u32xor
    movdn.4

    dup.13
    movup.6
    u32xor
    movdn.5

    dup.14
    movup.7
    u32xor
    movdn.6

    dup.15
    movup.8
    u32xor
    movdn.7
end

# column-wise mixing #
# see https://github.com/BLAKE3-team/BLAKE3/blob/da4c792d8094f35c05c41c9aeb5dfe4aa67ca1ac/reference_impl/reference_impl.rs#L55-L59 #
proc.columnar_mixing.1
    pushw.mem
    popw.local.0

    pushw.local.0
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem # = b #

    pushw.local.0
    repeat.3
        swap
        drop
    end
    pushw.mem # = a #

    dup.4
    movup.9
    u32add.unsafe
    drop
    u32add.unsafe
    drop

    dup.1
    dup.6
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.2
    drop

    dup.2
    dup.7
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.3
    drop

    dup.3
    dup.8
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.4
    drop

    # --- #

    pushw.local.0
    drop
    drop
    drop
    pushw.mem # = d #

    dupw.1    # copy a #

    movup.4
    u32xor
    u32rotr.16
    
    swap
    movup.4
    u32xor
    u32rotr.16
    swap
    
    movup.2
    movup.4
    u32xor
    u32rotr.16
    movdn.2
    
    movup.3
    movup.4
    u32xor
    u32rotr.16
    movdn.3

    # --- #

    pushw.local.0
    drop
    drop
    swap
    drop
    pushw.mem # = c #

    dupw.1    # copy d #

    movup.4
    u32add.unsafe
    drop

    swap
    movup.4
    u32add.unsafe
    drop
    swap
    
    movup.2
    movup.4
    u32add.unsafe
    drop
    movdn.2
    
    movup.3
    movup.4
    u32add.unsafe
    drop
    movdn.3

    # --- #

    movupw.3
    dupw.1
    
    movup.4
    u32xor
    u32rotr.12
    
    swap
    movup.4
    u32xor
    u32rotr.12
    swap
    
    movup.2
    movup.4
    u32xor
    u32rotr.12
    movdn.2
    
    movup.3
    movup.4
    u32xor
    u32rotr.12
    movdn.3
    
    movdnw.3

    # --- #

    pushw.local.0
    drop
    drop
    swap
    drop
    popw.mem # = c #

    pushw.local.0
    drop
    drop
    drop
    popw.mem # = d #

    # --- #

    dup.4
    movup.9
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    
    dup.1
    dup.6
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.2
    drop
    
    dup.2
    dup.7
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.3
    drop
    
    dup.3
    dup.8
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.4
    drop

    # --- #

    pushw.local.0
    drop
    drop
    drop
    pushw.mem # = d #

    dupw.1        # copy a #
    
    movup.4
    u32xor
    u32rotr.8
    
    swap
    movup.4
    u32xor
    u32rotr.8
    swap
    
    movup.2
    movup.4
    u32xor
    u32rotr.8
    movdn.2
    
    movup.3
    movup.4
    u32xor
    u32rotr.8
    movdn.3

    # --- #

    pushw.local.0
    drop
    drop
    swap
    drop
    pushw.mem # = c #

    dupw.1        # copy d #
    
    movup.4
    u32add.unsafe
    drop
    
    swap
    movup.4
    u32add.unsafe
    drop
    swap
    
    movup.2
    movup.4
    u32add.unsafe
    drop
    movdn.2
    
    movup.3
    movup.4
    u32add.unsafe
    drop
    movdn.3

    # --- #

    movupw.3
    dupw.1

    movup.4
    u32xor
    u32rotr.7

    swap
    movup.4
    u32xor
    u32rotr.7
    swap

    movup.2
    movup.4
    u32xor
    u32rotr.7
    movdn.2

    movup.3
    movup.4
    u32xor
    u32rotr.7
    movdn.3

    movdnw.3

    # --- #

    pushw.local.0
    drop
    drop
    swap
    drop
    popw.mem # = c #

    pushw.local.0
    drop
    drop
    drop
    popw.mem # = d #

    pushw.local.0
    repeat.3
        swap
        drop
    end
    popw.mem # = a #

    pushw.local.0
    drop
    repeat.2
        swap
        drop
    end
    popw.mem # = b #
end

# diagonal-wise mixing #
# see https://github.com/BLAKE3-team/BLAKE3/blob/da4c792d8094f35c05c41c9aeb5dfe4aa67ca1ac/reference_impl/reference_impl.rs#L60-L64 #
proc.diagonal_mixing.1
    pushw.mem
    popw.local.0

    pushw.local.0
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem # = b #

    pushw.local.0
    repeat.3
        swap
        drop
    end
    pushw.mem # = a #

    dup.5
    movup.9
    u32add.unsafe
    drop
    u32add.unsafe
    drop

    dup.1
    dup.7
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.2
    drop

    dup.2
    dup.8
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.3
    drop

    dup.3
    dup.5
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.4
    drop

    # --- #

    pushw.local.0
    drop
    drop
    drop
    pushw.mem # = d #

    dup.3
    dup.5
    u32xor
    u32rotr.16
    swap.4
    drop

    dup.5
    u32xor
    u32rotr.16

    swap
    dup.6
    u32xor
    u32rotr.16
    swap

    dup.2
    dup.8
    u32xor
    u32rotr.16
    swap.3
    drop

    # --- #

    pushw.local.0
    drop
    drop
    swap
    drop
    pushw.mem # = c #

    dup.2
    dup.8
    u32add.unsafe
    drop
    swap.3
    drop

    dup.3
    dup.5
    u32add.unsafe
    drop
    swap.4
    drop

    dup.5
    u32add.unsafe
    drop

    swap
    dup.6
    u32add.unsafe
    drop
    swap

    # --- #

    movupw.3

    swap
    dup.6
    u32xor
    u32rotr.12
    swap

    dup.2
    dup.8
    u32xor
    u32rotr.12
    swap.3
    drop

    dup.3
    dup.5
    u32xor
    u32rotr.12
    swap.4
    drop

    dup.5
    u32xor
    u32rotr.12

    movdnw.3

    # --- #

    pushw.local.0
    drop
    drop
    swap
    drop
    popw.mem # = c #

    pushw.local.0
    drop
    drop
    drop
    popw.mem # = d #

    # --- #

    dup.5
    movup.9
    u32add.unsafe
    drop
    u32add.unsafe
    drop

    dup.1
    dup.7
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.2
    drop

    dup.2
    dup.8
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.3
    drop

    dup.3
    dup.5
    movup.10
    u32add.unsafe
    drop
    u32add.unsafe
    drop
    swap.4
    drop

    # --- #

    pushw.local.0
    drop
    drop
    drop
    pushw.mem # = d #

    dup.3
    dup.5
    u32xor
    u32rotr.8
    swap.4
    drop

    dup.5
    u32xor
    u32rotr.8

    swap
    dup.6
    u32xor
    u32rotr.8
    swap

    dup.2
    dup.8
    u32xor
    u32rotr.8
    swap.3
    drop

    # --- #

    pushw.local.0
    drop
    drop
    swap
    drop
    pushw.mem # = c #

    dup.2
    dup.8
    u32add.unsafe
    drop
    swap.3
    drop

    dup.3
    dup.5
    u32add.unsafe
    drop
    swap.4
    drop

    dup.5
    u32add.unsafe
    drop

    swap
    dup.6
    u32add.unsafe
    drop
    swap

    # --- #

    movupw.3

    swap
    dup.6
    u32xor
    u32rotr.7
    swap

    dup.2
    dup.8
    u32xor
    u32rotr.7
    swap.3
    drop

    dup.3
    dup.5
    u32xor
    u32rotr.7
    swap.4
    drop

    dup.5
    u32xor
    u32rotr.7

    movdnw.3

    # --- #

    pushw.local.0
    drop
    drop
    swap
    drop
    popw.mem # = c #

    pushw.local.0
    drop
    drop
    drop
    popw.mem # = d #

    pushw.local.0
    repeat.3
        swap
        drop
    end
    popw.mem # = a #

    pushw.local.0
    drop
    repeat.2
        swap
        drop
    end
    popw.mem # = b #
end

proc.prepare_columnar_mixing_in_words.0
    dupw.1
    dupw.1

    movup.6
    movup.5
    movup.4
    movup.3
end

proc.prepare_diagonal_mixing_in_words.0
    dupw.3
    dupw.3

    movup.6
    movup.5
    movup.4
    movup.3
end

# see https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#L54-L65 #
proc.round.1
    pushw.mem
    popw.local.0

    # --- columnar mixing --- #
    # equivalent to https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#L55-L59 #
    exec.prepare_columnar_mixing_in_words
    push.env.locaddr.0
    exec.columnar_mixing

    # --- diagonal mixing --- #
    # equivalent to https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#L60-L64 #
    exec.prepare_diagonal_mixing_in_words
    push.env.locaddr.0
    exec.diagonal_mixing
end

# see https://github.com/itzmeanjan/blake3/blob/f07d32e/include/blake3.hpp#L1705-L1759 #
proc.compress.1
    popw.local.0

    # round 0 #
    push.env.locaddr.0
    exec.round
    exec.blake3_msg_words_permute

    # round 1 #
    push.env.locaddr.0
    exec.round
    exec.blake3_msg_words_permute

    # round 2 #
    push.env.locaddr.0
    exec.round
    exec.blake3_msg_words_permute

    # round 3 #
    push.env.locaddr.0
    exec.round
    exec.blake3_msg_words_permute

    # round 4 #
    push.env.locaddr.0
    exec.round
    exec.blake3_msg_words_permute

    # round 5 #
    push.env.locaddr.0
    exec.round
    exec.blake3_msg_words_permute

    # round 6 #
    push.env.locaddr.0
    exec.round
    # no permutation required after last round of mixing #
end

# blake3 2-to-1 hash function

Input: First 16 elements of stack ( i.e. stack top ) holds 64 -bytes input digest, 
  which is two blake3 digests concatenated next to each other
  
Output: First 8 elements of stack holds 32 -bytes blake3 digest, 
  while remaining 8 elements of stack top are zeroed #
export.hash.4
    # initializing blake3 hash state for 2-to-1 hashing #
    push.env.locaddr.3
    push.env.locaddr.2
    push.env.locaddr.1
    push.env.locaddr.0

    exec.initialize_hash_state

    # chunk compression, note only one chunk with one message block ( = 64 -bytes ) #
    push.env.locaddr.3
    push.env.locaddr.2
    push.env.locaddr.1
    push.env.locaddr.0

    exec.compress

    # dropping mixed/ permuted input message words from stack top #
    dropw
    dropw
    dropw
    dropw

    # bringing latest blake3 hash state from memory to stack #
    push.env.locaddr.3
    push.env.locaddr.2
    push.env.locaddr.1
    push.env.locaddr.0

    exec.from_mem_to_stack

    # now preparing top 8 elements of stack, so that they contains #
    # blake3 digest on input words #
    exec.prepare_digest

    movupw.3
    movupw.3
    dropw
    dropw
end
"),
// ----- std::crypto::hashes::sha256 --------------------------------------------------------------
("std::crypto::hashes::sha256", "# SHA256 function; see https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2.hpp#L73-L79 #
proc.small_sigma_0
    dup
    u32rotr.7

    swap

    dup
    u32rotr.18

    swap

    u32shr.3

    u32xor
    u32xor
end

# SHA256 function; see https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2.hpp#L81-L87 #
proc.small_sigma_1
    dup
    u32rotr.17

    swap

    dup
    u32rotr.19

    swap

    u32shr.10

    u32xor
    u32xor
end

# SHA256 function; see https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2.hpp#L57-L63 #
proc.cap_sigma_0
    dup
    u32rotr.2

    swap

    dup
    u32rotr.13

    swap

    u32rotr.22

    u32xor
    u32xor
end

# SHA256 function; see https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2.hpp#L65-L71 #
proc.cap_sigma_1
    dup
    u32rotr.6

    swap

    dup
    u32rotr.11

    swap

    u32rotr.25

    u32xor
    u32xor
end

# SHA256 function; see https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2.hpp#L37-L45 #
proc.ch
    swap
    dup.1
    u32and

    swap
    u32not

    movup.2
    u32and

    u32xor
end

# SHA256 function; see https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2.hpp#L47-L55 #
proc.maj
    dup.1
    dup.1
    u32and

    swap
    dup.3
    u32and

    movup.2
    movup.3
    u32and

    u32xor
    u32xor
end

# assume top 4 elements of stack are [3, 2, 1, 0, ...], then after execution of this function, stack should look like [0, 1, 2, 3, ...] #
proc.rev_element_order
    swap
    movup.2
    movup.3
end

proc.gen_four_message_words.1
    # compute message schedule msg[a + 0] | a % 4 == 0 #
    dup.6
    exec.small_sigma_1

    dup.2
    u32add.unsafe
    drop

    dup.10
    exec.small_sigma_0

    u32add.unsafe
    drop

    dup.9
    u32add.unsafe
    drop

    # compute message schedule msg[a + 1] #
    dup.8
    exec.small_sigma_1

    dup.4
    u32add.unsafe
    drop

    dup.12
    exec.small_sigma_0

    u32add.unsafe
    drop

    dup.11
    u32add.unsafe
    drop

    # compute message schedule msg[a + 2] #
    dup.1
    exec.small_sigma_1

    dup.6
    u32add.unsafe
    drop

    dup.14
    exec.small_sigma_0

    u32add.unsafe
    drop

    dup.13
    u32add.unsafe
    drop
    
    # compute message schedule msg[a + 3] #
    dup.1
    exec.small_sigma_1

    dup.8
    u32add.unsafe
    drop

    popw.local.0

    dup.12
    exec.small_sigma_0

    dup.12
    u32add.unsafe
    drop

    pushw.local.0
    movup.4

    u32add.unsafe
    drop

    # stack = [a + 3, a + 2, a + 1, a + 0, ...] #
    exec.rev_element_order
    # stack = [a + 0, a + 1, a + 2, a + 3, ...] #
end

proc.reorder_stack_words
    swapw
    movupw.3
    movupw.2
    movupw.3
end

# SHA256 function; see https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2.hpp#L89-L113 #
proc.prepare_message_schedule.5
    popw.local.0
    popw.local.1
    popw.local.2
    popw.local.3

    movupw.3
    movupw.3

    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.0
    repeat.3
        swap
        drop
    end

    popw.mem # write to mem msg[0, 1, 2, 3] #
    pushw.local.4

    exec.reorder_stack_words
    
    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.0
    drop
    repeat.2
        swap
        drop
    end

    popw.mem # write to mem msg[4, 5, 6, 7] #
    pushw.local.4

    exec.reorder_stack_words

    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.0
    drop
    drop
    swap
    drop

    popw.mem # write to mem msg[8, 9, 10, 11] #
    pushw.local.4

    exec.reorder_stack_words

    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.0
    drop
    drop
    drop

    popw.mem # write to mem msg[12, 13, 14, 15] #
    pushw.local.4

    exec.reorder_stack_words

    # --- #
    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.1
    repeat.3
        swap
        drop
    end

    popw.mem # write to mem msg[16, 17, 18, 19] #
    pushw.local.4

    exec.reorder_stack_words

    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.1
    drop
    repeat.2
        swap
        drop
    end

    popw.mem # write to mem msg[20, 21, 22, 23] #
    pushw.local.4

    exec.reorder_stack_words

    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.1
    drop
    drop
    swap
    drop

    popw.mem # write to mem msg[24, 25, 26, 27] #
    pushw.local.4

    exec.reorder_stack_words

    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.1
    drop
    drop
    drop

    popw.mem # write to mem msg[28, 29, 30, 31] #
    pushw.local.4

    exec.reorder_stack_words

    # --- #
    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.2
    repeat.3
        swap
        drop
    end

    popw.mem # write to mem msg[32, 33, 34, 35] #
    pushw.local.4

    exec.reorder_stack_words

    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.2
    drop
    repeat.2
        swap
        drop
    end

    popw.mem # write to mem msg[36, 37, 38, 39] #
    pushw.local.4

    exec.reorder_stack_words

    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.2
    drop
    drop
    swap
    drop

    popw.mem # write to mem msg[40, 41, 42, 43] #
    pushw.local.4

    exec.reorder_stack_words

    # --- #

    exec.gen_four_message_words

    popw.local.4
    movupw.2

    pushw.local.2
    drop
    drop
    drop

    popw.mem # write to mem msg[44, 45, 46, 47] #
    pushw.local.4

    movupw.3
    pushw.local.3
    repeat.3
        swap
        drop
    end
    popw.mem # write to mem msg[48, 49, 50, 51] #

    swapw
    pushw.local.3
    drop
    repeat.2
        swap
        drop
    end
    popw.mem # write to mem msg[52, 53, 54, 55] #

    swapw
    pushw.local.3
    drop
    drop
    swap
    drop
    popw.mem # write to mem msg[56, 57, 58, 59] #

    pushw.local.3
    drop
    drop
    drop
    popw.mem # write to mem msg[60, 61, 62, 63] #

    # --- #
end

proc.update_hash_state
    # stack = [a, b, c, d, e, f, g, h,  a, b, c, d, e, f, g, h] #

    movup.15
    movup.8
    u32add.unsafe
    drop # = h #

    movup.14
    movup.8
    u32add.unsafe
    drop # = g #

    movup.13
    movup.8
    u32add.unsafe
    drop # = f #

    movup.12
    movup.8
    u32add.unsafe
    drop # = e #

    movup.11
    movup.8
    u32add.unsafe
    drop # = d #

    movup.10
    movup.8
    u32add.unsafe
    drop # = c #

    movup.9
    movup.8
    u32add.unsafe
    drop # = b #

    movup.8
    movup.8
    u32add.unsafe
    drop # = a #

    # stack = [a, b, c, d, e, f, g, h] #
end

# can be treated same as https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2_256.hpp#L168-L175 #
proc.compute_next_working_variables
    # stack = [tmp1, tmp0, a, b, c, d, e, f, g, h] #

    movup.8 # = h #
    movup.8 # = g #
    movup.8 # = f #
    dup.4
    movup.9
    u32add.unsafe
    drop # = e #
    movup.8 # = d #
    movup.8 # = c #
    movup.8 # = b #
    movup.8
    movup.8
    u32add.unsafe
    drop # = a #
    movup.8
    drop

    # stack = [a', b', c', d', e', f', g', h'] #
end

# can be translated to https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2_256.hpp#L144-L187, where single round of SHA256 mixing is performed #
proc.mix.4
    popw.local.0
    popw.local.1
    popw.local.2
    popw.local.3
    
    # --- begin iteration t = 0 --- #

    dupw.1
    dupw.1

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x428a2f98
    u32add.unsafe
    drop

    pushw.local.0
    repeat.3
        swap
        drop
    end
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 1 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x71374491
    u32add.unsafe
    drop

    pushw.local.0
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 2 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xb5c0fbcf
    u32add.unsafe
    drop

    pushw.local.0
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 3 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xe9b5dba5
    u32add.unsafe
    drop

    pushw.local.0
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 4 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x3956c25b
    u32add.unsafe
    drop

    pushw.local.0
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 5 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x59f111f1
    u32add.unsafe
    drop

    pushw.local.0
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 6 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x923f82a4
    u32add.unsafe
    drop

    pushw.local.0
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 7 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xab1c5ed5
    u32add.unsafe
    drop

    pushw.local.0
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 8 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xd807aa98
    u32add.unsafe
    drop

    pushw.local.0
    drop
    drop
    swap
    drop
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 9 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x12835b01
    u32add.unsafe
    drop

    pushw.local.0
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 10 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x243185be
    u32add.unsafe
    drop

    pushw.local.0
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 11 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x550c7dc3
    u32add.unsafe
    drop

    pushw.local.0
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 12 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x72be5d74
    u32add.unsafe
    drop

    pushw.local.0
    drop
    drop
    drop
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 13 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x80deb1fe
    u32add.unsafe
    drop

    pushw.local.0
    drop
    drop
    drop
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 14 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x9bdc06a7
    u32add.unsafe
    drop

    pushw.local.0
    drop
    drop
    drop
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 15 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xc19bf174
    u32add.unsafe
    drop

    pushw.local.0
    drop
    drop
    drop
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 16 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xe49b69c1
    u32add.unsafe
    drop

    pushw.local.1
    repeat.3
        swap
        drop
    end
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 17 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xefbe4786
    u32add.unsafe
    drop

    pushw.local.1
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 18 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x0fc19dc6
    u32add.unsafe
    drop

    pushw.local.1
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 19 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x240ca1cc
    u32add.unsafe
    drop

    pushw.local.1
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 20 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x2de92c6f
    u32add.unsafe
    drop

    pushw.local.1
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 21 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x4a7484aa
    u32add.unsafe
    drop

    pushw.local.1
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 22 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x5cb0a9dc
    u32add.unsafe
    drop

    pushw.local.1
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 23 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x76f988da
    u32add.unsafe
    drop

    pushw.local.1
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 24 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x983e5152
    u32add.unsafe
    drop

    pushw.local.1
    drop
    drop
    swap
    drop
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 25 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xa831c66d
    u32add.unsafe
    drop

    pushw.local.1
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 26 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xb00327c8
    u32add.unsafe
    drop

    pushw.local.1
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 27 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xbf597fc7
    u32add.unsafe
    drop

    pushw.local.1
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 28 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xc6e00bf3
    u32add.unsafe
    drop

    pushw.local.1
    drop
    drop
    drop
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 29 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xd5a79147
    u32add.unsafe
    drop

    pushw.local.1
    drop
    drop
    drop
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 30 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x06ca6351
    u32add.unsafe
    drop

    pushw.local.1
    drop
    drop
    drop
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 31 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x14292967
    u32add.unsafe
    drop

    pushw.local.1
    drop
    drop
    drop
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 32 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x27b70a85
    u32add.unsafe
    drop

    pushw.local.2
    repeat.3
        swap
        drop
    end
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 33 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x2e1b2138
    u32add.unsafe
    drop

    pushw.local.2
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 34 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x4d2c6dfc
    u32add.unsafe
    drop

    pushw.local.2
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 35 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x53380d13
    u32add.unsafe
    drop

    pushw.local.2
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 36 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x650a7354
    u32add.unsafe
    drop

    pushw.local.2
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 37 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x766a0abb
    u32add.unsafe
    drop

    pushw.local.2
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 38 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x81c2c92e
    u32add.unsafe
    drop

    pushw.local.2
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 39 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x92722c85
    u32add.unsafe
    drop

    pushw.local.2
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 40 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xa2bfe8a1
    u32add.unsafe
    drop

    pushw.local.2
    drop
    drop
    swap
    drop
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 41 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xa81a664b
    u32add.unsafe
    drop

    pushw.local.2
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 42 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xc24b8b70
    u32add.unsafe
    drop

    pushw.local.2
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 43 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xc76c51a3
    u32add.unsafe
    drop

    pushw.local.2
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 44 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xd192e819
    u32add.unsafe
    drop

    pushw.local.2
    drop
    drop
    drop
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 45 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xd6990624
    u32add.unsafe
    drop

    pushw.local.2
    drop
    drop
    drop
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 46 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xf40e3585
    u32add.unsafe
    drop

    pushw.local.2
    drop
    drop
    drop
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 47 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x106aa070
    u32add.unsafe
    drop

    pushw.local.2
    drop
    drop
    drop
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 48 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x19a4c116
    u32add.unsafe
    drop

    pushw.local.3
    repeat.3
        swap
        drop
    end
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 49 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x1e376c08
    u32add.unsafe
    drop

    pushw.local.3
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 50 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x2748774c
    u32add.unsafe
    drop

    pushw.local.3
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 51 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x34b0bcb5
    u32add.unsafe
    drop

    pushw.local.3
    repeat.3
        swap
        drop
    end
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 52 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x391c0cb3
    u32add.unsafe
    drop

    pushw.local.3
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 53 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x4ed8aa4a
    u32add.unsafe
    drop

    pushw.local.3
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 54 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x5b9cca4f
    u32add.unsafe
    drop

    pushw.local.3
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 55 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x682e6ff3
    u32add.unsafe
    drop

    pushw.local.3
    drop
    repeat.2
        swap
        drop
    end
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 56 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x748f82ee
    u32add.unsafe
    drop

    pushw.local.3
    drop
    drop
    swap
    drop
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 57 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x78a5636f
    u32add.unsafe
    drop

    pushw.local.3
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 58 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x84c87814
    u32add.unsafe
    drop

    pushw.local.3
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 59 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x8cc70208
    u32add.unsafe
    drop

    pushw.local.3
    drop
    drop
    swap
    drop
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 60 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0x90befffa
    u32add.unsafe
    drop

    pushw.local.3
    drop
    drop
    drop
    pushw.mem
    repeat.3
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 61 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xa4506ceb
    u32add.unsafe
    drop

    pushw.local.3
    drop
    drop
    drop
    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 62 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xbef9a3f7
    u32add.unsafe
    drop

    pushw.local.3
    drop
    drop
    drop
    pushw.mem
    drop
    drop
    swap
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    # --- begin iteration t = 63 --- #

    dupw.1
    exec.ch
    u32add.unsafe
    drop
    dup.5
    exec.cap_sigma_1
    u32add.unsafe
    drop
    push.0xc67178f2
    u32add.unsafe
    drop

    pushw.local.3
    drop
    drop
    drop
    pushw.mem
    drop
    drop
    drop

    u32add.unsafe
    drop

    dupw
    drop
    exec.maj
    dup.2
    exec.cap_sigma_0
    u32add.unsafe
    drop

    exec.compute_next_working_variables

    exec.update_hash_state
end

# Computes SHA256 2-to-1 hash function; see https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2_256.hpp#L121-L196

Input: First 16 elements of stack ( i.e. stack top ) holds 64 -bytes input digest, 
  which is two sha256 digests ( each digest 32 -bytes i.e. 8 stack elements ) concatenated 
  next to each other
  
Output: First 8 elements of stack holds 32 -bytes blake3 digest, 
  while remaining 8 elements of stack top are zeroed #
export.hash.16
    push.env.locaddr.15
    push.env.locaddr.14
    push.env.locaddr.13
    push.env.locaddr.12

    push.env.locaddr.11
    push.env.locaddr.10
    push.env.locaddr.9
    push.env.locaddr.8

    push.env.locaddr.7
    push.env.locaddr.6
    push.env.locaddr.5
    push.env.locaddr.4

    push.env.locaddr.3
    push.env.locaddr.2
    push.env.locaddr.1
    push.env.locaddr.0

    exec.prepare_message_schedule

    # SHA256 initial hash values https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2_256.hpp#L15-L20 #
    push.0x5be0cd19.0x1f83d9ab.0x9b05688c.0x510e527f
    push.0xa54ff53a.0x3c6ef372.0xbb67ae85.0x6a09e667

    push.env.locaddr.15
    push.env.locaddr.14
    push.env.locaddr.13
    push.env.locaddr.12

    push.env.locaddr.11
    push.env.locaddr.10
    push.env.locaddr.9
    push.env.locaddr.8

    push.env.locaddr.7
    push.env.locaddr.6
    push.env.locaddr.5
    push.env.locaddr.4

    push.env.locaddr.3
    push.env.locaddr.2
    push.env.locaddr.1
    push.env.locaddr.0

    exec.mix

    # precompute message schedule for compile-time known 512 -bytes padding 
    words ( see https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2_256.hpp#L89-L99 ),
    i.e. 64 sha256 words.
    
    message schedule computation happens in https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2_256.hpp#L144-L146,
    note in following section, I'm precomputing message schedule for iteration `i = 1` ( see last hyperlink )
    #
    push.0.0.0.2147483648
    popw.local.0
    push.0.0.0.0
    popw.local.1
    push.0.0.0.0
    popw.local.2
    push.512.0.0.0
    popw.local.3
    push.20616.2117632.20971520.2147483648
    popw.local.4
    push.2684354592.84449090.575995924.570427392
    popw.local.5
    push.4202700544.1496221.6067200.1518862336
    popw.local.6
    push.3003913545.4142317530.291985753.3543279056
    popw.local.7
    push.2296832490.216179603.2642168871.145928272
    popw.local.8
    push.1324035729.3610378607.1738633033.2771075893
    popw.local.9
    push.2822718356.3803995842.2397971253.1572820453
    popw.local.10
    push.2958106055.3650881000.921948365.1168996599
    popw.local.11
    push.991993842.3820646885.3172022107.1773959876
    popw.local.12
    push.85264541.322392134.3797604839.419360279
    popw.local.13
    push.3328750644.822159570.640108622.1326255876
    popw.local.14
    push.2242356356.3852183409.1657999800.1107837388
    popw.local.15

    push.env.locaddr.15
    push.env.locaddr.14
    push.env.locaddr.13
    push.env.locaddr.12

    push.env.locaddr.11
    push.env.locaddr.10
    push.env.locaddr.9
    push.env.locaddr.8

    push.env.locaddr.7
    push.env.locaddr.6
    push.env.locaddr.5
    push.env.locaddr.4

    push.env.locaddr.3
    push.env.locaddr.2
    push.env.locaddr.1
    push.env.locaddr.0

    exec.mix
end
"),
// ----- std::math::secp256k1 ---------------------------------------------------------------------
("std::math::secp256k1", "# Given [a, b, c, carry] on stack top, following function computes 

  tmp = a + (b * c) + carry
  hi = tmp >> 32
  lo = tmp & 0xffff_ffff
  return (hi, lo)
  
  At end of execution of this function, stack top should look like [hi, lo] #
export.mac
  swap
  movup.2
  u32madd.unsafe
  
  swap
  movup.2
  u32add.unsafe

  movup.2
  u32add.unsafe
  drop
end

# Given [a0, a1, a2, a3, a4, a5, a6, a7, b, c_0_addr, c_1_addr] on stack top,
  this function computes a multiplication of u256 by u32, while also
  considering u256 computed during previous round.

  - Multiplicand u256 in this context is kept in memory, whose 8 limbs can be loaded
  into stack by pushing content at memory location `c_1_addr` & `c_0_addr`, in order.
  - Multiplier u32 is `b` i.e. stack[8] element.
  - Note, previous round's u256 is kept in first 8 stack elements.

  After finishing execution of this function, stack top should hold u288 i.e.

  [a0, a1, a2, a3, a4, a5, a6, a7, a8] | a8 = carry
#
export.u256xu32
  dup.9
  pushw.mem
  dup.12

  push.0
  swap
  movup.2
  swap
  movup.6
  exec.mac

  movup.2
  dup.12
  movup.6
  exec.mac

  movup.3
  dup.11
  movup.6
  exec.mac

  movup.4
  dup.10
  movup.6
  exec.mac

  dup.11
  pushw.mem

  movup.4
  swap
  dup.13
  movup.10
  exec.mac

  movup.2
  dup.12
  movup.10
  exec.mac

  movup.3
  dup.11
  movup.10
  exec.mac

  movup.4
  dup.10
  movup.10
  exec.mac

  swap
  movup.2
  movup.3
  movup.4
  movup.5
  movup.6
  movup.7
  movup.8
end
"),
// ----- std::math::u256 --------------------------------------------------------------------------
("std::math::u256", "export.add_unsafe
    swapw.3
    movup.3
    movup.7
    u32add.unsafe
    movup.4
    movup.7
    u32addc.unsafe
    movup.4
    movup.6
    u32addc.unsafe
    movup.4
    movup.5
    u32addc.unsafe
    movdn.12
    swapw.2
    movup.12
    movup.4
    movup.8
    u32addc.unsafe
    movup.4
    movup.7
    u32addc.unsafe
    movup.4
    movup.6
    u32addc.unsafe
    movup.4
    movup.5
    u32addc.unsafe
    drop
end

export.sub_unsafe
    swapw.3
    movup.3
    movup.7
    u32sub.unsafe
    movup.7
    u32add.unsafe
    movup.5
    movup.2
    u32sub.unsafe
    movup.2
    add
    movup.6
    u32add.unsafe
    movup.5
    movup.2
    u32sub.unsafe
    movup.2
    add
    movup.5
    u32add.unsafe
    movup.5
    movup.2
    u32sub.unsafe
    movup.2
    add
    movdn.12
    swapw.2
    movup.12
    movup.4
    u32add.unsafe
    movup.8
    movup.2
    u32sub.unsafe
    movup.2
    add
    movup.4
    u32add.unsafe
    movup.7
    movup.2
    u32sub.unsafe
    movup.2
    add
    movup.4
    u32add.unsafe
    movup.6
    movup.2
    u32sub.unsafe
    movup.2
    add
    movup.5
    movup.5
    movup.2
    u32add.unsafe
    drop
    u32sub.unsafe
    drop
end

export.and
    swapw.3
    movup.3
    movup.7
    u32and
    movup.3
    movup.6
    u32and
    movup.3
    movup.5
    u32and
    movup.3
    movup.4
    u32and
    swapw.2
    movup.3
    movup.7
    u32and
    movup.3
    movup.6
    u32and
    movup.3
    movup.5
    u32and
    movup.3
    movup.4
    u32and
end

export.or
    swapw.3
    movup.3
    movup.7
    u32or
    movup.3
    movup.6
    u32or
    movup.3
    movup.5
    u32or
    movup.3
    movup.4
    u32or
    swapw.2
    movup.3
    movup.7
    u32or
    movup.3
    movup.6
    u32or
    movup.3
    movup.5
    u32or
    movup.3
    movup.4
    u32or
end

export.xor
    swapw.3
    movup.3
    movup.7
    u32xor
    movup.3
    movup.6
    u32xor
    movup.3
    movup.5
    u32xor
    movup.3
    movup.4
    u32xor
    swapw.2
    movup.3
    movup.7
    u32xor
    movup.3
    movup.6
    u32xor
    movup.3
    movup.5
    u32xor
    movup.3
    movup.4
    u32xor
end

export.iszero_unsafe
    eq.0
    repeat.7
        swap
        eq.0
        and
    end
end

export.eq_unsafe
    swapw.3
    eqw
    movdn.8
    dropw
    dropw
    movdn.8
    eqw
    movdn.8
    dropw
    dropw
    and
end

# ===== MULTIPLICATION ========================================================================== #
proc.mulstep
    movdn.2
    u32madd.unsafe
    movdn.2
    u32add.unsafe
    movup.2
    add
end

proc.mulstep4
    movup.12
    dup.1
    movup.10
    push.0 # start k at 0 #
    exec.mulstep
    swap
    movdn.9
    dup.1
    movup.9
    movup.13
    swap.3
    exec.mulstep
    swap
    movdn.8
    dup.1
    movup.8
    movup.12
    swap.3
    exec.mulstep
    swap
    movdn.7
    dup.1
    movup.7
    movup.11
    swap.3
    exec.mulstep
    swap
    movdn.6
end

# Performs addition of two unsigned 256 bit integers discarding the overflow. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b7, b6, b5, b4, b3, b2, b1, b0, a7, a6, a5, a4, a3, a2, a1, a0, ...] -> [c7, c6, c5, c4, c3, c2, c1, c0, ...] #
# where c = (a * b) % 2^256, and a0, b0, and c0 are least significant 32-bit limbs of a, b, and c respectively. #
export.mul_unsafe.6
    # Memory storing setup #
    popw.local.0
    # b[5-8] at 0 #
    storew.local.1
    # b[0-4] at 1 #
    push.0 dropw
    # b[0] at top of stack, followed by a[0-7] #
    movdn.8
    storew.local.2
    # a[0-4] at 2 #
    swapw
    storew.local.3
    # a[5-8] at 3 #
    padw
    storew.local.4
    storew.local.5
    # p at 4 and 5 #

    # b[0] #
    dropw
    swapw
    pushw.local.4
    movdnw.2
    movup.12

    exec.mulstep4

    movdn.9
    movdn.9
    swapw
    popw.local.4
    pushw.local.5
    swapw
    movup.9
    movup.9

    dup.1
    movup.6
    movup.10
    swap.3
    exec.mulstep
    swap
    movdn.5
    dup.1
    movup.5
    movup.9
    swap.3
    exec.mulstep
    swap
    movdn.4
    dup.1
    movup.4
    movup.8
    swap.3
    exec.mulstep
    swap
    movdn.3
    swap
    movup.2
    movup.6
    swap.3
    exec.mulstep

    drop
    popw.local.5

    # b[1] #
    pushw.local.4
    pushw.local.5
    movup.7
    dropw
    pushw.local.3 pushw.local.2 # load the xs #
    pushw.local.1
    movup.2
    movdn.3
    push.0 dropw # only need b[1] #

    exec.mulstep4

    movdn.9
    movdn.9
    swapw
    movdn.3
    pushw.local.4
    push.0 dropw # only need p[0] #
    movdn.3
    # save p[0-3] to memory, not needed any more #
    popw.local.4

    pushw.local.5
    movup.3
    drop
    swapw
    movup.9
    movup.9

    dup.1
    movup.6
    movup.9
    swap.3
    exec.mulstep
    swap
    movdn.7
    dup.1
    movup.5
    movup.7
    swap.3
    exec.mulstep
    swap
    movdn.5
    swap
    movup.3
    movup.4
    swap.3
    exec.mulstep

    drop
    swap
    drop
    popw.local.5

    # b[2] #
    pushw.local.4
    pushw.local.5
    movup.7
    movup.7
    dropw
    pushw.local.3 pushw.local.2 # load the xs #
    pushw.local.1
    swap
    movdn.3
    push.0 dropw # only need b[1] #

    exec.mulstep4

    movdn.9
    movdn.9
    swapw
    movdn.3
    movdn.3
    pushw.local.4
    drop drop
    movdn.3
    movdn.3
    popw.local.4

    pushw.local.5
    movup.3
    movup.3
    drop
    drop
    swapw
    movup.9
    movup.9

    dup.1
    movup.6
    movup.8
    swap.3
    exec.mulstep
    swap
    movdn.6
    dup.1
    movup.5
    movup.6
    swap.3
    exec.mulstep
    swap
    swap drop
    movdn.3
    drop drop drop
    popw.local.5

    # b[3] #
    pushw.local.4
    pushw.local.5

    movup.7 movup.7 movup.7
    dropw
    pushw.local.3 pushw.local.2

    pushw.local.1
    movdn.3
    push.0 dropw

    exec.mulstep4

    movdn.9
    movdn.9

    swapw
    movup.3
    pushw.local.4
    drop
    movup.3

    popw.local.4
    pushw.local.5
    movdn.3
    push.0 dropw
    swapw
    movup.9
    movup.9

    swap
    movup.5
    movup.6
    swap.3
    exec.mulstep

    drop
    movdn.3
    push.0 dropw

    # b[4] #
    pushw.local.3 pushw.local.2 # load the xs #
    # OPTIM: don't need a[4-7] #, but can't use mulstep4 if we don't load #

    pushw.local.0
    push.0 dropw # b[4] #

    exec.mulstep4
    dropw drop drop # OPTIM: don't need a[4-7] #, but can't use mulstep4 if we don't load #

    # b[5] #
    pushw.local.3
    pushw.local.0
    movup.2 movdn.3
    push.0 dropw
    movup.7
    dup.1
    movup.6
    push.0
    exec.mulstep
    swap
    movdn.7
    movup.4
    dup.2
    movup.7
    swap.3
    exec.mulstep
    swap
    movdn.5
    swap
    movup.3
    movup.4
    swap.3
    exec.mulstep
    drop
    swap
    drop

    # b[6] #
    pushw.local.3
    pushw.local.0
    swap
    movdn.3
    push.0 dropw
    movup.6
    dup.1
    movup.6
    push.0
    exec.mulstep
    swap
    movdn.6
    swap
    movup.4
    movup.5
    swap.3
    exec.mulstep
    drop
    movdn.2
    drop drop

    # b[7] #
    pushw.local.3
    pushw.local.0

    movdn.3 push.0 dropw
    movup.4
    movup.5
    movdn.2
    push.0
    exec.mulstep
    drop
    movdn.3
    drop drop drop

    pushw.local.4
    swapw
end"),
// ----- std::math::u64 ---------------------------------------------------------------------------
("std::math::u64", "# ===== HELPER FUNCTIONS ======================================================================== #

# Asserts that both values at the top of the stack are u64 values. #
# The input values are assumed to be represented using 32 bit limbs, fails if they are not. #
proc.u64assert4
    u32assert
    movup.3
    u32assert
    movup.3
    u32assert
    movup.3
    u32assert
    movup.3
end

# ===== ADDITION ================================================================================ #

# Performs addition of two unsigned 64 bit integers discarding the overflow. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a + b) % 2^64 #
export.add_unsafe
    swap
    movup.3
    u32add.unsafe
    movup.3
    movup.3
    u32addc.unsafe
    drop
end

# ===== SUBTRACTION ============================================================================= #

# Performs subtraction of two unsigned 64 bit integers discarding the overflow. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a - b) % 2^64 #
export.sub_unsafe
    movup.3
    movup.2
    u32sub.unsafe
    movup.3
    movup.3
    u32sub.unsafe
    drop
    swap
    u32sub.unsafe
    drop
end

# ===== MULTIPLICATION ========================================================================== #

# Performs multiplication of two unsigned 64 bit integers discarding the overflow. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a * b) % 2^64 #
export.mul_unsafe
    dup.3
    dup.2
    u32mul.unsafe
    movup.4
    movup.4
    u32madd.unsafe
    drop
    movup.3
    movup.3
    u32madd.unsafe
    drop
end

# ===== COMPARISONS ============================================================================= #

# Performs less-than comparison of two unsigned 64 bit integers. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a < b, and 0 otherwise. #
export.lt_unsafe
    movup.3
    movup.2
    u32sub.unsafe
    movdn.3
    drop
    u32sub.unsafe
    swap
    eq.0
    movup.2
    and
    or
end

# Performs greater-than comparison of two unsigned 64 bit integers. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a > b, and 0 otherwise. #
export.gt_unsafe
    movup.2
    u32sub.unsafe
    movup.2
    movup.3
    u32sub.unsafe
    swap
    drop
    movup.2
    eq.0
    and
    or
end

# Performs less-than-or-equal comparison of two unsigned 64 bit integers. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a <= b, and 0 otherwise. #
export.lte_unsafe
    exec.gt_unsafe
    not
end

# Performs greater-than-or-equal comparison of two unsigned 64 bit integers. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a >= b, and 0 otherwise. #
export.gte_unsafe
    exec.lt_unsafe
    not
end

# Performs equality comparison of two unsigned 64 bit integers. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == b, and 0 otherwise. #
export.eq_unsafe
    movup.2
    u32eq
    swap
    movup.2
    u32eq
    and
end

# Performs comparison to zero of an unsigned 64 bit integer. #
# The input value is assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == 0, and 0 otherwise. #
export.eqz_unsafe
    u32eq.0
    swap
    u32eq.0
    and
end

# ===== BITWISE OPERATIONS ====================================================================== #

# Performs bitwise AND of two unsigned 64 bit integers. #
# The input values are assumed to be represented using 32 bit limbs, fails if they are not. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a AND b. #
export.and
    swap
    movup.3
    u32and
    swap
    movup.2
    u32and
end

# Performs bitwise OR of two unsigned 64 bit integers. #
# The input values are assumed to be represented using 32 bit limbs, fails if they are not. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a OR b. #
export.or
    swap
    movup.3
    u32or
    swap
    movup.2
    u32or
end

# Performs bitwise XOR of two unsigned 64 bit integers. #
# The input values are assumed to be represented using 32 bit limbs, fails if they are not. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a XOR b. #
export.xor
    swap
    movup.3
    u32xor
    swap
    movup.2
    u32xor
end

# ===== DIVISION ================================================================================ #

# Performs division of two unsigned 64 bit integers discarding the remainder. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a // b #
export.div_unsafe
    adv.u64div          # inject the quotient and the remainder into the advice tape #
    
    push.adv.1          # read the quotient from the advice tape and make sure it consists of #
    u32assert           # 32-bit limbs #
    push.adv.1          # TODO: this can be optimized once we have u32assert2 instruction #
    u32assert

    dup.3               # multiply quotient by the divisor and make sure the resulting value #
    dup.2               # fits into 2 32-bit limbs #
    u32mul.unsafe
    dup.4
    dup.4
    u32madd.unsafe
    eq.0
    assert
    dup.5
    dup.3
    u32madd.unsafe
    eq.0
    assert
    dup.4
    dup.3
    mul
    eq.0
    assert

    push.adv.1          # read the remainder from the advice tape and make sure it consists of #
    u32assert           # 32-bit limbs #
    push.adv.1
    u32assert

    movup.7             # make sure the divisor is greater than the remainder. this also consumes #
    movup.7             # the divisor #
    dup.3
    dup.3
    exec.gt_unsafe
    assert

    swap                # add remainder to the previous result; this also consumes the remainder #
    movup.3
    u32add.unsafe
    movup.3
    movup.3
    u32addc.unsafe
    eq.0
    assert

    movup.4             # make sure the result we got is equal to the dividend #
    assert.eq
    movup.3
    assert.eq           # quotient remains on the stack #
end
"),
];
