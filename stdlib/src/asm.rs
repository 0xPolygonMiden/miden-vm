//! This module is automatically generated during build time and should not be modified manually.

/// An array of modules defined in Miden standard library.
///
/// Entries in the array are tuples containing module namespace and module source code.
#[rustfmt::skip]
pub const MODULES: [(&str, &str); 8] = [
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

	storew.local.64
    swapw
	storew.local.0

	loadw.local.1
    swapw
	loadw.local.65
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.65
    swapw
	storew.local.1

	loadw.local.2
	swapw
    loadw.local.66
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.66
    swapw
    storew.local.2

	loadw.local.3
	swapw
    loadw.local.67
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.67
	swapw
	storew.local.3

	loadw.local.4
	swapw
	loadw.local.68
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.68
	swapw
	storew.local.4

	loadw.local.5
	swapw
	loadw.local.69
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.69
	swapw
	storew.local.5

	loadw.local.6
	swapw
	loadw.local.70
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.70
	swapw
	storew.local.6

	loadw.local.7
	swapw
	loadw.local.71
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.71
	swapw
	storew.local.7

	loadw.local.8
	swapw
	loadw.local.72
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.72
	swapw
	storew.local.8

	loadw.local.9
	swapw
	loadw.local.73
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.73
	swapw
	storew.local.9

	loadw.local.10
	swapw
	loadw.local.74
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.74
	swapw
	storew.local.10

	loadw.local.11
	swapw
	loadw.local.75
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.75
	swapw
	storew.local.11

	loadw.local.12
	swapw
	loadw.local.76
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.76
	swapw
	storew.local.12

	loadw.local.13
	swapw
	loadw.local.77
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.77
	swapw
	storew.local.13

	loadw.local.14
	swapw
	loadw.local.78
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.78
	swapw
	storew.local.14

	loadw.local.15
	swapw
	loadw.local.79
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.79
	swapw
	storew.local.15

	loadw.local.16
	swapw
	loadw.local.80
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.80
	swapw
	storew.local.16

	loadw.local.17
	swapw
	loadw.local.81
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.81
	swapw
	storew.local.17

	loadw.local.18
	swapw
	loadw.local.82
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.82
	swapw
	storew.local.18

	loadw.local.19
	swapw
	loadw.local.83
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.83
	swapw
	storew.local.19

	loadw.local.20
	swapw
	loadw.local.84
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.84
	swapw
	storew.local.20

	loadw.local.21
	swapw
	loadw.local.85
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.85
	swapw
	storew.local.21

	loadw.local.22
	swapw
	loadw.local.86
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.86
	swapw
	storew.local.22

	loadw.local.23
	swapw
	loadw.local.87
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.87
	swapw
	storew.local.23

	loadw.local.24
	swapw
	loadw.local.88
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.88
	swapw
	storew.local.24

	loadw.local.25
	swapw
	loadw.local.89
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.89
	swapw
	storew.local.25

	loadw.local.26
	swapw
	loadw.local.90
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.90
	swapw
	storew.local.26

	loadw.local.27
	swapw
	loadw.local.91
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.91
	swapw
	storew.local.27

	loadw.local.28
	swapw
	loadw.local.92
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.92
	swapw
	storew.local.28

	loadw.local.29
	swapw
	loadw.local.93
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.93
	swapw
	storew.local.29

	loadw.local.30
	swapw
	loadw.local.94
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.94
	swapw
	storew.local.30

	loadw.local.31
	swapw
	loadw.local.95
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.95
	swapw
	storew.local.31

	loadw.local.32
	swapw
	loadw.local.96
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.96
	swapw
	storew.local.32

	loadw.local.33
	swapw
	loadw.local.97
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.97
	swapw
	storew.local.33

	loadw.local.34
	swapw
	loadw.local.98
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.98
	swapw
	storew.local.34

	loadw.local.35
	swapw
	loadw.local.99
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.99
	swapw
	storew.local.35

	loadw.local.36
	swapw
	loadw.local.100
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.100
	swapw
	storew.local.36

	loadw.local.37
	swapw
	loadw.local.101
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.101
	swapw
	storew.local.37

	loadw.local.38
	swapw
	loadw.local.102
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.102
	swapw
	storew.local.38

	loadw.local.39
	swapw
	loadw.local.103
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.103
	swapw
	storew.local.39

	loadw.local.40
	swapw
	loadw.local.104
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.104
	swapw
	storew.local.40

	loadw.local.41
	swapw
	loadw.local.105
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.105
	swapw
	storew.local.41

	loadw.local.42
	swapw
	loadw.local.106
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.106
	swapw
	storew.local.42

	loadw.local.43
	swapw
	loadw.local.107
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.107
	swapw
	storew.local.43

	loadw.local.44
	swapw
	loadw.local.108
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.108
	swapw
	storew.local.44

	loadw.local.45
	swapw
	loadw.local.109
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.109
	swapw
	storew.local.45

	loadw.local.46
	swapw
	loadw.local.110
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.110
	swapw
	storew.local.46

	loadw.local.47
	swapw
	loadw.local.111
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.111
	swapw
	storew.local.47

	loadw.local.48
	swapw
	loadw.local.112
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.112
	swapw
	storew.local.48

	loadw.local.49
	swapw
	loadw.local.113
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.113
	swapw
	storew.local.49

	loadw.local.50
	swapw
	loadw.local.114
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.114
	swapw
	storew.local.50

	loadw.local.51
	swapw
	loadw.local.115
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.115
	swapw
	storew.local.51

	loadw.local.52
	swapw
	loadw.local.116
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.116
	swapw
	storew.local.52

	loadw.local.53
	swapw
	loadw.local.117
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.117
	swapw
	storew.local.53

	loadw.local.54
	swapw
	loadw.local.118
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.118
	swapw
	storew.local.54

	loadw.local.55
	swapw
	loadw.local.119
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.119
	swapw
	storew.local.55

	loadw.local.56
	swapw
	loadw.local.120
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.120
	swapw
	storew.local.56

	loadw.local.57
	swapw
	loadw.local.121
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.121
	swapw
	storew.local.57

	loadw.local.58
	swapw
	loadw.local.122
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.122
	swapw
	storew.local.58

	loadw.local.59
	swapw
	loadw.local.123
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.123
	swapw
	storew.local.59

	loadw.local.60
	swapw
	loadw.local.124
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.124
	swapw
	storew.local.60

	loadw.local.61
	swapw
	loadw.local.125
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.125
	swapw
	storew.local.61

	loadw.local.62
	swapw
	loadw.local.126
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.126
	swapw
	storew.local.62

	loadw.local.63
	swapw
	loadw.local.127
	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665

	exec.butterfly

	storew.local.127
	swapw
	storew.local.63

    # iter = 1

	loadw.local.0
	swapw
	loadw.local.32
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.32
	swapw
	storew.local.0

	loadw.local.1
	swapw
	loadw.local.33
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.33
	swapw
	storew.local.1

	loadw.local.2
	swapw
	loadw.local.34
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.34
	swapw
	storew.local.2

	loadw.local.3
	swapw
	loadw.local.35
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.35
	swapw
	storew.local.3

	loadw.local.4
	swapw
	loadw.local.36
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.36
	swapw
	storew.local.4

	loadw.local.5
	swapw
	loadw.local.37
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.37
	swapw
	storew.local.5

	loadw.local.6
	swapw
	loadw.local.38
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.38
	swapw
	storew.local.6

	loadw.local.7
	swapw
	loadw.local.39
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.39
	swapw
	storew.local.7

	loadw.local.8
	swapw
	loadw.local.40
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.40
	swapw
	storew.local.8

	loadw.local.9
	swapw
	loadw.local.41
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.41
	swapw
	storew.local.9

	loadw.local.10
	swapw
	loadw.local.42
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.42
	swapw
	storew.local.10

	loadw.local.11
	swapw
	loadw.local.43
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.43
	swapw
	storew.local.11

	loadw.local.12
	swapw
	loadw.local.44
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.44
	swapw
	storew.local.12

	loadw.local.13
	swapw
	loadw.local.45
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.45
	swapw
	storew.local.13

	loadw.local.14
	swapw
	loadw.local.46
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.46
	swapw
	storew.local.14

	loadw.local.15
	swapw
	loadw.local.47
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.47
	swapw
	storew.local.15

	loadw.local.16
	swapw
	loadw.local.48
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.48
	swapw
	storew.local.16

	loadw.local.17
	swapw
	loadw.local.49
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.49
	swapw
	storew.local.17

	loadw.local.18
	swapw
	loadw.local.50
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.50
	swapw
	storew.local.18

	loadw.local.19
	swapw
	loadw.local.51
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.51
	swapw
	storew.local.19

	loadw.local.20
	swapw
	loadw.local.52
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.52
	swapw
	storew.local.20

	loadw.local.21
	swapw
	loadw.local.53
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.53
	swapw
	storew.local.21

	loadw.local.22
	swapw
	loadw.local.54
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.54
	swapw
	storew.local.22

	loadw.local.23
	swapw
	loadw.local.55
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.55
	swapw
	storew.local.23

	loadw.local.24
	swapw
	loadw.local.56
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.56
	swapw
	storew.local.24

	loadw.local.25
	swapw
	loadw.local.57
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.57
	swapw
	storew.local.25

	loadw.local.26
	swapw
	loadw.local.58
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.58
	swapw
	storew.local.26

	loadw.local.27
	swapw
	loadw.local.59
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.59
	swapw
	storew.local.27

	loadw.local.28
	swapw
	loadw.local.60
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.60
	swapw
	storew.local.28

	loadw.local.29
	swapw
	loadw.local.61
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.61
	swapw
	storew.local.29

	loadw.local.30
	swapw
	loadw.local.62
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.62
	swapw
	storew.local.30

	loadw.local.31
	swapw
	loadw.local.63
	push.1099511627520.1099511627520.1099511627520.1099511627520

	exec.butterfly

	storew.local.63
	swapw
	storew.local.31

    # ---

	loadw.local.64
	swapw
	loadw.local.96
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.96
	swapw
	storew.local.64

	loadw.local.65
	swapw
	loadw.local.97
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.97
	swapw
	storew.local.65

	loadw.local.66
	swapw
	loadw.local.98
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.98
	swapw
	storew.local.66

	loadw.local.67
	swapw
	loadw.local.99
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.99
	swapw
	storew.local.67

	loadw.local.68
	swapw
	loadw.local.100
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.100
	swapw
	storew.local.68

	loadw.local.69
	swapw
	loadw.local.101
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.101
	swapw
	storew.local.69

	loadw.local.70
	swapw
	loadw.local.102
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.102
	swapw
	storew.local.70

	loadw.local.71
	swapw
	loadw.local.103
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.103
	swapw
	storew.local.71

	loadw.local.72
	swapw
	loadw.local.104
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.104
	swapw
	storew.local.72

	loadw.local.73
	swapw
	loadw.local.105
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.105
	swapw
	storew.local.73

	loadw.local.74
	swapw
	loadw.local.106
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.106
	swapw
	storew.local.74

	loadw.local.75
	swapw
	loadw.local.107
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.107
	swapw
	storew.local.75

	loadw.local.76
	swapw
	loadw.local.108
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.108
	swapw
	storew.local.76

	loadw.local.77
	swapw
	loadw.local.109
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.109
	swapw
	storew.local.77

	loadw.local.78
	swapw
	loadw.local.110
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.110
	swapw
	storew.local.78

	loadw.local.79
	swapw
	loadw.local.111
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.111
	swapw
	storew.local.79

	loadw.local.80
	swapw
	loadw.local.112
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.112
	swapw
	storew.local.80

	loadw.local.81
	swapw
	loadw.local.113
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.113
	swapw
	storew.local.81

	loadw.local.82
	swapw
	loadw.local.114
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.114
	swapw
	storew.local.82

	loadw.local.83
	swapw
	loadw.local.115
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.115
	swapw
	storew.local.83

	loadw.local.84
	swapw
	loadw.local.116
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.116
	swapw
	storew.local.84

	loadw.local.85
	swapw
	loadw.local.117
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.117
	swapw
	storew.local.85

	loadw.local.86
	swapw
	loadw.local.118
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.118
	swapw
	storew.local.86

	loadw.local.87
	swapw
	loadw.local.119
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.119
	swapw
	storew.local.87

	loadw.local.88
	swapw
	loadw.local.120
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.120
	swapw
	storew.local.88

	loadw.local.89
	swapw
	loadw.local.121
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.121
	swapw
	storew.local.89

	loadw.local.90
	swapw
	loadw.local.122
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.122
	swapw
	storew.local.90

	loadw.local.91
	swapw
	loadw.local.123
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.123
	swapw
	storew.local.91

	loadw.local.92
	swapw
	loadw.local.124
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.124
	swapw
	storew.local.92

	loadw.local.93
	swapw
	loadw.local.125
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.125
	swapw
	storew.local.93

	loadw.local.94
	swapw
	loadw.local.126
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.126
	swapw
	storew.local.94

	loadw.local.95
	swapw
	loadw.local.127
	push.16777216.16777216.16777216.16777216

	exec.butterfly

	storew.local.127
	swapw
	storew.local.95

    # iter = 2

	loadw.local.0
	swapw
	loadw.local.16
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.16
	swapw
	storew.local.0

	loadw.local.1
	swapw
	loadw.local.17
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.17
	swapw
	storew.local.1

	loadw.local.2
	swapw
	loadw.local.18
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.18
	swapw
	storew.local.2

	loadw.local.3
	swapw
	loadw.local.19
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.19
	swapw
	storew.local.3

	loadw.local.4
	swapw
	loadw.local.20
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.20
	swapw
	storew.local.4

	loadw.local.5
	swapw
	loadw.local.21
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.21
	swapw
	storew.local.5

	loadw.local.6
	swapw
	loadw.local.22
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.22
	swapw
	storew.local.6

	loadw.local.7
	swapw
	loadw.local.23
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.23
	swapw
	storew.local.7

	loadw.local.8
	swapw
	loadw.local.24
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.24
	swapw
	storew.local.8

	loadw.local.9
	swapw
	loadw.local.25
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.25
	swapw
	storew.local.9

	loadw.local.10
	swapw
	loadw.local.26
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.26
	swapw
	storew.local.10

	loadw.local.11
	swapw
	loadw.local.27
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.27
	swapw
	storew.local.11

	loadw.local.12
	swapw
	loadw.local.28
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.28
	swapw
	storew.local.12

	loadw.local.13
	swapw
	loadw.local.29
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.29
	swapw
	storew.local.13

	loadw.local.14
	swapw
	loadw.local.30
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.30
	swapw
	storew.local.14

	loadw.local.15
	swapw
	loadw.local.31
	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585

	exec.butterfly

	storew.local.31
	swapw
	storew.local.15

    # ---

	loadw.local.32
	swapw
	loadw.local.48
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.48
	swapw
	storew.local.32

	loadw.local.33
	swapw
	loadw.local.49
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.49
	swapw
	storew.local.33

	loadw.local.34
	swapw
	loadw.local.50
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.50
	swapw
	storew.local.34

	loadw.local.35
	swapw
	loadw.local.51
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.51
	swapw
	storew.local.35

	loadw.local.36
	swapw
	loadw.local.52
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.52
	swapw
	storew.local.36

	loadw.local.37
	swapw
	loadw.local.53
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.53
	swapw
	storew.local.37

	loadw.local.38
	swapw
	loadw.local.54
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.54
	swapw
	storew.local.38

	loadw.local.39
	swapw
	loadw.local.55
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.55
	swapw
	storew.local.39

	loadw.local.40
	swapw
	loadw.local.56
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.56
	swapw
	storew.local.40

	loadw.local.41
	swapw
	loadw.local.57
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.57
	swapw
	storew.local.41

	loadw.local.42
	swapw
	loadw.local.58
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.58
	swapw
	storew.local.42

	loadw.local.43
	swapw
	loadw.local.59
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.59
	swapw
	storew.local.43

	loadw.local.44
	swapw
	loadw.local.60
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.60
	swapw
	storew.local.44

	loadw.local.45
	swapw
	loadw.local.61
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.61
	swapw
	storew.local.45

	loadw.local.46
	swapw
	loadw.local.62
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.62
	swapw
	storew.local.46

	loadw.local.47
	swapw
	loadw.local.63
	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920

	exec.butterfly

	storew.local.63
	swapw
	storew.local.47

    # ---

	loadw.local.64
	swapw
	loadw.local.80
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.80
	swapw
	storew.local.64

	loadw.local.65
	swapw
	loadw.local.81
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.81
	swapw
	storew.local.65

	loadw.local.66
	swapw
	loadw.local.82
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.82
	swapw
	storew.local.66

	loadw.local.67
	swapw
	loadw.local.83
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.83
	swapw
	storew.local.67

	loadw.local.68
	swapw
	loadw.local.84
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.84
	swapw
	storew.local.68

	loadw.local.69
	swapw
	loadw.local.85
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.85
	swapw
	storew.local.69

	loadw.local.70
	swapw
	loadw.local.86
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.86
	swapw
	storew.local.70

	loadw.local.71
	swapw
	loadw.local.87
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.87
	swapw
	storew.local.71

	loadw.local.72
	swapw
	loadw.local.88
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.88
	swapw
	storew.local.72

	loadw.local.73
	swapw
	loadw.local.89
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.89
	swapw
	storew.local.73

	loadw.local.74
	swapw
	loadw.local.90
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.90
	swapw
	storew.local.74

	loadw.local.75
	swapw
	loadw.local.91
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.91
	swapw
	storew.local.75

	loadw.local.76
	swapw
	loadw.local.92
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.92
	swapw
	storew.local.76

	loadw.local.77
	swapw
	loadw.local.93
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.93
	swapw
	storew.local.77

	loadw.local.78
	swapw
	loadw.local.94
	push.4096.4096.4096.4096

	exec.butterfly

	storew.local.94
	swapw
	storew.local.78

	loadw.local.79
	swapw
	loadw.local.95
	push.4096.4096.4096.4096

	exec.butterfly

    storew.local.95
	swapw
	storew.local.79

    # ---

	loadw.local.96
	swapw
	loadw.local.112
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.112
	swapw
	storew.local.96

	loadw.local.97
	swapw
	loadw.local.113
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.113
	swapw
	storew.local.97

	loadw.local.98
	swapw
	loadw.local.114
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.114
	swapw
	storew.local.98

	loadw.local.99
	swapw
	loadw.local.115
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.115
	swapw
	storew.local.99

	loadw.local.100
	swapw
	loadw.local.116
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.116
	swapw
	storew.local.100

	loadw.local.101
	swapw
	loadw.local.117
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.117
	swapw
	storew.local.101

	loadw.local.102
	swapw
	loadw.local.118
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.118
	swapw
	storew.local.102

	loadw.local.103
	swapw
	loadw.local.119
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.119
	swapw
	storew.local.103

	loadw.local.104
	swapw
	loadw.local.120
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.120
	swapw
	storew.local.104

	loadw.local.105
	swapw
	loadw.local.121
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.121
	swapw
	storew.local.105

	loadw.local.106
	swapw
	loadw.local.122
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.122
	swapw
	storew.local.106

	loadw.local.107
	swapw
	loadw.local.123
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.123
	swapw
	storew.local.107

	loadw.local.108
	swapw
	loadw.local.124
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.124
	swapw
	storew.local.108

	loadw.local.109
	swapw
	loadw.local.125
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.125
	swapw
	storew.local.109

	loadw.local.110
	swapw
	loadw.local.126
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.126
	swapw
	storew.local.110

	loadw.local.111
	swapw
	loadw.local.127
	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345

	exec.butterfly

	storew.local.127
	swapw
	storew.local.111

    # iter = 3

	loadw.local.0
	swapw
	loadw.local.8
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	storew.local.8
	swapw
	storew.local.0

	loadw.local.1
	swapw
	loadw.local.9
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	storew.local.9
	swapw
	storew.local.1

	loadw.local.2
	swapw
	loadw.local.10
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	storew.local.10
	swapw
	storew.local.2

	loadw.local.3
	swapw
	loadw.local.11
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	storew.local.11
	swapw
	storew.local.3

	loadw.local.4
	swapw
	loadw.local.12
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	storew.local.12
	swapw
	storew.local.4

	loadw.local.5
	swapw
	loadw.local.13
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	storew.local.13
	swapw
	storew.local.5

	loadw.local.6
	swapw
	loadw.local.14
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	storew.local.14
	swapw
	storew.local.6

	loadw.local.7
	swapw
	loadw.local.15
	push.17179869180.17179869180.17179869180.17179869180

	exec.butterfly

	storew.local.15
	swapw
	storew.local.7

    # ---

    loadw.local.16
	swapw
	loadw.local.24
	push.262144.262144.262144.262144

	exec.butterfly

	storew.local.24
	swapw
	storew.local.16

	loadw.local.17
	swapw
	loadw.local.25
	push.262144.262144.262144.262144

	exec.butterfly

	storew.local.25
	swapw
	storew.local.17

	loadw.local.18
	swapw
	loadw.local.26
	push.262144.262144.262144.262144

	exec.butterfly

	storew.local.26
	swapw
	storew.local.18

	loadw.local.19
	swapw
	loadw.local.27
	push.262144.262144.262144.262144

	exec.butterfly

	storew.local.27
	swapw
	storew.local.19

	loadw.local.20
	swapw
	loadw.local.28
	push.262144.262144.262144.262144

	exec.butterfly

	storew.local.28
	swapw
	storew.local.20

	loadw.local.21
	swapw
	loadw.local.29
	push.262144.262144.262144.262144

	exec.butterfly

	storew.local.29
	swapw
	storew.local.21

	loadw.local.22
	swapw
	loadw.local.30
	push.262144.262144.262144.262144

	exec.butterfly

	storew.local.30
	swapw
	storew.local.22

	loadw.local.23
	swapw
	loadw.local.31
	push.262144.262144.262144.262144

	exec.butterfly

	storew.local.31
	swapw
	storew.local.23

    # ---

    loadw.local.32
	swapw
	loadw.local.40
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	storew.local.40
	swapw
	storew.local.32

	loadw.local.33
	swapw
	loadw.local.41
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	storew.local.41
	swapw
	storew.local.33

	loadw.local.34
	swapw
	loadw.local.42
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	storew.local.42
	swapw
	storew.local.34

	loadw.local.35
	swapw
	loadw.local.43
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	storew.local.43
	swapw
	storew.local.35

	loadw.local.36
	swapw
	loadw.local.44
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	storew.local.44
	swapw
	storew.local.36

	loadw.local.37
	swapw
	loadw.local.45
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	storew.local.45
	swapw
	storew.local.37

	loadw.local.38
	swapw
	loadw.local.46
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	storew.local.46
	swapw
	storew.local.38

	loadw.local.39
	swapw
	loadw.local.47
	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217

	exec.butterfly

	storew.local.47
	swapw
	storew.local.39

    # ---

    loadw.local.48
	swapw
	loadw.local.56
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	storew.local.56
	swapw
	storew.local.48

	loadw.local.49
	swapw
	loadw.local.57
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	storew.local.57
	swapw
	storew.local.49

	loadw.local.50
	swapw
	loadw.local.58
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	storew.local.58
	swapw
	storew.local.50

	loadw.local.51
	swapw
	loadw.local.59
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	storew.local.59
	swapw
	storew.local.51

	loadw.local.52
	swapw
	loadw.local.60
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	storew.local.60
	swapw
	storew.local.52

	loadw.local.53
	swapw
	loadw.local.61
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	storew.local.61
	swapw
	storew.local.53

	loadw.local.54
	swapw
	loadw.local.62
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	storew.local.62
	swapw
	storew.local.54

	loadw.local.55
	swapw
	loadw.local.63
	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880

	exec.butterfly

	storew.local.63
	swapw
	storew.local.55

    # ---

    loadw.local.64
	swapw
	loadw.local.72
	push.64.64.64.64

	exec.butterfly

	storew.local.72
	swapw
	storew.local.64

	loadw.local.65
	swapw
	loadw.local.73
	push.64.64.64.64

	exec.butterfly

	storew.local.73
	swapw
	storew.local.65

	loadw.local.66
	swapw
	loadw.local.74
	push.64.64.64.64

	exec.butterfly

	storew.local.74
	swapw
	storew.local.66

	loadw.local.67
	swapw
	loadw.local.75
	push.64.64.64.64

	exec.butterfly

	storew.local.75
	swapw
	storew.local.67

	loadw.local.68
	swapw
	loadw.local.76
	push.64.64.64.64

	exec.butterfly

	storew.local.76
	swapw
	storew.local.68

	loadw.local.69
	swapw
	loadw.local.77
	push.64.64.64.64

	exec.butterfly

	storew.local.77
	swapw
	storew.local.69

	loadw.local.70
	swapw
	loadw.local.78
	push.64.64.64.64

	exec.butterfly

	storew.local.78
	swapw
	storew.local.70

	loadw.local.71
	swapw
	loadw.local.79
	push.64.64.64.64

	exec.butterfly

	storew.local.79
	swapw
	storew.local.71

    # ---

    loadw.local.80
	swapw
	loadw.local.88
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	storew.local.88
	swapw
	storew.local.80

	loadw.local.81
	swapw
	loadw.local.89
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	storew.local.89
	swapw
	storew.local.81

	loadw.local.82
	swapw
	loadw.local.90
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	storew.local.90
	swapw
	storew.local.82

	loadw.local.83
	swapw
	loadw.local.91
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	storew.local.91
	swapw
	storew.local.83

	loadw.local.84
	swapw
	loadw.local.92
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	storew.local.92
	swapw
	storew.local.84

	loadw.local.85
	swapw
	loadw.local.93
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	storew.local.93
	swapw
	storew.local.85

	loadw.local.86
	swapw
	loadw.local.94
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	storew.local.94
	swapw
	storew.local.86

	loadw.local.87
	swapw
	loadw.local.95
	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337

	exec.butterfly

	storew.local.95
	swapw
	storew.local.87

    # ---

    loadw.local.96
	swapw
	loadw.local.104
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	storew.local.104
	swapw
	storew.local.96

	loadw.local.97
	swapw
	loadw.local.105
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	storew.local.105
	swapw
	storew.local.97

	loadw.local.98
	swapw
	loadw.local.106
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	storew.local.106
	swapw
	storew.local.98

	loadw.local.99
	swapw
	loadw.local.107
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	storew.local.107
	swapw
	storew.local.99

	loadw.local.100
	swapw
	loadw.local.108
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	storew.local.108
	swapw
	storew.local.100

	loadw.local.101
	swapw
	loadw.local.109
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	storew.local.109
	swapw
	storew.local.101

	loadw.local.102
	swapw
	loadw.local.110
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	storew.local.110
	swapw
	storew.local.102

	loadw.local.103
	swapw
	loadw.local.111
	push.70368744161280.70368744161280.70368744161280.70368744161280

	exec.butterfly

	storew.local.111
	swapw
	storew.local.103

    # ---

    loadw.local.112
	swapw
	loadw.local.120
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	storew.local.120
	swapw
	storew.local.112

	loadw.local.113
	swapw
	loadw.local.121
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	storew.local.121
	swapw
	storew.local.113

	loadw.local.114
	swapw
	loadw.local.122
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	storew.local.122
	swapw
	storew.local.114

	loadw.local.115
	swapw
	loadw.local.123
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	storew.local.123
	swapw
	storew.local.115

	loadw.local.116
	swapw
	loadw.local.124
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	storew.local.124
	swapw
	storew.local.116

	loadw.local.117
	swapw
	loadw.local.125
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	storew.local.125
	swapw
	storew.local.117

	loadw.local.118
	swapw
	loadw.local.126
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	storew.local.126
	swapw
	storew.local.118

	loadw.local.119
	swapw
	loadw.local.127
	push.1073741824.1073741824.1073741824.1073741824

	exec.butterfly

	storew.local.127
	swapw
	storew.local.119

    # iter = 4

    loadw.local.0
	swapw
	loadw.local.4
	push.18446744060824649729.18446744060824649729.18446744060824649729.18446744060824649729

	exec.butterfly

	storew.local.4
	swapw
	storew.local.0

	loadw.local.1
	swapw
	loadw.local.5
	push.18446744060824649729.18446744060824649729.18446744060824649729.18446744060824649729

	exec.butterfly

	storew.local.5
	swapw
	storew.local.1

	loadw.local.2
	swapw
	loadw.local.6
	push.18446744060824649729.18446744060824649729.18446744060824649729.18446744060824649729

	exec.butterfly

	storew.local.6
	swapw
	storew.local.2

	loadw.local.3
	swapw
	loadw.local.7
	push.18446744060824649729.18446744060824649729.18446744060824649729.18446744060824649729

	exec.butterfly

	storew.local.7
	swapw
	storew.local.3

    # ---

    loadw.local.8
	swapw
	loadw.local.12
	push.562949953290240.562949953290240.562949953290240.562949953290240

	exec.butterfly

	storew.local.12
	swapw
	storew.local.8

	loadw.local.9
	swapw
	loadw.local.13
	push.562949953290240.562949953290240.562949953290240.562949953290240

	exec.butterfly

	storew.local.13
	swapw
	storew.local.9

	loadw.local.10
	swapw
	loadw.local.14
	push.562949953290240.562949953290240.562949953290240.562949953290240

	exec.butterfly

	storew.local.14
	swapw
	storew.local.10

	loadw.local.11
	swapw
	loadw.local.15
	push.562949953290240.562949953290240.562949953290240.562949953290240

	exec.butterfly

	storew.local.15
	swapw
	storew.local.11

    # ---

    loadw.local.16
	swapw
	loadw.local.20
	push.512.512.512.512

	exec.butterfly

	storew.local.20
	swapw
	storew.local.16

	loadw.local.17
	swapw
	loadw.local.21
	push.512.512.512.512

	exec.butterfly

	storew.local.21
	swapw
	storew.local.17

	loadw.local.18
	swapw
	loadw.local.22
	push.512.512.512.512

	exec.butterfly

	storew.local.22
	swapw
	storew.local.18

	loadw.local.19
	swapw
	loadw.local.23
	push.512.512.512.512

	exec.butterfly

	storew.local.23
	swapw
	storew.local.19

    # ---

    loadw.local.24
	swapw
	loadw.local.28
	push.18302628881338728449.18302628881338728449.18302628881338728449.18302628881338728449

	exec.butterfly

	storew.local.28
	swapw
	storew.local.24

	loadw.local.25
	swapw
	loadw.local.29
	push.18302628881338728449.18302628881338728449.18302628881338728449.18302628881338728449

	exec.butterfly

	storew.local.29
	swapw
	storew.local.25

	loadw.local.26
	swapw
	loadw.local.30
	push.18302628881338728449.18302628881338728449.18302628881338728449.18302628881338728449

	exec.butterfly

	storew.local.30
	swapw
	storew.local.26

	loadw.local.27
	swapw
	loadw.local.31
	push.18302628881338728449.18302628881338728449.18302628881338728449.18302628881338728449

	exec.butterfly

	storew.local.31
	swapw
	storew.local.27

    # ---

    loadw.local.32
	swapw
	loadw.local.36
	push.137438953440.137438953440.137438953440.137438953440

	exec.butterfly

	storew.local.36
	swapw
	storew.local.32

	loadw.local.33
	swapw
	loadw.local.37
	push.137438953440.137438953440.137438953440.137438953440

	exec.butterfly

	storew.local.37
	swapw
	storew.local.33

	loadw.local.34
	swapw
	loadw.local.38
	push.137438953440.137438953440.137438953440.137438953440

	exec.butterfly

	storew.local.38
	swapw
	storew.local.34

	loadw.local.35
	swapw
	loadw.local.39
	push.137438953440.137438953440.137438953440.137438953440

	exec.butterfly

	storew.local.39
	swapw
	storew.local.35

    # ---

    loadw.local.40
	swapw
	loadw.local.44
	push.2097152.2097152.2097152.2097152

	exec.butterfly

	storew.local.44
	swapw
	storew.local.40

	loadw.local.41
	swapw
	loadw.local.45
	push.2097152.2097152.2097152.2097152

	exec.butterfly

	storew.local.45
	swapw
	storew.local.41

	loadw.local.42
	swapw
	loadw.local.46
	push.2097152.2097152.2097152.2097152

	exec.butterfly

	storew.local.46
	swapw
	storew.local.42

	loadw.local.43
	swapw
	loadw.local.47
	push.2097152.2097152.2097152.2097152

	exec.butterfly

	storew.local.47
	swapw
	storew.local.43

    # ---

    loadw.local.48
	swapw
	loadw.local.52
	push.18446708885042495489.18446708885042495489.18446708885042495489.18446708885042495489

	exec.butterfly

	storew.local.52
	swapw
	storew.local.48

	loadw.local.49
	swapw
	loadw.local.53
	push.18446708885042495489.18446708885042495489.18446708885042495489.18446708885042495489

	exec.butterfly

	storew.local.53
	swapw
	storew.local.49

	loadw.local.50
	swapw
	loadw.local.54
	push.18446708885042495489.18446708885042495489.18446708885042495489.18446708885042495489

	exec.butterfly

	storew.local.54
	swapw
	storew.local.50

	loadw.local.51
	swapw
	loadw.local.55
	push.18446708885042495489.18446708885042495489.18446708885042495489.18446708885042495489

	exec.butterfly

	storew.local.55
	swapw
	storew.local.51

    # ---

    loadw.local.56
	swapw
	loadw.local.60
	push.2305843008676823040.2305843008676823040.2305843008676823040.2305843008676823040

	exec.butterfly

	storew.local.60
	swapw
	storew.local.56

	loadw.local.57
	swapw
	loadw.local.61
	push.2305843008676823040.2305843008676823040.2305843008676823040.2305843008676823040

	exec.butterfly

	storew.local.61
	swapw
	storew.local.57

	loadw.local.58
	swapw
	loadw.local.62
	push.2305843008676823040.2305843008676823040.2305843008676823040.2305843008676823040

	exec.butterfly

	storew.local.62
	swapw
	storew.local.58

	loadw.local.59
	swapw
	loadw.local.63
	push.2305843008676823040.2305843008676823040.2305843008676823040.2305843008676823040

	exec.butterfly

	storew.local.63
	swapw
	storew.local.59

    # ---

    loadw.local.64
	swapw
	loadw.local.68
	push.8.8.8.8

	exec.butterfly

	storew.local.68
	swapw
	storew.local.64

	loadw.local.65
	swapw
	loadw.local.69
	push.8.8.8.8

	exec.butterfly

	storew.local.69
	swapw
	storew.local.65

	loadw.local.66
	swapw
	loadw.local.70
	push.8.8.8.8

	exec.butterfly

	storew.local.70
	swapw
	storew.local.66

	loadw.local.67
	swapw
	loadw.local.71
	push.8.8.8.8

	exec.butterfly

	storew.local.71
	swapw
	storew.local.67

    # ---

    loadw.local.72
	swapw
	loadw.local.76
	push.18444492269600899073.18444492269600899073.18444492269600899073.18444492269600899073

	exec.butterfly

	storew.local.76
	swapw
	storew.local.72

	loadw.local.73
	swapw
	loadw.local.77
	push.18444492269600899073.18444492269600899073.18444492269600899073.18444492269600899073

	exec.butterfly

	storew.local.77
	swapw
	storew.local.73

	loadw.local.74
	swapw
	loadw.local.78
	push.18444492269600899073.18444492269600899073.18444492269600899073.18444492269600899073

	exec.butterfly

	storew.local.78
	swapw
	storew.local.74

	loadw.local.75
	swapw
	loadw.local.79
	push.18444492269600899073.18444492269600899073.18444492269600899073.18444492269600899073

	exec.butterfly

	storew.local.79
	swapw
	storew.local.75

    # ---

    loadw.local.80
	swapw
	loadw.local.84
	push.8796093020160.8796093020160.8796093020160.8796093020160

	exec.butterfly

	storew.local.84
	swapw
	storew.local.80

	loadw.local.81
	swapw
	loadw.local.85
	push.8796093020160.8796093020160.8796093020160.8796093020160

	exec.butterfly

	storew.local.85
	swapw
	storew.local.81

	loadw.local.82
	swapw
	loadw.local.86
	push.8796093020160.8796093020160.8796093020160.8796093020160

	exec.butterfly

	storew.local.86
	swapw
	storew.local.82

	loadw.local.83
	swapw
	loadw.local.87
	push.8796093020160.8796093020160.8796093020160.8796093020160

	exec.butterfly

	storew.local.87
	swapw
	storew.local.83

    # ---

    loadw.local.88
	swapw
	loadw.local.92
	push.134217728.134217728.134217728.134217728

	exec.butterfly

	storew.local.92
	swapw
	storew.local.88

	loadw.local.89
	swapw
	loadw.local.93
	push.134217728.134217728.134217728.134217728

	exec.butterfly

	storew.local.93
	swapw
	storew.local.89

	loadw.local.90
	swapw
	loadw.local.94
	push.134217728.134217728.134217728.134217728

	exec.butterfly

	storew.local.94
	swapw
	storew.local.90

	loadw.local.91
	swapw
	loadw.local.95
	push.134217728.134217728.134217728.134217728

	exec.butterfly

	storew.local.95
	swapw
	storew.local.91

    # ---

    loadw.local.96
	swapw
	loadw.local.100
	push.18446743519658770433.18446743519658770433.18446743519658770433.18446743519658770433

	exec.butterfly

	storew.local.100
	swapw
	storew.local.96

	loadw.local.97
	swapw
	loadw.local.101
	push.18446743519658770433.18446743519658770433.18446743519658770433.18446743519658770433

	exec.butterfly

	storew.local.101
	swapw
	storew.local.97

	loadw.local.98
	swapw
	loadw.local.102
	push.18446743519658770433.18446743519658770433.18446743519658770433.18446743519658770433

	exec.butterfly

	storew.local.102
	swapw
	storew.local.98

	loadw.local.99
	swapw
	loadw.local.103
	push.18446743519658770433.18446743519658770433.18446743519658770433.18446743519658770433

	exec.butterfly

	storew.local.103
	swapw
	storew.local.99

    # ---

    loadw.local.104
	swapw
	loadw.local.108
	push.36028797010575360.36028797010575360.36028797010575360.36028797010575360

	exec.butterfly

	storew.local.108
	swapw
	storew.local.104

	loadw.local.105
	swapw
	loadw.local.109
	push.36028797010575360.36028797010575360.36028797010575360.36028797010575360

	exec.butterfly

	storew.local.109
	swapw
	storew.local.105

	loadw.local.106
	swapw
	loadw.local.110
	push.36028797010575360.36028797010575360.36028797010575360.36028797010575360

	exec.butterfly

	storew.local.110
	swapw
	storew.local.106

	loadw.local.107
	swapw
	loadw.local.111
	push.36028797010575360.36028797010575360.36028797010575360.36028797010575360

	exec.butterfly

	storew.local.111
	swapw
	storew.local.107

    # ---

    loadw.local.112
	swapw
	loadw.local.116
	push.32768.32768.32768.32768

	exec.butterfly

	storew.local.116
	swapw
	storew.local.112

	loadw.local.113
	swapw
	loadw.local.117
	push.32768.32768.32768.32768

	exec.butterfly

	storew.local.117
	swapw
	storew.local.113

	loadw.local.114
	swapw
	loadw.local.118
	push.32768.32768.32768.32768

	exec.butterfly

	storew.local.118
	swapw
	storew.local.114

	loadw.local.115
	swapw
	loadw.local.119
	push.32768.32768.32768.32768

	exec.butterfly

	storew.local.119
	swapw
	storew.local.115

    # ---

    loadw.local.120
	swapw
	loadw.local.124
	push.9223372032559808513.9223372032559808513.9223372032559808513.9223372032559808513

	exec.butterfly

	storew.local.124
	swapw
	storew.local.120

	loadw.local.121
	swapw
	loadw.local.125
	push.9223372032559808513.9223372032559808513.9223372032559808513.9223372032559808513

	exec.butterfly

	storew.local.125
	swapw
	storew.local.121

	loadw.local.122
	swapw
	loadw.local.126
	push.9223372032559808513.9223372032559808513.9223372032559808513.9223372032559808513

	exec.butterfly

	storew.local.126
	swapw
	storew.local.122

	loadw.local.123
	swapw
	loadw.local.127
	push.9223372032559808513.9223372032559808513.9223372032559808513.9223372032559808513

	exec.butterfly

	storew.local.127
	swapw
	storew.local.123

    # iter = 5

	loadw.local.0
	swapw
	loadw.local.2
	push.72058693532778496.72058693532778496.72058693532778496.72058693532778496

	exec.butterfly

	storew.local.2
	swapw
	storew.local.0

	loadw.local.1
	swapw
	loadw.local.3
	push.72058693532778496.72058693532778496.72058693532778496.72058693532778496

	exec.butterfly

	storew.local.3
	swapw
	storew.local.1

	# ---

	loadw.local.4
	swapw
	loadw.local.6
	push.18374687574905061377.18374687574905061377.18374687574905061377.18374687574905061377

	exec.butterfly

	storew.local.6
	swapw
	storew.local.4

	loadw.local.5
	swapw
	loadw.local.7
	push.18374687574905061377.18374687574905061377.18374687574905061377.18374687574905061377

	exec.butterfly

	storew.local.7
	swapw
	storew.local.5

	# ---

	loadw.local.8
	swapw
	loadw.local.10
	push.18446744065119551490.18446744065119551490.18446744065119551490.18446744065119551490

	exec.butterfly

	storew.local.10
	swapw
	storew.local.8

	loadw.local.9
	swapw
	loadw.local.11
	push.18446744065119551490.18446744065119551490.18446744065119551490.18446744065119551490

	exec.butterfly

	storew.local.11
	swapw
	storew.local.9

	# ---

	loadw.local.12
	swapw
	loadw.local.14
	push.4294901759.4294901759.4294901759.4294901759

	exec.butterfly

	storew.local.14
	swapw
	storew.local.12

	loadw.local.13
	swapw
	loadw.local.15
	push.4294901759.4294901759.4294901759.4294901759

	exec.butterfly

	storew.local.15
	swapw
	storew.local.13

	# ---

	loadw.local.16
	swapw
	loadw.local.18
	push.18446726477496979457.18446726477496979457.18446726477496979457.18446726477496979457

	exec.butterfly

	storew.local.18
	swapw
	storew.local.16

	loadw.local.17
	swapw
	loadw.local.19
	push.18446726477496979457.18446726477496979457.18446726477496979457.18446726477496979457

	exec.butterfly

	storew.local.19
	swapw
	storew.local.17

	# ---

	loadw.local.20
	swapw
	loadw.local.22
	push.18446726476960108545.18446726476960108545.18446726476960108545.18446726476960108545

	exec.butterfly

	storew.local.22
	swapw
	storew.local.20

	loadw.local.21
	swapw
	loadw.local.23
	push.18446726476960108545.18446726476960108545.18446726476960108545.18446726476960108545

	exec.butterfly

	storew.local.23
	swapw
	storew.local.21

	# ---

	loadw.local.24
	swapw
	loadw.local.26
	push.4503599627370480.4503599627370480.4503599627370480.4503599627370480

	exec.butterfly

	storew.local.26
	swapw
	storew.local.24

	loadw.local.25
	swapw
	loadw.local.27
	push.4503599627370480.4503599627370480.4503599627370480.4503599627370480

	exec.butterfly

	storew.local.27
	swapw
	storew.local.25

	# ---

	loadw.local.28
	swapw
	loadw.local.30
	push.4503599627370512.4503599627370512.4503599627370512.4503599627370512

	exec.butterfly

	storew.local.30
	swapw
	storew.local.28

	loadw.local.29
	swapw
	loadw.local.31
	push.4503599627370512.4503599627370512.4503599627370512.4503599627370512

	exec.butterfly

	storew.local.31
	swapw
	storew.local.29

	# ---

	loadw.local.32
	swapw
	loadw.local.34
	push.18158513693262871553.18158513693262871553.18158513693262871553.18158513693262871553

	exec.butterfly

	storew.local.34
	swapw
	storew.local.32

	loadw.local.33
	swapw
	loadw.local.35
	push.18158513693262871553.18158513693262871553.18158513693262871553.18158513693262871553

	exec.butterfly

	storew.local.35
	swapw
	storew.local.33

	# ---

	loadw.local.36
	swapw
	loadw.local.38
	push.288230376151710720.288230376151710720.288230376151710720.288230376151710720

	exec.butterfly

	storew.local.38
	swapw
	storew.local.36

	loadw.local.37
	swapw
	loadw.local.39
	push.288230376151710720.288230376151710720.288230376151710720.288230376151710720

	exec.butterfly

	storew.local.39
	swapw
	storew.local.37

	# ---

	loadw.local.40
	swapw
	loadw.local.42
	push.18445618186687873025.18445618186687873025.18445618186687873025.18445618186687873025

	exec.butterfly

	storew.local.42
	swapw
	storew.local.40

	loadw.local.41
	swapw
	loadw.local.43
	push.18445618186687873025.18445618186687873025.18445618186687873025.18445618186687873025

	exec.butterfly

	storew.local.43
	swapw
	storew.local.41

	# ---

	loadw.local.44
	swapw
	loadw.local.46
	push.18445618152328134657.18445618152328134657.18445618152328134657.18445618152328134657

	exec.butterfly

	storew.local.46
	swapw
	storew.local.44

	loadw.local.45
	swapw
	loadw.local.47
	push.18445618152328134657.18445618152328134657.18445618152328134657.18445618152328134657

	exec.butterfly

	storew.local.47
	swapw
	storew.local.45

	# ---

	loadw.local.48
	swapw
	loadw.local.50
	push.4611756386097823744.4611756386097823744.4611756386097823744.4611756386097823744

	exec.butterfly

	storew.local.50
	swapw
	storew.local.48

	loadw.local.49
	swapw
	loadw.local.51
	push.4611756386097823744.4611756386097823744.4611756386097823744.4611756386097823744

	exec.butterfly

	storew.local.51
	swapw
	storew.local.49

	# ---

	loadw.local.52
	swapw
	loadw.local.54
	push.13835128420805115905.13835128420805115905.13835128420805115905.13835128420805115905

	exec.butterfly

	storew.local.54
	swapw
	storew.local.52

	loadw.local.53
	swapw
	loadw.local.55
	push.13835128420805115905.13835128420805115905.13835128420805115905.13835128420805115905

	exec.butterfly

	storew.local.55
	swapw
	storew.local.53

	# ---

	loadw.local.56
	swapw
	loadw.local.58
	push.18446743794532483137.18446743794532483137.18446743794532483137.18446743794532483137

	exec.butterfly

	storew.local.58
	swapw
	storew.local.56

	loadw.local.57
	swapw
	loadw.local.59
	push.18446743794532483137.18446743794532483137.18446743794532483137.18446743794532483137

	exec.butterfly

	storew.local.59
	swapw
	storew.local.57

	# ---

	loadw.local.60
	swapw
	loadw.local.62
	push.274873712576.274873712576.274873712576.274873712576

	exec.butterfly

	storew.local.62
	swapw
	storew.local.60

	loadw.local.61
	swapw
	loadw.local.63
	push.274873712576.274873712576.274873712576.274873712576

	exec.butterfly

	storew.local.63
	swapw
	storew.local.61

	# ---

	loadw.local.64
	swapw
	loadw.local.66
	push.18446741870424883713.18446741870424883713.18446741870424883713.18446741870424883713

	exec.butterfly

	storew.local.66
	swapw
	storew.local.64

	loadw.local.65
	swapw
	loadw.local.67
	push.18446741870424883713.18446741870424883713.18446741870424883713.18446741870424883713

	exec.butterfly

	storew.local.67
	swapw
	storew.local.65

	# ---

	loadw.local.68
	swapw
	loadw.local.70
	push.18446741870357774849.18446741870357774849.18446741870357774849.18446741870357774849

	exec.butterfly

	storew.local.70
	swapw
	storew.local.68

	loadw.local.69
	swapw
	loadw.local.71
	push.18446741870357774849.18446741870357774849.18446741870357774849.18446741870357774849

	exec.butterfly

	storew.local.71
	swapw
	storew.local.69

	# ---

	loadw.local.72
	swapw
	loadw.local.74
	push.562949953421310.562949953421310.562949953421310.562949953421310

	exec.butterfly

	storew.local.74
	swapw
	storew.local.72

	loadw.local.73
	swapw
	loadw.local.75
	push.562949953421310.562949953421310.562949953421310.562949953421310

	exec.butterfly

	storew.local.75
	swapw
	storew.local.73

	# ---

	loadw.local.76
	swapw
	loadw.local.78
	push.562949953421314.562949953421314.562949953421314.562949953421314

	exec.butterfly

	storew.local.78
	swapw
	storew.local.76

	loadw.local.77
	swapw
	loadw.local.79
	push.562949953421314.562949953421314.562949953421314.562949953421314

	exec.butterfly

	storew.local.79
	swapw
	storew.local.77

	# ---

	loadw.local.80
	swapw
	loadw.local.82
	push.16140901060200882177.16140901060200882177.16140901060200882177.16140901060200882177

	exec.butterfly

	storew.local.82
	swapw
	storew.local.80

	loadw.local.81
	swapw
	loadw.local.83
	push.16140901060200882177.16140901060200882177.16140901060200882177.16140901060200882177

	exec.butterfly

	storew.local.83
	swapw
	storew.local.81

	# ---

	loadw.local.84
	swapw
	loadw.local.86
	push.2305843009213685760.2305843009213685760.2305843009213685760.2305843009213685760

	exec.butterfly

	storew.local.86
	swapw
	storew.local.84

	loadw.local.85
	swapw
	loadw.local.87
	push.2305843009213685760.2305843009213685760.2305843009213685760.2305843009213685760

	exec.butterfly

	storew.local.87
	swapw
	storew.local.85

	# ---

	loadw.local.88
	swapw
	loadw.local.90
	push.18437737007600893953.18437737007600893953.18437737007600893953.18437737007600893953

	exec.butterfly

	storew.local.90
	swapw
	storew.local.88

	loadw.local.89
	swapw
	loadw.local.91
	push.18437737007600893953.18437737007600893953.18437737007600893953.18437737007600893953

	exec.butterfly

	storew.local.91
	swapw
	storew.local.89

	# ---

	loadw.local.92
	swapw
	loadw.local.94
	push.18437736732722987009.18437736732722987009.18437736732722987009.18437736732722987009

	exec.butterfly

	storew.local.94
	swapw
	storew.local.92

	loadw.local.93
	swapw
	loadw.local.95
	push.18437736732722987009.18437736732722987009.18437736732722987009.18437736732722987009

	exec.butterfly

	storew.local.95
	swapw
	storew.local.93

	# ---

	loadw.local.96
	swapw
	loadw.local.98
	push.576469548262227968.576469548262227968.576469548262227968.576469548262227968

	exec.butterfly

	storew.local.98
	swapw
	storew.local.96

	loadw.local.97
	swapw
	loadw.local.99
	push.576469548262227968.576469548262227968.576469548262227968.576469548262227968

	exec.butterfly

	storew.local.99
	swapw
	storew.local.97

	# ---

	loadw.local.100
	swapw
	loadw.local.102
	push.17870292113338400769.17870292113338400769.17870292113338400769.17870292113338400769

	exec.butterfly

	storew.local.102
	swapw
	storew.local.100

	loadw.local.101
	swapw
	loadw.local.103
	push.17870292113338400769.17870292113338400769.17870292113338400769.17870292113338400769

	exec.butterfly

	storew.local.103
	swapw
	storew.local.101

	# ---

	loadw.local.104
	swapw
	loadw.local.106
	push.18446744035054321673.18446744035054321673.18446744035054321673.18446744035054321673

	exec.butterfly

	storew.local.106
	swapw
	storew.local.104

	loadw.local.105
	swapw
	loadw.local.107
	push.18446744035054321673.18446744035054321673.18446744035054321673.18446744035054321673

	exec.butterfly

	storew.local.107
	swapw
	storew.local.105

	# ---

	loadw.local.108
	swapw
	loadw.local.110
	push.34359214072.34359214072.34359214072.34359214072

	exec.butterfly

	storew.local.110
	swapw
	storew.local.108

	loadw.local.109
	swapw
	loadw.local.111
	push.34359214072.34359214072.34359214072.34359214072

	exec.butterfly

	storew.local.111
	swapw
	storew.local.109

	# ---

	loadw.local.112
	swapw
	loadw.local.114
	push.18446603334073745409.18446603334073745409.18446603334073745409.18446603334073745409

	exec.butterfly

	storew.local.114
	swapw
	storew.local.112

	loadw.local.113
	swapw
	loadw.local.115
	push.18446603334073745409.18446603334073745409.18446603334073745409.18446603334073745409

	exec.butterfly

	storew.local.115
	swapw
	storew.local.113

	# ---

	loadw.local.116
	swapw
	loadw.local.118
	push.18446603329778778113.18446603329778778113.18446603329778778113.18446603329778778113

	exec.butterfly

	storew.local.118
	swapw
	storew.local.116

	loadw.local.117
	swapw
	loadw.local.119
	push.18446603329778778113.18446603329778778113.18446603329778778113.18446603329778778113

	exec.butterfly

	storew.local.119
	swapw
	storew.local.117

	# ---

	loadw.local.120
	swapw
	loadw.local.122
	push.36028797018963840.36028797018963840.36028797018963840.36028797018963840

	exec.butterfly

	storew.local.122
	swapw
	storew.local.120

	loadw.local.121
	swapw
	loadw.local.123
	push.36028797018963840.36028797018963840.36028797018963840.36028797018963840

	exec.butterfly

	storew.local.123
	swapw
	storew.local.121

	# ---

	loadw.local.124
	swapw
	loadw.local.126
	push.36028797018964096.36028797018964096.36028797018964096.36028797018964096

	exec.butterfly

	storew.local.126
	swapw
	storew.local.124

	loadw.local.125
	swapw
	loadw.local.127
	push.36028797018964096.36028797018964096.36028797018964096.36028797018964096

	exec.butterfly

	storew.local.127
	swapw
	storew.local.125

	# iter = 6

    loadw.local.0
	swapw
	loadw.local.1
	push.16192975500896648969.16192975500896648969.16192975500896648969.16192975500896648969

	exec.butterfly

	storew.local.1
	swapw
	storew.local.0

	# ---

	loadw.local.2
	swapw
	loadw.local.3
	push.13801972045324315718.13801972045324315718.13801972045324315718.13801972045324315718

	exec.butterfly

	storew.local.3
	swapw
	storew.local.2

	# ---

	loadw.local.4
	swapw
	loadw.local.5
	push.10105805016917838453.10105805016917838453.10105805016917838453.10105805016917838453

	exec.butterfly

	storew.local.5
	swapw
	storew.local.4

	# ---

	loadw.local.6
	swapw
	loadw.local.7
	push.7884753188935386879.7884753188935386879.7884753188935386879.7884753188935386879

	exec.butterfly

	storew.local.7
	swapw
	storew.local.6

	# ---

	loadw.local.8
	swapw
	loadw.local.9
	push.4299803665592489687.4299803665592489687.4299803665592489687.4299803665592489687

	exec.butterfly

	storew.local.9
	swapw
	storew.local.8

	# ---

	loadw.local.10
	swapw
	loadw.local.11
	push.17330401598553671485.17330401598553671485.17330401598553671485.17330401598553671485

	exec.butterfly

	storew.local.11
	swapw
	storew.local.10

	# ---

	loadw.local.12
	swapw
	loadw.local.13
	push.10382722127243543029.10382722127243543029.10382722127243543029.10382722127243543029

	exec.butterfly

	storew.local.13
	swapw
	storew.local.12

	# ---

	loadw.local.14
	swapw
	loadw.local.15
	push.12053668962110821384.12053668962110821384.12053668962110821384.12053668962110821384

	exec.butterfly

	storew.local.15
	swapw
	storew.local.14

	# ---

	loadw.local.16
	swapw
	loadw.local.17
	push.3328437340319972906.3328437340319972906.3328437340319972906.3328437340319972906

	exec.butterfly

	storew.local.17
	swapw
	storew.local.16

	# ---

	loadw.local.18
	swapw
	loadw.local.19
	push.411429644661718300.411429644661718300.411429644661718300.411429644661718300

	exec.butterfly

	storew.local.19
	swapw
	storew.local.18

	# ---

	loadw.local.20
	swapw
	loadw.local.21
	push.16933017626115159474.16933017626115159474.16933017626115159474.16933017626115159474

	exec.butterfly

	storew.local.21
	swapw
	storew.local.20

	# ---

	loadw.local.22
	swapw
	loadw.local.23
	push.2341058142559915780.2341058142559915780.2341058142559915780.2341058142559915780

	exec.butterfly

	storew.local.23
	swapw
	storew.local.22

	# ---

	loadw.local.24
	swapw
	loadw.local.25
	push.3332764170168812040.3332764170168812040.3332764170168812040.3332764170168812040

	exec.butterfly

	storew.local.25
	swapw
	storew.local.24

	# ---

	loadw.local.26
	swapw
	loadw.local.27
	push.16329239638270742865.16329239638270742865.16329239638270742865.16329239638270742865

	exec.butterfly

	storew.local.27
	swapw
	storew.local.26

	# ---

	loadw.local.28
	swapw
	loadw.local.29
	push.1135478653231209757.1135478653231209757.1135478653231209757.1135478653231209757

	exec.butterfly

	storew.local.29
	swapw
	storew.local.28

	# ---

	loadw.local.30
	swapw
	loadw.local.31
	push.6562114217670983589.6562114217670983589.6562114217670983589.6562114217670983589

	exec.butterfly

	storew.local.31
	swapw
	storew.local.30

	# ---

	loadw.local.32
	swapw
	loadw.local.33
	push.2843318466875884251.2843318466875884251.2843318466875884251.2843318466875884251

	exec.butterfly

	storew.local.33
	swapw
	storew.local.32

	# ---

	loadw.local.34
	swapw
	loadw.local.35
	push.9083829225849678056.9083829225849678056.9083829225849678056.9083829225849678056

	exec.butterfly

	storew.local.35
	swapw
	storew.local.34

	# ---

	loadw.local.36
	swapw
	loadw.local.37
	push.8215369291935911999.8215369291935911999.8215369291935911999.8215369291935911999

	exec.butterfly

	storew.local.37
	swapw
	storew.local.36

	# ---

	loadw.local.38
	swapw
	loadw.local.39
	push.1506708620263852673.1506708620263852673.1506708620263852673.1506708620263852673

	exec.butterfly

	storew.local.39
	swapw
	storew.local.38

	# ---

	loadw.local.40
	swapw
	loadw.local.41
	push.8180754653145198927.8180754653145198927.8180754653145198927.8180754653145198927

	exec.butterfly

	storew.local.41
	swapw
	storew.local.40

	# ---

	loadw.local.42
	swapw
	loadw.local.43
	push.3291437157293746400.3291437157293746400.3291437157293746400.3291437157293746400

	exec.butterfly

	storew.local.43
	swapw
	storew.local.42

	# ---

	loadw.local.44
	swapw
	loadw.local.45
	push.6336932523019185545.6336932523019185545.6336932523019185545.6336932523019185545

	exec.butterfly

	storew.local.45
	swapw
	storew.local.44

	# ---

	loadw.local.46
	swapw
	loadw.local.47
	push.281721071064741919.281721071064741919.281721071064741919.281721071064741919

	exec.butterfly

	storew.local.47
	swapw
	storew.local.46

	# ---

	loadw.local.48
	swapw
	loadw.local.49
	push.416595521271101505.416595521271101505.416595521271101505.416595521271101505

	exec.butterfly

	storew.local.49
	swapw
	storew.local.48

	# ---

	loadw.local.50
	swapw
	loadw.local.51
	push.18182056015521604139.18182056015521604139.18182056015521604139.18182056015521604139

	exec.butterfly

	storew.local.51
	swapw
	storew.local.50

	# ---

	loadw.local.52
	swapw
	loadw.local.53
	push.7059463857684370340.7059463857684370340.7059463857684370340.7059463857684370340

	exec.butterfly

	storew.local.53
	swapw
	storew.local.52

	# ---

	loadw.local.54
	swapw
	loadw.local.55
	push.7737793303239342069.7737793303239342069.7737793303239342069.7737793303239342069

	exec.butterfly

	storew.local.55
	swapw
	storew.local.54

	# ---

	loadw.local.56
	swapw
	loadw.local.57
	push.15951685255325333175.15951685255325333175.15951685255325333175.15951685255325333175

	exec.butterfly

	storew.local.57
	swapw
	storew.local.56

	# ---

	loadw.local.58
	swapw
	loadw.local.59
	push.9516004302527281633.9516004302527281633.9516004302527281633.9516004302527281633

	exec.butterfly

	storew.local.59
	swapw
	storew.local.58

	# ---

	loadw.local.60
	swapw
	loadw.local.61
	push.9274800740290006948.9274800740290006948.9274800740290006948.9274800740290006948

	exec.butterfly

	storew.local.61
	swapw
	storew.local.60

	# ---

	loadw.local.62
	swapw
	loadw.local.63
	push.4195631349813649467.4195631349813649467.4195631349813649467.4195631349813649467

	exec.butterfly

	storew.local.63
	swapw
	storew.local.62

	# ---

	loadw.local.64
	swapw
	loadw.local.65
	push.5575382163818481237.5575382163818481237.5575382163818481237.5575382163818481237

	exec.butterfly

	storew.local.65
	swapw
	storew.local.64

	# ---

	loadw.local.66
	swapw
	loadw.local.67
	push.4404853092538523347.4404853092538523347.4404853092538523347.4404853092538523347

	exec.butterfly

	storew.local.67
	swapw
	storew.local.66

	# ---

	loadw.local.68
	swapw
	loadw.local.69
	push.8288405288461869359.8288405288461869359.8288405288461869359.8288405288461869359

	exec.butterfly

	storew.local.69
	swapw
	storew.local.68

	# ---

	loadw.local.70
	swapw
	loadw.local.71
	push.9952623958621855812.9952623958621855812.9952623958621855812.9952623958621855812

	exec.butterfly

	storew.local.71
	swapw
	storew.local.70

	# ---

	loadw.local.72
	swapw
	loadw.local.73
	push.1356658891109943458.1356658891109943458.1356658891109943458.1356658891109943458

	exec.butterfly

	storew.local.73
	swapw
	storew.local.72

	# ---

	loadw.local.74
	swapw
	loadw.local.75
	push.7298973816981743824.7298973816981743824.7298973816981743824.7298973816981743824

	exec.butterfly

	storew.local.75
	swapw
	storew.local.74

	# ---

	loadw.local.76
	swapw
	loadw.local.77
	push.18142929134658341675.18142929134658341675.18142929134658341675.18142929134658341675

	exec.butterfly

	storew.local.77
	swapw
	storew.local.76

	# ---

	loadw.local.78
	swapw
	loadw.local.79
	push.1362567150328163374.1362567150328163374.1362567150328163374.1362567150328163374

	exec.butterfly

	storew.local.79
	swapw
	storew.local.78

	# ---

	loadw.local.80
	swapw
	loadw.local.81
	push.5029422726070465669.5029422726070465669.5029422726070465669.5029422726070465669

	exec.butterfly

	storew.local.81
	swapw
	storew.local.80

	# ---

	loadw.local.82
	swapw
	loadw.local.83
	push.17449332314429639298.17449332314429639298.17449332314429639298.17449332314429639298

	exec.butterfly

	storew.local.83
	swapw
	storew.local.82

	# ---

	loadw.local.84
	swapw
	loadw.local.85
	push.13039192753378044028.13039192753378044028.13039192753378044028.13039192753378044028

	exec.butterfly

	storew.local.85
	swapw
	storew.local.84

	# ---

	loadw.local.86
	swapw
	loadw.local.87
	push.5965722551466996711.5965722551466996711.5965722551466996711.5965722551466996711

	exec.butterfly

	storew.local.87
	swapw
	storew.local.86

	# ---

	loadw.local.88
	swapw
	loadw.local.89
	push.6336321165505697069.6336321165505697069.6336321165505697069.6336321165505697069

	exec.butterfly

	storew.local.89
	swapw
	storew.local.88

	# ---

	loadw.local.90
	swapw
	loadw.local.91
	push.5209436881246729393.5209436881246729393.5209436881246729393.5209436881246729393

	exec.butterfly

	storew.local.91
	swapw
	storew.local.90

	# ---

	loadw.local.92
	swapw
	loadw.local.93
	push.13949104517951277988.13949104517951277988.13949104517951277988.13949104517951277988

	exec.butterfly

	storew.local.93
	swapw
	storew.local.92

	# ---

	loadw.local.94
	swapw
	loadw.local.95
	push.9778634991702905054.9778634991702905054.9778634991702905054.9778634991702905054

	exec.butterfly

	storew.local.95
	swapw
	storew.local.94

	# ---

	loadw.local.96
	swapw
	loadw.local.97
	push.14004640413449681173.14004640413449681173.14004640413449681173.14004640413449681173

	exec.butterfly

	storew.local.97
	swapw
	storew.local.96

	# ---

	loadw.local.98
	swapw
	loadw.local.99
	push.912371727122717978.912371727122717978.912371727122717978.912371727122717978

	exec.butterfly

	storew.local.99
	swapw
	storew.local.98

	# ---

	loadw.local.100
	swapw
	loadw.local.101
	push.13797081185216407910.13797081185216407910.13797081185216407910.13797081185216407910

	exec.butterfly

	storew.local.101
	swapw
	storew.local.100

	# ---

	loadw.local.102
	swapw
	loadw.local.103
	push.4782006911144666502.4782006911144666502.4782006911144666502.4782006911144666502

	exec.butterfly

	storew.local.103
	swapw
	storew.local.102

	# ---

	loadw.local.104
	swapw
	loadw.local.105
	push.3341893669734556710.3341893669734556710.3341893669734556710.3341893669734556710

	exec.butterfly

	storew.local.105
	swapw
	storew.local.104

	# ---

	loadw.local.106
	swapw
	loadw.local.107
	push.10467450029535024137.10467450029535024137.10467450029535024137.10467450029535024137

	exec.butterfly

	storew.local.107
	swapw
	storew.local.106

	# ---

	loadw.local.108
	swapw
	loadw.local.109
	push.12079821679951430619.12079821679951430619.12079821679951430619.12079821679951430619

	exec.butterfly

	storew.local.109
	swapw
	storew.local.108

	# ---

	loadw.local.110
	swapw
	loadw.local.111
	push.10832292272906805046.10832292272906805046.10832292272906805046.10832292272906805046

	exec.butterfly

	storew.local.111
	swapw
	storew.local.110

	# ---

	loadw.local.112
	swapw
	loadw.local.113
	push.7709569171718681254.7709569171718681254.7709569171718681254.7709569171718681254

	exec.butterfly

	storew.local.113
	swapw
	storew.local.112

	# ---

	loadw.local.114
	swapw
	loadw.local.115
	push.16792080670893602455.16792080670893602455.16792080670893602455.16792080670893602455

	exec.butterfly

	storew.local.115
	swapw
	storew.local.114

	# ---

	loadw.local.116
	swapw
	loadw.local.117
	push.10967010099451201909.10967010099451201909.10967010099451201909.10967010099451201909

	exec.butterfly

	storew.local.117
	swapw
	storew.local.116

	# ---

	loadw.local.118
	swapw
	loadw.local.119
	push.5834015391316509212.5834015391316509212.5834015391316509212.5834015391316509212

	exec.butterfly

	storew.local.119
	swapw
	storew.local.118

	# ---

	loadw.local.120
	swapw
	loadw.local.121
	push.10853271128879547664.10853271128879547664.10853271128879547664.10853271128879547664

	exec.butterfly

	storew.local.121
	swapw
	storew.local.120

	# ---

	loadw.local.122
	swapw
	loadw.local.123
	push.3051558327610197629.3051558327610197629.3051558327610197629.3051558327610197629

	exec.butterfly

	storew.local.123
	swapw
	storew.local.122

	# ---

	loadw.local.124
	swapw
	loadw.local.125
	push.16016224591364643153.16016224591364643153.16016224591364643153.16016224591364643153

	exec.butterfly

	storew.local.125
	swapw
	storew.local.124

	# ---

	loadw.local.126
	swapw
	loadw.local.127
	push.10900537202625306992.10900537202625306992.10900537202625306992.10900537202625306992

	exec.butterfly

	storew.local.127
	swapw
	storew.local.126

	# iter = 7

    loadw.local.0
    swapw
    loadw.local.1

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

    storew.local.1
	swapw
	storew.local.0

    # ---

    loadw.local.2
	swapw
	loadw.local.3

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

    storew.local.3
	swapw
	storew.local.2

    # ---

    loadw.local.4
	swapw
	loadw.local.5

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

    storew.local.5
	swapw
	storew.local.4

    # ---

    loadw.local.6
	swapw
	loadw.local.7

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

    storew.local.7
	swapw
	storew.local.6

    # ---

    loadw.local.8
	swapw
	loadw.local.9

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

    storew.local.9
	swapw
	storew.local.8

    # ---

    loadw.local.10
	swapw
	loadw.local.11

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

    storew.local.11
	swapw
	storew.local.10

    # ---

    loadw.local.12
	swapw
	loadw.local.13

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

    storew.local.13
	swapw
	storew.local.12

    # ---

    loadw.local.14
	swapw
	loadw.local.15

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

    storew.local.15
	swapw
	storew.local.14

    # ---

    loadw.local.16
	swapw
	loadw.local.17

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

    storew.local.17
	swapw
	storew.local.16

    # ---

    loadw.local.18
	swapw
	loadw.local.19

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

    storew.local.19
	swapw
	storew.local.18

    # ---

    loadw.local.20
	swapw
	loadw.local.21

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

    storew.local.21
	swapw
	storew.local.20

    # ---

    loadw.local.22
	swapw
	loadw.local.23

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

    storew.local.23
	swapw
	storew.local.22

    # ---

    loadw.local.24
	swapw
	loadw.local.25

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

    storew.local.25
	swapw
	storew.local.24

    # ---

    loadw.local.26
	swapw
	loadw.local.27

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

    storew.local.27
	swapw
	storew.local.26

    # ---

    loadw.local.28
	swapw
	loadw.local.29

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

    storew.local.29
	swapw
	storew.local.28

    # ---

    loadw.local.30
	swapw
	loadw.local.31

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

    storew.local.31
	swapw
	storew.local.30

    # ---

    loadw.local.32
	swapw
	loadw.local.33

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

    storew.local.33
	swapw
	storew.local.32

    # ---

    loadw.local.34
	swapw
	loadw.local.35

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

    storew.local.35
	swapw
	storew.local.34

    # ---

    loadw.local.36
	swapw
	loadw.local.37

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

    storew.local.37
	swapw
	storew.local.36

    # ---

    loadw.local.38
	swapw
	loadw.local.39

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

    storew.local.39
	swapw
	storew.local.38

    # ---

    loadw.local.40
	swapw
	loadw.local.41

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

    storew.local.41
	swapw
	storew.local.40

    # ---

    loadw.local.42
	swapw
	loadw.local.43

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

    storew.local.43
	swapw
	storew.local.42

    # ---

    loadw.local.44
	swapw
	loadw.local.45

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

    storew.local.45
	swapw
	storew.local.44

    # ---

    loadw.local.46
	swapw
	loadw.local.47

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

    storew.local.47
	swapw
	storew.local.46

    # ---

    loadw.local.48
	swapw
	loadw.local.49

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

    storew.local.49
	swapw
	storew.local.48

    # ---

    loadw.local.50
	swapw
	loadw.local.51

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

    storew.local.51
	swapw
	storew.local.50

    # ---

    loadw.local.52
	swapw
	loadw.local.53

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

    storew.local.53
	swapw
	storew.local.52

    # ---

    loadw.local.54
	swapw
	loadw.local.55

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

    storew.local.55
	swapw
	storew.local.54

    # ---

    loadw.local.56
	swapw
	loadw.local.57

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

    storew.local.57
	swapw
	storew.local.56

    # ---

    loadw.local.58
	swapw
	loadw.local.59

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

    storew.local.59
	swapw
	storew.local.58

    # ---

    loadw.local.60
	swapw
	loadw.local.61

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

    storew.local.61
	swapw
	storew.local.60

    # ---

    loadw.local.62
	swapw
	loadw.local.63

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

    storew.local.63
	swapw
	storew.local.62

    # ---

    loadw.local.64
	swapw
	loadw.local.65

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

    storew.local.65
	swapw
	storew.local.64

    # ---

    loadw.local.66
	swapw
	loadw.local.67

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

    storew.local.67
	swapw
	storew.local.66

    # ---

    loadw.local.68
	swapw
	loadw.local.69

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

    storew.local.69
	swapw
	storew.local.68

    # ---

    loadw.local.70
	swapw
	loadw.local.71

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

    storew.local.71
	swapw
	storew.local.70

    # ---

    loadw.local.72
	swapw
	loadw.local.73

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

    storew.local.73
	swapw
	storew.local.72

    # ---

    loadw.local.74
	swapw
	loadw.local.75

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

    storew.local.75
	swapw
	storew.local.74

    # ---

    loadw.local.76
	swapw
	loadw.local.77

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

    storew.local.77
	swapw
	storew.local.76

    # ---

    loadw.local.78
	swapw
	loadw.local.79

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

    storew.local.79
	swapw
	storew.local.78

    # ---

    loadw.local.80
	swapw
	loadw.local.81

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

    storew.local.81
	swapw
	storew.local.80

    # ---

    loadw.local.82
	swapw
	loadw.local.83

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

    storew.local.83
	swapw
	storew.local.82

    # ---

    loadw.local.84
	swapw
	loadw.local.85

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

    storew.local.85
	swapw
	storew.local.84

    # ---

    loadw.local.86
	swapw
	loadw.local.87

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

    storew.local.87
	swapw
	storew.local.86

    # ---

    loadw.local.88
	swapw
	loadw.local.89

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

    storew.local.89
	swapw
	storew.local.88

    # ---

    loadw.local.90
	swapw
	loadw.local.91

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

    storew.local.91
	swapw
	storew.local.90

    # ---

    loadw.local.92
	swapw
	loadw.local.93

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

    storew.local.93
	swapw
	storew.local.92

    # ---

    loadw.local.94
	swapw
	loadw.local.95

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

    storew.local.95
	swapw
	storew.local.94

    # ---

    loadw.local.96
	swapw
	loadw.local.97

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

    storew.local.97
	swapw
	storew.local.96

    # ---

    loadw.local.98
	swapw
	loadw.local.99

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

    storew.local.99
	swapw
	storew.local.98

    # ---

    loadw.local.100
	swapw
	loadw.local.101

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

    storew.local.101
	swapw
	storew.local.100

    # ---

    loadw.local.102
	swapw
	loadw.local.103

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

    storew.local.103
	swapw
	storew.local.102

    # ---

    loadw.local.104
	swapw
	loadw.local.105

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

    storew.local.105
	swapw
	storew.local.104

    # ---

    loadw.local.106
	swapw
	loadw.local.107

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

    storew.local.107
	swapw
	storew.local.106

    # ---

    loadw.local.108
	swapw
	loadw.local.109

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

    storew.local.109
	swapw
	storew.local.108

    # ---

    loadw.local.110
	swapw
	loadw.local.111

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

    storew.local.111
	swapw
	storew.local.110

    # ---

    loadw.local.112
	swapw
	loadw.local.113

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

    storew.local.113
	swapw
	storew.local.112

    # ---

    loadw.local.114
	swapw
	loadw.local.115

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

    storew.local.115
	swapw
	storew.local.114

    # ---

    loadw.local.116
	swapw
	loadw.local.117

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

    storew.local.117
	swapw
	storew.local.116

    # ---

    loadw.local.118
	swapw
	loadw.local.119

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

    storew.local.119
	swapw
	storew.local.118

    # ---

    loadw.local.120
	swapw
	loadw.local.121

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

    storew.local.121
	swapw
	storew.local.120

    # ---

    loadw.local.122
	swapw
	loadw.local.123

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

    storew.local.123
	swapw
	storew.local.122

    # ---

    loadw.local.124
	swapw
	loadw.local.125

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

    storew.local.125
	swapw
	storew.local.124

    # ---

    loadw.local.126
	swapw
	loadw.local.127

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

    storew.local.127
	swapw
	storew.local.126

    # iter = 8

    loadw.local.0
	swapw
	loadw.local.1

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

    storew.local.1
	swapw
	storew.local.0

    # ---

    loadw.local.2
	swapw
	loadw.local.3

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

    storew.local.3
	swapw
	storew.local.2

    # ---

    loadw.local.4
	swapw
	loadw.local.5

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

    storew.local.5
	swapw
	storew.local.4

    # ---

    loadw.local.6
	swapw
	loadw.local.7

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

    storew.local.7
	swapw
	storew.local.6

    # ---

    loadw.local.8
	swapw
	loadw.local.9

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

    storew.local.9
	swapw
	storew.local.8

    # ---

    loadw.local.10
	swapw
	loadw.local.11

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

    storew.local.11
	swapw
	storew.local.10

    # ---

    loadw.local.12
	swapw
	loadw.local.13

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

    storew.local.13
	swapw
	storew.local.12

    # ---

    loadw.local.14
	swapw
	loadw.local.15

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

    storew.local.15
	swapw
	storew.local.14

    # ---

    loadw.local.16
	swapw
	loadw.local.17

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

    storew.local.17
	swapw
	storew.local.16

    # ---

    loadw.local.18
	swapw
	loadw.local.19

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

    storew.local.19
	swapw
	storew.local.18

    # ---

    loadw.local.20
	swapw
	loadw.local.21

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

    storew.local.21
	swapw
	storew.local.20

    # ---

    loadw.local.22
	swapw
	loadw.local.23

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

    storew.local.23
	swapw
	storew.local.22

    # ---

    loadw.local.24
	swapw
	loadw.local.25

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

    storew.local.25
	swapw
	storew.local.24

    # ---

    loadw.local.26
	swapw
	loadw.local.27

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

    storew.local.27
	swapw
	storew.local.26

    # ---

    loadw.local.28
	swapw
	loadw.local.29

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

    storew.local.29
	swapw
	storew.local.28

    # ---

    loadw.local.30
	swapw
	loadw.local.31

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

    storew.local.31
	swapw
	storew.local.30

    # ---

    loadw.local.32
	swapw
	loadw.local.33

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

    storew.local.33
	swapw
	storew.local.32

    # ---

    loadw.local.34
	swapw
	loadw.local.35

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

    storew.local.35
	swapw
	storew.local.34

    # ---

    loadw.local.36
	swapw
	loadw.local.37

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

    storew.local.37
	swapw
	storew.local.36

    # ---

    loadw.local.38
	swapw
	loadw.local.39

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

    storew.local.39
	swapw
	storew.local.38

    # ---

    loadw.local.40
	swapw
	loadw.local.41

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

    storew.local.41
	swapw
	storew.local.40

    # ---

    loadw.local.42
	swapw
	loadw.local.43

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

    storew.local.43
	swapw
	storew.local.42

    # ---

    loadw.local.44
	swapw
	loadw.local.45

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

    storew.local.45
	swapw
	storew.local.44

    # ---

    loadw.local.46
	swapw
	loadw.local.47

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

    storew.local.47
	swapw
	storew.local.46

    # ---

    loadw.local.48
	swapw
	loadw.local.49

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

    storew.local.49
	swapw
	storew.local.48

    # ---

    loadw.local.50
	swapw
	loadw.local.51

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

    storew.local.51
	swapw
	storew.local.50

    # ---

    loadw.local.52
	swapw
	loadw.local.53

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

    storew.local.53
	swapw
	storew.local.52

    # ---

    loadw.local.54
	swapw
	loadw.local.55

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

    storew.local.55
	swapw
	storew.local.54

    # ---

    loadw.local.56
	swapw
	loadw.local.57

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

    storew.local.57
	swapw
	storew.local.56

    # ---

    loadw.local.58
	swapw
	loadw.local.59

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

    storew.local.59
	swapw
	storew.local.58

    # ---

    loadw.local.60
	swapw
	loadw.local.61

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

    storew.local.61
	swapw
	storew.local.60

    # ---

    loadw.local.62
	swapw
	loadw.local.63

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

    storew.local.63
	swapw
	storew.local.62

    # ---

    loadw.local.64
	swapw
	loadw.local.65

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

    storew.local.65
	swapw
	storew.local.64

    # ---

    loadw.local.66
	swapw
	loadw.local.67

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

    storew.local.67
	swapw
	storew.local.66

    # ---

    loadw.local.68
	swapw
	loadw.local.69

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

    storew.local.69
	swapw
	storew.local.68

    # ---

    loadw.local.70
	swapw
	loadw.local.71

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

    storew.local.71
	swapw
	storew.local.70

    # ---

    loadw.local.72
	swapw
	loadw.local.73

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

    storew.local.73
	swapw
	storew.local.72

    # ---

    loadw.local.74
	swapw
	loadw.local.75

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

    storew.local.75
	swapw
	storew.local.74

    # ---

    loadw.local.76
	swapw
	loadw.local.77

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

    storew.local.77
	swapw
	storew.local.76

    # ---

    loadw.local.78
	swapw
	loadw.local.79

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

    storew.local.79
	swapw
	storew.local.78

    # ---

    loadw.local.80
	swapw
	loadw.local.81

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

    storew.local.81
	swapw
	storew.local.80

    # ---

    loadw.local.82
	swapw
	loadw.local.83

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

    storew.local.83
	swapw
	storew.local.82

    # ---

    loadw.local.84
	swapw
	loadw.local.85

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

    storew.local.85
	swapw
	storew.local.84

    # ---

    loadw.local.86
	swapw
	loadw.local.87

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

    storew.local.87
	swapw
	storew.local.86

    # ---

    loadw.local.88
	swapw
	loadw.local.89

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

    storew.local.89
	swapw
	storew.local.88

    # ---

    loadw.local.90
	swapw
	loadw.local.91

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

    storew.local.91
	swapw
	storew.local.90

    # ---

    loadw.local.92
	swapw
	loadw.local.93

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

    storew.local.93
	swapw
	storew.local.92

    # ---

    loadw.local.94
	swapw
	loadw.local.95

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

    storew.local.95
	swapw
	storew.local.94

    # ---

    loadw.local.96
	swapw
	loadw.local.97

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

    storew.local.97
	swapw
	storew.local.96

    # ---

    loadw.local.98
	swapw
	loadw.local.99

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

    storew.local.99
	swapw
	storew.local.98

    # ---

    loadw.local.100
	swapw
	loadw.local.101

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

    storew.local.101
	swapw
	storew.local.100

    # ---

    loadw.local.102
	swapw
	loadw.local.103

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

    storew.local.103
	swapw
	storew.local.102

    # ---

    loadw.local.104
	swapw
	loadw.local.105

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

    storew.local.105
	swapw
	storew.local.104

    # ---

    loadw.local.106
	swapw
	loadw.local.107

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

    storew.local.107
	swapw
	storew.local.106

    # ---

    loadw.local.108
	swapw
	loadw.local.109

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

    storew.local.109
	swapw
	storew.local.108

    # ---

    loadw.local.110
	swapw
	loadw.local.111

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

    storew.local.111
	swapw
	storew.local.110

    # ---

    loadw.local.112
	swapw
	loadw.local.113

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

    storew.local.113
	swapw
	storew.local.112

    # ---

    loadw.local.114
	swapw
	loadw.local.115

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

    storew.local.115
	swapw
	storew.local.114

    # ---

    loadw.local.116
	swapw
	loadw.local.117

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

    storew.local.117
	swapw
	storew.local.116

    # ---

    loadw.local.118
	swapw
	loadw.local.119

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

    storew.local.119
	swapw
	storew.local.118

    # ---

    loadw.local.120
	swapw
	loadw.local.121

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

    storew.local.121
	swapw
	storew.local.120

    # ---

    loadw.local.122
	swapw
	loadw.local.123

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

    storew.local.123
	swapw
	storew.local.122

    # ---

    loadw.local.124
	swapw
	loadw.local.125

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

    storew.local.125
	swapw
	storew.local.124

    # ---

    loadw.local.126
	swapw
	loadw.local.127

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

    storew.local.127
	swapw
	storew.local.126

	dropw
	dropw

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

# Applies four inverse NTT butterflies on four different indices, given following stack state
#
# [k0, k1, k2, k3, A0, B0, C0, D0, A1, B1, C1, D1]
# 
# Here k`i` => i-th constant i.e. negative of ω raised to *some* power | ω => 2N -th primitive root of unity, N = 512
#
# A{0, 1} -> first inverse butterfly will be applied on these two elements
# B{0, 1} -> second inverse butterfly will be applied on these two elements
# C{0, 1} -> third inverse butterfly will be applied on these two elements
# D{0, 1} -> fourth inverse butterfly will be applied on these two elements
#
# Four independent inverse butterflies are applied in following way
#
# t0 = A1  			   | t1 = B1  			  | t2 = C1				 | t3 = D1
# --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- -
# A1' = t0 + A0		   | B1' = t1 + B0 		  | C1' = t2 + C0 		 | D1' = t3 + D0
# A0' = (t0 - A0) * k0 | B0' = (t1 - B0) * k1 | C0' = (t2 - C0) * k2 | D0' = (t3 - D0) * k3
#
# After four independent butterflies are applied, resulting stack state should look like
#
# [A0', B0', C0', D0', A1', B1', C1', D1']
proc.ibutterfly
	dupw.2
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

	movupw.2
	movupw.3

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

	movupw.2

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
end

# Given four elements on stack top, this routine multiplies each of them by invN = 18410715272404008961,
# such that N = 512
#
# invN = (1/ 512) modulo q | q = 2^64 - 2^32 + 1
#
# Expected input stack state:
#
# [a0, a1, a2, a3]
#
# After applying routine, stack looks like
#
# [a0', a1', a2', a3']
#
# a{i}' = (a{i} * invN) modulo q | i ∈ [0, 4)
proc.mul_by_invN
	push.18410715272404008961
	mul

	swap
	push.18410715272404008961
	mul
	swap

	movup.2
	push.18410715272404008961
	mul
	movdn.2

	movup.3
	push.18410715272404008961
	mul
	movdn.3
end

# Applies inverse NTT on a vector of length 512, where each element ∈ Zp | p = 2^64 − 2^32 + 1,
# producing elements in time domain in standard order, while input vector is expected to be in bit-reversed order.
#
# Static bit-reversed input vector ( i.e. [0..512) ) is accepted using function local memory, while after 
# applying inverse NTT, standard order output vector is also kept on same function local memory allocation --- this 
# section will be improved.
#
# This routine tests itself, but doesn't respond, in any meaningful way, when invoked from outside.
# The purpose of this function is asserting functional correctness of inverse NTT-512 implementation, while
# encapsulating the implementation.
export.backward.128
	# begin preparing development input

    push.500427581402858571.2463011040099663974.13255236560399798415.6147698371245747120
    popw.local.0
    push.4759188580461308943.2807061946257421748.7549830671545879423.9387934912362961312
    popw.local.1
    push.1388192801295105971.16298308954297267166.5463045908626770304.9379941516119320882
    popw.local.2
    push.1822341659846948200.7837612199872064446.13701360230683392458.3025227557696320612
    popw.local.3
    push.6661003217299415345.2462700283595696130.10287011748290197204.11593484223257809456
    popw.local.4
    push.5822565313136645408.13374962778532735081.9513121813828969915.4850126366893939331
    popw.local.5
    push.9638547351911424431.1284486619460005108.17760129267519767490.8639307051715995157
    popw.local.6
    push.13238262168040768060.12823255455035621696.14285126964169992246.17637713302469968180
    popw.local.7
    push.15517318235210799523.8090504461116217953.1357100511086411291.14223921105393507681
    popw.local.8
    push.11495840209035318117.4716997638082670922.1684552264558936468.16628668316477991361
    popw.local.9
    push.9373982051891349673.4375032670965432631.17813839478541020385.8712021422542957173
    popw.local.10
    push.7963379320027168585.15061433824670536866.14044826173259891759.1926093556853850187
    popw.local.11
    push.17117618612841673432.11152590746239846097.14431246337082958746.14691778372873896091
    popw.local.12
    push.303890312557052486.16546231301940396830.4084063692504982245.14862669681256781194
    popw.local.13
    push.12874831358695227040.8927299104241429512.1391337153954404998.7563043097586366000
    popw.local.14
    push.487169446989684856.3598143036621308872.3025759491575349789.17544691169439260104
    popw.local.15
    push.9893297249316795649.2882980834561558714.6080341855927780886.6666882237238639271
    popw.local.16
    push.3705911779901497798.11137670125329914006.3411987355997998953.4691550456846015466
    popw.local.17
    push.15796413081962541352.18296196013336192157.799757138718215649.15518117179961526146
    popw.local.18
    push.13210457250415703836.8162117950669587555.13196368854332472946.7347195004378104849
    popw.local.19
    push.8906355104260221321.6266966273130402481.657975828059970386.5452650972361431851
    popw.local.20
    push.8576758396778913845.5042479363983917261.14955072641990074669.1177534166750439230
    popw.local.21
    push.6334563537974372527.497910349234540620.3941628179342546836.8896467651864889830
    popw.local.22
    push.12425671526490530033.774530765448667322.9704822807517896607.13460463077709850377
    popw.local.23
    push.14318104668539212250.10225952875956458587.12498735210523199927.8065494508321203076
    popw.local.24
    push.12164956900973499431.8346724915334734524.6807078861963306588.6311439446657985780
    popw.local.25
    push.7234458743041469928.14244370490763271162.4239657429080771413.11445417186801385760
    popw.local.26
    push.11747418408334319738.14751110666906846153.5018146025947592012.2139508481948263956
    popw.local.27
    push.1973185003558773229.3702090447919014070.9021654833377757761.13564247872498662505
    popw.local.28
    push.3965062879362346539.3004215856372485490.10771985727518467339.4997611524108540570
    popw.local.29
    push.2312056974030883522.12465933509381032447.7706343149895562665.2219769481339195266
    popw.local.30
    push.4146773039321337126.9033129468865877333.2125184873369089124.5022786165037908972
    popw.local.31
    push.13678435790007910187.985664331094757213.17149781625007751176.4662969890869329954
    popw.local.32
    push.6861895092303478630.18012673797650473443.8222509106562826527.10150484129565507076
    popw.local.33
    push.5096773607628371884.14429583432640497786.14908977434228205095.16844253818212940857
    popw.local.34
    push.7458036209579834773.4955617923334453253.12367986838157190529.12395699022945804024
    popw.local.35
    push.13890387097742396980.6515488115992785920.18020016569845241237.9143623102345260628
    popw.local.36
    push.2905443428502578517.16875779936086595852.17390390595735418203.12996593942828004366
    popw.local.37
    push.18214974425583805276.13002876069586538529.9199250935105121089.893260271205161783
    popw.local.38
    push.6709534089282777356.12587785846481248845.5760084267134403683.5871387860019782338
    popw.local.39
    push.7245215770567422587.9607664737753257441.5716597287927550595.3123643438350635974
    popw.local.40
    push.5759208819531861758.17071143378045471018.8078443735104605287.16881060344087771413
    popw.local.41
    push.1473685036794685672.141332871558446952.6407770971883791349.12566331383071766254
    popw.local.42
    push.15369028662523164967.8861784141877776284.11894138910345911899.469389931994699889
    popw.local.43
    push.13875913004469395297.2607653131583029365.16586192596775565345.15664808304394316343
    popw.local.44
    push.16813439921261944248.14161113041748059521.13448834210131967244.8009025452734193607
    popw.local.45
    push.4083354598206562910.6251459103334690891.6137042318318339109.13995070365201497714
    popw.local.46
    push.12818043094147068368.9249214068095605136.3997466371394620132.14493471866567307848
    popw.local.47
    push.5394043712193799933.15239864752341206448.3588540624239934438.13015011347722958547
    popw.local.48
    push.11829529434721723333.12130786777414201524.3419201204817655041.3128672686463738131
    popw.local.49
    push.6140842141236339257.7500291227370840461.15516781916364721257.5151809639838655931
    popw.local.50
    push.13243563844154651511.762882067218168658.17726010777882042466.11690668512079064293
    popw.local.51
    push.3918065202896176478.11725206893555211986.17101791001764243145.10558916504036356933
    popw.local.52
    push.10298844172561774234.17149127386114289503.11567105031538459462.803701966684189555
    popw.local.53
    push.17482417277959843804.1810114756857547179.16018809242226017839.2803053123421498344
    popw.local.54
    push.13204537791056727921.8999334195551472243.13318542890811254177.78962158302801786
    popw.local.55
    push.11093885641065163374.13355052672226195728.7592879583466719407.12393637219601567574
    popw.local.56
    push.3411599003810787776.2809894435780251284.6794795563368961656.14284568787846633340
    popw.local.57
    push.16299086042706000560.3431893456709396822.6924418765183651467.1349795128299762686
    popw.local.58
    push.5027175826503224555.10153763531676333194.16264631822018623161.1583932303300965007
    popw.local.59
    push.7218523740236699299.17405989941569656846.1942275972796109015.7807832181012033386
    popw.local.60
    push.5715345431563076262.10005260587118472398.2393469611227278774.16405583093082798411
    popw.local.61
    push.8089586839234703419.18142885242969242100.9496386899791548746.16988798032855610364
    popw.local.62
    push.13755333174520886464.4289754023734046593.6558160888738056325.4370480212321907950
    popw.local.63
    push.1095320300961666936.12562227996983300356.16339859305517413399.1494047469102986519
    popw.local.64
    push.854047760369733976.14530409639472899115.5854381521638192751.10516920696937702284
    popw.local.65
    push.2794790736470622544.4139499927901685601.17422775311721259180.10904098461973645865
    popw.local.66
    push.1784571835427036944.4234979558599665650.9250129350657951385.7819865318527720810
    popw.local.67
    push.18142115543028914412.10045192632325951454.13588187127040443153.16289356085892318230
    popw.local.68
    push.17729711269237440361.539684312007626408.11430590887347575920.8111707298035988643
    popw.local.69
    push.8428499566768654683.6013205331994689614.13329490012213926327.11442474927709952764
    popw.local.70
    push.16150914592979533613.15471829253544609840.10586391764183012409.8762253157562392538
    popw.local.71
    push.6392279585533100357.17741352459101681712.11004455288779489872.8190839155548334141
    popw.local.72
    push.4226216350017515577.14880425420800392990.11015187404152435362.3457721476658561580
    popw.local.73
    push.14629100830552568255.7612415948668645325.14281378611832340921.2187849759750703504
    popw.local.74
    push.877672128107013028.2524681089307350526.5837373918705201518.1045809343595143126
    popw.local.75
    push.5920559297162622495.11259226891969400454.1828356708926881600.375546531597716834
    popw.local.76
    push.7917164687481063958.821000236133808134.4953050491861030149.1308096718102429686
    popw.local.77
    push.16363914024650516083.7468748459141628680.18151980406669433196.13281899104990252257
    popw.local.78
    push.8668665576569166951.5915985290118060008.15393692351097644975.4849705256848202365
    popw.local.79
    push.9554263480822607717.15384439712454326481.12628082128538211214.8300446822269204372
    popw.local.80
    push.14732015482137704088.2979547823604997972.7292662648869841962.12238253882750553858
    popw.local.81
    push.16519415090692894356.7198903717206004644.16536824509534370931.17320170229660244276
    popw.local.82
    push.14338251575625791894.1309308834650053177.14758645042887458947.7201793695830885568
    popw.local.83
    push.5716209785352087904.14537183770973180315.14516922055619556761.16250160353761920792
    popw.local.84
    push.3173145240318099632.12061541935646977447.3400167661560536242.8662993271246423627
    popw.local.85
    push.8719325338143121381.14364377592553572211.16797125524719969353.9779983570882049575
    popw.local.86
    push.5311413155140064380.14761196745008075213.13369371497705876272.3441777122008563845
    popw.local.87
    push.6874176837830348960.2543790951468878923.13603662903454429611.5334184973660170182
    popw.local.88
    push.2052697753100968697.2027790285986339592.10680417757191732550.50931374536222867
    popw.local.89
    push.8520311447926067785.8813959523570730359.17107102851440519601.17520109592597401123
    popw.local.90
    push.15783770904168829462.12164864114726281851.4206605768450390142.13694145796795458120
    popw.local.91
    push.14422448030219940688.11492798057464085775.16936924214205720797.5841129472983407877
    popw.local.92
    push.8925771421991641474.11266079826466912438.10951745108692586643.6507250621729077940
    popw.local.93
    push.17328677503143420133.8555391423963084189.1285178546161991355.431523066505444268
    popw.local.94
    push.10540350791453628906.14234637378559746361.13570070408968146198.13849482253040846080
    popw.local.95
    push.5008022948409793965.5530570934007304021.2449447675370487743.650415177035534115
    popw.local.96
    push.5441321007122731143.5781054458624323724.402139522378392068.8565569549360878607
    popw.local.97
    push.9589717239328382062.5030333191930369784.12296696041930375647.2141708533769732634
    popw.local.98
    push.6027690232728924930.1240774410427679033.963677192151881803.4543611496630253597
    popw.local.99
    push.2737555203331259767.1030608063572245572.9767474113466356503.2761035047280407935
    popw.local.100
    push.1618945402508718138.8922377290982201691.7374284328308128887.8841814831092757511
    popw.local.101
    push.12199497154981475450.3847290460963665198.3825152053016198315.11871822660196837602
    popw.local.102
    push.12598101527954240506.17350460119148800673.8664215757935537682.6994210944247916666
    popw.local.103
    push.16029478146060719718.2340432030104145622.12486487283416641615.2673356055215094872
    popw.local.104
    push.7096542296177827649.3100373081707278564.11972590391248175313.16877457782303791547
    popw.local.105
    push.15641825374164210288.16278699566452232814.14154189039299057850.7020793370540069683
    popw.local.106
    push.12698485145960549833.11369699668797798029.2672282024558520400.16507891939315727263
    popw.local.107
    push.5082896689942265990.3791366873874355075.618044444486631188.7312272130372524998
    popw.local.108
    push.696756705851448612.11040309650571431847.5473873804701425049.11707183425196948206
    popw.local.109
    push.9552704239592624845.14225547568313734297.17885312390645592151.3087123610587377460
    popw.local.110
    push.947019814211424257.9661188788039071482.1202539611214124159.16925721150628365575
    popw.local.111
    push.11242711842146528718.16517054008792145701.2661011257134350943.18382699622800959606
    popw.local.112
    push.1666674032747892084.9319704235952767853.5461734778947279056.8607463334904655371
    popw.local.113
    push.8251996508082379793.14059199343717200324.11841308919288316363.5163183654701938158
    popw.local.114
    push.4862507816692896766.11088068707522430673.12439375321289707536.14842082905936626972
    popw.local.115
    push.4336038796905611039.18188074785684425088.8085413517437825684.3956280272199302478
    popw.local.116
    push.9219657197664309210.14252391726017169882.11176792569105590956.7927475631615442421
    popw.local.117
    push.13245404777763005532.638525423443462447.14336541264070132902.792037797933480636
    popw.local.118
    push.360955568435882338.12055429652566909505.9571109272133006643.4268190099036611008
    popw.local.119
    push.6656980442522150754.5333503729525038918.9508126923216501423.10129622376870902946
    popw.local.120
    push.14946849288599984786.8954717507605904756.730511821011013473.14453836366880698356
    popw.local.121
    push.11499602538048347956.1012135699625126712.16540194865268245637.11497397510247217941
    popw.local.122
    push.7733872095211700020.17802026749540522578.1704669204159609424.9069880514720156888
    popw.local.123
    push.16405897176262122808.4286821862379136089.16258423984894898607.5686969124851045992
    popw.local.124
    push.7213033422352747723.8003252851648323516.11992570287022017322.10152352175912283478
    popw.local.125
    push.13914081284823479780.5942021648748510140.7617504743224559388.11886014067542442130
    popw.local.126
    push.16992025507167613444.7272726666596204218.8615776837219416649.6251952012393749681
    popw.local.127

	# end preparing development input
	# iter = 0

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

    push.12959998544531418328.16799868753440642108.6912136248533001504.257896047954371798

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.1
    swapw
    storew.local.0

    # ---

    loadw.local.2
    swapw
    loadw.local.3

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.12539708892944027283.12871350694930379971.13568943604939006010.14804671509201811970

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.3
    swapw
    storew.local.2

    # ---

    loadw.local.4
    swapw
    loadw.local.5

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.1941397000334789249.18101606309576656873.17783167795769062868.5280444194155204294

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.5
    swapw
    storew.local.4

    # ---

    loadw.local.6
    swapw
    loadw.local.7

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.11731715020642418612.1415419453610921553.9331374163590620309.12109705421302608020

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.7
    swapw
    storew.local.6

    # ---

    loadw.local.8
    swapw
    loadw.local.9

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.8083950796479296659.10737085212370118163.16317828492439126475.7756907657126989834

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.9
    swapw
    storew.local.8

    # ---

    loadw.local.10
    swapw
    loadw.local.11

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.16383575685779609937.18403601849434843390.11446268009178425019.5271741541623046617

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.11
    swapw
    storew.local.10

    # ---

    loadw.local.12
    swapw
    loadw.local.13

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.1619999818066427291.11323355628887372424.864017031066625188.4643923023347942555

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.13
    swapw
    storew.local.12

    # ---

    loadw.local.14
    swapw
    loadw.local.15

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.13096678655002118611.13138133880250412697.15531176002678313992.15685641990711164737

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.15
    swapw
    storew.local.14

    # ---

    loadw.local.16
    swapw
    loadw.local.17

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.11888040935601451494.9213008228395218639.15254225588420860719.4573069520345433394

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.17
    swapw
    storew.local.16

    # ---

    loadw.local.18
    swapw
    loadw.local.19

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.5556662515423975422.12458073038457296305.10599219190322488312.2185847064648409797

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.19
    swapw
    storew.local.18

    # ---

    loadw.local.20
    swapw
    loadw.local.21

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.2471455808525611920.17039746049376701324.4518113032494938455.17783460465441878945

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.21
    swapw
    storew.local.20

    # ---

    loadw.local.22
    swapw
    loadw.local.23

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.7679740417818447560.14267241681714216412.5138263668257324909.4106679476439837717

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.23
    swapw
    storew.local.22

    # ---

    loadw.local.24
    swapw
    loadw.local.25

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.7559811984562634734.7430863960585448835.11006777244921569212.17486776517187278376

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.25
    swapw
    storew.local.24

    # ---

    loadw.local.26
    swapw
    loadw.local.27

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.308931976065701490.11353340290879379826.2870607137738690347.18363833618917996149

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.27
    swapw
    storew.local.26

    # ---

    loadw.local.28
    swapw
    loadw.local.29

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.6097691134303827517.3457469037226225370.4212621207229430630.14406691742104117415

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.29
    swapw
    storew.local.28

    # ---

    loadw.local.30
    swapw
    loadw.local.31

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.5306268831781643008.17698160190544923319.1324902398790311039.7190759909111520345

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.31
    swapw
    storew.local.30

    # ---

    loadw.local.32
    swapw
    loadw.local.33

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.9627861440214039994.8740885839153244225.11780776132825664990.17917642060580152056

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.33
    swapw
    storew.local.32

    # ---

    loadw.local.34
    swapw
    loadw.local.35

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.2461841562019821461.15028382777741121447.8932772064328191883.15823298534785799625

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.35
    swapw
    storew.local.34

    # ---

    loadw.local.36
    swapw
    loadw.local.37

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.15415784495989080639.16099909724259186520.7440577883017277023.6014371623370100770

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.37
    swapw
    storew.local.36

    # ---

    loadw.local.38
    swapw
    loadw.local.39

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.1879817591510961655.18295090034566750882.9983907413951898936.2585806165873387916

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.39
    swapw
    storew.local.38

    # ---

    loadw.local.40
    swapw
    loadw.local.41

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.1247988426743987367.9546597805441465650.16121944306381782101.15905923861798891074

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.41
    swapw
    storew.local.40

    # ---

    loadw.local.42
    swapw
    loadw.local.43

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.4232816070675458120.2012488715532398315.3235915244053982668.14586854504982200837

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.43
    swapw
    storew.local.42

    # ---

    loadw.local.44
    swapw
    loadw.local.45

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.15038540732087693240.17233511790631916809.6084283033956854204.2239705257572519007

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.45
    swapw
    storew.local.44

    # ---

    loadw.local.46
    swapw
    loadw.local.47

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.7225259221282946803.4184390855894463221.12645811551425139186.18118813377585986234

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.47
    swapw
    storew.local.46

    # ---

    loadw.local.48
    swapw
    loadw.local.49

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.15594331550120231815.9983589126054413056.10755587837161802966.17078493612874372559

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.49
    swapw
    storew.local.48

    # ---

    loadw.local.50
    swapw
    loadw.local.51

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.3650541573257562281.11754060979178594938.14990416956088327889.4062943252717590188

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.51
    swapw
    storew.local.50

    # ---

    loadw.local.52
    swapw
    loadw.local.53

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.13781052940915215484.5828091010015769947.1913039459307282826.11760405707386568670

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.53
    swapw
    storew.local.52

    # ---

    loadw.local.54
    swapw
    loadw.local.55

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.16672792867292992783.155993580094600204.12273731855508974132.14390139890846703192

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.55
    swapw
    storew.local.54

    # ---

    loadw.local.56
    swapw
    loadw.local.57

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.10757588516645913927.1798767486355837899.9242871232219117186.14056801952326137183

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.57
    swapw
    storew.local.56

    # ---

    loadw.local.58
    swapw
    loadw.local.59

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.10946003652321694096.12257726419636086444.14074187984474348594.6081736730776967164

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.59
    swapw
    storew.local.58

    # ---

    loadw.local.60
    swapw
    loadw.local.61

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.4255134452441852017.1247948640756801632.5956134496998871451.4440654710286119610

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.61
    swapw
    storew.local.60

    # ---

    loadw.local.62
    swapw
    loadw.local.63

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.16597218757394956566.15304315674458262608.18014703180248802267.9731239941296990934

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.63
    swapw
    storew.local.62

    # ---

    loadw.local.64
    swapw
    loadw.local.65

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.14516885283035942005.11334068823260206571.8362186383759854260.4689912295553665450

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.65
    swapw
    storew.local.64

    # ---

    loadw.local.66
    swapw
    loadw.local.67

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.6108922642702621141.7305983592013185897.11626557742183179282.14358435046894549184

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.67
    swapw
    storew.local.66

    # ---

    loadw.local.68
    swapw
    loadw.local.69

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.13440262264613344657.224350547607727331.6743454643571072270.5957385981484432025

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.69
    swapw
    storew.local.68

    # ---

    loadw.local.70
    swapw
    loadw.local.71

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.3397360469478068274.6229932723140101208.3589423675261482283.6414348153479289383

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.71
    swapw
    storew.local.70

    # ---

    loadw.local.72
    swapw
    loadw.local.73

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.11977893002791800486.3107636527861734213.778741590392512651.4187015958668887546

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.73
    swapw
    storew.local.72

    # ---

    loadw.local.74
    swapw
    loadw.local.75

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.17820933843814429363.11557258861835081117.5454617847800030114.16885574308423315284

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.75
    swapw
    storew.local.74

    # ---

    loadw.local.76
    swapw
    loadw.local.77

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.8732139686409961871.12945973646291641022.10268645332677273943.14421297089005146422

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.77
    swapw
    storew.local.76

    # ---

    loadw.local.78
    swapw
    loadw.local.79

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.7681144356368296763.17054149009739409518.15288377769833835651.1794804380861818648

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.79
    swapw
    storew.local.78

    # ---

    loadw.local.80
    swapw
    loadw.local.81

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.16874606745241303831.11491806888718160052.9564262514387024666.7985079364596650331

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.81
    swapw
    storew.local.80

    # ---

    loadw.local.82
    swapw
    loadw.local.83

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.5649848956435614200.16885944481347625310.17638901753592829678.12781599562090518453

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.83
    swapw
    storew.local.82

    # ---

    loadw.local.84
    swapw
    loadw.local.85

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.5463754609422739804.3370246630088296031.10063675669397554566.16052622170793454809

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.85
    swapw
    storew.local.84

    # ---

    loadw.local.86
    swapw
    loadw.local.87

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.12081111149863113453.3638323995651455811.11102195893002206701.10789290780202129222

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.87
    swapw
    storew.local.86

    # ---

    loadw.local.88
    swapw
    loadw.local.89

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.8305303512655744958.5960347364878912233.11984005542840547177.10019076149651226019

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.89
    swapw
    storew.local.88

    # ---

    loadw.local.90
    swapw
    loadw.local.91

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.9906341360885134636.2727123837437860044.5869645476028340401.18147478832086943132

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.91
    swapw
    storew.local.90

    # ---

    loadw.local.92
    swapw
    loadw.local.93

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.4415168851831986019.10659847895797062167.15030590866359316324.12527349963958696492

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.93
    swapw
    storew.local.92

    # ---

    loadw.local.94
    swapw
    loadw.local.95

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.706231119554451775.6722429077522099244.6816548736552749790.8515228971291783927

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.95
    swapw
    storew.local.94

    # ---

    loadw.local.96
    swapw
    loadw.local.97

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.2526963974102883882.14807109221003868079.11285503742479007084.6392769726549651052

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.97
    swapw
    storew.local.96

    # ---

    loadw.local.98
    swapw
    loadw.local.99

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.2975131003309595864.1821014983830576591.9591778621339026828.16329435310479291959

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.99
    swapw
    storew.local.98

    # ---

    loadw.local.100
    swapw
    loadw.local.101

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.15137851097357772055.2849220811487664857.14151741787267893880.6871042604103756685

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.101
    swapw
    storew.local.100

    # ---

    loadw.local.102
    swapw
    loadw.local.103

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.6380552085956620921.5131277475016434399.5940943517668292086.5864494548669395898

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.103
    swapw
    storew.local.102

    # ---

    loadw.local.104
    swapw
    loadw.local.105

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.5354303957062182591.14568119870644612728.2947252693053877340.1508273997932245425

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.105
    swapw
    storew.local.104

    # ---

    loadw.local.106
    swapw
    loadw.local.107

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.4198074395846544547.16497053662173719388.1768967723408486735.7776409351543438706

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.107
    swapw
    storew.local.106

    # ---

    loadw.local.108
    swapw
    loadw.local.109

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.14150928548823798726.4156731661302306550.10634060002517168046.10022468250525998542

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.109
    swapw
    storew.local.108

    # ---

    loadw.local.110
    swapw
    loadw.local.111

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.371891375413699483.2533469881655645114.10422344362374670514.4347022422486734535

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.111
    swapw
    storew.local.110

    # ---

    loadw.local.112
    swapw
    loadw.local.113

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.3885345703086309931.13294663425500451833.919344321138294818.6130516502325630075

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.113
    swapw
    storew.local.112

    # ---

    loadw.local.114
    swapw
    loadw.local.115

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.18293846131416791945.13282612006153792674.13869829016883058002.2498549221880373044

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.115
    swapw
    storew.local.114

    # ---

    loadw.local.116
    swapw
    loadw.local.117

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.13475313378280530262.3497804344607115389.8854452095134239411.2308232038958038546

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.117
    swapw
    storew.local.116

    # ---

    loadw.local.118
    swapw
    loadw.local.119

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.6113546424387384073.2225341748615664720.8661276037555872257.1536941200771852370

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.119
    swapw
    storew.local.118

    # ---

    loadw.local.120
    swapw
    loadw.local.121

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.17223560565432245313.14027175702157419787.278167718576958090.1541649705628400031

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.121
    swapw
    storew.local.120

    # ---

    loadw.local.122
    swapw
    loadw.local.123

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.6296100189638712363.7354754569106358544.12636021555275895127.14123587056930693059

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.123
    swapw
    storew.local.122

    # ---

    loadw.local.124
    swapw
    loadw.local.125

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.12014883256269903942.17802733988925317760.13949976092203225093.12295529606174818960

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.125
    swapw
    storew.local.124

    # ---

    loadw.local.126
    swapw
    loadw.local.127

    movup.2
    swap
    movup.6
    movup.5

    movup.5
    movup.5
    movup.7
    movup.7

    push.18427631827164860274.15495384552830162325.15568786679171320491.9535690687442338791

    exec.ibutterfly

    movup.5
    swap
    movup.5

    movup.5
    movup.7
    movup.6
    movup.7

    storew.local.127
    swapw
    storew.local.126

    # iter = 1

    loadw.local.0
    swapw
    loadw.local.1

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.646951781859081502.646951781859081502.6028692054475264383.6028692054475264383

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.1
    swapw
    storew.local.0

    # ---

    loadw.local.2
    swapw
    loadw.local.3

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.14424608849493817968.14424608849493817968.9528805331154741816.9528805331154741816

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.3
    swapw
    storew.local.2

    # ---

    loadw.local.4
    swapw
    loadw.local.5

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.6667653815445493051.6667653815445493051.12030096568512274289.12030096568512274289

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.5
    swapw
    storew.local.4

    # ---

    loadw.local.6
    swapw
    loadw.local.7

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.3323814471437944900.3323814471437944900.16723337261179401086.16723337261179401086

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.7
    swapw
    storew.local.6

    # ---

    loadw.local.8
    swapw
    loadw.local.9

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.837762896875133902.837762896875133902.1100986903222193631.1100986903222193631

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.9
    swapw
    storew.local.8

    # ---

    loadw.local.10
    swapw
    loadw.local.11

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.1545333971289350229.1545333971289350229.4511425900152047486.4511425900152047486

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.11
    swapw
    storew.local.10

    # ---

    loadw.local.12
    swapw
    loadw.local.13

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.9809941408468046069.9809941408468046069.382428689435778886.382428689435778886

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.13
    swapw
    storew.local.12

    # ---

    loadw.local.14
    swapw
    loadw.local.15

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.4837070530626986986.4837070530626986986.2454730591976115881.2454730591976115881

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.15
    swapw
    storew.local.14

    # ---

    loadw.local.16
    swapw
    loadw.local.17

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.16447742384734775766.16447742384734775766.4007052201025272707.4007052201025272707

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.17
    swapw
    storew.local.16

    # ---

    loadw.local.18
    swapw
    loadw.local.19

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.8143771702088974879.8143771702088974879.4659489603533118441.4659489603533118441

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.19
    swapw
    storew.local.18

    # ---

    loadw.local.20
    swapw
    loadw.local.21

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.4716406379463037818.4716406379463037818.2443466371579597244.2443466371579597244

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.21
    swapw
    storew.local.20

    # ---

    loadw.local.22
    swapw
    loadw.local.23

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.7110695772441637899.7110695772441637899.5175614254872652016.5175614254872652016

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.23
    swapw
    storew.local.22

    # ---

    loadw.local.24
    swapw
    loadw.local.25

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.4692554990086031268.4692554990086031268.3059429515486231088.3059429515486231088

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.25
    swapw
    storew.local.24

    # ---

    loadw.local.26
    swapw
    loadw.local.27

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.1803076106186727246.1803076106186727246.1191100666394342727.1191100666394342727

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.27
    swapw
    storew.local.26

    # ---

    loadw.local.28
    swapw
    loadw.local.29

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.12362671770314801832.12362671770314801832.17644663131801795567.17644663131801795567

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.29
    swapw
    storew.local.28

    # ---

    loadw.local.30
    swapw
    loadw.local.31

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.9638848843637035273.9638848843637035273.6702103175001071216.6702103175001071216

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.31
    swapw
    storew.local.30

    # ---

    loadw.local.32
    swapw
    loadw.local.33

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.6333224326531788891.6333224326531788891.12514560211689189807.12514560211689189807

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.33
    swapw
    storew.local.32

    # ---

    loadw.local.34
    swapw
    loadw.local.35

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.526448012694119458.526448012694119458.14569244469219929255.14569244469219929255

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.35
    swapw
    storew.local.34

    # ---

    loadw.local.36
    swapw
    loadw.local.37

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.3863141824208378587.3863141824208378587.4764679877301742210.4764679877301742210

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.37
    swapw
    storew.local.36

    # ---

    loadw.local.38
    swapw
    loadw.local.39

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.18038462700764634276.18038462700764634276.16508747943021518732.16508747943021518732

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.39
    swapw
    storew.local.38

    # ---

    loadw.local.40
    swapw
    loadw.local.41

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.15245928743009060991.15245928743009060991.10094442559346256270.10094442559346256270

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.41
    swapw
    storew.local.40

    # ---

    loadw.local.42
    swapw
    loadw.local.43

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.10724885506133562476.10724885506133562476.17944731440328218283.17944731440328218283

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.43
    swapw
    storew.local.42

    # ---

    loadw.local.44
    swapw
    loadw.local.45

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.10763480545232365762.10763480545232365762.5095456396745892551.5095456396745892551

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.45
    swapw
    storew.local.44

    # ---

    loadw.local.46
    swapw
    loadw.local.47

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.8655137032736432017.8655137032736432017.7433403846946633395.7433403846946633395

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.47
    swapw
    storew.local.46

    # ---

    loadw.local.48
    swapw
    loadw.local.49

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.12458390524252444375.12458390524252444375.1223950879584769038.1223950879584769038

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.49
    swapw
    storew.local.48

    # ---

    loadw.local.50
    swapw
    loadw.local.51

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.15180493120214983961.15180493120214983961.2942775058270059609.2942775058270059609

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.51
    swapw
    storew.local.50

    # ---

    loadw.local.52
    swapw
    loadw.local.53

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.4211584101552955664.4211584101552955664.5873491337271928114.5873491337271928114

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.53
    swapw
    storew.local.52

    # ---

    loadw.local.54
    swapw
    loadw.local.55

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.10563982722973987470.10563982722973987470.13772306473425142486.13772306473425142486

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.55
    swapw
    storew.local.54

    # ---

    loadw.local.56
    swapw
    loadw.local.57

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.12320868084200588812.12320868084200588812.3870163035137971766.3870163035137971766

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.57
    swapw
    storew.local.56

    # ---

    loadw.local.58
    swapw
    loadw.local.59

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13900864053647703173.13900864053647703173.4126998567329314197.4126998567329314197

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.59
    swapw
    storew.local.58

    # ---

    loadw.local.60
    swapw
    loadw.local.61

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.12012107771410162524.12012107771410162524.14430643036723656017.14430643036723656017

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.61
    swapw
    storew.local.60

    # ---

    loadw.local.62
    swapw
    loadw.local.63

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.11478179872302871445.11478179872302871445.11286965527584982002.11286965527584982002

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.63
    swapw
    storew.local.62

    # ---

    loadw.local.64
    swapw
    loadw.local.65

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.16755100833091933884.16755100833091933884.1573035775395650770.1573035775395650770

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.65
    swapw
    storew.local.64

    # ---

    loadw.local.66
    swapw
    loadw.local.67

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.10350167037512812052.10350167037512812052.5464760906092500108.5464760906092500108

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.67
    swapw
    storew.local.66

    # ---

    loadw.local.68
    swapw
    loadw.local.69

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13205888274518958430.13205888274518958430.7005074122307514744.7005074122307514744

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.69
    swapw
    storew.local.68

    # ---

    loadw.local.70
    swapw
    loadw.local.71

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.10686628914424923326.10686628914424923326.3666314137763395334.3666314137763395334

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.71
    swapw
    storew.local.70

    # ---

    loadw.local.72
    swapw
    loadw.local.73

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.16774647971309520093.16774647971309520093.17703304740457489134.17703304740457489134

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.73
    swapw
    storew.local.72

    # ---

    loadw.local.74
    swapw
    loadw.local.75

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.10006174791165856646.10006174791165856646.2415297291837877958.2415297291837877958

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.75
    swapw
    storew.local.74

    # ---

    loadw.local.76
    swapw
    loadw.local.77

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.1414719954855472987.1414719954855472987.13283175983882289524.13283175983882289524

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.77
    swapw
    storew.local.76

    # ---

    loadw.local.78
    swapw
    loadw.local.79

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.12843857907683664409.12843857907683664409.15073366445557045075.15073366445557045075

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.79
    swapw
    storew.local.78

    # ---

    loadw.local.80
    swapw
    loadw.local.81

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.13413385849078745835.13413385849078745835.700360770216364989.700360770216364989

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.81
    swapw
    storew.local.80

    # ---

    loadw.local.82
    swapw
    loadw.local.83

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.11706055037741049324.11706055037741049324.10883769032692578351.10883769032692578351

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.83
    swapw
    storew.local.82

    # ---

    loadw.local.84
    swapw
    loadw.local.85

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.9014360022444159132.9014360022444159132.6824599109910832222.6824599109910832222

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.85
    swapw
    storew.local.84

    # ---

    loadw.local.86
    swapw
    loadw.local.87

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.5862457866249378161.5862457866249378161.4913598178833380825.4913598178833380825

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.87
    swapw
    storew.local.86

    # ---

    loadw.local.88
    swapw
    loadw.local.89

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.11317759638843783896.11317759638843783896.14031687523985394587.14031687523985394587

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.89
    swapw
    storew.local.88

    # ---

    loadw.local.90
    swapw
    loadw.local.91

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.10517142914396393667.10517142914396393667.9906467147968854674.9906467147968854674

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.91
    swapw
    storew.local.90

    # ---

    loadw.local.92
    swapw
    loadw.local.93

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.6262422051668515884.6262422051668515884.875634265288439343.875634265288439343

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.93
    swapw
    storew.local.92

    # ---

    loadw.local.94
    swapw
    loadw.local.95

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.5947514631656761496.5947514631656761496.5069975284574070497.5069975284574070497

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.95
    swapw
    storew.local.94

    # ---

    loadw.local.96
    swapw
    loadw.local.97

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.12050543972821699434.12050543972821699434.15181754998655957376.15181754998655957376

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.97
    swapw
    storew.local.96

    # ---

    loadw.local.98
    swapw
    loadw.local.99

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.1857313538295938082.1857313538295938082.4831070854124318830.4831070854124318830

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.99
    swapw
    storew.local.98

    # ---

    loadw.local.100
    swapw
    loadw.local.101

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.17952527571176918316.17952527571176918316.13987726993667822989.13987726993667822989

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.101
    swapw
    storew.local.100

    # ---

    loadw.local.102
    swapw
    loadw.local.103

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.5290167988639048753.5290167988639048753.7497696261353643620.7497696261353643620

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.103
    swapw
    storew.local.102

    # ---

    loadw.local.104
    swapw
    loadw.local.105

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.8187602034452531322.8187602034452531322.14040629553323055984.14040629553323055984

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.105
    swapw
    storew.local.104

    # ---

    loadw.local.106
    swapw
    loadw.local.107

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.6045115764991696949.6045115764991696949.14918307414590806615.14918307414590806615

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.107
    swapw
    storew.local.106

    # ---

    loadw.local.108
    swapw
    loadw.local.109

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.6529358023436602414.6529358023436602414.237214921853999334.237214921853999334

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.109
    swapw
    storew.local.108

    # ---

    loadw.local.110
    swapw
    loadw.local.111

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.8675931806573960433.8675931806573960433.5263632251618544322.5263632251618544322

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.111
    swapw
    storew.local.110

    # ---

    loadw.local.112
    swapw
    loadw.local.113

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.14493012083513256281.14493012083513256281.1221351532855077986.1221351532855077986

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.113
    swapw
    storew.local.112

    # ---

    loadw.local.114
    swapw
    loadw.local.115

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.5427855770283221382.5427855770283221382.4641337882585395997.4641337882585395997

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.115
    swapw
    storew.local.114

    # ---

    loadw.local.116
    swapw
    loadw.local.117

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.14858508306367504656.14858508306367504656.1755078694165381998.1755078694165381998

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.117
    swapw
    storew.local.116

    # ---

    loadw.local.118
    swapw
    loadw.local.119

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.7673168496654431239.7673168496654431239.4170631435500673867.4170631435500673867

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.119
    swapw
    storew.local.118

    # ---

    loadw.local.120
    swapw
    loadw.local.121

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.15341376048663650670.15341376048663650670.1897719374831994672.1897719374831994672

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.121
    swapw
    storew.local.120

    # ---

    loadw.local.122
    swapw
    loadw.local.123

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.14067222244347930501.14067222244347930501.5215569874119185934.5215569874119185934

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.123
    swapw
    storew.local.122

    # ---

    loadw.local.124
    swapw
    loadw.local.125

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.11467437981104406950.11467437981104406950.8665994900238946994.8665994900238946994

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.125
    swapw
    storew.local.124

    # ---

    loadw.local.126
    swapw
    loadw.local.127

    movdn.5
    movdn.5
    movup.7
    movup.7

    push.16802172059317642375.16802172059317642375.10160584067376497613.10160584067376497613

    exec.ibutterfly

    movdn.5
    movdn.5
    movup.7
    movup.7

    storew.local.127
    swapw
    storew.local.126

    # iter = 2

    loadw.local.0
    swapw
    loadw.local.1

    push.7546206866789277329.7546206866789277329.7546206866789277329.7546206866789277329

    exec.ibutterfly

    storew.local.1
    swapw
    storew.local.0

    # ---

    loadw.local.2
    swapw
    loadw.local.3

    push.2430519478049941168.2430519478049941168.2430519478049941168.2430519478049941168

    exec.ibutterfly

    storew.local.3
    swapw
    storew.local.2

    # ---

    loadw.local.4
    swapw
    loadw.local.5

    push.15395185741804386692.15395185741804386692.15395185741804386692.15395185741804386692

    exec.ibutterfly

    storew.local.5
    swapw
    storew.local.4

    # ---

    loadw.local.6
    swapw
    loadw.local.7

    push.7593472940535036657.7593472940535036657.7593472940535036657.7593472940535036657

    exec.ibutterfly

    storew.local.7
    swapw
    storew.local.6

    # ---

    loadw.local.8
    swapw
    loadw.local.9

    push.12612728678098075109.12612728678098075109.12612728678098075109.12612728678098075109

    exec.ibutterfly

    storew.local.9
    swapw
    storew.local.8

    # ---

    loadw.local.10
    swapw
    loadw.local.11

    push.7479733969963382412.7479733969963382412.7479733969963382412.7479733969963382412

    exec.ibutterfly

    storew.local.11
    swapw
    storew.local.10

    # ---

    loadw.local.12
    swapw
    loadw.local.13

    push.1654663398520981866.1654663398520981866.1654663398520981866.1654663398520981866

    exec.ibutterfly

    storew.local.13
    swapw
    storew.local.12

    # ---

    loadw.local.14
    swapw
    loadw.local.15

    push.10737174897695903067.10737174897695903067.10737174897695903067.10737174897695903067

    exec.ibutterfly

    storew.local.15
    swapw
    storew.local.14

    # ---

    loadw.local.16
    swapw
    loadw.local.17

    push.7614451796507779275.7614451796507779275.7614451796507779275.7614451796507779275

    exec.ibutterfly

    storew.local.17
    swapw
    storew.local.16

    # ---

    loadw.local.18
    swapw
    loadw.local.19

    push.6366922389463153702.6366922389463153702.6366922389463153702.6366922389463153702

    exec.ibutterfly

    storew.local.19
    swapw
    storew.local.18

    # ---

    loadw.local.20
    swapw
    loadw.local.21

    push.7979294039879560184.7979294039879560184.7979294039879560184.7979294039879560184

    exec.ibutterfly

    storew.local.21
    swapw
    storew.local.20

    # ---

    loadw.local.22
    swapw
    loadw.local.23

    push.15104850399680027611.15104850399680027611.15104850399680027611.15104850399680027611

    exec.ibutterfly

    storew.local.23
    swapw
    storew.local.22

    # ---

    loadw.local.24
    swapw
    loadw.local.25

    push.13664737158269917819.13664737158269917819.13664737158269917819.13664737158269917819

    exec.ibutterfly

    storew.local.25
    swapw
    storew.local.24

    # ---

    loadw.local.26
    swapw
    loadw.local.27

    push.4649662884198176411.4649662884198176411.4649662884198176411.4649662884198176411

    exec.ibutterfly

    storew.local.27
    swapw
    storew.local.26

    # ---

    loadw.local.28
    swapw
    loadw.local.29

    push.17534372342291866343.17534372342291866343.17534372342291866343.17534372342291866343

    exec.ibutterfly

    storew.local.29
    swapw
    storew.local.28

    # ---

    loadw.local.30
    swapw
    loadw.local.31

    push.4442103655964903148.4442103655964903148.4442103655964903148.4442103655964903148

    exec.ibutterfly

    storew.local.31
    swapw
    storew.local.30

    # ---

    loadw.local.32
    swapw
    loadw.local.33

    push.8668109077711679267.8668109077711679267.8668109077711679267.8668109077711679267

    exec.ibutterfly

    storew.local.33
    swapw
    storew.local.32

    # ---

    loadw.local.34
    swapw
    loadw.local.35

    push.4497639551463306333.4497639551463306333.4497639551463306333.4497639551463306333

    exec.ibutterfly

    storew.local.35
    swapw
    storew.local.34

    # ---

    loadw.local.36
    swapw
    loadw.local.37

    push.13237307188167854928.13237307188167854928.13237307188167854928.13237307188167854928

    exec.ibutterfly

    storew.local.37
    swapw
    storew.local.36

    # ---

    loadw.local.38
    swapw
    loadw.local.39

    push.12110422903908887252.12110422903908887252.12110422903908887252.12110422903908887252

    exec.ibutterfly

    storew.local.39
    swapw
    storew.local.38

    # ---

    loadw.local.40
    swapw
    loadw.local.41

    push.12481021517947587610.12481021517947587610.12481021517947587610.12481021517947587610

    exec.ibutterfly

    storew.local.41
    swapw
    storew.local.40

    # ---

    loadw.local.42
    swapw
    loadw.local.43

    push.5407551316036540293.5407551316036540293.5407551316036540293.5407551316036540293

    exec.ibutterfly

    storew.local.43
    swapw
    storew.local.42

    # ---

    loadw.local.44
    swapw
    loadw.local.45

    push.997411754984945023.997411754984945023.997411754984945023.997411754984945023

    exec.ibutterfly

    storew.local.45
    swapw
    storew.local.44

    # ---

    loadw.local.46
    swapw
    loadw.local.47

    push.13417321343344118652.13417321343344118652.13417321343344118652.13417321343344118652

    exec.ibutterfly

    storew.local.47
    swapw
    storew.local.46

    # ---

    loadw.local.48
    swapw
    loadw.local.49

    push.17084176919086420947.17084176919086420947.17084176919086420947.17084176919086420947

    exec.ibutterfly

    storew.local.49
    swapw
    storew.local.48

    # ---

    loadw.local.50
    swapw
    loadw.local.51

    push.303814934756242646.303814934756242646.303814934756242646.303814934756242646

    exec.ibutterfly

    storew.local.51
    swapw
    storew.local.50

    # ---

    loadw.local.52
    swapw
    loadw.local.53

    push.11147770252432840497.11147770252432840497.11147770252432840497.11147770252432840497

    exec.ibutterfly

    storew.local.53
    swapw
    storew.local.52

    # ---

    loadw.local.54
    swapw
    loadw.local.55

    push.17090085178304640863.17090085178304640863.17090085178304640863.17090085178304640863

    exec.ibutterfly

    storew.local.55
    swapw
    storew.local.54

    # ---

    loadw.local.56
    swapw
    loadw.local.57

    push.8494120110792728509.8494120110792728509.8494120110792728509.8494120110792728509

    exec.ibutterfly

    storew.local.57
    swapw
    storew.local.56

    # ---

    loadw.local.58
    swapw
    loadw.local.59

    push.10158338780952714962.10158338780952714962.10158338780952714962.10158338780952714962

    exec.ibutterfly

    storew.local.59
    swapw
    storew.local.58

    # ---

    loadw.local.60
    swapw
    loadw.local.61

    push.14041890976876060974.14041890976876060974.14041890976876060974.14041890976876060974

    exec.ibutterfly

    storew.local.61
    swapw
    storew.local.60

    # ---

    loadw.local.62
    swapw
    loadw.local.63

    push.12871361905596103084.12871361905596103084.12871361905596103084.12871361905596103084

    exec.ibutterfly

    storew.local.63
    swapw
    storew.local.62

    # ---

    loadw.local.64
    swapw
    loadw.local.65

    push.14251112719600934854.14251112719600934854.14251112719600934854.14251112719600934854

    exec.ibutterfly

    storew.local.65
    swapw
    storew.local.64

    # ---

    loadw.local.66
    swapw
    loadw.local.67

    push.9171943329124577373.9171943329124577373.9171943329124577373.9171943329124577373

    exec.ibutterfly

    storew.local.67
    swapw
    storew.local.66

    # ---

    loadw.local.68
    swapw
    loadw.local.69

    push.8930739766887302688.8930739766887302688.8930739766887302688.8930739766887302688

    exec.ibutterfly

    storew.local.69
    swapw
    storew.local.68

    # ---

    loadw.local.70
    swapw
    loadw.local.71

    push.2495058814089251146.2495058814089251146.2495058814089251146.2495058814089251146

    exec.ibutterfly

    storew.local.71
    swapw
    storew.local.70

    # ---

    loadw.local.72
    swapw
    loadw.local.73

    push.10708950766175242252.10708950766175242252.10708950766175242252.10708950766175242252

    exec.ibutterfly

    storew.local.73
    swapw
    storew.local.72

    # ---

    loadw.local.74
    swapw
    loadw.local.75

    push.11387280211730213981.11387280211730213981.11387280211730213981.11387280211730213981

    exec.ibutterfly

    storew.local.75
    swapw
    storew.local.74

    # ---

    loadw.local.76
    swapw
    loadw.local.77

    push.264688053892980182.264688053892980182.264688053892980182.264688053892980182

    exec.ibutterfly

    storew.local.77
    swapw
    storew.local.76

    # ---

    loadw.local.78
    swapw
    loadw.local.79

    push.18030148548143482816.18030148548143482816.18030148548143482816.18030148548143482816

    exec.ibutterfly

    storew.local.79
    swapw
    storew.local.78

    # ---

    loadw.local.80
    swapw
    loadw.local.81

    push.18165022998349842402.18165022998349842402.18165022998349842402.18165022998349842402

    exec.ibutterfly

    storew.local.81
    swapw
    storew.local.80

    # ---

    loadw.local.82
    swapw
    loadw.local.83

    push.12109811546395398776.12109811546395398776.12109811546395398776.12109811546395398776

    exec.ibutterfly

    storew.local.83
    swapw
    storew.local.82

    # ---

    loadw.local.84
    swapw
    loadw.local.85

    push.15155306912120837921.15155306912120837921.15155306912120837921.15155306912120837921

    exec.ibutterfly

    storew.local.85
    swapw
    storew.local.84

    # ---

    loadw.local.86
    swapw
    loadw.local.87

    push.10265989416269385394.10265989416269385394.10265989416269385394.10265989416269385394

    exec.ibutterfly

    storew.local.87
    swapw
    storew.local.86

    # ---

    loadw.local.88
    swapw
    loadw.local.89

    push.16940035449150731648.16940035449150731648.16940035449150731648.16940035449150731648

    exec.ibutterfly

    storew.local.89
    swapw
    storew.local.88

    # ---

    loadw.local.90
    swapw
    loadw.local.91

    push.10231374777478672322.10231374777478672322.10231374777478672322.10231374777478672322

    exec.ibutterfly

    storew.local.91
    swapw
    storew.local.90

    # ---

    loadw.local.92
    swapw
    loadw.local.93

    push.9362914843564906265.9362914843564906265.9362914843564906265.9362914843564906265

    exec.ibutterfly

    storew.local.93
    swapw
    storew.local.92

    # ---

    loadw.local.94
    swapw
    loadw.local.95

    push.15603425602538700070.15603425602538700070.15603425602538700070.15603425602538700070

    exec.ibutterfly

    storew.local.95
    swapw
    storew.local.94

    # ---

    loadw.local.96
    swapw
    loadw.local.97

    push.11884629851743600732.11884629851743600732.11884629851743600732.11884629851743600732

    exec.ibutterfly

    storew.local.97
    swapw
    storew.local.96

    # ---

    loadw.local.98
    swapw
    loadw.local.99

    push.17311265416183374564.17311265416183374564.17311265416183374564.17311265416183374564

    exec.ibutterfly

    storew.local.99
    swapw
    storew.local.98

    # ---

    loadw.local.100
    swapw
    loadw.local.101

    push.2117504431143841456.2117504431143841456.2117504431143841456.2117504431143841456

    exec.ibutterfly

    storew.local.101
    swapw
    storew.local.100

    # ---

    loadw.local.102
    swapw
    loadw.local.103

    push.15113979899245772281.15113979899245772281.15113979899245772281.15113979899245772281

    exec.ibutterfly

    storew.local.103
    swapw
    storew.local.102

    # ---

    loadw.local.104
    swapw
    loadw.local.105

    push.16105685926854668541.16105685926854668541.16105685926854668541.16105685926854668541

    exec.ibutterfly

    storew.local.105
    swapw
    storew.local.104

    # ---

    loadw.local.106
    swapw
    loadw.local.107

    push.1513726443299424847.1513726443299424847.1513726443299424847.1513726443299424847

    exec.ibutterfly

    storew.local.107
    swapw
    storew.local.106

    # ---

    loadw.local.108
    swapw
    loadw.local.109

    push.18035314424752866021.18035314424752866021.18035314424752866021.18035314424752866021

    exec.ibutterfly

    storew.local.109
    swapw
    storew.local.108

    # ---

    loadw.local.110
    swapw
    loadw.local.111

    push.15118306729094611415.15118306729094611415.15118306729094611415.15118306729094611415

    exec.ibutterfly

    storew.local.111
    swapw
    storew.local.110

    # ---

    loadw.local.112
    swapw
    loadw.local.113

    push.6393075107303762937.6393075107303762937.6393075107303762937.6393075107303762937

    exec.ibutterfly

    storew.local.113
    swapw
    storew.local.112

    # ---

    loadw.local.114
    swapw
    loadw.local.115

    push.8064021942171041292.8064021942171041292.8064021942171041292.8064021942171041292

    exec.ibutterfly

    storew.local.115
    swapw
    storew.local.114

    # ---

    loadw.local.116
    swapw
    loadw.local.117

    push.1116342470860912836.1116342470860912836.1116342470860912836.1116342470860912836

    exec.ibutterfly

    storew.local.117
    swapw
    storew.local.116

    # ---

    loadw.local.118
    swapw
    loadw.local.119

    push.14146940403822094634.14146940403822094634.14146940403822094634.14146940403822094634

    exec.ibutterfly

    storew.local.119
    swapw
    storew.local.118

    # ---

    loadw.local.120
    swapw
    loadw.local.121

    push.10561990880479197442.10561990880479197442.10561990880479197442.10561990880479197442

    exec.ibutterfly

    storew.local.121
    swapw
    storew.local.120

    # ---

    loadw.local.122
    swapw
    loadw.local.123

    push.8340939052496745868.8340939052496745868.8340939052496745868.8340939052496745868

    exec.ibutterfly

    storew.local.123
    swapw
    storew.local.122

    # ---

    loadw.local.124
    swapw
    loadw.local.125

    push.4644772024090268603.4644772024090268603.4644772024090268603.4644772024090268603

    exec.ibutterfly

    storew.local.125
    swapw
    storew.local.124

    # ---

    loadw.local.126
    swapw
    loadw.local.127

    push.2253768568517935352.2253768568517935352.2253768568517935352.2253768568517935352

    exec.ibutterfly

    storew.local.127
    swapw
    storew.local.126

    # iter = 3

	loadw.local.0
    swapw
    loadw.local.2

    push.18410715272395620225.18410715272395620225.18410715272395620225.18410715272395620225

    exec.ibutterfly

    storew.local.2
    swapw
    storew.local.0

    loadw.local.1
    swapw
    loadw.local.3

    push.18410715272395620225.18410715272395620225.18410715272395620225.18410715272395620225

    exec.ibutterfly

    storew.local.3
    swapw
    storew.local.1

    # ---

    loadw.local.4
    swapw
    loadw.local.6

    push.18410715272395620481.18410715272395620481.18410715272395620481.18410715272395620481

    exec.ibutterfly

    storew.local.6
    swapw
    storew.local.4

    loadw.local.5
    swapw
    loadw.local.7

    push.18410715272395620481.18410715272395620481.18410715272395620481.18410715272395620481

    exec.ibutterfly

    storew.local.7
    swapw
    storew.local.5

    # ---

    loadw.local.8
    swapw
    loadw.local.10

    push.140739635806208.140739635806208.140739635806208.140739635806208

    exec.ibutterfly

    storew.local.10
    swapw
    storew.local.8

    loadw.local.9
    swapw
    loadw.local.11

    push.140739635806208.140739635806208.140739635806208.140739635806208

    exec.ibutterfly

    storew.local.11
    swapw
    storew.local.9

    # ---

    loadw.local.12
    swapw
    loadw.local.14

    push.140735340838912.140735340838912.140735340838912.140735340838912

    exec.ibutterfly

    storew.local.14
    swapw
    storew.local.12

    loadw.local.13
    swapw
    loadw.local.15

    push.140735340838912.140735340838912.140735340838912.140735340838912

    exec.ibutterfly

    storew.local.15
    swapw
    storew.local.13

    # ---

    loadw.local.16
    swapw
    loadw.local.18

    push.18446744035055370249.18446744035055370249.18446744035055370249.18446744035055370249

    exec.ibutterfly

    storew.local.18
    swapw
    storew.local.16

    loadw.local.17
    swapw
    loadw.local.19

    push.18446744035055370249.18446744035055370249.18446744035055370249.18446744035055370249

    exec.ibutterfly

    storew.local.19
    swapw
    storew.local.17

    # ---

    loadw.local.20
    swapw
    loadw.local.22

    push.34360262648.34360262648.34360262648.34360262648

    exec.ibutterfly

    storew.local.22
    swapw
    storew.local.20

    loadw.local.21
    swapw
    loadw.local.23

    push.34360262648.34360262648.34360262648.34360262648

    exec.ibutterfly

    storew.local.23
    swapw
    storew.local.21

    # ---

    loadw.local.24
    swapw
    loadw.local.26

    push.576451956076183552.576451956076183552.576451956076183552.576451956076183552

    exec.ibutterfly

    storew.local.26
    swapw
    storew.local.24

    loadw.local.25
    swapw
    loadw.local.27

    push.576451956076183552.576451956076183552.576451956076183552.576451956076183552

    exec.ibutterfly

    storew.local.27
    swapw
    storew.local.25

    # ---

    loadw.local.28
    swapw
    loadw.local.30

    push.17870274521152356353.17870274521152356353.17870274521152356353.17870274521152356353

    exec.ibutterfly

    storew.local.30
    swapw
    storew.local.28

    loadw.local.29
    swapw
    loadw.local.31

    push.17870274521152356353.17870274521152356353.17870274521152356353.17870274521152356353

    exec.ibutterfly

    storew.local.31
    swapw
    storew.local.29

    # ---

    loadw.local.32
    swapw
    loadw.local.34

    push.9007336691597312.9007336691597312.9007336691597312.9007336691597312

    exec.ibutterfly

    storew.local.34
    swapw
    storew.local.32

    loadw.local.33
    swapw
    loadw.local.35

    push.9007336691597312.9007336691597312.9007336691597312.9007336691597312

    exec.ibutterfly

    storew.local.35
    swapw
    storew.local.33

    # ---

    loadw.local.36
    swapw
    loadw.local.38

    push.9007061813690368.9007061813690368.9007061813690368.9007061813690368

    exec.ibutterfly

    storew.local.38
    swapw
    storew.local.36

    loadw.local.37
    swapw
    loadw.local.39

    push.9007061813690368.9007061813690368.9007061813690368.9007061813690368

    exec.ibutterfly

    storew.local.39
    swapw
    storew.local.37

    # ---

    loadw.local.40
    swapw
    loadw.local.42

    push.16140901060200898561.16140901060200898561.16140901060200898561.16140901060200898561

    exec.ibutterfly

    storew.local.42
    swapw
    storew.local.40

    loadw.local.41
    swapw
    loadw.local.43

    push.16140901060200898561.16140901060200898561.16140901060200898561.16140901060200898561

    exec.ibutterfly

    storew.local.43
    swapw
    storew.local.41

    # ---

    loadw.local.44
    swapw
    loadw.local.46

    push.2305843009213702144.2305843009213702144.2305843009213702144.2305843009213702144

    exec.ibutterfly

    storew.local.46
    swapw
    storew.local.44

    loadw.local.45
    swapw
    loadw.local.47

    push.2305843009213702144.2305843009213702144.2305843009213702144.2305843009213702144

    exec.ibutterfly

    storew.local.47
    swapw
    storew.local.45

    # ---

    loadw.local.48
    swapw
    loadw.local.50

    push.18446181119461163007.18446181119461163007.18446181119461163007.18446181119461163007

    exec.ibutterfly

    storew.local.50
    swapw
    storew.local.48

    loadw.local.49
    swapw
    loadw.local.51

    push.18446181119461163007.18446181119461163007.18446181119461163007.18446181119461163007

    exec.ibutterfly

    storew.local.51
    swapw
    storew.local.49

    # ---

    loadw.local.52
    swapw
    loadw.local.54

    push.18446181119461163011.18446181119461163011.18446181119461163011.18446181119461163011

    exec.ibutterfly

    storew.local.54
    swapw
    storew.local.52

    loadw.local.53
    swapw
    loadw.local.55

    push.18446181119461163011.18446181119461163011.18446181119461163011.18446181119461163011

    exec.ibutterfly

    storew.local.55
    swapw
    storew.local.53

    # ---

    loadw.local.56
    swapw
    loadw.local.58

    push.2199056809472.2199056809472.2199056809472.2199056809472

    exec.ibutterfly

    storew.local.58
    swapw
    storew.local.56

    loadw.local.57
    swapw
    loadw.local.59

    push.2199056809472.2199056809472.2199056809472.2199056809472

    exec.ibutterfly

    storew.local.59
    swapw
    storew.local.57

    # ---

    loadw.local.60
    swapw
    loadw.local.62

    push.2198989700608.2198989700608.2198989700608.2198989700608

    exec.ibutterfly

    storew.local.62
    swapw
    storew.local.60

    loadw.local.61
    swapw
    loadw.local.63

    push.2198989700608.2198989700608.2198989700608.2198989700608

    exec.ibutterfly

    storew.local.63
    swapw
    storew.local.61

    # ---

    loadw.local.64
    swapw
    loadw.local.66

    push.18446743794540871745.18446743794540871745.18446743794540871745.18446743794540871745

    exec.ibutterfly

    storew.local.66
    swapw
    storew.local.64

    loadw.local.65
    swapw
    loadw.local.67

    push.18446743794540871745.18446743794540871745.18446743794540871745.18446743794540871745

    exec.ibutterfly

    storew.local.67
    swapw
    storew.local.65

    # ---

    loadw.local.68
    swapw
    loadw.local.70

    push.274882101184.274882101184.274882101184.274882101184

    exec.ibutterfly

    storew.local.70
    swapw
    storew.local.68

    loadw.local.69
    swapw
    loadw.local.71

    push.274882101184.274882101184.274882101184.274882101184

    exec.ibutterfly

    storew.local.71
    swapw
    storew.local.69

    # ---

    loadw.local.72
    swapw
    loadw.local.74

    push.4611615648609468416.4611615648609468416.4611615648609468416.4611615648609468416

    exec.ibutterfly

    storew.local.74
    swapw
    storew.local.72

    loadw.local.73
    swapw
    loadw.local.75

    push.4611615648609468416.4611615648609468416.4611615648609468416.4611615648609468416

    exec.ibutterfly

    storew.local.75
    swapw
    storew.local.73

    # ---

    loadw.local.76
    swapw
    loadw.local.78

    push.13834987683316760577.13834987683316760577.13834987683316760577.13834987683316760577

    exec.ibutterfly

    storew.local.78
    swapw
    storew.local.76

    loadw.local.77
    swapw
    loadw.local.79

    push.13834987683316760577.13834987683316760577.13834987683316760577.13834987683316760577

    exec.ibutterfly

    storew.local.79
    swapw
    storew.local.77

    # ---

    loadw.local.80
    swapw
    loadw.local.82

    push.1125917086449664.1125917086449664.1125917086449664.1125917086449664

    exec.ibutterfly

    storew.local.82
    swapw
    storew.local.80

    loadw.local.81
    swapw
    loadw.local.83

    push.1125917086449664.1125917086449664.1125917086449664.1125917086449664

    exec.ibutterfly

    storew.local.83
    swapw
    storew.local.81

    # ---

    loadw.local.84
    swapw
    loadw.local.86

    push.1125882726711296.1125882726711296.1125882726711296.1125882726711296

    exec.ibutterfly

    storew.local.86
    swapw
    storew.local.84

    loadw.local.85
    swapw
    loadw.local.87

    push.1125882726711296.1125882726711296.1125882726711296.1125882726711296

    exec.ibutterfly

    storew.local.87
    swapw
    storew.local.85

    # ---

    loadw.local.88
    swapw
    loadw.local.90

    push.18158513693262873601.18158513693262873601.18158513693262873601.18158513693262873601

    exec.ibutterfly

    storew.local.90
    swapw
    storew.local.88

    loadw.local.89
    swapw
    loadw.local.91

    push.18158513693262873601.18158513693262873601.18158513693262873601.18158513693262873601

    exec.ibutterfly

    storew.local.91
    swapw
    storew.local.89

    # ---

    loadw.local.92
    swapw
    loadw.local.94

    push.288230376151712768.288230376151712768.288230376151712768.288230376151712768

    exec.ibutterfly

    storew.local.94
    swapw
    storew.local.92

    loadw.local.93
    swapw
    loadw.local.95

    push.288230376151712768.288230376151712768.288230376151712768.288230376151712768

    exec.ibutterfly

    storew.local.95
    swapw
    storew.local.93

    # ---

    loadw.local.96
    swapw
    loadw.local.98

    push.18442240469787213809.18442240469787213809.18442240469787213809.18442240469787213809

    exec.ibutterfly

    storew.local.98
    swapw
    storew.local.96

    loadw.local.97
    swapw
    loadw.local.99

    push.18442240469787213809.18442240469787213809.18442240469787213809.18442240469787213809

    exec.ibutterfly

    storew.local.99
    swapw
    storew.local.97

    # ---

    loadw.local.100
    swapw
    loadw.local.102

    push.18442240469787213841.18442240469787213841.18442240469787213841.18442240469787213841

    exec.ibutterfly

    storew.local.102
    swapw
    storew.local.100

    loadw.local.101
    swapw
    loadw.local.103

    push.18442240469787213841.18442240469787213841.18442240469787213841.18442240469787213841

    exec.ibutterfly

    storew.local.103
    swapw
    storew.local.101

    # ---

    loadw.local.104
    swapw
    loadw.local.106

    push.17592454475776.17592454475776.17592454475776.17592454475776

    exec.ibutterfly

    storew.local.106
    swapw
    storew.local.104

    loadw.local.105
    swapw
    loadw.local.107

    push.17592454475776.17592454475776.17592454475776.17592454475776

    exec.ibutterfly

    storew.local.107
    swapw
    storew.local.105

    # ---

    loadw.local.108
    swapw
    loadw.local.110

    push.17591917604864.17591917604864.17591917604864.17591917604864

    exec.ibutterfly

    storew.local.110
    swapw
    storew.local.108

    loadw.local.109
    swapw
    loadw.local.111

    push.17591917604864.17591917604864.17591917604864.17591917604864

    exec.ibutterfly

    storew.local.111
    swapw
    storew.local.109

    # ---

    loadw.local.112
    swapw
    loadw.local.114

    push.18446744065119682562.18446744065119682562.18446744065119682562.18446744065119682562

    exec.ibutterfly

    storew.local.114
    swapw
    storew.local.112

    loadw.local.113
    swapw
    loadw.local.115

    push.18446744065119682562.18446744065119682562.18446744065119682562.18446744065119682562

    exec.ibutterfly

    storew.local.115
    swapw
    storew.local.113

    # ---

    loadw.local.116
    swapw
    loadw.local.118

    push.4295032831.4295032831.4295032831.4295032831

    exec.ibutterfly

    storew.local.118
    swapw
    storew.local.116

    loadw.local.117
    swapw
    loadw.local.119

    push.4295032831.4295032831.4295032831.4295032831

    exec.ibutterfly

    storew.local.119
    swapw
    storew.local.117

    # ---

    loadw.local.120
    swapw
    loadw.local.122

    push.72056494509522944.72056494509522944.72056494509522944.72056494509522944

    exec.ibutterfly

    storew.local.122
    swapw
    storew.local.120

    loadw.local.121
    swapw
    loadw.local.123

    push.72056494509522944.72056494509522944.72056494509522944.72056494509522944

    exec.ibutterfly

    storew.local.123
    swapw
    storew.local.121

    # ---

    loadw.local.124
    swapw
    loadw.local.126

    push.18374685375881805825.18374685375881805825.18374685375881805825.18374685375881805825

    exec.ibutterfly

    storew.local.126
    swapw
    storew.local.124

    loadw.local.125
    swapw
    loadw.local.127

    push.18374685375881805825.18374685375881805825.18374685375881805825.18374685375881805825

    exec.ibutterfly

    storew.local.127
    swapw
    storew.local.125

    # iter = 4

	    loadw.local.0
    swapw
    loadw.local.4

    push.9223372036854775808.9223372036854775808.9223372036854775808.9223372036854775808

    exec.ibutterfly

    storew.local.4
    swapw
    storew.local.0

    loadw.local.1
    swapw
    loadw.local.5

    push.9223372036854775808.9223372036854775808.9223372036854775808.9223372036854775808

    exec.ibutterfly

    storew.local.5
    swapw
    storew.local.1

    loadw.local.2
    swapw
    loadw.local.6

    push.9223372036854775808.9223372036854775808.9223372036854775808.9223372036854775808

    exec.ibutterfly

    storew.local.6
    swapw
    storew.local.2

    loadw.local.3
    swapw
    loadw.local.7

    push.9223372036854775808.9223372036854775808.9223372036854775808.9223372036854775808

    exec.ibutterfly

    storew.local.7
    swapw
    storew.local.3

    # ---

    loadw.local.8
    swapw
    loadw.local.12

    push.18446744069414551553.18446744069414551553.18446744069414551553.18446744069414551553

    exec.ibutterfly

    storew.local.12
    swapw
    storew.local.8

    loadw.local.9
    swapw
    loadw.local.13

    push.18446744069414551553.18446744069414551553.18446744069414551553.18446744069414551553

    exec.ibutterfly

    storew.local.13
    swapw
    storew.local.9

    loadw.local.10
    swapw
    loadw.local.14

    push.18446744069414551553.18446744069414551553.18446744069414551553.18446744069414551553

    exec.ibutterfly

    storew.local.14
    swapw
    storew.local.10

    loadw.local.11
    swapw
    loadw.local.15

    push.18446744069414551553.18446744069414551553.18446744069414551553.18446744069414551553

    exec.ibutterfly

    storew.local.15
    swapw
    storew.local.11

    # ---

    loadw.local.16
    swapw
    loadw.local.20

    push.18410715272404008961.18410715272404008961.18410715272404008961.18410715272404008961

    exec.ibutterfly

    storew.local.20
    swapw
    storew.local.16

    loadw.local.17
    swapw
    loadw.local.21

    push.18410715272404008961.18410715272404008961.18410715272404008961.18410715272404008961

    exec.ibutterfly

    storew.local.21
    swapw
    storew.local.17

    loadw.local.18
    swapw
    loadw.local.22

    push.18410715272404008961.18410715272404008961.18410715272404008961.18410715272404008961

    exec.ibutterfly

    storew.local.22
    swapw
    storew.local.18

    loadw.local.19
    swapw
    loadw.local.23

    push.18410715272404008961.18410715272404008961.18410715272404008961.18410715272404008961

    exec.ibutterfly

    storew.local.23
    swapw
    storew.local.19

    # ---

    loadw.local.24
    swapw
    loadw.local.28

    push.549755813888.549755813888.549755813888.549755813888

    exec.ibutterfly

    storew.local.28
    swapw
    storew.local.24

    loadw.local.25
    swapw
    loadw.local.29

    push.549755813888.549755813888.549755813888.549755813888

    exec.ibutterfly

    storew.local.29
    swapw
    storew.local.25

    loadw.local.26
    swapw
    loadw.local.30

    push.549755813888.549755813888.549755813888.549755813888

    exec.ibutterfly

    storew.local.30
    swapw
    storew.local.26

    loadw.local.27
    swapw
    loadw.local.31

    push.549755813888.549755813888.549755813888.549755813888

    exec.ibutterfly

    storew.local.31
    swapw
    storew.local.27

    # ---

    loadw.local.32
    swapw
    loadw.local.36

    push.18446744069280366593.18446744069280366593.18446744069280366593.18446744069280366593

    exec.ibutterfly

    storew.local.36
    swapw
    storew.local.32

    loadw.local.33
    swapw
    loadw.local.37

    push.18446744069280366593.18446744069280366593.18446744069280366593.18446744069280366593

    exec.ibutterfly

    storew.local.37
    swapw
    storew.local.33

    loadw.local.34
    swapw
    loadw.local.38

    push.18446744069280366593.18446744069280366593.18446744069280366593.18446744069280366593

    exec.ibutterfly

    storew.local.38
    swapw
    storew.local.34

    loadw.local.35
    swapw
    loadw.local.39

    push.18446744069280366593.18446744069280366593.18446744069280366593.18446744069280366593

    exec.ibutterfly

    storew.local.39
    swapw
    storew.local.35

    # ---

    loadw.local.40
    swapw
    loadw.local.44

    push.18446735273321564161.18446735273321564161.18446735273321564161.18446735273321564161

    exec.ibutterfly

    storew.local.44
    swapw
    storew.local.40

    loadw.local.41
    swapw
    loadw.local.45

    push.18446735273321564161.18446735273321564161.18446735273321564161.18446735273321564161

    exec.ibutterfly

    storew.local.45
    swapw
    storew.local.41

    loadw.local.42
    swapw
    loadw.local.46

    push.18446735273321564161.18446735273321564161.18446735273321564161.18446735273321564161

    exec.ibutterfly

    storew.local.46
    swapw
    storew.local.42

    loadw.local.43
    swapw
    loadw.local.47

    push.18446735273321564161.18446735273321564161.18446735273321564161.18446735273321564161

    exec.ibutterfly

    storew.local.47
    swapw
    storew.local.43

    # ---

    loadw.local.48
    swapw
    loadw.local.52

    push.2251799813685248.2251799813685248.2251799813685248.2251799813685248

    exec.ibutterfly

    storew.local.52
    swapw
    storew.local.48

    loadw.local.49
    swapw
    loadw.local.53

    push.2251799813685248.2251799813685248.2251799813685248.2251799813685248

    exec.ibutterfly

    storew.local.53
    swapw
    storew.local.49

    loadw.local.50
    swapw
    loadw.local.54

    push.2251799813685248.2251799813685248.2251799813685248.2251799813685248

    exec.ibutterfly

    storew.local.54
    swapw
    storew.local.50

    loadw.local.51
    swapw
    loadw.local.55

    push.2251799813685248.2251799813685248.2251799813685248.2251799813685248

    exec.ibutterfly

    storew.local.55
    swapw
    storew.local.51

    # ---

    loadw.local.56
    swapw
    loadw.local.60

    push.18446744069414584313.18446744069414584313.18446744069414584313.18446744069414584313

    exec.ibutterfly

    storew.local.60
    swapw
    storew.local.56

    loadw.local.57
    swapw
    loadw.local.61

    push.18446744069414584313.18446744069414584313.18446744069414584313.18446744069414584313

    exec.ibutterfly

    storew.local.61
    swapw
    storew.local.57

    loadw.local.58
    swapw
    loadw.local.62

    push.18446744069414584313.18446744069414584313.18446744069414584313.18446744069414584313

    exec.ibutterfly

    storew.local.62
    swapw
    storew.local.58

    loadw.local.59
    swapw
    loadw.local.63

    push.18446744069414584313.18446744069414584313.18446744069414584313.18446744069414584313

    exec.ibutterfly

    storew.local.63
    swapw
    storew.local.59

    # ---

    loadw.local.64
    swapw
    loadw.local.68

    push.16140901060737761281.16140901060737761281.16140901060737761281.16140901060737761281

    exec.ibutterfly

    storew.local.68
    swapw
    storew.local.64

    loadw.local.65
    swapw
    loadw.local.69

    push.16140901060737761281.16140901060737761281.16140901060737761281.16140901060737761281

    exec.ibutterfly

    storew.local.69
    swapw
    storew.local.65

    loadw.local.66
    swapw
    loadw.local.70

    push.16140901060737761281.16140901060737761281.16140901060737761281.16140901060737761281

    exec.ibutterfly

    storew.local.70
    swapw
    storew.local.66

    loadw.local.67
    swapw
    loadw.local.71

    push.16140901060737761281.16140901060737761281.16140901060737761281.16140901060737761281

    exec.ibutterfly

    storew.local.71
    swapw
    storew.local.67

    # ---

    loadw.local.72
    swapw
    loadw.local.76

    push.35184372088832.35184372088832.35184372088832.35184372088832

    exec.ibutterfly

    storew.local.76
    swapw
    storew.local.72

    loadw.local.73
    swapw
    loadw.local.77

    push.35184372088832.35184372088832.35184372088832.35184372088832

    exec.ibutterfly

    storew.local.77
    swapw
    storew.local.73

    loadw.local.74
    swapw
    loadw.local.78

    push.35184372088832.35184372088832.35184372088832.35184372088832

    exec.ibutterfly

    storew.local.78
    swapw
    storew.local.74

    loadw.local.75
    swapw
    loadw.local.79

    push.35184372088832.35184372088832.35184372088832.35184372088832

    exec.ibutterfly

    storew.local.79
    swapw
    storew.local.75

    # ---

    loadw.local.80
    swapw
    loadw.local.84

    push.18446744069412487169.18446744069412487169.18446744069412487169.18446744069412487169

    exec.ibutterfly

    storew.local.84
    swapw
    storew.local.80

    loadw.local.81
    swapw
    loadw.local.85

    push.18446744069412487169.18446744069412487169.18446744069412487169.18446744069412487169

    exec.ibutterfly

    storew.local.85
    swapw
    storew.local.81

    loadw.local.82
    swapw
    loadw.local.86

    push.18446744069412487169.18446744069412487169.18446744069412487169.18446744069412487169

    exec.ibutterfly

    storew.local.86
    swapw
    storew.local.82

    loadw.local.83
    swapw
    loadw.local.87

    push.18446744069412487169.18446744069412487169.18446744069412487169.18446744069412487169

    exec.ibutterfly

    storew.local.87
    swapw
    storew.local.83

    # ---

    loadw.local.88
    swapw
    loadw.local.92

    push.18446743931975630881.18446743931975630881.18446743931975630881.18446743931975630881

    exec.ibutterfly

    storew.local.92
    swapw
    storew.local.88

    loadw.local.89
    swapw
    loadw.local.93

    push.18446743931975630881.18446743931975630881.18446743931975630881.18446743931975630881

    exec.ibutterfly

    storew.local.93
    swapw
    storew.local.89

    loadw.local.90
    swapw
    loadw.local.94

    push.18446743931975630881.18446743931975630881.18446743931975630881.18446743931975630881

    exec.ibutterfly

    storew.local.94
    swapw
    storew.local.90

    loadw.local.91
    swapw
    loadw.local.95

    push.18446743931975630881.18446743931975630881.18446743931975630881.18446743931975630881

    exec.ibutterfly

    storew.local.95
    swapw
    storew.local.91

    # ---

    loadw.local.96
    swapw
    loadw.local.100

    push.144115188075855872.144115188075855872.144115188075855872.144115188075855872

    exec.ibutterfly

    storew.local.100
    swapw
    storew.local.96

    loadw.local.97
    swapw
    loadw.local.101

    push.144115188075855872.144115188075855872.144115188075855872.144115188075855872

    exec.ibutterfly

    storew.local.101
    swapw
    storew.local.97

    loadw.local.98
    swapw
    loadw.local.102

    push.144115188075855872.144115188075855872.144115188075855872.144115188075855872

    exec.ibutterfly

    storew.local.102
    swapw
    storew.local.98

    loadw.local.99
    swapw
    loadw.local.103

    push.144115188075855872.144115188075855872.144115188075855872.144115188075855872

    exec.ibutterfly

    storew.local.103
    swapw
    storew.local.99

    # ---

    loadw.local.104
    swapw
    loadw.local.108

    push.18446744069414583809.18446744069414583809.18446744069414583809.18446744069414583809

    exec.ibutterfly

    storew.local.108
    swapw
    storew.local.104

    loadw.local.105
    swapw
    loadw.local.109

    push.18446744069414583809.18446744069414583809.18446744069414583809.18446744069414583809

    exec.ibutterfly

    storew.local.109
    swapw
    storew.local.105

    loadw.local.106
    swapw
    loadw.local.110

    push.18446744069414583809.18446744069414583809.18446744069414583809.18446744069414583809

    exec.ibutterfly

    storew.local.110
    swapw
    storew.local.106

    loadw.local.107
    swapw
    loadw.local.111

    push.18446744069414583809.18446744069414583809.18446744069414583809.18446744069414583809

    exec.ibutterfly

    storew.local.111
    swapw
    storew.local.107

    # ---

    loadw.local.112
    swapw
    loadw.local.116

    push.18446181119461294081.18446181119461294081.18446181119461294081.18446181119461294081

    exec.ibutterfly

    storew.local.116
    swapw
    storew.local.112

    loadw.local.113
    swapw
    loadw.local.117

    push.18446181119461294081.18446181119461294081.18446181119461294081.18446181119461294081

    exec.ibutterfly

    storew.local.117
    swapw
    storew.local.113

    loadw.local.114
    swapw
    loadw.local.118

    push.18446181119461294081.18446181119461294081.18446181119461294081.18446181119461294081

    exec.ibutterfly

    storew.local.118
    swapw
    storew.local.114

    loadw.local.115
    swapw
    loadw.local.119

    push.18446181119461294081.18446181119461294081.18446181119461294081.18446181119461294081

    exec.ibutterfly

    storew.local.119
    swapw
    storew.local.115

    # ---

    loadw.local.120
    swapw
    loadw.local.124

    push.8589934592.8589934592.8589934592.8589934592

    exec.ibutterfly

    storew.local.124
    swapw
    storew.local.120

    loadw.local.121
    swapw
    loadw.local.125

    push.8589934592.8589934592.8589934592.8589934592

    exec.ibutterfly

    storew.local.125
    swapw
    storew.local.121

    loadw.local.122
    swapw
    loadw.local.126

    push.8589934592.8589934592.8589934592.8589934592

    exec.ibutterfly

    storew.local.126
    swapw
    storew.local.122

    loadw.local.123
    swapw
    loadw.local.127

    push.8589934592.8589934592.8589934592.8589934592

    exec.ibutterfly

    storew.local.127
    swapw
    storew.local.123

    # iter = 5

	    loadw.local.0
    swapw
    loadw.local.8

    push.18446744068340842497.18446744068340842497.18446744068340842497.18446744068340842497

    exec.ibutterfly

    storew.local.8
    swapw
    storew.local.0

    loadw.local.1
    swapw
    loadw.local.9

    push.18446744068340842497.18446744068340842497.18446744068340842497.18446744068340842497

    exec.ibutterfly

    storew.local.9
    swapw
    storew.local.1

    loadw.local.2
    swapw
    loadw.local.10

    push.18446744068340842497.18446744068340842497.18446744068340842497.18446744068340842497

    exec.ibutterfly

    storew.local.10
    swapw
    storew.local.2

    loadw.local.3
    swapw
    loadw.local.11

    push.18446744068340842497.18446744068340842497.18446744068340842497.18446744068340842497

    exec.ibutterfly

    storew.local.11
    swapw
    storew.local.3

    loadw.local.4
    swapw
    loadw.local.12

    push.18446744068340842497.18446744068340842497.18446744068340842497.18446744068340842497

    exec.ibutterfly

    storew.local.12
    swapw
    storew.local.4

    loadw.local.5
    swapw
    loadw.local.13

    push.18446744068340842497.18446744068340842497.18446744068340842497.18446744068340842497

    exec.ibutterfly

    storew.local.13
    swapw
    storew.local.5

    loadw.local.6
    swapw
    loadw.local.14

    push.18446744068340842497.18446744068340842497.18446744068340842497.18446744068340842497

    exec.ibutterfly

    storew.local.14
    swapw
    storew.local.6

    loadw.local.7
    swapw
    loadw.local.15

    push.18446744068340842497.18446744068340842497.18446744068340842497.18446744068340842497

    exec.ibutterfly

    storew.local.15
    swapw
    storew.local.7

    # ---

    loadw.local.16
    swapw
    loadw.local.24

    push.18446673700670423041.18446673700670423041.18446673700670423041.18446673700670423041

    exec.ibutterfly

    storew.local.24
    swapw
    storew.local.16

    loadw.local.17
    swapw
    loadw.local.25

    push.18446673700670423041.18446673700670423041.18446673700670423041.18446673700670423041

    exec.ibutterfly

    storew.local.25
    swapw
    storew.local.17

    loadw.local.18
    swapw
    loadw.local.26

    push.18446673700670423041.18446673700670423041.18446673700670423041.18446673700670423041

    exec.ibutterfly

    storew.local.26
    swapw
    storew.local.18

    loadw.local.19
    swapw
    loadw.local.27

    push.18446673700670423041.18446673700670423041.18446673700670423041.18446673700670423041

    exec.ibutterfly

    storew.local.27
    swapw
    storew.local.19

    loadw.local.20
    swapw
    loadw.local.28

    push.18446673700670423041.18446673700670423041.18446673700670423041.18446673700670423041

    exec.ibutterfly

    storew.local.28
    swapw
    storew.local.20

    loadw.local.21
    swapw
    loadw.local.29

    push.18446673700670423041.18446673700670423041.18446673700670423041.18446673700670423041

    exec.ibutterfly

    storew.local.29
    swapw
    storew.local.21

    loadw.local.22
    swapw
    loadw.local.30

    push.18446673700670423041.18446673700670423041.18446673700670423041.18446673700670423041

    exec.ibutterfly

    storew.local.30
    swapw
    storew.local.22

    loadw.local.23
    swapw
    loadw.local.31

    push.18446673700670423041.18446673700670423041.18446673700670423041.18446673700670423041

    exec.ibutterfly

    storew.local.31
    swapw
    storew.local.23

    # ---

    loadw.local.32
    swapw
    loadw.local.40

    push.18014398509481984.18014398509481984.18014398509481984.18014398509481984

    exec.ibutterfly

    storew.local.40
    swapw
    storew.local.32

    loadw.local.33
    swapw
    loadw.local.41

    push.18014398509481984.18014398509481984.18014398509481984.18014398509481984

    exec.ibutterfly

    storew.local.41
    swapw
    storew.local.33

    loadw.local.34
    swapw
    loadw.local.42

    push.18014398509481984.18014398509481984.18014398509481984.18014398509481984

    exec.ibutterfly

    storew.local.42
    swapw
    storew.local.34

    loadw.local.35
    swapw
    loadw.local.43

    push.18014398509481984.18014398509481984.18014398509481984.18014398509481984

    exec.ibutterfly

    storew.local.43
    swapw
    storew.local.35

    loadw.local.36
    swapw
    loadw.local.44

    push.18014398509481984.18014398509481984.18014398509481984.18014398509481984

    exec.ibutterfly

    storew.local.44
    swapw
    storew.local.36

    loadw.local.37
    swapw
    loadw.local.45

    push.18014398509481984.18014398509481984.18014398509481984.18014398509481984

    exec.ibutterfly

    storew.local.45
    swapw
    storew.local.37

    loadw.local.38
    swapw
    loadw.local.46

    push.18014398509481984.18014398509481984.18014398509481984.18014398509481984

    exec.ibutterfly

    storew.local.46
    swapw
    storew.local.38

    loadw.local.39
    swapw
    loadw.local.47

    push.18014398509481984.18014398509481984.18014398509481984.18014398509481984

    exec.ibutterfly

    storew.local.47
    swapw
    storew.local.39

    # ---

    loadw.local.48
    swapw
    loadw.local.56

    push.18446744069414584257.18446744069414584257.18446744069414584257.18446744069414584257

    exec.ibutterfly

    storew.local.56
    swapw
    storew.local.48

    loadw.local.49
    swapw
    loadw.local.57

    push.18446744069414584257.18446744069414584257.18446744069414584257.18446744069414584257

    exec.ibutterfly

    storew.local.57
    swapw
    storew.local.49

    loadw.local.50
    swapw
    loadw.local.58

    push.18446744069414584257.18446744069414584257.18446744069414584257.18446744069414584257

    exec.ibutterfly

    storew.local.58
    swapw
    storew.local.50

    loadw.local.51
    swapw
    loadw.local.59

    push.18446744069414584257.18446744069414584257.18446744069414584257.18446744069414584257

    exec.ibutterfly

    storew.local.59
    swapw
    storew.local.51

    loadw.local.52
    swapw
    loadw.local.60

    push.18446744069414584257.18446744069414584257.18446744069414584257.18446744069414584257

    exec.ibutterfly

    storew.local.60
    swapw
    storew.local.52

    loadw.local.53
    swapw
    loadw.local.61

    push.18446744069414584257.18446744069414584257.18446744069414584257.18446744069414584257

    exec.ibutterfly

    storew.local.61
    swapw
    storew.local.53

    loadw.local.54
    swapw
    loadw.local.62

    push.18446744069414584257.18446744069414584257.18446744069414584257.18446744069414584257

    exec.ibutterfly

    storew.local.62
    swapw
    storew.local.54

    loadw.local.55
    swapw
    loadw.local.63

    push.18446744069414584257.18446744069414584257.18446744069414584257.18446744069414584257

    exec.ibutterfly

    storew.local.63
    swapw
    storew.local.55

    # ---

    loadw.local.64
    swapw
    loadw.local.72

    push.18158513693329981441.18158513693329981441.18158513693329981441.18158513693329981441

    exec.ibutterfly

    storew.local.72
    swapw
    storew.local.64

    loadw.local.65
    swapw
    loadw.local.73

    push.18158513693329981441.18158513693329981441.18158513693329981441.18158513693329981441

    exec.ibutterfly

    storew.local.73
    swapw
    storew.local.65

    loadw.local.66
    swapw
    loadw.local.74

    push.18158513693329981441.18158513693329981441.18158513693329981441.18158513693329981441

    exec.ibutterfly

    storew.local.74
    swapw
    storew.local.66

    loadw.local.67
    swapw
    loadw.local.75

    push.18158513693329981441.18158513693329981441.18158513693329981441.18158513693329981441

    exec.ibutterfly

    storew.local.75
    swapw
    storew.local.67

    loadw.local.68
    swapw
    loadw.local.76

    push.18158513693329981441.18158513693329981441.18158513693329981441.18158513693329981441

    exec.ibutterfly

    storew.local.76
    swapw
    storew.local.68

    loadw.local.69
    swapw
    loadw.local.77

    push.18158513693329981441.18158513693329981441.18158513693329981441.18158513693329981441

    exec.ibutterfly

    storew.local.77
    swapw
    storew.local.69

    loadw.local.70
    swapw
    loadw.local.78

    push.18158513693329981441.18158513693329981441.18158513693329981441.18158513693329981441

    exec.ibutterfly

    storew.local.78
    swapw
    storew.local.70

    loadw.local.71
    swapw
    loadw.local.79

    push.18158513693329981441.18158513693329981441.18158513693329981441.18158513693329981441

    exec.ibutterfly

    storew.local.79
    swapw
    storew.local.71

    # ---

    loadw.local.80
    swapw
    loadw.local.88

    push.4398046511104.4398046511104.4398046511104.4398046511104

    exec.ibutterfly

    storew.local.88
    swapw
    storew.local.80

    loadw.local.81
    swapw
    loadw.local.89

    push.4398046511104.4398046511104.4398046511104.4398046511104

    exec.ibutterfly

    storew.local.89
    swapw
    storew.local.81

    loadw.local.82
    swapw
    loadw.local.90

    push.4398046511104.4398046511104.4398046511104.4398046511104

    exec.ibutterfly

    storew.local.90
    swapw
    storew.local.82

    loadw.local.83
    swapw
    loadw.local.91

    push.4398046511104.4398046511104.4398046511104.4398046511104

    exec.ibutterfly

    storew.local.91
    swapw
    storew.local.83

    loadw.local.84
    swapw
    loadw.local.92

    push.4398046511104.4398046511104.4398046511104.4398046511104

    exec.ibutterfly

    storew.local.92
    swapw
    storew.local.84

    loadw.local.85
    swapw
    loadw.local.93

    push.4398046511104.4398046511104.4398046511104.4398046511104

    exec.ibutterfly

    storew.local.93
    swapw
    storew.local.85

    loadw.local.86
    swapw
    loadw.local.94

    push.4398046511104.4398046511104.4398046511104.4398046511104

    exec.ibutterfly

    storew.local.94
    swapw
    storew.local.86

    loadw.local.87
    swapw
    loadw.local.95

    push.4398046511104.4398046511104.4398046511104.4398046511104

    exec.ibutterfly

    storew.local.95
    swapw
    storew.local.87

    # ---

    loadw.local.96
    swapw
    loadw.local.104

    push.18446744069414322177.18446744069414322177.18446744069414322177.18446744069414322177

    exec.ibutterfly

    storew.local.104
    swapw
    storew.local.96

    loadw.local.97
    swapw
    loadw.local.105

    push.18446744069414322177.18446744069414322177.18446744069414322177.18446744069414322177

    exec.ibutterfly

    storew.local.105
    swapw
    storew.local.97

    loadw.local.98
    swapw
    loadw.local.106

    push.18446744069414322177.18446744069414322177.18446744069414322177.18446744069414322177

    exec.ibutterfly

    storew.local.106
    swapw
    storew.local.98

    loadw.local.99
    swapw
    loadw.local.107

    push.18446744069414322177.18446744069414322177.18446744069414322177.18446744069414322177

    exec.ibutterfly

    storew.local.107
    swapw
    storew.local.99

    loadw.local.100
    swapw
    loadw.local.108

    push.18446744069414322177.18446744069414322177.18446744069414322177.18446744069414322177

    exec.ibutterfly

    storew.local.108
    swapw
    storew.local.100

    loadw.local.101
    swapw
    loadw.local.109

    push.18446744069414322177.18446744069414322177.18446744069414322177.18446744069414322177

    exec.ibutterfly

    storew.local.109
    swapw
    storew.local.101

    loadw.local.102
    swapw
    loadw.local.110

    push.18446744069414322177.18446744069414322177.18446744069414322177.18446744069414322177

    exec.ibutterfly

    storew.local.110
    swapw
    storew.local.102

    loadw.local.103
    swapw
    loadw.local.111

    push.18446744069414322177.18446744069414322177.18446744069414322177.18446744069414322177

    exec.ibutterfly

    storew.local.111
    swapw
    storew.local.103

    # ---

    loadw.local.112
    swapw
    loadw.local.120

    push.18446744052234715141.18446744052234715141.18446744052234715141.18446744052234715141

    exec.ibutterfly

    storew.local.120
    swapw
    storew.local.112

    loadw.local.113
    swapw
    loadw.local.121

    push.18446744052234715141.18446744052234715141.18446744052234715141.18446744052234715141

    exec.ibutterfly

    storew.local.121
    swapw
    storew.local.113

    loadw.local.114
    swapw
    loadw.local.122

    push.18446744052234715141.18446744052234715141.18446744052234715141.18446744052234715141

    exec.ibutterfly

    storew.local.122
    swapw
    storew.local.114

    loadw.local.115
    swapw
    loadw.local.123

    push.18446744052234715141.18446744052234715141.18446744052234715141.18446744052234715141

    exec.ibutterfly

    storew.local.123
    swapw
    storew.local.115

    loadw.local.116
    swapw
    loadw.local.124

    push.18446744052234715141.18446744052234715141.18446744052234715141.18446744052234715141

    exec.ibutterfly

    storew.local.124
    swapw
    storew.local.116

    loadw.local.117
    swapw
    loadw.local.125

    push.18446744052234715141.18446744052234715141.18446744052234715141.18446744052234715141

    exec.ibutterfly

    storew.local.125
    swapw
    storew.local.117

    loadw.local.118
    swapw
    loadw.local.126

    push.18446744052234715141.18446744052234715141.18446744052234715141.18446744052234715141

    exec.ibutterfly

    storew.local.126
    swapw
    storew.local.118

    loadw.local.119
    swapw
    loadw.local.127

    push.18446744052234715141.18446744052234715141.18446744052234715141.18446744052234715141

    exec.ibutterfly

    storew.local.127
    swapw
    storew.local.119

    # iter = 6

	loadw.local.0
    swapw
    loadw.local.16

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.16
    swapw
    storew.local.0

    loadw.local.1
    swapw
    loadw.local.17

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.17
    swapw
    storew.local.1

    loadw.local.2
    swapw
    loadw.local.18

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.18
    swapw
    storew.local.2

    loadw.local.3
    swapw
    loadw.local.19

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.19
    swapw
    storew.local.3

    loadw.local.4
    swapw
    loadw.local.20

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.20
    swapw
    storew.local.4

    loadw.local.5
    swapw
    loadw.local.21

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.21
    swapw
    storew.local.5

    loadw.local.6
    swapw
    loadw.local.22

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.22
    swapw
    storew.local.6

    loadw.local.7
    swapw
    loadw.local.23

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.23
    swapw
    storew.local.7

    loadw.local.8
    swapw
    loadw.local.24

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.24
    swapw
    storew.local.8

    loadw.local.9
    swapw
    loadw.local.25

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.25
    swapw
    storew.local.9

    loadw.local.10
    swapw
    loadw.local.26

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.26
    swapw
    storew.local.10

    loadw.local.11
    swapw
    loadw.local.27

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.27
    swapw
    storew.local.11

    loadw.local.12
    swapw
    loadw.local.28

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.28
    swapw
    storew.local.12

    loadw.local.13
    swapw
    loadw.local.29

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.29
    swapw
    storew.local.13

    loadw.local.14
    swapw
    loadw.local.30

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.30
    swapw
    storew.local.14

    loadw.local.15
    swapw
    loadw.local.31

    push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976

    exec.ibutterfly

    storew.local.31
    swapw
    storew.local.15

    # ---

    loadw.local.32
    swapw
    loadw.local.48

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.48
    swapw
    storew.local.32

    loadw.local.33
    swapw
    loadw.local.49

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.49
    swapw
    storew.local.33

    loadw.local.34
    swapw
    loadw.local.50

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.50
    swapw
    storew.local.34

    loadw.local.35
    swapw
    loadw.local.51

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.51
    swapw
    storew.local.35

    loadw.local.36
    swapw
    loadw.local.52

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.52
    swapw
    storew.local.36

    loadw.local.37
    swapw
    loadw.local.53

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.53
    swapw
    storew.local.37

    loadw.local.38
    swapw
    loadw.local.54

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.54
    swapw
    storew.local.38

    loadw.local.39
    swapw
    loadw.local.55

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.55
    swapw
    storew.local.39

    loadw.local.40
    swapw
    loadw.local.56

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.56
    swapw
    storew.local.40

    loadw.local.41
    swapw
    loadw.local.57

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.57
    swapw
    storew.local.41

    loadw.local.42
    swapw
    loadw.local.58

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.58
    swapw
    storew.local.42

    loadw.local.43
    swapw
    loadw.local.59

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.59
    swapw
    storew.local.43

    loadw.local.44
    swapw
    loadw.local.60

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.60
    swapw
    storew.local.44

    loadw.local.45
    swapw
    loadw.local.61

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.61
    swapw
    storew.local.45

    loadw.local.46
    swapw
    loadw.local.62

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.62
    swapw
    storew.local.46

    loadw.local.47
    swapw
    loadw.local.63

    push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225

    exec.ibutterfly

    storew.local.63
    swapw
    storew.local.47

    # ---

    loadw.local.64
    swapw
    loadw.local.80

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.80
    swapw
    storew.local.64

    loadw.local.65
    swapw
    loadw.local.81

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.81
    swapw
    storew.local.65

    loadw.local.66
    swapw
    loadw.local.82

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.82
    swapw
    storew.local.66

    loadw.local.67
    swapw
    loadw.local.83

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.83
    swapw
    storew.local.67

    loadw.local.68
    swapw
    loadw.local.84

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.84
    swapw
    storew.local.68

    loadw.local.69
    swapw
    loadw.local.85

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.85
    swapw
    storew.local.69

    loadw.local.70
    swapw
    loadw.local.86

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.86
    swapw
    storew.local.70

    loadw.local.71
    swapw
    loadw.local.87

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.87
    swapw
    storew.local.71

    loadw.local.72
    swapw
    loadw.local.88

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.88
    swapw
    storew.local.72

    loadw.local.73
    swapw
    loadw.local.89

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.89
    swapw
    storew.local.73

    loadw.local.74
    swapw
    loadw.local.90

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.90
    swapw
    storew.local.74

    loadw.local.75
    swapw
    loadw.local.91

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.91
    swapw
    storew.local.75

    loadw.local.76
    swapw
    loadw.local.92

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.92
    swapw
    storew.local.76

    loadw.local.77
    swapw
    loadw.local.93

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.93
    swapw
    storew.local.77

    loadw.local.78
    swapw
    loadw.local.94

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.94
    swapw
    storew.local.78

    loadw.local.79
    swapw
    loadw.local.95

    push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401

    exec.ibutterfly

    storew.local.95
    swapw
    storew.local.79

    # ---

    loadw.local.96
    swapw
    loadw.local.112

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.112
    swapw
    storew.local.96

    loadw.local.97
    swapw
    loadw.local.113

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.113
    swapw
    storew.local.97

    loadw.local.98
    swapw
    loadw.local.114

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.114
    swapw
    storew.local.98

    loadw.local.99
    swapw
    loadw.local.115

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.115
    swapw
    storew.local.99

    loadw.local.100
    swapw
    loadw.local.116

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.116
    swapw
    storew.local.100

    loadw.local.101
    swapw
    loadw.local.117

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.117
    swapw
    storew.local.101

    loadw.local.102
    swapw
    loadw.local.118

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.118
    swapw
    storew.local.102

    loadw.local.103
    swapw
    loadw.local.119

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.119
    swapw
    storew.local.103

    loadw.local.104
    swapw
    loadw.local.120

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.120
    swapw
    storew.local.104

    loadw.local.105
    swapw
    loadw.local.121

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.121
    swapw
    storew.local.105

    loadw.local.106
    swapw
    loadw.local.122

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.122
    swapw
    storew.local.106

    loadw.local.107
    swapw
    loadw.local.123

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.123
    swapw
    storew.local.107

    loadw.local.108
    swapw
    loadw.local.124

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.124
    swapw
    storew.local.108

    loadw.local.109
    swapw
    loadw.local.125

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.125
    swapw
    storew.local.109

    loadw.local.110
    swapw
    loadw.local.126

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.126
    swapw
    storew.local.110

    loadw.local.111
    swapw
    loadw.local.127

    push.68719476736.68719476736.68719476736.68719476736

    exec.ibutterfly

    storew.local.127
    swapw
    storew.local.111

    # iter = 7

	loadw.local.0
    swapw
    loadw.local.32

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.32
    swapw
    storew.local.0

    loadw.local.1
    swapw
    loadw.local.33

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.33
    swapw
    storew.local.1

    loadw.local.2
    swapw
    loadw.local.34

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.34
    swapw
    storew.local.2

    loadw.local.3
    swapw
    loadw.local.35

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.35
    swapw
    storew.local.3

    loadw.local.4
    swapw
    loadw.local.36

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.36
    swapw
    storew.local.4

    loadw.local.5
    swapw
    loadw.local.37

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.37
    swapw
    storew.local.5

    loadw.local.6
    swapw
    loadw.local.38

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.38
    swapw
    storew.local.6

    loadw.local.7
    swapw
    loadw.local.39

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.39
    swapw
    storew.local.7

    loadw.local.8
    swapw
    loadw.local.40

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.40
    swapw
    storew.local.8

    loadw.local.9
    swapw
    loadw.local.41

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.41
    swapw
    storew.local.9

    loadw.local.10
    swapw
    loadw.local.42

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.42
    swapw
    storew.local.10

    loadw.local.11
    swapw
    loadw.local.43

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.43
    swapw
    storew.local.11

    loadw.local.12
    swapw
    loadw.local.44

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.44
    swapw
    storew.local.12

    loadw.local.13
    swapw
    loadw.local.45

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.45
    swapw
    storew.local.13

    loadw.local.14
    swapw
    loadw.local.46

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.46
    swapw
    storew.local.14

    loadw.local.15
    swapw
    loadw.local.47

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.47
    swapw
    storew.local.15

    loadw.local.16
    swapw
    loadw.local.48

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.48
    swapw
    storew.local.16

    loadw.local.17
    swapw
    loadw.local.49

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.49
    swapw
    storew.local.17

    loadw.local.18
    swapw
    loadw.local.50

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.50
    swapw
    storew.local.18

    loadw.local.19
    swapw
    loadw.local.51

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.51
    swapw
    storew.local.19

    loadw.local.20
    swapw
    loadw.local.52

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.52
    swapw
    storew.local.20

    loadw.local.21
    swapw
    loadw.local.53

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.53
    swapw
    storew.local.21

    loadw.local.22
    swapw
    loadw.local.54

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.54
    swapw
    storew.local.22

    loadw.local.23
    swapw
    loadw.local.55

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.55
    swapw
    storew.local.23

    loadw.local.24
    swapw
    loadw.local.56

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.56
    swapw
    storew.local.24

    loadw.local.25
    swapw
    loadw.local.57

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.57
    swapw
    storew.local.25

    loadw.local.26
    swapw
    loadw.local.58

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.58
    swapw
    storew.local.26

    loadw.local.27
    swapw
    loadw.local.59

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.59
    swapw
    storew.local.27

    loadw.local.28
    swapw
    loadw.local.60

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.60
    swapw
    storew.local.28

    loadw.local.29
    swapw
    loadw.local.61

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.61
    swapw
    storew.local.29

    loadw.local.30
    swapw
    loadw.local.62

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.62
    swapw
    storew.local.30

    loadw.local.31
    swapw
    loadw.local.63

    push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105

    exec.ibutterfly

    storew.local.63
    swapw
    storew.local.31

    # ---

    loadw.local.64
    swapw
    loadw.local.96

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.96
    swapw
    storew.local.64

    loadw.local.65
    swapw
    loadw.local.97

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.97
    swapw
    storew.local.65

    loadw.local.66
    swapw
    loadw.local.98

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.98
    swapw
    storew.local.66

    loadw.local.67
    swapw
    loadw.local.99

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.99
    swapw
    storew.local.67

    loadw.local.68
    swapw
    loadw.local.100

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.100
    swapw
    storew.local.68

    loadw.local.69
    swapw
    loadw.local.101

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.101
    swapw
    storew.local.69

    loadw.local.70
    swapw
    loadw.local.102

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.102
    swapw
    storew.local.70

    loadw.local.71
    swapw
    loadw.local.103

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.103
    swapw
    storew.local.71

    loadw.local.72
    swapw
    loadw.local.104

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.104
    swapw
    storew.local.72

    loadw.local.73
    swapw
    loadw.local.105

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.105
    swapw
    storew.local.73

    loadw.local.74
    swapw
    loadw.local.106

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.106
    swapw
    storew.local.74

    loadw.local.75
    swapw
    loadw.local.107

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.107
    swapw
    storew.local.75

    loadw.local.76
    swapw
    loadw.local.108

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.108
    swapw
    storew.local.76

    loadw.local.77
    swapw
    loadw.local.109

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.109
    swapw
    storew.local.77

    loadw.local.78
    swapw
    loadw.local.110

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.110
    swapw
    storew.local.78

    loadw.local.79
    swapw
    loadw.local.111

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.111
    swapw
    storew.local.79

    loadw.local.80
    swapw
    loadw.local.112

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.112
    swapw
    storew.local.80

    loadw.local.81
    swapw
    loadw.local.113

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.113
    swapw
    storew.local.81

    loadw.local.82
    swapw
    loadw.local.114

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.114
    swapw
    storew.local.82

    loadw.local.83
    swapw
    loadw.local.115

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.115
    swapw
    storew.local.83

    loadw.local.84
    swapw
    loadw.local.116

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.116
    swapw
    storew.local.84

    loadw.local.85
    swapw
    loadw.local.117

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.117
    swapw
    storew.local.85

    loadw.local.86
    swapw
    loadw.local.118

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.118
    swapw
    storew.local.86

    loadw.local.87
    swapw
    loadw.local.119

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.119
    swapw
    storew.local.87

    loadw.local.88
    swapw
    loadw.local.120

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.120
    swapw
    storew.local.88

    loadw.local.89
    swapw
    loadw.local.121

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.121
    swapw
    storew.local.89

    loadw.local.90
    swapw
    loadw.local.122

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.122
    swapw
    storew.local.90

    loadw.local.91
    swapw
    loadw.local.123

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.123
    swapw
    storew.local.91

    loadw.local.92
    swapw
    loadw.local.124

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.124
    swapw
    storew.local.92

    loadw.local.93
    swapw
    loadw.local.125

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.125
    swapw
    storew.local.93

    loadw.local.94
    swapw
    loadw.local.126

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.126
    swapw
    storew.local.94

    loadw.local.95
    swapw
    loadw.local.127

    push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801

    exec.ibutterfly

    storew.local.127
    swapw
    storew.local.95

    # iter = 8

	loadw.local.0
    swapw
    loadw.local.64

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.64
    swapw
    storew.local.0

    loadw.local.1
    swapw
    loadw.local.65

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.65
    swapw
    storew.local.1

    loadw.local.2
    swapw
    loadw.local.66

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.66
    swapw
    storew.local.2

    loadw.local.3
    swapw
    loadw.local.67

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.67
    swapw
    storew.local.3

    loadw.local.4
    swapw
    loadw.local.68

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.68
    swapw
    storew.local.4

    loadw.local.5
    swapw
    loadw.local.69

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.69
    swapw
    storew.local.5

    loadw.local.6
    swapw
    loadw.local.70

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.70
    swapw
    storew.local.6

    loadw.local.7
    swapw
    loadw.local.71

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.71
    swapw
    storew.local.7

    loadw.local.8
    swapw
    loadw.local.72

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.72
    swapw
    storew.local.8

    loadw.local.9
    swapw
    loadw.local.73

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.73
    swapw
    storew.local.9

    loadw.local.10
    swapw
    loadw.local.74

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.74
    swapw
    storew.local.10

    loadw.local.11
    swapw
    loadw.local.75

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.75
    swapw
    storew.local.11

    loadw.local.12
    swapw
    loadw.local.76

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.76
    swapw
    storew.local.12

    loadw.local.13
    swapw
    loadw.local.77

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.77
    swapw
    storew.local.13

    loadw.local.14
    swapw
    loadw.local.78

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.78
    swapw
    storew.local.14

    loadw.local.15
    swapw
    loadw.local.79

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.79
    swapw
    storew.local.15

    loadw.local.16
    swapw
    loadw.local.80

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.80
    swapw
    storew.local.16

    loadw.local.17
    swapw
    loadw.local.81

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.81
    swapw
    storew.local.17

    loadw.local.18
    swapw
    loadw.local.82

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.82
    swapw
    storew.local.18

    loadw.local.19
    swapw
    loadw.local.83

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.83
    swapw
    storew.local.19

    loadw.local.20
    swapw
    loadw.local.84

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.84
    swapw
    storew.local.20

    loadw.local.21
    swapw
    loadw.local.85

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.85
    swapw
    storew.local.21

    loadw.local.22
    swapw
    loadw.local.86

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.86
    swapw
    storew.local.22

    loadw.local.23
    swapw
    loadw.local.87

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.87
    swapw
    storew.local.23

    loadw.local.24
    swapw
    loadw.local.88

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.88
    swapw
    storew.local.24

    loadw.local.25
    swapw
    loadw.local.89

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.89
    swapw
    storew.local.25

    loadw.local.26
    swapw
    loadw.local.90

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.90
    swapw
    storew.local.26

    loadw.local.27
    swapw
    loadw.local.91

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.91
    swapw
    storew.local.27

    loadw.local.28
    swapw
    loadw.local.92

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.92
    swapw
    storew.local.28

    loadw.local.29
    swapw
    loadw.local.93

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.93
    swapw
    storew.local.29

    loadw.local.30
    swapw
    loadw.local.94

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.94
    swapw
    storew.local.30

    loadw.local.31
    swapw
    loadw.local.95

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.95
    swapw
    storew.local.31

    loadw.local.32
    swapw
    loadw.local.96

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.96
    swapw
    storew.local.32

    loadw.local.33
    swapw
    loadw.local.97

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.97
    swapw
    storew.local.33

    loadw.local.34
    swapw
    loadw.local.98

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.98
    swapw
    storew.local.34

    loadw.local.35
    swapw
    loadw.local.99

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.99
    swapw
    storew.local.35

    loadw.local.36
    swapw
    loadw.local.100

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.100
    swapw
    storew.local.36

    loadw.local.37
    swapw
    loadw.local.101

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.101
    swapw
    storew.local.37

    loadw.local.38
    swapw
    loadw.local.102

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.102
    swapw
    storew.local.38

    loadw.local.39
    swapw
    loadw.local.103

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.103
    swapw
    storew.local.39

    loadw.local.40
    swapw
    loadw.local.104

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.104
    swapw
    storew.local.40

    loadw.local.41
    swapw
    loadw.local.105

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.105
    swapw
    storew.local.41

    loadw.local.42
    swapw
    loadw.local.106

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.106
    swapw
    storew.local.42

    loadw.local.43
    swapw
    loadw.local.107

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.107
    swapw
    storew.local.43

    loadw.local.44
    swapw
    loadw.local.108

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.108
    swapw
    storew.local.44

    loadw.local.45
    swapw
    loadw.local.109

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.109
    swapw
    storew.local.45

    loadw.local.46
    swapw
    loadw.local.110

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.110
    swapw
    storew.local.46

    loadw.local.47
    swapw
    loadw.local.111

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.111
    swapw
    storew.local.47

    loadw.local.48
    swapw
    loadw.local.112

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.112
    swapw
    storew.local.48

    loadw.local.49
    swapw
    loadw.local.113

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.113
    swapw
    storew.local.49

    loadw.local.50
    swapw
    loadw.local.114

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.114
    swapw
    storew.local.50

    loadw.local.51
    swapw
    loadw.local.115

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.115
    swapw
    storew.local.51

    loadw.local.52
    swapw
    loadw.local.116

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.116
    swapw
    storew.local.52

    loadw.local.53
    swapw
    loadw.local.117

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.117
    swapw
    storew.local.53

    loadw.local.54
    swapw
    loadw.local.118

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.118
    swapw
    storew.local.54

    loadw.local.55
    swapw
    loadw.local.119

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.119
    swapw
    storew.local.55

    loadw.local.56
    swapw
    loadw.local.120

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.120
    swapw
    storew.local.56

    loadw.local.57
    swapw
    loadw.local.121

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.121
    swapw
    storew.local.57

    loadw.local.58
    swapw
    loadw.local.122

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.122
    swapw
    storew.local.58

    loadw.local.59
    swapw
    loadw.local.123

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.123
    swapw
    storew.local.59

    loadw.local.60
    swapw
    loadw.local.124

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.124
    swapw
    storew.local.60

    loadw.local.61
    swapw
    loadw.local.125

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.125
    swapw
    storew.local.61

    loadw.local.62
    swapw
    loadw.local.126

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.126
    swapw
    storew.local.62

    loadw.local.63
    swapw
    loadw.local.127

    push.281474976710656.281474976710656.281474976710656.281474976710656

    exec.ibutterfly

    storew.local.127
    swapw
    storew.local.63

	# multiply by inverse of N (= 512)

	dropw

    loadw.local.0
    exec.mul_by_invN
    storew.local.0

    loadw.local.1
    exec.mul_by_invN
    storew.local.1

    loadw.local.2
    exec.mul_by_invN
    storew.local.2

    loadw.local.3
    exec.mul_by_invN
    storew.local.3

    loadw.local.4
    exec.mul_by_invN
    storew.local.4

    loadw.local.5
    exec.mul_by_invN
    storew.local.5

    loadw.local.6
    exec.mul_by_invN
    storew.local.6

    loadw.local.7
    exec.mul_by_invN
    storew.local.7

    loadw.local.8
    exec.mul_by_invN
    storew.local.8

    loadw.local.9
    exec.mul_by_invN
    storew.local.9

    loadw.local.10
    exec.mul_by_invN
    storew.local.10

    loadw.local.11
    exec.mul_by_invN
    storew.local.11

    loadw.local.12
    exec.mul_by_invN
    storew.local.12

    loadw.local.13
    exec.mul_by_invN
    storew.local.13

    loadw.local.14
    exec.mul_by_invN
    storew.local.14

    loadw.local.15
    exec.mul_by_invN
    storew.local.15

    loadw.local.16
    exec.mul_by_invN
    storew.local.16

    loadw.local.17
    exec.mul_by_invN
    storew.local.17

    loadw.local.18
    exec.mul_by_invN
    storew.local.18

    loadw.local.19
    exec.mul_by_invN
    storew.local.19

    loadw.local.20
    exec.mul_by_invN
    storew.local.20

    loadw.local.21
    exec.mul_by_invN
    storew.local.21

    loadw.local.22
    exec.mul_by_invN
    storew.local.22

    loadw.local.23
    exec.mul_by_invN
    storew.local.23

    loadw.local.24
    exec.mul_by_invN
    storew.local.24

    loadw.local.25
    exec.mul_by_invN
    storew.local.25

    loadw.local.26
    exec.mul_by_invN
    storew.local.26

    loadw.local.27
    exec.mul_by_invN
    storew.local.27

    loadw.local.28
    exec.mul_by_invN
    storew.local.28

    loadw.local.29
    exec.mul_by_invN
    storew.local.29

    loadw.local.30
    exec.mul_by_invN
    storew.local.30

    loadw.local.31
    exec.mul_by_invN
    storew.local.31

    loadw.local.32
    exec.mul_by_invN
    storew.local.32

    loadw.local.33
    exec.mul_by_invN
    storew.local.33

    loadw.local.34
    exec.mul_by_invN
    storew.local.34

    loadw.local.35
    exec.mul_by_invN
    storew.local.35

    loadw.local.36
    exec.mul_by_invN
    storew.local.36

    loadw.local.37
    exec.mul_by_invN
    storew.local.37

    loadw.local.38
    exec.mul_by_invN
    storew.local.38

    loadw.local.39
    exec.mul_by_invN
    storew.local.39

    loadw.local.40
    exec.mul_by_invN
    storew.local.40

    loadw.local.41
    exec.mul_by_invN
    storew.local.41

    loadw.local.42
    exec.mul_by_invN
    storew.local.42

    loadw.local.43
    exec.mul_by_invN
    storew.local.43

    loadw.local.44
    exec.mul_by_invN
    storew.local.44

    loadw.local.45
    exec.mul_by_invN
    storew.local.45

    loadw.local.46
    exec.mul_by_invN
    storew.local.46

    loadw.local.47
    exec.mul_by_invN
    storew.local.47

    loadw.local.48
    exec.mul_by_invN
    storew.local.48

    loadw.local.49
    exec.mul_by_invN
    storew.local.49

    loadw.local.50
    exec.mul_by_invN
    storew.local.50

    loadw.local.51
    exec.mul_by_invN
    storew.local.51

    loadw.local.52
    exec.mul_by_invN
    storew.local.52

    loadw.local.53
    exec.mul_by_invN
    storew.local.53

    loadw.local.54
    exec.mul_by_invN
    storew.local.54

    loadw.local.55
    exec.mul_by_invN
    storew.local.55

    loadw.local.56
    exec.mul_by_invN
    storew.local.56

    loadw.local.57
    exec.mul_by_invN
    storew.local.57

    loadw.local.58
    exec.mul_by_invN
    storew.local.58

    loadw.local.59
    exec.mul_by_invN
    storew.local.59

    loadw.local.60
    exec.mul_by_invN
    storew.local.60

    loadw.local.61
    exec.mul_by_invN
    storew.local.61

    loadw.local.62
    exec.mul_by_invN
    storew.local.62

    loadw.local.63
    exec.mul_by_invN
    storew.local.63

    loadw.local.64
    exec.mul_by_invN
    storew.local.64

    loadw.local.65
    exec.mul_by_invN
    storew.local.65

    loadw.local.66
    exec.mul_by_invN
    storew.local.66

    loadw.local.67
    exec.mul_by_invN
    storew.local.67

    loadw.local.68
    exec.mul_by_invN
    storew.local.68

    loadw.local.69
    exec.mul_by_invN
    storew.local.69

    loadw.local.70
    exec.mul_by_invN
    storew.local.70

    loadw.local.71
    exec.mul_by_invN
    storew.local.71

    loadw.local.72
    exec.mul_by_invN
    storew.local.72

    loadw.local.73
    exec.mul_by_invN
    storew.local.73

    loadw.local.74
    exec.mul_by_invN
    storew.local.74

    loadw.local.75
    exec.mul_by_invN
    storew.local.75

    loadw.local.76
    exec.mul_by_invN
    storew.local.76

    loadw.local.77
    exec.mul_by_invN
    storew.local.77

    loadw.local.78
    exec.mul_by_invN
    storew.local.78

    loadw.local.79
    exec.mul_by_invN
    storew.local.79

    loadw.local.80
    exec.mul_by_invN
    storew.local.80

    loadw.local.81
    exec.mul_by_invN
    storew.local.81

    loadw.local.82
    exec.mul_by_invN
    storew.local.82

    loadw.local.83
    exec.mul_by_invN
    storew.local.83

    loadw.local.84
    exec.mul_by_invN
    storew.local.84

    loadw.local.85
    exec.mul_by_invN
    storew.local.85

    loadw.local.86
    exec.mul_by_invN
    storew.local.86

    loadw.local.87
    exec.mul_by_invN
    storew.local.87

    loadw.local.88
    exec.mul_by_invN
    storew.local.88

    loadw.local.89
    exec.mul_by_invN
    storew.local.89

    loadw.local.90
    exec.mul_by_invN
    storew.local.90

    loadw.local.91
    exec.mul_by_invN
    storew.local.91

    loadw.local.92
    exec.mul_by_invN
    storew.local.92

    loadw.local.93
    exec.mul_by_invN
    storew.local.93

    loadw.local.94
    exec.mul_by_invN
    storew.local.94

    loadw.local.95
    exec.mul_by_invN
    storew.local.95

    loadw.local.96
    exec.mul_by_invN
    storew.local.96

    loadw.local.97
    exec.mul_by_invN
    storew.local.97

    loadw.local.98
    exec.mul_by_invN
    storew.local.98

    loadw.local.99
    exec.mul_by_invN
    storew.local.99

    loadw.local.100
    exec.mul_by_invN
    storew.local.100

    loadw.local.101
    exec.mul_by_invN
    storew.local.101

    loadw.local.102
    exec.mul_by_invN
    storew.local.102

    loadw.local.103
    exec.mul_by_invN
    storew.local.103

    loadw.local.104
    exec.mul_by_invN
    storew.local.104

    loadw.local.105
    exec.mul_by_invN
    storew.local.105

    loadw.local.106
    exec.mul_by_invN
    storew.local.106

    loadw.local.107
    exec.mul_by_invN
    storew.local.107

    loadw.local.108
    exec.mul_by_invN
    storew.local.108

    loadw.local.109
    exec.mul_by_invN
    storew.local.109

    loadw.local.110
    exec.mul_by_invN
    storew.local.110

    loadw.local.111
    exec.mul_by_invN
    storew.local.111

    loadw.local.112
    exec.mul_by_invN
    storew.local.112

    loadw.local.113
    exec.mul_by_invN
    storew.local.113

    loadw.local.114
    exec.mul_by_invN
    storew.local.114

    loadw.local.115
    exec.mul_by_invN
    storew.local.115

    loadw.local.116
    exec.mul_by_invN
    storew.local.116

    loadw.local.117
    exec.mul_by_invN
    storew.local.117

    loadw.local.118
    exec.mul_by_invN
    storew.local.118

    loadw.local.119
    exec.mul_by_invN
    storew.local.119

    loadw.local.120
    exec.mul_by_invN
    storew.local.120

    loadw.local.121
    exec.mul_by_invN
    storew.local.121

    loadw.local.122
    exec.mul_by_invN
    storew.local.122

    loadw.local.123
    exec.mul_by_invN
    storew.local.123

    loadw.local.124
    exec.mul_by_invN
    storew.local.124

    loadw.local.125
    exec.mul_by_invN
    storew.local.125

    loadw.local.126
    exec.mul_by_invN
    storew.local.126

    loadw.local.127
    exec.mul_by_invN
    storew.local.127

	dropw

	# begin asserting result

    pushw.local.0

    push.0
    assert.eq
    push.1
    assert.eq
    push.2
    assert.eq
    push.3
    assert.eq

    pushw.local.1

    push.4
    assert.eq
    push.5
    assert.eq
    push.6
    assert.eq
    push.7
    assert.eq

    pushw.local.2

    push.8
    assert.eq
    push.9
    assert.eq
    push.10
    assert.eq
    push.11
    assert.eq

    pushw.local.3

    push.12
    assert.eq
    push.13
    assert.eq
    push.14
    assert.eq
    push.15
    assert.eq

    pushw.local.4

    push.16
    assert.eq
    push.17
    assert.eq
    push.18
    assert.eq
    push.19
    assert.eq

    pushw.local.5

    push.20
    assert.eq
    push.21
    assert.eq
    push.22
    assert.eq
    push.23
    assert.eq

    pushw.local.6

    push.24
    assert.eq
    push.25
    assert.eq
    push.26
    assert.eq
    push.27
    assert.eq

    pushw.local.7

    push.28
    assert.eq
    push.29
    assert.eq
    push.30
    assert.eq
    push.31
    assert.eq

    pushw.local.8

    push.32
    assert.eq
    push.33
    assert.eq
    push.34
    assert.eq
    push.35
    assert.eq

    pushw.local.9

    push.36
    assert.eq
    push.37
    assert.eq
    push.38
    assert.eq
    push.39
    assert.eq

    pushw.local.10

    push.40
    assert.eq
    push.41
    assert.eq
    push.42
    assert.eq
    push.43
    assert.eq

    pushw.local.11

    push.44
    assert.eq
    push.45
    assert.eq
    push.46
    assert.eq
    push.47
    assert.eq

    pushw.local.12

    push.48
    assert.eq
    push.49
    assert.eq
    push.50
    assert.eq
    push.51
    assert.eq

    pushw.local.13

    push.52
    assert.eq
    push.53
    assert.eq
    push.54
    assert.eq
    push.55
    assert.eq

    pushw.local.14

    push.56
    assert.eq
    push.57
    assert.eq
    push.58
    assert.eq
    push.59
    assert.eq

    pushw.local.15

    push.60
    assert.eq
    push.61
    assert.eq
    push.62
    assert.eq
    push.63
    assert.eq

    pushw.local.16

    push.64
    assert.eq
    push.65
    assert.eq
    push.66
    assert.eq
    push.67
    assert.eq

    pushw.local.17

    push.68
    assert.eq
    push.69
    assert.eq
    push.70
    assert.eq
    push.71
    assert.eq

    pushw.local.18

    push.72
    assert.eq
    push.73
    assert.eq
    push.74
    assert.eq
    push.75
    assert.eq

    pushw.local.19

    push.76
    assert.eq
    push.77
    assert.eq
    push.78
    assert.eq
    push.79
    assert.eq

    pushw.local.20

    push.80
    assert.eq
    push.81
    assert.eq
    push.82
    assert.eq
    push.83
    assert.eq

    pushw.local.21

    push.84
    assert.eq
    push.85
    assert.eq
    push.86
    assert.eq
    push.87
    assert.eq

    pushw.local.22

    push.88
    assert.eq
    push.89
    assert.eq
    push.90
    assert.eq
    push.91
    assert.eq

    pushw.local.23

    push.92
    assert.eq
    push.93
    assert.eq
    push.94
    assert.eq
    push.95
    assert.eq

    pushw.local.24

    push.96
    assert.eq
    push.97
    assert.eq
    push.98
    assert.eq
    push.99
    assert.eq

    pushw.local.25

    push.100
    assert.eq
    push.101
    assert.eq
    push.102
    assert.eq
    push.103
    assert.eq

    pushw.local.26

    push.104
    assert.eq
    push.105
    assert.eq
    push.106
    assert.eq
    push.107
    assert.eq

    pushw.local.27

    push.108
    assert.eq
    push.109
    assert.eq
    push.110
    assert.eq
    push.111
    assert.eq

    pushw.local.28

    push.112
    assert.eq
    push.113
    assert.eq
    push.114
    assert.eq
    push.115
    assert.eq

    pushw.local.29

    push.116
    assert.eq
    push.117
    assert.eq
    push.118
    assert.eq
    push.119
    assert.eq

    pushw.local.30

    push.120
    assert.eq
    push.121
    assert.eq
    push.122
    assert.eq
    push.123
    assert.eq

    pushw.local.31

    push.124
    assert.eq
    push.125
    assert.eq
    push.126
    assert.eq
    push.127
    assert.eq

    pushw.local.32

    push.128
    assert.eq
    push.129
    assert.eq
    push.130
    assert.eq
    push.131
    assert.eq

    pushw.local.33

    push.132
    assert.eq
    push.133
    assert.eq
    push.134
    assert.eq
    push.135
    assert.eq

    pushw.local.34

    push.136
    assert.eq
    push.137
    assert.eq
    push.138
    assert.eq
    push.139
    assert.eq

    pushw.local.35

    push.140
    assert.eq
    push.141
    assert.eq
    push.142
    assert.eq
    push.143
    assert.eq

    pushw.local.36

    push.144
    assert.eq
    push.145
    assert.eq
    push.146
    assert.eq
    push.147
    assert.eq

    pushw.local.37

    push.148
    assert.eq
    push.149
    assert.eq
    push.150
    assert.eq
    push.151
    assert.eq

    pushw.local.38

    push.152
    assert.eq
    push.153
    assert.eq
    push.154
    assert.eq
    push.155
    assert.eq

    pushw.local.39

    push.156
    assert.eq
    push.157
    assert.eq
    push.158
    assert.eq
    push.159
    assert.eq

    pushw.local.40

    push.160
    assert.eq
    push.161
    assert.eq
    push.162
    assert.eq
    push.163
    assert.eq

    pushw.local.41

    push.164
    assert.eq
    push.165
    assert.eq
    push.166
    assert.eq
    push.167
    assert.eq

    pushw.local.42

    push.168
    assert.eq
    push.169
    assert.eq
    push.170
    assert.eq
    push.171
    assert.eq

    pushw.local.43

    push.172
    assert.eq
    push.173
    assert.eq
    push.174
    assert.eq
    push.175
    assert.eq

    pushw.local.44

    push.176
    assert.eq
    push.177
    assert.eq
    push.178
    assert.eq
    push.179
    assert.eq

    pushw.local.45

    push.180
    assert.eq
    push.181
    assert.eq
    push.182
    assert.eq
    push.183
    assert.eq

    pushw.local.46

    push.184
    assert.eq
    push.185
    assert.eq
    push.186
    assert.eq
    push.187
    assert.eq

    pushw.local.47

    push.188
    assert.eq
    push.189
    assert.eq
    push.190
    assert.eq
    push.191
    assert.eq

    pushw.local.48

    push.192
    assert.eq
    push.193
    assert.eq
    push.194
    assert.eq
    push.195
    assert.eq

    pushw.local.49

    push.196
    assert.eq
    push.197
    assert.eq
    push.198
    assert.eq
    push.199
    assert.eq

    pushw.local.50

    push.200
    assert.eq
    push.201
    assert.eq
    push.202
    assert.eq
    push.203
    assert.eq

    pushw.local.51

    push.204
    assert.eq
    push.205
    assert.eq
    push.206
    assert.eq
    push.207
    assert.eq

    pushw.local.52

    push.208
    assert.eq
    push.209
    assert.eq
    push.210
    assert.eq
    push.211
    assert.eq

    pushw.local.53

    push.212
    assert.eq
    push.213
    assert.eq
    push.214
    assert.eq
    push.215
    assert.eq

    pushw.local.54

    push.216
    assert.eq
    push.217
    assert.eq
    push.218
    assert.eq
    push.219
    assert.eq

    pushw.local.55

    push.220
    assert.eq
    push.221
    assert.eq
    push.222
    assert.eq
    push.223
    assert.eq

    pushw.local.56

    push.224
    assert.eq
    push.225
    assert.eq
    push.226
    assert.eq
    push.227
    assert.eq

    pushw.local.57

    push.228
    assert.eq
    push.229
    assert.eq
    push.230
    assert.eq
    push.231
    assert.eq

    pushw.local.58

    push.232
    assert.eq
    push.233
    assert.eq
    push.234
    assert.eq
    push.235
    assert.eq

    pushw.local.59

    push.236
    assert.eq
    push.237
    assert.eq
    push.238
    assert.eq
    push.239
    assert.eq

    pushw.local.60

    push.240
    assert.eq
    push.241
    assert.eq
    push.242
    assert.eq
    push.243
    assert.eq

    pushw.local.61

    push.244
    assert.eq
    push.245
    assert.eq
    push.246
    assert.eq
    push.247
    assert.eq

    pushw.local.62

    push.248
    assert.eq
    push.249
    assert.eq
    push.250
    assert.eq
    push.251
    assert.eq

    pushw.local.63

    push.252
    assert.eq
    push.253
    assert.eq
    push.254
    assert.eq
    push.255
    assert.eq

    pushw.local.64

    push.256
    assert.eq
    push.257
    assert.eq
    push.258
    assert.eq
    push.259
    assert.eq

    pushw.local.65

    push.260
    assert.eq
    push.261
    assert.eq
    push.262
    assert.eq
    push.263
    assert.eq

    pushw.local.66

    push.264
    assert.eq
    push.265
    assert.eq
    push.266
    assert.eq
    push.267
    assert.eq

    pushw.local.67

    push.268
    assert.eq
    push.269
    assert.eq
    push.270
    assert.eq
    push.271
    assert.eq

    pushw.local.68

    push.272
    assert.eq
    push.273
    assert.eq
    push.274
    assert.eq
    push.275
    assert.eq

    pushw.local.69

    push.276
    assert.eq
    push.277
    assert.eq
    push.278
    assert.eq
    push.279
    assert.eq

    pushw.local.70

    push.280
    assert.eq
    push.281
    assert.eq
    push.282
    assert.eq
    push.283
    assert.eq

    pushw.local.71

    push.284
    assert.eq
    push.285
    assert.eq
    push.286
    assert.eq
    push.287
    assert.eq

    pushw.local.72

    push.288
    assert.eq
    push.289
    assert.eq
    push.290
    assert.eq
    push.291
    assert.eq

    pushw.local.73

    push.292
    assert.eq
    push.293
    assert.eq
    push.294
    assert.eq
    push.295
    assert.eq

    pushw.local.74

    push.296
    assert.eq
    push.297
    assert.eq
    push.298
    assert.eq
    push.299
    assert.eq

    pushw.local.75

    push.300
    assert.eq
    push.301
    assert.eq
    push.302
    assert.eq
    push.303
    assert.eq

    pushw.local.76

    push.304
    assert.eq
    push.305
    assert.eq
    push.306
    assert.eq
    push.307
    assert.eq

    pushw.local.77

    push.308
    assert.eq
    push.309
    assert.eq
    push.310
    assert.eq
    push.311
    assert.eq

    pushw.local.78

    push.312
    assert.eq
    push.313
    assert.eq
    push.314
    assert.eq
    push.315
    assert.eq

    pushw.local.79

    push.316
    assert.eq
    push.317
    assert.eq
    push.318
    assert.eq
    push.319
    assert.eq

    pushw.local.80

    push.320
    assert.eq
    push.321
    assert.eq
    push.322
    assert.eq
    push.323
    assert.eq

    pushw.local.81

    push.324
    assert.eq
    push.325
    assert.eq
    push.326
    assert.eq
    push.327
    assert.eq

    pushw.local.82

    push.328
    assert.eq
    push.329
    assert.eq
    push.330
    assert.eq
    push.331
    assert.eq

    pushw.local.83

    push.332
    assert.eq
    push.333
    assert.eq
    push.334
    assert.eq
    push.335
    assert.eq

    pushw.local.84

    push.336
    assert.eq
    push.337
    assert.eq
    push.338
    assert.eq
    push.339
    assert.eq

    pushw.local.85

    push.340
    assert.eq
    push.341
    assert.eq
    push.342
    assert.eq
    push.343
    assert.eq

    pushw.local.86

    push.344
    assert.eq
    push.345
    assert.eq
    push.346
    assert.eq
    push.347
    assert.eq

    pushw.local.87

    push.348
    assert.eq
    push.349
    assert.eq
    push.350
    assert.eq
    push.351
    assert.eq

    pushw.local.88

    push.352
    assert.eq
    push.353
    assert.eq
    push.354
    assert.eq
    push.355
    assert.eq

    pushw.local.89

    push.356
    assert.eq
    push.357
    assert.eq
    push.358
    assert.eq
    push.359
    assert.eq

    pushw.local.90

    push.360
    assert.eq
    push.361
    assert.eq
    push.362
    assert.eq
    push.363
    assert.eq

    pushw.local.91

    push.364
    assert.eq
    push.365
    assert.eq
    push.366
    assert.eq
    push.367
    assert.eq

    pushw.local.92

    push.368
    assert.eq
    push.369
    assert.eq
    push.370
    assert.eq
    push.371
    assert.eq

    pushw.local.93

    push.372
    assert.eq
    push.373
    assert.eq
    push.374
    assert.eq
    push.375
    assert.eq

    pushw.local.94

    push.376
    assert.eq
    push.377
    assert.eq
    push.378
    assert.eq
    push.379
    assert.eq

    pushw.local.95

    push.380
    assert.eq
    push.381
    assert.eq
    push.382
    assert.eq
    push.383
    assert.eq

    pushw.local.96

    push.384
    assert.eq
    push.385
    assert.eq
    push.386
    assert.eq
    push.387
    assert.eq

    pushw.local.97

    push.388
    assert.eq
    push.389
    assert.eq
    push.390
    assert.eq
    push.391
    assert.eq

    pushw.local.98

    push.392
    assert.eq
    push.393
    assert.eq
    push.394
    assert.eq
    push.395
    assert.eq

    pushw.local.99

    push.396
    assert.eq
    push.397
    assert.eq
    push.398
    assert.eq
    push.399
    assert.eq

    pushw.local.100

    push.400
    assert.eq
    push.401
    assert.eq
    push.402
    assert.eq
    push.403
    assert.eq

    pushw.local.101

    push.404
    assert.eq
    push.405
    assert.eq
    push.406
    assert.eq
    push.407
    assert.eq

    pushw.local.102

    push.408
    assert.eq
    push.409
    assert.eq
    push.410
    assert.eq
    push.411
    assert.eq

    pushw.local.103

    push.412
    assert.eq
    push.413
    assert.eq
    push.414
    assert.eq
    push.415
    assert.eq

    pushw.local.104

    push.416
    assert.eq
    push.417
    assert.eq
    push.418
    assert.eq
    push.419
    assert.eq

    pushw.local.105

    push.420
    assert.eq
    push.421
    assert.eq
    push.422
    assert.eq
    push.423
    assert.eq

    pushw.local.106

    push.424
    assert.eq
    push.425
    assert.eq
    push.426
    assert.eq
    push.427
    assert.eq

    pushw.local.107

    push.428
    assert.eq
    push.429
    assert.eq
    push.430
    assert.eq
    push.431
    assert.eq

    pushw.local.108

    push.432
    assert.eq
    push.433
    assert.eq
    push.434
    assert.eq
    push.435
    assert.eq

    pushw.local.109

    push.436
    assert.eq
    push.437
    assert.eq
    push.438
    assert.eq
    push.439
    assert.eq

    pushw.local.110

    push.440
    assert.eq
    push.441
    assert.eq
    push.442
    assert.eq
    push.443
    assert.eq

    pushw.local.111

    push.444
    assert.eq
    push.445
    assert.eq
    push.446
    assert.eq
    push.447
    assert.eq

    pushw.local.112

    push.448
    assert.eq
    push.449
    assert.eq
    push.450
    assert.eq
    push.451
    assert.eq

    pushw.local.113

    push.452
    assert.eq
    push.453
    assert.eq
    push.454
    assert.eq
    push.455
    assert.eq

    pushw.local.114

    push.456
    assert.eq
    push.457
    assert.eq
    push.458
    assert.eq
    push.459
    assert.eq

    pushw.local.115

    push.460
    assert.eq
    push.461
    assert.eq
    push.462
    assert.eq
    push.463
    assert.eq

    pushw.local.116

    push.464
    assert.eq
    push.465
    assert.eq
    push.466
    assert.eq
    push.467
    assert.eq

    pushw.local.117

    push.468
    assert.eq
    push.469
    assert.eq
    push.470
    assert.eq
    push.471
    assert.eq

    pushw.local.118

    push.472
    assert.eq
    push.473
    assert.eq
    push.474
    assert.eq
    push.475
    assert.eq

    pushw.local.119

    push.476
    assert.eq
    push.477
    assert.eq
    push.478
    assert.eq
    push.479
    assert.eq

    pushw.local.120

    push.480
    assert.eq
    push.481
    assert.eq
    push.482
    assert.eq
    push.483
    assert.eq

    pushw.local.121

    push.484
    assert.eq
    push.485
    assert.eq
    push.486
    assert.eq
    push.487
    assert.eq

    pushw.local.122

    push.488
    assert.eq
    push.489
    assert.eq
    push.490
    assert.eq
    push.491
    assert.eq

    pushw.local.123

    push.492
    assert.eq
    push.493
    assert.eq
    push.494
    assert.eq
    push.495
    assert.eq

    pushw.local.124

    push.496
    assert.eq
    push.497
    assert.eq
    push.498
    assert.eq
    push.499
    assert.eq

    pushw.local.125

    push.500
    assert.eq
    push.501
    assert.eq
    push.502
    assert.eq
    push.503
    assert.eq

    pushw.local.126

    push.504
    assert.eq
    push.505
    assert.eq
    push.506
    assert.eq
    push.507
    assert.eq

    pushw.local.127

    push.508
    assert.eq
    push.509
    assert.eq
    push.510
    assert.eq
    push.511
    assert.eq

	# end asserting result
end
"),
// ----- std::math::secp256k1 ---------------------------------------------------------------------
("std::math::secp256k1", "# Given [b, c, a, carry] on stack top, following function computes
#
#  tmp = a + (b * c) + carry
#  hi = tmp >> 32
#  lo = tmp & 0xffff_ffff
#  return (hi, lo)
#
# At end of execution of this function, stack top should look like [hi, lo]
# See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/utils.py#L75-L80
proc.mac
  u32overflowing_madd

  movdn.2
  u32overflowing_add

  movup.2
  add
end

# Given [a, b, borrow] on stack top, following function computes
#
#  tmp = a - (b + borrow)
#  hi = tmp >> 32
#  lo = tmp & 0xffff_ffff
#  return (hi, lo)
#
# At end of execution of this function, stack top should look like [hi, lo]
# See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/utils.py#L83-L89
proc.sbb
  movdn.2
  add
  u32overflowing_sub
end

# Given a secp256k1 field element in radix-2^32 representation and 32 -bit unsigned integer,
# this routine computes a 288 -bit number.
#
# Input via stack is expected in this form
#
# [a0, a1, a2, a3, a4, a5, a6, a7, b] | a[0..8] -> 256 -bit number, b = 32 -bit number
#
# Computed output looks like below, on stack
#
# [carry, b7, b6, b5, b4, b3, b2, b1, b0]
proc.u256xu32
  movup.8
  
  push.0
  dup.1
  movup.3
  u32overflowing_madd
  
  dup.2
  movup.4
  u32overflowing_madd

  dup.3
  movup.5
  u32overflowing_madd

  dup.4
  movup.6
  u32overflowing_madd

  dup.5
  movup.7
  u32overflowing_madd

  dup.6
  movup.8
  u32overflowing_madd

  dup.7
  movup.9
  u32overflowing_madd

  movup.8
  movup.9
  u32overflowing_madd
end

# Given a 288 -bit number and 256 -bit number on stack ( in order ), this routine
# computes a 288 -bit number
#
# Expected stack state during routine invocation
#
# [carry, b7, b6, b5, b4, b3, b2, b1, b0, c0, c1, c2, c3, c4, c5, c6, c7]
#
# While after execution of this routine, stack should look like
#
# [d0, d1, d2, d3, d4, d5, d6, d7, carry]
proc.u288_add_u256
  swapw
  movupw.2

  u32overflowing_add

  movup.2
  movup.7
  u32overflowing_add3

  movup.3
  movup.6
  u32overflowing_add3

  movup.4
  movup.5
  movupw.2

  movup.2
  movup.4
  movup.6
  u32overflowing_add3

  movup.5
  movup.5
  u32overflowing_add3

  movup.3
  movup.4
  movupw.2

  movup.2
  movup.4
  movup.6
  u32overflowing_add3

  movup.5
  movup.5
  u32overflowing_add3

  movup.10
  movup.5
  u32overflowing_add3

  movup.4
  add

  swap
  movup.2
  movup.3
  movup.4
  movup.5
  movup.6
  movup.7
  movup.8
end

# Given [c0, c1, c2, c3, c4, c5, c6, c7, c8, pc] on stack top,
# this function attempts to reduce 288 -bit number to 256 -bit number
# along with carry, using montgomery reduction method
#
# In stack top content c[0..9] i.e. first 9 elements, holding 288 -bit
# number. Stack element `pc` ( at stack[9] ) is previous reduction round's
# carry ( for first reduction round, it'll be set to 0 ).
#
# After finishing execution of this function, stack top should look like
#
# [c0, c1, c2, c3, c4, c5, c6, c7, pc] | pc = next round's carry
proc.u288_reduce
  dup
  push.3525653809
  u32wrapping_mul 
  # q at stack top #

  push.0
  movup.2
  push.4294966319
  dup.3
  exec.mac

  swap
  drop

  movup.2
  push.4294967294
  dup.3
  exec.mac

  movup.3
  push.4294967295
  dup.4
  exec.mac

  movup.4
  push.4294967295
  dup.5
  exec.mac

  movup.5
  push.4294967295
  dup.6
  exec.mac

  movup.6
  push.4294967295
  dup.7
  exec.mac

  movup.7
  dup.7
  push.4294967295
  exec.mac

  movup.7
  movup.8
  swap
  push.4294967295
  exec.mac

  movup.9
  movup.9
  u32overflowing_add3

  swap
  movup.2
  movup.3
  movup.4
  movup.5
  movup.6
  movup.7
  movup.8
end

# Given two 256 -bit numbers on stack, where each number is represented in
# radix-2^32 form ( i.e. each number having eight 32 -bit limbs ), following function
# computes modular multiplication of those two operands, computing 256 -bit result.
#
# Stack expected as below, holding input
#
# [a0, a1, a2, a3, a4, a5, a6, a7, b0, b1, b2, b3, b4, b5, b6, b7] | a[0..8], b[0..8] are 256 -bit numbers
#
# After finishing execution of this function, stack should look like
#
# [c0, c1, c2, c3, c4, c5, c6, c7] | c[0..8] is a 256 -bit number
#
# Note, for computing modular multiplication of a[0..8] & b[0..8],
# school book multiplication equipped with montgomery reduction technique
# is used, which is why a[0..8], b[0..8] are expected to be in montgomery form,
# while computed c[0..8] will also be in montgomery form.
export.u256_mod_mul.2
  storew.local.0
  swapw
  storew.local.1
  swapw

  exec.u256xu32

  swap
  movup.2
  movup.3
  movup.4
  movup.5
  movup.6
  movup.7
  movup.8

  push.0
  movdn.9

  exec.u288_reduce

  movup.9
  pushw.local.1
  pushw.local.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  pushw.local.1
  pushw.local.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  pushw.local.1
  pushw.local.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  pushw.local.1
  pushw.local.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  pushw.local.1
  pushw.local.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  pushw.local.1
  pushw.local.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  pushw.local.1
  pushw.local.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.8
  movup.2
  dup.1
  add

  movup.2
  movup.2
  push.977

  u32overflowing_madd
  drop
end

# Given two 256 -bit numbers on stack, where each number is represented in
# radix-2^32 form ( i.e. each number having eight 32 -bit limbs ), following function
# computes modular addition of those two operands, in secp256k1 prime field.
#
# Stack expected as below, holding input
#
# [a0, a1, a2, a3, a4, a5, a6, a7, b0, b1, b2, b3, b4, b5, b6, b7] | a[0..8], b[0..8] are 256 -bit numbers
#
# After finishing execution of this function, stack should look like
#
# [c0, c1, c2, c3, c4, c5, c6, c7] | c[0..8] is a 256 -bit number
#
# This implementation takes inspiration from https://gist.github.com/itzmeanjan/d4853347dfdfa853993f5ea059824de6#file-test_montgomery_arithmetic-py-L236-L256
export.u256_mod_add
  movupw.2

  push.0
  movup.5
  u32overflowing_add3

  movup.2
  movup.5
  u32overflowing_add3

  movup.3
  movup.5
  u32overflowing_add3

  movup.4
  movup.5
  u32overflowing_add3

  movup.5
  movup.9
  u32overflowing_add3

  movup.6
  movup.9
  u32overflowing_add3

  movup.7
  movup.9
  u32overflowing_add3

  movup.8
  movup.9
  u32overflowing_add3

  movup.8
  dup.1
  push.977
  u32overflowing_madd
  drop

  swap
  movup.8
  add

  movup.2
  movup.3
  movup.4
  movup.5
  movup.6
  movup.7
  movup.6
  movup.7
end

# Given a secp256k1 field element ( say `a` ) on stack, represented in Montgomery form 
# ( i.e. number having eight 32 -bit limbs ), following function negates it to
# field element `a'` | a' + a = 0
#
# Stack expected as below, holding input
#
# [a0, a1, a2, a3, a4, a5, a6, a7] | a[0..8] is a secp256k1 field element
#
# After finishing execution of this function, stack should look like
#
# [c0, c1, c2, c3, c4, c5, c6, c7] | c[0..8] is a secp256k1 field element
#
# See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/field.py#L77-L95
export.u256_mod_neg
  push.0
  swap
  push.4294966319
  exec.sbb

  movup.2
  push.4294967294
  exec.sbb

  movup.3
  push.4294967295
  exec.sbb

  movup.4
  push.4294967295
  exec.sbb

  movup.5
  push.4294967295
  exec.sbb

  movup.6
  push.4294967295
  exec.sbb

  movup.7
  push.4294967295
  exec.sbb

  movup.8
  push.4294967295
  exec.sbb

  drop
  
  swap
  movup.2
  movup.3
  movup.4
  movup.5
  movup.6
  movup.7
end

# Given two secp256k1 field elements, say a, b, ( represented in Montgomery form, each number having 
# eight 32 -bit limbs ) on stack, following function computes modular subtraction of those 
# two operands c = a + (-b) = a - b
#
# Stack expected as below, holding input
#
# [a0, a1, a2, a3, a4, a5, a6, a7, b0, b1, b2, b3, b4, b5, b6, b7] | a[0..8], b[0..8] are secp256k1 field elements
#
# After finishing execution of this function, stack should look like
#
# [c0, c1, c2, c3, c4, c5, c6, c7] | c[0..8] is a secp256k1 field element
#
# See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/field.py#L97-L101
export.u256_mod_sub
  movupw.3
  movupw.3

  exec.u256_mod_neg
  exec.u256_mod_add
end

# Given a 256 -bit number on stack, represented in radix-2^32 
# form i.e. eight 32 -bit limbs, this routine computes Montgomery
# representation of provided radix-2^32 number.
#
# - u256 radix-2^32 form input expected on stack as
#
#  [a0, a1, a2, a3, a4, a5, a6, a7]
#
# - u256 montgomery form output on stack
#
# [a0`, a1`, a2`, a3`, a4`, a5`, a6`, a7`]
#
# See section 2.2 of https://eprint.iacr.org/2017/1057.pdf
export.to_mont
  push.0.0.0.0
  push.0.1.1954.954529 # pushed R2's radix-2^32 form;
                       # see https://gist.github.com/itzmeanjan/d4853347dfdfa853993f5ea059824de6

  exec.u256_mod_mul
end

# Given a 256 -bit number on stack, represented in Montgomery 
# form i.e. eight 32 -bit limbs, this routine computes radix-2^32
# representation of provided u256 number.
#
# - u256 montgomery form input on stack expected
#
#  [a0, a1, a2, a3, a4, a5, a6, a7]
#
# - u256 radix-2^32 form output on stack as
#
# [a0`, a1`, a2`, a3`, a4`, a5`, a6`, a7`]
#
# See section 2.2 of https://eprint.iacr.org/2017/1057.pdf
export.from_mont
  push.0.0.0.0
  push.0.0.0.1 # pushed 1's radix-2^32 form;
               # see https://gist.github.com/itzmeanjan/d4853347dfdfa853993f5ea059824de6

  exec.u256_mod_mul
end

# Given a secp256k1 point in projective coordinate system ( i.e. with x, y, z -coordinates
# as secp256k1 prime field elements, represented in Montgomery form ), this routine adds 
# that point with self i.e. does point doubling on elliptic curve, using exception-free 
# doubling formula from algorithm 9 of https://eprint.iacr.org/2015/1060.pdf, while 
# following prototype implementation https://github.com/itzmeanjan/secp256k1/blob/ec3652a/point.py#L131-L165
# 
# Input:
#
# 12 memory addresses on stack such that first 6 memory addresses are for input point &
# last 6 are for storing resulting point.
#
# First 6 addresses hold input elliptic curve point's x, y, z -coordinates, where each coordinate
# is represented in Montgomery form, as eight 32 -bit limbs.
#
# Similarly, last 6 addresses hold resulting (doubled) point's x, y, z -coordinates, where each
# coordinate is represented in Montgomery form, as eight 32 -bit limbs. Note, this is where
# output will be written, so called is expected to read doubled point from last 6 memory addresses.
#
# Expected stack during invocation of this routine:
#
#   [x_addr[0..4], x_addr[4..8], y_addr[0..4], y_addr[4..8], z_addr[0..4], z_addr[4..8], 
#     x3_addr[0..4], x3_addr[4..8], y3_addr[0..4], y3_addr[4..8], z3_addr[0..4], z3_addr[4..8]]
#
# Note, (X, Y, Z)    => input point
#       (X3, Y3, Z3) => output point
#
# Output:
#
# Last 6 memory addresses of 12 memory addresses which were provided during invocation, where resulting doubled
# point is kept in similar form. For seeing X3, Y3, Z3 -coordinates of doubled point, one needs to read from
# those 6 memory addresses.
#
# Stack at end of execution of routine looks like
#
#   [x3_addr[0..4], x3_addr[4..8], y3_addr[0..4], y3_addr[4..8], z3_addr[0..4], z3_addr[4..8]]
export.point_doubling.12
  dup.3
  pushw.mem
  dup.6
  pushw.mem         # y -coordinate on stack top

  dupw.1
  dupw.1            # repeated y -coordinate

  exec.u256_mod_mul # = t0

  storew.local.0
  swapw
  storew.local.1
  swapw             # cache t0

  dupw.1
  dupw.1            # repeated t0

  exec.u256_mod_add # = z3

  dupw.1
  dupw.1            # repeated z3

  exec.u256_mod_add # = z3

  dupw.1
  dupw.1            # repeated z3

  exec.u256_mod_add # = z3

  popw.local.2
  popw.local.3      # cache z3

  dup.5
  pushw.mem
  dup.8
  pushw.mem         # z -coordinate on stack top

  dup.11
  pushw.mem
  dup.14
  pushw.mem         # y -coordinate on stack top

  exec.u256_mod_mul # = t1

  popw.local.4
  popw.local.5      # cache t1

  dup.5
  pushw.mem
  dup.8
  pushw.mem         # z -coordinate on stack top

  dupw.1
  dupw.1            # repeated z

  exec.u256_mod_mul # = t2

  push.0.0.0.0
  push.0.0.21.20517 # = b3

  exec.u256_mod_mul # = t2

  storew.local.6
  swapw
  storew.local.7    # cache t2
  swapw

  pushw.local.3
  pushw.local.2     # = z3

  exec.u256_mod_mul # = x3

  popw.local.8
  popw.local.9      # cache x3

  pushw.local.7
  pushw.local.6     # = t2

  pushw.local.1
  pushw.local.0     # = t0

  exec.u256_mod_add # = y3

  popw.local.10
  popw.local.11     # cache y3

  pushw.local.5
  pushw.local.4     # = t1

  pushw.local.3
  pushw.local.2     # = z3

  exec.u256_mod_mul # = z3

  popw.local.2
  popw.local.3      # cache z3

  pushw.local.7
  pushw.local.6     # = t2

  dupw.1
  dupw.1            # repeated t2

  exec.u256_mod_add # = t1

  pushw.local.7
  pushw.local.6     # = t2

  exec.u256_mod_add # = t2

  pushw.local.1
  pushw.local.0     # = t0

  exec.u256_mod_sub # = t0

  storew.local.0
  swapw
  storew.local.1
  swapw             # cache t0

  pushw.local.11
  pushw.local.10    # = y3

  exec.u256_mod_mul # = y3

  pushw.local.9
  pushw.local.8     # = x3

  exec.u256_mod_add # = y3

  popw.local.10
  popw.local.11     # cache y3

  dup.3
  pushw.mem
  dup.6
  pushw.mem         # y -coordinate on stack top

  dup.9
  pushw.mem
  dup.12
  pushw.mem         # x -coordinate on stack top

  exec.u256_mod_mul # = t1

  pushw.local.1
  pushw.local.0     # = t0

  exec.u256_mod_mul # = x3

  dupw.1
  dupw.1            # repeated x3

  exec.u256_mod_add # = x3

  popw.local.8
  popw.local.9      # cache x3

  dropw
  drop
  drop

  dup
  pushw.local.8
  movup.4
  popw.mem          # write x3[0..4] to memory

  dup.1
  pushw.local.9
  movup.4
  popw.mem          # write x3[4..8] to memory

  dup.2
  pushw.local.10
  movup.4
  popw.mem          # write y3[0..4] to memory

  dup.3
  pushw.local.11
  movup.4
  popw.mem          # write y3[4..8] to memory

  dup.4
  pushw.local.2
  movup.4
  popw.mem          # write z3[0..4] to memory

  dup.5
  pushw.local.3
  movup.4
  popw.mem          # write z3[4..8] to memory
end

# Given two secp256k1 points in projective coordinate system ( i.e. with x, y, z -coordinates
# as secp256k1 prime field elements, represented in Montgomery form, each coordinate using eight 32 -bit limbs ),
# this routine adds those two points on elliptic curve, using exception-free addition formula from
# algorithm 7 of https://eprint.iacr.org/2015/1060.pdf, while following prototype
# implementation https://github.com/itzmeanjan/secp256k1/blob/ec3652a/point.py#L60-L115
# 
# Input:
#
# 18 memory addresses on stack such that first 6 memory addresses are for first input point, next 6
# memory addresses holding x, y, z -coordinates of second input point & last 6 addresses are for storing 
# resulting point ( addition of two input points ).
#
# Expected stack during invocation of this routine:
#
#   [x1_addr[0..4], x1_addr[4..8], y1_addr[0..4], y1_addr[4..8], z1_addr[0..4], z1_addr[4..8], 
#     x2_addr[0..4], x2_addr[4..8], y2_addr[0..4], y2_addr[4..8], z2_addr[0..4], z2_addr[4..8],
#       x3_addr[0..4], x3_addr[4..8], y3_addr[0..4], y3_addr[4..8], z3_addr[0..4], z3_addr[4..8]]
#
# Note, (X1, Y1, Z1)    => input point 1
#       (X2, Y2, Z2)    => input point 2
#       (X3, Y3, Z3)    => output point
#
# Output:
#
# Last 6 memory addresses of 18 input memory addresses which were provided during invocation, where resulting elliptic curve
# point is kept in similar form. For seeing X3, Y3, Z3 -coordinates of doubled point, one needs to read from
# those 6 memory addresses.
#
# Stack at end of execution of routine looks like
#
#   [x3_addr[0..4], x3_addr[4..8], y3_addr[0..4], y3_addr[4..8], z3_addr[0..4], z3_addr[4..8]]
export.point_addition.16
  dup.6
  dup.8

  pushw.mem
  movup.4
  pushw.mem # x2 on stack top

  dup.8
  dup.10

  pushw.mem
  movup.4
  pushw.mem # x1 on stack top

  exec.u256_mod_mul # = t0

  popw.local.0
  popw.local.1 # cache t0

  dup.8
  dup.10

  pushw.mem
  movup.4
  pushw.mem # y2 on stack top

  dup.10
  dup.12

  pushw.mem
  movup.4
  pushw.mem # y1 on stack top

  exec.u256_mod_mul # = t1

  popw.local.2
  popw.local.3 # cache t1

  dup.10
  dup.12

  pushw.mem
  movup.4
  pushw.mem # z2 on stack top

  dup.12
  dup.14

  pushw.mem
  movup.4
  pushw.mem # z1 on stack top

  exec.u256_mod_mul # = t2

  popw.local.4
  popw.local.5 # cache t2

  dup.2
  dup.4

  pushw.mem
  movup.4
  pushw.mem # y1 on stack top

  dup.8
  dup.10

  pushw.mem
  movup.4
  pushw.mem # x1 on stack top

  exec.u256_mod_add # = t3

  popw.local.6
  popw.local.7 # cache t3

  dup.8
  dup.10

  pushw.mem
  movup.4
  pushw.mem # y2 on stack top

  dup.15
  dup.15
  swap

  pushw.mem
  movup.4
  pushw.mem # x2 on stack top
  
  exec.u256_mod_add # = t4

  pushw.local.7
  pushw.local.6 # t3 loaded back

  exec.u256_mod_mul # = t3

  popw.local.6
  popw.local.7 # cache t3

  pushw.local.3
  pushw.local.2 # t1 loaded back

  pushw.local.1
  pushw.local.0 # t0 loaded back

  exec.u256_mod_add # = t4

  pushw.local.7
  pushw.local.6 # t3 loaded back

  exec.u256_mod_sub # = t3

  popw.local.6
  popw.local.7 # cache t3

  dup.2
  dup.4

  pushw.mem
  movup.4
  pushw.mem # y1 on stack top

  dup.12
  dup.14

  pushw.mem
  movup.4
  pushw.mem # z1 on stack top

  exec.u256_mod_add # = t4

  popw.local.8
  popw.local.9 # cache t4

  dup.11
  dup.11

  dup.10
  dup.12

  pushw.mem
  movup.4
  pushw.mem # y2 on stack top

  movup.8
  movup.9

  pushw.mem
  movup.4
  pushw.mem # z2 on stack top

  exec.u256_mod_add # = x3

  pushw.local.9
  pushw.local.8 # t4 loaded back

  exec.u256_mod_mul # = t4

  popw.local.8
  popw.local.9 # cache t4

  pushw.local.5
  pushw.local.4 # t2 loaded back

  pushw.local.3
  pushw.local.2 # t1 loaded back

  exec.u256_mod_add # = x3

  pushw.local.9
  pushw.local.8 # t4 loaded back

  exec.u256_mod_sub # = t4

  popw.local.8
  popw.local.9 # cache t4

  dup.4
  dup.6

  pushw.mem
  movup.4
  pushw.mem # z1 on stack top

  dup.8
  dup.10

  pushw.mem
  movup.4
  pushw.mem # x1 on stack top

  exec.u256_mod_add # = x3

  popw.local.10
  popw.local.11 # cache x3

  dup.10
  dup.12

  pushw.mem
  movup.4
  pushw.mem # z2 on stack top

  dup.15
  dup.15
  swap

  pushw.mem
  movup.4
  pushw.mem # x2 on stack top

  exec.u256_mod_add # = y3

  pushw.local.11
  pushw.local.10 # x3 loaded back

  exec.u256_mod_mul # = x3

  popw.local.10
  popw.local.11 # cache x3

  pushw.local.5
  pushw.local.4 # t2 loaded back

  pushw.local.1
  pushw.local.0 # t0 loaded back

  exec.u256_mod_add # = y3

  pushw.local.11
  pushw.local.10 # x3 loaded back

  exec.u256_mod_sub # = y3

  popw.local.12
  popw.local.13 # cache y3

  pushw.local.1
  pushw.local.0 # t0 loaded back

  dupw.1
  dupw.1

  exec.u256_mod_add # = x3

  storew.local.10
  swapw
  storew.local.11
  swapw # cache x3

  pushw.local.1
  pushw.local.0 # t0 loaded back

  exec.u256_mod_add # = t0

  popw.local.0
  popw.local.1 # cache t0

  push.0.0.0.0
  push.0.0.21.20517 # b3 on stack top

  pushw.local.5
  pushw.local.4 # t2 loaded back

  exec.u256_mod_mul # = t2

  storew.local.4
  swapw
  storew.local.5
  swapw # cache t2

  pushw.local.3
  pushw.local.2 # t1 loaded back

  exec.u256_mod_add # = z3

  popw.local.14
  popw.local.15 # cache z3

  pushw.local.5
  pushw.local.4 # t2 loaded back

  pushw.local.3
  pushw.local.2 # t1 loaded back

  exec.u256_mod_sub # = t1

  popw.local.2
  popw.local.3 # cache t1

  push.0.0.0.0
  push.0.0.21.20517 # b3 on stack top

  pushw.local.13
  pushw.local.12 # y3 loaded back

  exec.u256_mod_mul # = y3

  storew.local.12
  swapw
  storew.local.13
  swapw # cache y3

  pushw.local.9
  pushw.local.8 # t4 loaded back

  exec.u256_mod_mul # = x3

  popw.local.10
  popw.local.11 # cache x3

  pushw.local.3
  pushw.local.2 # t1 loaded back

  pushw.local.7
  pushw.local.6 # t3 loaded back

  exec.u256_mod_mul # = t2

  pushw.local.11
  pushw.local.10 # x3 loaded back

  exec.u256_mod_neg
  exec.u256_mod_add # = x3

  popw.local.10
  popw.local.11 # cache x3

  pushw.local.1
  pushw.local.0 # t0 loaded back

  pushw.local.13
  pushw.local.12 # y3 loaded back

  exec.u256_mod_mul # = y3

  popw.local.12
  popw.local.13 # cache y3

  pushw.local.15
  pushw.local.14 # z3 loaded back

  pushw.local.3
  pushw.local.2 # t1 loaded back

  exec.u256_mod_mul # = t1

  pushw.local.13
  pushw.local.12 # y3 loaded back

  exec.u256_mod_add # = y3

  popw.local.12
  popw.local.13 # cache y3

  pushw.local.7
  pushw.local.6 # t3 loaded back

  pushw.local.1
  pushw.local.0 # t0 loaded back

  exec.u256_mod_mul # = t0

  popw.local.0
  popw.local.1 # cache t0

  pushw.local.9
  pushw.local.8 # t4 loaded back

  pushw.local.15
  pushw.local.14 # z3 loaded back

  exec.u256_mod_mul # = z3

  pushw.local.1
  pushw.local.0 # t0 loaded back

  exec.u256_mod_add # = z3

  popw.local.14
  popw.local.15 # cache z3

  dropw
  dropw
  dropw

  pushw.local.10
  dup.4
  popw.mem          # write x3[0..4] to memory

  pushw.local.11
  dup.5
  popw.mem          # write x3[4..8] to memory

  pushw.local.12
  dup.6
  popw.mem          # write y3[0..4] to memory

  pushw.local.13
  dup.7
  popw.mem          # write y3[4..8] to memory

  pushw.local.14
  dup.8
  popw.mem          # write z3[0..4] to memory

  pushw.local.15
  dup.9
  popw.mem          # write z3[4..8] to memory
end

# Given a 256 -bit scalar, in radix-2^32 representation ( such that it
# takes 8 stack elements to represent whole scalar, where each limb is 
# of 32 -bit width ), this routine multiplies group identity point 
# ( 0, 1, 0 in projective coordinate system ) with given scalar, producing
# another point on secp256k1 curve, which will also be presented in projective coordinate
# system.
#
# Input:
#
# During invocation, this routine expects stack in following form
#
# [Sc0, Sc1, Sc2, Sc3, Sc4, Sc5, Sc6, Sc7, X_addr_0, X_addr_1, Y_addr_0, Y_addr_1, Z_addr_0, Z_addr_1]
#
# Sc{0..8}           -> 256 -bit scalar in radix-2^32 form | Sc0 is least significant limb & Sc7 is most significant limb
# X_addr_0, X_addr_1 -> Resulting secp256k1 point's X -coordinate to be placed, in Montgomery form, in given addresses
# Y_addr_0, Y_addr_1 -> Resulting secp256k1 point's Y -coordinate to be placed, in Montgomery form, in given addresses
# Z_addr_1, Z_addr_1 -> Resulting secp256k1 point's Z -coordinate to be placed, in Montgomery form, in given addresses
#
# Output:
#
# At end of execution of this routine, stack should look like below
#
# [X_addr_0, X_addr_1, Y_addr_0, Y_addr_1, Z_addr_0, Z_addr_1]
#
# X_addr_0, X_addr_1 -> Resulting secp256k1 point's X -coordinate written, in Montgomery form, in given addresses
# Y_addr_0, Y_addr_1 -> Resulting secp256k1 point's Y -coordinate written, in Montgomery form, in given addresses
# Z_addr_0, Z_addr_1 -> Resulting secp256k1 point's Z -coordinate written, in Montgomery form, in given addresses
#
# One interested in resulting point, should read from provided address on stack.
# 
# This routine implements double-and-add algorithm, while following 
# https://github.com/itzmeanjan/secp256k1/blob/d23ea7d/point.py#L174-L186 
export.point_mul.20
  # identity point of group (0, 1, 0) in projective coordinate
  # see https://github.com/itzmeanjan/secp256k1/blob/d23ea7d/point.py#L40-L45
  push.0.0.0.0
  popw.local.0
  push.0.0.0.0
  popw.local.1 # init & cache res_X

  push.0.0.1.977
  popw.local.2
  push.0.0.0.0
  popw.local.3  # init & cache res_Y

  push.0.0.0.0
  popw.local.4
  push.0.0.0.0
  popw.local.5  # init & cache res_Z

  popw.local.18
  popw.local.19

  # push (2^255)G into stack
  push.1767015067.3527058907.3725831105.456741272
  push.2390137912.1282011242.2190683269.3442419054
  push.3795524707.3432807938.3464672759.1770073772
  push.112682241.1539449350.22356095.833785547
  push.1486287845.3004908234.1106597725.778081023
  push.2518893645.1449684363.4238272990.1568923791

  # push (2^254)G into stack
  push.3321753728.3417442410.95544364.560677759
  push.949930655.1858648483.3255479703.636270793
  push.2764786988.2255507265.534201118.1268406717
  push.2840054024.3362847970.549055994.1698803586
  push.3919977144.392046710.1215837599.1895884648
  push.2181186994.1882380144.1948365018.2310826502

  # push (2^253)G into stack
  push.4246773447.3551240214.3835261861.215608593
  push.1499295070.3971437743.2217725047.3276766074
  push.637941066.4262787880.2876205873.3838430350
  push.2618960121.3277112134.2548144913.2302900082
  push.1484680481.1136445883.4106450200.2612850720
  push.1360840567.3731071105.615689712.1958952143

  # push (2^252)G into stack
  push.4095046996.2141155988.1989873639.1098634363
  push.2825534215.1305880981.842187130.934957739
  push.3565840709.2591895807.2473095747.4046137811
  push.1321637484.1327030418.2902000148.4053141646
  push.3768343242.327112700.2467568403.1541255891
  push.3164290634.2510017135.1351906398.275052315

  # push (2^251)G into stack
  push.3408819858.13908515.2830929943.1925067160
  push.550748983.1200583051.3108496349.1708525255
  push.13657435.466709090.2331149592.3083955378
  push.371813147.4208145029.1144509954.2115803330
  push.3459834965.3557149523.3355988002.212343495
  push.2983034454.2629555476.1952408093.4166516135

  # push (2^250)G into stack
  push.2328084612.418074529.1301558259.1427548481
  push.417790438.2317439352.3958708618.1110650634
  push.2912786659.1744005957.3445828053.1075248618
  push.845534863.4292867044.310255275.3021409946
  push.869397318.93253300.3475188449.1370550567
  push.3936742919.2772104824.1196829250.1483635998

  # push (2^249)G into stack
  push.3456486173.3694082533.2328185985.2920466896
  push.2518000253.3024655185.1574652291.1534474891
  push.2448680816.3412922716.3327752223.2373494510
  push.3640696437.2333975126.1615022893.2504190400
  push.3723744963.1747847880.3310452633.287120923
  push.2873818992.560017005.1390537144.642877591

  # push (2^248)G into stack
  push.2078975623.3619671174.3899400560.2278612219
  push.2192058477.3713811608.1874616361.255158776
  push.405048047.3452456883.4267489721.4202926471
  push.254103738.3976447841.1058597257.3095710914
  push.1309563026.2185586142.1795152983.3760278552
  push.149847018.43710904.573475438.400924673

  # push (2^247)G into stack
  push.3016661756.1200649498.2634850411.2747110743
  push.3052656681.1734130525.1880055269.9702456
  push.2755468688.3198554212.4084634815.1110277604
  push.1534805690.2618857725.3635522397.3957448775
  push.3350594128.3474745972.613125519.2325069777
  push.1195395795.847173656.1042229407.2353048631

  # push (2^246)G into stack
  push.182141876.3946645722.2341983359.3819303925
  push.1820294664.3746044143.4125010121.2748068242
  push.1679381327.2523859344.3072468730.3524156261
  push.651324272.4179278148.3433441038.2462280092
  push.4288982374.1217574074.2438325053.1113015771
  push.726578974.4271386481.664798730.2697487178

  # push (2^245)G into stack
  push.1092410868.1923897824.4285951771.2731199034
  push.1213798187.3810118122.3504956936.3007403676
  push.3992973367.2165149480.2506500644.2182645161
  push.1675201847.2473958234.1620101697.1831612855
  push.2413437811.4091633862.4236386153.4097743837
  push.593823559.3592854855.213157084.829358460

  # push (2^244)G into stack
  push.374212451.1344010413.3803115775.1995055872
  push.1103730782.1836000606.3578579675.4135321180
  push.1388646176.451875476.3347613652.2311582805
  push.1646328101.1156880648.1150213804.3195028175
  push.972964336.1343764905.1839974576.2572304389
  push.833904658.2913879953.3083685625.3003126163

  # push (2^243)G into stack
  push.1406194949.797375917.4197616069.73120315
  push.900860937.3301129074.104737844.1761853537
  push.2381579073.4129492154.3430521627.2014044312
  push.806461130.3624581514.2911627493.3192496244
  push.104013554.1758500829.3420551470.4017437352
  push.1976086277.180504913.1530408794.1459183005

  # push (2^242)G into stack
  push.1912206721.1463429452.3613798737.27046412
  push.2531269453.256614732.315908841.152364702
  push.1867739315.41237985.2038363597.2440212436
  push.2101402377.268059336.3624308985.3465908484
  push.1249714618.745770454.153252740.2819930461
  push.3144493745.2381952902.2629256137.2857567580

  # push (2^241)G into stack
  push.590932232.3439296894.2929528507.2541614374
  push.1362452159.2118801839.3206811157.2633066603
  push.3348224685.3067161788.3313004782.2588581966
  push.3765662067.2456443598.847343643.2448510023
  push.817961294.233589856.3957239612.215427003
  push.4063583217.465614150.3479138163.3385406759

  # push (2^240)G into stack
  push.1526077311.2666406927.1430060802.297533935
  push.3696765434.2214283621.522385123.1288882766
  push.3666167054.3859872276.1777208133.425280949
  push.997860002.2213288659.3778419282.1398914528
  push.3467610694.2330225310.3594111511.727250670
  push.1039953386.4252472174.947949313.61819516

  # push (2^239)G into stack
  push.2635234702.248663915.1231597263.3199395814
  push.2013116127.942897994.521384611.3818734666
  push.369895413.3638006788.3180312665.852362182
  push.2865456065.3704137580.3335499609.2096130576
  push.3375098642.646313020.1794406009.2432931828
  push.2491695632.1837210206.1833290135.1771847585

  # push (2^238)G into stack
  push.3522173991.2023314688.337772967.3390295520
  push.1967622689.1067891404.789799845.1906378354
  push.3944558367.2803630483.1001194909.2612394886
  push.3287628350.3129624068.525968774.2687896166
  push.3683708503.3675594325.1617746195.3389854201
  push.2866089984.1824562789.3963157458.3758632050

  # push (2^237)G into stack
  push.2225412875.2928183305.1298488813.761888910
  push.3933715661.3963583371.878384267.3775858351
  push.1038773072.1920588852.1257037570.1001181507
  push.1053158028.1666660416.1208633703.3234466328
  push.3863856840.2435457769.2371609754.3264611457
  push.694248678.2208979821.2467480025.2867378887

  # push (2^236)G into stack
  push.3897978566.2467883603.1025626003.3134316404
  push.736454246.2397184556.890213241.2975327423
  push.3812010854.2404766051.490309801.3215846786
  push.1625996265.836178867.3784638064.140279558
  push.529533830.641169704.3930210021.2977723362
  push.3598304296.1678631941.500566584.1639362574

  # push (2^235)G into stack
  push.3053492399.3234452290.3725204268.3082597979
  push.2556246659.92046245.3653694776.2204048581
  push.2454272254.2759588628.1899557210.2792843025
  push.1719659685.449596210.1812659793.1211636195
  push.4022887874.3403222840.159883978.1398586648
  push.2456990921.3490595374.2440218892.185657090

  # push (2^234)G into stack
  push.3770223320.2443982963.3790433734.1492334047
  push.3318792945.4120823233.1754638116.282802467
  push.2258770410.3759763491.3650017203.744570486
  push.459952549.2220102209.1285588733.4209046487
  push.3965394424.410154417.3538308522.1717240069
  push.788550021.1382601951.2554306479.3575808578

  # push (2^233)G into stack
  push.1918978714.3948082086.2498028497.3837142776
  push.571084896.2664177070.1203162646.1542631252
  push.834094117.528954524.1473403046.1504553596
  push.159696631.1267857207.1158643478.1694566227
  push.1055578266.82611738.3651300217.1308391227
  push.1944122387.1246899064.3398560350.4021755929

  # push (2^232)G into stack
  push.1336310405.1425090978.333090010.3184174827
  push.12546364.982720382.3225927904.1347277555
  push.542557784.4144894945.2539825585.48094730
  push.3479144599.1082334498.2530672539.205485172
  push.1542529012.3920563771.3459154938.773685725
  push.624042286.3813467983.4046361439.1144938196

  # push (2^231)G into stack
  push.321435440.1673620386.3066610418.642630809
  push.4137726641.829881322.1007667761.3831585089
  push.770847453.783940588.3137890895.1383720232
  push.1788926764.2139295993.4189083365.3900432388
  push.2309280304.3198409078.1202556162.700149846
  push.2086866628.3272630700.4108735625.295045197

  # push (2^230)G into stack
  push.4004524983.4174461079.3988751163.156028962
  push.2713474275.782135120.2053262251.3868711215
  push.2456973078.2128068043.2059065613.1157878633
  push.2495624436.1647388031.1511859266.280173054
  push.1016784963.410975754.224598369.2580931274
  push.1327974982.1428826325.3546421227.1266067080

  # push (2^229)G into stack
  push.2985226199.2248995759.457946349.1557038245
  push.1947392460.1884313194.2173431365.3204094193
  push.790274957.3591717566.1047017917.1041308951
  push.2026878265.2908214774.3812050392.1297388559
  push.3937302361.737404085.4190399179.479949600
  push.207475880.467262689.1819604680.1583971742

  # push (2^228)G into stack
  push.3034026076.3366514620.2160857415.3009457335
  push.3210442343.693589582.3463222115.3295249715
  push.22636217.778760472.3049304537.725737798
  push.3295925767.3946554521.1314482235.1870569031
  push.435404112.528732176.382744961.1292144435
  push.32854073.2878296021.3561173503.3952815453

  # push (2^227)G into stack
  push.3036687300.1450173606.3975698908.1522027464
  push.1303125934.658550511.3729257464.449156268
  push.1347635321.88934206.841531409.3640535427
  push.2434216359.496275057.1933866306.1803642124
  push.473050634.1327384209.1142353387.380473373
  push.4087716062.2668216568.526152869.888742082

  # push (2^226)G into stack
  push.2544895281.1372106970.607553643.1563159011
  push.3522965948.764856879.593559903.3828491558
  push.2536534403.505612747.2062700618.3027380143
  push.2930006168.3054251558.2078032525.3244059987
  push.3511077045.391731676.2114939200.1162785720
  push.2960453830.2210002259.1041747012.1992207679

  # push (2^225)G into stack
  push.1550977805.4289205885.444496993.1310459352
  push.871910711.3417036522.3716385505.11888219
  push.2622345671.3598831914.3446802864.848772170
  push.3638450560.358157191.3656970576.3929909964
  push.4110198145.265313956.1030822644.3157944671
  push.166152820.126492444.3378668395.3728748438

  # push (2^224)G into stack
  push.2927900728.1787920961.3664883545.2899562836
  push.2460030714.1975634687.4270931590.488120361
  push.2613008707.2408629383.782285353.2614556985
  push.1215951813.877552417.1465680632.3361307245
  push.567437848.2995526766.2847237315.2622486040
  push.2981892650.2535214426.1915130.2331001822

  # push (2^223)G into stack
  push.1283706606.3295423973.3568559575.1861829897
  push.2014404186.1887722909.2317691624.1350491520
  push.3957186729.966814106.992304085.139003780
  push.2837592290.204070225.1406493533.1942656126
  push.3438186533.3198272125.3341330868.1686941612
  push.2341314972.3300661555.3892604075.2474469554

  # push (2^222)G into stack
  push.3346616811.3216959112.2078620749.2753226745
  push.1275201972.1305465818.3683011006.1446705965
  push.318687452.3388583126.300732715.425100883
  push.3150505212.4267463796.2459992470.3411277296
  push.632279119.1370910167.4039663956.719823411
  push.3865600976.603315528.3245510671.111278871

  # push (2^221)G into stack
  push.2124892802.2899790009.2373233225.1730219547
  push.1846915631.2480097620.3577698349.1218544893
  push.1759826544.316908183.3833032603.3731983950
  push.4289112270.68670616.23969732.3865912684
  push.1651729448.3731468053.2542904658.689898465
  push.3855110489.3147500082.1522595904.3128843599

  # push (2^220)G into stack
  push.619358414.664394475.511703192.920858191
  push.3662878119.3108233390.685642640.2625539939
  push.1418563290.1315618897.1933499394.1508391442
  push.1005648548.772407716.4133727180.4111104471
  push.960838446.1836940398.424939287.2849959135
  push.464777089.1461079999.2740814415.1187689149

  # push (2^219)G into stack
  push.958277381.365702046.4274926853.2376482223
  push.1140131025.2664272694.4042707573.1300593223
  push.3016354591.44723240.2457111193.1354843961
  push.4069356548.2469839957.3755579156.283024429
  push.2798859732.1220532324.2329473049.133202497
  push.3842938795.4178807336.2829082774.3289599419

  # push (2^218)G into stack
  push.469751094.3697464372.1551273632.573613102
  push.2814366575.338306934.3219690755.1730228237
  push.260641556.2496893679.3476882565.3732026979
  push.3480440260.3405352696.2661974644.468622153
  push.111634516.1421316653.1439001701.3091605826
  push.3057766773.4023342987.1295014553.1623217457

  # push (2^217)G into stack
  push.1300072616.4201990711.2009407974.3082548322
  push.3550263079.401677998.130708854.1190102107
  push.2500058319.234254723.3250006028.1217549160
  push.247839968.1766270459.2199432616.2631364854
  push.80742189.1742470463.2487963811.542276317
  push.393331917.1230104509.1986738288.684678892

  # push (2^216)G into stack
  push.2063127465.2224776754.1565274094.3545492585
  push.701699999.2490924057.1701631579.1848505821
  push.2374461209.724323246.358319328.2136780222
  push.1088185355.755614813.754609756.489027891
  push.1560499650.1461437121.1617265066.3947089685
  push.2723511710.785436091.4285435836.2664086282

  # push (2^215)G into stack
  push.1441413985.3953059213.1252344493.2181548730
  push.1391979527.3517663363.2608650452.500316234
  push.163846830.2984106568.2436399119.723651998
  push.2276306899.4273564593.3138368756.2245223051
  push.1010136976.1404985580.2472469719.2305540222
  push.4139238349.1361599354.37455998.3200970598

  # push (2^214)G into stack
  push.3268333994.571220065.3904133961.3395645609
  push.2816355724.2744258411.3202744668.273468686
  push.1181699429.728245551.2540259105.2906420700
  push.4061702105.2478974976.3800309872.4008860087
  push.2356036762.1457047634.512843067.1519128330
  push.1540982819.563906995.2621622873.2673274228

  # push (2^213)G into stack
  push.234676145.256884026.52202501.2637262269
  push.2694474090.2557326045.3137518526.1240269207
  push.3961653970.3838159997.2970253599.551021872
  push.2420714952.2494664294.724965712.1669008241
  push.1678891300.819802084.920288413.3498333264
  push.3424082514.4080715965.3000272381.3577648268

  # push (2^212)G into stack
  push.369979095.3342424120.2515577834.1403293456
  push.162436098.2772186275.1292462380.143237952
  push.1666337119.1857063562.1929182677.3511332175
  push.1018428010.4294608175.2915772725.371340867
  push.404697502.4222226879.2229875264.2115445635
  push.1148365313.1962968134.3158968015.3323963493

  # push (2^211)G into stack
  push.782566951.2414451749.4244166595.352177853
  push.4043525414.2412876685.760012657.2626059230
  push.2883001739.3082794671.893382718.1944452668
  push.3366766502.395764187.954202289.3986851759
  push.4146647688.321033063.3510968976.1278862182
  push.961640246.51821053.138878530.390826772

  # push (2^210)G into stack
  push.3232692885.874595634.1944852801.2608688031
  push.169444817.3905741579.2645439209.3030101236
  push.2749272991.475310084.1967848598.3955805843
  push.2113111837.3665021265.3786338608.2298153658
  push.2573329529.3620885480.3216520821.1841945980
  push.4199822673.1305699101.2627451673.1788150534

  # push (2^209)G into stack
  push.1137057888.3435405761.3469613410.4027296466
  push.3731963929.2323396206.2647609854.3694143483
  push.1582405283.1264767328.2941904606.3536469041
  push.2563189470.1125728355.514083020.1124353541
  push.1799760839.3078682677.257258660.3762238685
  push.3709690563.549902884.2242194635.3739298855

  # push (2^208)G into stack
  push.448452926.3450985026.545514550.1201359236
  push.3634193660.3190442493.882081595.3933680047
  push.1349780974.3226205716.1817358996.4235352499
  push.1351166256.4196881392.4253428350.691124095
  push.2745366506.788658156.4242112957.1630977282
  push.2678622688.3066821601.3289052211.1248206257

  # push (2^207)G into stack
  push.3644518502.3732852513.2357128332.676331823
  push.1916025323.3701044413.1319585839.581894166
  push.3072009580.3143783003.1329858866.2689904133
  push.1620766189.966940869.3245372610.1886152893
  push.3046272278.358626846.3829258050.465812374
  push.944386678.2591633282.2504235071.567042626

  # push (2^206)G into stack
  push.3922987158.108814019.3777734856.1485000560
  push.2710768741.1616025433.546954416.3238254469
  push.2549414397.4105828035.3476228308.2978303903
  push.2822804352.935637314.1440711465.507956193
  push.3042735058.2851463274.777765340.4221346227
  push.3005200779.341834517.3731102939.152763094

  # push (2^205)G into stack
  push.3677926886.1599588132.2158810040.1055178879
  push.3127807599.3026904391.1976284050.1557420788
  push.1019404486.1111487779.537609494.3812769894
  push.2284708863.1803858496.3545549739.1363666819
  push.1930059599.3973734754.3106682314.3979222393
  push.1753919648.3472464120.3151257311.3379514080

  # push (2^204)G into stack
  push.1524339650.3515735467.3907989411.1648656604
  push.960803586.695656961.3591234774.663516183
  push.1993726107.525257035.844979478.3908804621
  push.2703094512.2651683439.904102291.626474857
  push.3523982667.324696846.3502414063.1131793216
  push.3796866466.3100978440.1642197136.4001983926

  # push (2^203)G into stack
  push.1391991968.418968215.1137955108.1712255338
  push.2262573183.4251152174.1743670167.868382033
  push.1535892432.3333248629.2043511247.2996391032
  push.2302826707.927635057.3373136182.238398822
  push.1185184074.3389154792.2460424441.2513942548
  push.2791808198.843805962.1860090059.2571212279

  # push (2^202)G into stack
  push.3766550217.457602604.1788386326.4160827950
  push.3359107157.3007700557.3168195456.4285623041
  push.749411667.3913905385.907503122.3793819610
  push.1879631097.952270060.2774695162.3321241734
  push.357330906.2785149658.606444315.2714856291
  push.1899817798.570856509.2225898115.1051650494

  # push (2^201)G into stack
  push.4095764453.1533214482.2510953440.1704507751
  push.511275850.3175558769.59135097.1287659454
  push.777643555.1501562020.3846194279.1527473716
  push.4092280765.294278980.2162967043.481098861
  push.2755496574.3519788633.919533222.2648912988
  push.3937457711.4137541527.992063813.3193654052

  # push (2^200)G into stack
  push.2907005800.3502237281.653703936.929946483
  push.297587544.794473728.3611244036.4070224159
  push.2965608115.4081526807.48830403.4014754019
  push.1367445881.2565299343.3139984011.1160618095
  push.4135990155.2603221192.1058306199.1888557965
  push.2009071546.2341486324.2515248183.2860729139

  # push (2^199)G into stack
  push.729134313.2172017006.109797606.507719978
  push.297107756.3142666049.1608657079.2573568521
  push.1371849441.2697974248.1265922011.1226939181
  push.1566331759.3357417936.2272206374.787274229
  push.2113559319.2261662807.3965887726.35353300
  push.1528193762.2689884256.1394603683.2276308131

  # push (2^198)G into stack
  push.922640438.3326491725.4148951913.1012599717
  push.1047209446.531143364.2798678993.1854873699
  push.1096473223.106254079.3522501426.4186953790
  push.1195938999.664763569.2142080281.1258390353
  push.78198817.2773655847.289346104.4040893901
  push.43181145.4271512875.2992013953.537063912

  # push (2^197)G into stack
  push.3006788464.3922936714.1990865200.2159906753
  push.3304257464.2537616956.3512623498.2126195581
  push.2463504785.749985839.1670927809.292870484
  push.2275477839.2837850480.1655960220.3845945523
  push.1504096042.2812881928.1080060923.123366491
  push.2794198664.533667712.761606379.1284702956

  # push (2^196)G into stack
  push.3621729482.2930415939.2210283962.3497752647
  push.3074152882.2072717813.1590290827.2333139902
  push.275398732.2538348840.2868690151.3827840943
  push.164954283.80506484.453749957.1446473025
  push.2828607444.444006774.1153496959.1413033905
  push.746273230.2552422355.395058354.2609693913

  # push (2^195)G into stack
  push.962742343.660648980.2700130693.2711707900
  push.2409085222.2422063545.1256650400.1518733434
  push.2124369539.351079963.3666187436.2299416501
  push.3927628642.2923643026.2221540048.824001450
  push.4027599687.3801023292.1774418420.164578292
  push.3175147862.3728138803.1983610580.357121855

  # push (2^194)G into stack
  push.4194594061.3046376037.615279429.2416563804
  push.2318448244.2408219374.53137671.2638019734
  push.835993623.237220582.2197896262.359783156
  push.242037600.3789219983.902632400.1476221585
  push.1842401186.770227603.2827187260.3227082663
  push.3750372195.2035230318.3416831941.601913095

  # push (2^193)G into stack
  push.2281574884.3468109719.3811887925.618240864
  push.743446139.1256465269.1505722664.807802512
  push.1359390184.540478134.1560717999.504399607
  push.1413652451.2028688940.914536411.3663570513
  push.1513135599.2589219924.2808858786.1625438931
  push.1213933469.1414913109.3762377620.2389861385

  # push (2^192)G into stack
  push.4119812715.2999452556.180875859.1322938549
  push.2888603813.3943606262.3079889941.1050687071
  push.2009691811.2882546771.4134283004.2391646836
  push.3105661636.4165081031.2402686886.2704817268
  push.4116855273.3414195230.1707157191.2140451481
  push.2872698197.1090373982.4141452066.1105078949

  # push (2^191)G into stack
  push.2937671341.1534065227.925069856.930630357
  push.270375959.87328024.2968835233.2210678258
  push.2636379163.1876157952.91472819.1364457688
  push.2408713695.1608465013.2938032522.3222242592
  push.4232136298.500599656.1980254106.1401512210
  push.160317760.970965281.617450999.4247894070

  # push (2^190)G into stack
  push.2686161319.689314721.2166867550.1840971114
  push.3385810362.3330711099.4068064403.1184469162
  push.1107933401.1375155703.1786752218.3981357010
  push.3916925008.1869955556.3866251702.1520426086
  push.784744739.498279707.367361455.316954808
  push.3935157946.1580307507.2250644174.2111463341

  # push (2^189)G into stack
  push.709389494.1709536089.2626782382.2343598486
  push.2342702666.2735168124.2320052430.844345549
  push.656256402.1350580952.3960259101.488937829
  push.2843153173.2952610112.1932654674.3072390569
  push.121112172.394822179.1382539451.255974591
  push.1172824150.2938808110.836617547.2098338587

  # push (2^188)G into stack
  push.3234620089.3791026977.3538719421.169647033
  push.1047066571.3292039577.4112704125.3013510086
  push.3918087802.5294091.1339402345.2713697084
  push.503468350.2737252546.1031061364.1487746185
  push.3995448026.1072833636.1591548871.178613484
  push.3753579883.2697680033.1367299585.2733294006

  # push (2^187)G into stack
  push.443574089.2513612014.451559263.1811553988
  push.920329661.1082979201.998203728.3163783243
  push.1158199531.3799617076.3853631473.833763321
  push.3174822453.991180681.2705927234.377884417
  push.1368384392.2327397257.72244593.1700814592
  push.3304653604.1672792160.3324660356.1546633361

  # push (2^186)G into stack
  push.686162940.2086455682.2696457207.3676608649
  push.2966833882.2351683939.1111934998.3100990287
  push.1237506035.4265919547.2089424609.3941932843
  push.210886216.3516169055.1859673874.894345970
  push.1100405854.1312685569.61228804.3777838166
  push.3466081709.2155866485.423378708.3584536917

  # push (2^185)G into stack
  push.3319977298.1001761551.954397555.697850973
  push.3405557562.4225516526.3753452237.3488373140
  push.3936080342.4150463149.3104492125.859234643
  push.3379919812.902031332.4127234423.1068203296
  push.526779139.2360844429.1085958826.2079343349
  push.2732300811.1034747874.1952067483.3840689472

  # push (2^184)G into stack
  push.762542982.1693412854.398831332.997153191
  push.648011985.2894234737.829511402.3797767974
  push.3187924916.786830645.3231598680.3698529334
  push.1306922071.92940236.1765659600.4246594444
  push.1700774722.4049959931.1055892457.3152238957
  push.2600871450.3694150655.2251812839.113681128

  # push (2^183)G into stack
  push.2648568157.802486206.1844235730.1127006053
  push.862089947.519164438.2477992711.1268333762
  push.2422932427.1243854876.859155462.1114290235
  push.3239328829.390770388.3076568543.1159901950
  push.1991575676.3851010087.4252846179.749245841
  push.4120163719.208444573.3273066836.2396310581

  # push (2^182)G into stack
  push.2325780958.1404316001.1024012863.1753029313
  push.902765376.1482768963.70200561.718195788
  push.106692475.494865399.2350813721.2191490510
  push.380174645.3572415106.3881554764.4086155489
  push.3531851127.127763693.3035754922.2010347478
  push.357397228.2407199632.1152780213.526524990

  # push (2^181)G into stack
  push.2460265381.4144472402.1999209392.47540040
  push.1855517426.1117565088.3496303175.3622632311
  push.2988512250.1558815963.3977787377.558855054
  push.1488642755.1803035816.884628605.1607593277
  push.1350799915.4123118312.2235979054.637155520
  push.2758940507.3572260954.1736307102.1446510083

  # push (2^180)G into stack
  push.2489227148.2417209724.2527997172.2244008580
  push.4189016784.1386868487.2015805813.3093829931
  push.3327815613.56105643.1560171414.2188376037
  push.2511445427.683260061.523237886.368897638
  push.2618869420.4023564048.1136765549.2021684249
  push.2620134969.158971595.1393095156.1886838778

  # push (2^179)G into stack
  push.3575442036.2293595514.872338064.660397928
  push.375966150.4187197681.2316199144.4122857260
  push.2135040285.3571659439.3746348982.1679301970
  push.1283860101.1502550714.455316307.3455732793
  push.271783886.669166123.1487292449.1525245500
  push.536842472.3564942660.1625988705.3824486601

  # push (2^178)G into stack
  push.1661239275.3726187141.1638625686.227416827
  push.2866583962.2942663523.3050012457.630862930
  push.3303669867.3334998183.3127375531.763491688
  push.2167892138.3364694040.896190836.1213716323
  push.846307017.1042860320.55717745.1282736061
  push.1458388276.3256615166.166790556.2414061174

  # push (2^177)G into stack
  push.3695043063.3720015305.832763561.404082991
  push.3783182816.2675786977.3362052445.3724241470
  push.4273124973.4009083844.139471253.2324698278
  push.4208234474.2802769324.690338428.3857503376
  push.1458468889.3623226609.4038826991.3454171911
  push.3260747829.505961830.964948814.1606163850

  # push (2^176)G into stack
  push.1064878524.3905951391.2860639113.271252031
  push.3585788122.1819876757.2092004150.677895715
  push.352574919.3948202420.92925261.77813632
  push.2156930754.1454279754.2238516829.1477840259
  push.1171379260.195320030.4050995933.942701412
  push.3679361876.1786587470.3330253884.1614258177

  # push (2^175)G into stack
  push.1026626164.698929808.3550246064.1187279806
  push.2855826582.1323044414.3685599927.2021328076
  push.4126630232.2439936881.819384768.2755116977
  push.191957737.2156796861.2457730292.1459335571
  push.4123952205.251004692.3428292584.737118408
  push.3760457677.3744585930.2980919435.3638455722

  # push (2^174)G into stack
  push.1362574001.2459510299.2777836709.4172165059
  push.3576642928.1673782302.3520797747.1885172576
  push.4047902427.1627383396.2301165011.1662316042
  push.3255039374.3683362559.2219244949.148516568
  push.1644097626.1470481828.3583775446.1060206785
  push.347758131.2042307323.3364394997.2868857603

  # push (2^173)G into stack
  push.3496511508.3240977.2052137645.1216365629
  push.2601190321.1827087234.3763719913.3386779367
  push.3963400091.2804451292.291357235.1347715243
  push.3923681703.3783117492.1987867447.2124922347
  push.3993795767.1140295863.1936656814.1879108671
  push.949467963.3654938676.2261950315.3621369856

  # push (2^172)G into stack
  push.238305217.1585442315.415298656.2905300146
  push.1326606968.2762804126.2747960092.4198618395
  push.2635902487.4271308573.1055526533.2750425995
  push.4241419297.3200751746.1702646116.2102320562
  push.3133691620.2462573880.2523426974.1654057057
  push.2237692178.3745562400.1904627517.3550879148

  # push (2^171)G into stack
  push.2448987925.3275442066.1827409637.1674290320
  push.3282184936.709775722.1526202438.3115170250
  push.3917916306.2676548023.1305427787.2591560018
  push.2227672957.1004174379.3282150805.2691638537
  push.95781997.4160360877.1090830158.1089602625
  push.3945153907.42688053.1599561854.2104929750

  # push (2^170)G into stack
  push.2410874676.4043533792.1401390958.574408037
  push.2741865528.3612459500.2537950457.2064564966
  push.2358681903.3871874951.4039825782.1137970111
  push.3513661688.2021908680.304851617.4004094745
  push.3516169663.1271621923.2492513716.832418194
  push.3578703254.3489860685.3809209770.1491810965

  # push (2^169)G into stack
  push.130735258.4060142944.1883010749.3160979394
  push.2180509229.2331008552.1029181051.49527350
  push.3135123011.791478914.3156881862.306857657
  push.3649368726.1189344453.1705257487.2248818897
  push.633969611.652656590.3025194991.253256001
  push.2610024954.171445359.4131381121.2223600951

  # push (2^168)G into stack
  push.1180270768.4070856243.1019180653.1108013791
  push.248035252.1956053934.1482204716.202099685
  push.1605216604.3487841655.3672032306.1217923547
  push.845017214.1525614536.2478474835.2882047916
  push.2416868770.842982332.1897801874.336353303
  push.2761635638.2501425637.1421115322.540961415

  # push (2^167)G into stack
  push.1887329780.3522056787.2697588105.2677053289
  push.4163355696.2553824423.632882177.1460416309
  push.1563901362.2462886146.1251013731.1913849350
  push.1882284178.2274240076.61356569.97975973
  push.3755274059.1093758680.2832778966.2707567368
  push.3180824165.2796858421.390155025.3252239109

  # push (2^166)G into stack
  push.817944084.2922799794.1965720193.3068454134
  push.135179130.405322578.123384097.3833149481
  push.1786448347.3123252037.4142919317.2000593835
  push.964688560.2478710384.184439768.3049668021
  push.1550661127.4113136477.4129107867.3450727525
  push.1709796860.3123488407.1268848768.2788334963

  # push (2^165)G into stack
  push.3214402739.2260872620.428885723.1320317911
  push.3095685296.2382988635.3764891871.2036557529
  push.309065148.1211888862.2287834734.2226190713
  push.2064128177.3765538787.3285321210.1088008546
  push.2214697074.4042098324.1875823367.954527288
  push.2226718651.1174910472.1549577981.2899960270

  # push (2^164)G into stack
  push.4245247200.1311108936.2234579946.701605203
  push.2480320038.1208633101.182733897.2462027731
  push.2085293697.1556521465.1670665409.2946548387
  push.4115073706.2043856024.3071963505.440821427
  push.3989935567.3764254773.1428435245.598748263
  push.1849060659.1284433717.2438538713.1736679491

  # push (2^163)G into stack
  push.564503032.3093958349.1701436582.508470974
  push.540457411.244968516.2079562759.162079306
  push.4125514701.283189010.2302495078.3481125069
  push.1541595784.1467679288.4093173307.1502371903
  push.3427738026.2163458914.2534286542.670047644
  push.1298864518.4123512033.2051052216.1319389080

  # push (2^162)G into stack
  push.2626421069.3013923005.3999983932.3727120669
  push.2176753313.1653542029.1543323461.2096306219
  push.1431848240.1480682263.933038027.500897650
  push.2995245769.2542725840.2964742948.1219200045
  push.3623148261.3141457921.1110942374.1809199798
  push.1378492106.2513425548.2045927279.3609631116

  # push (2^161)G into stack
  push.806597099.7405547.4096525630.1884454932
  push.511154139.1639256147.3513875857.1404364344
  push.698220954.2320123396.3767549439.3133813121
  push.2892217978.2244453941.1805654601.2085415298
  push.1906217998.229857777.880446017.4235385205
  push.3399265666.3623759520.2842470626.767441890

  # push (2^160)G into stack
  push.63117983.1187521712.2688609646.909566848
  push.2132793259.1797137216.3995667102.3967195197
  push.2350525150.621595852.156133309.2217548599
  push.965999638.3844643327.2205994417.3373054360
  push.3739359822.874169731.2210747310.2852570808
  push.1315823684.2794440437.1791126631.1134914047

  # push (2^159)G into stack
  push.3175450897.354570379.1102206875.1183272488
  push.577715395.3484327043.1163258242.2312722803
  push.1086151138.1666520193.4195531705.1681900111
  push.1483184174.1868917149.2696141113.4046771950
  push.1213778273.1436093414.34586292.3970718009
  push.552117193.1382740006.3895393951.1754314637

  # push (2^158)G into stack
  push.2475964213.1486635410.3696176718.399194739
  push.933616770.4280554689.1018855947.1685953442
  push.1443400350.2184489318.1358879432.884196765
  push.942482521.2123259483.898690763.4248309983
  push.1020796908.3273942413.1102842203.3813857876
  push.4269063248.3405512655.3016657240.2334632081

  # push (2^157)G into stack
  push.1413788852.3748821879.2803331137.2495568737
  push.3620985093.3509816817.3835351948.569581106
  push.3356106588.747404536.2184439659.3126202264
  push.2804006955.1477697650.3981985868.3585860031
  push.2663798275.3749853823.4123467059.345469238
  push.1198933407.2333643671.1240413.4072937665

  # push (2^156)G into stack
  push.2243633778.1526749261.3382364361.4066010598
  push.808987913.3741968100.1045223618.1007299885
  push.1903256371.2986175708.2949448216.4191546126
  push.1907231098.3362427654.1388992075.3781051589
  push.1641588201.1997556269.3052383629.1417490188
  push.724823211.1696035521.1405982363.1503913523

  # push (2^155)G into stack
  push.969597978.4197815787.2549204570.2390660699
  push.2773322276.4210519381.4031158241.1316466316
  push.540420806.1085928370.3472400130.1311479721
  push.2957661592.1768446826.2073159635.273160305
  push.564534705.3590651950.648080079.119337249
  push.3089564150.3751207063.1282659451.3150553535

  # push (2^154)G into stack
  push.1134162676.1562574376.3190511781.1420389826
  push.948327796.3676807926.3017390535.956433714
  push.3747196075.2186333639.1195494790.3731492381
  push.3475575137.3796718024.66691442.1778027685
  push.1683716820.3717024419.2285627997.4269125309
  push.1989752537.2626584356.1453296241.3300145207

  # push (2^153)G into stack
  push.842536906.469178569.1234219247.154499569
  push.2238199733.973539500.1572884162.2220314349
  push.2235034310.3630461779.1062669979.348510416
  push.3724198387.970705117.2098511613.3553089727
  push.2707033048.2856148546.1725453397.2459401074
  push.1414633309.2571998273.3448729772.2561368912

  # push (2^152)G into stack
  push.991231597.940860021.3801054757.3892051235
  push.2183317250.1176990790.985758808.3854368708
  push.381471743.1652457212.4054018291.1898221274
  push.2732390218.2694344811.479332761.1411019904
  push.3000682049.2858438453.1944465110.1431805862
  push.2546084155.3608554673.2405328688.3971361475

  # push (2^151)G into stack
  push.1319304454.328624789.2915081368.2491623375
  push.1832846760.1874310065.4112770673.71782914
  push.3964168896.3789324374.65590157.156407385
  push.2141131164.2390500499.4259384733.2509011624
  push.2580400090.4004401094.791246640.3578852948
  push.1643073482.2839435546.3019355970.188646773

  # push (2^150)G into stack
  push.1808123580.3848144968.637967900.3009040128
  push.4016507243.292370172.1888368110.862290367
  push.7922123.2772545079.2003728222.3187977222
  push.1402673100.1665145998.2418303385.4102225222
  push.579443649.204398433.2134757350.3542208621
  push.1622944700.2357953766.729038944.1466694589

  # push (2^149)G into stack
  push.2222357263.86162422.3543097117.1668404676
  push.2790064090.3566046262.1680493640.3773589374
  push.1516208887.3241099520.1470010580.2955634286
  push.460012929.1774535072.2114302751.2459109992
  push.1930928871.4255666404.2691552099.386455188
  push.4025725898.1821789688.2396070653.4255302914

  # push (2^148)G into stack
  push.3549350156.23411.1804247676.2301467555
  push.3157582275.2937611601.4276275493.1179879697
  push.53387044.317289168.3379380930.2980798659
  push.1197206032.2513745684.3245150371.1718269768
  push.2563520090.1347964491.1614423994.1739478643
  push.2064218735.3605377863.176942435.710936868

  # push (2^147)G into stack
  push.4191400341.1202587609.947879290.2854050194
  push.130779285.1492730115.2024272269.3453860775
  push.1574293651.3454274422.3072976757.3597577590
  push.2565524488.1213645335.1318888216.2320489479
  push.4205730677.667119327.1264760824.1869604868
  push.1771778487.1702483630.2520017881.2097599392

  # push (2^146)G into stack
  push.3669844999.2194479677.2360827768.2740703811
  push.1423054495.3705134590.778485318.3688557279
  push.973326635.2314976903.1005179394.284801849
  push.1933440197.255711093.4224281033.2793556242
  push.2774053275.950774412.879879733.2514940752
  push.2067901395.282245567.974220059.2605553653

  # push (2^145)G into stack
  push.2300631554.2768029517.3132875763.3442053077
  push.3564617578.3829789775.2903744190.3473714053
  push.1173922859.2530273468.3447249784.1676422960
  push.4269089657.159249346.2417143298.604633941
  push.1740791300.178466676.759101613.557586419
  push.4207096107.844498082.193986483.2315585509

  # push (2^144)G into stack
  push.1560337992.2136371016.2971344595.2622522778
  push.3432352577.2325131202.1089364199.1837417496
  push.2915115111.1834088157.837320649.582846028
  push.1137018281.1249747418.2360432081.531659163
  push.3245761527.2090793224.3803694223.2872224638
  push.1657470326.859950440.227049692.1917647286

  # push (2^143)G into stack
  push.2311239571.1476631995.3332104220.2566855325
  push.1507692569.1281600628.3223766956.155357635
  push.3998515147.3021581729.2074126042.1557537567
  push.2269876683.2132563350.2504038369.1970936459
  push.2227444990.3150820916.3856603633.2494141677
  push.1183512439.2499553057.457416044.736048145

  # push (2^142)G into stack
  push.1854176042.3823042056.1279545337.805818429
  push.3690810651.936539915.3704890400.4089034327
  push.555067810.2946559994.956354080.1845864765
  push.3626716624.3185634155.2696696971.256186495
  push.4016434047.3893954152.3169458504.658396597
  push.217610691.1132890332.3920496761.838200308

  # push (2^141)G into stack
  push.3774381613.2697887140.2639241439.2419218767
  push.16111364.2864130042.1477126338.2330202939
  push.1710140178.3785613242.1278628340.845164682
  push.4245740544.2831738898.40501219.2379008093
  push.72048536.2030351545.536540664.2426277382
  push.1104882810.1812522460.1450059043.2006763478

  # push (2^140)G into stack
  push.3228405253.2904260154.649894961.147244772
  push.4197499478.2904628123.4089742274.1442998264
  push.2182460110.501852113.2657791012.3755043103
  push.1390383913.4199578999.2591082506.792771445
  push.922459716.3353852483.3059868674.1426832883
  push.3827793999.2078345016.871864670.2918539937

  # push (2^139)G into stack
  push.936002814.1759351054.1143478402.2932513211
  push.1261403671.3135159688.1630687470.106067293
  push.934840656.2316700091.2457126757.4182809874
  push.458966165.67657828.2041041920.1381601645
  push.3041398650.3038364316.612250299.612905714
  push.3540255069.2266015559.2472946683.683280022

  # push (2^138)G into stack
  push.2517263576.4042753670.1491020741.2895083617
  push.2241787128.2139579637.3433130217.3211648711
  push.505264395.1050751288.3878113355.770538218
  push.3533937348.3275929629.1852009830.869635757
  push.397252675.999419436.3770044718.3897020825
  push.3700076716.3216215371.1540149976.228142684

  # push (2^137)G into stack
  push.2553685397.3154728855.3667551962.4069506276
  push.2689806454.130000742.3723042676.1354646945
  push.3200544417.625568451.2335576650.1015301833
  push.1203116301.1913917406.2929270809.4071001299
  push.655473077.1614452765.722266091.1198923528
  push.3041706134.3145900667.4111924089.886247057

  # push (2^136)G into stack
  push.450251810.928424712.2703391777.2563140621
  push.53950763.2668380391.671613586.1336192787
  push.3460243181.3844888820.3233286192.3274794079
  push.2200447727.2501315221.1860524645.1009959694
  push.2352684375.2580774530.761384299.21029020
  push.1975555664.2360794075.189404495.3125024026

  # push (2^135)G into stack
  push.3194892716.865637286.2765349307.108164572
  push.1496576875.336569921.825260533.94158768
  push.2363080104.1665441107.4066025515.2109204365
  push.814557639.4075164095.151506914.3802561407
  push.2377642332.3105748288.658941737.3848287119
  push.1230271679.2564534995.1538379360.3840376258

  # push (2^134)G into stack
  push.1580754376.193995505.1955262665.213724934
  push.984937033.1984873974.4263065565.370796384
  push.3804313617.2702658574.2023258108.1830829785
  push.3419321238.900548646.3171847505.1632871479
  push.1049157668.4048606552.3688805333.258041226
  push.3455615407.544460235.4283235531.2808734768

  # push (2^133)G into stack
  push.1641185695.3561105575.224771478.1529809627
  push.1841716269.490644074.3774337637.1617012474
  push.335950514.1135760495.1364328874.3947845556
  push.1442644806.1621608139.140707456.3161654250
  push.1103191670.2249743073.1880978773.2095022150
  push.3624950584.1533899097.2055253179.501738020

  # push (2^132)G into stack
  push.824501553.3874455310.1112455047.1694170026
  push.472671125.1675340871.2177502784.1352462753
  push.552859389.3708339056.4050877314.1674809457
  push.673494723.324283176.3849768216.2055261317
  push.1347529561.2802179893.2869951897.1536817341
  push.3890731577.457292971.40712529.388389916

  # push (2^131)G into stack
  push.1748776417.3010055774.2869800056.504899685
  push.700321620.3453076438.893514690.606089263
  push.4289716226.3144947240.2389855506.2980504791
  push.1177355171.3360333508.1386735730.1955848377
  push.2999380335.3697537888.531073802.2752901294
  push.371645915.2106918511.3162018088.1172841277

  # push (2^130)G into stack
  push.1933354832.1256628273.216683999.704075843
  push.4263111716.487763938.3110394647.554343052
  push.2489532327.770550708.2256565987.956757455
  push.1908248248.2540102707.3781633004.56254144
  push.3312469602.2288063290.3837528054.199189819
  push.2091068109.1644290748.3003631132.121358825

  # push (2^129)G into stack
  push.1927827307.2874280551.112860756.1698521712
  push.3476037993.3054816755.98465182.3952008231
  push.1441664212.2037172208.3858908676.334865804
  push.3736887147.607079206.2057184339.2688177366
  push.3192232650.1975346810.4278687702.1040683771
  push.3594205145.3478231389.1726614777.96238121

  # push (2^128)G into stack
  push.2895442476.2215110696.3024776878.2014863007
  push.988674444.1510610333.995649495.846126732
  push.2241673337.116511288.2963824869.3183902225
  push.3953373771.2194583657.2145614393.2722813744
  push.2143033467.58622371.19193329.435664139
  push.792077805.3849054254.3341320233.2298120071

  # push (2^127)G into stack
  push.3901108667.2978150578.2140484758.2473618984
  push.2903621440.65349278.1738331437.1559951867
  push.1289271936.1411810320.1612850052.604702419
  push.1863818287.2341353042.2056583376.2262508885
  push.671004760.98858366.1319996870.1510381939
  push.2169819339.3466937103.2726892299.3717659155

  # push (2^126)G into stack
  push.3406888139.1797554668.2261003503.1606664018
  push.1398369724.3222974794.722154086.2489917060
  push.1276942563.3085068952.3831705189.742553915
  push.2753827489.4037102244.370123499.1248970440
  push.2867019135.601428751.514669271.52087952
  push.3061577248.1609110525.1318799245.582470858

  # push (2^125)G into stack
  push.4158708877.1241978907.1578478365.3404809454
  push.2127292439.1469394576.593557084.86692016
  push.3992001302.2212804034.2533586678.271287025
  push.3888318136.4226168358.3420073953.679473937
  push.3405612224.2129199950.2840531772.3183265187
  push.1886596386.3920357886.2925563057.605688704

  # push (2^124)G into stack
  push.1047217830.1403044723.2236153565.2418722729
  push.3203704518.257684398.1458934746.3968043265
  push.1142669719.3588887267.3556210923.820810977
  push.2698799511.492065249.990858129.4131192616
  push.4036185603.2849598403.1651538958.3087474893
  push.87380209.4078895453.1639366402.91777993

  # push (2^123)G into stack
  push.2098318341.2576522618.2441984467.3299524076
  push.3818598150.1454216861.2702483629.4212146368
  push.3719755528.2012896675.3479094232.397353366
  push.2445645117.224857084.4135583575.648792613
  push.1115573121.1914720819.3608288040.1836825161
  push.3108746976.2602061706.3020194108.3349542632

  # push (2^122)G into stack
  push.547411365.1875091201.3619462357.4238855041
  push.825432954.3048967297.1526055797.687348508
  push.4219313016.116411704.2226511914.1950195406
  push.2654470705.1709740202.371540969.2783616578
  push.327901448.678003824.3685820211.3990855054
  push.1726591552.2103482580.128723946.1664048234

  # push (2^121)G into stack
  push.2381289541.3075195764.1635941985.3310123332
  push.4164151184.1568903507.4057020202.3371225242
  push.690226041.343580197.721098191.1047965908
  push.2072435197.2947835.1774211538.1275926818
  push.69374038.2358553540.282302138.1488056790
  push.1140001501.286141053.1712538385.2272347811

  # push (2^120)G into stack
  push.3271220055.2814967647.1133228357.3315355839
  push.2687949723.3328125129.1245445180.2378969076
  push.2796857514.634550735.3491693139.1615513233
  push.1407401414.4126905107.2154671797.415548181
  push.1804026901.2277639111.2523741659.2765154580
  push.2091545405.1149321410.2586317549.86331127

  # push (2^119)G into stack
  push.139247123.3077903373.3825563731.3705122380
  push.3792415994.2730560668.3846001716.3385775570
  push.4154659109.3257067329.3752303332.2260462306
  push.3969186073.3749056353.1434534918.1739538012
  push.247352440.3533768284.4074249497.3822515451
  push.627313317.2324693906.838202588.1031242801

  # push (2^118)G into stack
  push.2073564389.2689322569.3665402700.3464248338
  push.1605861937.1181659736.2081551892.3188897533
  push.2340961958.194438317.2612985168.2648169280
  push.4250521275.2361364460.1741518331.3877177070
  push.2721338808.1744567649.2473559136.164195959
  push.2333030850.1231655529.2534864055.4009649657

  # push (2^117)G into stack
  push.3196531058.1243515018.1212118128.1471115477
  push.1629769019.3011898360.685922507.795138259
  push.1115806542.2950317302.2536969431.3057369897
  push.3373095435.2717381327.1644852652.1600431714
  push.3329408522.3747620573.1818430778.4002898834
  push.3248842364.2812450433.3832086536.807052468

  # push (2^116)G into stack
  push.1130219357.1877859925.3926463433.655282123
  push.4092942795.1020537785.1314760526.2471424123
  push.2205536672.1952375961.3659938374.4281808523
  push.393537457.1346087180.3320063252.504978230
  push.397116000.2218639206.2832690611.3617536122
  push.2532399528.680289515.2192312939.2841923315

  # push (2^115)G into stack
  push.2972650254.848827651.3722641795.3902348624
  push.684636236.2852573017.3397940335.2943789561
  push.3097072122.2252897532.2190861257.3980304007
  push.1736809266.3071816178.1369998928.1862080476
  push.3954370789.2853133304.3195712977.3596292424
  push.818765039.3092247215.590017123.3800644428

  # push (2^114)G into stack
  push.3854801163.784757327.3372162199.3147183541
  push.3576771338.3293548109.3584476262.3266612892
  push.1490820778.2524040702.2949045560.767323863
  push.1230225231.145414961.2860643194.2019099054
  push.1167315975.2781533644.3878733038.164402913
  push.1512327570.251195742.3672725365.3383771381

  # push (2^113)G into stack
  push.665353583.127533633.1733343691.2464992126
  push.2388212050.419547213.3634128161.2569079333
  push.1669812522.195298905.2787149535.3114056813
  push.1390154626.2657677842.4005475763.1948752428
  push.362474641.1483757299.1482752995.4289161696
  push.1490889441.83398646.572879068.370871241

  # push (2^112)G into stack
  push.3463488704.1232505773.2007544314.4027405309
  push.1318572848.3513457001.1832881498.2708587029
  push.27736301.4276340165.4084768652.3069971457
  push.2108266665.3112865854.1886498248.149081560
  push.3413780705.1355210406.2383148278.3850206319
  push.120402000.2000063349.472439158.4003562864

  # push (2^111)G into stack
  push.686038994.931702740.147204559.1499823453
  push.2898145007.3566884982.3505683229.3073794165
  push.2359544288.1627206537.2692092431.373969561
  push.3051807764.633861420.1421982224.1438445533
  push.2104423586.1178437464.3061453190.2781933201
  push.2602052011.3107196463.955982553.2298050907

  # push (2^110)G into stack
  push.2427931454.3439410609.2632352345.1470675854
  push.4044202835.2978366651.2847348874.2617971131
  push.926218107.2885986491.3903999038.755000237
  push.1010530692.27495705.3733590032.65894539
  push.2674156217.1469372807.3351161501.3225878609
  push.2488695529.3663340612.9878138.3784070159

  # push (2^109)G into stack
  push.2311622675.3444708254.379693928.1465054857
  push.1923487622.241580526.2065847989.1560490530
  push.817380573.2501509175.1123613530.3648807890
  push.3705085246.3045889630.3524321254.1519051765
  push.376017388.922017845.3585836087.3945654906
  push.3493933080.2766792520.1426639440.3277093746

  # push (2^108)G into stack
  push.2629773589.3749041910.3364031030.298369384
  push.3635404226.2982070172.1443269872.2651068343
  push.816218342.3528049166.1917761070.1501245270
  push.4214490925.1906562462.1952197157.1599005289
  push.876509780.1085198534.1102146832.976721406
  push.2484169075.4220766875.1063432399.2900683076

  # push (2^107)G into stack
  push.1795711342.2558634291.3079953412.4257929129
  push.2662587842.1254455581.956335357.1548538666
  push.172774928.4063155861.1538650176.3299763994
  push.1321914552.2228030794.1514648476.668136951
  push.3732826757.2276102682.1379325263.3526236643
  push.792361920.300399448.3528400478.669124404

  # push (2^106)G into stack
  push.2108188033.1928983695.3942998603.2346131303
  push.3379731892.2891491678.2700850407.1579808355
  push.4155118099.2961520377.2637254360.3745765835
  push.2398274640.4173909105.3120283902.2576244133
  push.4022809935.742879883.1502188510.1162142104
  push.3003676891.127990804.748619701.2976996008

  # push (2^105)G into stack
  push.3099910167.2061596049.1531609397.157442371
  push.1025218763.1299560201.4136975719.3097106974
  push.1216944309.1264231549.851526522.816789900
  push.2145126067.2053539985.813270778.377203065
  push.2134278040.460748037.1514746734.2785187105
  push.3620512607.687083731.3852994955.2277125234

  # push (2^104)G into stack
  push.3249571275.2761391695.3028897536.1645505824
  push.11268439.1229227033.1848823491.3049245032
  push.1212688125.3562444091.2355173829.2489267689
  push.2104758122.1266914505.687082796.2186987764
  push.2749955548.1442125546.4236912991.100200505
  push.2257695780.3150774005.3970001509.4168348331

  # push (2^103)G into stack
  push.3662249190.2283137214.1717416594.2138980524
  push.2704435240.1865777396.3198177515.1845975775
  push.923748974.749916797.1405785536.3348961098
  push.1101723642.3570553653.2738459880.517023977
  push.3177407800.1971593846.3430148714.1232847134
  push.1293697378.3746003679.4078024924.2261132022

  # push (2^102)G into stack
  push.3946196921.66840545.4253794039.1624065450
  push.3863390043.1626757698.3718359335.286443397
  push.3944598048.3526935232.1927944567.647231806
  push.1500371005.2438658963.3421798001.3568821682
  push.4158117360.4027692640.3191217970.2629236568
  push.1466288375.908855089.3324924579.535293732

  # push (2^101)G into stack
  push.2411874093.2682826721.2148243734.3143472347
  push.153990694.4014110973.1595414016.771844077
  push.288620013.3265472674.695666818.3889255464
  push.3569846179.679436820.281349768.3622374413
  push.3834948292.3280602991.1935020228.1650621784
  push.3611366268.1831994131.1719041647.1954708515

  # push (2^100)G into stack
  push.1275109758.885852953.2928852466.2936388372
  push.631342944.2179504382.237867736.1445969530
  push.1053455946.457787251.2252553626.1383432065
  push.1861735910.3406293307.3082336510.4276027709
  push.2558794243.2635660306.2888006731.3211085519
  push.2201659057.2876929364.2065398710.2963265494

  # push (2^99)G into stack
  push.3953355374.2219686724.2563637000.1606929534
  push.654318585.755323251.1176107628.1969179532
  push.3962023612.683497106.846149992.2303458042
  push.3370732814.2437793624.3666376174.43863592
  push.1111412495.845861911.128539704.2889741525
  push.924747330.3934194257.1669859148.2436061424

  # push (2^98)G into stack
  push.2809668745.4058896609.162130346.1119936282
  push.797713258.2183927562.2593236792.2160162012
  push.831563243.1555017039.562259675.705342321
  push.641339019.3003232328.3827500595.25658289
  push.297324649.2974671270.1457728066.1138090666
  push.2215681696.1833117381.1052435029.3690170515

  # push (2^97)G into stack
  push.1106650743.3907097879.3656920813.2999104053
  push.2544146444.3723219934.1779055491.4059355383
  push.4011558202.2993173694.1143099324.3905370773
  push.4008534261.3921488869.3560147474.426442631
  push.3531792766.294827065.1721065140.3213216935
  push.1231664109.567958979.3727734293.2576619508

  # push (2^96)G into stack
  push.999638617.1559089557.3770806934.3380534272
  push.4111735944.102269432.467635238.1577869094
  push.2482778285.2301672744.555633540.2349740190
  push.3530652661.3557735053.1335483736.3474816117
  push.1983935828.3166429406.2672547269.1527713947
  push.3946426554.2808424719.4170712750.1543904120

  # push (2^95)G into stack
  push.621790620.1936802240.1529138668.833231145
  push.56821615.2081436588.1093641110.1777861755
  push.3423975001.1508641478.573623603.1974865253
  push.290607056.2277295677.223961665.2214882506
  push.2065572673.98781838.235353061.3408575061
  push.1517715605.509324179.1071779425.1403370981

  # push (2^94)G into stack
  push.1084586120.702283184.3023486909.3570162450
  push.1868842473.471489517.3240431408.2117560880
  push.3293037610.3985845895.3427308739.2661027154
  push.672412297.364455713.3151016307.1827031164
  push.1767690935.150380127.1706638548.1772638692
  push.2330202798.3793670437.1666342885.1131528605

  # push (2^93)G into stack
  push.3343899360.3992358927.2866847288.2632213500
  push.420949569.4247993496.3537190828.1313824226
  push.3898364616.610482295.1020747179.2757049303
  push.3881390677.2376572117.2628585139.771294546
  push.1990480316.3108426184.484457080.3469856670
  push.4106358830.2661783832.3776584720.4129318781

  # push (2^92)G into stack
  push.151730902.1032282314.2922436459.1396993633
  push.2339612881.3498123033.1366761769.3345003208
  push.2657196510.3644453791.2486409935.4039281400
  push.1152815177.3230722945.2581399890.4065463356
  push.821705374.3015236194.919943740.3722401695
  push.2095404446.1325909471.3727571052.929126487

  # push (2^91)G into stack
  push.751466010.2811067554.949527296.2283947930
  push.494832992.1492556077.687268660.1023426763
  push.2718144955.1650651555.2495057436.848817746
  push.266201960.3537383044.3505185577.178631504
  push.2487884998.828076153.310132277.4117694234
  push.3453202519.1562435575.4285537762.2659185081

  # push (2^90)G into stack
  push.1834901373.2617710988.668444809.3960149915
  push.2076799404.3271195648.3281460968.1433936315
  push.4124851481.861020709.2520462156.1798162091
  push.1160300146.1726410591.48937343.577065003
  push.1157956470.3318692241.1442303333.43200738
  push.434197593.1436116311.2289836964.3641424224

  # push (2^89)G into stack
  push.2420875917.3091767772.949655978.2440506923
  push.2748829623.455193681.678110168.3855606563
  push.2519679746.4249792059.1555524640.3524697156
  push.1845445059.82835290.3114620409.2927720915
  push.963493471.68985541.1043941206.3626189511
  push.767734023.4279048740.1157535992.2171654129

  # push (2^88)G into stack
  push.2957471010.3524468751.680681556.3448300765
  push.2830978373.3036320741.3540631672.1541874332
  push.387208902.2980155850.2638774085.2147884509
  push.2069884065.2871037537.4186082752.2270556422
  push.3886166885.1656273194.1512802251.52274770
  push.2364000695.2236109412.1337081733.3263008144

  # push (2^87)G into stack
  push.1152503915.4109489512.2705616261.150689496
  push.2011046877.707352156.2835874648.1958572988
  push.1964184935.398013355.2989388614.815040508
  push.1292508732.2351765213.4022486584.1819066919
  push.1946758545.2978678576.1568487482.2887488155
  push.2054128079.2230972820.1127625440.3877806084

  # push (2^86)G into stack
  push.1636722311.1547049913.1160754888.794510130
  push.3381405462.568768815.1920956361.1188271888
  push.4043651060.3495493696.1252064033.2559142819
  push.3892985643.1103735805.3157116484.2949779894
  push.101061509.1886272056.3632026625.2035677118
  push.45809781.769357890.2250396233.2343497395

  # push (2^85)G into stack
  push.1767575569.4243592530.3870425872.4279724803
  push.1045002525.2930492521.2806553287.4230456169
  push.146356977.899036247.280471051.1508419602
  push.22096684.207965602.4210943564.3340997638
  push.3046696047.3996587108.4112183189.3505849815
  push.2238069073.1972041509.3614873861.3435216957

  # push (2^84)G into stack
  push.2226359467.1234800008.836810939.1007386657
  push.4142097638.2354980066.2049983884.1728408872
  push.2044341764.1844924113.2633437963.1484033813
  push.1085851922.2152848749.3310053087.1878276787
  push.316863046.2372232826.538406169.3550510846
  push.2930319825.4834939.4228156803.3141696362

  # push (2^83)G into stack
  push.3916810877.1299080966.4267126602.4134597698
  push.1283118447.724071994.454377685.2305006999
  push.1452441665.2147736672.3161960445.2412667335
  push.1490055384.2096715467.4118188432.2731821351
  push.1497500411.1425757150.1279677033.1216685214
  push.1043810021.167539280.297646337.3690202998

  # push (2^82)G into stack
  push.1710544862.3146657047.2110032199.367863398
  push.4190172805.3941131213.3879978320.1209185419
  push.200659473.2013938264.4029364271.1302990387
  push.2733669384.1808969764.2545608112.419424531
  push.2792497975.3896150695.2601588359.2455467494
  push.4256535327.971953462.2690202946.3464971495

  # push (2^81)G into stack
  push.3579288157.2293609170.2944134579.1851177124
  push.2313100600.2724345757.1795155454.286184790
  push.3688744661.713843288.4211114079.4262002280
  push.471537044.4051323466.1851461799.1358762056
  push.3978895079.955654321.1597674889.2239392442
  push.965761796.909881260.423180520.1260309859

  # push (2^80)G into stack
  push.45045079.3313636243.4065224498.2317706449
  push.2536797638.3761558151.361836687.194462540
  push.542993901.1807479648.846889975.3208567005
  push.2437021524.2144223304.1558569048.2321955205
  push.2571300373.2854085925.1206048166.434005132
  push.2147183140.2435889009.1124217110.692822312

  # push (2^79)G into stack
  push.3168557127.1599831230.1908704830.3582279181
  push.2968239279.2710169141.1002962628.2026748420
  push.637579457.1426022496.3552004709.3521666672
  push.4225279449.3309431556.680901258.3062345760
  push.3193794080.3956168732.2400363838.2611008937
  push.1873016892.2908160895.1761588077.4130817054

  # push (2^78)G into stack
  push.2170317792.1249770147.1610165525.10143291
  push.2213739025.2367077836.4120472051.60025773
  push.411715520.2447522776.2310497932.3781337653
  push.1157975516.2479831368.11565890.2480571873
  push.1296809448.1251514215.932999437.4048013732
  push.2955562702.2067107445.2515787528.3157323974

  # push (2^77)G into stack
  push.3796516846.457878672.3796078088.133390108
  push.751714457.2921609162.2712683821.2673845263
  push.1900090986.2881736842.377646265.755042002
  push.1841547412.3167977330.844458091.354057312
  push.3977335559.438330674.3975547620.3361329560
  push.2624800978.1576084960.136666640.1429498917

  # push (2^76)G into stack
  push.57675626.3996181708.447113519.3325652353
  push.1620584395.2741203150.830642661.680216962
  push.4022591230.2839066415.2079673701.530147430
  push.2418964509.3263182117.3449144896.2671288163
  push.275957873.3947229107.2154425801.1891283735
  push.4083309621.1354160305.3345691164.3173403818

  # push (2^75)G into stack
  push.4025102189.3002044362.2610939516.968481390
  push.3349180874.284708997.1030669352.852011505
  push.1794423875.3659870744.1347323782.30742182
  push.3081651707.1870075313.4214507112.2948708405
  push.927854096.3419149860.1413812208.3997738346
  push.4092388786.1074258664.4212587402.2823109854

  # push (2^74)G into stack
  push.3765824723.3094155980.3563201915.2442217768
  push.1604304220.3213219414.2334978195.3552447160
  push.1442184620.1745833116.3342210715.686503619
  push.951728142.863796929.806408025.1349686452
  push.684080650.3020149742.2575799977.4167697701
  push.3889031629.3548347718.1528378206.1360271360

  # push (2^73)G into stack
  push.2021208394.4189291889.223928975.3084231900
  push.1001075623.1476446167.2116757194.657020538
  push.3451635145.3395420679.819372650.3183326785
  push.2422798041.605859356.2809127354.1885077772
  push.1673473568.3246396932.3233126230.3694757456
  push.3042011166.3980881876.2400451195.756850751

  # push (2^72)G into stack
  push.2686476327.3170189344.2469236587.545799926
  push.2709577220.2257654816.2708829808.3649193325
  push.3919016294.1315610693.2701541236.3111362429
  push.1808104770.2360198333.2482638774.3240947624
  push.126499372.2858614875.1820339848.3533311886
  push.1966465813.604647178.4196749756.789736869

  # push (2^71)G into stack
  push.346949059.3577731365.3661011466.2278886097
  push.2616179923.2620062287.847843478.3315509977
  push.790911321.1966535291.4197745626.3797023080
  push.521573832.2073770098.159582541.1578764349
  push.1320712841.189371538.2462092567.415180916
  push.2407095031.1420560899.2675219549.2301384663

  # push (2^70)G into stack
  push.765489644.4204278716.420831615.686289746
  push.4147811819.3812981490.2207581564.3186548262
  push.3403575861.3414551909.2845409721.2687779374
  push.330852323.2165209260.4186224570.1059145280
  push.115473950.822477978.1055291546.2879732828
  push.958053770.576130631.4155419959.1787957888

  # push (2^69)G into stack
  push.193950791.3758628274.1961876981.1054091348
  push.3816305847.2405711196.2636355792.2128378950
  push.2945370933.103726037.2975308047.1661158929
  push.4108900642.980052233.2348361273.2734504617
  push.2488278332.489109384.2830315848.916058668
  push.484431564.2490928341.1874515438.4041735840

  # push (2^68)G into stack
  push.440355473.1420892297.4060729519.104154129
  push.220540804.3850811378.2558908538.2436681557
  push.4280960021.1956846976.150088583.209108884
  push.1165237124.1570111634.1954462842.3022480631
  push.614633720.3576486872.1822861865.1460303818
  push.2102862854.4271041081.2471590568.1403471254

  # push (2^67)G into stack
  push.2790742442.1647864019.3653429430.2226224714
  push.2989216687.1806064091.1180412485.3220192391
  push.1445606176.2552927947.1547874355.466254502
  push.1226218693.3127214280.3888602397.854560206
  push.32975235.1901170796.2306495411.2596063487
  push.3993810553.3730211394.1937555018.2236390468

  # push (2^66)G into stack
  push.2195337937.783308501.2289061228.771833956
  push.3059474998.1121901657.1019618172.3863235390
  push.4002389836.3127936607.3603937497.3005053939
  push.2969337763.1464112746.771926169.2799799103
  push.2548570297.4281812028.48393916.3998577308
  push.2748119327.1620764752.2946535205.1489292690

  # push (2^65)G into stack
  push.3781102756.230988237.2945637147.2174577042
  push.3219804275.4123662966.3658544901.3293455390
  push.2159015885.1044022165.2815084326.2219235391
  push.858162732.3912755742.2883780014.189242651
  push.2780136961.3904655003.1587424501.2652816032
  push.1750856401.854713088.3198409211.390915607

  # push (2^64)G into stack
  push.4286314443.881422584.1757972216.2844756714
  push.627807224.1897164151.3998369064.3248241325
  push.1294275752.4173584190.451385138.1139812898
  push.1047474443.2470633620.2465897793.1686758339
  push.3051371635.2724935968.2360519577.3060172390
  push.2105582013.4028571826.1398680223.929018991

  # push (2^63)G into stack
  push.3287381371.3514713235.3266414667.3832987379
  push.20693004.2502528377.2409187345.507577010
  push.805807871.3470821989.2554015457.3847848274
  push.2054686107.245411910.3303524564.1966565957
  push.3118428633.2959647369.3170637709.3372563825
  push.3469890068.3946394267.511676447.3307380463

  # push (2^62)G into stack
  push.2198053893.4122910262.3106106528.71799408
  push.4092185250.851266439.171714579.3801417272
  push.83989058.2924946953.3482638132.48594429
  push.2873721938.3342064369.342108151.3423621429
  push.4176795803.3018042694.3874520595.1890360578
  push.2769350307.3068640479.1693301391.1534698952

  # push (2^61)G into stack
  push.1766609812.2608568819.1370861363.3300260032
  push.1042186249.3945050167.2479429112.3552045223
  push.2213231104.2298001093.1842716286.1070815964
  push.947249811.2187459943.3764781689.1174693308
  push.933696081.2439244851.2615315712.913884404
  push.4186698633.1481211838.1371610537.2327704874

  # push (2^60)G into stack
  push.726014853.2559541446.674293275.701555975
  push.3061487694.4147612975.607809775.1089287651
  push.1645294878.997276205.634264882.3408792906
  push.2732399273.1682772479.3396290441.2834527101
  push.2658904629.3454091496.2807737979.3100099893
  push.3850750957.2196229415.1401630839.3212445147

  # push (2^59)G into stack
  push.2942247545.986635644.1051121938.4283439008
  push.4006816892.1268487531.920457673.484783460
  push.12096429.937681154.2410134947.3960205325
  push.1391447411.1514973159.3925070901.3714989833
  push.1827635415.4205372956.634973838.2642160681
  push.366266016.2179268796.3962779698.2869202920

  # push (2^58)G into stack
  push.3077906979.887309179.1155359897.3296386596
  push.3586907049.3047188635.3344388971.1349055725
  push.2651068967.917918296.848198734.1676871213
  push.2008467572.2467008527.10799570.147667135
  push.2460693588.2408572052.1048380281.210768845
  push.1595555664.560100810.3622299272.2184017943

  # push (2^57)G into stack
  push.452052554.2695624202.2696142657.3325074837
  push.1318738957.3769892322.3771464647.1936655279
  push.2410497750.2320046195.131304288.809022823
  push.234978893.3854381809.3355693258.2862004276
  push.2335629766.3192022847.1485647499.4114042288
  push.474582552.145974266.481590094.3890661320

  # push (2^56)G into stack
  push.644457494.294749723.3842984229.3525701427
  push.1201925116.1539169354.935101213.497800014
  push.1590292234.3982591116.2110200815.2335589337
  push.2006818131.2226054321.3408831415.3550900475
  push.1654467477.22959250.3011660105.1915149317
  push.3357835888.1199147507.852159997.1607276660

  # push (2^55)G into stack
  push.394837729.1010332507.3156944571.2143230840
  push.3052473835.3237345988.4223460066.4156192200
  push.4001034428.3315474656.4050303753.1689644323
  push.3520181822.1626008312.3556938481.3059913474
  push.2405949463.612439938.1198226394.1290408248
  push.1337257023.2097803714.2186964309.69574727

  # push (2^54)G into stack
  push.4086335455.4174644744.2418015280.1854689529
  push.3127854526.3935157653.3975445554.3336576001
  push.4026394363.3678135263.615594919.2550570861
  push.1107823231.603239084.382051012.3624661940
  push.3637624183.1559805603.1333100442.805741554
  push.1264250571.2358786109.267699852.397371568

  # push (2^53)G into stack
  push.3507786090.2609550599.2646253414.3308469527
  push.3780185729.913878096.122715666.3011542514
  push.2542825453.918556626.2904917918.2336285602
  push.15182657.551915631.2262568712.2017605335
  push.1579678045.2422919622.4285065105.4210704817
  push.4206239977.906494042.3036256728.469383574

  # push (2^52)G into stack
  push.4231928456.2296171318.3702809920.753297862
  push.173736952.1293986505.2136282037.2743837395
  push.2544117520.3785075401.785316795.3331397673
  push.876551824.4253025791.3370128350.1665076137
  push.4124258349.3715133632.3652150622.393900791
  push.1515776755.1670037745.3655800101.427888614

  # push (2^51)G into stack
  push.615802927.3298463238.2997144325.3167811090
  push.733833886.65213119.1910805953.3269891206
  push.2604912474.1817185359.1717463059.1951395870
  push.2281282091.2471339352.3656431778.350550997
  push.3056134945.4229139886.939290135.51360330
  push.69881060.1033278612.3851756353.3366537903

  # push (2^50)G into stack
  push.789267740.3629968881.1664142384.2098587670
  push.2832581714.1028213555.429877432.3895211061
  push.474094330.2176442027.1386272556.758145133
  push.1215912599.969643301.2262906276.3714363160
  push.3920810919.2647092939.4023234298.842016212
  push.3890280116.2092183917.3197959608.915862962

  # push (2^49)G into stack
  push.3441957840.423571584.1961125286.3816809819
  push.2126009920.674356086.3726766216.3234863489
  push.2449763730.2394496880.738155419.4236696056
  push.2832582698.829271886.3153051282.1977879392
  push.1051337337.2228308901.788313012.1252691825
  push.3027343521.290966086.2290059468.71556765

  # push (2^48)G into stack
  push.3356353071.3616840600.1435566412.1090990745
  push.1887798308.15330796.2072835962.3936084927
  push.388672815.2835872507.1352073514.2642551675
  push.865261756.628467235.3534700632.257948540
  push.2355394185.2542849143.2322905173.3655294323
  push.3380407829.1964231819.2385143312.3590821817

  # push (2^47)G into stack
  push.209477031.3093707709.4202746756.3588225431
  push.2626062656.2969960579.795045560.1582482099
  push.3098282248.2375464833.2753068900.3096449706
  push.1171302602.1894168152.3888981693.3119052069
  push.3679292966.3415736093.4256668678.1507188281
  push.2537906632.3870987310.896931191.2693046389

  # push (2^46)G into stack
  push.15108816.2267610399.3891474949.1238181223
  push.1147770607.2748335909.2770239403.2976132499
  push.450074572.1586083369.1057828945.3887057063
  push.2124744571.2346921524.2086647707.3681689303
  push.2399604296.182328218.1760516211.1873185083
  push.3539050180.1781066486.43373161.28255872

  # push (2^45)G into stack
  push.4064832735.3539470659.3958207760.3912356637
  push.164251623.2078471405.3328639016.3001905155
  push.4036200447.1482344847.314907145.2151045431
  push.1911268241.1480569170.3584517467.1841579576
  push.1404132585.1151185088.767324933.2333244476
  push.1455456054.754945335.676985571.2914491643

  # push (2^44)G into stack
  push.1090101602.1205871786.654432486.3225965430
  push.3963186459.1312373161.3916546418.614671223
  push.3673851801.1115410695.3724980483.3822720403
  push.1359135530.3000899183.2440466837.70908625
  push.1690494135.638546508.6924999.2818818501
  push.372104104.1584408275.2944612036.1338203853

  # push (2^43)G into stack
  push.1076151823.1869215212.3007531426.1975684790
  push.3012532821.866724528.1718901424.143820326
  push.1219396925.1173748168.2109412734.2759454143
  push.810391584.2694572670.3103996657.1689488452
  push.3709863201.3778143269.2896263906.125746319
  push.3919125906.4277306591.3238831685.2931784487

  # push (2^42)G into stack
  push.1279883447.1293954212.4182039084.3246576295
  push.2595402405.1065652033.3754886750.2642017518
  push.543015909.627283566.3757523287.4037047826
  push.1916228510.3794860938.87259858.2134340506
  push.206309447.3076885834.2680212206.2389366466
  push.4064955947.177800874.1883664398.1464320399

  # push (2^41)G into stack
  push.2538873761.635116843.2411495690.1743653792
  push.799616421.598310406.3892303814.994983060
  push.4168957777.2777478178.2762588939.635394239
  push.1070043390.4062955027.3408978548.2630134036
  push.3880482791.3490715278.3102890453.1121678620
  push.1454822092.3285728435.1378742502.2286879824

  # push (2^40)G into stack
  push.3708375435.2356284909.871247942.462792261
  push.2299504905.3491302099.1827124646.3927673984
  push.975509937.643555121.2995576306.2581588267
  push.1744963383.712453441.858381141.386431441
  push.2835085961.3716299352.237555353.1921923892
  push.1819790809.2178166904.1239928156.1080120604

  # push (2^39)G into stack
  push.1760021538.1246477538.4020221089.3103984320
  push.2188414816.1361704363.2222248250.3334053791
  push.4224874170.263556204.566886706.3338797609
  push.2461586542.2413395531.3976053596.1633706202
  push.1926013630.484251948.415199817.1631103609
  push.1391526698.1921207145.2742955422.1535773226

  # push (2^38)G into stack
  push.4190278158.3792837316.3788873439.820867659
  push.3025821908.3300532140.1772759211.764207700
  push.133427612.1750317322.3224897846.2169197977
  push.3967207165.3274186100.2836574355.503213154
  push.1505148098.1765012388.2178624482.91443771
  push.2831523941.731204757.1174842552.849958179

  # push (2^37)G into stack
  push.1377451392.1184542162.1605555025.2571230163
  push.3169832676.3679023124.3388703121.123804462
  push.723606342.1797479561.1778447394.695004162
  push.4070648574.1483168998.1858512776.1982122996
  push.2194757856.1271759781.3812828764.1428180415
  push.732008708.1643820701.1273211715.2564901160

  # push (2^36)G into stack
  push.2640778917.132490793.363394045.1911428880
  push.1223845880.3053283226.752051814.2177297027
  push.339227243.1041902608.1737746765.4234871778
  push.1971945276.1246327437.2318244733.392695281
  push.55085673.21751716.838984379.4037945171
  push.3739923889.2737757661.188520867.3134731939

  # push (2^35)G into stack
  push.4240540981.1038306641.45839330.3995232274
  push.2166099010.193908061.2535080685.2680547871
  push.4246543775.1889068112.279926811.2028601906
  push.374462255.1803158578.78057496.2160895405
  push.2900988164.93926751.2043608283.742260490
  push.490073796.82576743.3736828200.2440595515

  # push (2^34)G into stack
  push.3447163093.240345314.298868343.2854454384
  push.2772346691.4292677869.2712131591.1034868192
  push.2729188126.1484193751.3310765108.2566167763
  push.3902178200.899397401.1257173239.3980824920
  push.1283932233.3511547519.3538672689.11314807
  push.1163623805.1948397494.3700537393.849137747

  # push (2^33)G into stack
  push.201781902.1879208033.366368531.414570348
  push.2725203297.3303225153.3960676961.1547404409
  push.1202035494.287739238.571740796.3299463628
  push.2323633777.1732115118.2807546675.750641548
  push.155733807.2116491013.4173803077.3196485543
  push.3855932326.2341890268.2652231975.2356926667

  # push (2^32)G into stack
  push.3350578690.615105488.1907097206.2601362594
  push.1973164475.3378236195.1258653325.1576379166
  push.2683751654.671746037.2154085447.4171665086
  push.151326720.2244384650.2564835854.2688349067
  push.129624219.339457830.630538015.3385965042
  push.4243021446.3930248375.2200770692.1999075395

  # push (2^31)G into stack
  push.1537097577.599500388.3188112682.4220919932
  push.3365993235.3060310166.4166074507.1130079062
  push.325213743.3456678408.1460551922.225117710
  push.1295148995.3086274105.2998223779.3644828036
  push.3425771566.128473889.2514510162.2442663658
  push.3036851339.1923498161.726990368.3678600041

  # push (2^30)G into stack
  push.2711196609.1792639260.3615873011.2733173106
  push.3427855566.1602100217.508036311.3522943865
  push.1054087177.3996065130.1153902543.1308735309
  push.3083051596.3455229223.3806463300.1067311995
  push.1077170122.2606831513.3331061849.4085012881
  push.2259138269.1269082684.3518089684.1408579258

  # push (2^29)G into stack
  push.1746220919.3674660492.3263188964.3563175696
  push.3606314372.2559042446.2601336813.2405628422
  push.605916942.2000138665.1814215102.3563723739
  push.3906121869.1890695637.1473469579.435252301
  push.2719014285.1389965106.567774045.2983137038
  push.2660485347.280790875.789341098.1784021326

  # push (2^28)G into stack
  push.3032668618.388189360.2716077947.95629646
  push.622684270.4179996509.247155377.2968359192
  push.3716024826.1233465505.1748859360.1474305547
  push.2736240994.204620363.4198863172.649949720
  push.3282419999.3093287994.2398937564.1992150115
  push.2275040839.2829168165.3451153835.873194490

  # push (2^27)G into stack
  push.3694830907.182787795.2738894540.3036843795
  push.1095887046.4032951432.4174004315.2872058775
  push.2972896576.3146916577.1550249297.1954517472
  push.3399380590.1508547522.2142859827.1489169666
  push.2111777238.2607907222.1236593688.3793816203
  push.2242801002.1887372273.2261272626.697912920

  # push (2^26)G into stack
  push.352153844.705577837.3808872676.1102893726
  push.2465044802.1876314770.1618583023.4201745150
  push.4188015931.1170021009.3217613595.2766194525
  push.4180617317.2240474317.2237465588.504535800
  push.2470191657.1513487983.418351892.1119652998
  push.213605742.3333096384.3217865903.2426287159

  # push (2^25)G into stack
  push.2085282704.2941705734.1694467478.2630684944
  push.1327291043.217154376.1729612892.1565252408
  push.993766506.1627295219.1554902617.629484924
  push.3992782803.3093793053.2242553943.1562562381
  push.1816515825.3248445909.1011391674.510317626
  push.3879371210.3081527059.1276965238.941234928

  # push (2^24)G into stack
  push.3148124861.38952013.3121562829.308148021
  push.2813675221.1303543082.904071087.593415411
  push.266020797.1033180496.3919775067.1367605140
  push.2295255712.603738871.568714404.91523412
  push.1473277186.516789549.1236670775.3241023408
  push.3595860352.359733990.1998376628.336093607

  # push (2^23)G into stack
  push.128933005.1843219761.3178323727.3041293948
  push.783037882.4180090065.491508136.3319848281
  push.3624473750.2952716925.3626123566.3939227508
  push.2240713870.2366730935.1092671880.2600008203
  push.3602441774.1079003990.2719087255.2889008302
  push.1699129945.4088667013.2221645807.2263082900

  # push (2^22)G into stack
  push.2164444161.3254136612.31091988.2657278511
  push.2825705970.894983440.92604647.1669533578
  push.2936379902.171638723.2240304277.1414814578
  push.3482569193.4239763898.730303841.2872292874
  push.3426423006.2009945648.1276908713.1016628304
  push.1226687375.3732033419.115004589.2189803440

  # push (2^21)G into stack
  push.3364027452.3000806926.2213349683.2229151259
  push.672509300.246558793.1700474509.1043351151
  push.672254616.2507033870.1329846737.2584119235
  push.1725042470.1354922919.3698165144.2935842768
  push.609348168.1469485514.3737623623.3981176228
  push.2650406160.778134635.1314740119.784819757

  # push (2^20)G into stack
  push.3427443063.2860962761.2699599650.3132555353
  push.2956912514.1820976906.2858508979.3798288869
  push.3416354503.3396109829.1820376351.2676305039
  push.3280520326.1791571865.3528296494.1537819910
  push.4226202437.3446455985.2921220771.2720721362
  push.2439832784.2241875899.2018568169.352425916

  # push (2^19)G into stack
  push.3782714730.3644631196.696007875.1704138562
  push.3064552040.1970812605.116048174.2935924752
  push.531073595.2716651538.445341804.376382049
  push.149245088.1699366357.643757068.1988497688
  push.3876605360.3717017061.1792035884.3658180245
  push.9689525.3726457710.3737655682.334914726

  # push (2^18)G into stack
  push.2881216904.2602803124.1931543326.3882383748
  push.3490219988.2976614537.4225664693.115915521
  push.3257309353.2970517532.1632831680.1663479127
  push.3293155039.1591403760.3205355140.3183698196
  push.840954074.2449757058.1379796681.3174738595
  push.2888001427.725880224.3916942879.2971492725

  # push (2^17)G into stack
  push.497957642.409392798.3397792514.121517090
  push.2208886148.2594586529.1793088961.1024640302
  push.1301017829.149015777.2073426303.2696251698
  push.962013080.3029188982.1759358514.2260129774
  push.1176472168.204738826.1035950330.825829971
  push.1995151119.2875022893.4126873466.1465695309

  # push (2^16)G into stack
  push.2933503355.1150771951.1010323564.4167348229
  push.1982999048.864871296.1544240413.3816477355
  push.2713543322.2042185290.11499049.3239708649
  push.2596216462.3792018179.3285818282.756956372
  push.686489741.1349583.1129616092.580263380
  push.375341506.1929451238.2202716750.3251649326

  # push (2^15)G into stack
  push.1503048786.2253075200.1799563349.1602884526
  push.3363774533.3802573440.1406190571.1439164895
  push.1739612738.1344947376.1943259727.3984490149
  push.3711094428.636908438.3875751458.1202552066
  push.4131906068.3317883546.2714198383.2001977271
  push.617143603.3914299867.973684328.1024725525

  # push (2^14)G into stack
  push.117479910.4096465048.3748213334.1414641509
  push.1080456887.2494968804.2088850801.3335786921
  push.1913316320.3313693373.1790647232.3862320970
  push.3411429597.4034043018.47726183.1219468969
  push.4118717561.2897385214.80459097.551686853
  push.3428312537.245232532.159612118.1418946393

  # push (2^13)G into stack
  push.1071817889.2535981847.620428557.2926284367
  push.2950014256.1234570333.3441862140.1884477444
  push.355318376.3348606533.26758351.110106597
  push.2137706936.1237760563.4027551371.959797812
  push.3801555833.606845849.2896886910.209612018
  push.1036223050.951134442.706521218.3302693766

  # push (2^12)G into stack
  push.2453519330.2007784047.757956299.3351232194
  push.3796496381.3873867960.4223744311.3807558599
  push.886461396.99491659.2951913132.3050734995
  push.113833265.53899627.2198645230.2433783572
  push.2717673753.2030337976.2469272834.1039085029
  push.2920809398.4247331910.3148245799.620054778

  # push (2^11)G into stack
  push.1092243945.4177617868.2411532286.1464808813
  push.3312066550.1673939901.801066763.35873813
  push.3268773358.1186072370.3532693070.1754135012
  push.3799491819.2512492662.1561581297.1582858834
  push.4075424326.2861468264.2826966158.1674842800
  push.2610818279.3536713782.3912065715.3176669737

  # push (2^10)G into stack
  push.3053614628.223221158.4248989966.3480032262
  push.1571409508.3157339099.2485825496.1088857644
  push.2186442817.2178970503.2795624768.536421003
  push.447109814.1113377320.2767739398.2329310267
  push.2085994872.3160934912.2377651848.3919221800
  push.1427940723.7428921.1799368815.1571365520

  # push (2^9)G into stack
  push.3366549089.3619273576.2503475385.2238764922
  push.1237597377.1596640373.1776096883.628236075
  push.3569717.73386979.61235791.1396660245
  push.163567449.2607846704.3022796594.258440691
  push.1757200845.1579712529.3709155749.2490525753
  push.1271163719.83748696.1243934934.1725045020

  # push (2^8)G into stack
  push.57736146.2003662631.1532849746.541811467
  push.1019743195.3715983474.2560515167.3916021123
  push.1682858830.972953161.3109216878.2729081681
  push.1416992387.4248334506.2282654796.2958880515
  push.3463148253.356534422.2789290949.2123776697
  push.3651363183.836005144.3738400743.1202071911

  # push (2^7)G into stack
  push.2281031812.2248683113.1719068146.1024688983
  push.1975931675.1402488620.1339206136.2915828072
  push.2523154591.3118632178.936853110.1450840467
  push.1582281044.3267081159.106787623.2219015849
  push.633324806.2889210420.2971080174.4043757103
  push.3457405453.2387867469.2030555195.2314634766

  # push (2^6)G into stack
  push.381366728.4201301544.1849045455.3628236831
  push.970167946.40178237.1094968387.73287484
  push.3313277609.3625027388.2823865272.1607317395
  push.2452202656.1756523026.2650272126.3072086044
  push.4256731487.1901518509.4257856434.1704010953
  push.1026205514.557988677.1811735612.2526085733

  # push (2^5)G into stack
  push.3904370366.3487403656.3148714344.2420508884
  push.3573223072.3532032629.752551711.4091923949
  push.1099166461.2723866852.45181322.3558355291
  push.3911776812.4104022181.1400047881.2096740742
  push.3275764810.1859615953.1889484915.3920395556
  push.394522011.3501175395.1444831051.2712217010

  # push (2^4)G into stack
  push.3510650449.435018679.1039046904.1998515401
  push.1979715416.533757142.870983663.3321979243
  push.2230544006.4199600988.2877849544.2612568121
  push.702108359.3709866115.197069253.2527134796
  push.2081094477.3806951506.3254560673.721367143
  push.1231228381.1813340316.821340817.722512669

  # push (2^3)G into stack
  push.1658585184.2503743746.3205251131.4072748768
  push.2637562955.3679743535.2221345410.2840325697
  push.2749723167.857329376.1744556541.2858939227
  push.3779299846.3281205734.1040561013.2394388049
  push.3848741185.1749606857.2031190394.2257649329
  push.2068125131.1459464947.408359304.1594726639

  # push (2^2)G into stack
  push.1352383970.2724730208.2099681297.2989814637
  push.247196370.264991192.1311059568.2063719771
  push.581575810.1361063577.1272615727.723804695
  push.4073438323.828812798.281675353.2210016624
  push.2832828146.2614124149.2170633936.2236521962
  push.2879351502.263705507.995471108.1088921762

  # push (2^1)G into stack
  push.579919648.1848170264.3884180958.2687252166
  push.2688095969.1673463056.4082826636.2545792257
  push.1440660280.524996660.3425160008.2758035882
  push.2826781693.3043178571.483526712.3875396767
  push.617223735.3276311816.1644336685.4191201175
  push.3437933890.2072183026.4256012599.474728642

  # push (2^0)G into stack
  push.0.0.0.0
  push.0.0.1.977
  push.3477046559.3567616726.1891022234.2887369014
  push.2382126429.522045005.2975770322.3554388962
  push.2575427139.3909656392.2543798464.872223388
  push.589179219.700212955.3610652250.1216225431

  repeat.2
    repeat.4
      repeat.32
        pushw.local.18
        dup
        push.1
        u32checked_and
        movdn.4
        u32unchecked_shr.1
        popw.local.18

        if.true
          popw.local.12
          popw.local.13
          popw.local.14
          popw.local.15      
          popw.local.16
          popw.local.17

          push.env.locaddr.11
          push.env.locaddr.10
          push.env.locaddr.9
          push.env.locaddr.8
          push.env.locaddr.7
          push.env.locaddr.6

          push.env.locaddr.17
          push.env.locaddr.16
          push.env.locaddr.15
          push.env.locaddr.14
          push.env.locaddr.13
          push.env.locaddr.12

          push.env.locaddr.5
          push.env.locaddr.4
          push.env.locaddr.3
          push.env.locaddr.2
          push.env.locaddr.1
          push.env.locaddr.0

          exec.point_addition

          drop
          drop

          loadw.local.6
          storew.local.0
          loadw.local.7
          storew.local.1

          loadw.local.8
          storew.local.2
          loadw.local.9
          storew.local.3

          loadw.local.10
          storew.local.4
          loadw.local.11
          storew.local.5

          dropw
        else
          repeat.6
            dropw
          end
        end
      end

      pushw.local.18
      movdn.3
      popw.local.18
    end

    pushw.local.19
    popw.local.18
  end

  dup
  pushw.local.0
  movup.4
  popw.mem          # write x[0..4] to memory

  dup.1
  pushw.local.1
  movup.4
  popw.mem          # write x[4..8] to memory

  dup.2
  pushw.local.2
  movup.4
  popw.mem          # write y[0..4] to memory

  dup.3
  pushw.local.3
  movup.4
  popw.mem          # write y[4..8] to memory

  dup.4
  pushw.local.4
  movup.4
  popw.mem          # write z[0..4] to memory

  dup.5
  pushw.local.5
  movup.4
  popw.mem          # write z[4..8] to memory
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
    assert_eq
    movup.3
    assert_eq           # quotient remains on the stack
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
    assert_eq
    movup.3
    assert_eq           # remainder remains on the stack
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
    assert_eq
    movup.5
    assert_eq           # remainder remains on the stack
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
