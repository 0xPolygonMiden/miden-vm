//! This module is automatically generated during build time and should not be modified manually.

/// An array of modules defined in Miden standard library.
///
/// Entries in the array are tuples containing module namespace and module source code.
#[rustfmt::skip]
pub const MODULES: [(&str, &str); 7] = [
// ----- std::crypto::hashes::blake3 --------------------------------------------------------------
("std::crypto::hashes::blake3", "# Initializes four memory addresses, provided for storing initial 4x4 blake3 
# state matrix ( i.e. 16 elements each of 32 -bit ), for computing blake3 2-to-1 hash
#
# Expected stack state:
#
# [state_0_3_addr, state_4_7_addr, state_8_11_addr, state_12_15_addr]
#
# Note, state_`i`_`j`_addr -> absolute address of {state[i], state[i+1], state[i+2], state[i+3]} in memory | j = i+3
#
# Final stack state:
#
# [...]
#
# Initialized stack state is written back to provided memory addresses.
#
# Functionally this routine is equivalent to https://github.com/itzmeanjan/blake3/blob/f07d32e/include/blake3.hpp#L1709-L1713
proc.initialize
    push.0xA54FF53A.0x3C6EF372.0xBB67AE85.0x6A09E667
    movup.4
    popw.mem

    push.0x5BE0CD19.0x1F83D9AB.0x9B05688C.0x510E527F
    movup.4
    popw.mem

    push.0xA54FF53A.0x3C6EF372.0xBB67AE85.0x6A09E667
    movup.4
    popw.mem

    push.11.64.0.0
    movup.4
    popw.mem
end

# Permutes ordered message words, kept on stack top ( = sixteen 32 -bit BLAKE3 words )
#
# Expected stack top: 
#
# [s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15]
#
# After permutation, stack top:
#
# [s2, s6, s3, s10, s7, s0, s4, s13, s1, s11, s12, s5, s9, s14, s15, s8]
#
# See https://github.com/itzmeanjan/blake3/blob/f07d32ec10cbc8a10663b7e6539e0b1dab3e453b/include/blake3.hpp#L1623-L1639
# and https://github.com/maticnetwork/miden/pull/313#discussion_r922627984
proc.permute_msg_words
    movdn.7
    movup.5
    movdn.2
    movup.4
    movdn.7
    swapw.3
    swap
    movdn.7
    swapdw
    movup.2
    movdn.7
    swapw
    swapw.2
    movup.3
    movdn.6
    movdn.5
    movup.3
    swapw
    movup.3
    swapdw
end

# Given blake3 state matrix on stack top ( in order ) as 16 elements ( each of 32 -bit ),
# this routine computes output chaining value i.e. 2-to-1 hashing digest.
#
# Expected stack state:
#
# [state0, state1, state2, state3, state4, state5, state6, state7, state8, state9, state10, state11, state12, state13, state14, state15]
#
# After finalizing, stack should look like
#
# [dig0, dig1, dig2, dig3, dig4, dig5, dig6, dig7]
#
# See https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#L116-L119 ,
# you'll notice I've skipped executing second statement in loop body of above hyperlinked implementation,
# that's because it doesn't dictate what output of 2-to-1 hash will be.
proc.finalize
    movup.8
    u32checked_xor

    swap
    movup.8
    u32checked_xor
    swap

    movup.2
    movup.8
    u32checked_xor
    movdn.2

    movup.3
    movup.8
    u32checked_xor
    movdn.3

    movup.4
    movup.8
    u32checked_xor
    movdn.4

    movup.5
    movup.8
    u32checked_xor
    movdn.5

    movup.6
    movup.8
    u32checked_xor
    movdn.6

    movup.7
    movup.8
    u32checked_xor
    movdn.7
end

# Given blake3 state matrix ( total 16 elements, each of 32 -bit ) and 
# 8 message words ( each of 32 -bit ), this routine performs column-wise mixing
# of message words into blake3 hash state.
#
# Functionality wise this routine is equivalent to https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#L55-L59
#
# Expected stack state:
#
# [state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr, m0, m1, m2, m3, m4, m5, m6, m7]
#
# Note, state_`i`_`j`_addr -> absolute address of {state[i], state[i+1], state[i+2], state[i+3]} in memory | j = i+3
#
# Meaning four consecutive blake3 state words can be read from memory easily.
#
# Final stack state:
#
# [state0, state1, state2, state3, state4, state5, state6, state7, state8, state9, state10, state11, state12, state13, state14, state15]
#
# i.e. whole blake3 state is placed on stack ( in order ).
proc.columnar_mixing.1
    swapw.2
    swapw

    movup.7
    movup.6
    movup.5
    movup.4

    storew.local.0

    movup.9
    loadw.mem
    movup.8
    pushw.mem

    movup.8
    dup.5
    u32overflowing_add3
    drop

    swap
    movup.8
    dup.6
    u32overflowing_add3
    drop
    swap

    movup.2
    dup.6
    movup.9
    u32overflowing_add3
    drop
    movdn.2

    movup.3
    dup.7
    movup.9
    u32overflowing_add3
    drop
    movdn.3

    movup.9
    pushw.mem

    dup.4
    u32checked_xor
    u32unchecked_rotr.16
    
    swap
    dup.5
    u32checked_xor
    u32unchecked_rotr.16
    swap

    movup.2
    dup.6
    u32checked_xor
    u32unchecked_rotr.16
    movdn.2

    movup.3
    dup.7
    u32checked_xor
    u32unchecked_rotr.16
    movdn.3

    movup.12
    pushw.mem

    dup.4
    u32wrapping_add

    swap
    dup.5
    u32wrapping_add
    swap

    movup.2
    dup.6
    u32wrapping_add
    movdn.2

    movup.3
    dup.7
    u32wrapping_add
    movdn.3

    movupw.3

    dup.4
    u32checked_xor
    u32unchecked_rotr.12
    
    swap
    dup.5
    u32checked_xor
    u32unchecked_rotr.12
    swap

    movup.2
    dup.6
    u32checked_xor
    u32unchecked_rotr.12
    movdn.2

    movup.3
    dup.7
    u32checked_xor
    u32unchecked_rotr.12
    movdn.3

    movupw.3
    pushw.local.0
    swapw

    movup.4
    dup.8
    u32overflowing_add3
    drop

    swap
    movup.4
    dup.8
    u32overflowing_add3
    drop
    swap

    movup.2
    movup.4
    dup.8
    u32overflowing_add3
    drop
    movdn.2

    movup.3
    movup.4
    dup.8
    u32overflowing_add3
    drop
    movdn.3

    movupw.3

    dup.4
    u32checked_xor
    u32unchecked_rotr.8
    
    swap
    dup.5
    u32checked_xor
    u32unchecked_rotr.8
    swap

    movup.2
    dup.6
    u32checked_xor
    u32unchecked_rotr.8
    movdn.2

    movup.3
    dup.7
    u32checked_xor
    u32unchecked_rotr.8
    movdn.3

    movupw.3

    dup.4
    u32wrapping_add

    swap
    dup.5
    u32wrapping_add
    swap

    movup.2
    dup.6
    u32wrapping_add
    movdn.2

    movup.3
    dup.7
    u32wrapping_add
    movdn.3

    movupw.3

    dup.4
    u32checked_xor
    u32unchecked_rotr.7

    swap
    dup.5
    u32checked_xor
    u32unchecked_rotr.7
    swap

    movup.2
    dup.6
    u32checked_xor
    u32unchecked_rotr.7
    movdn.2

    movup.3
    dup.7
    u32checked_xor
    u32unchecked_rotr.7
    movdn.3

    movupw.3
end

# Given blake3 state matrix ( total 16 elements, each of 32 -bit ) and 
# 8 message words ( each of 32 -bit ), this routine performs diagonal-wise mixing
# of message words into blake3 hash state.
#
# Functionality wise this routine is equivalent to https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#L61-L64
#
# Expected stack state:
#
# [state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr, m0, m1, m2, m3, m4, m5, m6, m7]
#
# Note, state_`i`_`j`_addr -> absolute address of {state[i], state[i+1], state[i+2], state[i+3]} in memory | j = i+3
#
# Meaning four consecutive blake3 state words can be read from memory easily.
#
# Final stack state:
#
# [state0, state1, state2, state3, state4, state5, state6, state7, state8, state9, state10, state11, state12, state13, state14, state15]
#
# i.e. whole blake3 state is placed on stack ( in order ).
proc.diagonal_mixing.1
    swapw.2
    swapw

    movup.7
    movup.6
    movup.5
    movup.4

    storew.local.0

    movup.9
    loadw.mem
    movup.8
    pushw.mem

    movup.8
    dup.6
    u32overflowing_add3
    drop

    swap
    movup.8
    dup.7
    u32overflowing_add3
    drop
    swap

    movup.2
    movup.8
    dup.8
    u32overflowing_add3
    drop
    movdn.2

    movup.3
    movup.8
    dup.5
    u32overflowing_add3
    drop
    movdn.3

    movup.9
    pushw.mem

    movup.3
    dup.4
    u32checked_xor
    u32unchecked_rotr.16
    movdn.3

    dup.5
    u32checked_xor
    u32unchecked_rotr.16

    swap
    dup.6
    u32checked_xor
    u32unchecked_rotr.16
    swap

    movup.2
    dup.7
    u32checked_xor
    u32unchecked_rotr.16
    movdn.2

    movup.12
    pushw.mem

    movup.2
    dup.7
    u32wrapping_add
    movdn.2

    movup.3
    dup.4
    u32wrapping_add
    movdn.3

    dup.5
    u32wrapping_add

    swap
    dup.6
    u32wrapping_add
    swap

    movupw.3

    swap
    dup.6
    u32checked_xor
    u32unchecked_rotr.12
    swap

    movup.2
    dup.7
    u32checked_xor
    u32unchecked_rotr.12
    movdn.2

    movup.3
    dup.4
    u32checked_xor
    u32unchecked_rotr.12
    movdn.3

    dup.5
    u32checked_xor
    u32unchecked_rotr.12

    movupw.3
    pushw.local.0
    swapw

    movup.4
    dup.9
    u32overflowing_add3
    drop

    swap
    movup.4
    dup.9
    u32overflowing_add3
    drop
    swap

    movup.2
    movup.4
    dup.9
    u32overflowing_add3
    drop
    movdn.2

    movup.3
    movup.4
    dup.5
    u32overflowing_add3
    drop
    movdn.3

    movupw.3

    movup.3
    dup.4
    u32checked_xor
    u32unchecked_rotr.8
    movdn.3

    dup.5
    u32checked_xor
    u32unchecked_rotr.8

    swap
    dup.6
    u32checked_xor
    u32unchecked_rotr.8
    swap

    movup.2
    dup.7
    u32checked_xor
    u32unchecked_rotr.8
    movdn.2

    movupw.3

    movup.2
    dup.7
    u32wrapping_add
    movdn.2

    movup.3
    dup.4
    u32wrapping_add
    movdn.3

    dup.5
    u32wrapping_add

    swap
    dup.6
    u32wrapping_add
    swap

    movupw.3

    swap
    dup.6
    u32checked_xor
    u32unchecked_rotr.7
    swap

    movup.2
    dup.7
    u32checked_xor
    u32unchecked_rotr.7
    movdn.2

    movup.3
    dup.4
    u32checked_xor
    u32unchecked_rotr.7
    movdn.3

    dup.5
    u32checked_xor
    u32unchecked_rotr.7

    movupw.3
end

# Given blake3 state matrix ( total 16 elements, each of 32 -bit ) and 
# 16 message words ( each of 32 -bit ), this routine applies single round of mixing
# of message words into hash state i.e. msg_word[0..8] are mixed into hash state using
# columnar mixing while remaining message words ( msg_word[8..16] ) are mixed into hash state
# using diagonal mixing.
#
# Functionality wise this routine is equivalent to https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#L54-L65
#
# Expected stack state:
#
# [state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr, m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15]
#
# Note, state_`i`_`j`_addr -> absolute address of {state[i], state[i+1], state[i+2], state[i+3]} in memory | j = i+3
#
# Meaning four consecutive blake3 state words can be read from memory easily.
#
# Final stack state:
#
# [...]
#
# i.e. mixed state matrix lives in memory addresses {state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr}, 
# which were provided, on stack top, while invoking this routine.
proc.round.5
    storew.local.0

    exec.columnar_mixing

    popw.local.1
    popw.local.2
    popw.local.3
    popw.local.4

    push.env.locaddr.4
    push.env.locaddr.3
    push.env.locaddr.2
    push.env.locaddr.1

    exec.diagonal_mixing

    pushw.local.0
    swapw
    movup.4
    popw.mem

    repeat.3
        push.0
        movdn.3
        swapw
        movup.4
        popw.mem
    end

    repeat.3
        drop
    end
end

# Given blake3 state matrix ( total 16 elements, each of 32 -bit ) and a message block
# i.e. 16 message words ( each of 32 -bit ), this routine applies 7 rounds of mixing
# of (permuted) message words into hash state.
#
# Functionality wise this routine is equivalent to https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#L75-L114
#
# Expected stack state:
#
# [state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr, m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15]
#
# Note, state_`i`_`j`_addr -> absolute address of {state[i], state[i+1], state[i+2], state[i+3]} in memory | j = i+3
#
# Meaning four consecutive blake3 state words can be read from memory easily.
#
# Final stack state:
#
# [...]
#
# i.e. 7 -round mixed state matrix lives in memory addresses {state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr}, 
# which were provided, on stack top, while invoking this routine. So updated state matrix can be read by caller routine, by reading
# the content of memory addresses where state was provided as routine input.
proc.compress.1
    popw.local.0

    # apply first 6 rounds of mixing
    repeat.6
        # round `i` | i ∈ [1..7)
        repeat.4
            dupw.3
        end

        pushw.local.0
        exec.round
        exec.permute_msg_words
    end

    # round 7 ( last round, so no message word permutation required )
    pushw.local.0
    exec.round
end

# Blake3 2-to-1 hash function, which takes 64 -bytes input and produces 32 -bytes output digest
#
# Expected stack state:
#
# [msg0, msg1, msg2, msg3, msg4, msg5, msg6, msg7, msg8, msg9, msg10, msg11, msg12, msg13, msg14, msg15]
#
# msg`i` -> 32 -bit message word | i ∈ [0, 16)
#
# Output stack state:
#
# [dig0, dig1, dig2, dig3, dig4, dig5, dig6, dig7]
#
# dig`i` -> 32 -bit digest word | i ∈ [0, 8)
export.hash.4
    push.env.locaddr.3
    push.env.locaddr.2
    push.env.locaddr.1
    push.env.locaddr.0

    exec.initialize

    # Note, chunk compression routine needs to compress only one chunk with one message 
    # block ( = 64 -bytes ) because what we're doing here is 2-to-1 hashing i.e. 64 -bytes 
    # input being converted to 32 -bytes output

    push.env.locaddr.3
    push.env.locaddr.2
    push.env.locaddr.1
    push.env.locaddr.0

    exec.compress

    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.finalize
end
"),
// ----- std::crypto::hashes::keccak256 -----------------------------------------------------------
("std::crypto::hashes::keccak256", "# if stack top has [d, c, b, a], after completion of execution of
# this procedure stack top should look like [a, b, c, d]
proc.rev_4_elements
    swap
    movup.2
    movup.3
end

# given four elements of from each of a, b sets, following procedure computes a[i] ^ b[i] ∀ i = [0, 3]
proc.xor_4_elements
    movup.7
    u32checked_xor

    swap

    movup.6
    u32checked_xor

    movup.2
    movup.5
    u32checked_xor

    movup.4
    movup.4
    u32checked_xor
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's θ function, which is
# implemented in terms of 32 -bit word size;
# see https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L55-L98 for original implementation
proc.theta.7
    popw.local.0
    popw.local.1
    popw.local.2
    popw.local.3

    # --- begin https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L71-L79 ---

    # compute a[0] ^ a[10] ^ a[20] ^ a[30] ^ a[40]
    loadw.local.0
    swap
    drop
    movup.2
    drop

    pushw.mem
    repeat.3
        swap
        drop
    end

    swap
    pushw.mem
    drop
    drop
    swap
    drop

    u32checked_xor

    pushw.local.1
    drop
    swap
    drop

    pushw.mem
    repeat.3
        swap
        drop
    end

    swap
    pushw.mem
    drop
    drop
    swap
    drop

    u32checked_xor
    u32checked_xor

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

    u32checked_xor

    # stack = [c_0]
    # -----
    # compute a[1] ^ a[11] ^ a[21] ^ a[31] ^ a[41]

    pushw.local.0
    swap
    drop
    movup.2
    drop

    pushw.mem
    drop
    repeat.2
        swap
        drop
    end

    swap
    pushw.mem
    drop
    drop
    drop

    u32checked_xor

    pushw.local.1
    drop
    swap
    drop

    pushw.mem

    drop
    repeat.2
        swap
        drop
    end

    swap

    pushw.mem
    drop
    drop
    drop

    u32checked_xor
    u32checked_xor

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

    u32checked_xor

    # stack = [c_1, c_0]
    # -----
    # compute a[2] ^ a[12] ^ a[22] ^ a[32] ^ a[42]

    pushw.local.0
    repeat.2
        swap
        drop
    end

    pushw.mem

    drop
    drop
    swap
    drop

    swap

    pushw.mem

    repeat.3
        swap
        drop
    end

    u32checked_xor

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

    u32checked_xor

    pushw.local.2

    swap
    drop
    movup.2
    drop

    pushw.mem

    repeat.3
        swap
        drop
    end

    swap

    pushw.mem

    drop
    drop
    swap
    drop

    u32checked_xor
    u32checked_xor

    # stack = [c_2, c_1, c_0]
    # -----
    # compute a[3] ^ a[13] ^ a[23] ^ a[33] ^ a[43]

    pushw.local.0

    repeat.2
        swap
        drop
    end

    pushw.mem

    drop
    drop
    drop

    swap

    pushw.mem

    drop
    repeat.2
        swap
        drop
    end

    u32checked_xor

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

    u32checked_xor

    pushw.local.2

    swap
    drop
    movup.2
    drop

    pushw.mem

    drop
    repeat.2
        swap
        drop
    end

    swap

    pushw.mem

    drop
    drop
    drop

    u32checked_xor
    u32checked_xor

    # stack = [c_3, c_2, c_1, c_0]
    # -----
    # compute a[4] ^ a[14] ^ a[24] ^ a[34] ^ a[44]

    pushw.local.0

    drop
    swap
    drop

    pushw.mem

    repeat.3
        swap
        drop
    end

    swap

    pushw.mem

    drop
    drop
    swap
    drop

    u32checked_xor

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

    u32checked_xor

    pushw.local.2

    repeat.2
        swap
        drop
    end

    pushw.mem

    drop
    drop
    swap
    drop

    swap

    pushw.mem

    repeat.3
        swap
        drop
    end

    u32checked_xor
    u32checked_xor

    # stack = [c_4, c_3, c_2, c_1, c_0]
    # -----
    # compute a[5] ^ a[15] ^ a[25] ^ a[35] ^ a[45]

    pushw.local.0

    drop
    swap
    drop

    pushw.mem

    drop
    repeat.2
        swap
        drop
    end

    swap

    pushw.mem

    drop
    drop
    drop

    u32checked_xor

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

    u32checked_xor

    pushw.local.2

    repeat.2
        swap
        drop
    end

    pushw.mem

    drop
    drop
    drop

    swap

    pushw.mem

    drop
    repeat.2
        swap
        drop
    end

    u32checked_xor
    u32checked_xor

    # stack = [c_5, c_4, c_3, c_2, c_1, c_0]
    # -----
    # compute a[6] ^ a[16] ^ a[26] ^ a[36] ^ a[46]

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

    pushw.local.1

    swap
    drop
    movup.2
    drop

    pushw.mem

    repeat.3
        swap
        drop
    end

    swap

    pushw.mem

    drop
    drop
    swap
    drop

    u32checked_xor
    u32checked_xor

    pushw.local.2

    drop
    swap
    drop

    pushw.mem

    repeat.3
        swap
        drop
    end

    swap

    pushw.mem

    drop
    drop
    swap
    drop

    u32checked_xor
    u32checked_xor

    # stack = [c_6, c_5, c_4, c_3, c_2, c_1, c_0]
    # -----
    # compute a[7] ^ a[17] ^ a[27] ^ a[37] ^ a[47]

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

    pushw.local.1

    swap
    drop
    movup.2
    drop

    pushw.mem

    drop
    repeat.2
        swap
        drop
    end

    swap

    pushw.mem

    drop
    drop
    drop

    u32checked_xor
    u32checked_xor

    pushw.local.2

    drop
    swap
    drop

    pushw.mem

    drop
    repeat.2
        swap
        drop
    end

    swap

    pushw.mem

    drop
    drop
    drop

    u32checked_xor
    u32checked_xor

    # stack = [c_7, c_6, c_5, c_4, c_3, c_2, c_1, c_0]
    # -----
    # compute a[8] ^ a[18] ^ a[28] ^ a[38] ^ a[48]

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

    pushw.local.1

    repeat.2
        swap
        drop
    end

    pushw.mem

    drop
    drop
    swap
    drop

    swap

    pushw.mem

    repeat.3
        swap
        drop
    end

    u32checked_xor
    u32checked_xor

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

    u32checked_xor

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

    u32checked_xor

    # stack = [c_8, c_7, c_6, c_5, c_4, c_3, c_2, c_1, c_0]
    # -----
    # compute a[9] ^ a[19] ^ a[29] ^ a[39] ^ a[49]

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

    pushw.local.1

    repeat.2
        swap
        drop
    end

    pushw.mem

    drop
    drop
    drop

    swap

    pushw.mem

    drop
    repeat.2
        swap
        drop
    end

    u32checked_xor
    u32checked_xor

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

    u32checked_xor
    u32checked_xor

    push.0.0

    # stack = [0, 0, c_9, c_8, c_7, c_6, c_5, c_4, c_3, c_2, c_1, c_0]

    exec.rev_4_elements
    popw.local.6 # -> to mem [c8, c9, 0, 0]

    exec.rev_4_elements
    popw.local.5 # -> to mem [c4, c5, c6, c7]

    exec.rev_4_elements
    popw.local.4 # -> to mem [c0, c1, c2, c3]

    # --- end https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L71-L79 ---

    # --- begin https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L81-L91 ---

    pushw.local.6
    movup.3
    drop
    movup.2
    drop

    pushw.local.4
    drop
    drop

    movup.3
    u32checked_xor

    swap
    movup.2
    swap

    u32checked_rotl.1
    u32checked_xor

    # stack = [d0, d1]

    pushw.local.4
    movup.3
    drop
    movup.2
    drop

    pushw.local.5
    movup.3
    drop
    movup.2
    drop

    movup.3
    u32checked_xor

    swap
    u32checked_rotl.1
    movup.2
    u32checked_xor

    # stack = [d2, d3, d0, d1]

    movup.3
    movup.3

    # stack = [d0, d1, d2, d3]

    pushw.local.4
    drop
    drop

    pushw.local.5
    drop
    drop

    movup.3
    u32checked_xor

    swap
    u32checked_rotl.1
    movup.2
    u32checked_xor

    # stack = [d4, d5, d0, d1, d2, d3]

    pushw.local.5
    movup.3
    drop
    movup.2
    drop

    pushw.local.6
    movup.3
    drop
    movup.2
    drop

    movup.3
    u32checked_xor

    swap
    u32checked_rotl.1
    movup.2
    u32checked_xor

    # stack = [d6, d7, d4, d5, d0, d1, d2, d3]

    movup.3
    movup.3

    # stack = [d4, d5, d6, d7, d0, d1, d2, d3]

    pushw.local.5
    drop
    drop

    pushw.local.4
    movup.3
    drop
    movup.2
    drop

    movup.3
    u32checked_xor

    swap
    u32checked_rotl.1
    movup.2
    u32checked_xor

    # stack = [d8, d9, d4, d5, d6, d7, d0, d1, d2, d3]

    push.0.0
    movup.3
    movup.3

    # stack = [d8, d9, 0, 0, d4, d5, d6, d7, d0, d1, d2, d3]

    popw.local.6 # -> to mem [d8, d9, 0, 0]
    popw.local.5 # -> to mem [d4, d5, d6, d7]
    popw.local.4 # -> to mem [d0, d1, d2, d3]

    # --- end https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L81-L91 ---

    pushw.local.0
    dupw

    pushw.mem

    pushw.local.4
    exec.rev_4_elements

    exec.xor_4_elements # compute state[0..4]

    movup.7
    popw.mem

    pushw.mem

    pushw.local.5
    exec.rev_4_elements

    exec.xor_4_elements # compute state[4..8]

    movup.6
    popw.mem

    pushw.mem

    pushw.local.6
    exec.rev_4_elements

    drop
    drop

    pushw.local.4
    exec.rev_4_elements

    drop
    drop

    exec.xor_4_elements # compute state[8..12]

    movup.5
    popw.mem

    pushw.mem

    pushw.local.4
    drop
    drop
    swap

    pushw.local.5
    exec.rev_4_elements

    drop
    drop

    exec.xor_4_elements # compute state[12..16]

    movup.4
    popw.mem

    pushw.local.1
    dupw

    pushw.mem

    pushw.local.5
    drop
    drop
    swap

    pushw.local.6
    exec.rev_4_elements

    drop
    drop

    exec.xor_4_elements # compute state[16..20]

    movup.7
    popw.mem

    pushw.mem

    pushw.local.4
    exec.rev_4_elements

    exec.xor_4_elements # compute state[20..24]

    movup.6
    popw.mem

    pushw.mem

    pushw.local.5
    exec.rev_4_elements

    exec.xor_4_elements # compute state[24..28]

    movup.5
    popw.mem

    pushw.mem

    pushw.local.6
    exec.rev_4_elements

    drop
    drop

    pushw.local.4
    exec.rev_4_elements

    drop
    drop

    exec.xor_4_elements # compute state[28..32]

    movup.4
    popw.mem

    pushw.local.2
    dupw

    pushw.mem

    pushw.local.4
    drop
    drop
    swap

    pushw.local.5
    exec.rev_4_elements

    drop
    drop

    exec.xor_4_elements # compute state[32..36]

    movup.7
    popw.mem

    pushw.mem

    pushw.local.5
    drop
    drop
    swap

    pushw.local.6
    exec.rev_4_elements

    drop
    drop

    exec.xor_4_elements # compute state[36..40]

    movup.6
    popw.mem

    pushw.mem

    pushw.local.4
    exec.rev_4_elements

    exec.xor_4_elements # compute state[40..44]

    movup.5
    popw.mem

    pushw.mem

    pushw.local.5
    exec.rev_4_elements

    exec.xor_4_elements # compute state[44..48]

    movup.4
    popw.mem

    pushw.local.3

    repeat.3
        swap
        drop
    end

    dup
    pushw.mem

    pushw.local.6
    exec.rev_4_elements

    exec.xor_4_elements # compute state[48..50]

    movup.4
    popw.mem
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ρ ( rho ) function, which is
# implemented in terms of 32 -bit word size; see https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L115-L147
proc.rho.4
    popw.local.0
    popw.local.1
    popw.local.2
    popw.local.3

    pushw.local.0
    dupw

    pushw.mem
    exec.rev_4_elements

    u32checked_rotl.1
    swap

    exec.rev_4_elements

    movup.7
    popw.mem # wrote state[0..4]

    pushw.mem

    u32checked_rotl.31
    swap
    u32checked_rotl.31
    swap

    exec.rev_4_elements

    u32checked_rotl.14
    swap
    u32checked_rotl.14
    swap

    exec.rev_4_elements

    movup.6
    popw.mem # wrote state[4..8]

    pushw.mem

    u32checked_rotl.13
    swap
    u32checked_rotl.14

    exec.rev_4_elements

    u32checked_rotl.18
    swap
    u32checked_rotl.18
    swap

    exec.rev_4_elements

    movup.5
    popw.mem # wrote state[8..12]

    pushw.mem

    u32checked_rotl.22
    swap
    u32checked_rotl.22
    swap

    exec.rev_4_elements

    u32checked_rotl.3
    swap
    u32checked_rotl.3
    swap

    exec.rev_4_elements

    movup.4
    popw.mem # wrote state[12..16]

    pushw.local.1
    dupw

    pushw.mem

    u32checked_rotl.27
    swap
    u32checked_rotl.28

    exec.rev_4_elements

    u32checked_rotl.10
    swap
    u32checked_rotl.10
    swap

    exec.rev_4_elements

    movup.7
    popw.mem # wrote state[16..20]

    pushw.mem

    u32checked_rotl.1
    swap
    u32checked_rotl.2

    exec.rev_4_elements

    u32checked_rotl.5
    swap
    u32checked_rotl.5
    swap

    exec.rev_4_elements

    movup.6
    popw.mem # wrote state[20..24]

    pushw.mem

    u32checked_rotl.21
    swap
    u32checked_rotl.22

    exec.rev_4_elements

    u32checked_rotl.13
    swap
    u32checked_rotl.12

    exec.rev_4_elements

    movup.5
    popw.mem # wrote state[24..28]

    pushw.mem

    u32checked_rotl.19
    swap
    u32checked_rotl.20

    exec.rev_4_elements

    u32checked_rotl.21
    swap
    u32checked_rotl.20

    exec.rev_4_elements

    movup.4
    popw.mem # wrote state[28..32]

    pushw.local.2
    dupw

    pushw.mem

    u32checked_rotl.22
    swap
    u32checked_rotl.23

    exec.rev_4_elements

    u32checked_rotl.8
    swap
    u32checked_rotl.7

    exec.rev_4_elements

    movup.7
    popw.mem # wrote state[32..36]

    pushw.mem

    u32checked_rotl.10
    swap
    u32checked_rotl.11

    exec.rev_4_elements

    u32checked_rotl.4
    swap
    u32checked_rotl.4
    swap

    exec.rev_4_elements

    movup.6
    popw.mem # wrote state[36..40]

    pushw.mem

    u32checked_rotl.9
    swap
    u32checked_rotl.9
    swap

    exec.rev_4_elements

    u32checked_rotl.1
    swap
    u32checked_rotl.1
    swap

    exec.rev_4_elements

    movup.5
    popw.mem # wrote state[40..44]

    pushw.mem

    u32checked_rotl.30
    swap
    u32checked_rotl.31

    exec.rev_4_elements

    u32checked_rotl.28
    swap
    u32checked_rotl.28
    swap

    exec.rev_4_elements

    movup.4
    popw.mem # wrote state[44..48]

    pushw.local.3

    repeat.3
        swap
        drop
    end

    dup

    pushw.mem

    u32checked_rotl.7
    swap
    u32checked_rotl.7
    swap

    movup.4
    popw.mem # wrote state[48..50]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's π function, which is
# implemented in terms of 32 -bit word size; see https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L169-L207
proc.pi.17
    popw.local.0
    popw.local.1
    popw.local.2
    popw.local.3

    pushw.local.0
    repeat.2
        swap
        drop
    end

    swap
    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    movup.2
    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    popw.local.4 # wrote state[0..4]

    pushw.local.2

    drop
    repeat.2
        swap
        drop
    end

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    pushw.local.1

    drop
    drop
    swap
    drop

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    popw.local.5 # wrote state[4..8]

    pushw.local.0

    drop
    repeat.2
        swap
        drop
    end

    pushw.mem

    drop
    drop

    pushw.local.3

    repeat.3
        swap
        drop
    end

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    popw.local.6 # wrote state[8..12]

    pushw.local.1

    exec.rev_4_elements

    drop
    drop

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    movup.2

    pushw.mem

    drop
    drop

    popw.local.7 # wrote state[12..16]

    pushw.local.2

    repeat.2
        swap
        drop
    end

    swap

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    movup.2

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    popw.local.8 # wrote state[16..20]

    pushw.local.0

    repeat.2
        swap
        drop
    end

    swap

    pushw.mem

    drop
    drop

    movup.2

    pushw.mem

    drop
    drop

    popw.local.9 # wrote state[20..24]

    pushw.local.2

    drop
    repeat.2
        swap
        drop
    end

    pushw.mem

    drop
    drop

    pushw.local.1

    drop
    drop
    swap
    drop

    pushw.mem

    drop
    drop

    popw.local.10 # wrote state[24..28]

    pushw.local.0

    drop
    drop
    swap
    drop

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    pushw.local.2

    drop
    drop
    swap
    drop

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    popw.local.11 # wrote state[28..32]

    pushw.local.1

    drop
    repeat.2
        swap
        drop
    end

    pushw.mem

    drop
    drop

    pushw.local.0

    drop
    drop
    swap
    drop

    pushw.mem

    drop
    drop

    popw.local.12 # wrote state[32..36]

    pushw.local.2

    repeat.2
        swap
        drop
    end

    swap

    pushw.mem

    drop
    drop

    movup.2

    pushw.mem

    drop
    drop

    popw.local.13 # wrote state[36..40]

    pushw.local.1

    repeat.3
        swap
        drop
    end

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    pushw.local.0

    drop
    repeat.2
        swap
        drop
    end

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    popw.local.14 # wrote state[40..44]

    pushw.local.1

    drop
    drop
    drop

    pushw.mem

    popw.local.15 # wrote state[44..48]

    pushw.local.2

    drop
    drop
    swap
    drop

    pushw.mem

    drop
    drop
    push.0.0

    exec.rev_4_elements

    swap

    popw.local.16 # wrote state[48..50]

    pushw.local.0

    pushw.local.4
    movup.4
    storew.mem # final write state[0..4]

    loadw.local.5
    movup.4
    storew.mem # final write state[4..8]

    loadw.local.6
    movup.4
    storew.mem # final write state[8..12]

    loadw.local.7
    movup.4
    storew.mem # final write state[12..16]

    loadw.local.1

    pushw.local.8
    movup.4
    storew.mem # final write state[16..20]

    loadw.local.9
    movup.4
    storew.mem # final write state[20..24]

    loadw.local.10
    movup.4
    storew.mem # final write state[24..28]

    loadw.local.11
    movup.4
    storew.mem # final write state[28..32]

    loadw.local.2

    pushw.local.12
    movup.4
    storew.mem # final write state[32..36]

    loadw.local.13
    movup.4
    storew.mem # final write state[36..40]

    loadw.local.14
    movup.4
    storew.mem # final write state[40..44]

    loadw.local.15
    movup.4
    storew.mem # final write state[44..48]

    loadw.local.16

    pushw.local.3
    repeat.3
        swap
        drop
    end

    storew.mem # final write state[48..50]
    dropw
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's χ function, which is
# implemented in terms of 32 -bit word size; see https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L233-L271
proc.chi.7
    popw.local.0
    popw.local.1
    popw.local.2
    popw.local.3

    pushw.local.0

    exec.rev_4_elements

    drop
    drop

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    movup.2

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and
    swap

    pushw.local.0

    drop
    repeat.2
        swap
        drop
    end

    pushw.mem

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    exec.rev_4_elements
    swap

    popw.local.4 # write to c[0..4]

    pushw.local.0

    drop
    movup.2
    drop

    swap

    pushw.mem

    exec.rev_4_elements

    drop
    drop
    swap

    movup.2

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    pushw.local.0

    swap
    drop
    movup.2
    drop
    swap

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    u32checked_not
    swap
    u32checked_not

    movup.2
    pushw.mem

    exec.rev_4_elements

    drop
    drop

    movup.3
    u32checked_and

    swap
    movup.2
    u32checked_and

    swap
    exec.rev_4_elements

    popw.local.5 # write to c[4..8]

    pushw.local.0

    repeat.3
        swap
        drop
    end

    pushw.mem

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    push.0.0
    exec.rev_4_elements

    popw.local.6 # write to c[8..10]

    pushw.local.0

    movup.3
    drop

    dup
    pushw.mem
    pushw.local.4

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4

    popw.mem # write to state[0..4]

    dup
    pushw.mem
    pushw.local.5

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4

    popw.mem # write to state[4..8]

    dup
    pushw.mem
    pushw.local.6

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4

    popw.mem # write to state[8..10]

    pushw.local.0

    drop
    drop
    drop

    pushw.mem

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    swap
    push.0.0

    popw.local.4 # write to c[0..2]

    pushw.local.1

    repeat.3
        swap
        drop
    end

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    pushw.local.0

    drop
    drop
    drop

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    movup.3
    u32checked_and

    swap
    movup.2
    u32checked_and

    pushw.local.1

    repeat.3
        swap
        drop
    end

    pushw.mem

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    exec.rev_4_elements
    popw.local.5 # write to c[2..6]

    pushw.local.1

    repeat.3
        swap
        drop
    end

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    pushw.local.0

    drop
    drop
    swap
    drop

    pushw.mem

    drop
    drop

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    pushw.local.0

    drop
    drop

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    movup.3
    u32checked_and

    swap
    movup.2
    u32checked_and
    swap

    exec.rev_4_elements
    popw.local.6 # write to c[6..10]

    pushw.local.0

    drop
    drop

    dup
    pushw.mem

    pushw.local.4

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[10..12]

    dup
    pushw.mem

    pushw.local.5

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[12..16]

    pushw.local.1

    repeat.3
        swap
        drop
    end

    dup
    pushw.mem

    pushw.local.6

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[16..20]

    pushw.local.1

    drop
    movup.2
    drop

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    movup.3
    u32checked_and

    swap
    movup.2
    u32checked_and
    swap

    pushw.local.1

    drop
    drop
    swap
    drop

    pushw.mem

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    exec.rev_4_elements
    popw.local.4 # write to c[0..4]

    pushw.local.1

    drop
    drop

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    movup.3
    u32checked_and

    swap
    movup.2
    u32checked_and
    swap

    pushw.local.1

    drop
    drop
    drop

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    u32checked_not
    swap
    u32checked_not

    pushw.local.1

    drop
    repeat.2
        swap
        drop
    end

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    movup.3
    u32checked_and

    swap
    movup.2
    u32checked_and
    swap

    exec.rev_4_elements
    popw.local.5 # write to c[4..8]

    pushw.local.1

    drop
    repeat.2
        swap
        drop
    end

    pushw.mem

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    push.0.0
    exec.rev_4_elements

    popw.local.6 # write to c[8..10]

    pushw.local.1

    drop

    dup
    pushw.mem

    pushw.local.4

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[20..24]

    dup
    pushw.mem

    pushw.local.5

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[24..28]

    dup
    pushw.mem

    pushw.local.6

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[28..30]

    pushw.local.2

    repeat.3
        swap
        drop
    end

    pushw.mem

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and
    swap

    push.0.0
    popw.local.4 # write to c[0..2]

    pushw.local.2
    movup.2
    drop
    movup.2
    drop

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    dup.2

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    movup.3
    u32checked_and

    swap
    movup.2
    u32checked_and
    swap

    movup.2
    pushw.mem

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    exec.rev_4_elements
    popw.local.5 # write to c[2..6]

    pushw.local.2

    drop
    repeat.2
        swap
        drop
    end

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    pushw.local.1

    drop
    drop
    drop

    pushw.mem

    drop
    drop

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    pushw.local.1

    drop
    drop
    drop

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    pushw.local.2

    repeat.3
        swap
        drop
    end

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    movup.3
    u32checked_and

    swap
    movup.2
    u32checked_and
    swap

    exec.rev_4_elements
    popw.local.6 # write to c[6..10]

    pushw.local.1

    drop
    drop
    drop

    dup

    pushw.mem

    pushw.local.4

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[30..32]

    pushw.local.2

    exec.rev_4_elements

    drop
    drop
    swap

    dup
    pushw.mem

    pushw.local.5

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[32..36]

    dup
    pushw.mem

    pushw.local.6

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[36..40]

    pushw.local.2

    drop
    drop

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    movup.3
    u32checked_and

    swap
    movup.2
    u32checked_and
    swap

    pushw.local.2

    drop
    drop
    drop

    pushw.mem

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    exec.rev_4_elements
    popw.local.4 # write to c[0..4]

    pushw.local.2

    drop
    drop
    drop

    pushw.mem

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    pushw.local.3

    repeat.3
        swap
        drop
    end

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    movup.3
    u32checked_and

    swap
    movup.2
    u32checked_and
    swap

    pushw.local.3

    repeat.3
        swap
        drop
    end

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    u32checked_not
    swap
    u32checked_not

    pushw.local.2

    drop
    drop
    swap
    drop

    pushw.mem

    exec.rev_4_elements

    drop
    drop

    movup.3
    u32checked_and

    swap
    movup.2
    u32checked_and
    swap

    exec.rev_4_elements
    popw.local.5 # write to c[4..8]

    pushw.local.2

    drop
    drop
    swap
    drop

    pushw.mem

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    u32checked_and

    swap
    movup.2
    u32checked_and

    push.0.0

    exec.rev_4_elements
    popw.local.6 # write to c[8..10]

    pushw.local.2

    drop
    drop

    dup
    pushw.mem

    pushw.local.4

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[40..44]

    dup
    pushw.mem

    pushw.local.5

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[44..48]

    pushw.local.3

    repeat.3
        swap
        drop
    end

    dup
    pushw.mem

    pushw.local.6

    exec.rev_4_elements
    exec.xor_4_elements

    movup.4
    popw.mem # write to state[48..50]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 0u) as template arguments
proc.iota_round_1
    dup
    pushw.mem

    push.1
    u32checked_xor

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 137u) as template arguments
proc.iota_round_2
    dup
    pushw.mem

    swap

    push.137
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 2147483787u) as template arguments
proc.iota_round_3
    dup
    pushw.mem

    swap

    push.2147483787
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 2147516544u) as template arguments
proc.iota_round_4
    dup
    pushw.mem

    swap

    push.2147516544
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 139u) as template arguments
proc.iota_round_5
    dup
    pushw.mem

    push.1
    u32checked_xor

    swap

    push.139
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 32768u) as template arguments
proc.iota_round_6
    dup
    pushw.mem

    push.1
    u32checked_xor

    swap

    push.32768
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 2147516552u) as template arguments
proc.iota_round_7
    dup
    pushw.mem

    push.1
    u32checked_xor

    swap

    push.2147516552
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 2147483778u) as template arguments
proc.iota_round_8
    dup
    pushw.mem

    push.1
    u32checked_xor

    swap

    push.2147483778
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 11u) as template arguments
proc.iota_round_9
    dup
    pushw.mem

    swap

    push.11
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 10u) as template arguments
proc.iota_round_10
    dup
    pushw.mem

    swap

    push.10
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 32898u) as template arguments
proc.iota_round_11
    dup
    pushw.mem

    push.1
    u32checked_xor

    swap

    push.32898
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 32771u) as template arguments
proc.iota_round_12
    dup
    pushw.mem

    swap

    push.32771
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 32907u) as template arguments
proc.iota_round_13
    dup
    pushw.mem

    push.1
    u32checked_xor

    swap

    push.32907
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 2147483659u) as template arguments
proc.iota_round_14
    dup
    pushw.mem

    push.1
    u32checked_xor

    swap

    push.2147483659
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 2147483786u) as template arguments
proc.iota_round_15
    dup
    pushw.mem

    push.1
    u32checked_xor

    swap

    push.2147483786
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 2147483777u) as template arguments
proc.iota_round_16
    dup
    pushw.mem

    push.1
    u32checked_xor

    swap

    push.2147483777
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 2147483777u) as template arguments
proc.iota_round_17
    dup
    pushw.mem

    swap

    push.2147483777
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 2147483656u) as template arguments
proc.iota_round_18
    dup
    pushw.mem

    swap

    push.2147483656
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 131u) as template arguments
proc.iota_round_19
    dup
    pushw.mem

    swap

    push.131
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 2147516419u) as template arguments
proc.iota_round_20
    dup
    pushw.mem

    swap

    push.2147516419
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 2147516552u) as template arguments
proc.iota_round_21
    dup
    pushw.mem

    push.1
    u32checked_xor

    swap

    push.2147516552
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 2147483784u) as template arguments
proc.iota_round_22
    dup
    pushw.mem

    swap

    push.2147483784
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (1u, 32768u) as template arguments
proc.iota_round_23
    dup
    pushw.mem

    push.1
    u32checked_xor

    swap

    push.32768
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] | b = 1600, n_r = 24, permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size; imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (0u, 2147516546u) as template arguments
proc.iota_round_24
    dup
    pushw.mem

    swap

    push.2147516546
    u32checked_xor

    swap

    movup.4
    popw.mem # write to state[0..2]
