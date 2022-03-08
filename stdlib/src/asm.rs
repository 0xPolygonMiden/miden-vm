//! This module is automatically generated during build time and should not be modified manually.

/// An array of modules defined in Miden standard library.
///
/// Entries in the array are tuples containing module namespace and module source code.
#[rustfmt::skip]
pub const MODULES: [(&str, &str); 3] = [
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
end"),
// ----- std::math::u64 ---------------------------------------------------------------------------
("std::math::u64", "# Performs addition of two unsigned 64 bit integers discarding the overflow. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a + b) % 2^64 #
export.add_unsafe
    swap
    movup.3
    u32add.unsafe
    movup.3
    movup.3
    u32addc
    drop
end

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
    u32madd
    drop
    movup.3
    movup.3
    u32madd
    drop
end

# ===== DIVISION ================================================================================ #

# Performs division of two unsigned 64 bit integers discarding the remainder. #
# The input values are assumed to be represented using 32 bit limbs, but this is not checked. #
# Stack transition looks as follows: #
# [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a // b #
export.div_unsafe
    adv.u64div          # inject the quotient and the remainder into the advice tape #
    
    push.adv.1          # read the values from the advice tape and make sure they are u32's #
    u32assert           # TODO: this can be optimized once we have u32assert2 instruction #
    push.adv.1
    u32assert
    push.adv.1
    u32assert
    push.adv.1
    u32assert

    dup.5               # multiply quotient by the divisor; this also consumes the divisor #
    dup.4
    u32mul.unsafe
    dup.6
    dup.6
    u32madd
    eq.0
    assert
    movup.7
    dup.5
    u32madd
    eq.0
    assert
    movup.6
    dup.5
    mul
    eq.0
    assert

    swap                # add remainder to the previous result; this also consumes the remainder #
    movup.3
    u32add.unsafe
    movup.3
    movup.3
    u32addc
    eq.0
    assert

    movup.4             # make sure the result we got is equal to the dividend #
    assert.eq
    movup.3
    assert.eq           # quotient remains on the stack #
end
"),
];
