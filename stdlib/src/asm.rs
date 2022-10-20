//! This module is automatically generated during build time and should not be modified manually.

/// An array of modules defined in Miden standard library.
///
/// Entries in the array are tuples containing module namespace and module source code.
#[rustfmt::skip]
pub const MODULES: [(&str, &str); 12] = [
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

# Given an elliptic curve point in projective coordinate system ( total 24 field elements 
# required for representing x, y, z coordinate values s.t. they are provided by 6 distinct 
# memory addresses ) and a 256 -bit scalar, in radix-2^32 representation ( such that it 
# takes 8 stack elements to represent whole scalar, where each limb is of 32 -bit width ), 
# this routine multiplies elliptic curve point by given scalar, producing another point 
# on secp256k1 curve, which will also be presented in projective coordinate system.
#
# Input:
#
# During invocation, this routine expects stack in following form
#
# [X_addr_0, X_addr_1, Y_addr_0, Y_addr_1, Z_addr_0, Z_addr_1, Sc0, Sc1, Sc2, Sc3, Sc4, Sc5, Sc6, Sc7, X'_addr_0, X'_addr_1, Y'_addr_0, Y'_addr_1, Z'_addr_0, Z'_addr_1, ...]
#
# X_addr_0, X_addr_1 -> Input secp256k1 point's X -coordinate to be placed, in Montgomery form, in given addresses
# Y_addr_0, Y_addr_1 -> Input secp256k1 point's Y -coordinate to be placed, in Montgomery form, in given addresses
# Z_addr_1, Z_addr_1 -> Input secp256k1 point's Z -coordinate to be placed, in Montgomery form, in given addresses
# Sc{0..8}           -> 256 -bit scalar in radix-2^32 form | Sc0 is least significant limb & Sc7 is most significant limb
# X'_addr_0, X'_addr_1 -> Resulting secp256k1 point's X -coordinate to be placed, in Montgomery form, in given addresses
# Y'_addr_0, Y'_addr_1 -> Resulting secp256k1 point's Y -coordinate to be placed, in Montgomery form, in given addresses
# Z'_addr_1, Z'_addr_1 -> Resulting secp256k1 point's Z -coordinate to be placed, in Montgomery form, in given addresses
#
# Output:
#
# At end of execution of this routine, stack should look like below
#
# [X_addr_0, X_addr_1, Y_addr_0, Y_addr_1, Z_addr_0, Z_addr_1, ...]
#
# X_addr_0, X_addr_1 -> Resulting secp256k1 point's X -coordinate written, in Montgomery form, in given addresses
# Y_addr_0, Y_addr_1 -> Resulting secp256k1 point's Y -coordinate written, in Montgomery form, in given addresses
# Z_addr_0, Z_addr_1 -> Resulting secp256k1 point's Z -coordinate written, in Montgomery form, in given addresses
#
# One interested in resulting point, should read from provided addresses on stack.
# 
# This routine implements double-and-add algorithm, while following 
# https://github.com/itzmeanjan/secp256k1/blob/d23ea7d/point.py#L174-L186 
export.point_mul.18
  # initialize `base`
  push.0.0.0.0

  movup.4
  mem_loadw
  loc_storew.0

  movup.4
  mem_loadw
  loc_storew.1

  movup.4
  mem_loadw
  loc_storew.2

  movup.4
  mem_loadw
  loc_storew.3

  movup.4
  mem_loadw
  loc_storew.4

  movup.4
  mem_loadw
  loc_storew.5

  dropw

  # initialize `res` ( with group identity )
  # See https://github.com/itzmeanjan/secp256k1/blob/d23ea7d/point.py#L40-L45
  push.0.0.0.0
  loc_storew.6
  loc_storew.7
  dropw

  push.0.0.1.977
  loc_storew.8
  dropw
  push.0.0.0.0
  loc_storew.9

  loc_storew.10
  loc_storew.11

  dropw

  repeat.8
    repeat.32
      dup
      push.1
      u32checked_and

      if.true
        # res = base + res
        locaddr.17
        locaddr.16
        locaddr.15
        locaddr.14
        locaddr.13
        locaddr.12

        # res
        locaddr.11
        locaddr.10
        locaddr.9
        locaddr.8
        locaddr.7
        locaddr.6

        # base
        locaddr.5
        locaddr.4
        locaddr.3
        locaddr.2
        locaddr.1
        locaddr.0

        exec.point_addition

        # write res back
        push.0.0.0.0

        movup.4
        mem_loadw
        loc_storew.6

        movup.4
        mem_loadw
        loc_storew.7

        movup.4
        mem_loadw
        loc_storew.8

        movup.4
        mem_loadw
        loc_storew.9

        movup.4
        mem_loadw
        loc_storew.10

        movup.4
        mem_loadw
        loc_storew.11

        dropw
      end

      # base = base + base
      locaddr.17
      locaddr.16
      locaddr.15
      locaddr.14
      locaddr.13
      locaddr.12

      # base
      locaddr.5
      locaddr.4
      locaddr.3
      locaddr.2
      locaddr.1
      locaddr.0

      exec.point_doubling

      # write base back
      push.0.0.0.0

      movup.4
      mem_loadw
      loc_storew.0

      movup.4
      mem_loadw
      loc_storew.1

      movup.4
      mem_loadw
      loc_storew.2

      movup.4
      mem_loadw
      loc_storew.3

      movup.4
      mem_loadw
      loc_storew.4

      movup.4
      mem_loadw
      loc_storew.5

      dropw

      u32unchecked_shr.1
    end
  
    drop
  end

  # write resulting point to provided output memory addresses
  push.0.0.0.0

  loc_loadw.6
  dup.4
  mem_storew

  loc_loadw.7
  dup.5
  mem_storew

  loc_loadw.8
  dup.6
  mem_storew

  loc_loadw.9
  dup.7
  mem_storew

  loc_loadw.10
  dup.8
  mem_storew

  loc_loadw.11
  dup.9
  mem_storew

  dropw
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