end

# keccak-p[b, n_r] permutation round, without `iota` function
# ( all other functions i.e. `theta`, `rho`, `pi`, `chi` are applied in order ) | b = 1600, n_r = 24
#
# As `iota` function involves xoring constant factors with first lane of state array ( read state[0, 0] ),
# specialised implementations are maintained; see above; required to be invoked seperately after completion of
# this procedure's execution.
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L325-L340
proc.round.4
    storew.local.0
    swapw
    storew.local.1
    movupw.2
    storew.local.2
    movupw.3
    storew.local.3

    # reverse placement order of four VM words
    swapw
    movupw.2
    movupw.3

    exec.theta

    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.rho

    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.pi

    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.chi
end

# keccak-p[1600, 24] permutation, which applies 24 rounds on state array of size  5 x 5 x 64, where each
# 64 -bit lane is represented in bit interleaved form ( in terms of two 32 -bit words ).
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L379-L427
proc.keccak_p.4
    popw.local.0
    popw.local.1
    popw.local.2
    popw.local.3

    # permutation round 1
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_1

    # permutation round 2
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_2

    # permutation round 3
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_3

    # permutation round 4
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_4

    # permutation round 5
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_5

    # permutation round 6
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_6

    # permutation round 7
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_7

    # permutation round 8
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_8

    # permutation round 9
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_9

    # permutation round 10
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_10

    # permutation round 11
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_11

    # permutation round 12
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_12

    # permutation round 13
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_13

    # permutation round 14
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_14

    # permutation round 15
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_15

    # permutation round 16
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_16

    # permutation round 17
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_17

    # permutation round 18
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_18

    # permutation round 19
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_19

    # permutation round 20
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_20

    # permutation round 21
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_21

    # permutation round 22
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_22

    # permutation round 23
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_23

    # permutation round 24
    pushw.local.3
    pushw.local.2
    pushw.local.1
    pushw.local.0

    exec.round

    pushw.local.0

    repeat.3
        swap
        drop
    end

    exec.iota_round_24
end

# given two 32 -bit unsigned integers ( standard form ), representing upper and lower
# portion of a 64 -bit unsigned integer ( actually a keccak-[1600, 24] lane ),
# this function converts them into bit interleaved representation, where two 32 -bit
# unsigned integers ( even portion & then odd portion ) hold bits in even and odd
# indices of 64 -bit unsigned integer ( remember it's represented in terms of
# two 32 -bit elements )
#
# Read more about bit interleaved representation in section 2.1 of https://keccak.team/files/Keccak-implementation-3.2.pdf
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/utils.hpp#L123-L149
export.to_bit_interleaved
    dup.1

    push.1
    u32checked_and

    dup.2
    u32checked_shr.1
    push.1
    u32checked_and

    swap

    dup.3

    u32checked_shr.2
    push.1
    u32checked_and

    u32checked_shl.1
    u32checked_or

    swap

    dup.3

    u32checked_shr.3
    push.1
    u32checked_and

    u32checked_shl.1
    u32checked_or

    swap

    dup.3

    u32checked_shr.4
    push.1
    u32checked_and

    u32checked_shl.2
    u32checked_or

    swap

    dup.3

    u32checked_shr.5
    push.1
    u32checked_and

    u32checked_shl.2
    u32checked_or

    swap

    dup.3

    u32checked_shr.6
    push.1
    u32checked_and

    u32checked_shl.3
    u32checked_or

    swap

    dup.3

    u32checked_shr.7
    push.1
    u32checked_and

    u32checked_shl.3
    u32checked_or

    swap

    dup.3

    u32checked_shr.8
    push.1
    u32checked_and

    u32checked_shl.4
    u32checked_or

    swap

    dup.3

    u32checked_shr.9
    push.1
    u32checked_and

    u32checked_shl.4
    u32checked_or

    swap

    dup.3

    u32checked_shr.10
    push.1
    u32checked_and

    u32checked_shl.5
    u32checked_or

    swap

    dup.3

    u32checked_shr.11
    push.1
    u32checked_and

    u32checked_shl.5
    u32checked_or

    swap

    dup.3

    u32checked_shr.12
    push.1
    u32checked_and

    u32checked_shl.6
    u32checked_or

    swap

    dup.3

    u32checked_shr.13
    push.1
    u32checked_and

    u32checked_shl.6
    u32checked_or

    swap

    dup.3

    u32checked_shr.14
    push.1
    u32checked_and

    u32checked_shl.7
    u32checked_or

    swap

    dup.3

    u32checked_shr.15
    push.1
    u32checked_and

    u32checked_shl.7
    u32checked_or

    swap

    dup.3

    u32checked_shr.16
    push.1
    u32checked_and

    u32checked_shl.8
    u32checked_or

    swap

    dup.3

    u32checked_shr.17
    push.1
    u32checked_and

    u32checked_shl.8
    u32checked_or

    swap

    dup.3

    u32checked_shr.18
    push.1
    u32checked_and

    u32checked_shl.9
    u32checked_or

    swap

    dup.3

    u32checked_shr.19
    push.1
    u32checked_and

    u32checked_shl.9
    u32checked_or

    swap

    dup.3

    u32checked_shr.20
    push.1
    u32checked_and

    u32checked_shl.10
    u32checked_or

    swap

    dup.3

    u32checked_shr.21
    push.1
    u32checked_and

    u32checked_shl.10
    u32checked_or

    swap

    dup.3

    u32checked_shr.22
    push.1
    u32checked_and

    u32checked_shl.11
    u32checked_or

    swap

    dup.3

    u32checked_shr.23
    push.1
    u32checked_and

    u32checked_shl.11
    u32checked_or

    swap

    dup.3

    u32checked_shr.24
    push.1
    u32checked_and

    u32checked_shl.12
    u32checked_or

    swap

    dup.3

    u32checked_shr.25
    push.1
    u32checked_and

    u32checked_shl.12
    u32checked_or

    swap

    dup.3

    u32checked_shr.26
    push.1
    u32checked_and

    u32checked_shl.13
    u32checked_or

    swap

    dup.3

    u32checked_shr.27
    push.1
    u32checked_and

    u32checked_shl.13
    u32checked_or

    swap

    dup.3

    u32checked_shr.28
    push.1
    u32checked_and

    u32checked_shl.14
    u32checked_or

    swap

    dup.3

    u32checked_shr.29
    push.1
    u32checked_and

    u32checked_shl.14
    u32checked_or

    swap

    dup.3

    u32checked_shr.30
    push.1
    u32checked_and

    u32checked_shl.15
    u32checked_or

    swap

    dup.3

    u32checked_shr.31
    push.1
    u32checked_and

    u32checked_shl.15
    u32checked_or

    swap

    dup.2

    push.1
    u32checked_and

    u32checked_shl.16
    u32checked_or

    swap

    dup.2

    u32checked_shr.1
    push.1
    u32checked_and

    u32checked_shl.16
    u32checked_or

    swap

    dup.2

    u32checked_shr.2
    push.1
    u32checked_and

    u32checked_shl.17
    u32checked_or

    swap

    dup.2

    u32checked_shr.3
    push.1
    u32checked_and

    u32checked_shl.17
    u32checked_or

    swap

    dup.2

    u32checked_shr.4
    push.1
    u32checked_and

    u32checked_shl.18
    u32checked_or

    swap

    dup.2

    u32checked_shr.5
    push.1
    u32checked_and

    u32checked_shl.18
    u32checked_or

    swap

    dup.2

    u32checked_shr.6
    push.1
    u32checked_and

    u32checked_shl.19
    u32checked_or

    swap

    dup.2

    u32checked_shr.7
    push.1
    u32checked_and

    u32checked_shl.19
    u32checked_or

    swap

    dup.2

    u32checked_shr.8
    push.1
    u32checked_and

    u32checked_shl.20
    u32checked_or

    swap

    dup.2

    u32checked_shr.9
    push.1
    u32checked_and

    u32checked_shl.20
    u32checked_or

    swap

    dup.2

    u32checked_shr.10
    push.1
    u32checked_and

    u32checked_shl.21
    u32checked_or

    swap

    dup.2

    u32checked_shr.11
    push.1
    u32checked_and

    u32checked_shl.21
    u32checked_or

    swap

    dup.2

    u32checked_shr.12
    push.1
    u32checked_and

    u32checked_shl.22
    u32checked_or

    swap

    dup.2

    u32checked_shr.13
    push.1
    u32checked_and

    u32checked_shl.22
    u32checked_or

    swap

    dup.2

    u32checked_shr.14
    push.1
    u32checked_and

    u32checked_shl.23
    u32checked_or

    swap

    dup.2

    u32checked_shr.15
    push.1
    u32checked_and

    u32checked_shl.23
    u32checked_or

    swap

    dup.2

    u32checked_shr.16
    push.1
    u32checked_and

    u32checked_shl.24
    u32checked_or

    swap

    dup.2

    u32checked_shr.17
    push.1
    u32checked_and

    u32checked_shl.24
    u32checked_or

    swap

    dup.2

    u32checked_shr.18
    push.1
    u32checked_and

    u32checked_shl.25
    u32checked_or

    swap

    dup.2

    u32checked_shr.19
    push.1
    u32checked_and

    u32checked_shl.25
    u32checked_or

    swap

    dup.2

    u32checked_shr.20
    push.1
    u32checked_and

    u32checked_shl.26
    u32checked_or

    swap

    dup.2

    u32checked_shr.21
    push.1
    u32checked_and

    u32checked_shl.26
    u32checked_or

    swap

    dup.2

    u32checked_shr.22
    push.1
    u32checked_and

    u32checked_shl.27
    u32checked_or

    swap

    dup.2

    u32checked_shr.23
    push.1
    u32checked_and

    u32checked_shl.27
    u32checked_or

    swap

    dup.2

    u32checked_shr.24
    push.1
    u32checked_and

    u32checked_shl.28
    u32checked_or

    swap

    dup.2

    u32checked_shr.25
    push.1
    u32checked_and

    u32checked_shl.28
    u32checked_or

    swap

    dup.2

    u32checked_shr.26
    push.1
    u32checked_and

    u32checked_shl.29
    u32checked_or

    swap

    dup.2

    u32checked_shr.27
    push.1
    u32checked_and

    u32checked_shl.29
    u32checked_or

    swap

    dup.2

    u32checked_shr.28
    push.1
    u32checked_and

    u32checked_shl.30
    u32checked_or

    swap

    dup.2

    u32checked_shr.29
    push.1
    u32checked_and

    u32checked_shl.30
    u32checked_or

    swap

    dup.2

    u32checked_shr.30
    push.1
    u32checked_and

    u32checked_shl.31
    u32checked_or

    swap

    dup.2

    u32checked_shr.31
    push.1
    u32checked_and

    u32checked_shl.31
    u32checked_or

    swap
end

