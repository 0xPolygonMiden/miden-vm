//! This module is automatically generated during build time and should not be modified manually.

/// An array of modules defined in Miden standard library.
///
/// Entries in the array are tuples containing module namespace and module source code.
#[rustfmt::skip]
pub const MODULES: [(&str, &str); 13] = [
// ----- std::crypto::dsa::falcon -----------------------------------------------------------------
("std::crypto::dsa::falcon", "use.std::math::poly512

# Given an element on stack top, this routine normalizes that element in 
# interval (-q/2, q/2] | q = 12289
#
# Imagine, a is the provided element, which needs to be normalized
#
# b = normalize(a)
#   = (a + (q >> 1)) % q - (q >> 1) | a ∈ [0, q), q = 12289
#
# Note, normalization requires that we can represent the number as signed integer,
# which is not allowed inside Miden VM stack. But we can ignore the sign of integer and only
# store the absolute value as field element. This can be safely done because after normalization
# anyway `b` will be squared ( for computing norm of a vector i.e. polynomial, where b is a coefficient ).
# That means we can just drop the sign, and that's what is done in this routine.
#
# To be more concrete, normalization of 12166 ( = a ) should result into -123, but absolute value 
# 123 will be kept on stack. While normalization of 21, should result into 21, which has absolute
# value 21 --- that's what is kept on stack.
#
# Expected stack state :
#
# [a, ...]
#
# After normalization ( represented using unsigned integer i.e. Miden field element ) stack looks like
#
# [b, ...]
proc.normalize
    dup
    push.6144
    gt

    if.true
        push.6144
        add

        exec.poly512::mod_12289

        dup
        push.6144
        gte

        if.true
            push.6144
            sub
        else
            push.6144
            swap
            sub
        end
    end
end

# Given four elements from Falcon prime field, on stack top, this routine 
# normalizes each of them, using above defined `normalize()` routine.
#
# Expected stack state :
#
# [a0, a1, a2, a3, ...]
#
# Output stack state :
#
# [b0, b1, b2, b3, ...]
#
# b`i` = normalize(a`i`) | i ∈ [0..4)
proc.normalize_word
    exec.normalize

    swap
    exec.normalize
    swap

    movup.2
    exec.normalize
    movdn.2

    movup.3
    exec.normalize
    movdn.3
end

# Given a degree 512 polynomial on stack, using its starting (absolute) memory address, 
# this routine normalizes each coefficient of the polynomial, using above defined 
# `normalize()` routine
#
# Imagine, f is the given polynomial of degree 512. It can be normalized using
#
# g = [normalize(f[i]) for i in range(512)]
#
# Expected stack state :
#
# [f_start_addr, g_start_addr, ...] | next 127 absolute addresses can be computed using `INCR` instruction
#
# Post normalization stack state looks like
#
# [ ... ]
#
# Note, input polynomial which is provided using memory addresses, is not mutated.
export.normalize_poly512
    push.0.0.0.0

    repeat.128
        dup.4
        mem_loadw

        exec.normalize_word

        dup.5
        mem_storew

        movup.5
        add.1
        movdn.5

        movup.4
        add.1
        movdn.4
    end

    dropw
    drop
    drop
end

# Given four elements on stack top, this routine computes squared norm of that
# vector ( read polynomial ) with four coefficients.
#
# Imagine, given vector is f, which is described as
#
# f = [a0, a1, a2, a3]
#
# Norm of that vector is
#
# √(a0 ^ 2 + a1 ^ 2 + a2 ^ 2 + a3 ^ 2)
#
# But we need squared norm, which is just skipping the final square root operation.
#
# Expected stack state :
#
# [a0, a1, a2, a3, ...]
#
# Final stack state :
#
# [b, ...] | b = a0 ^ 2 + a1 ^ 2 + a2 ^ 2 + a3 ^ 2
proc.squared_norm_word
    dup
    mul

    swap
    dup
    mul

    add

    swap
    dup
    mul

    add

    swap
    dup
    mul

    add
end

# Given a degree 512 polynomial in coefficient form, as starting (absolute) memory address 
# on stack, this routine computes squared norm of that vector, using following formula
#
# Say, f = [a0, a1, a2, ..., a510, a511]
#      g = sq_norm(f) = a0 ^ 2 + a1 ^ 2 + ... + a510 ^ 2 + a511 ^ 2
#
# Expected input stack state :
#
# [f_start_addr, ...] | f_addr`i` holds f[(i << 2) .. ((i+1) << 2)]
#
# Consecutive 127 addresses on stack can be computed using `INCR` instruction, because memory 
# addresses are consecutive i.e. monotonically increasing by 1.
#
# Final stack state :
#
# [g, ...] | g = sq_norm(f)
export.squared_norm_poly512
    push.0.0.0.0.0

    repeat.128
        dup.5
        mem_loadw

        exec.squared_norm_word
        add

        swap
        add.1
        swap

        push.0.0.0.0
    end

    dropw
    swap
    drop
end

# Falcon-512 Digital Signature Verification routine
#
# Given four degree-511 polynomials, using initial absolute memory addresses on stack, 
# this routine checks whether it's a valid Falcon signature or not.
#
# Four degree-511 polynomials, which are provided ( in order )
#
# f = [f0, f1, ..., f510, f511] -> decompressed Falcon-512 signature
# g = [g0, g1, ..., g510, g511] -> public key used for signing input message
# h = [h0, h1, ..., h510, h511] -> input message hashed using SHAKE256 XOF and converted to polynomial
# k = [k0, k1, ..., k510, k511] -> [abs(i) for i in f] | abs(a) = a < 0 ? 0 - a : a
#
# Each of these polynomials are represented using starting absolute memory address. Contiguous 127 
# memory addresses can be computed by repeated application of INCR instruction ( read add.1 ) on previous
# absolute memory address.
#
# f`i` holds f[(i << 2) .. ((i+1) << 2)] | i ∈ [0..128)
# g`i` holds g[(i << 2) .. ((i+1) << 2)] | i ∈ [0..128)
# h`i` holds h[(i << 2) .. ((i+1) << 2)] | i ∈ [0..128)
# k`i` holds k[(i << 2) .. ((i+1) << 2)] | i ∈ [0..128)
#
# Expected stack state :
#
# [f_start_addr, g_start_addr, h_start_addr, k_start_addr, ...]
#
# After execution of verification routine, stack looks like
#
# [ ... ]
#
# If verification fails, program panics, due to failure in assertion !
#
# Note, input memory addresses are considered to be immutable.
export.verify.257
    locaddr.0
    movdn.2
    exec.poly512::mul_zq

    locaddr.128
    locaddr.0
    exec.poly512::neg_zq

    locaddr.0
    swap
    locaddr.128
    exec.poly512::add_zq

    locaddr.128
    locaddr.0
    exec.normalize_poly512

    # compute squared norm of s0

    locaddr.128
    exec.squared_norm_poly512

    locaddr.256
    mem_store
    drop

    # compute squared norm of s1 ( where s1 is provided as polynomial
    # with coefficients represented using absolute value i.e. signs are ignored )

    exec.squared_norm_poly512

    locaddr.256
    mem_load
    add

    # check that norm of the signature is small enough

    push.34034726 # constant sig_bound for Falcon-512 signature
    lte
    assert
end
"),
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
    mem_storew
    dropw

    push.0x5BE0CD19.0x1F83D9AB.0x9B05688C.0x510E527F
    movup.4
    mem_storew
    dropw

    push.0xA54FF53A.0x3C6EF372.0xBB67AE85.0x6A09E667
    movup.4
    mem_storew
    dropw

    push.11.64.0.0
    movup.4
    mem_storew
    dropw
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

    loc_storew.0

    movup.9
    mem_loadw
    movup.8
    push.0.0.0.0
    movup.4
    mem_loadw

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
    push.0.0.0.0
    movup.4
    mem_loadw

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
    push.0.0.0.0
    movup.4
    mem_loadw

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
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0

    movup.9
    mem_loadw
    movup.8
    push.0.0.0.0
    movup.4
    mem_loadw

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
    push.0.0.0.0
    movup.4
    mem_loadw

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
    push.0.0.0.0
    movup.4
    mem_loadw

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
    push.0.0.0.0
    loc_loadw.0
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
    loc_storew.0

    exec.columnar_mixing

    loc_storew.1
    dropw
    loc_storew.2
    dropw
    loc_storew.3
    dropw
    loc_storew.4
    dropw

    locaddr.4
    locaddr.3
    locaddr.2
    locaddr.1

    exec.diagonal_mixing

    push.0.0.0.0
    loc_loadw.0
    swapw
    movup.4
    mem_storew
    dropw

    repeat.3
        push.0
        movdn.3
        swapw
        movup.4
        mem_storew
        dropw
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
    loc_storew.0
    dropw

    # apply first 6 rounds of mixing
    repeat.6
        # round `i` | i ∈ [1..7)
        repeat.4
            dupw.3
        end

        push.0.0.0.0
        loc_loadw.0
        exec.round
        exec.permute_msg_words
    end

    # round 7 ( last round, so no message word permutation required )
    push.0.0.0.0
    loc_loadw.0
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
    locaddr.3
    locaddr.2
    locaddr.1
    locaddr.0

    exec.initialize

    # Note, chunk compression routine needs to compress only one chunk with one message 
    # block ( = 64 -bytes ) because what we're doing here is 2-to-1 hashing i.e. 64 -bytes 
    # input being converted to 32 -bytes output

    locaddr.3
    locaddr.2
    locaddr.1
    locaddr.0

    exec.compress

    push.0.0.0.0
    loc_loadw.3
    push.0.0.0.0
    loc_loadw.2
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0

    exec.finalize
end
"),
// ----- std::crypto::hashes::keccak256 -----------------------------------------------------------
("std::crypto::hashes::keccak256", "# Keccak-p[1600, 24] permutation's θ step mapping function, which is implemented 
# in terms of 32 -bit word size ( bit interleaved representation )
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L55-L98 for original implementation
#
# Expected stack state :
#
# [state_addr, ...]
#
# Final stack state :
#
# [ ... ]
#
# Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
# s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#
# Consecutive memory addresses can be computed by repeated application of `add.1`.
proc.theta.3
    dup
    locaddr.0
    mem_store
    drop

    # compute (S[0] ^ S[10] ^ S[20] ^ S[30] ^ S[40], S[1] ^ S[11] ^ S[21] ^ S[31] ^ S[41])

    # bring S[0], S[1]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.2
    add.2

    # bring S[10], S[11]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.3

    # bring S[20], S[21]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.2

    # bring S[30], S[31]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.3

    # bring S[40], S[41]
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.2
    u32checked_xor

    swap

    movup.2
    u32checked_xor

    swap

    # stack = [c0, c1]
    # compute (S[2] ^ S[12] ^ S[22] ^ S[32] ^ S[42], S[3] ^ S[13] ^ S[23] ^ S[33] ^ S[43])

    locaddr.0
    mem_load
    
    # bring S[2], S[3]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.2
    add.3

    # bring S[12], S[13]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.2

    # bring S[22], S[23]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.3

    # bring S[32], S[33]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.2

    # bring S[42], S[43]
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.2
    u32checked_xor

    swap

    movup.2
    u32checked_xor

    swap

    movup.3
    movup.3

    # stack = [c0, c1, c2, c3]

    locaddr.1
    mem_storew
    dropw

    # compute (S[4] ^ S[14] ^ S[24] ^ S[34] ^ S[44], S[5] ^ S[15] ^ S[25] ^ S[35] ^ S[45])

    locaddr.0
    mem_load
    add.1

    # bring S[4], S[5]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.2
    add.2

    # bring S[14], S[15]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.3

    # bring S[24], S[25]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.2

    # bring S[34], S[35]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.3

    # bring S[44], S[45]
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.2
    u32checked_xor

    swap

    movup.2
    u32checked_xor

    swap

    # stack = [c4, c5]
    # compute (S[6] ^ S[16] ^ S[26] ^ S[36] ^ S[46], S[7] ^ S[17] ^ S[27] ^ S[37] ^ S[47])

    locaddr.0
    mem_load
    add.1
    
    # bring S[6], S[7]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.2
    add.3

    # bring S[16], S[17]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.2

    # bring S[26], S[27]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.3

    # bring S[36], S[37]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.2

    # bring S[46], S[47]
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.2
    u32checked_xor

    swap

    movup.2
    u32checked_xor

    swap

    movup.3
    movup.3

    # stack = [c4, c5, c6, c7]

    locaddr.2
    mem_storew
    dropw

    # compute (S[8] ^ S[18] ^ S[28] ^ S[38] ^ S[48], S[9] ^ S[19] ^ S[29] ^ S[39] ^ S[49])

    locaddr.0
    mem_load
    add.2

    # bring S[8], S[9]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.2
    add.2

    # bring S[18], S[19]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.3

    # bring S[28], S[29]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.2

    # bring S[38], S[39]
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    movup.3
    u32checked_xor

    swap

    movup.3
    u32checked_xor

    swap

    movup.2
    add.3

    # bring S[48], S[49]
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.2
    u32checked_xor

    swap

    movup.2
    u32checked_xor

    swap

    # stack = [c8, c9]

    locaddr.2
    push.0.0.0.0
    movup.4
    mem_loadw
    locaddr.1
    push.0.0.0.0
    movup.4
    mem_loadw

    # stack = [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9]

    dup.8
    dup.4
    u32unchecked_rotl.1
    u32checked_xor

    dup.10
    dup.4
    u32checked_xor

    dup.2
    dup.8
    u32unchecked_rotl.1
    u32checked_xor

    dup.4
    dup.8
    u32checked_xor

    movup.6
    dup.11
    u32unchecked_rotl.1
    u32checked_xor

    movup.7
    dup.10
    u32checked_xor

    movup.8
    movup.13
    u32unchecked_rotl.1
    u32checked_xor

    movup.9
    movup.12
    u32checked_xor

    movup.10
    movup.10
    u32unchecked_rotl.1
    u32checked_xor

    movup.10
    movup.10
    u32checked_xor

    # stack = [d9, d8, d7, d6, d5, d4, d3, d2, d1, d0]

    swap
    movup.2
    movup.3
    movup.4
    movup.5
    movup.6
    movup.7
    movup.8
    movup.9

    # stack = [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9]

    locaddr.0
    mem_load

    # compute state[0..4)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    dup.5
    u32checked_xor

    swap
    dup.6
    u32checked_xor
    swap

    movup.2
    dup.7
    u32checked_xor
    movdn.2

    movup.3
    dup.8
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[4..8)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    dup.9
    u32checked_xor

    swap
    dup.10
    u32checked_xor
    swap

    movup.2
    dup.11
    u32checked_xor
    movdn.2

    movup.3
    dup.12
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[8..12)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    dup.13
    u32checked_xor

    swap
    dup.14
    u32checked_xor
    swap

    movup.2
    dup.5
    u32checked_xor
    movdn.2

    movup.3
    dup.6
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[12..16)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    dup.7
    u32checked_xor

    swap
    dup.8
    u32checked_xor
    swap

    movup.2
    dup.9
    u32checked_xor
    movdn.2

    movup.3
    dup.10
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[16..20)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    dup.11
    u32checked_xor

    swap
    dup.12
    u32checked_xor
    swap

    movup.2
    dup.13
    u32checked_xor
    movdn.2

    movup.3
    dup.14
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[20..24)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    dup.5
    u32checked_xor

    swap
    dup.6
    u32checked_xor
    swap

    movup.2
    dup.7
    u32checked_xor
    movdn.2

    movup.3
    dup.8
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[24..28)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    dup.9
    u32checked_xor

    swap
    dup.10
    u32checked_xor
    swap

    movup.2
    dup.11
    u32checked_xor
    movdn.2

    movup.3
    dup.12
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[28..32)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    dup.13
    u32checked_xor

    swap
    dup.14
    u32checked_xor
    swap

    movup.2
    dup.5
    u32checked_xor
    movdn.2

    movup.3
    dup.6
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[32..36)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    dup.7
    u32checked_xor

    swap
    dup.8
    u32checked_xor
    swap

    movup.2
    dup.9
    u32checked_xor
    movdn.2

    movup.3
    dup.10
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[36..40)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    dup.11
    u32checked_xor

    swap
    dup.12
    u32checked_xor
    swap

    movup.2
    dup.13
    u32checked_xor
    movdn.2

    movup.3
    dup.14
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[40..44)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.5
    u32checked_xor

    swap
    movup.5
    u32checked_xor
    swap

    movup.2
    movup.5
    u32checked_xor
    movdn.2

    movup.3
    movup.5
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[44..48)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.5
    u32checked_xor

    swap
    movup.5
    u32checked_xor
    swap

    movup.2
    movup.5
    u32checked_xor
    movdn.2

    movup.3
    movup.5
    u32checked_xor
    movdn.3

    dup.4
    mem_storew
    dropw

    add.1

    # compute state[48..50)

    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.5
    u32checked_xor

    swap
    movup.5
    u32checked_xor
    swap

    movup.4
    mem_storew
    dropw
end

# Keccak-p[1600, 24] permutation's ρ step mapping function, which is implemented 
# in terms of 32 -bit word size ( bit interleaved representation )
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L115-L147 for original implementation
#
# Expected stack state :
#
# [state_addr, ...]
#
# Final stack state :
#
# [ ... ]
#
# Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
# s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#
# Consecutive memory addresses can be computed by repeated application of `add.1`.
proc.rho.1
    dup
    locaddr.0
    mem_store
    drop

    # rotate state[0..4)
    push.0.0.0.0
    dup.4
    mem_loadw

    movup.3
    u32unchecked_rotl.1
    movdn.2

    movup.4
    dup
    add.1
    movdn.5
    mem_storew

    # rotate state[4..8)
    dup.4
    mem_loadw

    u32unchecked_rotl.31
    swap
    u32unchecked_rotl.31
    swap

    movup.2
    u32unchecked_rotl.14
    movdn.2
    movup.3
    u32unchecked_rotl.14
    movdn.3

    movup.4
    dup
    add.1
    movdn.5
    mem_storew

    # rotate state[8..12)
    dup.4
    mem_loadw

    u32unchecked_rotl.13
    swap
    u32unchecked_rotl.14

    movup.2
    u32unchecked_rotl.18
    movdn.2
    movup.3
    u32unchecked_rotl.18
    movdn.3

    movup.4
    dup
    add.1
    movdn.5
    mem_storew

    # rotate state[12..16)
    dup.4
    mem_loadw

    u32unchecked_rotl.22
    swap
    u32unchecked_rotl.22
    swap

    movup.2
    u32unchecked_rotl.3
    movdn.2
    movup.3
    u32unchecked_rotl.3
    movdn.3

    movup.4
    dup
    add.1
    movdn.5
    mem_storew

    # rotate state[16..20)
    dup.4
    mem_loadw

    u32unchecked_rotl.27
    swap
    u32unchecked_rotl.28

    movup.2
    u32unchecked_rotl.10
    movdn.2
    movup.3
    u32unchecked_rotl.10
    movdn.3

    movup.4
    dup
    add.1
    movdn.5
    mem_storew

    # rotate state[20..24)
    dup.4
    mem_loadw

    u32unchecked_rotl.1
    swap
    u32unchecked_rotl.2

    movup.2
    u32unchecked_rotl.5
    movdn.2
    movup.3
    u32unchecked_rotl.5
    movdn.3

    movup.4
    dup
    add.1
    movdn.5
    mem_storew

    # rotate state[24..28)
    dup.4
    mem_loadw

    u32unchecked_rotl.21
    swap
    u32unchecked_rotl.22

    movup.2
    u32unchecked_rotl.12
    movdn.3
    movup.2
    u32unchecked_rotl.13
    movdn.2

    movup.4
    dup
    add.1
    movdn.5
    mem_storew

    # rotate state[28..32)
    dup.4
    mem_loadw

    u32unchecked_rotl.19
    swap
    u32unchecked_rotl.20

    movup.2
    u32unchecked_rotl.20
    movdn.3
    movup.2
    u32unchecked_rotl.21
    movdn.2

    movup.4
    dup
    add.1
    movdn.5
    mem_storew
     
    # rotate state[32..36)
    dup.4
    mem_loadw

    u32unchecked_rotl.22
    swap
    u32unchecked_rotl.23

    movup.2
    u32unchecked_rotl.7
    movdn.3
    movup.2
    u32unchecked_rotl.8
    movdn.2

    movup.4
    dup
    add.1
    movdn.5
    mem_storew

    # rotate state[36..40)
    dup.4
    mem_loadw

    u32unchecked_rotl.10
    swap
    u32unchecked_rotl.11

    movup.2
    u32unchecked_rotl.4
    movdn.2
    movup.3
    u32unchecked_rotl.4
    movdn.3

    movup.4
    dup
    add.1
    movdn.5
    mem_storew

    # rotate state[40..44)
    dup.4
    mem_loadw
    
    u32unchecked_rotl.9
    swap
    u32unchecked_rotl.9
    swap

    movup.2
    u32unchecked_rotl.1
    movdn.2
    movup.3
    u32unchecked_rotl.1
    movdn.3

    movup.4
    dup
    add.1
    movdn.5
    mem_storew

    # rotate state[44..48)
    dup.4
    mem_loadw

    u32unchecked_rotl.30
    swap
    u32unchecked_rotl.31

    movup.2
    u32unchecked_rotl.28
    movdn.2
    movup.3
    u32unchecked_rotl.28
    movdn.3

    movup.4
    dup
    add.1
    movdn.5
    mem_storew

    # rotate state[48..50)
    dup.4
    mem_loadw

    u32unchecked_rotl.7
    swap
    u32unchecked_rotl.7
    swap

    movup.4
    mem_storew
    dropw
end

# Keccak-p[1600, 24] permutation's π step mapping function, which is implemented 
# in terms of 32 -bit word size ( bit interleaved representation )
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L169-L207 for original implementation
#
# Expected stack state :
#
# [state_addr, ...]
#
# Final stack state :
#
# [ ... ]
#
# Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
# s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#
# Consecutive memory addresses can be computed by repeated application of `add.1`.
proc.pi.14
    dup
    locaddr.0
    mem_store
    drop

    locaddr.1
    swap
    push.0.0.0.0

    # place state[0..4) to desired location(s)
    dup.4
    mem_loadw

    push.0.0
    movdn.3
    movdn.3

    dup.7
    mem_storew

    drop
    drop
    movdn.3
    movdn.3

    dup.5
    add.5
    mem_storew

    # place state[4..8) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0
    movdn.3
    movdn.3

    dup.7
    add.10
    mem_storew

    drop
    drop

    dup.5
    add.2
    mem_storew

    # place state[8..12) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0

    dup.7
    add.7
    mem_storew

    movup.2
    drop
    movup.2
    drop

    movdn.3
    movdn.3

    dup.5
    add.8
    mem_storew

    # place state[12..16) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    dup.5
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    dup.7
    mem_storew

    dup.7
    add.5
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    dup.5
    add.5
    mem_storew

    # place state[16..20) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    dup.5
    add.10
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    dup.7
    add.10
    mem_storew

    dropw

    push.0.0
    movdn.3
    movdn.3

    dup.5
    add.3
    mem_storew

    # place state[20..24) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    dup.5
    add.3
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    dup.7
    add.3
    mem_storew

    dup.7
    add.8
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    dup.5
    add.8
    mem_storew

    # place state[24..28) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0
    movdn.3
    movdn.3

    dup.7
    add.1
    mem_storew

    drop
    drop
    movdn.3
    movdn.3

    dup.5
    add.6
    mem_storew

    # place state[28..32) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    dup.5
    add.11
    mem_storew

    # place state[32..36) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0
    movdn.3
    movdn.3

    dup.7
    add.4
    mem_storew

    drop
    drop
    movdn.3
    movdn.3

    dup.5
    add.9
    mem_storew

    # place state[36..40) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    dup.5
    add.1
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    dup.7
    add.1
    mem_storew

    dup.7
    add.6
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    dup.5
    add.6
    mem_storew

    # place state[40..44) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    dup.5
    add.7
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop
    movup.3
    movup.3

    dup.7
    add.7
    mem_storew

    dropw

    push.0.0
    movdn.3
    movdn.3

    dup.5
    add.12
    mem_storew

    # place state[44..48) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    dup.5
    add.4
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    dup.7
    add.4
    mem_storew

    dup.7
    add.9
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    dup.5
    add.9
    mem_storew

    # place state[48..50) to desired location(s)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    dup.5
    add.2
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop
    movdn.3
    movdn.3

    dup.7
    add.2
    mem_storew

    drop
    drop

    # memcpy
    movup.4
    drop
    locaddr.0
    mem_load
    movdn.4

    repeat.13
        dup.5
        mem_loadw

        dup.4
        mem_storew

        movup.4
        add.1
        movdn.4

        movup.5
        add.1
        movdn.5
    end

    dropw
    drop
    drop
end

# Keccak-p[1600, 24] permutation's χ step mapping function, which is implemented 
# in terms of 32 -bit word size ( bit interleaved representation )
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L233-L271 for original implementation
#
# Expected stack state :
#
# [state_addr, ...]
#
# Final stack state :
#
# [ ... ]
#
# Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
# s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#
# Consecutive memory addresses can be computed by repeated application of `add.1`.
proc.chi.4
    dup
    locaddr.0
    mem_store
    drop

    # process state[0..10)
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    add.1
    dup
    movdn.3

    push.0.0.0.0
    movup.4
    mem_loadw

    dup.1
    dup.1

    movup.6
    u32checked_and

    swap

    movup.6
    u32checked_and

    swap

    movup.3
    u32checked_not
    movup.3
    u32checked_not

    movup.4
    u32checked_and
    swap
    movup.4
    u32checked_and
    swap

    movup.3
    movup.3

    locaddr.1
    mem_storew

    dup.4
    mem_loadw

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    add.1
    dup
    movdn.3

    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    dup.1
    dup.1

    movup.4
    u32checked_and
    swap
    movup.4
    u32checked_and
    swap

    movup.3
    movup.3

    movup.4
    sub.2
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.5
    u32checked_not
    movup.5
    u32checked_not

    dup.2
    u32checked_and
    swap
    dup.3
    u32checked_and
    swap

    movup.7
    movup.7

    locaddr.2
    mem_storew
    dropw

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

    locaddr.0
    mem_load

    push.0.0.0.0

    dup.4
    mem_loadw

    locaddr.1
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    locaddr.2
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    movup.5
    u32checked_xor
    swap
    movup.5
    u32checked_xor
    swap

    dup.4
    mem_storew

    # process state[10..20)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    u32checked_not
    swap
    u32checked_not
    swap

    dup.3
    dup.3

    movup.2
    u32checked_and
    swap
    movup.2
    u32checked_and
    swap

    push.0.0
    locaddr.1
    mem_storew

    movup.6
    add.1
    dup
    movdn.7

    mem_loadw

    movup.5
    movup.5

    u32checked_not
    swap
    u32checked_not
    swap

    dup.2
    u32checked_and
    swap
    dup.3
    u32checked_and
    swap

    movup.3
    movup.3

    u32checked_not
    swap
    u32checked_not
    swap

    dup.4
    u32checked_and
    swap
    dup.5
    u32checked_and
    swap

    movup.3
    movup.3

    locaddr.2
    mem_storew

    movup.6
    sub.2
    dup
    movdn.7

    mem_loadw

    drop
    drop

    dup.1
    dup.1

    movup.4
    u32checked_not
    movup.5
    u32checked_not
    swap

    movup.2
    u32checked_and
    swap
    movup.2
    u32checked_and
    swap

    movup.3
    movup.3

    movup.4
    add.1
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.3
    movup.3

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

    movup.3
    movup.3

    locaddr.3
    mem_storew

    locaddr.0
    mem_load
    add.2
    dup
    movdn.5

    mem_loadw

    push.0.0.0.0
    loc_loadw.1

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0.0.0
    loc_loadw.2
    
    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0.0.0
    loc_loadw.3
    
    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    # process state[20..30)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    drop
    drop

    u32checked_not
    swap
    u32checked_not
    swap

    movup.2
    add.1
    movdn.2

    dup.2
    push.0.0.0.0
    movup.4
    mem_loadw

    dup.1
    dup.1

    movup.6
    u32checked_and
    swap
    movup.6
    u32checked_and
    swap

    movup.3
    movup.3

    u32checked_not
    swap
    u32checked_not
    swap

    dup.4
    u32checked_and
    swap
    dup.5
    u32checked_and
    swap

    movup.3
    movup.3

    loc_storew.1

    movup.6
    add.1
    movdn.6

    dup.6
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    dup.1
    dup.1

    movup.5
    movup.5

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

    movup.4
    sub.2
    movdn.4

    dup.4
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.7
    movup.7

    u32checked_not
    swap
    u32checked_not
    swap

    dup.3
    dup.3

    movup.2
    u32checked_and
    swap
    movup.2
    u32checked_and
    swap

    movup.7
    movup.7

    loc_storew.2
    dropw

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
    movdn.3
    movdn.3

    loc_storew.3

    dup.4
    mem_loadw

    push.0.0.0.0
    loc_loadw.1

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0.0.0
    loc_loadw.2

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0.0.0
    loc_loadw.3

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    # process state[30..40)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    u32checked_not
    swap
    u32checked_not
    swap

    dup.3
    dup.3

    movup.2
    u32checked_and
    swap
    movup.2
    u32checked_and
    swap

    push.0.0
    loc_storew.1

    movup.6
    add.1
    movdn.6

    dup.6
    mem_loadw

    movup.5
    movup.5

    u32checked_not
    swap
    u32checked_not
    swap

    dup.3
    dup.3

    movup.2
    u32checked_and
    swap
    movup.2
    u32checked_and
    swap

    movup.3
    movup.3

    u32checked_not
    swap
    u32checked_not
    swap

    dup.5
    dup.5

    movup.2
    u32checked_and
    swap
    movup.2
    u32checked_and
    swap

    movup.3
    movup.3

    loc_storew.2

    movup.6
    sub.2
    movdn.6

    dup.6
    mem_loadw

    drop
    drop

    movup.3
    movup.3

    u32checked_not
    swap
    u32checked_not
    swap

    dup.3
    dup.3

    movup.2
    u32checked_and
    swap
    movup.2
    u32checked_and
    swap

    movup.4
    add.1
    movdn.4

    dup.4
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.5
    movup.5

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

    movup.3
    movup.3

    loc_storew.3

    movup.4
    sub.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0.0.0
    loc_loadw.1

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0.0.0
    loc_loadw.2

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0.0.0
    loc_loadw.3

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    # process state[40..50)
    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    drop
    drop

    movup.2
    add.1
    movdn.2

    dup.2
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.5
    movup.5

    u32checked_not
    swap
    u32checked_not
    swap

    dup.3
    dup.3

    movup.2
    u32checked_and
    swap
    movup.2
    u32checked_and
    swap

    movup.3
    movup.3

    u32checked_not
    swap
    u32checked_not
    swap

    dup.5
    dup.5

    movup.2
    u32checked_and
    swap
    movup.2
    u32checked_and
    swap

    movup.3
    movup.3

    loc_storew.1

    movup.6
    add.1
    movdn.6

    dup.6
    mem_loadw

    movup.2
    drop
    movup.2
    drop

    movup.3
    movup.3

    u32checked_not
    swap
    u32checked_not
    swap

    dup.3
    dup.3

    movup.2
    u32checked_and
    swap
    movup.2
    u32checked_and
    swap

    movup.4
    sub.2
    movdn.4

    dup.4
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.7
    movup.7

    u32checked_not
    swap
    u32checked_not
    swap

    dup.3
    dup.3

    movup.2
    u32checked_and
    swap
    movup.2
    u32checked_and
    swap

    movup.7
    movup.7

    loc_storew.2
    dropw

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
    movdn.3
    movdn.3

    loc_storew.3

    dup.4
    mem_loadw

    push.0.0.0.0
    loc_loadw.1

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0.0.0
    loc_loadw.2

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    movup.4
    add.1
    movdn.4

    dup.4
    mem_loadw

    push.0.0.0.0
    loc_loadw.3

    movup.4
    u32checked_xor

    swap
    movup.4
    u32checked_xor
    swap

    movup.2
    movup.4
    u32checked_xor
    movdn.2

    movup.3
    movup.4
    u32checked_xor
    movdn.3

    dup.4
    mem_storew

    dropw
    drop
end

# Keccak-p[1600, 24] permutation's ι ( iota ) function, which is
# implemented in terms of 32 -bit word size ( bit interleaved form ); 
# imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
# invoked with (c0, c1) as template arguments
#
# Expected stack state :
#
# [state_addr, c0, c1, ...]
#
# Final stack state :
#
# [ ... ]
#
# All this routine does is
#
# state[0] ^= c0
# state[1] ^= c1
proc.iota
    dup
    push.0.0.0.0
    movup.4
    mem_loadw

    movup.5
    u32checked_xor

    swap

    movup.5
    u32checked_xor

    swap

    movup.4
    mem_storew
    dropw
end

# Keccak-p[1600, 24] permutation round, without `iota` function ( all other 
# functions i.e. `theta`, `rho`, `pi`, `chi` are applied in order )
#
# As `iota` function involves xoring constant factors with first lane of state array 
# ( read state[0, 0] ), it's required to invoke them seperately after completion of
# this procedure's execution.
#
# Expected stack state :
#
# [start_addr, ... ]
#
# After finishing execution, stack looks like
#
# [ ... ]
#
# Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
# s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#
# Consecutive memory addresses can be computed by repeated application of `add.1`.
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L325-L340
proc.round
    dup
    exec.theta

    dup
    exec.rho

    dup
    exec.pi

    exec.chi
end

# Keccak-p[1600, 24] permutation, applying 24 rounds on state array of size  5 x 5 x 64, 
# where each 64 -bit lane is represented in bit interleaved form ( in terms of two 32 -bit words ).
#
# Expected stack state :
#
# [start_addr, ... ]
#
# After finishing execution, stack looks like
#
# [ ... ]
#
# Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
# s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#
# Consecutive memory addresses can be computed by repeated application of `add.1`.
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L379-L427
proc.keccak_p
    # permutation round 1
    dup
    exec.round

    push.0.1
    dup.2
    exec.iota

    # permutation round 2
    dup
    exec.round

    push.137.0
    dup.2
    exec.iota

    # permutation round 3
    dup
    exec.round

    push.2147483787.0
    dup.2
    exec.iota

    # permutation round 4
    dup
    exec.round

    push.2147516544.0
    dup.2
    exec.iota

    # permutation round 5
    dup
    exec.round

    push.139.1
    dup.2
    exec.iota

    # permutation round 6
    dup
    exec.round

    push.32768.1
    dup.2
    exec.iota

    # permutation round 7
    dup
    exec.round

    push.2147516552.1
    dup.2
    exec.iota

    # permutation round 8
    dup
    exec.round

    push.2147483778.1
    dup.2
    exec.iota

    # permutation round 9
    dup
    exec.round

    push.11.0
    dup.2
    exec.iota

    # permutation round 10
    dup
    exec.round

    push.10.0
    dup.2
    exec.iota

    # permutation round 11
    dup
    exec.round

    push.32898.1
    dup.2
    exec.iota

    # permutation round 12
    dup
    exec.round

    push.32771.0
    dup.2
    exec.iota

    # permutation round 13
    dup
    exec.round

    push.32907.1
    dup.2
    exec.iota

    # permutation round 14
    dup
    exec.round

    push.2147483659.1
    dup.2
    exec.iota

    # permutation round 15
    dup
    exec.round

    push.2147483786.1
    dup.2
    exec.iota

    # permutation round 16
    dup
    exec.round

    push.2147483777.1
    dup.2
    exec.iota

    # permutation round 17
    dup
    exec.round

    push.2147483777.0
    dup.2
    exec.iota

    # permutation round 18
    dup
    exec.round

    push.2147483656.0
    dup.2
    exec.iota

    # permutation round 19
    dup
    exec.round

    push.131.0
    dup.2
    exec.iota

    # permutation round 20
    dup
    exec.round

    push.2147516419.0
    dup.2
    exec.iota

    # permutation round 21
    dup
    exec.round

    push.2147516552.1
    dup.2
    exec.iota

    # permutation round 22
    dup
    exec.round

    push.2147483784.0
    dup.2
    exec.iota

    # permutation round 23
    dup
    exec.round

    push.32768.1
    dup.2
    exec.iota

    # permutation round 24
    dup
    exec.round

    push.2147516546.0
    movup.2
    exec.iota
end

# Given two 32 -bit unsigned integers ( standard form ), representing upper and lower
# bits of a 64 -bit unsigned integer ( actually a keccak-[1600, 24] lane ),
# this function converts them into bit interleaved representation, where two 32 -bit
# unsigned integers ( even portion & then odd portion ) hold bits in even and odd
# indices of 64 -bit unsigned integer ( remember it's represented in terms of
# two 32 -bit elements )
#
# Input stack state :
#
# [hi, lo, ...]
#
# After application of bit interleaving, stack looks like
#
# [even, odd, ...]
#
# Read more about bit interleaved representation in section 2.1 of https://keccak.team/files/Keccak-implementation-3.2.pdf
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/utils.hpp#L123-L149
# for reference implementation in higher level language.
export.to_bit_interleaved
    push.0.0

    repeat.16
        u32unchecked_shr.1
        swap
        u32unchecked_shr.1
        swap

        # ---

        dup.3
        dup.3

        push.1
        u32checked_and
        swap
        push.1
        u32checked_and
        swap

        u32unchecked_shl.31
        swap
        u32unchecked_shl.15
        swap

        u32checked_xor
        u32checked_xor

        # ---

        dup.3
        dup.3

        push.2
        u32checked_and
        swap
        push.2
        u32checked_and
        swap

        u32unchecked_shl.30
        swap
        u32unchecked_shl.14
        swap

        movup.3
        u32checked_xor
        u32checked_xor
        swap

        # ---

        movup.2
        u32unchecked_shr.2
        movdn.2

        movup.3
        u32unchecked_shr.2
        movdn.3
    end

    movup.2
    drop
    movup.2
    drop
end

# Given two 32 -bit unsigned integers ( in bit interleaved form ), representing even and odd
# positioned bits of a 64 -bit unsigned integer ( actually a keccak-[1600, 24] lane ),
# this function converts them into standard representation, where two 32 -bit
# unsigned integers hold higher ( 32 -bit ) and lower ( 32 -bit ) bits of standard
# representation of 64 -bit unsigned integer
#
# Input stack state :
#
# [even, odd, ...]
#
# After application of logic, stack looks like
#
# [hi, lo, ...]
#
# This function reverts the action done by `to_bit_interleaved` function implemented above.
#
# Read more about bit interleaved representation in section 2.1 of https://keccak.team/files/Keccak-implementation-3.2.pdf
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/utils.hpp#L151-L175
# for reference implementation in higher level language.
export.from_bit_interleaved
    push.0.0

    repeat.16
        u32unchecked_shr.2
        swap
        u32unchecked_shr.2
        swap

        # ---

        dup.3
        dup.3

        push.1
        u32checked_and
        swap
        push.1
        u32checked_and
        
        u32unchecked_shl.31
        swap
        u32unchecked_shl.30
        u32checked_xor

        movup.2
        u32checked_xor
        swap

        # ---

        dup.3
        dup.3

        push.65536
        u32checked_and
        swap
        push.65536
        u32checked_and

        u32unchecked_shl.15
        swap
        u32unchecked_shl.14
        u32checked_xor

        u32checked_xor

        # ---

        movup.2
        u32unchecked_shr.1
        movdn.2

        movup.3
        u32unchecked_shr.1
        movdn.3
    end

    movup.2
    drop
    movup.2
    drop
end

# Given 64 -bytes input ( in terms of sixteen u32 elements on stack top ) to 2-to-1
# keccak256 hash function, this function prepares 5 x 5 x 64 keccak-p[1600, 24] state
# bit array such that each of twenty five 64 -bit wide lane is represented in bit
# interleaved form, using two 32 -bit integers. After completion of execution of
# this function, state array should live in allocated memory ( total fifty u32 elements, stored in
# 13 consecutive memory addresses s.t. starting absolute address is provided ).
#
# Input stack state :
#
# [state_addr, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, ...]
#
# Note, state_addr is the starting absolute memory address where keccak-p[1600, 24] state
# is kept. Consecutive addresses can be computed by repeated application of `add.1` instruction.
#
# Final stack state :
#
# [...]
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/keccak_256.hpp#L73-L153
proc.to_state_array
    repeat.4
        movdn.4
        exec.to_bit_interleaved

        movup.3
        movup.3

        exec.to_bit_interleaved

        movup.3
        movup.3

        dup.4
        mem_storew
        dropw

        add.1
    end

    push.0.0.0.1
    dup.4
    mem_storew
    dropw

    add.1

    push.0.0.0.0
    dup.4
    mem_storew
    dropw

    add.1

    push.0.0.0.0
    dup.4
    mem_storew
    dropw

    add.1

    push.0.0.0.0
    dup.4
    mem_storew
    dropw

    add.1

    push.0.0.2147483648.0
    dup.4
    mem_storew
    dropw

    add.1

    push.0.0.0.0
    dup.4
    mem_storew
    dropw

    add.1

    push.0.0.0.0
    dup.4
    mem_storew
    dropw

    add.1

    push.0.0.0.0
    dup.4
    mem_storew
    dropw

    add.1

    push.0.0.0.0
    movup.4
    mem_storew
    dropw
end

# Given 32 -bytes digest ( in terms of eight u32 elements on stack top ) in bit interleaved form,
# this function attempts to convert those into standard representation, where eight u32 elements
# live on stack top, each pair of them hold higher and lower bits of 64 -bit unsigned
# integer ( lane of keccak-p[1600, 24] state array )
#
# Input stack state :
#
# [lane0_even, lane0_odd, lane1_even, lane1_odd, lane2_even, lane2_odd, lane3_even, lane3_odd, ...]
#
# Output stack state :
#
# [dig0_hi, dig0_lo, dig1_hi, dig1_lo, dig2_hi, dig2_lo, dig3_hi, dig3_lo, ...]
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/keccak_256.hpp#L180-L209
proc.to_digest
    repeat.4
        movup.7
        movup.7

        exec.from_bit_interleaved
    end
end

# Given 64 -bytes input, in terms of sixteen 32 -bit unsigned integers, where each pair
# of them holding higher & lower 32 -bits of 64 -bit unsigned integer ( reinterpreted on
# host CPU from little endian byte array ) respectively, this function computes 32 -bytes
# keccak256 digest, held on stack top, represented in terms of eight 32 -bit unsigned integers,
# where each pair of them keeps higher and lower 32 -bits of 64 -bit unsigned integer respectively
#
# Expected stack state :
#
# [iword0, iword1, iword2, iword3, iword4, iword5, iword6, iword7, 
#  iword8, iword9, iword10, iword11, iword12, iword13, iword14, iword15, ... ]
#
# Final stack state :
#
# [oword0, oword1, oword2, oword3, oword4, oword5, oword6, oword7, ... ]
#
# See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/keccak_256.hpp#L232-L257
export.hash.13
    # prapare keccak256 state from input message
    locaddr.0
    exec.to_state_array

    # apply keccak-p[1600, 24] permutation
    locaddr.0
    exec.keccak_p

    # prapare keccak256 digest from state
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
    exec.to_digest
end"),
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
    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

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
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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

    loc_storew.0
    dropw
    loc_storew.1
    dropw

    movupw.2
    movupw.3
    movupw.3

    exec.rev_element_order

    push.0x19a4c116
    push.0.0.0.0
    loc_loadw.1
    push.0.0.0.0
    loc_loadw.0
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
// ----- std::math::ec_ext5 -----------------------------------------------------------------------
("std::math::ec_ext5", "use.std::math::ext5

# Given an encoded elliptic curve point `w` s.t. it's expressed using
# an element ∈ GF(p^5) | p = 2^64 - 2^32 + 1, this routine verifies whether
# given point can be successfully decoded or not
#
# Expected stack state 
#
# [w0, w1, w2, w3, w4, ...]
#
# Final stack state 
#
# [flg, ...]
#
# If w can be decoded, flg = 1
# Else flg = 0
#
# Note, if w = (0, 0, 0, 0, 0), it can be successfully decoded to point 
# at infinity i.e. flg = 1, in that case.
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1043-L1052
# for reference implementation
export.validate
    repeat.5
        dup.4
    end

    exec.ext5::square
    sub.2 # = e

    exec.ext5::square
    swap
    sub.1052
    swap # = delta
    
    exec.ext5::legendre
    eq.1
    movdn.5

    push.1
    repeat.5
        swap
        eq.0
        and
    end

    or
end

# Given an encoded elliptic curve point `w` s.t. it's expressed using
# an element ∈ GF(p^5) | p = 2^64 - 2^32 + 1, this routine attempts to decode
# it into x, y coordinates, along with boolean field element denoting whether it's
# point-at-infinity or not.
#
# Expected stack state 
#
# [w0, w1, w2, w3, w4, ...]
#
# Final state state 
#
# [x0, x1, x2, x3, x4, y0, y1, y2, y3, y4, inf, flg, ...]
#
# If `w` has be decoded, flg = 1
# Else flg = 0 and x, y = (0, 0)
#
# Note, when w = (0, 0, 0, 0, 0), it will be successfully decoded to
# point-at-infinity i.e. x, y = (0, 0) and flg = 1
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1022-L1041
# for reference implementation
export.decode
    repeat.5
        dup.4
    end

    exec.ext5::square
    sub.2 # = e

    repeat.5
        dup.4
    end

    exec.ext5::square
    swap
    sub.1052
    swap # = delta

    exec.ext5::sqrt # = (r, c)

    repeat.5
        dup.10
    end

    repeat.5
        dup.9
    end

    exec.ext5::add
    push.0.0.0.0.9223372034707292161
    exec.ext5::mul # = x1

    repeat.5
        movup.9
    end

    repeat.5
        movup.15
    end

    exec.ext5::sub
    push.0.0.0.0.9223372034707292161
    exec.ext5::mul # = x2

    repeat.5
        movup.9
    end

    repeat.5
        dup.4
    end

    exec.ext5::legendre
    eq.1

    if.true
        repeat.5
            movup.5
            drop
        end
    else
        repeat.5
            drop
        end
    end # = x

    repeat.5
        dup.10
    end

    repeat.5
        dup.9
    end

    exec.ext5::mul
    repeat.5
        movup.4
        neg
    end # = y

    dup.10
    not # = inf

    push.1
    repeat.5
        movup.13
        eq.0
        and
    end

    movup.12
    or # = c

    swap

    repeat.5
        movup.6
    end

    repeat.5
        movup.11
    end

    add.6148914689804861441 # = x
end

# Given an elliptic curve point as Weierstraß coordinates (X, Y) along with
# boolean field element `inf`, denoting whether this is point-at-infinity or not, 
# this routine encodes it to a single element ∈ GF(p^5) | p = 2^64 - 2^32 + 1
#
# Expected stack state 
#
# [x0, x1, x2, x3, x4, y0, y1, y2, y3, y4, inf, ...]
#
# Final stack state 
#
# [w0, w1, w2, w3, w4, ...]
#
# Note, when inf = 1, encoded point w = (0, 0, 0, 0, 0)
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1214-L1216
# for reference implementation.
export.encode
    push.0.0.0.0.6148914689804861441 # = a/ 3

    exec.ext5::sub # = (a/ 3) - x

    repeat.5
        movup.9
    end

    exec.ext5::div # = w = y/ ((a/ 3) - x)

    movup.5
    if.true
        repeat.5
            drop
        end

        push.0.0.0.0.0
    end
end

# Given two elliptic curve points ( say a, b ) as Weierstraß coordinates (X, Y) on stack,
# this routine computes elliptic curve point c, resulting from a + b.
#
# Following point addition formula is complete and it works when two points are 
# same/ different or input operands are point-at-infinity.
#
# Expected stack state
#
# [x1_0, x1_1, x1_2, x1_3, x1_4, y1_0, y1_1, y1_2, y1_3, y1_4, inf1, x2_0, x2_1, x2_2, x2_3, x2_4, y2_0, y2_1, y2_2, y2_3, y2_4, inf2, ...]
#
# s.t. x1_{0..5} -> x1, y1_{0..5} -> y1 |> a = (x1, y1, inf1)
#      x2_{0..5} -> x2, y2_{0..5} -> y2 |> b = (x2, y2, inf2)
#
# Final stack state
#
# [x3_0, x3_1, x3_2, x3_3, x3_4, y3_0, y3_1, y3_2, y3_3, y3_4, inf3, ...]
#
# Read point addition section ( on page 8 ) of https://ia.cr/2022/274
# For reference implementation see https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1228-L1255
export.add.10
    loc_storew.0
    dropw
    loc_store.1 # cached x1
    drop

    loc_storew.2
    dropw
    loc_store.3 # cached y1
    drop

    loc_store.4 # cached inf1
    drop

    loc_storew.5
    dropw
    loc_store.6 # cached x2
    drop

    loc_storew.7
    dropw
    loc_store.8 # cached y2
    drop

    loc_store.9 # cached inf2
    drop

    loc_load.6
    push.0.0.0.0
    loc_loadw.5 # bring x2

    loc_load.1
    push.0.0.0.0
    loc_loadw.0 # bring x1

    exec.ext5::eq
    dup

    if.true
        loc_load.1
        push.0.0.0.0
        loc_loadw.0 # bring x1

        exec.ext5::square

        repeat.5
            movup.4
            mul.3
        end

        add.6148914689804861439
        swap
        add.263
        swap
    else
        loc_load.3
        push.0.0.0.0
        loc_loadw.2 # bring y1

        loc_load.8
        push.0.0.0.0
        loc_loadw.7 # bring y2

        exec.ext5::sub
    end # = λ0

    dup.5

    if.true
        loc_load.3
        push.0.0.0.0
        loc_loadw.2 # bring y1

        repeat.5
            movup.4
            mul.2
        end
    else
        loc_load.1
        push.0.0.0.0
        loc_loadw.0 # bring x1

        loc_load.6
        push.0.0.0.0
        loc_loadw.5 # bring x2

        exec.ext5::sub
    end # = λ1

    repeat.5
        movup.9
    end

    exec.ext5::div # = λ

    repeat.5
        dup.4
    end

    exec.ext5::square # = λ^2

    loc_load.6
    push.0.0.0.0
    loc_loadw.5 # bring x2

    loc_load.1
    push.0.0.0.0
    loc_loadw.0 # bring x1

    exec.ext5::add

    repeat.5
        movup.9
    end

    exec.ext5::sub # compute x3

    repeat.5
        dup.4
    end

    loc_load.1
    push.0.0.0.0
    loc_loadw.0 # bring x1

    exec.ext5::sub

    repeat.5
        movup.14
    end

    exec.ext5::mul

    loc_load.3
    push.0.0.0.0
    loc_loadw.2 # bring y1

    repeat.5
        movup.9
    end

    exec.ext5::sub # compute y3

    movup.10

    loc_load.3
    push.0.0.0.0
    loc_loadw.2 # bring y1

    loc_load.8
    push.0.0.0.0
    loc_loadw.7 # bring y2

    exec.ext5::neq

    and # compute inf3

    movdn.5

    # finalize selection of y3

    loc_load.8
    push.0.0.0.0
    loc_loadw.7 # bring y2

    loc_load.4 # bring inf1

    if.true
        repeat.5
            movup.5
            drop
        end
    else
        repeat.5
            drop
        end
    end

    loc_load.3
    push.0.0.0.0
    loc_loadw.2 # bring y1

    loc_load.9 # bring inf2

    if.true
        repeat.5
            movup.5
            drop
        end
    else
        repeat.5
            drop
        end
    end

    # finalize selection of x3

    repeat.5
        movup.10
    end

    loc_load.6
    push.0.0.0.0
    loc_loadw.5 # bring x2

    loc_load.4 # bring inf1

    if.true
        repeat.5
            movup.5
            drop
        end
    else
        repeat.5
            drop
        end
    end

    loc_load.1
    push.0.0.0.0
    loc_loadw.0 # bring x1

    loc_load.9 # bring inf2

    if.true
        repeat.5
            movup.5
            drop
        end
    else
        repeat.5
            drop
        end
    end

    # finalize selection of inf3

    movup.10
    loc_load.9 # bring inf2
    loc_load.4 # bring inf1
    cdrop

    loc_load.4 # bring inf1
    loc_load.9 # bring inf2
    cdrop

    movdn.10
end

# Given one elliptic curve point ( say a ) as Weierstraß coordinates (X, Y) on stack,
# this routine computes elliptic curve point b s.t. b = 2 * a.
#
# Following point doubling formula is complete and it works only when input operand is
# a non-infinity point, then resulting point b should also be non-infinity.
#
# Note, result of add(a, b) = double(a) | a = b
#
# Expected stack state
#
# [x0, x1, x2, x3, x4, y0, y1, y2, y3, y4, inf, ...]
#
# s.t. x{0..5} -> x, y{0..5} -> y |> a = (x, y, inf)
#
# Final stack state
#
# [x'0, x'1, x'2, x'3, x'4, y'0, y'1, y'2, y'3, y'4, inf, ...]
#
# Read point addition section ( on page 8 ) of https://ia.cr/2022/274
# For reference implementation see https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1270-L1280
export.double.5
    loc_storew.0
    dropw
    loc_store.1 # cached x
    drop

    loc_storew.2
    dropw
    loc_store.3 # cached y
    drop

    loc_store.4 # cached inf
    drop

    loc_load.3
    push.0.0.0.0
    loc_loadw.2 # bring y

    repeat.5
        movup.4
        mul.2
    end # compute λ1

    loc_load.1
    push.0.0.0.0
    loc_loadw.0 # bring x

    exec.ext5::square

    repeat.5
        movup.4
        mul.3
    end

    add.6148914689804861439
    swap
    add.263
    swap # compute λ0

    exec.ext5::div # compute λ

    loc_load.1
    push.0.0.0.0
    loc_loadw.0 # bring x

    repeat.5
        movup.4
        mul.2
    end

    repeat.5
        dup.9
    end

    exec.ext5::square
    exec.ext5::sub # compute x'

    repeat.5
        dup.4
    end

    loc_load.1
    push.0.0.0.0
    loc_loadw.0 # bring x

    exec.ext5::sub

    repeat.5
        movup.14
    end

    exec.ext5::mul

    loc_load.3
    push.0.0.0.0
    loc_loadw.2 # bring y

    repeat.5
        movup.9
    end

    exec.ext5::sub # compute y'

    repeat.5
        movup.9
    end

    loc_load.4
    movdn.10
end
"),
// ----- std::math::ext2 --------------------------------------------------------------------------
("std::math::ext2", "# Given a stack with initial configuration given by [a1,a0,b1,b0,...] where a = (a0,a1) and
# b = (b0,b1) represent elements in the extension field of degree 2, the procedure outputs the 
# product c = (c1,c0) where c0 = a0b0 - 2(a1b1) and c1 = (a0 + a1)(b0 + b1) - a0b0
export.mul
    dupw            #[a1,a0,b1,b0,a1,a0,b1,b0,...]
    swap.3          #[b0,a0,b1,a1,a1,a0,b1,b0,...]
    mul             #[b0a0,b1,a1,a1,a0,b1,b0,...]
    dup             #[b0a0,b0a0,b1,a1,a1,a0,b1,b0,...]
    movdn.7         #[b0a0,b1,a1,a1,a0,b1,b0,b0a0,...]
    movdn.2         #[b1,a1,b0a0,a1,a0,b1,b0,b0a0,...]
    mul.2           #[2b1,a1,b0a0,a1,a0,b1,b0,b0a0,...]
    mul             #[2b1a1,b0a0,a1,a0,b1,b0,b0a0,...]
    sub             #[b0a0-2b1a1,a1,a0,b1,b0,b0a0,...]
    movdn.5         #[a1,a0,b1,b0,b0a0,b0a0-2b1a1,...]
    add             #[a1+a0,b1,b0,b0a0,b0a0-2b1a1,...]
    swap.2          #[b0,b1,a1+a0,b0a0,b0a0-2b1a1,...]
    add             #[b0+b1,a1+a0,b0a0,b0a0-2b1a1,...]
    mul             #[(b0+b1)(a1+a0),b0a0,b0a0-2b1a1,...]
    swap            #[b0a0,(b0+b1)(a1+a0),b0a0-2b1a1,...]
    sub             #[(b0+b1)(a1+a0)-b0a0,b0a0-2b1a1,...]
end

# Given a stack with initial configuration given by [x,a1,a0,...] where a = (a0,a1) is an element
# in the field extension and x is an element of the base field, this procedure computes the multiplication
# of x, when looked at as (x,0), with a in the extension field. The output is [xa1,xa0,...]
export.mul_base
    dup         #[x,x,a1,a0,...]
    movdn.3     #[x,a1,a0,x,...]
    mul         #[xa1,a0,x,...]
    movdn.2     #[a0,x,xa1,...]
    mul         #[xa0,xa1,...]
    swap        #[xa1,xa0,...]
end

# Given a stack in the following initial configuration [a1,a0,b1,b0,...] the following
# procedure computes [a1+b1,a0+b0,...]
export.add
    swap        #[a0,a1,b1,b0,...]
    movup.3     #[b0,a0,a1,b1,...]
    add         #[b0+a0,a1,b1,...]
    movdn.2     #[a1,b1,b0+a0,...]
    add         #[a1+b1,b0+a0,...]
end

# Given a stack in the following initial configuration [a1,a0,b1,b0,...] the following
# procedure computes [a1-b1,a0-b0,...]
export.sub
    swap        #[a0,a1,b1,b0,...]
    movup.3     #[b0,a0,a1,b1,...]
    sub         #[a0-b0,a1,b1,...]
    movdn.2     #[a1,b1,a0-b0,...]
    swap        #[b1,a1,a0-b0,...]
    sub         #[a1-b1,a0-b0,...]
end"),
// ----- std::math::ext5 --------------------------------------------------------------------------
("std::math::ext5", "# Given two GF(p^5) elements on stack, this routine computes modular
# addition over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#
# Expected stack state :
#
# [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#
# After application of routine stack :
#
# [c0, c1, c2, c3, c4, ...] s.t. c = a + b
#
# See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#
# For reference implementation in high level language, see 
# https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L607-L616
export.add
    repeat.5
        movup.5
        add
        movdn.4
    end
end

# Given two GF(p^5) elements on stack, this routine subtracts second
# element from first one, over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#
# Expected stack state :
#
# [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#
# After application of routine stack :
#
# [c0, c1, c2, c3, c4, ...] s.t. c = a - b
#
# See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#
# For reference implementation in high level language, see 
# https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L629-L638
export.sub
    repeat.5
        movup.5
        sub
        movdn.4
    end
end

# Given two GF(p^5) elements on stack, this routine computes modular
# multiplication ( including reduction by irreducible polynomial ) 
# over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#
# Expected stack state :
#
# [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#
# After application of routine stack :
#
# [c0, c1, c2, c3, c4, ...] s.t. c = a * b
#
# See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#
# For reference implementation in high level language, see 
# https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L676-L689
export.mul
    # compute {c0, c1, c2, c3, c4} - five coefficients of resulting
    # degree-4 polynomial
    
    # compute c4
    dup.9
    dup.1
    mul

    dup.9
    dup.3
    mul

    add

    dup.8
    dup.4
    mul

    add

    dup.7
    dup.5
    mul

    add

    dup.6
    dup.6
    mul
    
    add

    # compute c3
    dup.9
    dup.2
    mul

    dup.9
    dup.4
    mul
    
    add

    dup.8
    dup.5
    mul

    add

    dup.7
    dup.6
    mul

    add

    dup.11
    dup.7
    mul
    mul.3

    add

    # compute c2
    dup.9
    dup.3
    mul

    dup.9
    dup.5
    mul

    add

    dup.8
    dup.6
    mul
    
    add

    dup.12
    dup.7
    mul
    mul.3

    add

    dup.11
    dup.8
    mul
    mul.3

    add

    # compute c1
    dup.9
    dup.4
    mul

    dup.9
    dup.6
    mul

    add

    dup.13
    dup.7
    mul
    mul.3

    add

    dup.12
    dup.8
    mul
    mul.3

    add

    dup.11
    dup.9
    mul
    mul.3

    add

    # compute c0
    movup.9
    movup.5
    mul

    movup.12
    movup.6
    mul
    mul.3

    add

    movup.10
    movup.6
    mul
    mul.3

    add

    movup.8
    movup.6
    mul
    mul.3

    add

    movup.6
    movup.6
    mul
    mul.3

    add
end

# Given one GF(p^5) element on stack, this routine computes modular
# squaring ( including reduction by irreducible polynomial ) 
# over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#
# This routine has same effect as calling mul(a, a) | a ∈ GF(p^5)
#
# Expected stack state :
#
# [a0, a1, a2, a3, a4, ...]
#
# After application of routine stack :
#
# [b0, b1, b2, b3, b4, ...] s.t. b = a * a
#
# See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#
# For reference implementation in high level language, see 
# https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L709-L715
export.square
    # compute {b0, b1, b2, b3, b4} - five coefficients of resulting
    # degree-4 polynomial

    # compute b4
    dup.2
    dup.3
    mul

    dup.5
    dup.2
    mul
    mul.2

    add

    dup.4
    dup.3
    mul
    mul.2

    add

    # compute b3
    dup.4
    dup.2
    mul
    mul.2

    dup.4
    dup.4
    mul
    mul.2

    add

    dup.6
    dup.7
    mul
    mul.3

    add

    # compute b2
    dup.3
    dup.4
    mul
    
    dup.5
    dup.4
    mul
    mul.2

    add

    dup.7
    dup.7
    mul
    mul.6

    add

    # compute b1
    dup.4
    dup.4
    mul
    mul.2

    dup.7
    dup.8
    mul
    mul.3

    add

    dup.8
    dup.7
    mul
    mul.6

    add

    # compute b0
    dup.4
    movup.5
    mul

    movup.8
    movup.6
    mul
    mul.6

    add

    movup.6
    movup.6
    mul
    mul.6

    add
end

# Given an element a ∈ GF(p^5), this routine applies Frobenius operator
# once, raising the element to the power of p | p = 2^64 - 2^32 + 1.
#
# Expected stack state :
#
# [a0, a1, a2, a3, a4, ...]
#
# Final stack state :
#
# [b0, b1, b2, b3, b4, ...]
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L723-L737
# for reference implementation in high-level language.
proc.frobenius_once
    movup.4
    mul.1373043270956696022

    movup.4
    mul.211587555138949697

    movup.4
    mul.15820824984080659046

    movup.4
    mul.1041288259238279555

    movup.4
end

# Given an element a ∈ GF(p^5), this routine applies Frobenius operator
# twice, raising the element to the power of p^2 | p = 2^64 - 2^32 + 1.
#
# Expected stack state :
#
# [a0, a1, a2, a3, a4, ...]
#
# Final stack state :
#
# [b0, b1, b2, b3, b4, ...]
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L739-L749
# for reference implementation in high-level language.
proc.frobenius_twice
    movup.4
    mul.211587555138949697

    movup.4
    mul.1041288259238279555

    movup.4
    mul.1373043270956696022

    movup.4
    mul.15820824984080659046

    movup.4
end

# Given one GF(p^5) element on stack, this routine computes multiplicative
# inverse over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#
# Expected stack state :
#
# [a0, a1, a2, a3, a4, ...]
#
# After application of routine stack :
#
# [b0, b1, b2, b3, b4, ...] s.t. b = 1 / a
#
# See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#
# For reference implementation in high level language, see 
# https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L751-L775
#
# Note, this routine will not panic even when operand `a` is zero.
export.inv
    repeat.5
        dup.4
    end

    exec.frobenius_once # = t0

    repeat.5
        dup.4
    end

    exec.frobenius_once # = t0.frobenius_once()
    exec.mul            # = t1

    repeat.5
        dup.4
    end

    exec.frobenius_twice # = t1.frobenius_twice()
    exec.mul             # = t2

    movup.5
    dup.1
    mul

    movup.6
    dup.6
    mul
    mul.3

    add

    movup.6
    dup.5
    mul
    mul.3

    add

    movup.6
    dup.4
    mul
    mul.3

    add

    movup.6
    dup.3
    mul
    mul.3

    add                    # = t3

    dup
    push.0
    eq
    add
    inv                    # = t4

    movup.5
    dup.1
    mul

    movup.5
    dup.2
    mul

    movup.5
    dup.3
    mul

    movup.5
    dup.4
    mul

    movup.5
    movup.5
    mul
end

# Given two GF(p^5) elements ( say a, b ) on stack, this routine computes
# modular division over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#
# Expected stack state :
#
# [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#
# After application of routine stack :
#
# [c0, c1, c2, c3, c4, ...] s.t. c = a / b
#
# See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#
# For reference implementation in high level language, see 
# https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L777-L781
export.div
    repeat.5
        movup.9
    end

    exec.inv
    exec.mul
end

# Given an element v ∈ Z_q | q = 2^64 - 2^32 + 1, and n on stack, this routine
# raises it to the power 2^n, by means of n successive squarings
#
# Expected stack stack
#
# [v, n, ...] | n >= 0
#
# After finishing execution stack
#
# [v', ...] s.t. v' = v ^ (2^n)
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L461-L469
# for reference implementation in higher level language
proc.base_msquare
    swap
    dup
    neq.0

    while.true
        sub.1
        swap
        dup
        mul

        swap
        dup
        neq.0
    end

    drop
end

# Given an element v ∈ Z_q | q = 2^64 - 2^32 + 1, this routine attempts to compute
# square root of v, if that number is a square.
#
# Expected stack state :
#
# [v, ...]
#
# After finishing execution stack looks like :
#
# [v', flg, ...]
#
# If flg = 1, it denotes v' is square root of v i.e. v' * v' = v ( mod q )
# If flg = 0, then v' = 0, denoting v doesn't have a square root
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L349-L446
# for reference implementation in higher level language.
proc.base_sqrt
    dup # = x

    push.31
    swap
    exec.base_msquare # = u

    dup
    dup
    mul # = u^2

    movup.2
    dup
    eq.0
    add

    div # = v

    # j = 1
    # i = 32 - j = 31
    dup
    push.30
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.4614640910117430873
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.1753635133440165772
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 2
    # i = 32 - j = 30
    dup
    push.29
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.9123114210336311365
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.4614640910117430873
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 3
    # i = 32 - j = 29
    dup
    push.28
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.16116352524544190054
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.9123114210336311365
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 4
    # i = 32 - j = 28
    dup
    push.27
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.6414415596519834757
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.16116352524544190054
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 5
    # i = 32 - j = 27
    dup
    push.26
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.1213594585890690845
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.6414415596519834757
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 6
    # i = 32 - j = 26
    dup
    push.25
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.17096174751763063430
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.1213594585890690845
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 7
    # i = 32 - j = 25
    dup
    push.24
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.5456943929260765144
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.17096174751763063430
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 8
    # i = 32 - j = 24
    dup
    push.23
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.9713644485405565297
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.5456943929260765144
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 9
    # i = 32 - j = 23
    dup
    push.22
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.16905767614792059275
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.9713644485405565297
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 10
    # i = 32 - j = 22
    dup
    push.21
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.5416168637041100469
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.16905767614792059275
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 11
    # i = 32 - j = 21
    dup
    push.20
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.17654865857378133588
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.5416168637041100469
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 12
    # i = 32 - j = 20
    dup
    push.19
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.3511170319078647661
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.17654865857378133588
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 13
    # i = 32 - j = 19
    dup
    push.18
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.18146160046829613826
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.3511170319078647661
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 14
    # i = 32 - j = 18
    dup
    push.17
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.9306717745644682924
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.18146160046829613826
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 15
    # i = 32 - j = 17
    dup
    push.16
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.12380578893860276750
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.9306717745644682924
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 16
    # i = 32 - j = 16
    dup
    push.15
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.6115771955107415310
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.12380578893860276750
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 17
    # i = 32 - j = 15
    dup
    push.14
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.17776499369601055404
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.6115771955107415310
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 18
    # i = 32 - j = 14
    dup
    push.13
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.16207902636198568418
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.17776499369601055404
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 19
    # i = 32 - j = 13
    dup
    push.12
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.1532612707718625687
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.16207902636198568418
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 20
    # i = 32 - j = 12
    dup
    push.11
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.17492915097719143606
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.1532612707718625687
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 21
    # i = 32 - j = 11
    dup
    push.10
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.455906449640507599
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.17492915097719143606
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 22
    # i = 32 - j = 10
    dup
    push.9
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.11353340290879379826
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.455906449640507599
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 23
    # i = 32 - j = 9
    dup
    push.8
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.1803076106186727246
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.11353340290879379826
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 24
    # i = 32 - j = 8
    dup
    push.7
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.13797081185216407910
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.1803076106186727246
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 25
    # i = 32 - j = 7
    dup
    push.6
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.17870292113338400769
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.13797081185216407910
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 26
    # i = 32 - j = 6
    dup
    push.5
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.549755813888
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.17870292113338400769
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 27
    # i = 32 - j = 5
    dup
    push.4
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.70368744161280
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.549755813888
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 28
    # i = 32 - j = 4
    dup
    push.3
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.17293822564807737345
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.70368744161280
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 29
    # i = 32 - j = 3
    dup
    push.2
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.18446744069397807105
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.17293822564807737345
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 30
    # i = 32 - j = 2
    dup
    push.1
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.281474976710656
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.18446744069397807105
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap

    # j = 31
    # i = 32 - j = 1
    dup
    push.0
    swap
    exec.base_msquare # = w

    eq.18446744069414584320 # = cc

    dup.1
    mul.18446744069414584320
    movup.2
    swap
    dup.2
    cdrop # = v'

    dup.2
    mul.281474976710656
    movup.3
    swap
    movup.3
    cdrop # = u'

    swap # On stack [v, u, ...]

    dup
    eq.0
    swap
    eq.1
    or # = cc

    swap
    dup.1
    mul # On stack [u * cc, cc, ...]
end

# Given an element v ∈ Z_q | q = 2^64 - 2^32 + 1, this routine computes
# legendre symbol, by raising that element to the power (p-1) / 2
#
# Expected stack state :
#
# [v, ...]
#
# After finishing execution stack looks like
#
# [v', ...] s.t. v' = legendre symbol of v
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L448-L459
# for reference implementation in higher level language.
proc.base_legendre
    repeat.31
        dup
        mul
    end

    dup

    repeat.32
        dup
        mul
    end

    swap
    dup
    eq.0
    add

    div
end

# Given an element v ∈ GF(p^5), this routine computes its legendre symbol,
# which is an element ∈ GF(p) | p = 2^64 - 2^32 + 1
#
# At beginning stack looks like
#
# [a0, a1, a2, a3, a4, ...]
#
# At end stack looks like
#
# [b, ...] s.t. b = legendre symbol of a
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L857-L877
# for reference implementation in higher level language.
export.legendre
    repeat.5
        dup.4
    end

    exec.frobenius_once

    repeat.5
        dup.4
    end

    exec.frobenius_once
    exec.mul

    repeat.5
        dup.4
    end

    exec.frobenius_twice
    exec.mul

    movup.5
    mul

    movup.5
    movup.5
    mul
    mul.3
    
    add

    movup.4
    movup.4
    mul
    mul.3

    add

    movup.3
    movup.3
    mul
    mul.3

    add

    movup.2
    movup.2
    mul
    mul.3

    add

    exec.base_legendre
end

# Given an element v ∈ GF(p^5), this routine attempts to compute square root of v, 
# if that number is a square.
#
# At beginning stack looks like
#
# [a0, a1, a2, a3, a4, ...]
#
# At end stack looks like
#
# [b0, b1, b2, b3, b4, flg, ...]
#
# If flg = 1, it denotes v' = {b0, b1, b2, b3, b4} is square root of v i.e. v' * v' = v ( mod GF(p^5) )
# If flg = 0, then v' = {0, 0, 0, 0, 0}, denoting v doesn't have a square root
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L879-L910
# for reference implementation in higher level language.
export.sqrt
    repeat.5
        dup.4
    end

    repeat.31
        repeat.5
            dup.4
        end

        exec.mul
    end # = v

    repeat.5
        dup.4
    end

    repeat.32
        repeat.5
            dup.4
        end

        exec.mul
    end

    exec.div

    repeat.5
        dup.9
    end

    exec.mul # = d

    repeat.5
        dup.4
    end

    exec.frobenius_twice
    exec.mul
    exec.frobenius_once # = e

    repeat.5
        dup.4
    end

    exec.square # = f

    movup.10
    mul

    swap
    movup.13
    mul
    mul.3
    add

    swap
    movup.11
    mul
    mul.3
    add

    swap
    movup.9
    mul
    mul.3
    add

    swap
    movup.7
    mul
    mul.3
    add # = g

    exec.base_sqrt # On stack [s, c, e0, e1, e2, e3, e4, ...]

    repeat.5
        movup.6
    end

    exec.inv # = e'

    repeat.5
        movup.4
        dup.5
        mul
    end

    movup.5
    drop # On stack [e0, e1, e2, e3, e4, c, ...]
end

# Given two elements a, b ∈ GF(p^5), this routine produces single field element r,
# denoting whether a == b.
#
# Expected stack state 
#
# [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#
# Final stack state 
#
# [r, ...]
#
# If a == b { r = 1 } Else { r = 0 }
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L797-L806
# for reference implementation.
export.eq
    push.1

    swap
    movup.6
    eq
    and

    swap
    movup.5
    eq
    and

    swap
    movup.4
    eq
    and

    swap
    movup.3
    eq
    and

    swap
    movup.2
    eq
    and
end

# Given two elements a, b ∈ GF(p^5), this routine produces single field element r,
# denoting whether a != b.
#
# Expected stack state 
#
# [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#
# Final stack state 
#
# [r, ...]
#
# If a != b { r = 1 } Else { r = 0 }
#
# See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L813-L822
# for reference implementation.
export.neq
    push.0

    swap
    movup.6
    neq
    or

    swap
    movup.5
    neq
    or

    swap
    movup.4
    neq
    or

    swap
    movup.3
    neq
    or

    swap
    movup.2
    neq
    or
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
# Expected stack state as input:
#
# [start_addr, ...] | Single absolute memory address, where polynomial starts
#
# Note, total 128 memory addresses are required for storing whole polynomial. Next 127
# addresses are consecutive i.e. computable by using `add.1` instruction on previous address.
#
# addr{i} holds values V[(i << 2) .. ((i+1) << 2)] | i ∈ [0, 128) and addr0 = start_addr
#
# After applying NTT, bit-reversed order vector is returned back as single absolute memory
# addresses on stack, where it begins storing the polynomial. Consecutive 127 addresses should be
# computable using `add.1` instruction.
#
# [start_addr', ...] | Single absolute memory address, where resulting polynomial starts
#
# Note, input memory allocation is not mutated, instead output is stored in different memory allocation.
export.forward.128
    # prepare input

	locaddr.0
	push.0.0.0.0

	repeat.128
		dup.5
		mem_loadw

		dup.4
		mem_storew

		movup.5
		add.1
		movdn.5

		movup.4
		add.1
		movdn.4
	end

	dropw
	drop
	drop

    # iter = 0

	push.18446462594437873665.18446462594437873665.18446462594437873665.18446462594437873665
	repeat.63
		dupw
	end

	push.0.0.0.0.0.0.0.0

	locaddr.64
	movdn.8
	locaddr.0
	movdn.8

	repeat.64
		dup.8
		mem_loadw

		swapw

		dup.9
		mem_loadw

		movup.13
		movup.13
		movup.13
		movup.13

		exec.butterfly

		dup.9
		mem_storew

		swapw

		dup.8
		mem_storew

		movup.8
		add.1
		movdn.8

		movup.9
		add.1
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 1

	push.16777216.16777216.16777216.16777216
	repeat.31
		dupw
	end

	push.1099511627520.1099511627520.1099511627520.1099511627520
	repeat.31
		dupw
	end

	push.0.0.0.0
	dupw

	locaddr.32
	movdn.8
	locaddr.0
	movdn.8

	repeat.2
		repeat.32
			dup.8
			mem_loadw

			swapw

			dup.9
			mem_loadw

			movup.13
			movup.13
			movup.13
			movup.13

			exec.butterfly

			dup.9
			mem_storew

			swapw

			dup.8
			mem_storew

			movup.8
			add.1
			movdn.8

			movup.9
			add.1
			movdn.9
		end

		movup.8
		add.32
		movdn.8

		movup.9
		add.32
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 2

	push.17293822564807737345.17293822564807737345.17293822564807737345.17293822564807737345
	repeat.15
		dupw
	end

	push.4096.4096.4096.4096
	repeat.15
		dupw
	end

	push.4503599626321920.4503599626321920.4503599626321920.4503599626321920
	repeat.15
		dupw
	end

	push.18446744000695107585.18446744000695107585.18446744000695107585.18446744000695107585
	repeat.15
		dupw
	end

	push.0.0.0.0
	dupw

	locaddr.16
	movdn.8
	locaddr.0
	movdn.8

	repeat.4
		repeat.16
			dup.8
			mem_loadw

			swapw

			dup.9
			mem_loadw

			movup.13
			movup.13
			movup.13
			movup.13

			exec.butterfly

			dup.9
			mem_storew

			swapw

			dup.8
			mem_storew

			movup.8
			add.1
			movdn.8

			movup.9
			add.1
			movdn.9
		end

		movup.8
		add.16
		movdn.8

		movup.9
		add.16
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 3

	push.1073741824.1073741824.1073741824.1073741824
	repeat.7
		dupw
	end

	push.70368744161280.70368744161280.70368744161280.70368744161280
	repeat.7
		dupw
	end

	push.18428729670905102337.18428729670905102337.18428729670905102337.18428729670905102337
	repeat.7
		dupw
	end

	push.64.64.64.64
	repeat.7
		dupw
	end

	push.288230376084602880.288230376084602880.288230376084602880.288230376084602880
	repeat.7
		dupw
	end

	push.18446739671368073217.18446739671368073217.18446739671368073217.18446739671368073217
	repeat.7
		dupw
	end

	push.262144.262144.262144.262144
	repeat.7
		dupw
	end

	push.17179869180.17179869180.17179869180.17179869180
	repeat.7
		dupw
	end

	push.0.0.0.0
	dupw

	locaddr.8
	movdn.8
	locaddr.0
	movdn.8

	repeat.8
		repeat.8
			dup.8
			mem_loadw

			swapw

			dup.9
			mem_loadw

			movup.13
			movup.13
			movup.13
			movup.13

			exec.butterfly

			dup.9
			mem_storew

			swapw

			dup.8
			mem_storew

			movup.8
			add.1
			movdn.8

			movup.9
			add.1
			movdn.9
		end

		movup.8
		add.8
		movdn.8

		movup.9
		add.8
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 4

	push.9223372032559808513.9223372032559808513.9223372032559808513.9223372032559808513
	repeat.3
		dupw
	end

	push.32768.32768.32768.32768
	repeat.3
		dupw
	end

	push.36028797010575360.36028797010575360.36028797010575360.36028797010575360
	repeat.3
		dupw
	end

	push.18446743519658770433.18446743519658770433.18446743519658770433.18446743519658770433
	repeat.3
		dupw
	end

	push.134217728.134217728.134217728.134217728
	repeat.3
		dupw
	end

	push.8796093020160.8796093020160.8796093020160.8796093020160
	repeat.3
		dupw
	end

	push.18444492269600899073.18444492269600899073.18444492269600899073.18444492269600899073
	repeat.3
		dupw
	end

	push.8.8.8.8
	repeat.3
		dupw
	end

	push.2305843008676823040.2305843008676823040.2305843008676823040.2305843008676823040
	repeat.3
		dupw
	end

	push.18446708885042495489.18446708885042495489.18446708885042495489.18446708885042495489
	repeat.3
		dupw
	end

	push.2097152.2097152.2097152.2097152
	repeat.3
		dupw
	end

	push.137438953440.137438953440.137438953440.137438953440
	repeat.3
		dupw
	end

	push.18302628881338728449.18302628881338728449.18302628881338728449.18302628881338728449
	repeat.3
		dupw
	end

	push.512.512.512.512
	repeat.3
		dupw
	end

	push.562949953290240.562949953290240.562949953290240.562949953290240
	repeat.3
		dupw
	end

	push.18446744060824649729.18446744060824649729.18446744060824649729.18446744060824649729
	repeat.3
		dupw
	end

	push.0.0.0.0
	dupw

	locaddr.4
	movdn.8
	locaddr.0
	movdn.8

	repeat.16
		repeat.4
			dup.8
			mem_loadw

			swapw

			dup.9
			mem_loadw

			movup.13
			movup.13
			movup.13
			movup.13

			exec.butterfly

			dup.9
			mem_storew

			swapw

			dup.8
			mem_storew

			movup.8
			add.1
			movdn.8

			movup.9
			add.1
			movdn.9
		end

		movup.8
		add.4
		movdn.8

		movup.9
		add.4
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 5

	push.36028797018964096.36028797018964096.36028797018964096.36028797018964096
	push.36028797018964096.36028797018964096.36028797018964096.36028797018964096

	push.36028797018963840.36028797018963840.36028797018963840.36028797018963840
	push.36028797018963840.36028797018963840.36028797018963840.36028797018963840

	push.18446603329778778113.18446603329778778113.18446603329778778113.18446603329778778113
	push.18446603329778778113.18446603329778778113.18446603329778778113.18446603329778778113

	push.18446603334073745409.18446603334073745409.18446603334073745409.18446603334073745409
	push.18446603334073745409.18446603334073745409.18446603334073745409.18446603334073745409

	push.34359214072.34359214072.34359214072.34359214072
	push.34359214072.34359214072.34359214072.34359214072

	push.18446744035054321673.18446744035054321673.18446744035054321673.18446744035054321673
	push.18446744035054321673.18446744035054321673.18446744035054321673.18446744035054321673

	push.17870292113338400769.17870292113338400769.17870292113338400769.17870292113338400769
	push.17870292113338400769.17870292113338400769.17870292113338400769.17870292113338400769

	push.576469548262227968.576469548262227968.576469548262227968.576469548262227968
	push.576469548262227968.576469548262227968.576469548262227968.576469548262227968

	push.18437736732722987009.18437736732722987009.18437736732722987009.18437736732722987009
	push.18437736732722987009.18437736732722987009.18437736732722987009.18437736732722987009

	push.18437737007600893953.18437737007600893953.18437737007600893953.18437737007600893953
	push.18437737007600893953.18437737007600893953.18437737007600893953.18437737007600893953

	push.2305843009213685760.2305843009213685760.2305843009213685760.2305843009213685760
	push.2305843009213685760.2305843009213685760.2305843009213685760.2305843009213685760

	push.16140901060200882177.16140901060200882177.16140901060200882177.16140901060200882177
	push.16140901060200882177.16140901060200882177.16140901060200882177.16140901060200882177

	push.562949953421314.562949953421314.562949953421314.562949953421314
	push.562949953421314.562949953421314.562949953421314.562949953421314

	push.562949953421310.562949953421310.562949953421310.562949953421310
	push.562949953421310.562949953421310.562949953421310.562949953421310

	push.18446741870357774849.18446741870357774849.18446741870357774849.18446741870357774849
	push.18446741870357774849.18446741870357774849.18446741870357774849.18446741870357774849

	push.18446741870424883713.18446741870424883713.18446741870424883713.18446741870424883713
	push.18446741870424883713.18446741870424883713.18446741870424883713.18446741870424883713

	push.274873712576.274873712576.274873712576.274873712576
	push.274873712576.274873712576.274873712576.274873712576

	push.18446743794532483137.18446743794532483137.18446743794532483137.18446743794532483137
	push.18446743794532483137.18446743794532483137.18446743794532483137.18446743794532483137

	push.13835128420805115905.13835128420805115905.13835128420805115905.13835128420805115905
	push.13835128420805115905.13835128420805115905.13835128420805115905.13835128420805115905

	push.4611756386097823744.4611756386097823744.4611756386097823744.4611756386097823744
	push.4611756386097823744.4611756386097823744.4611756386097823744.4611756386097823744

	push.18445618152328134657.18445618152328134657.18445618152328134657.18445618152328134657
	push.18445618152328134657.18445618152328134657.18445618152328134657.18445618152328134657

	push.18445618186687873025.18445618186687873025.18445618186687873025.18445618186687873025
	push.18445618186687873025.18445618186687873025.18445618186687873025.18445618186687873025

	push.288230376151710720.288230376151710720.288230376151710720.288230376151710720
	push.288230376151710720.288230376151710720.288230376151710720.288230376151710720

	push.18158513693262871553.18158513693262871553.18158513693262871553.18158513693262871553
	push.18158513693262871553.18158513693262871553.18158513693262871553.18158513693262871553

	push.4503599627370512.4503599627370512.4503599627370512.4503599627370512
	push.4503599627370512.4503599627370512.4503599627370512.4503599627370512

	push.4503599627370480.4503599627370480.4503599627370480.4503599627370480
	push.4503599627370480.4503599627370480.4503599627370480.4503599627370480

	push.18446726476960108545.18446726476960108545.18446726476960108545.18446726476960108545
	push.18446726476960108545.18446726476960108545.18446726476960108545.18446726476960108545

	push.18446726477496979457.18446726477496979457.18446726477496979457.18446726477496979457
	push.18446726477496979457.18446726477496979457.18446726477496979457.18446726477496979457

	push.4294901759.4294901759.4294901759.4294901759
	push.4294901759.4294901759.4294901759.4294901759

	push.18446744065119551490.18446744065119551490.18446744065119551490.18446744065119551490
	push.18446744065119551490.18446744065119551490.18446744065119551490.18446744065119551490

	push.18374687574905061377.18374687574905061377.18374687574905061377.18374687574905061377
	push.18374687574905061377.18374687574905061377.18374687574905061377.18374687574905061377

	push.72058693532778496.72058693532778496.72058693532778496.72058693532778496
	push.72058693532778496.72058693532778496.72058693532778496.72058693532778496
	
	push.0.0.0.0
	dupw

	locaddr.2
	movdn.8
	locaddr.0
	movdn.8

	repeat.32
		repeat.2
			dup.8
			mem_loadw

			swapw

			dup.9
			mem_loadw

			movup.13
			movup.13
			movup.13
			movup.13

			exec.butterfly

			dup.9
			mem_storew

			swapw

			dup.8
			mem_storew

			movup.8
			add.1
			movdn.8

			movup.9
			add.1
			movdn.9
		end

		movup.8
		add.2
		movdn.8

		movup.9
		add.2
		movdn.9
	end

	drop
	drop

	dropw
	dropw

	# iter = 6

	push.10900537202625306992.10900537202625306992.10900537202625306992.10900537202625306992
	push.16016224591364643153.16016224591364643153.16016224591364643153.16016224591364643153
	push.3051558327610197629.3051558327610197629.3051558327610197629.3051558327610197629
	push.10853271128879547664.10853271128879547664.10853271128879547664.10853271128879547664
	push.5834015391316509212.5834015391316509212.5834015391316509212.5834015391316509212
	push.10967010099451201909.10967010099451201909.10967010099451201909.10967010099451201909
	push.16792080670893602455.16792080670893602455.16792080670893602455.16792080670893602455
	push.7709569171718681254.7709569171718681254.7709569171718681254.7709569171718681254
	push.10832292272906805046.10832292272906805046.10832292272906805046.10832292272906805046
	push.12079821679951430619.12079821679951430619.12079821679951430619.12079821679951430619
	push.10467450029535024137.10467450029535024137.10467450029535024137.10467450029535024137
	push.3341893669734556710.3341893669734556710.3341893669734556710.3341893669734556710
	push.4782006911144666502.4782006911144666502.4782006911144666502.4782006911144666502
	push.13797081185216407910.13797081185216407910.13797081185216407910.13797081185216407910
	push.912371727122717978.912371727122717978.912371727122717978.912371727122717978
	push.14004640413449681173.14004640413449681173.14004640413449681173.14004640413449681173
	push.9778634991702905054.9778634991702905054.9778634991702905054.9778634991702905054
	push.13949104517951277988.13949104517951277988.13949104517951277988.13949104517951277988
	push.5209436881246729393.5209436881246729393.5209436881246729393.5209436881246729393
	push.6336321165505697069.6336321165505697069.6336321165505697069.6336321165505697069
	push.5965722551466996711.5965722551466996711.5965722551466996711.5965722551466996711
	push.13039192753378044028.13039192753378044028.13039192753378044028.13039192753378044028
	push.17449332314429639298.17449332314429639298.17449332314429639298.17449332314429639298
	push.5029422726070465669.5029422726070465669.5029422726070465669.5029422726070465669
	push.1362567150328163374.1362567150328163374.1362567150328163374.1362567150328163374
	push.18142929134658341675.18142929134658341675.18142929134658341675.18142929134658341675
	push.7298973816981743824.7298973816981743824.7298973816981743824.7298973816981743824
	push.1356658891109943458.1356658891109943458.1356658891109943458.1356658891109943458
	push.9952623958621855812.9952623958621855812.9952623958621855812.9952623958621855812
	push.8288405288461869359.8288405288461869359.8288405288461869359.8288405288461869359
	push.4404853092538523347.4404853092538523347.4404853092538523347.4404853092538523347
	push.5575382163818481237.5575382163818481237.5575382163818481237.5575382163818481237
	push.4195631349813649467.4195631349813649467.4195631349813649467.4195631349813649467
	push.9274800740290006948.9274800740290006948.9274800740290006948.9274800740290006948
	push.9516004302527281633.9516004302527281633.9516004302527281633.9516004302527281633
	push.15951685255325333175.15951685255325333175.15951685255325333175.15951685255325333175
	push.7737793303239342069.7737793303239342069.7737793303239342069.7737793303239342069
	push.7059463857684370340.7059463857684370340.7059463857684370340.7059463857684370340
	push.18182056015521604139.18182056015521604139.18182056015521604139.18182056015521604139
	push.416595521271101505.416595521271101505.416595521271101505.416595521271101505
	push.281721071064741919.281721071064741919.281721071064741919.281721071064741919
	push.6336932523019185545.6336932523019185545.6336932523019185545.6336932523019185545
	push.3291437157293746400.3291437157293746400.3291437157293746400.3291437157293746400
	push.8180754653145198927.8180754653145198927.8180754653145198927.8180754653145198927
	push.1506708620263852673.1506708620263852673.1506708620263852673.1506708620263852673
	push.8215369291935911999.8215369291935911999.8215369291935911999.8215369291935911999
	push.9083829225849678056.9083829225849678056.9083829225849678056.9083829225849678056
	push.2843318466875884251.2843318466875884251.2843318466875884251.2843318466875884251
	push.6562114217670983589.6562114217670983589.6562114217670983589.6562114217670983589
	push.1135478653231209757.1135478653231209757.1135478653231209757.1135478653231209757
	push.16329239638270742865.16329239638270742865.16329239638270742865.16329239638270742865
	push.3332764170168812040.3332764170168812040.3332764170168812040.3332764170168812040
	push.2341058142559915780.2341058142559915780.2341058142559915780.2341058142559915780
	push.16933017626115159474.16933017626115159474.16933017626115159474.16933017626115159474
	push.411429644661718300.411429644661718300.411429644661718300.411429644661718300
	push.3328437340319972906.3328437340319972906.3328437340319972906.3328437340319972906
	push.12053668962110821384.12053668962110821384.12053668962110821384.12053668962110821384
	push.10382722127243543029.10382722127243543029.10382722127243543029.10382722127243543029
	push.17330401598553671485.17330401598553671485.17330401598553671485.17330401598553671485
	push.4299803665592489687.4299803665592489687.4299803665592489687.4299803665592489687
	push.7884753188935386879.7884753188935386879.7884753188935386879.7884753188935386879
	push.10105805016917838453.10105805016917838453.10105805016917838453.10105805016917838453
	push.13801972045324315718.13801972045324315718.13801972045324315718.13801972045324315718
	push.16192975500896648969.16192975500896648969.16192975500896648969.16192975500896648969

	push.0.0.0.0
	dupw

	locaddr.1
	movdn.8
	locaddr.0
	movdn.8

	repeat.64
		dup.8
		mem_loadw

		swapw

		dup.9
		mem_loadw

		movup.13
		movup.13
		movup.13
		movup.13

		exec.butterfly

		dup.9
		mem_storew

		swapw

		dup.8
		mem_storew

		movup.8
		add.2
		movdn.8

		movup.9
		add.2
		movdn.9
	end

	drop
	drop

	dropw
	dropw

	# iter = 7

	push.12418052014939319938.12418052014939319938.17799792287555502819.17799792287555502819
	push.8917938738259842505.8917938738259842505.4022135219920766353.4022135219920766353
	push.6416647500902310032.6416647500902310032.11779090253969091270.11779090253969091270
	push.1723406808235183235.1723406808235183235.15122929597976639421.15122929597976639421
	push.17345757166192390690.17345757166192390690.17608981172539450419.17608981172539450419
	push.13935318169262536835.13935318169262536835.16901410098125234092.16901410098125234092
	push.18064315379978805435.18064315379978805435.8636802660946538252.8636802660946538252
	push.15992013477438468440.15992013477438468440.13609673538787597335.13609673538787597335
	push.14439691868389311614.14439691868389311614.1999001684679808555.1999001684679808555
	push.13787254465881465880.13787254465881465880.10302972367325609442.10302972367325609442
	push.16003277697834987077.16003277697834987077.13730337689951546503.13730337689951546503
	push.13271129814541932305.13271129814541932305.11336048296972946422.11336048296972946422
	push.15387314553928353233.15387314553928353233.13754189079328553053.13754189079328553053
	push.17255643403020241594.17255643403020241594.16643667963227857075.16643667963227857075
	push.802080937612788754.802080937612788754.6084072299099782489.6084072299099782489
	push.11744640894413513105.11744640894413513105.8807895225777549048.8807895225777549048
	push.5932183857725394514.5932183857725394514.12113519742882795430.12113519742882795430
	push.3877499600194655066.3877499600194655066.17920296056720464863.17920296056720464863
	push.13682064192112842111.13682064192112842111.14583602245206205734.14583602245206205734
	push.1937996126393065589.1937996126393065589.408281368649950045.408281368649950045
	push.8352301510068328051.8352301510068328051.3200815326405523330.3200815326405523330
	push.502012629086366038.502012629086366038.7721858563281021845.7721858563281021845
	push.13351287672668691770.13351287672668691770.7683263524182218559.7683263524182218559
	push.11013340222467950926.11013340222467950926.9791607036678152304.9791607036678152304
	push.17222793189829815283.17222793189829815283.5988353545162139946.5988353545162139946
	push.15503969011144524712.15503969011144524712.3266250949199600360.3266250949199600360
	push.12573252732142656207.12573252732142656207.14235159967861628657.14235159967861628657
	push.4674437595989441835.4674437595989441835.7882761346440596851.7882761346440596851
	push.14576581034276612555.14576581034276612555.6125875985213995509.6125875985213995509
	push.14319745502085270124.14319745502085270124.4545880015766881148.4545880015766881148
	push.4016101032690928304.4016101032690928304.6434636298004421797.6434636298004421797
	push.7159778541829602319.7159778541829602319.6968564197111712876.6968564197111712876
	push.16873708294018933551.16873708294018933551.1691643236322650437.1691643236322650437
	push.12981983163322084213.12981983163322084213.8096577031901772269.8096577031901772269
	push.11441669947107069577.11441669947107069577.5240855794895625891.5240855794895625891
	push.14780429931651188987.14780429931651188987.7760115154989660995.7760115154989660995
	push.743439328957095187.743439328957095187.1672096098105064228.1672096098105064228
	push.16031446777576706363.16031446777576706363.8440569278248727675.8440569278248727675
	push.5163568085532294797.5163568085532294797.17032024114559111334.17032024114559111334
	push.3373377623857539246.3373377623857539246.5602886161730919912.5602886161730919912
	push.17746383299198219332.17746383299198219332.5033358220335838486.5033358220335838486
	push.7562975036722005970.7562975036722005970.6740689031673534997.6740689031673534997
	push.11622144959503752099.11622144959503752099.9432384046970425189.9432384046970425189
	push.13533145890581203496.13533145890581203496.12584286203165206160.12584286203165206160
	push.4415056545429189734.4415056545429189734.7128984430570800425.7128984430570800425
	push.8540276921445729647.8540276921445729647.7929601155018190654.7929601155018190654
	push.17571109804126144978.17571109804126144978.12184322017746068437.12184322017746068437
	push.13376768784840513824.13376768784840513824.12499229437757822825.12499229437757822825
	push.3264989070758626945.3264989070758626945.6396200096592884887.6396200096592884887
	push.13615673215290265491.13615673215290265491.16589430531118646239.16589430531118646239
	push.4459017075746761332.4459017075746761332.494216498237666005.494216498237666005
	push.10949047808060940701.10949047808060940701.13156576080775535568.13156576080775535568
	push.4406114516091528337.4406114516091528337.10259142034962052999.10259142034962052999
	push.3528436654823777706.3528436654823777706.12401628304422887372.12401628304422887372
	push.18209529147560584987.18209529147560584987.11917386045977981907.11917386045977981907
	push.13183111817796039999.13183111817796039999.9770812262840623888.9770812262840623888
	push.17225392536559506335.17225392536559506335.3953731985901328040.3953731985901328040
	push.13805406186829188324.13805406186829188324.13018888299131362939.13018888299131362939
	push.16691665375249202323.16691665375249202323.3588235763047079665.3588235763047079665
	push.14276112633913910454.14276112633913910454.10773575572760153082.10773575572760153082
	push.16549024694582589649.16549024694582589649.3105368020750933651.3105368020750933651
	push.13231174195295398387.13231174195295398387.4379521825066653820.4379521825066653820
	push.9780749169175637327.9780749169175637327.6979306088310177371.6979306088310177371
	push.8286160002038086708.8286160002038086708.1644572010096941946.1644572010096941946

	push.0.0.0.0
	dupw

	locaddr.1
	movdn.8
	locaddr.0
	movdn.8

	repeat.64
		dup.8
		mem_loadw

		swapw

		dup.9
		mem_loadw

		movdn.5
		movdn.5
		movup.7
		movup.7

		movup.13
		movup.13
		movup.13
		movup.13

		exec.butterfly

		movdn.5
		movdn.5
		movup.7
		movup.7

		dup.9
		mem_storew

		swapw

		dup.8
		mem_storew

		movup.8
		add.2
		movdn.8

		movup.9
		add.2
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 8

	push.18188848021460212523.11534607820881582817.1646875315973942213.5486745524883165993
	push.3642072560212772351.4877800464475578311.5575393374484204350.5907035176470557038
	push.13166299875259380027.663576273645521453.345137759837927448.16505347069079795072
	push.6337038648111976301.9115369905823964012.17031324615803662768.6715029048772165709
	push.10689836412287594487.2128915576975457846.7709658857044466158.10362793272935287662
	push.13175002527791537704.7000476060236159302.43142219979740931.2063168383634974384
	push.13802821046066641766.17582727038347959133.7123388440527211897.16826744251348157030
	push.2761102078703419584.2915568066736270329.5308610189164171624.5350065414412465710
	push.13873674549069150927.3192518480993723602.9233735841019365682.6558703133813132827
	push.16260897004766174524.7847524879092096009.5988671030957288016.12890081553990608899
	push.663283603972705376.13928631036919645866.1406998020037882997.15975288260888972401
	push.14340064592974746604.13308480401157259412.4179502387700367909.10767003651596136761
	push.959967552227305945.7439966824493015109.11015880108829135486.10886932084851949587
	push.82910450496588172.15576136931675893974.7093403778535204495.18137812093348882831
	push.4040052327310466906.14234122862185153691.14989275032188358951.12349052935110756804
	push.11255984160303063976.17121841670624273282.748583878869661002.13140475237632941313
	push.529102008834432265.6665967936588919331.9705858230261340096.8818882629200544327
	push.2623445534628784696.9513972005086392438.3418361291673462874.15984902507394762860
	push.12432372446044483551.11006166186397307298.2346834345155397801.3030959573425503682
	push.15860937903541196405.8462836655462685385.151654034847833439.16566926477903622666
	push.2540820207615693247.2324799763032802220.8900146263973118671.17198755642670596954
	push.3859889564432383484.15210828825360601653.16434255353882186006.14213927998739126201
	push.16207038811842065314.12362461035457730117.1213232278782667512.3408203337326891081
	push.327930691828598087.5800932517989445135.14262353213520121100.11221484848131637518
	push.1368250456540211762.7691156232252781355.8463154943360171265.2852412519294352506
	push.14383800816696994133.3456327113326256432.6692683090235989383.14796202496157022040
	push.6686338362028015651.16533704610107301495.12618653059398814374.4665691128499368837
	push.4056604178567881129.6173012213905610189.18290750489319984117.1773951202121591538
	push.4389942117088447138.9203872837195467135.16647976583058746422.7689155552768670394
	push.12365007338637617157.4372556084940235727.6189017649778497877.7500740417092890225
	push.14006089359128464711.12490609572415712870.17198795428657782689.14191609616972732304
	push.8715504128117593387.432040889165782054.3142428394956321713.1849525312019627755
	push.13756831773860918871.10084557685654730061.7112675246154377750.3929858786378642316
	push.4088309022520035137.6820186327231405039.11140760477401398424.12337821426711963180
	push.12489358087930152296.11703289425843512051.18222393521806856990.5006481804801239664
	push.12032395915935294938.14857320394153102038.12216811346274483113.15049383599936516047
	push.14259728110745696775.17668002479022071670.15339107541552850108.6468851066622783835
	push.1561169760991269037.12992126221614554207.6889485207579503204.625810225600154958
	push.4025446980409437899.8178098736737310378.5500770423122943299.9714604383004622450
	push.16651939688552765673.3158366299580748670.1392595059675174803.10765599713046287558
	push.10461664704817933990.8882481555027559655.6954937180696424269.1572137324173280490
	push.5665144507324065868.807842315821754643.1560799588066959011.12796895112978970121
	push.2394121898621129512.8383068400017029755.15076497439326288290.12982989459991844517
	push.7657453289212455099.7344548176412377620.14808420073763128510.6365632919551470868
	push.8427667919763358302.6462738526574037144.12486396704535672088.10141440556758839363
	push.299265237327641189.12577098593386243920.15719620231976724277.8540402708529449685
	push.5919394105455887829.3416153203055267997.7786896173617522154.14031575217582598302
	push.9931515098122800394.11630195332861834531.11724314991892485077.17740512949860132546
	push.12053974342864933269.7161240326935577237.3639634848410716242.15919780095311700439
	push.2117308758935292362.8854965448075557493.16625729085584007730.15471613066104988457
	push.11575701465310827636.4295002282146690441.15597523257926919464.3308892972056812266
	push.12582249520745188423.12505800551746292235.13315466594398149922.12066191983457963400
	push.16938470071482338896.15499491376360706981.3878624198769971593.13092440112352401730
	push.10670334717871145615.16677776346006097586.1949690407240864933.14248669673568039774
	push.8424275818888585779.7812684066897416275.14290012408112277771.4295815520590785595
	push.14099721646927849786.8024399707039913807.15913274187758939207.18074852694000884838
	push.12316227567088954246.17527399748276289503.5152080643914132488.14561398366328274390
	push.15948194847534211277.4576915052531526319.5164132063260791647.152897937997792376
	push.16138512030456545775.9592291974280344910.14948939724807468932.4971430691134054059
	push.16909802868642731951.9785468031858712064.16221402320798919601.12333197645027200248
	push.16905094363786184290.18168576350837626231.4419568367257164534.1223183503982339008
	push.4323157012483891262.5810722514138689194.11091989500308225777.12150643879775871958
	push.6151214463239765361.4496767977211359228.644010080489266561.6431860813144680379
	push.8911053381972245530.2877957390243263830.2951359516584421996.19112242249724047

	push.0.0.0.0
	dupw

	locaddr.1
	movdn.8
	locaddr.0
	movdn.8

	repeat.64
		dup.8
		mem_loadw

		swapw

		dup.9
		mem_loadw

		movup.2
		swap
		movup.6
		movup.5

		movup.5
		movup.5
		movup.7
		movup.7

		movup.13
		movup.13
		movup.13
		movup.13

		exec.butterfly

		movup.5
		swap
		movup.5

		movup.5
		movup.7
		movup.6
		movup.7

		dup.9
		mem_storew

		swapw

		dup.8
		mem_storew

		movup.8
		add.2
		movdn.8

		movup.9
		add.2
		movdn.9
	end

	drop
	drop

	dropw
	dropw

	# bit-reversed order NTT vector lives in absolute memory address
	# starting at 👇; total 128 consecutive addresses are used for storing
	# whole polynomial ( of degree 512 )

	locaddr.0
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
# producing elements in time domain in standard order, while input vector is expected to be in 
# bit-reversed order.
#
# Expected stack state as input:
#
# [start_addr, ...] | Single absolute memory address, where polynomial starts
#
# Note, total 128 memory addresses are required for storing whole polynomial. Next 127
# addresses are consecutive i.e. computable by using `add.1` instruction on previous address.
#
# addr{i} holds values V[(i << 2) .. ((i+1) << 2)] | i ∈ [0, 128) and addr0 = start_addr
#
# After applying iNTT, normal order vector is returned back as single absolute memory
# addresses on stack, where it begins storing the polynomial. Consecutive 127 addresses should 
# similarly be computable using `add.1` instruction.
#
# [start_addr', ...] | Single absolute memory address, where resulting polynomial starts
#
# Note, input memory allocation is not mutated, instead output is stored in different memory allocation.
export.backward.128
	# prepare input

	locaddr.0
	push.0.0.0.0

	repeat.128
		dup.5
		mem_loadw

		dup.4
		mem_storew

		movup.5
		add.1
		movdn.5

		movup.4
		add.1
		movdn.4
	end

	dropw
	drop
	drop

	# iter = 0

	push.18427631827164860274.15495384552830162325.15568786679171320491.9535690687442338791
	push.12014883256269903942.17802733988925317760.13949976092203225093.12295529606174818960
	push.6296100189638712363.7354754569106358544.12636021555275895127.14123587056930693059
	push.17223560565432245313.14027175702157419787.278167718576958090.1541649705628400031
	push.6113546424387384073.2225341748615664720.8661276037555872257.1536941200771852370
	push.13475313378280530262.3497804344607115389.8854452095134239411.2308232038958038546
	push.18293846131416791945.13282612006153792674.13869829016883058002.2498549221880373044
	push.3885345703086309931.13294663425500451833.919344321138294818.6130516502325630075
	push.371891375413699483.2533469881655645114.10422344362374670514.4347022422486734535
	push.14150928548823798726.4156731661302306550.10634060002517168046.10022468250525998542
	push.4198074395846544547.16497053662173719388.1768967723408486735.7776409351543438706
	push.5354303957062182591.14568119870644612728.2947252693053877340.1508273997932245425
	push.6380552085956620921.5131277475016434399.5940943517668292086.5864494548669395898
	push.15137851097357772055.2849220811487664857.14151741787267893880.6871042604103756685
	push.2975131003309595864.1821014983830576591.9591778621339026828.16329435310479291959
	push.2526963974102883882.14807109221003868079.11285503742479007084.6392769726549651052
	push.706231119554451775.6722429077522099244.6816548736552749790.8515228971291783927
	push.4415168851831986019.10659847895797062167.15030590866359316324.12527349963958696492
	push.9906341360885134636.2727123837437860044.5869645476028340401.18147478832086943132
	push.8305303512655744958.5960347364878912233.11984005542840547177.10019076149651226019
	push.12081111149863113453.3638323995651455811.11102195893002206701.10789290780202129222
	push.5463754609422739804.3370246630088296031.10063675669397554566.16052622170793454809
	push.5649848956435614200.16885944481347625310.17638901753592829678.12781599562090518453
	push.16874606745241303831.11491806888718160052.9564262514387024666.7985079364596650331
	push.7681144356368296763.17054149009739409518.15288377769833835651.1794804380861818648
	push.8732139686409961871.12945973646291641022.10268645332677273943.14421297089005146422
	push.17820933843814429363.11557258861835081117.5454617847800030114.16885574308423315284
	push.11977893002791800486.3107636527861734213.778741590392512651.4187015958668887546
	push.3397360469478068274.6229932723140101208.3589423675261482283.6414348153479289383
	push.13440262264613344657.224350547607727331.6743454643571072270.5957385981484432025
	push.6108922642702621141.7305983592013185897.11626557742183179282.14358435046894549184
	push.14516885283035942005.11334068823260206571.8362186383759854260.4689912295553665450
	push.16597218757394956566.15304315674458262608.18014703180248802267.9731239941296990934
	push.4255134452441852017.1247948640756801632.5956134496998871451.4440654710286119610
	push.10946003652321694096.12257726419636086444.14074187984474348594.6081736730776967164
	push.10757588516645913927.1798767486355837899.9242871232219117186.14056801952326137183
	push.16672792867292992783.155993580094600204.12273731855508974132.14390139890846703192
	push.13781052940915215484.5828091010015769947.1913039459307282826.11760405707386568670
	push.3650541573257562281.11754060979178594938.14990416956088327889.4062943252717590188
	push.15594331550120231815.9983589126054413056.10755587837161802966.17078493612874372559
	push.7225259221282946803.4184390855894463221.12645811551425139186.18118813377585986234
	push.15038540732087693240.17233511790631916809.6084283033956854204.2239705257572519007
	push.4232816070675458120.2012488715532398315.3235915244053982668.14586854504982200837
	push.1247988426743987367.9546597805441465650.16121944306381782101.15905923861798891074
	push.1879817591510961655.18295090034566750882.9983907413951898936.2585806165873387916
	push.15415784495989080639.16099909724259186520.7440577883017277023.6014371623370100770
	push.2461841562019821461.15028382777741121447.8932772064328191883.15823298534785799625
	push.9627861440214039994.8740885839153244225.11780776132825664990.17917642060580152056
	push.5306268831781643008.17698160190544923319.1324902398790311039.7190759909111520345
	push.6097691134303827517.3457469037226225370.4212621207229430630.14406691742104117415
	push.308931976065701490.11353340290879379826.2870607137738690347.18363833618917996149
	push.7559811984562634734.7430863960585448835.11006777244921569212.17486776517187278376
	push.7679740417818447560.14267241681714216412.5138263668257324909.4106679476439837717
	push.2471455808525611920.17039746049376701324.4518113032494938455.17783460465441878945
	push.5556662515423975422.12458073038457296305.10599219190322488312.2185847064648409797
	push.11888040935601451494.9213008228395218639.15254225588420860719.4573069520345433394
	push.13096678655002118611.13138133880250412697.15531176002678313992.15685641990711164737
	push.1619999818066427291.11323355628887372424.864017031066625188.4643923023347942555
	push.16383575685779609937.18403601849434843390.11446268009178425019.5271741541623046617
	push.8083950796479296659.10737085212370118163.16317828492439126475.7756907657126989834
	push.11731715020642418612.1415419453610921553.9331374163590620309.12109705421302608020
	push.1941397000334789249.18101606309576656873.17783167795769062868.5280444194155204294
	push.12539708892944027283.12871350694930379971.13568943604939006010.14804671509201811970
	push.12959998544531418328.16799868753440642108.6912136248533001504.257896047954371798

	push.0.0.0.0
	dupw

	locaddr.1
	movdn.8
	locaddr.0
	movdn.8

	repeat.64
		dup.8
		mem_loadw

		swapw

		dup.9
		mem_loadw

		movup.2
		swap
		movup.6
		movup.5

		movup.5
		movup.5
		movup.7
		movup.7

		movup.13
		movup.13
		movup.13
		movup.13

		exec.ibutterfly

		movup.5
		swap
		movup.5

		movup.5
		movup.7
		movup.6
		movup.7

		dup.9
		mem_storew

		swapw

		dup.8
		mem_storew

		movup.8
		add.2
		movdn.8

		movup.9
		add.2
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 1

	push.16802172059317642375.16802172059317642375.10160584067376497613.10160584067376497613
	push.11467437981104406950.11467437981104406950.8665994900238946994.8665994900238946994
	push.14067222244347930501.14067222244347930501.5215569874119185934.5215569874119185934
	push.15341376048663650670.15341376048663650670.1897719374831994672.1897719374831994672
	push.7673168496654431239.7673168496654431239.4170631435500673867.4170631435500673867
	push.14858508306367504656.14858508306367504656.1755078694165381998.1755078694165381998
	push.5427855770283221382.5427855770283221382.4641337882585395997.4641337882585395997
	push.14493012083513256281.14493012083513256281.1221351532855077986.1221351532855077986
	push.8675931806573960433.8675931806573960433.5263632251618544322.5263632251618544322
	push.6529358023436602414.6529358023436602414.237214921853999334.237214921853999334
	push.6045115764991696949.6045115764991696949.14918307414590806615.14918307414590806615
	push.8187602034452531322.8187602034452531322.14040629553323055984.14040629553323055984
	push.5290167988639048753.5290167988639048753.7497696261353643620.7497696261353643620
	push.17952527571176918316.17952527571176918316.13987726993667822989.13987726993667822989
	push.1857313538295938082.1857313538295938082.4831070854124318830.4831070854124318830
	push.12050543972821699434.12050543972821699434.15181754998655957376.15181754998655957376
	push.5947514631656761496.5947514631656761496.5069975284574070497.5069975284574070497
	push.6262422051668515884.6262422051668515884.875634265288439343.875634265288439343
	push.10517142914396393667.10517142914396393667.9906467147968854674.9906467147968854674
	push.11317759638843783896.11317759638843783896.14031687523985394587.14031687523985394587
	push.5862457866249378161.5862457866249378161.4913598178833380825.4913598178833380825
	push.9014360022444159132.9014360022444159132.6824599109910832222.6824599109910832222
	push.11706055037741049324.11706055037741049324.10883769032692578351.10883769032692578351
	push.13413385849078745835.13413385849078745835.700360770216364989.700360770216364989
	push.12843857907683664409.12843857907683664409.15073366445557045075.15073366445557045075
	push.1414719954855472987.1414719954855472987.13283175983882289524.13283175983882289524
	push.10006174791165856646.10006174791165856646.2415297291837877958.2415297291837877958
	push.16774647971309520093.16774647971309520093.17703304740457489134.17703304740457489134
	push.10686628914424923326.10686628914424923326.3666314137763395334.3666314137763395334
	push.13205888274518958430.13205888274518958430.7005074122307514744.7005074122307514744
	push.10350167037512812052.10350167037512812052.5464760906092500108.5464760906092500108
	push.16755100833091933884.16755100833091933884.1573035775395650770.1573035775395650770
	push.11478179872302871445.11478179872302871445.11286965527584982002.11286965527584982002
	push.12012107771410162524.12012107771410162524.14430643036723656017.14430643036723656017
	push.13900864053647703173.13900864053647703173.4126998567329314197.4126998567329314197
	push.12320868084200588812.12320868084200588812.3870163035137971766.3870163035137971766
	push.10563982722973987470.10563982722973987470.13772306473425142486.13772306473425142486
	push.4211584101552955664.4211584101552955664.5873491337271928114.5873491337271928114
	push.15180493120214983961.15180493120214983961.2942775058270059609.2942775058270059609
	push.12458390524252444375.12458390524252444375.1223950879584769038.1223950879584769038
	push.8655137032736432017.8655137032736432017.7433403846946633395.7433403846946633395
	push.10763480545232365762.10763480545232365762.5095456396745892551.5095456396745892551
	push.10724885506133562476.10724885506133562476.17944731440328218283.17944731440328218283
	push.15245928743009060991.15245928743009060991.10094442559346256270.10094442559346256270
	push.18038462700764634276.18038462700764634276.16508747943021518732.16508747943021518732
	push.3863141824208378587.3863141824208378587.4764679877301742210.4764679877301742210
	push.526448012694119458.526448012694119458.14569244469219929255.14569244469219929255
	push.6333224326531788891.6333224326531788891.12514560211689189807.12514560211689189807
	push.9638848843637035273.9638848843637035273.6702103175001071216.6702103175001071216
	push.12362671770314801832.12362671770314801832.17644663131801795567.17644663131801795567
	push.1803076106186727246.1803076106186727246.1191100666394342727.1191100666394342727
	push.4692554990086031268.4692554990086031268.3059429515486231088.3059429515486231088
	push.7110695772441637899.7110695772441637899.5175614254872652016.5175614254872652016
	push.4716406379463037818.4716406379463037818.2443466371579597244.2443466371579597244
	push.8143771702088974879.8143771702088974879.4659489603533118441.4659489603533118441
	push.16447742384734775766.16447742384734775766.4007052201025272707.4007052201025272707
	push.4837070530626986986.4837070530626986986.2454730591976115881.2454730591976115881
	push.9809941408468046069.9809941408468046069.382428689435778886.382428689435778886
	push.1545333971289350229.1545333971289350229.4511425900152047486.4511425900152047486
	push.837762896875133902.837762896875133902.1100986903222193631.1100986903222193631
	push.3323814471437944900.3323814471437944900.16723337261179401086.16723337261179401086
	push.6667653815445493051.6667653815445493051.12030096568512274289.12030096568512274289
	push.14424608849493817968.14424608849493817968.9528805331154741816.9528805331154741816
	push.646951781859081502.646951781859081502.6028692054475264383.6028692054475264383

	push.0.0.0.0
	dupw

	locaddr.1
	movdn.8
	locaddr.0
	movdn.8

	repeat.64
		dup.8
		mem_loadw

		swapw

		dup.9
		mem_loadw

		movdn.5
		movdn.5
		movup.7
		movup.7

		movup.13
		movup.13
		movup.13
		movup.13

		exec.ibutterfly

		movdn.5
		movdn.5
		movup.7
		movup.7

		dup.9
		mem_storew

		swapw

		dup.8
		mem_storew

		movup.8
		add.2
		movdn.8

		movup.9
		add.2
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 2

	push.2253768568517935352.2253768568517935352.2253768568517935352.2253768568517935352
	push.4644772024090268603.4644772024090268603.4644772024090268603.4644772024090268603
	push.8340939052496745868.8340939052496745868.8340939052496745868.8340939052496745868
	push.10561990880479197442.10561990880479197442.10561990880479197442.10561990880479197442
	push.14146940403822094634.14146940403822094634.14146940403822094634.14146940403822094634
	push.1116342470860912836.1116342470860912836.1116342470860912836.1116342470860912836
	push.8064021942171041292.8064021942171041292.8064021942171041292.8064021942171041292
	push.6393075107303762937.6393075107303762937.6393075107303762937.6393075107303762937
	push.15118306729094611415.15118306729094611415.15118306729094611415.15118306729094611415
	push.18035314424752866021.18035314424752866021.18035314424752866021.18035314424752866021
	push.1513726443299424847.1513726443299424847.1513726443299424847.1513726443299424847
	push.16105685926854668541.16105685926854668541.16105685926854668541.16105685926854668541
	push.15113979899245772281.15113979899245772281.15113979899245772281.15113979899245772281
	push.2117504431143841456.2117504431143841456.2117504431143841456.2117504431143841456
	push.17311265416183374564.17311265416183374564.17311265416183374564.17311265416183374564
	push.11884629851743600732.11884629851743600732.11884629851743600732.11884629851743600732
	push.15603425602538700070.15603425602538700070.15603425602538700070.15603425602538700070
	push.9362914843564906265.9362914843564906265.9362914843564906265.9362914843564906265
	push.10231374777478672322.10231374777478672322.10231374777478672322.10231374777478672322
	push.16940035449150731648.16940035449150731648.16940035449150731648.16940035449150731648
	push.10265989416269385394.10265989416269385394.10265989416269385394.10265989416269385394
	push.15155306912120837921.15155306912120837921.15155306912120837921.15155306912120837921
	push.12109811546395398776.12109811546395398776.12109811546395398776.12109811546395398776
	push.18165022998349842402.18165022998349842402.18165022998349842402.18165022998349842402
	push.18030148548143482816.18030148548143482816.18030148548143482816.18030148548143482816
	push.264688053892980182.264688053892980182.264688053892980182.264688053892980182
	push.11387280211730213981.11387280211730213981.11387280211730213981.11387280211730213981
	push.10708950766175242252.10708950766175242252.10708950766175242252.10708950766175242252
	push.2495058814089251146.2495058814089251146.2495058814089251146.2495058814089251146
	push.8930739766887302688.8930739766887302688.8930739766887302688.8930739766887302688
	push.9171943329124577373.9171943329124577373.9171943329124577373.9171943329124577373
	push.14251112719600934854.14251112719600934854.14251112719600934854.14251112719600934854
	push.12871361905596103084.12871361905596103084.12871361905596103084.12871361905596103084
	push.14041890976876060974.14041890976876060974.14041890976876060974.14041890976876060974
	push.10158338780952714962.10158338780952714962.10158338780952714962.10158338780952714962
	push.8494120110792728509.8494120110792728509.8494120110792728509.8494120110792728509
	push.17090085178304640863.17090085178304640863.17090085178304640863.17090085178304640863
	push.11147770252432840497.11147770252432840497.11147770252432840497.11147770252432840497
	push.303814934756242646.303814934756242646.303814934756242646.303814934756242646
	push.17084176919086420947.17084176919086420947.17084176919086420947.17084176919086420947
	push.13417321343344118652.13417321343344118652.13417321343344118652.13417321343344118652
	push.997411754984945023.997411754984945023.997411754984945023.997411754984945023
	push.5407551316036540293.5407551316036540293.5407551316036540293.5407551316036540293
	push.12481021517947587610.12481021517947587610.12481021517947587610.12481021517947587610
	push.12110422903908887252.12110422903908887252.12110422903908887252.12110422903908887252
	push.13237307188167854928.13237307188167854928.13237307188167854928.13237307188167854928
	push.4497639551463306333.4497639551463306333.4497639551463306333.4497639551463306333
	push.8668109077711679267.8668109077711679267.8668109077711679267.8668109077711679267
	push.4442103655964903148.4442103655964903148.4442103655964903148.4442103655964903148
	push.17534372342291866343.17534372342291866343.17534372342291866343.17534372342291866343
	push.4649662884198176411.4649662884198176411.4649662884198176411.4649662884198176411
	push.13664737158269917819.13664737158269917819.13664737158269917819.13664737158269917819
	push.15104850399680027611.15104850399680027611.15104850399680027611.15104850399680027611
	push.7979294039879560184.7979294039879560184.7979294039879560184.7979294039879560184
	push.6366922389463153702.6366922389463153702.6366922389463153702.6366922389463153702
	push.7614451796507779275.7614451796507779275.7614451796507779275.7614451796507779275
	push.10737174897695903067.10737174897695903067.10737174897695903067.10737174897695903067
	push.1654663398520981866.1654663398520981866.1654663398520981866.1654663398520981866
	push.7479733969963382412.7479733969963382412.7479733969963382412.7479733969963382412
	push.12612728678098075109.12612728678098075109.12612728678098075109.12612728678098075109
	push.7593472940535036657.7593472940535036657.7593472940535036657.7593472940535036657
	push.15395185741804386692.15395185741804386692.15395185741804386692.15395185741804386692
	push.2430519478049941168.2430519478049941168.2430519478049941168.2430519478049941168
	push.7546206866789277329.7546206866789277329.7546206866789277329.7546206866789277329

	push.0.0.0.0
	dupw

	locaddr.1
	movdn.8
	locaddr.0
	movdn.8

	repeat.64
		dup.8
		mem_loadw

		swapw

		dup.9
		mem_loadw

		movup.13
		movup.13
		movup.13
		movup.13

		exec.ibutterfly

		dup.9
		mem_storew

		swapw

		dup.8
		mem_storew

		movup.8
		add.2
		movdn.8

		movup.9
		add.2
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 3

    push.18374685375881805825.18374685375881805825.18374685375881805825.18374685375881805825
    push.18374685375881805825.18374685375881805825.18374685375881805825.18374685375881805825

    push.72056494509522944.72056494509522944.72056494509522944.72056494509522944
    push.72056494509522944.72056494509522944.72056494509522944.72056494509522944

    push.4295032831.4295032831.4295032831.4295032831
    push.4295032831.4295032831.4295032831.4295032831

    push.18446744065119682562.18446744065119682562.18446744065119682562.18446744065119682562
    push.18446744065119682562.18446744065119682562.18446744065119682562.18446744065119682562

    push.17591917604864.17591917604864.17591917604864.17591917604864
    push.17591917604864.17591917604864.17591917604864.17591917604864

    push.17592454475776.17592454475776.17592454475776.17592454475776
    push.17592454475776.17592454475776.17592454475776.17592454475776

    push.18442240469787213841.18442240469787213841.18442240469787213841.18442240469787213841
    push.18442240469787213841.18442240469787213841.18442240469787213841.18442240469787213841

    push.18442240469787213809.18442240469787213809.18442240469787213809.18442240469787213809
    push.18442240469787213809.18442240469787213809.18442240469787213809.18442240469787213809

    push.288230376151712768.288230376151712768.288230376151712768.288230376151712768
    push.288230376151712768.288230376151712768.288230376151712768.288230376151712768

    push.18158513693262873601.18158513693262873601.18158513693262873601.18158513693262873601
    push.18158513693262873601.18158513693262873601.18158513693262873601.18158513693262873601

    push.1125882726711296.1125882726711296.1125882726711296.1125882726711296
    push.1125882726711296.1125882726711296.1125882726711296.1125882726711296

    push.1125917086449664.1125917086449664.1125917086449664.1125917086449664
    push.1125917086449664.1125917086449664.1125917086449664.1125917086449664

    push.13834987683316760577.13834987683316760577.13834987683316760577.13834987683316760577
    push.13834987683316760577.13834987683316760577.13834987683316760577.13834987683316760577

    push.4611615648609468416.4611615648609468416.4611615648609468416.4611615648609468416
    push.4611615648609468416.4611615648609468416.4611615648609468416.4611615648609468416

    push.274882101184.274882101184.274882101184.274882101184
    push.274882101184.274882101184.274882101184.274882101184

    push.18446743794540871745.18446743794540871745.18446743794540871745.18446743794540871745
    push.18446743794540871745.18446743794540871745.18446743794540871745.18446743794540871745

    push.2198989700608.2198989700608.2198989700608.2198989700608
    push.2198989700608.2198989700608.2198989700608.2198989700608

    push.2199056809472.2199056809472.2199056809472.2199056809472
    push.2199056809472.2199056809472.2199056809472.2199056809472

    push.18446181119461163011.18446181119461163011.18446181119461163011.18446181119461163011
    push.18446181119461163011.18446181119461163011.18446181119461163011.18446181119461163011

    push.18446181119461163007.18446181119461163007.18446181119461163007.18446181119461163007
    push.18446181119461163007.18446181119461163007.18446181119461163007.18446181119461163007

    push.2305843009213702144.2305843009213702144.2305843009213702144.2305843009213702144
    push.2305843009213702144.2305843009213702144.2305843009213702144.2305843009213702144

    push.16140901060200898561.16140901060200898561.16140901060200898561.16140901060200898561
    push.16140901060200898561.16140901060200898561.16140901060200898561.16140901060200898561

    push.9007061813690368.9007061813690368.9007061813690368.9007061813690368
    push.9007061813690368.9007061813690368.9007061813690368.9007061813690368

    push.9007336691597312.9007336691597312.9007336691597312.9007336691597312
    push.9007336691597312.9007336691597312.9007336691597312.9007336691597312

    push.17870274521152356353.17870274521152356353.17870274521152356353.17870274521152356353
    push.17870274521152356353.17870274521152356353.17870274521152356353.17870274521152356353

    push.576451956076183552.576451956076183552.576451956076183552.576451956076183552
    push.576451956076183552.576451956076183552.576451956076183552.576451956076183552

	push.34360262648.34360262648.34360262648.34360262648
	push.34360262648.34360262648.34360262648.34360262648

    push.18446744035055370249.18446744035055370249.18446744035055370249.18446744035055370249
    push.18446744035055370249.18446744035055370249.18446744035055370249.18446744035055370249

    push.140735340838912.140735340838912.140735340838912.140735340838912
    push.140735340838912.140735340838912.140735340838912.140735340838912

    push.140739635806208.140739635806208.140739635806208.140739635806208
    push.140739635806208.140739635806208.140739635806208.140739635806208

    push.18410715272395620481.18410715272395620481.18410715272395620481.18410715272395620481
    push.18410715272395620481.18410715272395620481.18410715272395620481.18410715272395620481

    push.18410715272395620225.18410715272395620225.18410715272395620225.18410715272395620225
    push.18410715272395620225.18410715272395620225.18410715272395620225.18410715272395620225

	push.0.0.0.0
	dupw

	locaddr.2
	movdn.8
	locaddr.0
	movdn.8

	repeat.32
		repeat.2
			dup.8
			mem_loadw

			swapw

			dup.9
			mem_loadw

			movup.13
			movup.13
			movup.13
			movup.13

			exec.ibutterfly

			dup.9
			mem_storew

			swapw

			dup.8
			mem_storew

			movup.8
			add.1
			movdn.8

			movup.9
			add.1
			movdn.9
		end

		movup.8
		add.2
		movdn.8

		movup.9
		add.2
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 4

	push.8589934592.8589934592.8589934592.8589934592
	repeat.3
		dupw
	end

	push.18446181119461294081.18446181119461294081.18446181119461294081.18446181119461294081
	repeat.3
		dupw
	end

	push.18446744069414583809.18446744069414583809.18446744069414583809.18446744069414583809
	repeat.3
		dupw
	end

	push.144115188075855872.144115188075855872.144115188075855872.144115188075855872
	repeat.3
		dupw
	end

	push.18446743931975630881.18446743931975630881.18446743931975630881.18446743931975630881
	repeat.3
		dupw
	end

	push.18446744069412487169.18446744069412487169.18446744069412487169.18446744069412487169
	repeat.3
		dupw
	end

	push.35184372088832.35184372088832.35184372088832.35184372088832
	repeat.3
		dupw
	end

	push.16140901060737761281.16140901060737761281.16140901060737761281.16140901060737761281
	repeat.3
		dupw
	end

	push.18446744069414584313.18446744069414584313.18446744069414584313.18446744069414584313
	repeat.3
		dupw
	end

	push.2251799813685248.2251799813685248.2251799813685248.2251799813685248
	repeat.3
		dupw
	end

	push.18446735273321564161.18446735273321564161.18446735273321564161.18446735273321564161
	repeat.3
		dupw
	end

	push.18446744069280366593.18446744069280366593.18446744069280366593.18446744069280366593
	repeat.3
		dupw
	end

	push.549755813888.549755813888.549755813888.549755813888
	repeat.3
		dupw
	end

	push.18410715272404008961.18410715272404008961.18410715272404008961.18410715272404008961
	repeat.3
		dupw
	end

	push.18446744069414551553.18446744069414551553.18446744069414551553.18446744069414551553
	repeat.3
		dupw
	end

	push.9223372036854775808.9223372036854775808.9223372036854775808.9223372036854775808
	repeat.3
		dupw
	end

	push.0.0.0.0
	dupw

	locaddr.4
	movdn.8
	locaddr.0
	movdn.8

	repeat.16
		repeat.4
			dup.8
			mem_loadw

			swapw

			dup.9
			mem_loadw

			movup.13
			movup.13
			movup.13
			movup.13

			exec.ibutterfly

			dup.9
			mem_storew

			swapw

			dup.8
			mem_storew

			movup.8
			add.1
			movdn.8

			movup.9
			add.1
			movdn.9
		end

		movup.8
		add.4
		movdn.8

		movup.9
		add.4
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 5

	push.18446744052234715141.18446744052234715141.18446744052234715141.18446744052234715141
	repeat.7
		dupw
	end

	push.18446744069414322177.18446744069414322177.18446744069414322177.18446744069414322177
	repeat.7
		dupw
	end

	push.4398046511104.4398046511104.4398046511104.4398046511104
	repeat.7
		dupw
	end

	push.18158513693329981441.18158513693329981441.18158513693329981441.18158513693329981441
	repeat.7
		dupw
	end

	push.18446744069414584257.18446744069414584257.18446744069414584257.18446744069414584257
	repeat.7
		dupw
	end

	push.18014398509481984.18014398509481984.18014398509481984.18014398509481984
	repeat.7
		dupw
	end

	push.18446673700670423041.18446673700670423041.18446673700670423041.18446673700670423041
	repeat.7
		dupw
	end

	push.18446744068340842497.18446744068340842497.18446744068340842497.18446744068340842497
	repeat.7
		dupw
	end

	push.0.0.0.0
	dupw

	locaddr.8
	movdn.8
	locaddr.0
	movdn.8

	repeat.8
		repeat.8
			dup.8
			mem_loadw

			swapw

			dup.9
			mem_loadw

			movup.13
			movup.13
			movup.13
			movup.13

			exec.ibutterfly

			dup.9
			mem_storew

			swapw

			dup.8
			mem_storew

			movup.8
			add.1
			movdn.8

			movup.9
			add.1
			movdn.9
		end

		movup.8
		add.8
		movdn.8

		movup.9
		add.8
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 6

	push.68719476736.68719476736.68719476736.68719476736
	repeat.15
		dupw
	end

	push.18442240469788262401.18442240469788262401.18442240469788262401.18442240469788262401
	repeat.15
		dupw
	end

	push.18446744069414580225.18446744069414580225.18446744069414580225.18446744069414580225
	repeat.15
		dupw
	end

	push.1152921504606846976.1152921504606846976.1152921504606846976.1152921504606846976
	repeat.15
		dupw
	end

	push.0.0.0.0
	dupw

	locaddr.16
	movdn.8
	locaddr.0
	movdn.8

	repeat.4
		repeat.16
			dup.8
			mem_loadw

			swapw

			dup.9
			mem_loadw

			movup.13
			movup.13
			movup.13
			movup.13

			exec.ibutterfly

			dup.9
			mem_storew

			swapw

			dup.8
			mem_storew

			movup.8
			add.1
			movdn.8

			movup.9
			add.1
			movdn.9
		end

		movup.8
		add.16
		movdn.8

		movup.9
		add.16
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 7

	push.18446742969902956801.18446742969902956801.18446742969902956801.18446742969902956801
	repeat.31
		dupw
	end

	push.18446744069397807105.18446744069397807105.18446744069397807105.18446744069397807105
	repeat.31
		dupw
	end

	push.0.0.0.0
	dupw

	locaddr.32
	movdn.8
	locaddr.0
	movdn.8

	repeat.2
		repeat.32
			dup.8
			mem_loadw

			swapw

			dup.9
			mem_loadw

			movup.13
			movup.13
			movup.13
			movup.13

			exec.ibutterfly

			dup.9
			mem_storew

			swapw

			dup.8
			mem_storew

			movup.8
			add.1
			movdn.8

			movup.9
			add.1
			movdn.9
		end

		movup.8
		add.32
		movdn.8

		movup.9
		add.32
		movdn.9
	end

	drop
	drop

	dropw
	dropw

    # iter = 8

	push.281474976710656.281474976710656.281474976710656.281474976710656
	repeat.63
		dupw
	end

	push.0.0.0.0
	dupw

	locaddr.64
	movdn.8
	locaddr.0
	movdn.8

	repeat.64
		dup.8
		mem_loadw

		swapw

		dup.9
		mem_loadw

		movup.13
		movup.13
		movup.13
		movup.13

		exec.ibutterfly

		dup.9
		mem_storew

		swapw

		dup.8
		mem_storew

		movup.8
		add.1
		movdn.8

		movup.9
		add.1
		movdn.9
	end

	drop
	drop

	# multiply by inverse of N (= 512)

	dropw

	locaddr.0
	movdn.4

	repeat.128
		dup.4
		mem_loadw

		exec.mul_by_invN

		dup.4
		mem_storew

		movup.4
		add.1
		movdn.4
	end

	dropw
	drop

	# normal order iNTT vector lives in absolute memory address
	# starting at 👇; total 128 consecutive addresses are used for storing
	# whole polynomial ( of degree 512 )

    locaddr.0
end
"),
// ----- std::math::poly512 -----------------------------------------------------------------------
("std::math::poly512", "use.std::math::ntt512
use.std::math::u64

# Given two consecutive words on stack, this routine performs 
# element wise multiplication, while keeping resulting single
# word on stack.
#
# Expected stack state looks like
#
# [a0, a1, a2, a3, b0, b1, b2, b3]
#
# What this routine does is
#
# c`i` = a`i` * b`i` mod P | i ∈ [0, 4), P = 2 ^ 64 - 2 ^ 32 + 1
#
# Output stack state looks like
#
# [c0, c1, c2, c3]
proc.mul_word
    movup.4
    mul
    movdn.6

    movup.3
    mul
    movdn.5

    movup.2
    mul
    movdn.4

    mul
    movdn.3
end

# Given two consecutive words on stack, this routine performs 
# element wise addition, while keeping resulting single
# word on stack.
#
# Expected stack state looks like
#
# [a0, a1, a2, a3, b0, b1, b2, b3]
#
# What this routine does is
#
# c`i` = a`i` + b`i` mod P | i ∈ [0, 4), P = 2 ^ 64 - 2 ^ 32 + 1
#
# Output stack state looks like
#
# [c0, c1, c2, c3]
proc.add_word
    movup.4
    add
    movdn.6

    movup.3
    add
    movdn.5

    movup.2
    add
    movdn.4

    add
    movdn.3
end

# Given dividend ( i.e. field element a ) on stack top, this routine computes c = a % 12289
#
# Expected stack state
#
# [a, ...]
#
# Output stack state looks like
#
# [c, ...] | c = a % 12289
export.mod_12289
    u32split
    push.12289.0

    adv.u64div

    adv_push.2
    u32assert.2

    swap
    push.12289
    u32overflowing_mul

    movup.2
    push.12289
    u32overflowing_madd
    drop

    adv_push.2
    drop
    u32assert

    dup

    movup.3
    u32overflowing_add

    movup.3
    u32overflowing_add
    drop

    movup.5
    assert_eq
    movup.4
    assert_eq

    swap
    drop
    swap
    drop
end

# Given four elements on stack top, this routine reduces them by applying
# modular division by 12289 ( = Falcon Signature Algorithm's Prime Number )
#
# Input stack state :
#
# [a0, a1, a3, a3, ...]
#
# Operated such that
#
# b`i` = a`i` % 12289 | i ∈ [0..4)
#
# Output stack state :
#
# [b0, b1, b2, b3, ...]
proc.mod_12289_word
    exec.mod_12289

    swap
    exec.mod_12289
    swap

    movup.2
    exec.mod_12289
    movdn.2

    movup.3
    exec.mod_12289
    movdn.3
end

# Given an operand on stack, this routine negates the element, using modular arithmetic
# over Falcon Digital Signature Algorithm's prime field = 12289.
#
# All this routine does is
#
# b = (0 - a) % Q
#   = Q - a % Q | Q = 12289
#
# Input stack state
#
# [a,  ...]
#
# Output stack state looks like
#
# [b, ...] | b ∈ [0..12289)
proc.neg
    exec.mod_12289

    push.12289
    swap
    sub
end

# Given four elements on stack, this routine negates those, using modular arithmetic
# over Falcon Digital Signature Algorithm's prime field = 12289.
#
# All this routine does is
#
# b`i` = (0 - a`i`) % Q
#   = Q - a`i` % Q | Q = 12289 & i ∈ [0..4)
#
# Input stack state
#
# [a0, a1, a2, a3, ...]
#
# Output stack state looks like
#
# [b0, b1, b2, b3 ...] | b`i` ∈ [0..12289)
proc.neg_word
    exec.neg

    swap
    exec.neg
    swap

    movup.2
    exec.neg
    movdn.2

    movup.3
    exec.neg
    movdn.3
end

# Given a field element, this routine does centered reduction using Miden VM
# prime ( say Q ) and then reduces it using Falcon Post Quantum Digital 
# Signature Algorithm prime ( say Q' )
#
# Q = 2 ^ 64 - 2 ^ 32 + 1
# Q' = 12289
#
# Expected stack state
#
# [a, ...]
#
# All this routine does is
#
# if a > (Q >> 1):
#   b = (a - Q) % Q'
# else:
#   b = a % Q'
#
# Final stack state looks like
#
# [b, ...]
proc.reduce
    dup
    push.9223372034707292160
    gt

    if.true
        exec.mod_12289

        dup
        push.7002
        u32unchecked_gte

        if.true
            sub.7002
        else
            push.7002
            swap
            sub

            push.12289
            swap
            sub
        end
    else
        exec.mod_12289
    end
end

# Reduces four consecutive elements living on stack top using `reduce` routine ( defined above )
#
# Expected stack state
#
# [a0, a1, a2, a3, ...]
#
# What this routine does is
#
# b`i` = reduce(a`i`)
#
# Final stack state looks like
#
# [b0, b1, b2, b3, ...]
proc.reduce_word
    exec.reduce

    swap
    exec.reduce
    swap

    movup.2
    exec.reduce
    movdn.2

    movup.3
    exec.reduce
    movdn.3
end

# Given two polynomials of degree 512 on stack as absolute memory addresses,
# this routine computes polynomial multiplication, using NTT and iNTT.
#
# Imagine, two polynomials are f, g
#
# h = f . g, can be computed using
#
# iNTT(NTT(f) * NTT(g))
#
# Note, * -> element wise multiplication of polynomial coefficients in NTT domain
#
# Input stack state :
#
# [f_start_addr, g_start_addr, h_start_addr, ...]
#
# - {f, g, h}_addr`i` -> {f, g, h}[ (i << 2) .. ((i+1) << 2) ), address holding four consecutive coefficients
# - {f, g, h}_addr0 -> {f, g, h}_start_addr
#
# Output stack state :
#
# [ ... ]
#
# Consecutive 127 memory addresses can be computed from starting memory address ( living on stack top ) by 
# continuing to apply `INCR` ( = add.1 ) instruction on previous absolute memory address.
#
# Note, input memory addresses are considered to be read-only, they are not mutated.
export.mul_zq.128
    exec.ntt512::forward

    locaddr.0
    push.0.0.0.0

    repeat.128
        dup.5
        mem_loadw

        dup.4
        mem_storew

        movup.5
        add.1
        movdn.5

        movup.4
        add.1
        movdn.4
    end

    dropw
    drop
    drop

    exec.ntt512::forward

    locaddr.0
    push.0.0.0.0.0.0.0.0

    repeat.128
        dup.9
        mem_loadw

        swapw

        dup.8
        mem_loadw

        exec.mul_word

        dup.4
        mem_storew

        movup.5
        add.1
        movdn.5

        movup.4
        add.1
        movdn.4

        push.0.0.0.0
    end

    dropw
    dropw
    drop
    drop

    locaddr.0

    exec.ntt512::backward

    push.0.0.0.0

    repeat.128
        dup.4
        mem_loadw

        exec.reduce_word

        dup.5
        mem_storew

        movup.5
        add.1
        movdn.5

        movup.4
        add.1
        movdn.4
    end

    dropw
    drop
    drop
end

# Given two polynomials of degree 512 on stack as absolute memory addresses,
# this routine computes polynomial addition.
#
# Imagine, two polynomials f, g
#
# h = f + g, can be computed as
#
# [(f[i] + g[i]) % Q for i in range(512)] | Q = 12289 ( = Falcon Digital Signature Algorithm's Prime Number )
#
# Input stack state :
#
# [f_start_addr, g_start_addr, h_start_addr, ...]
#
# - {f, g, h}_addr`i` -> {f, g, h}[ (i << 2) .. ((i+1) << 2) ), address holding four consecutive coefficients
# - {f, g, h}_addr0 -> {f, g, h}_start_addr
#
# Output stack state :
#
# [ ... ]
#
# Consecutive 127 memory addresses can be computed from starting memory address ( living on stack top ) by 
# continuing to apply `INCR` ( = add.1 ) instruction on previous absolute memory address.
#
# Note, input memory addresses are considered to be read-only, they are not mutated.
export.add_zq
    push.0.0.0.0.0.0.0.0

    repeat.128
        dup.8
        mem_loadw

        swapw

        dup.9
        mem_loadw

        exec.add_word
        exec.mod_12289_word

        dup.6
        mem_storew

        movup.6
        add.1
        movdn.6

        movup.5
        add.1
        movdn.5

        movup.4
        add.1
        movdn.4

        push.0.0.0.0
    end

    push.0
    dropw
    dropw
    dropw
end

# Given one polynomial of degree 512 on stack as absolute memory addresses,
# this routine negates each coefficient of that polynomial.
#
# Imagine, polynomial f
#
# g = -f, can be computed as
#
# [(-f[i]) % Q for i in range(512)] | Q = 12289 ( = Falcon Digital Signature Algorithm's Prime Number )
#
# Input stack state :
#
# [f_start_addr, g_start_addr, ...]
#
# - {f,g}_addr`i` -> {f,g}[ (i << 2) .. ((i+1) << 2) ), address holding four consecutive coefficients
# - {f,g}_addr0 -> {f,g}_start_addr
#
# Output stack state :
#
# [ ... ]
#
# Consecutive 127 memory addresses can be computed from starting memory address ( living on stack top ) by 
# continuing to apply `INCR` ( = add.1 ) instruction on previous absolute memory address.
#
# Note, input memory addresses are considered to be read-only, they are not mutated.
export.neg_zq
    push.0.0.0.0

    repeat.128
        dup.4
        mem_loadw

        exec.neg_word

        dup.5
        mem_storew

        movup.5
        add.1
        movdn.5

        movup.4
        add.1
        movdn.4
    end

    dropw
    drop
    drop
end

# Given two polynomials of degree 512 on stack as absolute memory addresses,
# this routine subtracts second polynomial from first one.
#
# Imagine, two polynomials f, g
#
# h = f - g, can be computed as
#
# [(f[i] - g[i]) % Q for i in range(512)] | Q = 12289 ( = Falcon Digital Signature Algorithm's Prime Number )
#
# Input stack state :
#
# [f_start_addr, g_start_addr, h_start_addr ...]
#
# - {f, g, h}_addr`i` -> {f, g, h}[ (i << 2) .. ((i+1) << 2) ), address holding four consecutive coefficients
# - {f, g, h}_addr0 -> {f, g, h}_start_addr
#
# Output stack state :
#
# [ ... ]
#
# Consecutive 127 memory addresses can be computed from starting memory address ( living on stack top ) by 
# continuing to apply `INCR` ( = add.1 ) instruction on previous absolute memory address.
#
# Note, input memory addresses are considered to be read-only, they are not mutated.
export.sub_zq.128
    locaddr.0
    movup.2
    exec.neg_zq

    locaddr.0
    exec.add_zq
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
  loc_storew.0
  swapw
  loc_storew.1
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
  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0

  exec.u256xu32
  exec.u288_add_u256
  exec.u288_reduce

  movup.9
  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0

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
  push.0.0.0.0
  movup.4
  mem_loadw
  dup.6
  push.0.0.0.0
  movup.4
  mem_loadw         # y -coordinate on stack top

  dupw.1
  dupw.1            # repeated y -coordinate

  exec.u256_mod_mul # = t0

  loc_storew.0
  swapw
  loc_storew.1
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

  loc_storew.2
  dropw       
  loc_storew.3
  dropw             # cache z3

  dup.5
  push.0.0.0.0
  movup.4
  mem_loadw
  dup.8
  push.0.0.0.0
  movup.4
  mem_loadw         # z -coordinate on stack top

  dup.11
  push.0.0.0.0
  movup.4
  mem_loadw
  dup.14
  push.0.0.0.0
  movup.4
  mem_loadw         # y -coordinate on stack top

  exec.u256_mod_mul # = t1

  loc_storew.4
  dropw       
  loc_storew.5
  dropw             # cache t1

  dup.5
  push.0.0.0.0
  movup.4
  mem_loadw
  dup.8
  push.0.0.0.0
  movup.4
  mem_loadw         # z -coordinate on stack top

  dupw.1
  dupw.1            # repeated z

  exec.u256_mod_mul # = t2

  push.0.0.0.0
  push.0.0.21.20517 # = b3

  exec.u256_mod_mul # = t2

  loc_storew.6
  swapw
  loc_storew.7    # cache t2
  swapw

  push.0.0.0.0
  loc_loadw.3
  push.0.0.0.0
  loc_loadw.2     # = z3

  exec.u256_mod_mul # = x3

  loc_storew.8
  dropw       
  loc_storew.9
  dropw             # cache x3

  push.0.0.0.0
  loc_loadw.7
  push.0.0.0.0
  loc_loadw.6     # = t2

  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0     # = t0

  exec.u256_mod_add # = y3

  loc_storew.10
  dropw       
  loc_storew.11
  dropw           # cache y3

  push.0.0.0.0
  loc_loadw.5
  push.0.0.0.0
  loc_loadw.4     # = t1

  push.0.0.0.0
  loc_loadw.3
  push.0.0.0.0
  loc_loadw.2     # = z3

  exec.u256_mod_mul # = z3

  loc_storew.2
  dropw       
  loc_storew.3
  dropw             # cache z3

  push.0.0.0.0
  loc_loadw.7
  push.0.0.0.0
  loc_loadw.6     # = t2

  dupw.1
  dupw.1            # repeated t2

  exec.u256_mod_add # = t1

  push.0.0.0.0
  loc_loadw.7
  push.0.0.0.0
  loc_loadw.6     # = t2

  exec.u256_mod_add # = t2

  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0     # = t0

  exec.u256_mod_sub # = t0

  loc_storew.0
  swapw
  loc_storew.1
  swapw             # cache t0

  push.0.0.0.0
  loc_loadw.11
  push.0.0.0.0
  loc_loadw.10    # = y3

  exec.u256_mod_mul # = y3

  push.0.0.0.0
  loc_loadw.9
  push.0.0.0.0
  loc_loadw.8     # = x3

  exec.u256_mod_add # = y3

  loc_storew.10
  dropw       
  loc_storew.11
  dropw            # cache y3

  dup.3
  push.0.0.0.0
  movup.4
  mem_loadw
  dup.6
  push.0.0.0.0
  movup.4
  mem_loadw         # y -coordinate on stack top

  dup.9
  push.0.0.0.0
  movup.4
  mem_loadw
  dup.12
  push.0.0.0.0
  movup.4
  mem_loadw         # x -coordinate on stack top

  exec.u256_mod_mul # = t1

  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0     # = t0

  exec.u256_mod_mul # = x3

  dupw.1
  dupw.1            # repeated x3

  exec.u256_mod_add # = x3

  loc_storew.8
  dropw       
  loc_storew.9
  dropw             # cache x3

  dropw
  drop
  drop

  dup
  push.0.0.0.0
  loc_loadw.8
  movup.4
  mem_storew
  dropw              # write x3[0..4] to memory

  dup.1
  push.0.0.0.0
  loc_loadw.9
  movup.4
  mem_storew
  dropw              # write x3[4..8] to memory

  dup.2
  push.0.0.0.0
  loc_loadw.10
  movup.4
  mem_storew
  dropw              # write y3[0..4] to memory

  dup.3
  push.0.0.0.0
  loc_loadw.11
  movup.4
  mem_storew
  dropw              # write y3[4..8] to memory

  dup.4
  push.0.0.0.0
  loc_loadw.2
  movup.4
  mem_storew
  dropw              # write z3[0..4] to memory

  dup.5
  push.0.0.0.0
  loc_loadw.3
  movup.4
  mem_storew
  dropw              # write z3[4..8] to memory
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

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # x2 on stack top

  dup.8
  dup.10

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # x1 on stack top

  exec.u256_mod_mul # = t0

  loc_storew.0
  dropw       
  loc_storew.1
  dropw        # cache t0

  dup.8
  dup.10

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # y2 on stack top

  dup.10
  dup.12

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # y1 on stack top

  exec.u256_mod_mul # = t1

  loc_storew.2
  dropw       
  loc_storew.3
  dropw        # cache t1

  dup.10
  dup.12

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # z2 on stack top

  dup.12
  dup.14

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # z1 on stack top

  exec.u256_mod_mul # = t2

  loc_storew.4
  dropw       
  loc_storew.5
  dropw        # cache t2

  dup.2
  dup.4

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # y1 on stack top

  dup.8
  dup.10

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # x1 on stack top

  exec.u256_mod_add # = t3

  loc_storew.6
  dropw       
  loc_storew.7
  dropw        # cache t3

  dup.8
  dup.10

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # y2 on stack top

  dup.15
  dup.15
  swap

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # x2 on stack top
  
  exec.u256_mod_add # = t4

  push.0.0.0.0
  loc_loadw.7
  push.0.0.0.0
  loc_loadw.6 # t3 loaded back

  exec.u256_mod_mul # = t3

  loc_storew.6
  dropw       
  loc_storew.7
  dropw        # cache t3

  push.0.0.0.0
  loc_loadw.3
  push.0.0.0.0
  loc_loadw.2 # t1 loaded back

  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0 # t0 loaded back

  exec.u256_mod_add # = t4

  push.0.0.0.0
  loc_loadw.7
  push.0.0.0.0
  loc_loadw.6 # t3 loaded back

  exec.u256_mod_sub # = t3

  loc_storew.6
  dropw       
  loc_storew.7
  dropw        # cache t3

  dup.2
  dup.4

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # y1 on stack top

  dup.12
  dup.14

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # z1 on stack top

  exec.u256_mod_add # = t4

  loc_storew.8
  dropw       
  loc_storew.9
  dropw        # cache t4

  dup.11
  dup.11

  dup.10
  dup.12

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # y2 on stack top

  movup.8
  movup.9

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # z2 on stack top

  exec.u256_mod_add # = x3

  push.0.0.0.0
  loc_loadw.9
  push.0.0.0.0
  loc_loadw.8 # t4 loaded back

  exec.u256_mod_mul # = t4

  loc_storew.8
  dropw       
  loc_storew.9
  dropw        # cache t4

  push.0.0.0.0
  loc_loadw.5
  push.0.0.0.0
  loc_loadw.4 # t2 loaded back

  push.0.0.0.0
  loc_loadw.3
  push.0.0.0.0
  loc_loadw.2 # t1 loaded back

  exec.u256_mod_add # = x3

  push.0.0.0.0
  loc_loadw.9
  push.0.0.0.0
  loc_loadw.8 # t4 loaded back

  exec.u256_mod_sub # = t4

  loc_storew.8
  dropw       
  loc_storew.9
  dropw        # cache t4

  dup.4
  dup.6

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # z1 on stack top

  dup.8
  dup.10

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # x1 on stack top

  exec.u256_mod_add # = x3

  loc_storew.10
  dropw       
  loc_storew.11
  dropw       # cache x3

  dup.10
  dup.12

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # z2 on stack top

  dup.15
  dup.15
  swap

  push.0.0.0.0
  movup.4
  mem_loadw
  movup.4
  push.0.0.0.0
  movup.4
  mem_loadw # x2 on stack top

  exec.u256_mod_add # = y3

  push.0.0.0.0
  loc_loadw.11
  push.0.0.0.0
  loc_loadw.10 # x3 loaded back

  exec.u256_mod_mul # = x3

  loc_storew.10
  dropw       
  loc_storew.11
  dropw       # cache x3

  push.0.0.0.0
  loc_loadw.5
  push.0.0.0.0
  loc_loadw.4 # t2 loaded back

  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0 # t0 loaded back

  exec.u256_mod_add # = y3

  push.0.0.0.0
  loc_loadw.11
  push.0.0.0.0
  loc_loadw.10 # x3 loaded back

  exec.u256_mod_sub # = y3

  loc_storew.12
  dropw       
  loc_storew.13
  dropw       # cache y3

  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0 # t0 loaded back

  dupw.1
  dupw.1

  exec.u256_mod_add # = x3

  loc_storew.10
  swapw
  loc_storew.11
  swapw # cache x3

  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0 # t0 loaded back

  exec.u256_mod_add # = t0

  loc_storew.0
  dropw       
  loc_storew.1
  dropw        # cache t0

  push.0.0.0.0
  push.0.0.21.20517 # b3 on stack top

  push.0.0.0.0
  loc_loadw.5
  push.0.0.0.0
  loc_loadw.4 # t2 loaded back

  exec.u256_mod_mul # = t2

  loc_storew.4
  swapw
  loc_storew.5
  swapw # cache t2

  push.0.0.0.0
  loc_loadw.3
  push.0.0.0.0
  loc_loadw.2 # t1 loaded back

  exec.u256_mod_add # = z3

  loc_storew.14
  dropw       
  loc_storew.15
  dropw       # cache z3

  push.0.0.0.0
  loc_loadw.5
  push.0.0.0.0
  loc_loadw.4 # t2 loaded back

  push.0.0.0.0
  loc_loadw.3
  push.0.0.0.0
  loc_loadw.2 # t1 loaded back

  exec.u256_mod_sub # = t1

  loc_storew.2
  dropw       
  loc_storew.3
  dropw        # cache t1

  push.0.0.0.0
  push.0.0.21.20517 # b3 on stack top

  push.0.0.0.0
  loc_loadw.13
  push.0.0.0.0
  loc_loadw.12 # y3 loaded back

  exec.u256_mod_mul # = y3

  loc_storew.12
  swapw
  loc_storew.13
  swapw # cache y3

  push.0.0.0.0
  loc_loadw.9
  push.0.0.0.0
  loc_loadw.8 # t4 loaded back

  exec.u256_mod_mul # = x3

  loc_storew.10
  dropw       
  loc_storew.11
  dropw       # cache x3

  push.0.0.0.0
  loc_loadw.3
  push.0.0.0.0
  loc_loadw.2 # t1 loaded back

  push.0.0.0.0
  loc_loadw.7
  push.0.0.0.0
  loc_loadw.6 # t3 loaded back

  exec.u256_mod_mul # = t2

  push.0.0.0.0
  loc_loadw.11
  push.0.0.0.0
  loc_loadw.10 # x3 loaded back

  exec.u256_mod_neg
  exec.u256_mod_add # = x3

  loc_storew.10
  dropw       
  loc_storew.11
  dropw       # cache x3

  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0 # t0 loaded back

  push.0.0.0.0
  loc_loadw.13
  push.0.0.0.0
  loc_loadw.12 # y3 loaded back

  exec.u256_mod_mul # = y3

  loc_storew.12
  dropw       
  loc_storew.13
  dropw       # cache y3

  push.0.0.0.0
  loc_loadw.15
  push.0.0.0.0
  loc_loadw.14 # z3 loaded back

  push.0.0.0.0
  loc_loadw.3
  push.0.0.0.0
  loc_loadw.2 # t1 loaded back

  exec.u256_mod_mul # = t1

  push.0.0.0.0
  loc_loadw.13
  push.0.0.0.0
  loc_loadw.12 # y3 loaded back

  exec.u256_mod_add # = y3

  loc_storew.12
  dropw       
  loc_storew.13
  dropw       # cache y3

  push.0.0.0.0
  loc_loadw.7
  push.0.0.0.0
  loc_loadw.6 # t3 loaded back

  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0 # t0 loaded back

  exec.u256_mod_mul # = t0

  loc_storew.0
  dropw       
  loc_storew.1
  dropw        # cache t0

  push.0.0.0.0
  loc_loadw.9
  push.0.0.0.0
  loc_loadw.8 # t4 loaded back

  push.0.0.0.0
  loc_loadw.15
  push.0.0.0.0
  loc_loadw.14 # z3 loaded back

  exec.u256_mod_mul # = z3

  push.0.0.0.0
  loc_loadw.1
  push.0.0.0.0
  loc_loadw.0 # t0 loaded back

  exec.u256_mod_add # = z3

  loc_storew.14
  dropw       
  loc_storew.15
  dropw       # cache z3

  dropw
  dropw
  dropw

  push.0.0.0.0
  loc_loadw.10
  dup.4
  mem_storew
  dropw              # write x3[0..4] to memory

  push.0.0.0.0
  loc_loadw.11
  dup.5
  mem_storew
  dropw              # write x3[4..8] to memory

  push.0.0.0.0
  loc_loadw.12
  dup.6
  mem_storew
  dropw              # write y3[0..4] to memory

  push.0.0.0.0
  loc_loadw.13
  dup.7
  mem_storew
  dropw              # write y3[4..8] to memory

  push.0.0.0.0
  loc_loadw.14
  dup.8
  mem_storew
  dropw              # write z3[0..4] to memory

  push.0.0.0.0
  loc_loadw.15
  dup.9
  mem_storew
  dropw              # write z3[4..8] to memory
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
  loc_storew.0
  dropw       
  push.0.0.0.0
  loc_storew.1
  dropw        # init & cache res_X

  push.0.0.1.977
  loc_storew.2
  dropw       
  push.0.0.0.0
  loc_storew.3
  dropw         # init & cache res_Y

  push.0.0.0.0
  loc_storew.4
  dropw       
  push.0.0.0.0
  loc_storew.5
  dropw         # init & cache res_Z

  loc_storew.18
  dropw       
  loc_storew.19
  dropw       

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
        push.0.0.0.0
        loc_loadw.18
        dup
        push.1
        u32checked_and
        movdn.4
        u32unchecked_shr.1
        loc_storew.18
        dropw       

        if.true
          loc_storew.12
          dropw       
          loc_storew.13
          dropw       
          loc_storew.14
          dropw       
          loc_storew.15
          dropw             
          loc_storew.16
          dropw       
          loc_storew.17
          dropw       

          locaddr.11
          locaddr.10
          locaddr.9
          locaddr.8
          locaddr.7
          locaddr.6

          locaddr.17
          locaddr.16
          locaddr.15
          locaddr.14
          locaddr.13
          locaddr.12

          locaddr.5
          locaddr.4
          locaddr.3
          locaddr.2
          locaddr.1
          locaddr.0

          exec.point_addition

          drop
          drop

          loc_loadw.6
          loc_storew.0
          loc_loadw.7
          loc_storew.1

          loc_loadw.8
          loc_storew.2
          loc_loadw.9
          loc_storew.3

          loc_loadw.10
          loc_storew.4
          loc_loadw.11
          loc_storew.5

          dropw
        else
          repeat.6
            dropw
          end
        end
      end

      push.0.0.0.0
      loc_loadw.18
      movdn.3
      loc_storew.18
      dropw       
    end

    push.0.0.0.0
    loc_loadw.19
    loc_storew.18
    dropw       
  end

  dup
  push.0.0.0.0
  loc_loadw.0
  movup.4
  mem_storew
  dropw              # write x[0..4] to memory

  dup.1
  push.0.0.0.0
  loc_loadw.1
  movup.4
  mem_storew
  dropw              # write x[4..8] to memory

  dup.2
  push.0.0.0.0
  loc_loadw.2
  movup.4
  mem_storew
  dropw              # write y[0..4] to memory

  dup.3
  push.0.0.0.0
  loc_loadw.3
  movup.4
  mem_storew
  dropw              # write y[4..8] to memory

  dup.4
  push.0.0.0.0
  loc_loadw.4
  movup.4
  mem_storew
  dropw              # write z[0..4] to memory

  dup.5
  push.0.0.0.0
  loc_loadw.5
  movup.4
  mem_storew
  dropw              # write z[4..8] to memory
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
    loc_storew.0
    dropw
    # b[5-8] at 0
    loc_storew.1
    # b[0-4] at 1
    push.0 dropw
    # b[0] at top of stack, followed by a[0-7]
    movdn.8
    loc_storew.2
    # a[0-4] at 2
    swapw
    loc_storew.3
    # a[5-8] at 3
    padw
    loc_storew.4
    loc_storew.5
    # p at 4 and 5

    # b[0]
    dropw
    swapw
    push.0.0.0.0
    loc_loadw.4
    movdnw.2
    movup.12

    exec.mulstep4

    movdn.9
    movdn.9
    swapw
    loc_storew.4
    dropw
    push.0.0.0.0
    loc_loadw.5
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
    loc_storew.5
    dropw

    # b[1]
    push.0.0.0.0
    loc_loadw.4
    push.0.0.0.0
    loc_loadw.5
    movup.7
    dropw
    push.0.0.0.0
    loc_loadw.3 push.0.0.0.0
    loc_loadw.2 # load the xs
    push.0.0.0.0
    loc_loadw.1
    movup.2
    movdn.3
    push.0 dropw # only need b[1]

    exec.mulstep4

    movdn.9
    movdn.9
    swapw
    movdn.3
    push.0.0.0.0
    loc_loadw.4
    push.0 dropw # only need p[0]
    movdn.3
    # save p[0-3] to memory, not needed any more
    loc_storew.4
    dropw

    push.0.0.0.0
    loc_loadw.5
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
    loc_storew.5
    dropw

    # b[2]
    push.0.0.0.0
    loc_loadw.4
    push.0.0.0.0
    loc_loadw.5
    movup.7
    movup.7
    dropw
    push.0.0.0.0
    loc_loadw.3 push.0.0.0.0
    loc_loadw.2 # load the xs
    push.0.0.0.0
    loc_loadw.1
    swap
    movdn.3
    push.0 dropw # only need b[1]

    exec.mulstep4

    movdn.9
    movdn.9
    swapw
    movdn.3
    movdn.3
    push.0.0.0.0
    loc_loadw.4
    drop drop
    movdn.3
    movdn.3
    loc_storew.4
    dropw

    push.0.0.0.0
    loc_loadw.5
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
    loc_storew.5
    dropw

    # b[3]
    push.0.0.0.0
    loc_loadw.4
    push.0.0.0.0
    loc_loadw.5

    movup.7 movup.7 movup.7
    dropw
    push.0.0.0.0
    loc_loadw.3 push.0.0.0.0
    loc_loadw.2

    push.0.0.0.0
    loc_loadw.1
    movdn.3
    push.0 dropw

    exec.mulstep4

    movdn.9
    movdn.9

    swapw
    movup.3
    push.0.0.0.0
    loc_loadw.4
    drop
    movup.3

    loc_storew.4
    dropw
    push.0.0.0.0
    loc_loadw.5
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
    push.0.0.0.0
    loc_loadw.3 push.0.0.0.0
    loc_loadw.2 # load the xs
    # OPTIM: don't need a[4-7], but can't use mulstep4 if we don't load

    push.0.0.0.0
    loc_loadw.0
    push.0 dropw # b[4]

    exec.mulstep4
    dropw drop drop # OPTIM: don't need a[4-7], but can't use mulstep4 if we don't load

    # b[5]
    push.0.0.0.0
    loc_loadw.3
    push.0.0.0.0
    loc_loadw.0
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
    push.0.0.0.0
    loc_loadw.3
    push.0.0.0.0
    loc_loadw.0
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
    push.0.0.0.0
    loc_loadw.3
    push.0.0.0.0
    loc_loadw.0

    movdn.3 push.0 dropw
    movup.4
    movup.5
    movdn.2
    push.0
    exec.mulstep
    drop
    movdn.3
    drop drop drop

    push.0.0.0.0
    loc_loadw.4
    swapw
end"),
// ----- std::math::u64 ---------------------------------------------------------------------------
("std::math::u64", "# ===== HELPER FUNCTIONS ==========================================================================

# Asserts that both values at the top of the stack are u64 values.
# The input values are assumed to be represented using 32 bit limbs, fails if they are not.
proc.u32assert4
    u32assert.2
    movup.3
    movup.3
    u32assert.2
    movup.3
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
    swap
    movup.3
    u32assert.2
    u32overflowing_add
    movup.3
    movup.3
    u32assert.2
    u32overflowing_add3
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
    movup.3
    movup.2
    u32assert.2
    u32overflowing_sub
    movup.3
    movup.3
    u32assert.2
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
    dup.3
    dup.2
    u32assert.2         # make sure lower limbs of operands are 32-bit
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
    u32assert.2         # make sure higher limbs of operands are 32-bit
    u32overflowing_madd
    movup.3
    movup.2
    u32overflowing_add
    add
    add
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
    movup.3
    movup.2
    u32assert.2
    u32overflowing_sub
    movdn.3
    drop
    u32assert.2
    u32overflowing_sub
    swap
    eq.0
    movup.2
    and
    or
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
    movup.2
    u32assert.2
    u32overflowing_sub
    movup.2
    movup.3
    u32assert.2
    u32overflowing_sub
    swap
    drop
    movup.2
    eq.0
    and
    or
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
    exec.checked_gt
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
    exec.checked_lt
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
    movup.2
    u32checked_eq
    swap
    movup.2
    u32checked_eq
    and
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
    exec.checked_eq
    not
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
    u32assert.2
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

    adv_push.2          # read the quotient from the advice tape and make sure it consists of
    u32assert.2         # 32-bit limbs

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

    adv_push.2          # read the remainder from the advice tape and make sure it consists of
    u32assert.2         # 32-bit limbs

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

    adv_push.2          # read the quotient from the advice tape and make sure it consists of
    u32assert.2         # 32-bit limbs

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

    adv_push.2          # read the remainder from the advice tape and make sure it consists of
    u32assert.2         # 32-bit limbs

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

    adv_push.2          # read the quotient from the advice tape and make sure it consists of
    u32assert.2         # 32-bit limbs

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

    adv_push.2          # read the remainder from the advice tape and make sure it consists of
    u32assert.2         # 32-bit limbs

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
# The shift value should be in the range [0, 64), otherwise it will result in an
# error.
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a << b mod 2^64.
# This takes 28 cycles.
export.unchecked_shl
    pow2
    u32split
    exec.wrapping_mul
end


# Performs right shift of one unsigned 64-bit integer using the pow2 operation.
# The input value to be shifted is assumed to be represented using 32 bit limbs.
# The shift value should be in the range [0, 64), otherwise it will result in an
# error.
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a >> b.
# This takes 44 cycles.
export.unchecked_shr
    pow2
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
# The shift value should be in the range [0, 64), otherwise it will result in an
# error.
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [d_hi, d_lo, c_hi, c_lo, ...], where (d,c) = a << b,
# which d contains the bits shifted out.
# This takes 35 cycles.
export.overflowing_shl
    pow2
    u32split
    exec.overflowing_mul
end

# Performs right shift of one unsigned 64-bit integer preserving the overflow and
# using the pow2 operation.
# The input value to be shifted is assumed to be represented using 32 bit limbs.
# The shift value should be in the range [0, 64), otherwise it will result in an
# error.
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [d_hi, d_lo, c_hi, c_lo, ...], where c = a >> b, d = a << (64 - b).
# This takes 94 cycles.
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
# The shift value should be in the range [0, 64), otherwise it will result in an
# error.
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a << b mod 2^64.
# This takes 35 cycles.
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
    pow2
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
# The shift value should be in the range [0, 64), otherwise it will result in an
# error.
# Stack transition looks as follows:
# [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a << b mod 2^64.
# This takes 40 cycles.
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
    pow2
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
# are removed in such a way that the top 16 elements of the stack remain unchanged. If the stack
# would otherwise contain more than 16 elements at the end of execution, then adding a call to this 
# function at the end will reduce the size of the public inputs that are shared with the verifier.
# Input: Stack with 16 or more elements.
# Output: Stack with only the original top 16 elements.
export.truncate_stack.4
    loc_storew.0
    dropw
    loc_storew.1
    dropw
    loc_storew.2
    dropw
    loc_storew.3
    dropw
    sdepth
    neq.16
    while.true
        dropw
        sdepth
        neq.16
    end
    loc_loadw.3
    swapw.3
    loc_loadw.2
    swapw.2
    loc_loadw.1
    swapw.1
    loc_loadw.0
end
"),
];