# given two 32 -bit unsigned integers ( bit interleaved form ), representing even and odd
# positioned bits of a 64 -bit unsigned integer ( actually a keccak-[1600, 24] lane ),
# this function converts them into standard representation, where two 32 -bit
# unsigned integers hold higher ( 32 -bit ) and lower ( 32 -bit ) bits of standard
# representation of 64 -bit unsigned integer ( remember it's represented in terms of
# two 32 -bit elements )
#
# This function reverts the action done by `to_bit_interleaved` function implemented above.
#
# Read more about bit interleaved representation in section 2.1 of https://keccak.team/files/Keccak-implementation-3.2.pdf
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/utils.hpp#L151-L175
export.from_bit_interleaved
    dup

    push.1
    u32checked_and

    dup.2

    push.1
    u32checked_and

    u32checked_shl.1
    u32checked_or

    dup.1

    u32checked_shr.1
    push.1
    u32checked_and

    u32checked_shl.2
    u32checked_or

    dup.2

    u32checked_shr.1
    push.1
    u32checked_and

    u32checked_shl.3
    u32checked_or

    dup.1

    u32checked_shr.2
    push.1
    u32checked_and

    u32checked_shl.4
    u32checked_or

    dup.2

    u32checked_shr.2
    push.1
    u32checked_and

    u32checked_shl.5
    u32checked_or

    dup.1

    u32checked_shr.3
    push.1
    u32checked_and

    u32checked_shl.6
    u32checked_or

    dup.2

    u32checked_shr.3
    push.1
    u32checked_and

    u32checked_shl.7
    u32checked_or

    dup.1

    u32checked_shr.4
    push.1
    u32checked_and

    u32checked_shl.8
    u32checked_or

    dup.2

    u32checked_shr.4
    push.1
    u32checked_and

    u32checked_shl.9
    u32checked_or

    dup.1

    u32checked_shr.5
    push.1
    u32checked_and

    u32checked_shl.10
    u32checked_or

    dup.2

    u32checked_shr.5
    push.1
    u32checked_and

    u32checked_shl.11
    u32checked_or

    dup.1

    u32checked_shr.6
    push.1
    u32checked_and

    u32checked_shl.12
    u32checked_or

    dup.2

    u32checked_shr.6
    push.1
    u32checked_and

    u32checked_shl.13
    u32checked_or

    dup.1

    u32checked_shr.7
    push.1
    u32checked_and

    u32checked_shl.14
    u32checked_or

    dup.2

    u32checked_shr.7
    push.1
    u32checked_and

    u32checked_shl.15
    u32checked_or

    dup.1

    u32checked_shr.8
    push.1
    u32checked_and

    u32checked_shl.16
    u32checked_or

    dup.2

    u32checked_shr.8
    push.1
    u32checked_and

    u32checked_shl.17
    u32checked_or

    dup.1

    u32checked_shr.9
    push.1
    u32checked_and

    u32checked_shl.18
    u32checked_or

    dup.2

    u32checked_shr.9
    push.1
    u32checked_and

    u32checked_shl.19
    u32checked_or

    dup.1

    u32checked_shr.10
    push.1
    u32checked_and

    u32checked_shl.20
    u32checked_or

    dup.2

    u32checked_shr.10
    push.1
    u32checked_and

    u32checked_shl.21
    u32checked_or

    dup.1

    u32checked_shr.11
    push.1
    u32checked_and

    u32checked_shl.22
    u32checked_or

    dup.2

    u32checked_shr.11
    push.1
    u32checked_and

    u32checked_shl.23
    u32checked_or

    dup.1

    u32checked_shr.12
    push.1
    u32checked_and

    u32checked_shl.24
    u32checked_or

    dup.2

    u32checked_shr.12
    push.1
    u32checked_and

    u32checked_shl.25
    u32checked_or

    dup.1

    u32checked_shr.13
    push.1
    u32checked_and

    u32checked_shl.26
    u32checked_or

    dup.2

    u32checked_shr.13
    push.1
    u32checked_and

    u32checked_shl.27
    u32checked_or

    dup.1

    u32checked_shr.14
    push.1
    u32checked_and

    u32checked_shl.28
    u32checked_or

    dup.2

    u32checked_shr.14
    push.1
    u32checked_and

    u32checked_shl.29
    u32checked_or

    dup.1

    u32checked_shr.15
    push.1
    u32checked_and

    u32checked_shl.30
    u32checked_or

    dup.2

    u32checked_shr.15
    push.1
    u32checked_and

    u32checked_shl.31
    u32checked_or

    dup.1

    u32checked_shr.16
    push.1
    u32checked_and

    dup.3

    u32checked_shr.16
    push.1
    u32checked_and

    u32checked_shl.1
    u32checked_or

    dup.2

    u32checked_shr.17
    push.1
    u32checked_and

    u32checked_shl.2
    u32checked_or

    dup.3

    u32checked_shr.17
    push.1
    u32checked_and

    u32checked_shl.3
    u32checked_or

    dup.2

    u32checked_shr.18
    push.1
    u32checked_and

    u32checked_shl.4
    u32checked_or

    dup.3

    u32checked_shr.18
    push.1
    u32checked_and

    u32checked_shl.5
    u32checked_or

    dup.2

    u32checked_shr.19
    push.1
    u32checked_and

    u32checked_shl.6
    u32checked_or

    dup.3

    u32checked_shr.19
    push.1
    u32checked_and

    u32checked_shl.7
    u32checked_or

    dup.2

    u32checked_shr.20
    push.1
    u32checked_and

    u32checked_shl.8
    u32checked_or

    dup.3

    u32checked_shr.20
    push.1
    u32checked_and

    u32checked_shl.9
    u32checked_or

    dup.2

    u32checked_shr.21
    push.1
    u32checked_and

    u32checked_shl.10
    u32checked_or

    dup.3

    u32checked_shr.21
    push.1
    u32checked_and

    u32checked_shl.11
    u32checked_or

    dup.2

    u32checked_shr.22
    push.1
    u32checked_and

    u32checked_shl.12
    u32checked_or

    dup.3

    u32checked_shr.22
    push.1
    u32checked_and

    u32checked_shl.13
    u32checked_or

    dup.2

    u32checked_shr.23
    push.1
    u32checked_and

    u32checked_shl.14
    u32checked_or

    dup.3

    u32checked_shr.23
    push.1
    u32checked_and

    u32checked_shl.15
    u32checked_or

    dup.2

    u32checked_shr.24
    push.1
    u32checked_and

    u32checked_shl.16
    u32checked_or

    dup.3

    u32checked_shr.24
    push.1
    u32checked_and

    u32checked_shl.17
    u32checked_or

    dup.2

    u32checked_shr.25
    push.1
    u32checked_and

    u32checked_shl.18
    u32checked_or

    dup.3

    u32checked_shr.25
    push.1
    u32checked_and

    u32checked_shl.19
    u32checked_or

    dup.2

    u32checked_shr.26
    push.1
    u32checked_and

    u32checked_shl.20
    u32checked_or

    dup.3

    u32checked_shr.26
    push.1
    u32checked_and

    u32checked_shl.21
    u32checked_or

    dup.2

    u32checked_shr.27
    push.1
    u32checked_and

    u32checked_shl.22
    u32checked_or

    dup.3

    u32checked_shr.27
    push.1
    u32checked_and

    u32checked_shl.23
    u32checked_or

    dup.2

    u32checked_shr.28
    push.1
    u32checked_and

    u32checked_shl.24
    u32checked_or

    dup.3

    u32checked_shr.28
    push.1
    u32checked_and

    u32checked_shl.25
    u32checked_or

    dup.2

    u32checked_shr.29
    push.1
    u32checked_and

    u32checked_shl.26
    u32checked_or

    dup.3

    u32checked_shr.29
    push.1
    u32checked_and

    u32checked_shl.27
    u32checked_or

    dup.2

    u32checked_shr.30
    push.1
    u32checked_and

    u32checked_shl.28
    u32checked_or

    dup.3

    u32checked_shr.30
    push.1
    u32checked_and

    u32checked_shl.29
    u32checked_or

    dup.2

    u32checked_shr.31
    push.1
    u32checked_and

    u32checked_shl.30
    u32checked_or

    dup.3

    u32checked_shr.31
    push.1
    u32checked_and

    u32checked_shl.31
    u32checked_or
end

# given 64 -bytes input ( in terms of sixteen u32 elements on stack top ) to 2-to-1
# keccak256 hash function, this function prepares 5 x 5 x 64 keccak-p[1600, 24] state
# bit array such that each of twenty five 64 -bit wide lane is represented in bit
# interleaved form, using two 32 -bit integers. After completion of execution of
# this function, state array should live in allocated memory ( fifty u32 elements ).
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/keccak_256.hpp#L73-L153
proc.to_state_array.4
    popw.local.0
    popw.local.1
    popw.local.2
    popw.local.3

    exec.to_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap

    exec.rev_4_elements
    swap

    exec.to_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap

    movup.2
    movup.3

    pushw.local.0

    repeat.3
        swap
        drop
    end

    popw.mem # write to state[0..4]

    exec.to_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap

    exec.rev_4_elements
    swap

    exec.to_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap

    movup.2
    movup.3

    pushw.local.0

    drop
    repeat.2
        swap
        drop
    end

    popw.mem # write to state[4..8]

    exec.to_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap

    exec.rev_4_elements
    swap

    exec.to_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap

    movup.2
    movup.3

    pushw.local.0

    drop
    drop
    swap
    drop

    popw.mem # write to state[8..12]

    exec.to_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap

    exec.rev_4_elements
    swap

    exec.to_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap

    movup.2
    movup.3

    pushw.local.0

    drop
    drop
    drop

    popw.mem # write to state[12..16]

    push.0.0.0.1

    pushw.local.1

    repeat.3
        swap
        drop
    end

    popw.mem # write to state[16..20]

    push.0.0.0.0

    pushw.local.1

    drop
    repeat.2
        swap
        drop
    end

    popw.mem # write to state[20..24]

    push.0.0.0.0

    pushw.local.1

    drop
    drop
    swap
    drop

    popw.mem # write to state[24..28]

    push.0.0.0.0

    pushw.local.1

    drop
    drop
    drop

    popw.mem # write to state[28..32]

    push.0.0.2147483648.0

    pushw.local.2

    repeat.3
        swap
        drop
    end

    popw.mem # write to state[32..36]

    push.0.0.0.0

    pushw.local.2

    drop
    repeat.2
        swap
        drop
    end

    popw.mem # write to state[36..40]

    push.0.0.0.0

    pushw.local.2

    drop
    drop
    swap
    drop

    popw.mem # write to state[40..44]

    push.0.0.0.0

    pushw.local.2

    drop
    drop
    drop

    popw.mem # write to state[44..48]

    push.0.0.0.0

    pushw.local.3

    repeat.3
        swap
        drop
    end

    popw.mem # write to state[48..50]
end

# given 32 -bytes digest ( in terms of eight u32 elements on stack top ) in bit interleaved form,
# this function attempts to convert those into standard representation, where eight u32 elements
# live on stack top, each pair of them hold higher and lower bits of 64 -bit unsigned
# integer ( lane of keccak-p[1600, 24] state array )
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/keccak_256.hpp#L180-L209
proc.to_digest
    movup.7
    movup.7

    exec.from_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap

    movup.7
    movup.7

    exec.from_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap

    movup.7
    movup.7

    exec.from_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap

    movup.7
    movup.7

    exec.from_bit_interleaved

    exec.rev_4_elements
    drop
    drop
    swap
end

# given 64 -bytes input, in terms of sixteen 32 -bit unsigned integers, where each pair
# of them holding higher & lower 32 -bits of 64 -bit unsigned integer ( reinterpreted on
# host CPU from little endian byte array ) respectively, this function computes 32 -bytes
# keccak256 digest, held on stack top, represented in terms of eight 32 -bit unsigned integers,
# where each pair of them keeps higher and lower 32 -bits of 64 -bit unsigned integer respectively
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/keccak_256.hpp#L232-L257
export.hash.13
    push.0.0.0
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

    exec.to_state_array

    push.0.0.0
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

    exec.keccak_p

    pushw.local.1
    pushw.local.0

    exec.to_digest
end
"),
// ----- std::crypto::hashes::sha256 --------------------------------------------------------------
("std::crypto::hashes::sha256", "# Given [x, ...] on stack top, this routine computes [y, ...]
# such that y = σ_0(x), as defined in SHA specification
#
# See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L73-L79
proc.small_sigma_0
    dup
    u32unchecked_rotr.7

    swap

    dup
    u32unchecked_rotr.18

    swap

    u32unchecked_shr.3

    u32checked_xor
    u32checked_xor
end

# Given [x, ...] on stack top, this routine computes [y, ...]
# such that y = σ_1(x), as defined in SHA specification
#
# See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L81-L87
proc.small_sigma_1
    dup
    u32unchecked_rotr.17

    swap

    dup
    u32unchecked_rotr.19

    swap

    u32unchecked_shr.10

    u32checked_xor
    u32checked_xor
end

# Given [x, ...] on stack top, this routine computes [y, ...]
# such that y = Σ_0(x), as defined in SHA specification
#
# See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L57-L63
proc.cap_sigma_0
    dup
    u32unchecked_rotr.2

    swap

    dup
    u32unchecked_rotr.13

    swap

    u32unchecked_rotr.22

    u32checked_xor
    u32checked_xor
end

# Given [x, ...] on stack top, this routine computes [y, ...]
# such that y = Σ_1(x), as defined in SHA specification
#
# See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L65-L71
proc.cap_sigma_1
    dup
    u32unchecked_rotr.6

    swap

    dup
    u32unchecked_rotr.11

    swap

    u32unchecked_rotr.25

    u32checked_xor
    u32checked_xor
end

# Given [x, y, z, ...] on stack top, this routine computes [o, ...]
# such that o = ch(x, y, z), as defined in SHA specification
#
# See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L37-L45
proc.ch
    swap
    dup.1
    u32checked_and

    swap
    u32checked_not

    movup.2
    u32checked_and

    u32checked_xor
end

# Given [x, y, z, ...] on stack top, this routine computes [o, ...]
# such that o = maj(x, y, z), as defined in SHA specification
#
# See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L47-L55
proc.maj
    dup.1
    dup.1
    u32checked_and

    swap
    dup.3
    u32checked_and

    movup.2
    movup.3
    u32checked_and

    u32checked_xor
    u32checked_xor
end

# Given [a, b, c, d, ...] on stack top, this routine reverses order of first 
# four elements on stack top such that final stack state looks like [d, c, b, a, ...]
proc.rev_element_order
    swap
    movup.2
    movup.3
end

# Given [a, b, c, d, ...] on stack top, this routine computes next message schedule word
# using following formula
#
# t0 = small_sigma_1(a) + b
# t1 = small_sigma_0(c) + d
# return t0 + t1
#
# If to be computed message schedule word has index i ∈ [16, 64), then 
# a, b, c, d will have following indices in message schedule
#
# a = msg[i - 2]
# b = msg[i - 7]
# c = msg[i - 15]
# d = msg[i - 16]
proc.compute_message_schedule_word
    exec.small_sigma_1
    movup.2
    exec.small_sigma_0

    u32overflowing_add3
    drop
    u32wrapping_add
end

# Given eight working variables of SHA256 ( i.e. hash state ), a 32 -bit round constant & 
# 32 -bit message word on stack top, this routine consumes constant & message word into 
# hash state.
#
# Expected stack state looks like
#
# [a, b, c, d, e, f, g, h, CONST_i, WORD_i] | i ∈ [0, 64)
#
# After finishing execution, stack looks like
#
# [a', b', c', d', e', f', g', h']
#
# See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2_256.hpp#L165-L175
proc.consume_message_word
    dup.6
    dup.6
    dup.6
    exec.ch

    movup.9
    movup.10

    u32overflowing_add3
    drop

    dup.5
    exec.cap_sigma_1

    movup.9
    u32overflowing_add3
    drop

    dup.3
    dup.3
    dup.3
    exec.maj

    dup.2
    exec.cap_sigma_0

    u32wrapping_add

    movup.5
    dup.2
    u32wrapping_add
    movdn.5

    u32wrapping_add
end

# Given 32 -bytes hash state ( in terms of 8 SHA256 words ) and 64 -bytes input 
# message ( in terms of 16 SHA256 words ) on stack top, this routine computes
# whole message schedule of 64 message words and consumes them into hash state.
#
# Expected stack state:
#
# [state0, state1, state2, state3, state4, state5, state6, state7, msg0, msg1, msg2, msg3, msg4, msg5, msg6, msg7, msg8, msg9, msg10, msg11, msg12, msg13, msg14, msg15]
#
# Final stack state after completion of execution
#
# [state0', state1', state2', state3', state4', state5', state6', state7']
#
# Note, each SHA256 word is 32 -bit wide
#
# See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L89-L113
# & https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2_256.hpp#L148-L187 ( loop body execution when i = 0 )
proc.prepare_message_schedule_and_consume.2
    popw.local.0
    popw.local.1

    dup.15
    dup.15

    dup.11
    swap
    dup.4
    dup.4
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[16]

    swap
    dup.12
    swap
    dup.5
    dup.5
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[17]

    dup.1
    dup.14
    swap
    dup.7
    dup.7
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[18]

    dup.15
    dup.2
    dup.9
    dup.9
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[19]

    swapw

    push.0x428a2f98
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[0]

    push.0x71374491
    movdn.8
    exec.consume_message_word # consume msg[1]

    push.0xb5c0fbcf
    movdn.8
    exec.consume_message_word # consume msg[2]

    push.0xe9b5dba5
    movdn.8
    exec.consume_message_word # consume msg[3]

    popw.local.0
    popw.local.1

    dup.15
    dup.15
    dup.15

    dup.4
    dup.9
    dup.9
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[20]

    swap
    dup.3
    dup.10
    dup.10
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[21]

    movup.2
    dup.2
    dup.11
    dup.11
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[22]

    dup.6
    dup.2
    dup.13
    dup.13
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[23]

    movupw.2

    push.0x3956c25b
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[4]

    push.0x59f111f1
    movdn.8
    exec.consume_message_word # consume msg[5]

    push.0x923f82a4
    movdn.8
    exec.consume_message_word # consume msg[6]

    push.0xab1c5ed5
    movdn.8
    exec.consume_message_word # consume msg[7]

    popw.local.0
    popw.local.1

    dup.6
    dup.2
    dup.11
    dup.11
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[24]

    dup.6
    dup.2
    dup.13
    dup.13
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[25]

    dup.6
    dup.2
    dup.15
    dup.15
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[26]

    dup.15
    dup.15
    swap
    dup.8
    dup.4
    exec.compute_message_schedule_word # computed msg[27]

    movupw.3

    push.0xd807aa98
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[8]

    push.0x12835b01
    movdn.8
    exec.consume_message_word # consume msg[9]

    push.0x243185be
    movdn.8
    exec.consume_message_word # consume msg[10]

    push.0x550c7dc3
    movdn.8
    exec.consume_message_word # consume msg[11]

    popw.local.0
    popw.local.1

    movupw.3
    movupw.3

    dup.14
    dup.10
    dup.7
    dup.7
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[28]

    dup.14
    dup.10
    dup.9
    dup.9
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[29]

    dup.14
    dup.2
    dup.11
    dup.11
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[30]

    dup.14
    dup.2
    dup.8
    dup.13
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[31]

    movupw.2

    push.0x72be5d74
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[12]

    push.0x80deb1fe
    movdn.8
    exec.consume_message_word # consume msg[13]

    push.0x9bdc06a7
    movdn.8
    exec.consume_message_word # consume msg[14]

    push.0xc19bf174
    movdn.8
    exec.consume_message_word # consume msg[15]

    popw.local.0
    popw.local.1

    movupw.3

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[32]

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[33]

    dup.14
    dup.2
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[34]

    dup.10
    dup.2
    dup.8
    dup.14
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[35]

    movupw.3
    exec.rev_element_order

    push.0xe49b69c1
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[16]

    push.0xefbe4786
    movdn.8
    exec.consume_message_word # consume msg[17]

    push.0x0fc19dc6
    movdn.8
    exec.consume_message_word # consume msg[18]

    push.0x240ca1cc
    movdn.8
    exec.consume_message_word # consume msg[19]

    popw.local.0
    popw.local.1

    movupw.3

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[36]

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[37]

    dup.14
    dup.2
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[38]

    dup.10
    dup.2
    dup.8
    dup.14
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[39]

    movupw.3
    exec.rev_element_order

    push.0x2de92c6f
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[20]

    push.0x4a7484aa
    movdn.8
    exec.consume_message_word # consume msg[21]

    push.0x5cb0a9dc
    movdn.8
    exec.consume_message_word # consume msg[22]

    push.0x76f988da
    movdn.8
    exec.consume_message_word # consume msg[23]

    popw.local.0
    popw.local.1

    movupw.3

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[40]

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[41]

    dup.14
    dup.2
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[42]

    dup.10
    dup.2
    dup.13
    dup.9
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[43]

    movupw.3
    exec.rev_element_order

    push.0x983e5152
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[24]

    push.0xa831c66d
    movdn.8
    exec.consume_message_word # consume msg[25]

    push.0xb00327c8
    movdn.8
    exec.consume_message_word # consume msg[26]

    push.0xbf597fc7
    movdn.8
    exec.consume_message_word # consume msg[27]

    popw.local.0
    popw.local.1

    movupw.3

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[44]

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[45]

    dup.14
    dup.2
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[46]

    dup.10
    dup.2
    dup.8
    dup.14
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[47]

    movupw.3
    exec.rev_element_order

    push.0xc6e00bf3
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[28]

    push.0xd5a79147
    movdn.8
    exec.consume_message_word # consume msg[29]

    push.0x06ca6351
    movdn.8
    exec.consume_message_word # consume msg[30]

    push.0x14292967
    movdn.8
    exec.consume_message_word # consume msg[31]

    popw.local.0
    popw.local.1

    movupw.3

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[48]

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[49]

    dup.14
    dup.2
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[50]

    dup.10
    dup.2
    dup.8
    dup.14
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[51]

    movupw.3
    exec.rev_element_order

    push.0x27b70a85
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[32]

    push.0x2e1b2138
    movdn.8
    exec.consume_message_word # consume msg[33]

    push.0x4d2c6dfc
    movdn.8
    exec.consume_message_word # consume msg[34]

    push.0x53380d13
    movdn.8
    exec.consume_message_word # consume msg[35]

    popw.local.0
    popw.local.1

    movupw.3

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[52]

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[53]

    dup.14
    dup.2
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[54]

    dup.10
    dup.2
    dup.8
    dup.14
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[55]

    movupw.3
    exec.rev_element_order

    push.0x650a7354
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[36]

    push.0x766a0abb
    movdn.8
    exec.consume_message_word # consume msg[37]

    push.0x81c2c92e
    movdn.8
    exec.consume_message_word # consume msg[38]

    push.0x92722c85
    movdn.8
    exec.consume_message_word # consume msg[39]

    popw.local.0
    popw.local.1

    movupw.3

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[56]

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[57]

    dup.14
    dup.2
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[58]

    dup.10
    dup.2
    dup.8
    dup.14
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[59]

    movupw.3
    exec.rev_element_order

    push.0xa2bfe8a1
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[40]

    push.0xa81a664b
    movdn.8
    exec.consume_message_word # consume msg[41]

    push.0xc24b8b70
    movdn.8
    exec.consume_message_word # consume msg[42]

    push.0xc76c51a3
    movdn.8
    exec.consume_message_word # consume msg[43]

    popw.local.0
    popw.local.1

    movupw.3

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[60]

    dup.14
    dup.6
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[61]

    dup.14
    dup.2
    dup.13
    dup.13
    movdn.3
    movdn.3
    exec.compute_message_schedule_word # computed msg[62]

    dup.10
    dup.2
    dup.8
    dup.14
    movdn.3
    movdn.2
    exec.compute_message_schedule_word # computed msg[63]

    movupw.3
    exec.rev_element_order

    push.0xd192e819
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[44]

    push.0xd6990624
    movdn.8
    exec.consume_message_word # consume msg[45]

    push.0xf40e3585
    movdn.8
    exec.consume_message_word # consume msg[46]

    push.0x106aa070
    movdn.8
    exec.consume_message_word # consume msg[47]

    popw.local.0
    popw.local.1

    movupw.2
    movupw.3
    movupw.3

    exec.rev_element_order

    push.0x19a4c116
    pushw.local.1
    pushw.local.0
    exec.consume_message_word # consume msg[48]

    push.0x1e376c08
    movdn.8
    exec.consume_message_word # consume msg[49]

    push.0x2748774c
    movdn.8
    exec.consume_message_word # consume msg[50]

    push.0x34b0bcb5
    movdn.8
    exec.consume_message_word # consume msg[51]

    movupw.2
    exec.rev_element_order
    movdnw.2

    push.0x391c0cb3
    movdn.8
    exec.consume_message_word # consume msg[52]

    push.0x4ed8aa4a
    movdn.8
    exec.consume_message_word # consume msg[53]

    push.0x5b9cca4f
    movdn.8
    exec.consume_message_word # consume msg[54]

    push.0x682e6ff3
    movdn.8
    exec.consume_message_word # consume msg[55]

    movupw.2
    exec.rev_element_order
    movdnw.2

    push.0x748f82ee
    movdn.8
    exec.consume_message_word # consume msg[56]

    push.0x78a5636f
    movdn.8
    exec.consume_message_word # consume msg[57]

    push.0x84c87814
    movdn.8
    exec.consume_message_word # consume msg[58]

    push.0x8cc70208
    movdn.8
    exec.consume_message_word # consume msg[59]

    movupw.2
    exec.rev_element_order
    movdnw.2

    push.0x90befffa
    movdn.8
    exec.consume_message_word # consume msg[60]

    push.0xa4506ceb
    movdn.8
    exec.consume_message_word # consume msg[61]

    push.0xbef9a3f7
    movdn.8
    exec.consume_message_word # consume msg[62]

    push.0xc67178f2
    movdn.8
    exec.consume_message_word # consume msg[63]

    push.0x6a09e667
    u32wrapping_add

    swap
    push.0xbb67ae85
    u32wrapping_add
    swap

    movup.2
    push.0x3c6ef372
    u32wrapping_add
    movdn.2

    movup.3
    push.0xa54ff53a
    u32wrapping_add
    movdn.3

    movup.4
    push.0x510e527f
    u32wrapping_add
    movdn.4

    movup.5
    push.0x9b05688c
    u32wrapping_add
    movdn.5

    movup.6
    push.0x1f83d9ab
    u32wrapping_add
    movdn.6

    movup.7
    push.0x5be0cd19
    u32wrapping_add
    movdn.7
end

# Given 32 -bytes hash state ( in terms of 8 SHA256 words ) and precomputed message 
# schedule of padding bytes ( in terms of 64 message words ), this routine consumes
# that into hash state, leaving final hash state, which is 32 -bytes SHA256 digest.
#
# Note, in SHA256 2-to-1 hashing, 64 -bytes are padded, which is processed as second message
# block ( each SHA256 message block is 64 -bytes wide ). That message block is used for generating 
# message schedule of 64 SHA256 words. That's exactly what can be precomputed & is consumed here 
# ( in this routine ) into provided hash state.
#
# Expected stack state:
#
# [state0, state1, state2, state3, state4, state5, state6, state7, ...]
#
# Final stack state after completion of execution
#
# [state0', state1', state2', state3', state4', state5', state6', state7']
#
# Note, each SHA256 word is 32 -bit wide
#
# See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2_256.hpp#L148-L187 ( loop 
# body execution when i = 1 i.e. consuming padding bytes )
proc.consume_padding_message_schedule
    dupw.1
    dupw.1

    push.2147483648
    movdn.8
    push.0x428a2f98
    movdn.8
    exec.consume_message_word # consume msg[0]

    push.0
    movdn.8
    push.0x71374491
    movdn.8
    exec.consume_message_word # consume msg[1]

    push.0
    movdn.8
    push.0xb5c0fbcf
    movdn.8
    exec.consume_message_word # consume msg[2]

    push.0
    movdn.8
    push.0xe9b5dba5
    movdn.8
    exec.consume_message_word # consume msg[3]

    push.0
    movdn.8
    push.0x3956c25b
    movdn.8
    exec.consume_message_word # consume msg[4]

    push.0
    movdn.8
    push.0x59f111f1
    movdn.8
    exec.consume_message_word # consume msg[5]

    push.0
    movdn.8
    push.0x923f82a4
    movdn.8
    exec.consume_message_word # consume msg[6]

    push.0
    movdn.8
    push.0xab1c5ed5
    movdn.8
    exec.consume_message_word # consume msg[7]

    push.0
    movdn.8
    push.0xd807aa98
    movdn.8
    exec.consume_message_word # consume msg[8]

    push.0
    movdn.8
    push.0x12835b01
    movdn.8
    exec.consume_message_word # consume msg[9]

    push.0
    movdn.8
    push.0x243185be
    movdn.8
    exec.consume_message_word # consume msg[10]

    push.0
    movdn.8
    push.0x550c7dc3
    movdn.8
    exec.consume_message_word # consume msg[11]

    push.0
    movdn.8
    push.0x72be5d74
    movdn.8
    exec.consume_message_word # consume msg[12]

    push.0
    movdn.8
    push.0x80deb1fe
    movdn.8
    exec.consume_message_word # consume msg[13]

    push.0
    movdn.8
    push.0x9bdc06a7
    movdn.8
    exec.consume_message_word # consume msg[14]

    push.512
    movdn.8
    push.0xc19bf174
    movdn.8
    exec.consume_message_word # consume msg[15]

    push.2147483648
    movdn.8
    push.0xe49b69c1
    movdn.8
    exec.consume_message_word # consume msg[16]

    push.20971520
    movdn.8
    push.0xefbe4786
    movdn.8
    exec.consume_message_word # consume msg[17]

    push.2117632
    movdn.8
    push.0x0fc19dc6
    movdn.8
    exec.consume_message_word # consume msg[18]

    push.20616
    movdn.8
    push.0x240ca1cc
    movdn.8
    exec.consume_message_word # consume msg[19]

    push.570427392
    movdn.8
    push.0x2de92c6f
    movdn.8
    exec.consume_message_word # consume msg[20]

    push.575995924
    movdn.8
    push.0x4a7484aa
    movdn.8
    exec.consume_message_word # consume msg[21]

    push.84449090
    movdn.8
    push.0x5cb0a9dc
    movdn.8
    exec.consume_message_word # consume msg[22]

    push.2684354592
    movdn.8
    push.0x76f988da
    movdn.8
    exec.consume_message_word # consume msg[23]

    push.1518862336
    movdn.8
    push.0x983e5152
    movdn.8
    exec.consume_message_word # consume msg[24]

    push.6067200
    movdn.8
    push.0xa831c66d
    movdn.8
    exec.consume_message_word # consume msg[25]

    push.1496221
    movdn.8
    push.0xb00327c8
    movdn.8
    exec.consume_message_word # consume msg[26]

    push.4202700544
    movdn.8
    push.0xbf597fc7
    movdn.8
    exec.consume_message_word # consume msg[27]

    push.3543279056
    movdn.8
    push.0xc6e00bf3
    movdn.8
    exec.consume_message_word # consume msg[28]

    push.291985753
    movdn.8
    push.0xd5a79147
    movdn.8
    exec.consume_message_word # consume msg[29]

    push.4142317530
    movdn.8
    push.0x06ca6351
    movdn.8
    exec.consume_message_word # consume msg[30]

    push.3003913545
    movdn.8
    push.0x14292967
    movdn.8
    exec.consume_message_word # consume msg[31]

    push.145928272
    movdn.8
    push.0x27b70a85
    movdn.8
    exec.consume_message_word # consume msg[32]

    push.2642168871
    movdn.8
    push.0x2e1b2138
    movdn.8
    exec.consume_message_word # consume msg[33]

    push.216179603
    movdn.8
    push.0x4d2c6dfc
    movdn.8
    exec.consume_message_word # consume msg[34]

    push.2296832490
    movdn.8
    push.0x53380d13
    movdn.8
    exec.consume_message_word # consume msg[35]

    push.2771075893
    movdn.8
    push.0x650a7354
    movdn.8
    exec.consume_message_word # consume msg[36]

    push.1738633033
    movdn.8
    push.0x766a0abb
    movdn.8
    exec.consume_message_word # consume msg[37]

    push.3610378607
    movdn.8
    push.0x81c2c92e
    movdn.8
    exec.consume_message_word # consume msg[38]

    push.1324035729
    movdn.8
    push.0x92722c85
    movdn.8
    exec.consume_message_word # consume msg[39]

    push.1572820453
    movdn.8
    push.0xa2bfe8a1
    movdn.8
    exec.consume_message_word # consume msg[40]

    push.2397971253
    movdn.8
    push.0xa81a664b
    movdn.8
    exec.consume_message_word # consume msg[41]

    push.3803995842
    movdn.8
    push.0xc24b8b70
    movdn.8
    exec.consume_message_word # consume msg[42]

    push.2822718356
    movdn.8
    push.0xc76c51a3
    movdn.8
    exec.consume_message_word # consume msg[43]

    push.1168996599
    movdn.8
    push.0xd192e819
    movdn.8
    exec.consume_message_word # consume msg[44]

    push.921948365
    movdn.8
    push.0xd6990624
    movdn.8
    exec.consume_message_word # consume msg[45]

    push.3650881000
    movdn.8
    push.0xf40e3585
    movdn.8
    exec.consume_message_word # consume msg[46]

    push.2958106055
    movdn.8
    push.0x106aa070
    movdn.8
    exec.consume_message_word # consume msg[47]

    push.1773959876
    movdn.8
    push.0x19a4c116
    movdn.8
    exec.consume_message_word # consume msg[48]

    push.3172022107
    movdn.8
    push.0x1e376c08
    movdn.8
    exec.consume_message_word # consume msg[49]

    push.3820646885
    movdn.8
    push.0x2748774c
    movdn.8
    exec.consume_message_word # consume msg[50]

    push.991993842
    movdn.8
    push.0x34b0bcb5
    movdn.8
    exec.consume_message_word # consume msg[51]

    push.419360279
    movdn.8
    push.0x391c0cb3
    movdn.8
    exec.consume_message_word # consume msg[52]

    push.3797604839
    movdn.8
    push.0x4ed8aa4a
    movdn.8
    exec.consume_message_word # consume msg[53]

    push.322392134
    movdn.8
    push.0x5b9cca4f
    movdn.8
    exec.consume_message_word # consume msg[54]

    push.85264541
    movdn.8
    push.0x682e6ff3
    movdn.8
    exec.consume_message_word # consume msg[55]

    push.1326255876
    movdn.8
    push.0x748f82ee
    movdn.8
    exec.consume_message_word # consume msg[56]

    push.640108622
    movdn.8
    push.0x78a5636f
    movdn.8
    exec.consume_message_word # consume msg[57]

    push.822159570
    movdn.8
    push.0x84c87814
    movdn.8
    exec.consume_message_word # consume msg[58]

    push.3328750644
    movdn.8
    push.0x8cc70208
    movdn.8
    exec.consume_message_word # consume msg[59]

    push.1107837388
    movdn.8
    push.0x90befffa
    movdn.8
    exec.consume_message_word # consume msg[60]

    push.1657999800
    movdn.8
    push.0xa4506ceb
    movdn.8
    exec.consume_message_word # consume msg[61]

    push.3852183409
    movdn.8
    push.0xbef9a3f7
    movdn.8
    exec.consume_message_word # consume msg[62]

    push.2242356356
    movdn.8
    push.0xc67178f2
    movdn.8
    exec.consume_message_word # consume msg[63]

    movup.8
    u32wrapping_add

    swap
    movup.8
    u32wrapping_add
    swap

    movup.2
    movup.8
    u32wrapping_add
    movdn.2

    movup.3
    movup.8
    u32wrapping_add
    movdn.3

    movup.4
    movup.8
    u32wrapping_add
    movdn.4

    movup.5
    movup.8
    u32wrapping_add
    movdn.5

    movup.6
    movup.8
    u32wrapping_add
    movdn.6

    movup.7
    movup.8
    u32wrapping_add
    movdn.7
end

# Given 64 -bytes input, this routine computes 32 -bytes SAH256 digest
#
# Expected stack state:
#
# [m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15] | m[0,16) = 32 -bit word
#
# Note, each SHA256 word is 32 -bit wide, so that's how input is expected.
# If you've 64 -bytes, consider packing 4 consecutive bytes into single word, 
# maintaining big endian byte order.
#
# Final stack state:
#
# [dig0, dig1, dig2, dig3, dig4, dig5, dig6, dig7]
#
# SHA256 digest is represented in terms of eight 32 -bit words ( big endian byte order ).
export.hash
    push.0x5be0cd19.0x1f83d9ab.0x9b05688c.0x510e527f
    push.0xa54ff53a.0x3c6ef372.0xbb67ae85.0x6a09e667

    exec.prepare_message_schedule_and_consume
    exec.consume_padding_message_schedule
end
"),
// ----- std::math::ntt512 ------------------------------------------------------------------------
("std::math::ntt512", "# Applies four NTT butterflies on four different indices, given following stack state
#
# [k0, k1, k2, k3, A0, B0, C0, D0, A1, B1, C1, D1]
# 
# Here k`i` => i-th constant i.e. ω raised to *some* power | ω => 2N -th primitive root of unity, N = 512
#
# A{0, 1} -> first butterfly will be applied on these two elements
# B{0, 1} -> second butterfly will be applied on these two elements
# C{0, 1} -> third butterfly will be applied on these two elements
# D{0, 1} -> fourth butterfly will be applied on these two elements
#
# Four independent butterflies are applied in following way
#
# ζ = k0 * A0  | ζ = k1 * B0  | ζ = k2 * C0  | ζ = k3 * D0
# --- --- --- --- --- --- --- --- --- --- --- --- --- --- -
# A0' = A1 - ζ | B0' = B1 - ζ | C0' = C1 - ζ | D0' = D1 - ζ
# A1' = A1 + ζ | B1' = B1 + ζ | C1' = C1 + ζ | D1' = D1 + ζ
#
# After four independent butterflies are applied, resulting stack state should look like
#
# [A0', B0', C0', D0', A1', B1', C1', D1']
proc.butterfly
    movup.4
    mul

    swap
    movup.4
    mul
    swap

    movup.2
    movup.4
    mul
    movdn.2

    movup.3
    movup.4
    mul
    movdn.3

    dupw
    dupw.2

    movup.4
    add

    swap
    movup.4
    add
    swap

    movup.2
    movup.4
    add
    movdn.2

    movup.3
    movup.4
    add
    movdn.3

    swapw
    movupw.2

    movup.4
    sub

    swap
    movup.4
    sub
    swap

    movup.2
    movup.4
    sub
    movdn.2

    movup.3
    movup.4
    sub
    movdn.3
end

# Applies forward NTT on a vector of length 512, where each element ∈ Zp | p = 2^64 − 2^32 + 1,
# producing elements in frequency domain in bit-reversed order.
#
# Static input vector ( i.e. [0..512) ) is accepted using function local memory, while after 
# applying NTT, bit-reversed output vector is also kept on same function local memory allocation --- this 
# section will be improved.
#
# This routine tests itself, but doesn't respond, in any meaningful way, when invoked from outside.
# The purpose of this function is asserting functional correctness of NTT-512 implementation, while
# encapsulating the implementation.
export.forward.128
    # begin preparing development input

	push.3.2.1.0
	popw.local.0
	push.7.6.5.4
	popw.local.1
	push.11.10.9.8
	popw.local.2
	push.15.14.13.12
	popw.local.3
	push.19.18.17.16
	popw.local.4
	push.23.22.21.20
	popw.local.5
	push.27.26.25.24
	popw.local.6
	push.31.30.29.28
	popw.local.7
	push.35.34.33.32
	popw.local.8
	push.39.38.37.36
	popw.local.9
	push.43.42.41.40
	popw.local.10
	push.47.46.45.44
	popw.local.11
	push.51.50.49.48
	popw.local.12
	push.55.54.53.52
	popw.local.13
	push.59.58.57.56
	popw.local.14
	push.63.62.61.60
	popw.local.15
	push.67.66.65.64
	popw.local.16
	push.71.70.69.68
	popw.local.17
	push.75.74.73.72
	popw.local.18
	push.79.78.77.76
	popw.local.19
	push.83.82.81.80
	popw.local.20
	push.87.86.85.84
	popw.local.21
	push.91.90.89.88
	popw.local.22
	push.95.94.93.92
	popw.local.23
	push.99.98.97.96
	popw.local.24
	push.103.102.101.100
	popw.local.25
	push.107.106.105.104
	popw.local.26
	push.111.110.109.108
	popw.local.27
	push.115.114.113.112
	popw.local.28
	push.119.118.117.116
	popw.local.29
	push.123.122.121.120
	popw.local.30
	push.127.126.125.124
	popw.local.31
	push.131.130.129.128
	popw.local.32
	push.135.134.133.132
	popw.local.33
	push.139.138.137.136
	popw.local.34
	push.143.142.141.140
	popw.local.35
	push.147.146.145.144
	popw.local.36
	push.151.150.149.148
	popw.local.37
	push.155.154.153.152
	popw.local.38
	push.159.158.157.156
	popw.local.39
	push.163.162.161.160
	popw.local.40
	push.167.166.165.164
	popw.local.41
	push.171.170.169.168
	popw.local.42
	push.175.174.173.172
	popw.local.43
	push.179.178.177.176
	popw.local.44
	push.183.182.181.180
	popw.local.45
	push.187.186.185.184
	popw.local.46
	push.191.190.189.188
	popw.local.47
	push.195.194.193.192
	popw.local.48
	push.199.198.197.196
	popw.local.49
	push.203.202.201.200
	popw.local.50
	push.207.206.205.204
	popw.local.51
	push.211.210.209.208
	popw.local.52
	push.215.214.213.212
	popw.local.53
	push.219.218.217.216
	popw.local.54
	push.223.222.221.220
	popw.local.55
	push.227.226.225.224
	popw.local.56
	push.231.230.229.228
	popw.local.57
	push.235.234.233.232
	popw.local.58
	push.239.238.237.236
	popw.local.59
	push.243.242.241.240
	popw.local.60
	push.247.246.245.244
	popw.local.61
	push.251.250.249.248
	popw.local.62
	push.255.254.253.252
	popw.local.63
	push.259.258.257.256
	popw.local.64
	push.263.262.261.260
	popw.local.65
	push.267.266.265.264
	popw.local.66
	push.271.270.269.268
	popw.local.67
	push.275.274.273.272
	popw.local.68
	push.279.278.277.276
	popw.local.69
	push.283.282.281.280
	popw.local.70
	push.287.286.285.284
	popw.local.71
	push.291.290.289.288
	popw.local.72
	push.295.294.293.292
	popw.local.73
	push.299.298.297.296
	popw.local.74
	push.303.302.301.300
	popw.local.75
	push.307.306.305.304
	popw.local.76
	push.311.310.309.308
	popw.local.77
	push.315.314.313.312
	popw.local.78
	push.319.318.317.316
	popw.local.79
	push.323.322.321.320
	popw.local.80
	push.327.326.325.324
	popw.local.81
	push.331.330.329.328
	popw.local.82
	push.335.334.333.332
	popw.local.83
	push.339.338.337.336
	popw.local.84
	push.343.342.341.340
	popw.local.85
	push.347.346.345.344
	popw.local.86
	push.351.350.349.348
	popw.local.87
	push.355.354.353.352
	popw.local.88
	push.359.358.357.356
	popw.local.89
	push.363.362.361.360
	popw.local.90
	push.367.366.365.364
	popw.local.91
	push.371.370.369.368
	popw.local.92
	push.375.374.373.372
	popw.local.93
	push.379.378.377.376
	popw.local.94
	push.383.382.381.380
	popw.local.95
	push.387.386.385.384
	popw.local.96
	push.391.390.389.388
	popw.local.97
	push.395.394.393.392
	popw.local.98
	push.399.398.397.396
	popw.local.99
	push.403.402.401.400
	popw.local.100
	push.407.406.405.404
	popw.local.101
	push.411.410.409.408
	popw.local.102
	push.415.414.413.412
	popw.local.103
	push.419.418.417.416
	popw.local.104
	push.423.422.421.420
	popw.local.105
	push.427.426.425.424
	popw.local.106
	push.431.430.429.428
	popw.local.107
	push.435.434.433.432
	popw.local.108
	push.439.438.437.436
	popw.local.109
	push.443.442.441.440
	popw.local.110
	push.447.446.445.444
	popw.local.111
	push.451.450.449.448
	popw.local.112
	push.455.454.453.452
	popw.local.113
	push.459.458.457.456
	popw.local.114
	push.463.462.461.460
	popw.local.115
	push.467.466.465.464
	popw.local.116
	push.471.470.469.468
	popw.local.117
	push.475.474.473.472
	popw.local.118
	push.479.478.477.476
	popw.local.119
	push.483.482.481.480
	popw.local.120
	push.487.486.485.484
	popw.local.121
	push.491.490.489.488
	popw.local.122
	push.495.494.493.492
	popw.local.123
	push.499.498.497.496
	popw.local.124
	push.503.502.501.500
	popw.local.125
	push.507.506.505.504
	popw.local.126
	push.511.510.509.508
	popw.local.127

    # end preparing development input
    # iter = 0

	pushw.local.0
	pushw.local.64
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.64
	popw.local.0

	pushw.local.1
	pushw.local.65
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.65
	popw.local.1

	pushw.local.2
	pushw.local.66
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.66
	popw.local.2

	pushw.local.3
	pushw.local.67
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.67
	popw.local.3

	pushw.local.4
	pushw.local.68
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.68
	popw.local.4

	pushw.local.5
	pushw.local.69
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.69
	popw.local.5

	pushw.local.6
	pushw.local.70
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.70
	popw.local.6

	pushw.local.7
	pushw.local.71
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.71
	popw.local.7

	pushw.local.8
	pushw.local.72
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.72
	popw.local.8

	pushw.local.9
	pushw.local.73
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.73
	popw.local.9

	pushw.local.10
	pushw.local.74
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.74
	popw.local.10

	pushw.local.11
	pushw.local.75
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.75
	popw.local.11

	pushw.local.12
	pushw.local.76
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.76
	popw.local.12

	pushw.local.13
	pushw.local.77
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.77
	popw.local.13

	pushw.local.14
	pushw.local.78
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.78
	popw.local.14

	pushw.local.15
	pushw.local.79
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.79
	popw.local.15

	pushw.local.16
	pushw.local.80
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.80
	popw.local.16

	pushw.local.17
	pushw.local.81
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.81
	popw.local.17

	pushw.local.18
	pushw.local.82
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.82
	popw.local.18

	pushw.local.19
	pushw.local.83
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.83
	popw.local.19

	pushw.local.20
	pushw.local.84
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.84
	popw.local.20

	pushw.local.21
	pushw.local.85
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.85
	popw.local.21

	pushw.local.22
	pushw.local.86
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.86
	popw.local.22

	pushw.local.23
	pushw.local.87
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.87
	popw.local.23

	pushw.local.24
	pushw.local.88
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.88
	popw.local.24

	pushw.local.25
	pushw.local.89
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.89
	popw.local.25

	pushw.local.26
	pushw.local.90
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.90
	popw.local.26

	pushw.local.27
	pushw.local.91
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.91
	popw.local.27

	pushw.local.28
	pushw.local.92
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.92
	popw.local.28

	pushw.local.29
	pushw.local.93
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.93
	popw.local.29

	pushw.local.30
	pushw.local.94
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.94
	popw.local.30

	pushw.local.31
	pushw.local.95
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.95
	popw.local.31

	pushw.local.32
	pushw.local.96
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.96
	popw.local.32

	pushw.local.33
	pushw.local.97
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.97
	popw.local.33

	pushw.local.34
	pushw.local.98
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.98
	popw.local.34

	pushw.local.35
	pushw.local.99
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.99
	popw.local.35

	pushw.local.36
	pushw.local.100
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.100
	popw.local.36

	pushw.local.37
	pushw.local.101
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.101
	popw.local.37

	pushw.local.38
	pushw.local.102
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.102
	popw.local.38

	pushw.local.39
	pushw.local.103
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.103
	popw.local.39

	pushw.local.40
	pushw.local.104
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.104
	popw.local.40

	pushw.local.41
	pushw.local.105
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.105
	popw.local.41

	pushw.local.42
	pushw.local.106
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.106
	popw.local.42

	pushw.local.43
	pushw.local.107
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.107
	popw.local.43

	pushw.local.44
	pushw.local.108
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.108
	popw.local.44

	pushw.local.45
	pushw.local.109
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.109
	popw.local.45

	pushw.local.46
	pushw.local.110
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.110
	popw.local.46

	pushw.local.47
	pushw.local.111
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.111
	popw.local.47

	pushw.local.48
	pushw.local.112
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.112
	popw.local.48

	pushw.local.49
	pushw.local.113
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.113
	popw.local.49

	pushw.local.50
	pushw.local.114
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.114
	popw.local.50

	pushw.local.51
	pushw.local.115
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.115
	popw.local.51

	pushw.local.52
	pushw.local.116
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.116
	popw.local.52

	pushw.local.53
	pushw.local.117
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.117
	popw.local.53

	pushw.local.54
	pushw.local.118
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.118
	popw.local.54

	pushw.local.55
	pushw.local.119
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.119
	popw.local.55

	pushw.local.56
	pushw.local.120
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.120
	popw.local.56

	pushw.local.57
	pushw.local.121
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.121
	popw.local.57

	pushw.local.58
	pushw.local.122
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.122
	popw.local.58

	pushw.local.59
	pushw.local.123
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.123
	popw.local.59

	pushw.local.60
	pushw.local.124
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.124
	popw.local.60

	pushw.local.61
	pushw.local.125
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.125
	popw.local.61

	pushw.local.62
	pushw.local.126
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.126
	popw.local.62

	pushw.local.63
	pushw.local.127
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	popw.local.127
	popw.local.63

    # iter = 1

	pushw.local.0
	pushw.local.32
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.32
	popw.local.0

	pushw.local.1
	pushw.local.33
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.33
	popw.local.1

	pushw.local.2
	pushw.local.34
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.34
	popw.local.2

	pushw.local.3
	pushw.local.35
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.35
	popw.local.3

	pushw.local.4
	pushw.local.36
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.36
	popw.local.4

	pushw.local.5
	pushw.local.37
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.37
	popw.local.5

	pushw.local.6
	pushw.local.38
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.38
	popw.local.6

	pushw.local.7
	pushw.local.39
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.39
	popw.local.7

	pushw.local.8
	pushw.local.40
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.40
	popw.local.8

	pushw.local.9
	pushw.local.41
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.41
	popw.local.9

	pushw.local.10
	pushw.local.42
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.42
	popw.local.10

	pushw.local.11
	pushw.local.43
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.43
	popw.local.11

	pushw.local.12
	pushw.local.44
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.44
	popw.local.12

	pushw.local.13
	pushw.local.45
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.45
	popw.local.13

	pushw.local.14
	pushw.local.46
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.46
	popw.local.14

	pushw.local.15
	pushw.local.47
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.47
	popw.local.15

	pushw.local.16
	pushw.local.48
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.48
	popw.local.16

	pushw.local.17
	pushw.local.49
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.49
	popw.local.17

	pushw.local.18
	pushw.local.50
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.50
	popw.local.18

	pushw.local.19
	pushw.local.51
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.51
	popw.local.19

	pushw.local.20
	pushw.local.52
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.52
	popw.local.20

	pushw.local.21
	pushw.local.53
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.53
	popw.local.21

	pushw.local.22
	pushw.local.54
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.54
	popw.local.22

	pushw.local.23
	pushw.local.55
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.55
	popw.local.23

	pushw.local.24
	pushw.local.56
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.56
	popw.local.24

	pushw.local.25
	pushw.local.57
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.57
	popw.local.25

	pushw.local.26
	pushw.local.58
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.58
	popw.local.26

	pushw.local.27
	pushw.local.59
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.59
	popw.local.27

	pushw.local.28
	pushw.local.60
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.60
	popw.local.28

	pushw.local.29
	pushw.local.61
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.61
	popw.local.29

	pushw.local.30
	pushw.local.62
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.62
	popw.local.30

	pushw.local.31
	pushw.local.63
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	popw.local.63
	popw.local.31

    # ---

	pushw.local.64
	pushw.local.96
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.96
	popw.local.64

	pushw.local.65
	pushw.local.97
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.97
	popw.local.65

	pushw.local.66
	pushw.local.98
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.98
	popw.local.66

	pushw.local.67
	pushw.local.99
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.99
	popw.local.67

	pushw.local.68
	pushw.local.100
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.100
	popw.local.68

	pushw.local.69
	pushw.local.101
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.101
	popw.local.69

	pushw.local.70
	pushw.local.102
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.102
	popw.local.70

	pushw.local.71
	pushw.local.103
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.103
	popw.local.71

	pushw.local.72
	pushw.local.104
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.104
	popw.local.72

	pushw.local.73
	pushw.local.105
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.105
	popw.local.73

	pushw.local.74
	pushw.local.106
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.106
	popw.local.74

	pushw.local.75
	pushw.local.107
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.107
	popw.local.75

	pushw.local.76
	pushw.local.108
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.108
	popw.local.76

	pushw.local.77
	pushw.local.109
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.109
	popw.local.77

	pushw.local.78
	pushw.local.110
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.110
	popw.local.78

	pushw.local.79
	pushw.local.111
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.111
	popw.local.79

	pushw.local.80
	pushw.local.112
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.112
	popw.local.80

	pushw.local.81
	pushw.local.113
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.113
	popw.local.81

	pushw.local.82
	pushw.local.114
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.114
	popw.local.82

	pushw.local.83
	pushw.local.115
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.115
	popw.local.83

	pushw.local.84
	pushw.local.116
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.116
	popw.local.84

	pushw.local.85
	pushw.local.117
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.117
	popw.local.85

	pushw.local.86
	pushw.local.118
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.118
	popw.local.86

	pushw.local.87
	pushw.local.119
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.119
	popw.local.87

	pushw.local.88
	pushw.local.120
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.120
	popw.local.88

	pushw.local.89
	pushw.local.121
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.121
	popw.local.89

	pushw.local.90
	pushw.local.122
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.122
	popw.local.90

	pushw.local.91
	pushw.local.123
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.123
	popw.local.91

	pushw.local.92
	pushw.local.124
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.124
	popw.local.92

	pushw.local.93
	pushw.local.125
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.125
	popw.local.93

	pushw.local.94
	pushw.local.126
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.126
	popw.local.94

	pushw.local.95
	pushw.local.127
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	popw.local.127
	popw.local.95

    # iter = 2

	pushw.local.0
	pushw.local.16
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.16
	popw.local.0

	pushw.local.1
	pushw.local.17
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.17
	popw.local.1

	pushw.local.2
	pushw.local.18
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.18
	popw.local.2

	pushw.local.3
	pushw.local.19
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.19
	popw.local.3

	pushw.local.4
	pushw.local.20
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.20
	popw.local.4

	pushw.local.5
	pushw.local.21
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.21
	popw.local.5

	pushw.local.6
	pushw.local.22
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.22
	popw.local.6

	pushw.local.7
	pushw.local.23
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.23
	popw.local.7

	pushw.local.8
	pushw.local.24
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.24
	popw.local.8

	pushw.local.9
	pushw.local.25
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.25
	popw.local.9

	pushw.local.10
	pushw.local.26
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.26
	popw.local.10

	pushw.local.11
	pushw.local.27
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.27
	popw.local.11

	pushw.local.12
	pushw.local.28
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.28
	popw.local.12

	pushw.local.13
	pushw.local.29
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.29
	popw.local.13

	pushw.local.14
	pushw.local.30
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.30
	popw.local.14

	pushw.local.15
	pushw.local.31
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	popw.local.31
	popw.local.15

    # ---

	pushw.local.32
	pushw.local.48
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.48
	popw.local.32

	pushw.local.33
	pushw.local.49
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.49
	popw.local.33

	pushw.local.34
	pushw.local.50
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.50
	popw.local.34

	pushw.local.35
	pushw.local.51
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.51
	popw.local.35

	pushw.local.36
	pushw.local.52
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.52
	popw.local.36

	pushw.local.37
	pushw.local.53
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.53
	popw.local.37

	pushw.local.38
	pushw.local.54
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.54
	popw.local.38

	pushw.local.39
	pushw.local.55
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.55
	popw.local.39

	pushw.local.40
	pushw.local.56
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.56
	popw.local.40

	pushw.local.41
	pushw.local.57
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.57
	popw.local.41

	pushw.local.42
	pushw.local.58
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.58
	popw.local.42

	pushw.local.43
	pushw.local.59
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.59
	popw.local.43

	pushw.local.44
	pushw.local.60
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.60
	popw.local.44

	pushw.local.45
	pushw.local.61
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.61
	popw.local.45

	pushw.local.46
	pushw.local.62
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.62
	popw.local.46

	pushw.local.47
	pushw.local.63
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	popw.local.63
	popw.local.47

    # ---

	pushw.local.64
	pushw.local.80
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.80
	popw.local.64

	pushw.local.65
	pushw.local.81
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.81
	popw.local.65

	pushw.local.66
	pushw.local.82
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.82
	popw.local.66

	pushw.local.67
	pushw.local.83
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.83
	popw.local.67

	pushw.local.68
	pushw.local.84
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.84
	popw.local.68

	pushw.local.69
	pushw.local.85
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.85
	popw.local.69

	pushw.local.70
	pushw.local.86
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.86
	popw.local.70

	pushw.local.71
	pushw.local.87
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.87
	popw.local.71

	pushw.local.72
	pushw.local.88
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.88
	popw.local.72

	pushw.local.73
	pushw.local.89
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.89
	popw.local.73

	pushw.local.74
	pushw.local.90
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.90
	popw.local.74

	pushw.local.75
	pushw.local.91
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.91
	popw.local.75

	pushw.local.76
	pushw.local.92
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.92
	popw.local.76

	pushw.local.77
	pushw.local.93
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.93
	popw.local.77

	pushw.local.78
	pushw.local.94
	push.4096.4096.4096.4096

	exec.butterfly

	popw.local.94
	popw.local.78

	pushw.local.79
	pushw.local.95
	push.4096.4096.4096.4096

	exec.butterfly

    popw.local.95
	popw.local.79

    # ---

	pushw.local.96
	pushw.local.112
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.112
	popw.local.96

	pushw.local.97
	pushw.local.113
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.113
	popw.local.97

	pushw.local.98
	pushw.local.114
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.114
	popw.local.98

	pushw.local.99
	pushw.local.115
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.115
	popw.local.99

	pushw.local.100
	pushw.local.116
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.116
	popw.local.100

	pushw.local.101
	pushw.local.117
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.117
	popw.local.101

	pushw.local.102
	pushw.local.118
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.118
	popw.local.102

	pushw.local.103
	pushw.local.119
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.119
	popw.local.103

	pushw.local.104
	pushw.local.120
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.120
	popw.local.104

	pushw.local.105
	pushw.local.121
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.121
	popw.local.105

	pushw.local.106
	pushw.local.122
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.122
	popw.local.106

	pushw.local.107
	pushw.local.123
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.123
	popw.local.107

	pushw.local.108
	pushw.local.124
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.124
	popw.local.108

	pushw.local.109
	pushw.local.125
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.125
	popw.local.109

	pushw.local.110
	pushw.local.126
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.126
	popw.local.110

	pushw.local.111
	pushw.local.127
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	popw.local.127
	popw.local.111

    # iter = 3

	pushw.local.0
	pushw.local.8
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	popw.local.8
	popw.local.0

	pushw.local.1
	pushw.local.9
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	popw.local.9
	popw.local.1

	pushw.local.2
	pushw.local.10
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	popw.local.10
	popw.local.2

	pushw.local.3
	pushw.local.11
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	popw.local.11
	popw.local.3

	pushw.local.4
	pushw.local.12
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	popw.local.12
	popw.local.4

	pushw.local.5
	pushw.local.13
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	popw.local.13
	popw.local.5

	pushw.local.6
	pushw.local.14
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	popw.local.14
	popw.local.6

	pushw.local.7
	pushw.local.15
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	popw.local.15
	popw.local.7

    # ---

    pushw.local.16
	pushw.local.24
	push.262144.262144.262144.262144

	exec.butterfly

	popw.local.24
	popw.local.16

	pushw.local.17
	pushw.local.25
	push.262144.262144.262144.262144

	exec.butterfly

	popw.local.25
	popw.local.17

	pushw.local.18
	pushw.local.26
	push.262144.262144.262144.262144

	exec.butterfly

	popw.local.26
	popw.local.18

	pushw.local.19
	pushw.local.27
	push.262144.262144.262144.262144

	exec.butterfly

	popw.local.27
	popw.local.19

	pushw.local.20
	pushw.local.28
	push.262144.262144.262144.262144

	exec.butterfly

	popw.local.28
	popw.local.20

	pushw.local.21
	pushw.local.29
	push.262144.262144.262144.262144

	exec.butterfly

	popw.local.29
	popw.local.21

	pushw.local.22
	pushw.local.30
	push.262144.262144.262144.262144

	exec.butterfly

	popw.local.30
	popw.local.22

	pushw.local.23
	pushw.local.31
	push.262144.262144.262144.262144

	exec.butterfly

	popw.local.31
	popw.local.23

    # ---

    pushw.local.32
	pushw.local.40
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	popw.local.40
	popw.local.32

	pushw.local.33
	pushw.local.41
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	popw.local.41
	popw.local.33

	pushw.local.34
	pushw.local.42
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	popw.local.42
	popw.local.34

	pushw.local.35
	pushw.local.43
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	popw.local.43
	popw.local.35

	pushw.local.36
	pushw.local.44
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	popw.local.44
	popw.local.36

	pushw.local.37
	pushw.local.45
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	popw.local.45
	popw.local.37

	pushw.local.38
	pushw.local.46
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	popw.local.46
	popw.local.38

	pushw.local.39
	pushw.local.47
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	popw.local.47
	popw.local.39

    # ---

    pushw.local.48
	pushw.local.56
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	popw.local.56
	popw.local.48

	pushw.local.49
	pushw.local.57
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	popw.local.57
	popw.local.49

	pushw.local.50
	pushw.local.58
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	popw.local.58
	popw.local.50

	pushw.local.51
	pushw.local.59
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	popw.local.59
	popw.local.51

	pushw.local.52
	pushw.local.60
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	popw.local.60
	popw.local.52

	pushw.local.53
	pushw.local.61
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	popw.local.61
	popw.local.53

	pushw.local.54
	pushw.local.62
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	popw.local.62
	popw.local.54

	pushw.local.55
	pushw.local.63
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	popw.local.63
	popw.local.55

    # ---

    pushw.local.64
	pushw.local.72
	push.64.64.64.64

	exec.butterfly

	popw.local.72
	popw.local.64

	pushw.local.65
	pushw.local.73
	push.64.64.64.64

	exec.butterfly

	popw.local.73
	popw.local.65

	pushw.local.66
	pushw.local.74
	push.64.64.64.64

	exec.butterfly

	popw.local.74
	popw.local.66

	pushw.local.67
	pushw.local.75
	push.64.64.64.64

	exec.butterfly

	popw.local.75
	popw.local.67

	pushw.local.68
	pushw.local.76
	push.64.64.64.64

	exec.butterfly

	popw.local.76
	popw.local.68

	pushw.local.69
	pushw.local.77
	push.64.64.64.64

	exec.butterfly

	popw.local.77
	popw.local.69

	pushw.local.70
	pushw.local.78
	push.64.64.64.64

	exec.butterfly

	popw.local.78
	popw.local.70

	pushw.local.71
	pushw.local.79
	push.64.64.64.64

	exec.butterfly

	popw.local.79
	popw.local.71

    # ---

    pushw.local.80
	pushw.local.88
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	popw.local.88
	popw.local.80

	pushw.local.81
	pushw.local.89
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	popw.local.89
	popw.local.81

	pushw.local.82
	pushw.local.90
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	popw.local.90
	popw.local.82

	pushw.local.83
	pushw.local.91
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	popw.local.91
	popw.local.83

	pushw.local.84
	pushw.local.92
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	popw.local.92
	popw.local.84

	pushw.local.85
	pushw.local.93
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	popw.local.93
	popw.local.85

	pushw.local.86
	pushw.local.94
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	popw.local.94
	popw.local.86

	pushw.local.87
	pushw.local.95
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	popw.local.95
	popw.local.87

    # ---

    pushw.local.96
	pushw.local.104
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	popw.local.104
	popw.local.96

	pushw.local.97
	pushw.local.105
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	popw.local.105
	popw.local.97

	pushw.local.98
	pushw.local.106
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	popw.local.106
	popw.local.98

	pushw.local.99
	pushw.local.107
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	popw.local.107
	popw.local.99

	pushw.local.100
	pushw.local.108
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	popw.local.108
	popw.local.100

	pushw.local.101
	pushw.local.109
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	popw.local.109
	popw.local.101

	pushw.local.102
	pushw.local.110
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	popw.local.110
	popw.local.102

	pushw.local.103
	pushw.local.111
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	popw.local.111
	popw.local.103

    # ---

    pushw.local.112
	pushw.local.120
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	popw.local.120
	popw.local.112

	pushw.local.113
	pushw.local.121
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	popw.local.121
	popw.local.113

	pushw.local.114
	pushw.local.122
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	popw.local.122
	popw.local.114

	pushw.local.115
	pushw.local.123
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	popw.local.123
	popw.local.115

	pushw.local.116
	pushw.local.124
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	popw.local.124
	popw.local.116

	pushw.local.117
	pushw.local.125
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	popw.local.125
	popw.local.117

	pushw.local.118
	pushw.local.126
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	popw.local.126
	popw.local.118

	pushw.local.119
	pushw.local.127
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	popw.local.127
	popw.local.119

    # iter = 4

    pushw.local.0
	pushw.local.4
	push.18446744060824649729.18446744060824649729.18446744060824649729.18446744060824649729

	exec.butterfly

	popw.local.4
	popw.local.0

	pushw.local.1
	pushw.local.5
	push.18446744060824649729.18446744060824649729.18446744060824649729.18446744060824649729

	exec.butterfly

	popw.local.5
	popw.local.1

	pushw.local.2
	pushw.local.6
	push.18446744060824649729.18446744060824649729.18446744060824649729.18446744060824649729

	exec.butterfly

	popw.local.6
	popw.local.2

	pushw.local.3
	pushw.local.7
	push.18446744060824649729.18446744060824649729.18446744060824649729.18446744060824649729

	exec.butterfly

	popw.local.7
	popw.local.3

    # ---

    pushw.local.8
	pushw.local.12
	push.562949953290240.562949953290240.562949953290240.562949953290240

	exec.butterfly

	popw.local.12
	popw.local.8

	pushw.local.9
	pushw.local.13
	push.562949953290240.562949953290240.562949953290240.562949953290240

	exec.butterfly

	popw.local.13
	popw.local.9

	pushw.local.10
	pushw.local.14
	push.562949953290240.562949953290240.562949953290240.562949953290240

	exec.butterfly

	popw.local.14
	popw.local.10

	pushw.local.11
	pushw.local.15
	push.562949953290240.562949953290240.562949953290240.562949953290240

	exec.butterfly

	popw.local.15
	popw.local.11

    # ---

    pushw.local.16
	pushw.local.20
	push.512.512.512.512

	exec.butterfly

	popw.local.20
	popw.local.16

	pushw.local.17
	pushw.local.21
	push.512.512.512.512

	exec.butterfly

	popw.local.21
	popw.local.17

	pushw.local.18
	pushw.local.22
	push.512.512.512.512

	exec.butterfly

	popw.local.22
	popw.local.18

	pushw.local.19
	pushw.local.23
	push.512.512.512.512

	exec.butterfly

	popw.local.23
	popw.local.19

    # ---

    pushw.local.24
	pushw.local.28
	push.18302628881338728449.18302628881338728449.18302628881338728449.18302628881338728449

	exec.butterfly

	popw.local.28
	popw.local.24

	pushw.local.25
	pushw.local.29
	push.18302628881338728449.18302628881338728449.18302628881338728449.18302628881338728449

	exec.butterfly

	popw.local.29
	popw.local.25

	pushw.local.26
	pushw.local.30
	push.18302628881338728449.18302628881338728449.18302628881338728449.18302628881338728449

	exec.butterfly

	popw.local.30
	popw.local.26

	pushw.local.27
	pushw.local.31
	push.18302628881338728449.18302628881338728449.18302628881338728449.18302628881338728449

	exec.butterfly

	popw.local.31
	popw.local.27

    # ---

    pushw.local.32
	pushw.local.36
	push.137438953440.137438953440.137438953440.137438953440

	exec.butterfly

	popw.local.36
	popw.local.32

	pushw.local.33
	pushw.local.37
	push.137438953440.137438953440.137438953440.137438953440

	exec.butterfly

	popw.local.37
	popw.local.33

	pushw.local.34
	pushw.local.38
	push.137438953440.137438953440.137438953440.137438953440

	exec.butterfly

	popw.local.38
	popw.local.34

	pushw.local.35
	pushw.local.39
	push.137438953440.137438953440.137438953440.137438953440

	exec.butterfly

	popw.local.39
	popw.local.35

    # ---

    pushw.local.40
	pushw.local.44
	push.2097152.2097152.2097152.2097152

	exec.butterfly

	popw.local.44
	popw.local.40

	pushw.local.41
	pushw.local.45
	push.2097152.2097152.2097152.2097152

	exec.butterfly

	popw.local.45
	popw.local.41

	pushw.local.42
	pushw.local.46
	push.2097152.2097152.2097152.2097152

	exec.butterfly

	popw.local.46
	popw.local.42

	pushw.local.43
	pushw.local.47
	push.2097152.2097152.2097152.2097152

	exec.butterfly

	popw.local.47
	popw.local.43

    # ---

    pushw.local.48
	pushw.local.52
	push.18446708885042495489.18446708885042495489.18446708885042495489.18446708885042495489

	exec.butterfly

	popw.local.52
	popw.local.48

	pushw.local.49
	pushw.local.53
	push.18446708885042495489.18446708885042495489.18446708885042495489.18446708885042495489

	exec.butterfly

	popw.local.53
	popw.local.49

	pushw.local.50
	pushw.local.54
	push.18446708885042495489.18446708885042495489.18446708885042495489.18446708885042495489

	exec.butterfly

	popw.local.54
	popw.local.50

	pushw.local.51
	pushw.local.55
	push.18446708885042495489.18446708885042495489.18446708885042495489.18446708885042495489

	exec.butterfly

	popw.local.55
	popw.local.51

    # ---

    pushw.local.56
	pushw.local.60
	push.2305843008676823040.2305843008676823040.2305843008676823040.2305843008676823040

	exec.butterfly

	popw.local.60
	popw.local.56

	pushw.local.57
	pushw.local.61
	push.2305843008676823040.2305843008676823040.2305843008676823040.2305843008676823040

	exec.butterfly

	popw.local.61
	popw.local.57

	pushw.local.58
	pushw.local.62
	push.2305843008676823040.2305843008676823040.2305843008676823040.2305843008676823040

	exec.butterfly

	popw.local.62
	popw.local.58

	pushw.local.59
	pushw.local.63
	push.2305843008676823040.2305843008676823040.2305843008676823040.2305843008676823040

	exec.butterfly

	popw.local.63
	popw.local.59

    # ---

    pushw.local.64
	pushw.local.68
	push.8.8.8.8

	exec.butterfly

	popw.local.68
	popw.local.64

	pushw.local.65
	pushw.local.69
	push.8.8.8.8

	exec.butterfly

	popw.local.69
	popw.local.65

	pushw.local.66
	pushw.local.70
	push.8.8.8.8

	exec.butterfly

	popw.local.70
	popw.local.66

	pushw.local.67
	pushw.local.71
	push.8.8.8.8

	exec.butterfly

	popw.local.71
	popw.local.67

    # ---

    pushw.local.72
	pushw.local.76
	push.18444492269600899073.18444492269600899073.18444492269600899073.18444492269600899073

	exec.butterfly

	popw.local.76
	popw.local.72

	pushw.local.73
	pushw.local.77
	push.18444492269600899073.18444492269600899073.18444492269600899073.18444492269600899073

	exec.butterfly

	popw.local.77
	popw.local.73

	pushw.local.74
	pushw.local.78
	push.18444492269600899073.18444492269600899073.18444492269600899073.18444492269600899073

	exec.butterfly

	popw.local.78
	popw.local.74

	pushw.local.75
	pushw.local.79
	push.18444492269600899073.18444492269600899073.18444492269600899073.18444492269600899073

	exec.butterfly

	popw.local.79
	popw.local.75

    # ---

    pushw.local.80
	pushw.local.84
	push.8796093020160.8796093020160.8796093020160.8796093020160

	exec.butterfly

	popw.local.84
	popw.local.80

	pushw.local.81
	pushw.local.85
	push.8796093020160.8796093020160.8796093020160.8796093020160

	exec.butterfly

	popw.local.85
	popw.local.81

	pushw.local.82
	pushw.local.86
	push.8796093020160.8796093020160.8796093020160.8796093020160

	exec.butterfly

	popw.local.86
	popw.local.82

	pushw.local.83
	pushw.local.87
	push.8796093020160.8796093020160.8796093020160.8796093020160

	exec.butterfly

	popw.local.87
	popw.local.83

    # ---

    pushw.local.88
	pushw.local.92
	push.134217728.134217728.134217728.134217728

	exec.butterfly

	popw.local.92
	popw.local.88

	pushw.local.89
	pushw.local.93
	push.134217728.134217728.134217728.134217728

	exec.butterfly

	popw.local.93
	popw.local.89

	pushw.local.90
	pushw.local.94
	push.134217728.134217728.134217728.134217728

	exec.butterfly

	popw.local.94
	popw.local.90

	pushw.local.91
	pushw.local.95
	push.134217728.134217728.134217728.134217728

	exec.butterfly

	popw.local.95
	popw.local.91

    # ---

    pushw.local.96
	pushw.local.100
	push.18446743519658770433.18446743519658770433.18446743519658770433.18446743519658770433

	exec.butterfly

	popw.local.100
	popw.local.96

	pushw.local.97
	pushw.local.101
	push.18446743519658770433.18446743519658770433.18446743519658770433.18446743519658770433

	exec.butterfly

	popw.local.101
	popw.local.97

	pushw.local.98
	pushw.local.102
	push.18446743519658770433.18446743519658770433.18446743519658770433.18446743519658770433

	exec.butterfly

	popw.local.102
	popw.local.98

	pushw.local.99
	pushw.local.103
	push.18446743519658770433.18446743519658770433.18446743519658770433.18446743519658770433

	exec.butterfly

	popw.local.103
	popw.local.99

    # ---

    pushw.local.104
	pushw.local.108
	push.36028797010575360.36028797010575360.36028797010575360.36028797010575360

	exec.butterfly

	popw.local.108
	popw.local.104

	pushw.local.105
	pushw.local.109
	push.36028797010575360.36028797010575360.36028797010575360.36028797010575360

	exec.butterfly

	popw.local.109
	popw.local.105

	pushw.local.106
	pushw.local.110
	push.36028797010575360.36028797010575360.36028797010575360.36028797010575360

	exec.butterfly

	popw.local.110
	popw.local.106

	pushw.local.107
	pushw.local.111
	push.36028797010575360.36028797010575360.36028797010575360.36028797010575360

	exec.butterfly

	popw.local.111
	popw.local.107

    # ---

    pushw.local.112
	pushw.local.116
	push.32768.32768.32768.32768

	exec.butterfly

	popw.local.116
	popw.local.112

	pushw.local.113
	pushw.local.117
	push.32768.32768.32768.32768

	exec.butterfly

	popw.local.117
	popw.local.113

	pushw.local.114
	pushw.local.118
	push.32768.32768.32768.32768

	exec.butterfly

	popw.local.118
	popw.local.114

	pushw.local.115
	pushw.local.119
	push.32768.32768.32768.32768

	exec.butterfly

	popw.local.119
	popw.local.115

    # ---

    pushw.local.120
	pushw.local.124
	push.9223372032559808513.9223372032559808513.9223372032559808513.9223372032559808513

	exec.butterfly

	popw.local.124
	popw.local.120

	pushw.local.121
	pushw.local.125
	push.9223372032559808513.9223372032559808513.9223372032559808513.9223372032559808513

	exec.butterfly

	popw.local.125
	popw.local.121

	pushw.local.122
	pushw.local.126
	push.9223372032559808513.9223372032559808513.9223372032559808513.9223372032559808513

	exec.butterfly

	popw.local.126
	popw.local.122

	pushw.local.123
	pushw.local.127
	push.9223372032559808513.9223372032559808513.9223372032559808513.9223372032559808513

	exec.butterfly

	popw.local.127
	popw.local.123

    # iter = 5

	pushw.local.0
	pushw.local.2
	push.72058693532778496.72058693532778496.72058693532778496.72058693532778496

	exec.butterfly

	popw.local.2
	popw.local.0

	pushw.local.1
	pushw.local.3
	push.72058693532778496.72058693532778496.72058693532778496.72058693532778496

	exec.butterfly

	popw.local.3
	popw.local.1

	# ---

	pushw.local.4
	pushw.local.6
	push.18374687574905061377.18374687574905061377.18374687574905061377.18374687574905061377

	exec.butterfly

	popw.local.6
	popw.local.4

	pushw.local.5
	pushw.local.7
	push.18374687574905061377.18374687574905061377.18374687574905061377.18374687574905061377

	exec.butterfly

	popw.local.7
	popw.local.5

	# ---

	pushw.local.8
	pushw.local.10
	push.18446744065119551490.18446744065119551490.18446744065119551490.18446744065119551490

	exec.butterfly

	popw.local.10
	popw.local.8

	pushw.local.9
	pushw.local.11
	push.18446744065119551490.18446744065119551490.18446744065119551490.18446744065119551490

	exec.butterfly

	popw.local.11
	popw.local.9

	# ---

	pushw.local.12
	pushw.local.14
	push.4294901759.4294901759.4294901759.4294901759

	exec.butterfly

	popw.local.14
	popw.local.12

	pushw.local.13
	pushw.local.15
	push.4294901759.4294901759.4294901759.4294901759

	exec.butterfly

	popw.local.15
	popw.local.13

	# ---

	pushw.local.16
	pushw.local.18
	push.18446726477496979457.18446726477496979457.18446726477496979457.18446726477496979457

	exec.butterfly

	popw.local.18
	popw.local.16

	pushw.local.17
	pushw.local.19
	push.18446726477496979457.18446726477496979457.18446726477496979457.18446726477496979457

	exec.butterfly

	popw.local.19
	popw.local.17

	# ---

	pushw.local.20
	pushw.local.22
	push.18446726476960108545.18446726476960108545.18446726476960108545.18446726476960108545

	exec.butterfly

	popw.local.22
	popw.local.20

	pushw.local.21
	pushw.local.23
	push.18446726476960108545.18446726476960108545.18446726476960108545.18446726476960108545

	exec.butterfly

	popw.local.23
	popw.local.21

	# ---

	pushw.local.24
	pushw.local.26
	push.4503599627370480.4503599627370480.4503599627370480.4503599627370480

	exec.butterfly

	popw.local.26
	popw.local.24

	pushw.local.25
	pushw.local.27
	push.4503599627370480.4503599627370480.4503599627370480.4503599627370480

	exec.butterfly

	popw.local.27
	popw.local.25

	# ---

	pushw.local.28
	pushw.local.30
	push.4503599627370512.4503599627370512.4503599627370512.4503599627370512

	exec.butterfly

	popw.local.30
	popw.local.28

	pushw.local.29
	pushw.local.31
	push.4503599627370512.4503599627370512.4503599627370512.4503599627370512

	exec.butterfly

	popw.local.31
	popw.local.29

	# ---

	pushw.local.32
	pushw.local.34
	push.18158513693262871553.18158513693262871553.18158513693262871553.18158513693262871553

	exec.butterfly

	popw.local.34
	popw.local.32

	pushw.local.33
	pushw.local.35
	push.18158513693262871553.18158513693262871553.18158513693262871553.18158513693262871553

	exec.butterfly

	popw.local.35
	popw.local.33

	# ---

	pushw.local.36
	pushw.local.38
	push.288230376151710720.288230376151710720.288230376151710720.288230376151710720

	exec.butterfly

	popw.local.38
	popw.local.36

	pushw.local.37
	pushw.local.39
	push.288230376151710720.288230376151710720.288230376151710720.288230376151710720

	exec.butterfly

	popw.local.39
	popw.local.37

	# ---

	pushw.local.40
	pushw.local.42
	push.18445618186687873025.18445618186687873025.18445618186687873025.18445618186687873025

	exec.butterfly

	popw.local.42
	popw.local.40

	pushw.local.41
	pushw.local.43
	push.18445618186687873025.18445618186687873025.18445618186687873025.18445618186687873025

	exec.butterfly

	popw.local.43
	popw.local.41

	# ---

	pushw.local.44
	pushw.local.46
	push.18445618152328134657.18445618152328134657.18445618152328134657.18445618152328134657

	exec.butterfly

	popw.local.46
	popw.local.44

	pushw.local.45
	pushw.local.47
	push.18445618152328134657.18445618152328134657.18445618152328134657.18445618152328134657

	exec.butterfly

	popw.local.47
	popw.local.45

	# ---

	pushw.local.48
	pushw.local.50
	push.4611756386097823744.4611756386097823744.4611756386097823744.4611756386097823744

	exec.butterfly

	popw.local.50
	popw.local.48

	pushw.local.49
	pushw.local.51
	push.4611756386097823744.4611756386097823744.4611756386097823744.4611756386097823744

	exec.butterfly

	popw.local.51
	popw.local.49

	# ---

	pushw.local.52
	pushw.local.54
	push.13835128420805115905.13835128420805115905.13835128420805115905.13835128420805115905

	exec.butterfly

	popw.local.54
	popw.local.52

	pushw.local.53
	pushw.local.55
	push.13835128420805115905.13835128420805115905.13835128420805115905.13835128420805115905

	exec.butterfly

	popw.local.55
	popw.local.53

	# ---

	pushw.local.56
	pushw.local.58
	push.18446743794532483137.18446743794532483137.18446743794532483137.18446743794532483137

	exec.butterfly

	popw.local.58
	popw.local.56

	pushw.local.57
	pushw.local.59
	push.18446743794532483137.18446743794532483137.18446743794532483137.18446743794532483137

	exec.butterfly

	popw.local.59
	popw.local.57

	# ---

	pushw.local.60
	pushw.local.62
	push.274873712576.274873712576.274873712576.274873712576

	exec.butterfly

	popw.local.62
	popw.local.60

	pushw.local.61
	pushw.local.63
	push.274873712576.274873712576.274873712576.274873712576

	exec.butterfly

	popw.local.63
	popw.local.61

	# ---

	pushw.local.64
	pushw.local.66
	push.18446741870424883713.18446741870424883713.18446741870424883713.18446741870424883713

	exec.butterfly

	popw.local.66
	popw.local.64

	pushw.local.65
	pushw.local.67
	push.18446741870424883713.18446741870424883713.18446741870424883713.18446741870424883713

	exec.butterfly

	popw.local.67
	popw.local.65

	# ---

	pushw.local.68
	pushw.local.70
	push.18446741870357774849.18446741870357774849.18446741870357774849.18446741870357774849

	exec.butterfly

	popw.local.70
	popw.local.68

	pushw.local.69
	pushw.local.71
	push.18446741870357774849.18446741870357774849.18446741870357774849.18446741870357774849

	exec.butterfly

	popw.local.71
	popw.local.69

	# ---

	pushw.local.72
	pushw.local.74
	push.562949953421310.562949953421310.562949953421310.562949953421310

	exec.butterfly

	popw.local.74
	popw.local.72

	pushw.local.73
	pushw.local.75
	push.562949953421310.562949953421310.562949953421310.562949953421310

	exec.butterfly

	popw.local.75
	popw.local.73

	# ---

	pushw.local.76
	pushw.local.78
	push.562949953421314.562949953421314.562949953421314.562949953421314

	exec.butterfly

	popw.local.78
	popw.local.76

	pushw.local.77
	pushw.local.79
	push.562949953421314.562949953421314.562949953421314.562949953421314

	exec.butterfly

	popw.local.79
	popw.local.77

	# ---

	pushw.local.80
	pushw.local.82
	push.16140901060200882177.16140901060200882177.16140901060200882177.16140901060200882177

	exec.butterfly

	popw.local.82
	popw.local.80

	pushw.local.81
	pushw.local.83
	push.16140901060200882177.16140901060200882177.16140901060200882177.16140901060200882177

	exec.butterfly

	popw.local.83
	popw.local.81

	# ---

	pushw.local.84
	pushw.local.86
	push.2305843009213685760.2305843009213685760.2305843009213685760.2305843009213685760

	exec.butterfly

	popw.local.86
	popw.local.84

	pushw.local.85
	pushw.local.87
	push.2305843009213685760.2305843009213685760.2305843009213685760.2305843009213685760

	exec.butterfly

	popw.local.87
	popw.local.85

	# ---

	pushw.local.88
	pushw.local.90
	push.18437737007600893953.18437737007600893953.18437737007600893953.18437737007600893953

	exec.butterfly

	popw.local.90
	popw.local.88

	pushw.local.89
	pushw.local.91
	push.18437737007600893953.18437737007600893953.18437737007600893953.18437737007600893953

	exec.butterfly

	popw.local.91
	popw.local.89

	# ---

	pushw.local.92
	pushw.local.94
	push.18437736732722987009.18437736732722987009.18437736732722987009.18437736732722987009

	exec.butterfly

	popw.local.94
	popw.local.92

	pushw.local.93
	pushw.local.95
	push.18437736732722987009.18437736732722987009.18437736732722987009.18437736732722987009

	exec.butterfly

	popw.local.95
	popw.local.93

	# ---

	pushw.local.96
	pushw.local.98
	push.576469548262227968.576469548262227968.576469548262227968.576469548262227968

	exec.butterfly

	popw.local.98
	popw.local.96

	pushw.local.97
	pushw.local.99
	push.576469548262227968.576469548262227968.576469548262227968.576469548262227968

	exec.butterfly

	popw.local.99
	popw.local.97

	# ---

	pushw.local.100
	pushw.local.102
	push.17870292113338400769.17870292113338400769.17870292113338400769.17870292113338400769

	exec.butterfly

	popw.local.102
	popw.local.100

	pushw.local.101
	pushw.local.103
	push.17870292113338400769.17870292113338400769.17870292113338400769.17870292113338400769

	exec.butterfly

	popw.local.103
	popw.local.101

	# ---

	pushw.local.104
	pushw.local.106
	push.18446744035054321673.18446744035054321673.18446744035054321673.18446744035054321673

	exec.butterfly

	popw.local.106
	popw.local.104

	pushw.local.105
	pushw.local.107
	push.18446744035054321673.18446744035054321673.18446744035054321673.18446744035054321673

	exec.butterfly

	popw.local.107
	popw.local.105

	# ---

	pushw.local.108
	pushw.local.110
	push.34359214072.34359214072.34359214072.34359214072

	exec.butterfly

	popw.local.110
	popw.local.108

	pushw.local.109
	pushw.local.111
	push.34359214072.34359214072.34359214072.34359214072

	exec.butterfly

	popw.local.111
	popw.local.109

	# ---

	pushw.local.112
	pushw.local.114
	push.18446603334073745409.18446603334073745409.18446603334073745409.18446603334073745409

	exec.butterfly

	popw.local.114
	popw.local.112

	pushw.local.113
	pushw.local.115
	push.18446603334073745409.18446603334073745409.18446603334073745409.18446603334073745409

	exec.butterfly

	popw.local.115
	popw.local.113

	# ---

	pushw.local.116
	pushw.local.118
	push.18446603329778778113.18446603329778778113.18446603329778778113.18446603329778778113

	exec.butterfly

	popw.local.118
	popw.local.116

	pushw.local.117
	pushw.local.119
	push.18446603329778778113.18446603329778778113.18446603329778778113.18446603329778778113

	exec.butterfly

	popw.local.119
	popw.local.117

	# ---

	pushw.local.120
	pushw.local.122
	push.36028797018963840.36028797018963840.36028797018963840.36028797018963840

	exec.butterfly

	popw.local.122
	popw.local.120

	pushw.local.121
	pushw.local.123
	push.36028797018963840.36028797018963840.36028797018963840.36028797018963840

	exec.butterfly

	popw.local.123
	popw.local.121

	# ---

	pushw.local.124
	pushw.local.126
	push.36028797018964096.36028797018964096.36028797018964096.36028797018964096

	exec.butterfly

	popw.local.126
	popw.local.124

	pushw.local.125
	pushw.local.127
	push.36028797018964096.36028797018964096.36028797018964096.36028797018964096

	exec.butterfly

	popw.local.127
	popw.local.125

	# iter = 6

    pushw.local.0
	pushw.local.1
	push.16192975500896648969.16192975500896648969.16192975500896648969.16192975500896648969

	exec.butterfly

	popw.local.1
	popw.local.0

	# ---

	pushw.local.2
	pushw.local.3
	push.13801972045324315718.13801972045324315718.13801972045324315718.13801972045324315718

	exec.butterfly

	popw.local.3
	popw.local.2

	# ---

	pushw.local.4
	pushw.local.5
	push.10105805016917838453.10105805016917838453.10105805016917838453.10105805016917838453

	exec.butterfly

	popw.local.5
	popw.local.4

	# ---

	pushw.local.6
	pushw.local.7
	push.7884753188935386879.7884753188935386879.7884753188935386879.7884753188935386879

	exec.butterfly

	popw.local.7
	popw.local.6

	# ---

	pushw.local.8
	pushw.local.9
	push.4299803665592489687.4299803665592489687.4299803665592489687.4299803665592489687

	exec.butterfly

	popw.local.9
	popw.local.8

	# ---

	pushw.local.10
	pushw.local.11
	push.17330401598553671485.17330401598553671485.17330401598553671485.17330401598553671485

	exec.butterfly

	popw.local.11
	popw.local.10

	# ---

	pushw.local.12
	pushw.local.13
	push.10382722127243543029.10382722127243543029.10382722127243543029.10382722127243543029

	exec.butterfly

	popw.local.13
	popw.local.12

	# ---

	pushw.local.14
	pushw.local.15
	push.12053668962110821384.12053668962110821384.12053668962110821384.12053668962110821384

	exec.butterfly

	popw.local.15
	popw.local.14

	# ---

	pushw.local.16
	pushw.local.17
	push.3328437340319972906.3328437340319972906.3328437340319972906.3328437340319972906

	exec.butterfly

	popw.local.17
	popw.local.16

	# ---

	pushw.local.18
	pushw.local.19
	push.411429644661718300.411429644661718300.411429644661718300.411429644661718300

	exec.butterfly

	popw.local.19
	popw.local.18

	# ---

	pushw.local.20
	pushw.local.21
	push.16933017626115159474.16933017626115159474.16933017626115159474.16933017626115159474

	exec.butterfly

	popw.local.21
	popw.local.20

	# ---

	pushw.local.22
	pushw.local.23
	push.2341058142559915780.2341058142559915780.2341058142559915780.2341058142559915780

	exec.butterfly

	popw.local.23
	popw.local.22

	# ---

	pushw.local.24
	pushw.local.25
	push.3332764170168812040.3332764170168812040.3332764170168812040.3332764170168812040

	exec.butterfly

	popw.local.25
	popw.local.24

	# ---

	pushw.local.26
	pushw.local.27
	push.16329239638270742865.16329239638270742865.16329239638270742865.16329239638270742865

	exec.butterfly

	popw.local.27
	popw.local.26

	# ---

	pushw.local.28
	pushw.local.29
	push.1135478653231209757.1135478653231209757.1135478653231209757.1135478653231209757

	exec.butterfly

	popw.local.29
	popw.local.28

	# ---

	pushw.local.30
	pushw.local.31
	push.6562114217670983589.6562114217670983589.6562114217670983589.6562114217670983589

	exec.butterfly

	popw.local.31
	popw.local.30

	# ---

	pushw.local.32
	pushw.local.33
	push.2843318466875884251.2843318466875884251.2843318466875884251.2843318466875884251

	exec.butterfly

	popw.local.33
	popw.local.32

	# ---

	pushw.local.34
	pushw.local.35
	push.9083829225849678056.9083829225849678056.9083829225849678056.9083829225849678056

	exec.butterfly

	popw.local.35
	popw.local.34

	# ---

	pushw.local.36
	pushw.local.37
	push.8215369291935911999.8215369291935911999.8215369291935911999.8215369291935911999

	exec.butterfly

	popw.local.37
	popw.local.36

	# ---

	pushw.local.38
	pushw.local.39
	push.1506708620263852673.1506708620263852673.1506708620263852673.1506708620263852673

	exec.butterfly

	popw.local.39
	popw.local.38

	# ---

	pushw.local.40
	pushw.local.41
	push.8180754653145198927.8180754653145198927.8180754653145198927.8180754653145198927

	exec.butterfly

	popw.local.41
	popw.local.40

	# ---

	pushw.local.42
	pushw.local.43
	push.3291437157293746400.3291437157293746400.3291437157293746400.3291437157293746400

	exec.butterfly

	popw.local.43
	popw.local.42

	# ---

	pushw.local.44
	pushw.local.45
	push.6336932523019185545.6336932523019185545.6336932523019185545.6336932523019185545

	exec.butterfly

	popw.local.45
	popw.local.44

	# ---

	pushw.local.46
	pushw.local.47
	push.281721071064741919.281721071064741919.281721071064741919.281721071064741919

	exec.butterfly

	popw.local.47
	popw.local.46

	# ---

	pushw.local.48
	pushw.local.49
	push.416595521271101505.416595521271101505.416595521271101505.416595521271101505

	exec.butterfly

	popw.local.49
	popw.local.48

	# ---

	pushw.local.50
	pushw.local.51
	push.18182056015521604139.18182056015521604139.18182056015521604139.18182056015521604139

	exec.butterfly

	popw.local.51
	popw.local.50

	# ---

	pushw.local.52
	pushw.local.53
	push.7059463857684370340.7059463857684370340.7059463857684370340.7059463857684370340

	exec.butterfly

	popw.local.53
	popw.local.52

	# ---

	pushw.local.54
	pushw.local.55
	push.7737793303239342069.7737793303239342069.7737793303239342069.7737793303239342069

	exec.butterfly

	popw.local.55
	popw.local.54

	# ---

	pushw.local.56
	pushw.local.57
	push.15951685255325333175.15951685255325333175.15951685255325333175.15951685255325333175

	exec.butterfly

	popw.local.57
	popw.local.56

	# ---

	pushw.local.58
	pushw.local.59
	push.9516004302527281633.9516004302527281633.9516004302527281633.9516004302527281633

	exec.butterfly

	popw.local.59
	popw.local.58

	# ---

	pushw.local.60
	pushw.local.61
	push.9274800740290006948.9274800740290006948.9274800740290006948.9274800740290006948

	exec.butterfly

	popw.local.61
	popw.local.60

	# ---

	pushw.local.62
	pushw.local.63
	push.4195631349813649467.4195631349813649467.4195631349813649467.4195631349813649467

	exec.butterfly

	popw.local.63
	popw.local.62

	# ---

	pushw.local.64
	pushw.local.65
	push.5575382163818481237.5575382163818481237.5575382163818481237.5575382163818481237

	exec.butterfly

	popw.local.65
	popw.local.64

	# ---

	pushw.local.66
	pushw.local.67
	push.4404853092538523347.4404853092538523347.4404853092538523347.4404853092538523347

	exec.butterfly

	popw.local.67
	popw.local.66

	# ---

	pushw.local.68
	pushw.local.69
	push.8288405288461869359.8288405288461869359.8288405288461869359.8288405288461869359

	exec.butterfly

	popw.local.69
	popw.local.68

	# ---

	pushw.local.70
	pushw.local.71
	push.9952623958621855812.9952623958621855812.9952623958621855812.9952623958621855812

	exec.butterfly

	popw.local.71
	popw.local.70

	# ---

	pushw.local.72
	pushw.local.73
	push.1356658891109943458.1356658891109943458.1356658891109943458.1356658891109943458

	exec.butterfly

	popw.local.73
	popw.local.72

	# ---

	pushw.local.74
	pushw.local.75
	push.7298973816981743824.7298973816981743824.7298973816981743824.7298973816981743824

	exec.butterfly

	popw.local.75
	popw.local.74

	# ---

	pushw.local.76
	pushw.local.77
	push.18142929134658341675.18142929134658341675.18142929134658341675.18142929134658341675

	exec.butterfly

	popw.local.77
	popw.local.76

	# ---

	pushw.local.78
	pushw.local.79
	push.1362567150328163374.1362567150328163374.1362567150328163374.1362567150328163374

	exec.butterfly

	popw.local.79
	popw.local.78

	# ---

	pushw.local.80
	pushw.local.81
	push.5029422726070465669.5029422726070465669.5029422726070465669.5029422726070465669

	exec.butterfly

	popw.local.81
	popw.local.80

	# ---

	pushw.local.82
	pushw.local.83
	push.17449332314429639298.17449332314429639298.17449332314429639298.17449332314429639298

	exec.butterfly

	popw.local.83
	popw.local.82

	# ---

	pushw.local.84
	pushw.local.85
	push.13039192753378044028.13039192753378044028.13039192753378044028.13039192753378044028

	exec.butterfly

	popw.local.85
	popw.local.84

	# ---

	pushw.local.86
	pushw.local.87
	push.5965722551466996711.5965722551466996711.5965722551466996711.5965722551466996711

	exec.butterfly

	popw.local.87
	popw.local.86

	# ---

	pushw.local.88
	pushw.local.89
	push.6336321165505697069.6336321165505697069.6336321165505697069.6336321165505697069

	exec.butterfly

	popw.local.89
	popw.local.88

	# ---

	pushw.local.90
	pushw.local.91
	push.5209436881246729393.5209436881246729393.5209436881246729393.5209436881246729393

	exec.butterfly

	popw.local.91
	popw.local.90

	# ---

	pushw.local.92
	pushw.local.93
	push.13949104517951277988.13949104517951277988.13949104517951277988.13949104517951277988

	exec.butterfly

	popw.local.93
	popw.local.92

	# ---

	pushw.local.94
	pushw.local.95
	push.9778634991702905054.9778634991702905054.9778634991702905054.9778634991702905054

	exec.butterfly

	popw.local.95
	popw.local.94

	# ---

	pushw.local.96
	pushw.local.97
	push.14004640413449681173.14004640413449681173.14004640413449681173.14004640413449681173

	exec.butterfly

	popw.local.97
	popw.local.96

	# ---

	pushw.local.98
	pushw.local.99
	push.912371727122717978.912371727122717978.912371727122717978.912371727122717978

	exec.butterfly

	popw.local.99
	popw.local.98

	# ---

	pushw.local.100
	pushw.local.101
	push.13797081185216407910.13797081185216407910.13797081185216407910.13797081185216407910

	exec.butterfly

	popw.local.101
	popw.local.100

	# ---

	pushw.local.102
	pushw.local.103
	push.4782006911144666502.4782006911144666502.4782006911144666502.4782006911144666502

	exec.butterfly

	popw.local.103
	popw.local.102

	# ---

	pushw.local.104
	pushw.local.105
	push.3341893669734556710.3341893669734556710.3341893669734556710.3341893669734556710

	exec.butterfly

	popw.local.105
	popw.local.104

	# ---

	pushw.local.106
	pushw.local.107
	push.10467450029535024137.10467450029535024137.10467450029535024137.10467450029535024137

	exec.butterfly

	popw.local.107
	popw.local.106

	# ---

	pushw.local.108
	pushw.local.109
	push.12079821679951430619.12079821679951430619.12079821679951430619.12079821679951430619

	exec.butterfly

	popw.local.109
	popw.local.108

	# ---

	pushw.local.110
	pushw.local.111
	push.10832292272906805046.10832292272906805046.10832292272906805046.10832292272906805046

	exec.butterfly

	popw.local.111
	popw.local.110

	# ---

	pushw.local.112
	pushw.local.113
	push.7709569171718681254.7709569171718681254.7709569171718681254.7709569171718681254

	exec.butterfly

	popw.local.113
	popw.local.112

	# ---

	pushw.local.114
	pushw.local.115
	push.16792080670893602455.16792080670893602455.16792080670893602455.16792080670893602455

	exec.butterfly

	popw.local.115
	popw.local.114

	# ---

	pushw.local.116
	pushw.local.117
	push.10967010099451201909.10967010099451201909.10967010099451201909.10967010099451201909

	exec.butterfly

	popw.local.117
	popw.local.116

	# ---

	pushw.local.118
	pushw.local.119
	push.5834015391316509212.5834015391316509212.5834015391316509212.5834015391316509212

	exec.butterfly

	popw.local.119
	popw.local.118

	# ---

	pushw.local.120
	pushw.local.121
	push.10853271128879547664.10853271128879547664.10853271128879547664.10853271128879547664

	exec.butterfly

	popw.local.121
	popw.local.120

	# ---

	pushw.local.122
	pushw.local.123
	push.3051558327610197629.3051558327610197629.3051558327610197629.3051558327610197629

	exec.butterfly

	popw.local.123
	popw.local.122

	# ---

	pushw.local.124
	pushw.local.125
	push.16016224591364643153.16016224591364643153.16016224591364643153.16016224591364643153

	exec.butterfly

	popw.local.125
	popw.local.124

	# ---

	pushw.local.126
	pushw.local.127
	push.10900537202625306992.10900537202625306992.10900537202625306992.10900537202625306992

	exec.butterfly

	popw.local.127
	popw.local.126

	# iter = 7

    pushw.local.0
    pushw.local.1

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.8286160002038086708.8286160002038086708.1644572010096941946.1644572010096941946

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.1
    popw.local.0

    # ---

    pushw.local.2
    pushw.local.3

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.9780749169175637327.9780749169175637327.6979306088310177371.6979306088310177371

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.3
    popw.local.2

    # ---

    pushw.local.4
    pushw.local.5

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13231174195295398387.13231174195295398387.4379521825066653820.4379521825066653820

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.5
    popw.local.4

    # ---

    pushw.local.6
    pushw.local.7

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.16549024694582589649.16549024694582589649.3105368020750933651.3105368020750933651

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.7
    popw.local.6

    # ---

    pushw.local.8
    pushw.local.9

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.14276112633913910454.14276112633913910454.10773575572760153082.10773575572760153082

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.9
    popw.local.8

    # ---

    pushw.local.10
    pushw.local.11

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.16691665375249202323.16691665375249202323.3588235763047079665.3588235763047079665

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.11
    popw.local.10

    # ---

    pushw.local.12
    pushw.local.13

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13805406186829188324.13805406186829188324.13018888299131362939.13018888299131362939

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.13
    popw.local.12

    # ---

    pushw.local.14
    pushw.local.15

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.17225392536559506335.17225392536559506335.3953731985901328040.3953731985901328040

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.15
    popw.local.14

    # ---

    pushw.local.16
    pushw.local.17

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13183111817796039999.13183111817796039999.9770812262840623888.9770812262840623888

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.17
    popw.local.16

    # ---

    pushw.local.18
    pushw.local.19

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.18209529147560584987.18209529147560584987.11917386045977981907.11917386045977981907

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.19
    popw.local.18

    # ---

    pushw.local.20
    pushw.local.21

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.3528436654823777706.3528436654823777706.12401628304422887372.12401628304422887372

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.21
    popw.local.20

    # ---

    pushw.local.22
    pushw.local.23

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.4406114516091528337.4406114516091528337.10259142034962052999.10259142034962052999

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.23
    popw.local.22

    # ---

    pushw.local.24
    pushw.local.25

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.10949047808060940701.10949047808060940701.13156576080775535568.13156576080775535568

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.25
    popw.local.24

    # ---

    pushw.local.26
    pushw.local.27

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.4459017075746761332.4459017075746761332.494216498237666005.494216498237666005

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.27
    popw.local.26

    # ---

    pushw.local.28
    pushw.local.29

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13615673215290265491.13615673215290265491.16589430531118646239.16589430531118646239

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.29
    popw.local.28

    # ---

    pushw.local.30
    pushw.local.31

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.3264989070758626945.3264989070758626945.6396200096592884887.6396200096592884887

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.31
    popw.local.30

    # ---

    pushw.local.32
    pushw.local.33

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13376768784840513824.13376768784840513824.12499229437757822825.12499229437757822825

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.33
    popw.local.32

    # ---

    pushw.local.34
    pushw.local.35

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.17571109804126144978.17571109804126144978.12184322017746068437.12184322017746068437

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.35
    popw.local.34

    # ---

    pushw.local.36
    pushw.local.37

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.8540276921445729647.8540276921445729647.7929601155018190654.7929601155018190654

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.37
    popw.local.36

    # ---

    pushw.local.38
    pushw.local.39

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.4415056545429189734.4415056545429189734.7128984430570800425.7128984430570800425

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.39
    popw.local.38

    # ---

    pushw.local.40
    pushw.local.41

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13533145890581203496.13533145890581203496.12584286203165206160.12584286203165206160

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.41
    popw.local.40

    # ---

    pushw.local.42
    pushw.local.43

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.11622144959503752099.11622144959503752099.9432384046970425189.9432384046970425189

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.43
    popw.local.42

    # ---

    pushw.local.44
    pushw.local.45

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.7562975036722005970.7562975036722005970.6740689031673534997.6740689031673534997

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.45
    popw.local.44

    # ---

    pushw.local.46
    pushw.local.47

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.17746383299198219332.17746383299198219332.5033358220335838486.5033358220335838486

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.47
    popw.local.46

    # ---

    pushw.local.48
    pushw.local.49

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.3373377623857539246.3373377623857539246.5602886161730919912.5602886161730919912

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.49
    popw.local.48

    # ---

    pushw.local.50
    pushw.local.51

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.5163568085532294797.5163568085532294797.17032024114559111334.17032024114559111334

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.51
    popw.local.50

    # ---

    pushw.local.52
    pushw.local.53

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.16031446777576706363.16031446777576706363.8440569278248727675.8440569278248727675

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.53
    popw.local.52

    # ---

    pushw.local.54
    pushw.local.55

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.743439328957095187.743439328957095187.1672096098105064228.1672096098105064228

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.55
    popw.local.54

    # ---

    pushw.local.56
    pushw.local.57

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.14780429931651188987.14780429931651188987.7760115154989660995.7760115154989660995

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.57
    popw.local.56

    # ---

    pushw.local.58
    pushw.local.59

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.11441669947107069577.11441669947107069577.5240855794895625891.5240855794895625891

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.59
    popw.local.58

    # ---

    pushw.local.60
    pushw.local.61

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.12981983163322084213.12981983163322084213.8096577031901772269.8096577031901772269

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.61
    popw.local.60

    # ---

    pushw.local.62
    pushw.local.63

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.16873708294018933551.16873708294018933551.1691643236322650437.1691643236322650437

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.63
    popw.local.62

    # ---

    pushw.local.64
    pushw.local.65

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.7159778541829602319.7159778541829602319.6968564197111712876.6968564197111712876

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.65
    popw.local.64

    # ---

    pushw.local.66
    pushw.local.67

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.4016101032690928304.4016101032690928304.6434636298004421797.6434636298004421797

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.67
    popw.local.66

    # ---

    pushw.local.68
    pushw.local.69

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.14319745502085270124.14319745502085270124.4545880015766881148.4545880015766881148

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.69
    popw.local.68

    # ---

    pushw.local.70
    pushw.local.71

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.14576581034276612555.14576581034276612555.6125875985213995509.6125875985213995509

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.71
    popw.local.70

    # ---

    pushw.local.72
    pushw.local.73

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.4674437595989441835.4674437595989441835.7882761346440596851.7882761346440596851

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.73
    popw.local.72

    # ---

    pushw.local.74
    pushw.local.75

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.12573252732142656207.12573252732142656207.14235159967861628657.14235159967861628657

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.75
    popw.local.74

    # ---

    pushw.local.76
    pushw.local.77

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.15503969011144524712.15503969011144524712.3266250949199600360.3266250949199600360

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.77
    popw.local.76

    # ---

    pushw.local.78
    pushw.local.79

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.17222793189829815283.17222793189829815283.5988353545162139946.5988353545162139946

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.79
    popw.local.78

    # ---

    pushw.local.80
    pushw.local.81

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.11013340222467950926.11013340222467950926.9791607036678152304.9791607036678152304

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.81
    popw.local.80

    # ---

    pushw.local.82
    pushw.local.83

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13351287672668691770.13351287672668691770.7683263524182218559.7683263524182218559

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.83
    popw.local.82

    # ---

    pushw.local.84
    pushw.local.85

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.502012629086366038.502012629086366038.7721858563281021845.7721858563281021845

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.85
    popw.local.84

    # ---

    pushw.local.86
    pushw.local.87

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.8352301510068328051.8352301510068328051.3200815326405523330.3200815326405523330

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.87
    popw.local.86

    # ---

    pushw.local.88
    pushw.local.89

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.1937996126393065589.1937996126393065589.408281368649950045.408281368649950045

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.89
    popw.local.88

    # ---

    pushw.local.90
    pushw.local.91

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13682064192112842111.13682064192112842111.14583602245206205734.14583602245206205734

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.91
    popw.local.90

    # ---

    pushw.local.92
    pushw.local.93

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.3877499600194655066.3877499600194655066.17920296056720464863.17920296056720464863

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.93
    popw.local.92

    # ---

    pushw.local.94
    pushw.local.95

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.5932183857725394514.5932183857725394514.12113519742882795430.12113519742882795430

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.95
    popw.local.94

    # ---

    pushw.local.96
    pushw.local.97

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.11744640894413513105.11744640894413513105.8807895225777549048.8807895225777549048

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.97
    popw.local.96

    # ---

    pushw.local.98
    pushw.local.99

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.802080937612788754.802080937612788754.6084072299099782489.6084072299099782489

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.99
    popw.local.98

    # ---

    pushw.local.100
    pushw.local.101

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.17255643403020241594.17255643403020241594.16643667963227857075.16643667963227857075

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.101
    popw.local.100

    # ---

    pushw.local.102
    pushw.local.103

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.15387314553928353233.15387314553928353233.13754189079328553053.13754189079328553053

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.103
    popw.local.102

    # ---

    pushw.local.104
    pushw.local.105

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13271129814541932305.13271129814541932305.11336048296972946422.11336048296972946422

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.105
    popw.local.104

    # ---

    pushw.local.106
    pushw.local.107

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.16003277697834987077.16003277697834987077.13730337689951546503.13730337689951546503

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.107
    popw.local.106

    # ---

    pushw.local.108
    pushw.local.109

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13787254465881465880.13787254465881465880.10302972367325609442.10302972367325609442

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.109
    popw.local.108

    # ---

    pushw.local.110
    pushw.local.111

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.14439691868389311614.14439691868389311614.1999001684679808555.1999001684679808555

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.111
    popw.local.110

    # ---

    pushw.local.112
    pushw.local.113

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.15992013477438468440.15992013477438468440.13609673538787597335.13609673538787597335

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.113
    popw.local.112

    # ---

    pushw.local.114
    pushw.local.115

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.18064315379978805435.18064315379978805435.8636802660946538252.8636802660946538252

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.115
    popw.local.114

    # ---

    pushw.local.116
    pushw.local.117

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13935318169262536835.13935318169262536835.16901410098125234092.16901410098125234092

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.117
    popw.local.116

    # ---

    pushw.local.118
    pushw.local.119

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.17345757166192390690.17345757166192390690.17608981172539450419.17608981172539450419

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.119
    popw.local.118

    # ---

    pushw.local.120
    pushw.local.121

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.1723406808235183235.1723406808235183235.15122929597976639421.15122929597976639421

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.121
    popw.local.120

    # ---

    pushw.local.122
    pushw.local.123

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.6416647500902310032.6416647500902310032.11779090253969091270.11779090253969091270

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.123
    popw.local.122

    # ---

    pushw.local.124
    pushw.local.125

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.8917938738259842505.8917938738259842505.4022135219920766353.4022135219920766353

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.125
    popw.local.124

    # ---

    pushw.local.126
    pushw.local.127

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.12418052014939319938.12418052014939319938.17799792287555502819.17799792287555502819

    exec.butterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    popw.local.127
    popw.local.126

    # iter = 8

    pushw.local.0
    pushw.local.1

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.8911053381972245530.2877957390243263830.2951359516584421996.19112242249724047

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.1
    popw.local.0

    # ---

    pushw.local.2
    pushw.local.3

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.6151214463239765361.4496767977211359228.644010080489266561.6431860813144680379

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.3
    popw.local.2

    # ---

    pushw.local.4
    pushw.local.5

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.4323157012483891262.5810722514138689194.11091989500308225777.12150643879775871958

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.5
    popw.local.4

    # ---

    pushw.local.6
    pushw.local.7

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.16905094363786184290.18168576350837626231.4419568367257164534.1223183503982339008

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.7
    popw.local.6

    # ---

    pushw.local.8
    pushw.local.9

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.16909802868642731951.9785468031858712064.16221402320798919601.12333197645027200248

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.9
    popw.local.8

    # ---

    pushw.local.10
    pushw.local.11

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.16138512030456545775.9592291974280344910.14948939724807468932.4971430691134054059

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.11
    popw.local.10

    # ---

    pushw.local.12
    pushw.local.13

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.15948194847534211277.4576915052531526319.5164132063260791647.152897937997792376

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.13
    popw.local.12

    # ---

    pushw.local.14
    pushw.local.15

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.12316227567088954246.17527399748276289503.5152080643914132488.14561398366328274390

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.15
    popw.local.14

    # ---

    pushw.local.16
    pushw.local.17

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.14099721646927849786.8024399707039913807.15913274187758939207.18074852694000884838

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.17
    popw.local.16

    # ---

    pushw.local.18
    pushw.local.19

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.8424275818888585779.7812684066897416275.14290012408112277771.4295815520590785595

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.19
    popw.local.18

    # ---

    pushw.local.20
    pushw.local.21

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.10670334717871145615.16677776346006097586.1949690407240864933.14248669673568039774

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.21
    popw.local.20

    # ---

    pushw.local.22
    pushw.local.23

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.16938470071482338896.15499491376360706981.3878624198769971593.13092440112352401730

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.23
    popw.local.22

    # ---

    pushw.local.24
    pushw.local.25

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.12582249520745188423.12505800551746292235.13315466594398149922.12066191983457963400

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.25
    popw.local.24

    # ---

    pushw.local.26
    pushw.local.27

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.11575701465310827636.4295002282146690441.15597523257926919464.3308892972056812266

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.27
    popw.local.26

    # ---

    pushw.local.28
    pushw.local.29

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.2117308758935292362.8854965448075557493.16625729085584007730.15471613066104988457

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.29
    popw.local.28

    # ---

    pushw.local.30
    pushw.local.31

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.12053974342864933269.7161240326935577237.3639634848410716242.15919780095311700439

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.31
    popw.local.30

    # ---

    pushw.local.32
    pushw.local.33

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.9931515098122800394.11630195332861834531.11724314991892485077.17740512949860132546

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.33
    popw.local.32

    # ---

    pushw.local.34
    pushw.local.35

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.5919394105455887829.3416153203055267997.7786896173617522154.14031575217582598302

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.35
    popw.local.34

    # ---

    pushw.local.36
    pushw.local.37

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.299265237327641189.12577098593386243920.15719620231976724277.8540402708529449685

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.37
    popw.local.36

    # ---

    pushw.local.38
    pushw.local.39

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.8427667919763358302.6462738526574037144.12486396704535672088.10141440556758839363

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.39
    popw.local.38

    # ---

    pushw.local.40
    pushw.local.41

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.7657453289212455099.7344548176412377620.14808420073763128510.6365632919551470868

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.41
    popw.local.40

    # ---

    pushw.local.42
    pushw.local.43

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.2394121898621129512.8383068400017029755.15076497439326288290.12982989459991844517

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.43
    popw.local.42

    # ---

    pushw.local.44
    pushw.local.45

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.5665144507324065868.807842315821754643.1560799588066959011.12796895112978970121

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.45
    popw.local.44

    # ---

    pushw.local.46
    pushw.local.47

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.10461664704817933990.8882481555027559655.6954937180696424269.1572137324173280490

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.47
    popw.local.46

    # ---

    pushw.local.48
    pushw.local.49

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.16651939688552765673.3158366299580748670.1392595059675174803.10765599713046287558

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.49
    popw.local.48

    # ---

    pushw.local.50
    pushw.local.51

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.4025446980409437899.8178098736737310378.5500770423122943299.9714604383004622450

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.51
    popw.local.50

    # ---

    pushw.local.52
    pushw.local.53

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.1561169760991269037.12992126221614554207.6889485207579503204.625810225600154958

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.53
    popw.local.52

    # ---

    pushw.local.54
    pushw.local.55

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.14259728110745696775.17668002479022071670.15339107541552850108.6468851066622783835

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.55
    popw.local.54

    # ---

    pushw.local.56
    pushw.local.57

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.12032395915935294938.14857320394153102038.12216811346274483113.15049383599936516047

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.57
    popw.local.56

    # ---

    pushw.local.58
    pushw.local.59

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.12489358087930152296.11703289425843512051.18222393521806856990.5006481804801239664

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.59
    popw.local.58

    # ---

    pushw.local.60
    pushw.local.61

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.4088309022520035137.6820186327231405039.11140760477401398424.12337821426711963180

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.61
    popw.local.60

    # ---

    pushw.local.62
    pushw.local.63

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.13756831773860918871.10084557685654730061.7112675246154377750.3929858786378642316

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.63
    popw.local.62

    # ---

    pushw.local.64
    pushw.local.65

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.8715504128117593387.432040889165782054.3142428394956321713.1849525312019627755

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.65
    popw.local.64

    # ---

    pushw.local.66
    pushw.local.67

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.14006089359128464711.12490609572415712870.17198795428657782689.14191609616972732304

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.67
    popw.local.66

    # ---

    pushw.local.68
    pushw.local.69

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.12365007338637617157.4372556084940235727.6189017649778497877.7500740417092890225

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.69
    popw.local.68

    # ---

    pushw.local.70
    pushw.local.71

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.4389942117088447138.9203872837195467135.16647976583058746422.7689155552768670394

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.71
    popw.local.70

    # ---

    pushw.local.72
    pushw.local.73

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.4056604178567881129.6173012213905610189.18290750489319984117.1773951202121591538

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.73
    popw.local.72

    # ---

    pushw.local.74
    pushw.local.75

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.6686338362028015651.16533704610107301495.12618653059398814374.4665691128499368837

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.75
    popw.local.74

    # ---

    pushw.local.76
    pushw.local.77

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.14383800816696994133.3456327113326256432.6692683090235989383.14796202496157022040

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.77
    popw.local.76

    # ---

    pushw.local.78
    pushw.local.79

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.1368250456540211762.7691156232252781355.8463154943360171265.2852412519294352506

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.79
    popw.local.78

    # ---

    pushw.local.80
    pushw.local.81

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.327930691828598087.5800932517989445135.14262353213520121100.11221484848131637518

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.81
    popw.local.80

    # ---

    pushw.local.82
    pushw.local.83

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.16207038811842065314.12362461035457730117.1213232278782667512.3408203337326891081

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.83
    popw.local.82

    # ---

    pushw.local.84
    pushw.local.85

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.3859889564432383484.15210828825360601653.16434255353882186006.14213927998739126201

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.85
    popw.local.84

    # ---

    pushw.local.86
    pushw.local.87

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.2540820207615693247.2324799763032802220.8900146263973118671.17198755642670596954

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.87
    popw.local.86

    # ---

    pushw.local.88
    pushw.local.89

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.15860937903541196405.8462836655462685385.151654034847833439.16566926477903622666

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.89
    popw.local.88

    # ---

    pushw.local.90
    pushw.local.91

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.12432372446044483551.11006166186397307298.2346834345155397801.3030959573425503682

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.91
    popw.local.90

    # ---

    pushw.local.92
    pushw.local.93

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.2623445534628784696.9513972005086392438.3418361291673462874.15984902507394762860

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.93
    popw.local.92

    # ---

    pushw.local.94
    pushw.local.95

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.529102008834432265.6665967936588919331.9705858230261340096.8818882629200544327

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.95
    popw.local.94

    # ---

    pushw.local.96
    pushw.local.97

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.11255984160303063976.17121841670624273282.748583878869661002.13140475237632941313

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.97
    popw.local.96

    # ---

    pushw.local.98
    pushw.local.99

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.4040052327310466906.14234122862185153691.14989275032188358951.12349052935110756804

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.99
    popw.local.98

    # ---

    pushw.local.100
    pushw.local.101

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.82910450496588172.15576136931675893974.7093403778535204495.18137812093348882831

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.101
    popw.local.100

    # ---

    pushw.local.102
    pushw.local.103

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.959967552227305945.7439966824493015109.11015880108829135486.10886932084851949587

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.103
    popw.local.102

    # ---

    pushw.local.104
    pushw.local.105

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.14340064592974746604.13308480401157259412.4179502387700367909.10767003651596136761

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.105
    popw.local.104

    # ---

    pushw.local.106
    pushw.local.107

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.663283603972705376.13928631036919645866.1406998020037882997.15975288260888972401

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.107
    popw.local.106

    # ---

    pushw.local.108
    pushw.local.109

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.16260897004766174524.7847524879092096009.5988671030957288016.12890081553990608899

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.109
    popw.local.108

    # ---

    pushw.local.110
    pushw.local.111

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.13873674549069150927.3192518480993723602.9233735841019365682.6558703133813132827

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.111
    popw.local.110

    # ---

    pushw.local.112
    pushw.local.113

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.2761102078703419584.2915568066736270329.5308610189164171624.5350065414412465710

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.113
    popw.local.112

    # ---

    pushw.local.114
    pushw.local.115

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.13802821046066641766.17582727038347959133.7123388440527211897.16826744251348157030

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.115
    popw.local.114

    # ---

    pushw.local.116
    pushw.local.117

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.13175002527791537704.7000476060236159302.43142219979740931.2063168383634974384

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.117
    popw.local.116

    # ---

    pushw.local.118
    pushw.local.119

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.10689836412287594487.2128915576975457846.7709658857044466158.10362793272935287662

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.119
    popw.local.118

    # ---

    pushw.local.120
    pushw.local.121

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.6337038648111976301.9115369905823964012.17031324615803662768.6715029048772165709

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.121
    popw.local.120

    # ---

    pushw.local.122
    pushw.local.123

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.13166299875259380027.663576273645521453.345137759837927448.16505347069079795072

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.123
    popw.local.122

    # ---

    pushw.local.124
    pushw.local.125

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.3642072560212772351.4877800464475578311.5575393374484204350.5907035176470557038

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.125
    popw.local.124

    # ---

    pushw.local.126
    pushw.local.127

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.18188848021460212523.11534607820881582817.1646875315973942213.5486745524883165993

    exec.butterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    popw.local.127
    popw.local.126

    # begin asserting result

    pushw.local.0

    push.6147698371245747120
    assert.eq
    push.13255236560399798415
    assert.eq
    push.2463011040099663974
    assert.eq
    push.500427581402858571
    assert.eq

    pushw.local.1

    push.9387934912362961312
    assert.eq
    push.7549830671545879423
    assert.eq
    push.2807061946257421748
    assert.eq
    push.4759188580461308943
    assert.eq

    pushw.local.2

    push.9379941516119320882
    assert.eq
    push.5463045908626770304
    assert.eq
    push.16298308954297267166
    assert.eq
    push.1388192801295105971
    assert.eq

    pushw.local.3

    push.3025227557696320612
    assert.eq
    push.13701360230683392458
    assert.eq
    push.7837612199872064446
    assert.eq
    push.1822341659846948200
    assert.eq

    pushw.local.4

    push.11593484223257809456
    assert.eq
    push.10287011748290197204
    assert.eq
    push.2462700283595696130
    assert.eq
    push.6661003217299415345
    assert.eq

    pushw.local.5

    push.4850126366893939331
    assert.eq
    push.9513121813828969915
    assert.eq
    push.13374962778532735081
    assert.eq
    push.5822565313136645408
    assert.eq

    pushw.local.6

    push.8639307051715995157
    assert.eq
    push.17760129267519767490
    assert.eq
    push.1284486619460005108
    assert.eq
    push.9638547351911424431
    assert.eq

    pushw.local.7

    push.17637713302469968180
    assert.eq
    push.14285126964169992246
    assert.eq
    push.12823255455035621696
    assert.eq
    push.13238262168040768060
    assert.eq

    pushw.local.8

    push.14223921105393507681
    assert.eq
    push.1357100511086411291
    assert.eq
    push.8090504461116217953
    assert.eq
    push.15517318235210799523
    assert.eq

    pushw.local.9

    push.16628668316477991361
    assert.eq
    push.1684552264558936468
    assert.eq
    push.4716997638082670922
    assert.eq
    push.11495840209035318117
    assert.eq

    pushw.local.10

    push.8712021422542957173
    assert.eq
    push.17813839478541020385
    assert.eq
    push.4375032670965432631
    assert.eq
    push.9373982051891349673
    assert.eq

    pushw.local.11

    push.1926093556853850187
    assert.eq
    push.14044826173259891759
    assert.eq
    push.15061433824670536866
    assert.eq
    push.7963379320027168585
    assert.eq

    pushw.local.12

    push.14691778372873896091
    assert.eq
    push.14431246337082958746
    assert.eq
    push.11152590746239846097
    assert.eq
    push.17117618612841673432
    assert.eq

    pushw.local.13

    push.14862669681256781194
    assert.eq
    push.4084063692504982245
    assert.eq
    push.16546231301940396830
    assert.eq
    push.303890312557052486
    assert.eq

    pushw.local.14

    push.7563043097586366000
    assert.eq
    push.1391337153954404998
    assert.eq
    push.8927299104241429512
    assert.eq
    push.12874831358695227040
    assert.eq

    pushw.local.15

    push.17544691169439260104
    assert.eq
    push.3025759491575349789
    assert.eq
    push.3598143036621308872
    assert.eq
    push.487169446989684856
    assert.eq

    pushw.local.16

    push.6666882237238639271
    assert.eq
    push.6080341855927780886
    assert.eq
    push.2882980834561558714
    assert.eq
    push.9893297249316795649
    assert.eq

    pushw.local.17

    push.4691550456846015466
    assert.eq
    push.3411987355997998953
    assert.eq
    push.11137670125329914006
    assert.eq
    push.3705911779901497798
    assert.eq

    pushw.local.18

    push.15518117179961526146
    assert.eq
    push.799757138718215649
    assert.eq
    push.18296196013336192157
    assert.eq
    push.15796413081962541352
    assert.eq

    pushw.local.19

    push.7347195004378104849
    assert.eq
    push.13196368854332472946
    assert.eq
    push.8162117950669587555
    assert.eq
    push.13210457250415703836
    assert.eq

    pushw.local.20

    push.5452650972361431851
    assert.eq
    push.657975828059970386
    assert.eq
    push.6266966273130402481
    assert.eq
    push.8906355104260221321
    assert.eq

    pushw.local.21

    push.1177534166750439230
    assert.eq
    push.14955072641990074669
    assert.eq
    push.5042479363983917261
    assert.eq
    push.8576758396778913845
    assert.eq

    pushw.local.22

    push.8896467651864889830
    assert.eq
    push.3941628179342546836
    assert.eq
    push.497910349234540620
    assert.eq
    push.6334563537974372527
    assert.eq

    pushw.local.23

    push.13460463077709850377
    assert.eq
    push.9704822807517896607
    assert.eq
    push.774530765448667322
    assert.eq
    push.12425671526490530033
    assert.eq

    pushw.local.24

    push.8065494508321203076
    assert.eq
    push.12498735210523199927
    assert.eq
    push.10225952875956458587
    assert.eq
    push.14318104668539212250
    assert.eq

    pushw.local.25

    push.6311439446657985780
    assert.eq
    push.6807078861963306588
    assert.eq
    push.8346724915334734524
    assert.eq
    push.12164956900973499431
    assert.eq

    pushw.local.26

    push.11445417186801385760
    assert.eq
    push.4239657429080771413
    assert.eq
    push.14244370490763271162
    assert.eq
    push.7234458743041469928
    assert.eq

    pushw.local.27

    push.2139508481948263956
    assert.eq
    push.5018146025947592012
    assert.eq
    push.14751110666906846153
    assert.eq
    push.11747418408334319738
    assert.eq

    pushw.local.28

    push.13564247872498662505
    assert.eq
    push.9021654833377757761
    assert.eq
    push.3702090447919014070
    assert.eq
    push.1973185003558773229
    assert.eq

    pushw.local.29

    push.4997611524108540570
    assert.eq
    push.10771985727518467339
    assert.eq
    push.3004215856372485490
    assert.eq
    push.3965062879362346539
    assert.eq

    pushw.local.30

    push.2219769481339195266
    assert.eq
    push.7706343149895562665
    assert.eq
    push.12465933509381032447
    assert.eq
    push.2312056974030883522
    assert.eq

    pushw.local.31

    push.5022786165037908972
    assert.eq
    push.2125184873369089124
    assert.eq
    push.9033129468865877333
    assert.eq
    push.4146773039321337126
    assert.eq

    pushw.local.32

    push.4662969890869329954
    assert.eq
    push.17149781625007751176
    assert.eq
    push.985664331094757213
    assert.eq
    push.13678435790007910187
    assert.eq

    pushw.local.33

    push.10150484129565507076
    assert.eq
    push.8222509106562826527
    assert.eq
    push.18012673797650473443
    assert.eq
    push.6861895092303478630
    assert.eq

    pushw.local.34

    push.16844253818212940857
    assert.eq
    push.14908977434228205095
    assert.eq
    push.14429583432640497786
    assert.eq
    push.5096773607628371884
    assert.eq

    pushw.local.35

    push.12395699022945804024
    assert.eq
    push.12367986838157190529
    assert.eq
    push.4955617923334453253
    assert.eq
    push.7458036209579834773
    assert.eq

    pushw.local.36

    push.9143623102345260628
    assert.eq
    push.18020016569845241237
    assert.eq
    push.6515488115992785920
    assert.eq
    push.13890387097742396980
    assert.eq

    pushw.local.37

    push.12996593942828004366
    assert.eq
    push.17390390595735418203
    assert.eq
    push.16875779936086595852
    assert.eq
    push.2905443428502578517
    assert.eq

    pushw.local.38

    push.893260271205161783
    assert.eq
    push.9199250935105121089
    assert.eq
    push.13002876069586538529
    assert.eq
    push.18214974425583805276
    assert.eq

    pushw.local.39

    push.5871387860019782338
    assert.eq
    push.5760084267134403683
    assert.eq
    push.12587785846481248845
    assert.eq
    push.6709534089282777356
    assert.eq

    pushw.local.40

    push.3123643438350635974
    assert.eq
    push.5716597287927550595
    assert.eq
    push.9607664737753257441
    assert.eq
    push.7245215770567422587
    assert.eq

    pushw.local.41

    push.16881060344087771413
    assert.eq
    push.8078443735104605287
    assert.eq
    push.17071143378045471018
    assert.eq
    push.5759208819531861758
    assert.eq

    pushw.local.42

    push.12566331383071766254
    assert.eq
    push.6407770971883791349
    assert.eq
    push.141332871558446952
    assert.eq
    push.1473685036794685672
    assert.eq

    pushw.local.43

    push.469389931994699889
    assert.eq
    push.11894138910345911899
    assert.eq
    push.8861784141877776284
    assert.eq
    push.15369028662523164967
    assert.eq

    pushw.local.44

    push.15664808304394316343
    assert.eq
    push.16586192596775565345
    assert.eq
    push.2607653131583029365
    assert.eq
    push.13875913004469395297
    assert.eq

    pushw.local.45

    push.8009025452734193607
    assert.eq
    push.13448834210131967244
    assert.eq
    push.14161113041748059521
    assert.eq
    push.16813439921261944248
    assert.eq

    pushw.local.46

    push.13995070365201497714
    assert.eq
    push.6137042318318339109
    assert.eq
    push.6251459103334690891
    assert.eq
    push.4083354598206562910
    assert.eq

    pushw.local.47

    push.14493471866567307848
    assert.eq
    push.3997466371394620132
    assert.eq
    push.9249214068095605136
    assert.eq
    push.12818043094147068368
    assert.eq

    pushw.local.48

    push.13015011347722958547
    assert.eq
    push.3588540624239934438
    assert.eq
    push.15239864752341206448
    assert.eq
    push.5394043712193799933
    assert.eq

    pushw.local.49

    push.3128672686463738131
    assert.eq
    push.3419201204817655041
    assert.eq
    push.12130786777414201524
    assert.eq
    push.11829529434721723333
    assert.eq

    pushw.local.50

    push.5151809639838655931
    assert.eq
    push.15516781916364721257
    assert.eq
    push.7500291227370840461
    assert.eq
    push.6140842141236339257
    assert.eq

    pushw.local.51

    push.11690668512079064293
    assert.eq
    push.17726010777882042466
    assert.eq
    push.762882067218168658
    assert.eq
    push.13243563844154651511
    assert.eq

    pushw.local.52

    push.10558916504036356933
    assert.eq
    push.17101791001764243145
    assert.eq
    push.11725206893555211986
    assert.eq
    push.3918065202896176478
    assert.eq

    pushw.local.53

    push.803701966684189555
    assert.eq
    push.11567105031538459462
    assert.eq
    push.17149127386114289503
    assert.eq
    push.10298844172561774234
    assert.eq

    pushw.local.54

    push.2803053123421498344
    assert.eq
    push.16018809242226017839
    assert.eq
    push.1810114756857547179
    assert.eq
    push.17482417277959843804
    assert.eq

    pushw.local.55

    push.78962158302801786
    assert.eq
    push.13318542890811254177
    assert.eq
    push.8999334195551472243
    assert.eq
    push.13204537791056727921
    assert.eq

    pushw.local.56

    push.12393637219601567574
    assert.eq
    push.7592879583466719407
    assert.eq
    push.13355052672226195728
    assert.eq
    push.11093885641065163374
    assert.eq

    pushw.local.57

    push.14284568787846633340
    assert.eq
    push.6794795563368961656
    assert.eq
    push.2809894435780251284
    assert.eq
    push.3411599003810787776
    assert.eq

    pushw.local.58

    push.1349795128299762686
    assert.eq
    push.6924418765183651467
    assert.eq
    push.3431893456709396822
    assert.eq
    push.16299086042706000560
    assert.eq

    pushw.local.59

    push.1583932303300965007
    assert.eq
    push.16264631822018623161
    assert.eq
    push.10153763531676333194
    assert.eq
    push.5027175826503224555
    assert.eq

    pushw.local.60

    push.7807832181012033386
    assert.eq
    push.1942275972796109015
    assert.eq
    push.17405989941569656846
    assert.eq
    push.7218523740236699299
    assert.eq

    pushw.local.61

    push.16405583093082798411
    assert.eq
    push.2393469611227278774
    assert.eq
    push.10005260587118472398
    assert.eq
    push.5715345431563076262
    assert.eq

    pushw.local.62

    push.16988798032855610364
    assert.eq
    push.9496386899791548746
    assert.eq
    push.18142885242969242100
    assert.eq
    push.8089586839234703419
    assert.eq

    pushw.local.63

    push.4370480212321907950
    assert.eq
    push.6558160888738056325
    assert.eq
    push.4289754023734046593
    assert.eq
    push.13755333174520886464
    assert.eq

    pushw.local.64

    push.1494047469102986519
    assert.eq
    push.16339859305517413399
    assert.eq
    push.12562227996983300356
    assert.eq
    push.1095320300961666936
    assert.eq

    pushw.local.65

    push.10516920696937702284
    assert.eq
    push.5854381521638192751
    assert.eq
    push.14530409639472899115
    assert.eq
    push.854047760369733976
    assert.eq

    pushw.local.66

    push.10904098461973645865
    assert.eq
    push.17422775311721259180
    assert.eq
    push.4139499927901685601
    assert.eq
    push.2794790736470622544
    assert.eq

    pushw.local.67

    push.7819865318527720810
    assert.eq
    push.9250129350657951385
    assert.eq
    push.4234979558599665650
    assert.eq
    push.1784571835427036944
    assert.eq

    pushw.local.68

    push.16289356085892318230
    assert.eq
    push.13588187127040443153
    assert.eq
    push.10045192632325951454
    assert.eq
    push.18142115543028914412
    assert.eq

    pushw.local.69

    push.8111707298035988643
    assert.eq
    push.11430590887347575920
    assert.eq
    push.539684312007626408
    assert.eq
    push.17729711269237440361
    assert.eq

    pushw.local.70

    push.11442474927709952764
    assert.eq
    push.13329490012213926327
    assert.eq
    push.6013205331994689614
    assert.eq
    push.8428499566768654683
    assert.eq

    pushw.local.71

    push.8762253157562392538
    assert.eq
    push.10586391764183012409
    assert.eq
    push.15471829253544609840
    assert.eq
    push.16150914592979533613
    assert.eq

    pushw.local.72

    push.8190839155548334141
    assert.eq
    push.11004455288779489872
    assert.eq
    push.17741352459101681712
    assert.eq
    push.6392279585533100357
    assert.eq

    pushw.local.73

    push.3457721476658561580
    assert.eq
    push.11015187404152435362
    assert.eq
    push.14880425420800392990
    assert.eq
    push.4226216350017515577
    assert.eq

    pushw.local.74

    push.2187849759750703504
    assert.eq
    push.14281378611832340921
    assert.eq
    push.7612415948668645325
    assert.eq
    push.14629100830552568255
    assert.eq

    pushw.local.75

    push.1045809343595143126
    assert.eq
    push.5837373918705201518
    assert.eq
    push.2524681089307350526
    assert.eq
    push.877672128107013028
    assert.eq

    pushw.local.76

    push.375546531597716834
    assert.eq
    push.1828356708926881600
    assert.eq
    push.11259226891969400454
    assert.eq
    push.5920559297162622495
    assert.eq

    pushw.local.77

    push.1308096718102429686
    assert.eq
    push.4953050491861030149
    assert.eq
    push.821000236133808134
    assert.eq
    push.7917164687481063958
    assert.eq

    pushw.local.78

    push.13281899104990252257
    assert.eq
    push.18151980406669433196
    assert.eq
    push.7468748459141628680
    assert.eq
    push.16363914024650516083
    assert.eq

    pushw.local.79

    push.4849705256848202365
    assert.eq
    push.15393692351097644975
    assert.eq
    push.5915985290118060008
    assert.eq
    push.8668665576569166951
    assert.eq

    pushw.local.80

    push.8300446822269204372
    assert.eq
    push.12628082128538211214
    assert.eq
    push.15384439712454326481
    assert.eq
    push.9554263480822607717
    assert.eq

    pushw.local.81

    push.12238253882750553858
    assert.eq
    push.7292662648869841962
    assert.eq
    push.2979547823604997972
    assert.eq
    push.14732015482137704088
    assert.eq

    pushw.local.82

    push.17320170229660244276
    assert.eq
    push.16536824509534370931
    assert.eq
    push.7198903717206004644
    assert.eq
    push.16519415090692894356
    assert.eq

    pushw.local.83

    push.7201793695830885568
    assert.eq
    push.14758645042887458947
    assert.eq
    push.1309308834650053177
    assert.eq
    push.14338251575625791894
    assert.eq

    pushw.local.84

    push.16250160353761920792
    assert.eq
    push.14516922055619556761
    assert.eq
    push.14537183770973180315
    assert.eq
    push.5716209785352087904
    assert.eq

    pushw.local.85

    push.8662993271246423627
    assert.eq
    push.3400167661560536242
    assert.eq
    push.12061541935646977447
    assert.eq
    push.3173145240318099632
    assert.eq

    pushw.local.86

    push.9779983570882049575
    assert.eq
    push.16797125524719969353
    assert.eq
    push.14364377592553572211
    assert.eq
    push.8719325338143121381
    assert.eq

    pushw.local.87

    push.3441777122008563845
    assert.eq
    push.13369371497705876272
    assert.eq
    push.14761196745008075213
    assert.eq
    push.5311413155140064380
    assert.eq

    pushw.local.88

    push.5334184973660170182
    assert.eq
    push.13603662903454429611
    assert.eq
    push.2543790951468878923
    assert.eq
    push.6874176837830348960
    assert.eq

    pushw.local.89

    push.50931374536222867
    assert.eq
    push.10680417757191732550
    assert.eq
    push.2027790285986339592
    assert.eq
    push.2052697753100968697
    assert.eq

    pushw.local.90

    push.17520109592597401123
    assert.eq
    push.17107102851440519601
    assert.eq
    push.8813959523570730359
    assert.eq
    push.8520311447926067785
    assert.eq

    pushw.local.91

    push.13694145796795458120
    assert.eq
    push.4206605768450390142
    assert.eq
    push.12164864114726281851
    assert.eq
    push.15783770904168829462
    assert.eq

    pushw.local.92

    push.5841129472983407877
    assert.eq
    push.16936924214205720797
    assert.eq
    push.11492798057464085775
    assert.eq
    push.14422448030219940688
    assert.eq

    pushw.local.93

    push.6507250621729077940
    assert.eq
    push.10951745108692586643
    assert.eq
    push.11266079826466912438
    assert.eq
    push.8925771421991641474
    assert.eq

    pushw.local.94

    push.431523066505444268
    assert.eq
    push.1285178546161991355
    assert.eq
    push.8555391423963084189
    assert.eq
    push.17328677503143420133
    assert.eq

    pushw.local.95

    push.13849482253040846080
    assert.eq
    push.13570070408968146198
    assert.eq
    push.14234637378559746361
    assert.eq
    push.10540350791453628906
    assert.eq

    pushw.local.96

    push.650415177035534115
    assert.eq
    push.2449447675370487743
    assert.eq
    push.5530570934007304021
    assert.eq
    push.5008022948409793965
    assert.eq

    pushw.local.97

    push.8565569549360878607
    assert.eq
    push.402139522378392068
    assert.eq
    push.5781054458624323724
    assert.eq
    push.5441321007122731143
    assert.eq

    pushw.local.98

    push.2141708533769732634
    assert.eq
    push.12296696041930375647
    assert.eq
    push.5030333191930369784
    assert.eq
    push.9589717239328382062
    assert.eq

    pushw.local.99

    push.4543611496630253597
    assert.eq
    push.963677192151881803
    assert.eq
    push.1240774410427679033
    assert.eq
    push.6027690232728924930
    assert.eq

    pushw.local.100

    push.2761035047280407935
    assert.eq
    push.9767474113466356503
    assert.eq
    push.1030608063572245572
    assert.eq
    push.2737555203331259767
    assert.eq

    pushw.local.101

    push.8841814831092757511
    assert.eq
    push.7374284328308128887
    assert.eq
    push.8922377290982201691
    assert.eq
    push.1618945402508718138
    assert.eq

    pushw.local.102

    push.11871822660196837602
    assert.eq
    push.3825152053016198315
    assert.eq
    push.3847290460963665198
    assert.eq
    push.12199497154981475450
    assert.eq

    pushw.local.103

    push.6994210944247916666
    assert.eq
    push.8664215757935537682
    assert.eq
    push.17350460119148800673
    assert.eq
    push.12598101527954240506
    assert.eq

    pushw.local.104

    push.2673356055215094872
    assert.eq
    push.12486487283416641615
    assert.eq
    push.2340432030104145622
    assert.eq
    push.16029478146060719718
    assert.eq

    pushw.local.105

    push.16877457782303791547
    assert.eq
    push.11972590391248175313
    assert.eq
    push.3100373081707278564
    assert.eq
    push.7096542296177827649
    assert.eq

    pushw.local.106

    push.7020793370540069683
    assert.eq
    push.14154189039299057850
    assert.eq
    push.16278699566452232814
    assert.eq
    push.15641825374164210288
    assert.eq

    pushw.local.107

    push.16507891939315727263
    assert.eq
    push.2672282024558520400
    assert.eq
    push.11369699668797798029
    assert.eq
    push.12698485145960549833
    assert.eq

    pushw.local.108

    push.7312272130372524998
    assert.eq
    push.618044444486631188
    assert.eq
    push.3791366873874355075
    assert.eq
    push.5082896689942265990
    assert.eq

    pushw.local.109

    push.11707183425196948206
    assert.eq
    push.5473873804701425049
    assert.eq
    push.11040309650571431847
    assert.eq
    push.696756705851448612
    assert.eq

    pushw.local.110

    push.3087123610587377460
    assert.eq
    push.17885312390645592151
    assert.eq
    push.14225547568313734297
    assert.eq
    push.9552704239592624845
    assert.eq

    pushw.local.111

    push.16925721150628365575
    assert.eq
    push.1202539611214124159
    assert.eq
    push.9661188788039071482
    assert.eq
    push.947019814211424257
    assert.eq

    pushw.local.112

    push.18382699622800959606
    assert.eq
    push.2661011257134350943
    assert.eq
    push.16517054008792145701
    assert.eq
    push.11242711842146528718
    assert.eq

    pushw.local.113

    push.8607463334904655371
    assert.eq
    push.5461734778947279056
    assert.eq
    push.9319704235952767853
    assert.eq
    push.1666674032747892084
    assert.eq

    pushw.local.114

    push.5163183654701938158
    assert.eq
    push.11841308919288316363
    assert.eq
    push.14059199343717200324
    assert.eq
    push.8251996508082379793
    assert.eq

    pushw.local.115

    push.14842082905936626972
    assert.eq
    push.12439375321289707536
    assert.eq
    push.11088068707522430673
    assert.eq
    push.4862507816692896766
    assert.eq

    pushw.local.116

    push.3956280272199302478
    assert.eq
    push.8085413517437825684
    assert.eq
    push.18188074785684425088
    assert.eq
    push.4336038796905611039
    assert.eq

    pushw.local.117

    push.7927475631615442421
    assert.eq
    push.11176792569105590956
    assert.eq
    push.14252391726017169882
    assert.eq
    push.9219657197664309210
    assert.eq

    pushw.local.118

    push.792037797933480636
    assert.eq
    push.14336541264070132902
    assert.eq
    push.638525423443462447
    assert.eq
    push.13245404777763005532
    assert.eq

    pushw.local.119

    push.4268190099036611008
    assert.eq
    push.9571109272133006643
    assert.eq
    push.12055429652566909505
    assert.eq
    push.360955568435882338
    assert.eq

    pushw.local.120

    push.10129622376870902946
    assert.eq
    push.9508126923216501423
    assert.eq
    push.5333503729525038918
    assert.eq
    push.6656980442522150754
    assert.eq

    pushw.local.121

    push.14453836366880698356
    assert.eq
    push.730511821011013473
    assert.eq
    push.8954717507605904756
    assert.eq
    push.14946849288599984786
    assert.eq

    pushw.local.122

    push.11497397510247217941
    assert.eq
    push.16540194865268245637
    assert.eq
    push.1012135699625126712
    assert.eq
    push.11499602538048347956
    assert.eq

    pushw.local.123

    push.9069880514720156888
    assert.eq
    push.1704669204159609424
    assert.eq
    push.17802026749540522578
    assert.eq
    push.7733872095211700020
    assert.eq

    pushw.local.124

    push.5686969124851045992
    assert.eq
    push.16258423984894898607
    assert.eq
    push.4286821862379136089
    assert.eq
    push.16405897176262122808
    assert.eq

    pushw.local.125

    push.10152352175912283478
    assert.eq
    push.11992570287022017322
    assert.eq
    push.8003252851648323516
    assert.eq
    push.7213033422352747723
    assert.eq

    pushw.local.126

    push.11886014067542442130
    assert.eq
    push.7617504743224559388
    assert.eq
    push.5942021648748510140
    assert.eq
    push.13914081284823479780
    assert.eq

    pushw.local.127

    push.6251952012393749681
    assert.eq
    push.8615776837219416649
    assert.eq
    push.7272726666596204218
    assert.eq
    push.16992025507167613444
    assert.eq

    # end asserting result
end
"),
// ----- std::math::u256 --------------------------------------------------------------------------
("std::math::u256", "export.add_unsafe
    swapw.3
    movup.3
    movup.7
    u32overflowing_add
    movup.4
    movup.7
    u32overflowing_add3
    movup.4
    movup.6
    u32overflowing_add3
    movup.4
    movup.5
    u32overflowing_add3
    movdn.12
    swapw.2
    movup.12
    movup.4
    movup.8
    u32overflowing_add3
    movup.4
    movup.7
    u32overflowing_add3
    movup.4
    movup.6
    u32overflowing_add3
    movup.4
    movup.5
    u32overflowing_add3
    drop
end

export.sub_unsafe
    swapw.3
    movup.3
    movup.7
    u32overflowing_sub
    movup.7
    u32overflowing_add
    movup.5
    movup.2
    u32overflowing_sub
    movup.2
    add
    movup.6
    u32overflowing_add
    movup.5
    movup.2
    u32overflowing_sub
    movup.2
    add
    movup.5
    u32overflowing_add
    movup.5
    movup.2
    u32overflowing_sub
    movup.2
    add
    movdn.12
    swapw.2
    movup.12
    movup.4
    u32overflowing_add
    movup.8
    movup.2
    u32overflowing_sub
    movup.2
    add
    movup.4
    u32overflowing_add
    movup.7
    movup.2
    u32overflowing_sub
    movup.2
    add
    movup.4
    u32overflowing_add
    movup.6
    movup.2
    u32overflowing_sub
    movup.2
    add
    movup.5
    movup.5
    movup.2
    u32overflowing_add
    drop
    u32overflowing_sub
    drop
end

export.and
    swapw.3
    movup.3
    movup.7
    u32checked_and
    movup.3
    movup.6
    u32checked_and
    movup.3
    movup.5
    u32checked_and
    movup.3
    movup.4
    u32checked_and
    swapw.2
    movup.3
    movup.7
    u32checked_and
    movup.3
    movup.6
    u32checked_and
    movup.3
    movup.5
    u32checked_and
    movup.3
    movup.4
    u32checked_and
end

export.or
    swapw.3
    movup.3
    movup.7
    u32checked_or
    movup.3
    movup.6
    u32checked_or
    movup.3
    movup.5
    u32checked_or
    movup.3
    movup.4
    u32checked_or
    swapw.2
    movup.3
    movup.7
    u32checked_or
    movup.3
    movup.6
    u32checked_or
    movup.3
    movup.5
    u32checked_or
    movup.3
    movup.4
    u32checked_or
end

export.xor
    swapw.3
    movup.3
    movup.7
    u32checked_xor
    movup.3
    movup.6
    u32checked_xor
    movup.3
    movup.5
    u32checked_xor
    movup.3
    movup.4
    u32checked_xor
    swapw.2
    movup.3
    movup.7
    u32checked_xor
    movup.3
    movup.6
    u32checked_xor
    movup.3
    movup.5
    u32checked_xor
    movup.3
    movup.4
    u32checked_xor
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

# ===== MULTIPLICATION ============================================================================

proc.mulstep
    movdn.2
    u32overflowing_madd
    movdn.2
    u32overflowing_add
    movup.2
    add
end

proc.mulstep4
    movup.12
    dup.1
    movup.10
    push.0 # start k at 0
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

# Performs addition of two unsigned 256 bit integers discarding the overflow.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b7, b6, b5, b4, b3, b2, b1, b0, a7, a6, a5, a4, a3, a2, a1, a0, ...] -> [c7, c6, c5, c4, c3, c2, c1, c0, ...]
# where c = (a * b) % 2^256, and a0, b0, and c0 are least significant 32-bit limbs of a, b, and c respectively.
export.mul_unsafe.6
    # Memory storing setup
    popw.local.0
    # b[5-8] at 0
    storew.local.1
    # b[0-4] at 1
    push.0 dropw
    # b[0] at top of stack, followed by a[0-7]
    movdn.8
    storew.local.2
    # a[0-4] at 2
    swapw
    storew.local.3
    # a[5-8] at 3
    padw
    storew.local.4
    storew.local.5
    # p at 4 and 5

    # b[0]
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

    # b[1]
    pushw.local.4
    pushw.local.5
    movup.7
    dropw
    pushw.local.3 pushw.local.2 # load the xs
    pushw.local.1
    movup.2
    movdn.3
    push.0 dropw # only need b[1]

    exec.mulstep4

    movdn.9
    movdn.9
    swapw
    movdn.3
    pushw.local.4
    push.0 dropw # only need p[0]
    movdn.3
    # save p[0-3] to memory, not needed any more
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

    # b[2]
    pushw.local.4
    pushw.local.5
    movup.7
    movup.7
    dropw
    pushw.local.3 pushw.local.2 # load the xs
    pushw.local.1
    swap
    movdn.3
    push.0 dropw # only need b[1]

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

    # b[3]
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

    # b[4]
    pushw.local.3 pushw.local.2 # load the xs
    # OPTIM: don't need a[4-7], but can't use mulstep4 if we don't load

    pushw.local.0
    push.0 dropw # b[4]

    exec.mulstep4
    dropw drop drop # OPTIM: don't need a[4-7], but can't use mulstep4 if we don't load

    # b[5]
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

    # b[6]
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

    # b[7]
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
("std::math::u64", "# ===== HELPER FUNCTIONS ==========================================================================

# Asserts that both values at the top of the stack are u64 values.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
proc.u32assert4
    u32assert
    movup.3
    u32assert
    movup.3
    u32assert
    movup.3
    u32assert
    movup.3
end

# ===== ADDITION ==================================================================================

# Performs addition of two unsigned 64 bit integers preserving the overflow.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [overflowing_flag, c_hi, c_lo, ...], where c = (a + b) % 2^64
export.overflowing_add
    swap
    movup.3
    u32overflowing_add
    movup.3
    movup.3
    u32overflowing_add3
end

# Performs addition of two unsigned 64 bit integers discarding the overflow.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a + b) % 2^64
export.wrapping_add
    exec.overflowing_add
    drop
end

# Performs addition of two unsigned 64 bit integers, fails when overflowing.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a + b) % 2^64
export.checked_add
    exec.u32assert4
    exec.overflowing_add
    eq.0
    assert
end

# ===== SUBTRACTION ===============================================================================

# Performs subtraction of two unsigned 64 bit integers discarding the overflow.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a - b) % 2^64
export.wrapping_sub
    movup.3
    movup.2
    u32overflowing_sub
    movup.3
    movup.3
    u32overflowing_sub
    drop
    swap
    u32overflowing_sub
    drop
end

# Performs subtraction of two unsigned 64 bit integers, fails when underflowing.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a - b) % 2^64
export.checked_sub
    exec.u32assert4
    movup.3
    movup.2
    u32overflowing_sub
    movup.3
    movup.3
    u32overflowing_sub
    eq.0
    assert
    swap
    u32overflowing_sub
    eq.0
    assert
end

# Performs subtraction of two unsigned 64 bit integers preserving the overflow.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [underflowing_flag, c_hi, c_lo, ...], where c = (a - b) % 2^64
export.overflowing_sub
    movup.3
    movup.2
    u32overflowing_sub
    movup.3
    movup.3
    u32overflowing_sub
    swap
    movup.2
    u32overflowing_sub
    movup.2
    or
end

# ===== MULTIPLICATION ============================================================================

# Performs multiplication of two unsigned 64 bit integers discarding the overflow.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a * b) % 2^64
export.wrapping_mul
    dup.3
    dup.2
    u32overflowing_mul
    movup.4
    movup.4
    u32overflowing_madd
    drop
    movup.3
    movup.3
    u32overflowing_madd
    drop
end

# Performs multiplication of two unsigned 64 bit integers preserving the overflow.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_mid_hi, c_mid_lo, c_lo, ...], where c = (a * b) % 2^64
# This takes 18 cycles.
export.overflowing_mul
    dup.3
    dup.2
    u32overflowing_mul
    dup.4
    movup.4
    u32overflowing_madd
    swap
    movup.5
    dup.4
    u32overflowing_madd
    movup.5
    movup.5
    u32overflowing_madd
    movup.3
    movup.2
    u32overflowing_add
    movup.2
    add
end

# Performs multiplication of two unsigned 64 bit integers, fails when overflowing.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a * b) % 2^64
export.checked_mul
    exec.u32assert4
    exec.overflowing_mul
    u32checked_or
    eq.0
    assert
end

# ===== COMPARISONS ===============================================================================

# Performs less-than comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a < b, and 0 otherwise.
export.unchecked_lt
    movup.3
    movup.2
    u32overflowing_sub
    movdn.3
    drop
    u32overflowing_sub
    swap
    eq.0
    movup.2
    and
    or
end

# Performs less-than comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a < b, and 0 otherwise.
export.checked_lt
    exec.u32assert4
    exec.unchecked_lt
end

# Performs greater-than comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a > b, and 0 otherwise.
# This takes 11 cycles.
export.unchecked_gt
    movup.2
    u32overflowing_sub
    movup.2
    movup.3
    u32overflowing_sub
    swap
    drop
    movup.2
    eq.0
    and
    or
end

# Performs greater-than comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a > b, and 0 otherwise.
export.checked_gt
    exec.u32assert4
    exec.unchecked_gt
end

# Performs less-than-or-equal comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a <= b, and 0 otherwise.
export.unchecked_lte
    exec.unchecked_gt
    not
end

# Performs less-than-or-equal comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a <= b, and 0 otherwise.
export.checked_lte
    exec.u32assert4
    exec.unchecked_gt
    not
end

# Performs greater-than-or-equal comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a >= b, and 0 otherwise.
export.unchecked_gte
    exec.unchecked_lt
    not
end

# Performs greater-than-or-equal comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a >= b, and 0 otherwise.
export.checked_gte
    exec.u32assert4
    exec.unchecked_lt
    not
end

# Performs equality comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == b, and 0 otherwise.
export.unchecked_eq
    movup.2
    u32checked_eq
    swap
    movup.2
    u32checked_eq
    and
end

# Performs equality comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == b, and 0 otherwise.
export.checked_eq
    exec.u32assert4
    exec.unchecked_eq
end

# Performs inequality comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a != b, and 0 otherwise.
export.unchecked_neq
    movup.2
    u32checked_neq
    swap
    movup.2
    u32checked_neq
    or
end

# Performs inequality comparison of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == b, and 0 otherwise.
export.checked_neq
    exec.u32assert4
    exec.unchecked_eq
end

# Performs comparison to zero of an unsigned 64 bit integer.
# The input value is assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == 0, and 0 otherwise.
export.unchecked_eqz
    eq.0
    swap
    eq.0
    and
end

# Performs comparison to zero of an unsigned 64 bit integer.
# The input value is assumed to be represented using 32 bit limbs, fails if it is not.
# Stack transition looks as follows:
# [a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == 0, and 0 otherwise.
export.checked_eqz
    u32assert
    swap
    u32assert
    swap
    eq.0
    swap
    eq.0
    and
end

# Compares two unsigned 64 bit integers and drop the larger one from the stack.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a when a < b, and b otherwise.
export.unchecked_min
    dupw
    exec.unchecked_gt
    movup.4
    movup.3
    dup.2
    cdrop
    movdn.3
    cdrop
end

# Compares two unsigned 64 bit integers and drop the larger one from the stack.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a when a < b, and b otherwise.
export.checked_min
    exec.u32assert4
    exec.unchecked_min
end

# Compares two unsigned 64 bit integers and drop the smaller one from the stack.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a when a > b, and b otherwise.
export.unchecked_max
    dupw
    exec.unchecked_lt
    movup.4
    movup.3
    dup.2
    cdrop
    movdn.3
    cdrop
end

# Compares two unsigned 64 bit integers and drop the smaller one from the stack.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a when a > b, and b otherwise.
export.checked_max
    exec.u32assert4
    exec.unchecked_max
end


# ===== DIVISION ==================================================================================

# Performs division of two unsigned 64 bit integers discarding the remainder.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a // b
export.unchecked_div
    adv.u64div          # inject the quotient and the remainder into the advice tape

    push.adv.1          # read the quotient from the advice tape and make sure it consists of
    u32assert           # 32-bit limbs
    push.adv.1          # TODO: this can be optimized once we have u32assert2 instruction
    u32assert

    dup.3               # multiply quotient by the divisor and make sure the resulting value
    dup.2               # fits into 2 32-bit limbs
    u32overflowing_mul
    dup.4
    dup.4
    u32overflowing_madd
    eq.0
    assert
    dup.5
    dup.3
    u32overflowing_madd
    eq.0
    assert
    dup.4
    dup.3
    mul
    eq.0
    assert

    push.adv.1          # read the remainder from the advice tape and make sure it consists of
    u32assert           # 32-bit limbs
    push.adv.1
    u32assert

    movup.7             # make sure the divisor is greater than the remainder. this also consumes
    movup.7             # the divisor
    dup.3
    dup.3
    exec.unchecked_gt
    assert

    swap                # add remainder to the previous result; this also consumes the remainder
    movup.3
    u32overflowing_add
    movup.3
    movup.3
    u32overflowing_add3
    eq.0
    assert

    movup.4             # make sure the result we got is equal to the dividend
    assert.eq
    movup.3
    assert.eq           # quotient remains on the stack
end

# Performs division of two unsigned 64 bit integers discarding the remainder.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a // b
export.checked_div
    exec.u32assert4
    exec.unchecked_div
end

# ===== MODULO OPERATION ==========================================================================

# Performs modulo operation of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a % b
export.unchecked_mod
    adv.u64div          # inject the quotient and the remainder into the advice tape

    push.adv.1          # read the quotient from the advice tape and make sure it consists of
    u32assert           # 32-bit limbs
    push.adv.1          # TODO: this can be optimized once we have u32assert2 instruction
    u32assert

    dup.3               # multiply quotient by the divisor and make sure the resulting value
    dup.2               # fits into 2 32-bit limbs
    u32overflowing_mul
    dup.4
    movup.4
    u32overflowing_madd
    eq.0
    assert
    dup.4
    dup.3
    u32overflowing_madd
    eq.0
    assert
    dup.3
    movup.3
    mul
    eq.0
    assert

    push.adv.1          # read the remainder from the advice tape and make sure it consists of
    u32assert           # 32-bit limbs
    push.adv.1
    u32assert

    movup.5             # make sure the divisor is greater than the remainder. this also consumes
    movup.5             # the divisor
    dup.3
    dup.3
    exec.unchecked_gt
    assert

    dup.1               # add remainder to the previous result
    movup.4
    u32overflowing_add
    movup.4
    dup.3
    u32overflowing_add3
    eq.0
    assert

    movup.4             # make sure the result we got is equal to the dividend
    assert.eq
    movup.3
    assert.eq           # remainder remains on the stack
end

# Performs modulo operation of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a % b
export.checked_mod
    exec.u32assert4
    exec.unchecked_mod
end

# ===== DIVMOD OPERATION ==========================================================================

# Performs divmod operation of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [r_hi, r_lo, q_hi, q_lo ...], where r = a % b, q = a / b
export.unchecked_divmod
    adv.u64div          # inject the quotient and the remainder into the advice tape

    push.adv.1          # read the quotient from the advice tape and make sure it consists of
    u32assert           # 32-bit limbs
    push.adv.1          # TODO: this can be optimized once we have u32assert2 instruction
    u32assert

    dup.3               # multiply quotient by the divisor and make sure the resulting value
    dup.2               # fits into 2 32-bit limbs
    u32overflowing_mul
    dup.4
    dup.4
    u32overflowing_madd
    eq.0
    assert
    dup.5
    dup.3
    u32overflowing_madd
    eq.0
    assert
    dup.4
    dup.3
    mul
    eq.0
    assert

    push.adv.1          # read the remainder from the advice tape and make sure it consists of
    u32assert           # 32-bit limbs 
    push.adv.1
    u32assert

    movup.7             # make sure the divisor is greater than the remainder. this also consumes
    movup.7             # the divisor
    dup.3
    dup.3
    exec.unchecked_gt
    assert

    dup.1               # add remainder to the previous result
    movup.4
    u32overflowing_add
    movup.4
    dup.3
    u32overflowing_add3
    eq.0
    assert

    movup.6             # make sure the result we got is equal to the dividend
    assert.eq
    movup.5
    assert.eq           # remainder remains on the stack
end

# Performs divmod operation of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [r_hi, r_lo, q_hi, q_lo ...], where r = a % b, q = a / b
export.checked_divmod
    exec.u32assert4
    exec.unchecked_divmod
end

# ===== BITWISE OPERATIONS ========================================================================

# Performs bitwise AND of two unsigned 64-bit integers.
# The input values are assumed to be represented using 32 bit limbs, but this is not checked.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a AND b.
export.checked_and
    swap
    movup.3
    u32checked_and
    swap
    movup.2
    u32checked_and
end

# Performs bitwise OR of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a OR b.
export.checked_or
    swap
    movup.3
    u32checked_or
    swap
    movup.2
    u32checked_or
end

# Performs bitwise XOR of two unsigned 64 bit integers.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
# Stack transition looks as follows:
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a XOR b.
export.checked_xor
    swap
    movup.3
    u32checked_xor
    swap
    movup.2
    u32checked_xor
end

# Performs left shift of one unsigned 64-bit integer using the pow2 operation.
# The input value to be shifted is assumed to be represented using 32 bit limbs.
# The shift value is assumed to be in the range [0, 64).
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a << b mod 2^64.
# This takes 50 cycles.
export.unchecked_shl
    unchecked_pow2
    u32split
    exec.wrapping_mul
end


# Performs right shift of one unsigned 64-bit integer using the pow2 operation.
# The input value to be shifted is assumed to be represented using 32 bit limbs.
# The shift value is assumed to be in the range [0, 64).
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a >> b.
# This takes 66 cycles.
export.unchecked_shr
    unchecked_pow2
    u32split

    dup.1
    add
    movup.2
    swap
    u32unchecked_divmod
    movup.3
    movup.3
    dup
    eq.0
    u32overflowing_sub
    not
    movdn.4
    dup
    movdn.4
    u32unchecked_divmod
    drop
    push.4294967296
    dup.5
    mul
    movup.4
    div
    movup.2
    mul
    add
    movup.2
    cswap
end

# Performs left shift of one unsigned 64-bit integer preserving the overflow and
# using the pow2 operation.
# The input value to be shifted is assumed to be represented using 32 bit limbs.
# The shift value is assumed to be in the range [0, 64).
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [d_hi, d_lo, c_hi, c_lo, ...], where (d,c) = a << b, 
# which d contains the bits shifted out.
# This takes 57 cycles.
export.overflowing_shl
    unchecked_pow2
    u32split
    exec.overflowing_mul
end

# Performs right shift of one unsigned 64-bit integer preserving the overflow and
# using the pow2 operation.
# The input value to be shifted is assumed to be represented using 32 bit limbs.
# The shift value is assumed to be in the range [0, 64).
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [d_hi, d_lo, c_hi, c_lo, ...], where c = a >> b, d = a << (64 - b).
# This takes 138 cycles.
export.overflowing_shr
    push.64             # (64 - b)
    dup.1
    sub

    dup.3               # dup [b, a_hi, a_lo]
    dup.3
    dup.3
    exec.unchecked_shr  # c = a >> b

    movdn.5             # move result [c_hi, c_lo] to be in the format [d_hi, d_lo, c_hi, c_lo, ...]
    movdn.5

    padw                # padding positions 0, 1, 2, 3 and 4 to be able to use cdropw
    push.0

    movup.6             # bring and b
    eq.0
    cdropw              # if b is 0, swap the positions 0, 1, 2 and 3 with 0, (64 - b), a_hi, a_lo
                        # regardless of this condition, drop 0, 1, 2 and 3
    drop                # drop the last added 0 or dup b to keep the format [b, a_hi, a_lo, ....]

    exec.unchecked_shl  # d = a << (64 - b)
end

# Performs left rotation of one unsigned 64-bit integer using the pow2 operation.
# The input value to be shifted is assumed to be represented using 32 bit limbs.
# The shift value is assumed to be in the range [0, 64).
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a << b mod 2^64.
# This takes 57 cycles.
export.unchecked_rotl
    push.31
    dup.1
    u32overflowing_sub
    swap
    drop
    movdn.3

    # Shift the low limb.
    push.31
    u32checked_and
    unchecked_pow2
    dup
    movup.3
    u32overflowing_mul

    # Shift the high limb.
    movup.3
    movup.3
    u32overflowing_madd

    # Carry the overflow shift to the low bits.
    movup.2
    add
    swap

    # Conditionally select the limb order based on whether it's shifting by > 31 or not.
    movup.2
    cswap
end

# Performs right rotation of one unsigned 64-bit integer using the pow2 operation.
# The input value to be shifted is assumed to be represented using 32 bit limbs.
# The shift value is assumed to be in the range [0, 64).
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a << b mod 2^64.
# This takes 62 cycles.
export.unchecked_rotr
    push.31
    dup.1
    u32overflowing_sub
    swap
    drop
    movdn.3

    # Shift the low limb left by 32-b.
    push.31
    u32checked_and
    push.32
    swap
    u32overflowing_sub
    drop
    unchecked_pow2
    dup
    movup.3
    u32overflowing_mul

    # Shift the high limb left by 32-b.
    movup.3
    movup.3
    u32overflowing_madd

    # Carry the overflow shift to the low bits.
    movup.2
    add
    swap

    # Conditionally select the limb order based on whether it's shifting by > 31 or not.
    movup.2
    not
    cswap
end
"),
// ----- std::sys ---------------------------------------------------------------------------------
("std::sys", "# Removes elements deep in the stack until the depth of the stack is exactly 16. The elements
# are removed in such a way that the top 16 elements of the stack remain unchanged.
# Input: Stack with 16 or more elements.
# Output: Stack with only the original top 16 elements.
export.finalize_stack.4
    popw.local.0
    popw.local.1
    popw.local.2
    popw.local.3
    push.env.sdepth
    neq.16
    while.true
        dropw
        push.env.sdepth
        neq.16
    end
    loadw.local.3
    swapw.3
    loadw.local.2
    swapw.2
    loadw.local.1
    swapw.1
    loadw.local.0
end
"),
];
