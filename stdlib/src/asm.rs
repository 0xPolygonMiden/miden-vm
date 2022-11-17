//! This module is automatically generated during build time and should not be modified manually.

/// An array of modules defined in Miden standard library.
///
/// Entries in the array are tuples containing module namespace and module parsed+serialized.
#[rustfmt::skip]
pub const MODULES: [(&str, vm_assembly::ProcedureId, &str, &[u8]); 13] = [
("std::sys",vm_assembly::ProcedureId([108, 156, 72, 120, 241, 17, 15, 48, 166, 221, 38, 219, 215, 149, 220, 31, 0, 39, 175, 30, 100, 43, 252, 186]),"#! Removes elements deep in the stack until the depth of the stack is exactly 16. The elements
#! are removed in such a way that the top 16 elements of the stack remain unchanged. If the stack
#! would otherwise contain more than 16 elements at the end of execution, then adding a call to this 
#! function at the end will reduce the size of the public inputs that are shared with the verifier.
#! Input: Stack with 16 or more elements.
#! Output: Stack with only the original top 16 elements.
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
",&[1, 0, 14, 116, 114, 117, 110, 99, 97, 116, 101, 95, 115, 116, 97, 99, 107, 218, 1, 82, 101, 109, 111, 118, 101, 115, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 100, 101, 101, 112, 32, 105, 110, 32, 116, 104, 101, 32, 115, 116, 97, 99, 107, 32, 117, 110, 116, 105, 108, 32, 116, 104, 101, 32, 100, 101, 112, 116, 104, 32, 111, 102, 32, 116, 104, 101, 32, 115, 116, 97, 99, 107, 32, 105, 115, 32, 101, 120, 97, 99, 116, 108, 121, 32, 49, 54, 46, 32, 84, 104, 101, 32, 101, 108, 101, 109, 101, 110, 116, 115, 10, 97, 114, 101, 32, 114, 101, 109, 111, 118, 101, 100, 32, 105, 110, 32, 115, 117, 99, 104, 32, 97, 32, 119, 97, 121, 32, 116, 104, 97, 116, 32, 116, 104, 101, 32, 116, 111, 112, 32, 49, 54, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 111, 102, 32, 116, 104, 101, 32, 115, 116, 97, 99, 107, 32, 114, 101, 109, 97, 105, 110, 32, 117, 110, 99, 104, 97, 110, 103, 101, 100, 46, 32, 73, 102, 32, 116, 104, 101, 32, 115, 116, 97, 99, 107, 10, 119, 111, 117, 108, 100, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 32, 99, 111, 110, 116, 97, 105, 110, 32, 109, 111, 114, 101, 32, 116, 104, 97, 110, 32, 49, 54, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 97, 116, 32, 116, 104, 101, 32, 101, 110, 100, 32, 111, 102, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 44, 32, 116, 104, 101, 110, 32, 97, 100, 100, 105, 110, 103, 32, 97, 32, 99, 97, 108, 108, 32, 116, 111, 32, 116, 104, 105, 115, 10, 102, 117, 110, 99, 116, 105, 111, 110, 32, 97, 116, 32, 116, 104, 101, 32, 101, 110, 100, 32, 119, 105, 108, 108, 32, 114, 101, 100, 117, 99, 101, 32, 116, 104, 101, 32, 115, 105, 122, 101, 32, 111, 102, 32, 116, 104, 101, 32, 112, 117, 98, 108, 105, 99, 32, 105, 110, 112, 117, 116, 115, 32, 116, 104, 97, 116, 32, 97, 114, 101, 32, 115, 104, 97, 114, 101, 100, 32, 119, 105, 116, 104, 32, 116, 104, 101, 32, 118, 101, 114, 105, 102, 105, 101, 114, 46, 10, 73, 110, 112, 117, 116, 58, 32, 83, 116, 97, 99, 107, 32, 119, 105, 116, 104, 32, 49, 54, 32, 111, 114, 32, 109, 111, 114, 101, 32, 101, 108, 101, 109, 101, 110, 116, 115, 46, 10, 79, 117, 116, 112, 117, 116, 58, 32, 83, 116, 97, 99, 107, 32, 119, 105, 116, 104, 32, 111, 110, 108, 121, 32, 116, 104, 101, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 116, 111, 112, 32, 49, 54, 32, 101, 108, 101, 109, 101, 110, 116, 115, 46, 1, 4, 0, 18, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 200, 3, 0, 0, 0, 0, 0, 0, 0, 108, 187, 24, 16, 0, 0, 0, 0, 0, 0, 0, 255, 3, 0, 108, 187, 24, 16, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 147, 194, 2, 0, 0, 0, 0, 0, 0, 0, 146, 194, 1, 0, 0, 0, 0, 0, 0, 0, 145, 194, 0, 0, 0, 0, 0, 0, 0, 0]),
("std::crypto::hashes::sha256",vm_assembly::ProcedureId([133, 1, 193, 139, 249, 107, 201, 66, 165, 23, 211, 154, 221, 104, 154, 149, 124, 222, 95, 77, 74, 205, 105, 176]),"#! Given [x, ...] on stack top, this routine computes [y, ...]
#! such that y = σ_0(x), as defined in SHA specification
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L73-L79
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

#! Given [x, ...] on stack top, this routine computes [y, ...]
#! such that y = σ_1(x), as defined in SHA specification
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L81-L87
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

#! Given [x, ...] on stack top, this routine computes [y, ...]
#! such that y = Σ_0(x), as defined in SHA specification
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L57-L63
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

#! Given [x, ...] on stack top, this routine computes [y, ...]
#! such that y = Σ_1(x), as defined in SHA specification
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L65-L71
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

#! Given [x, y, z, ...] on stack top, this routine computes [o, ...]
#! such that o = ch(x, y, z), as defined in SHA specification
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L37-L45
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

#! Given [x, y, z, ...] on stack top, this routine computes [o, ...]
#! such that o = maj(x, y, z), as defined in SHA specification
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L47-L55
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

#! Given [a, b, c, d, ...] on stack top, this routine reverses order of first 
#! four elements on stack top such that final stack state looks like [d, c, b, a, ...]
proc.rev_element_order
    swap
    movup.2
    movup.3
end

#! Given [a, b, c, d, ...] on stack top, this routine computes next message schedule word
#! using following formula
#!
#! t0 = small_sigma_1(a) + b
#! t1 = small_sigma_0(c) + d
#! return t0 + t1
#!
#! If to be computed message schedule word has index i ∈ [16, 64), then 
#! a, b, c, d will have following indices in message schedule
#!
#! a = msg[i - 2]
#! b = msg[i - 7]
#! c = msg[i - 15]
#! d = msg[i - 16]
proc.compute_message_schedule_word
    exec.small_sigma_1
    movup.2
    exec.small_sigma_0

    u32overflowing_add3
    drop
    u32wrapping_add
end

#! Given eight working variables of SHA256 ( i.e. hash state ), a 32 -bit round constant & 
#! 32 -bit message word on stack top, this routine consumes constant & message word into 
#! hash state.
#!
#! Expected stack state looks like
#!
#! [a, b, c, d, e, f, g, h, CONST_i, WORD_i] | i ∈ [0, 64)
#!
#! After finishing execution, stack looks like
#!
#! [a', b', c', d', e', f', g', h']
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2_256.hpp#L165-L175
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

#! Given 32 -bytes hash state ( in terms of 8 SHA256 words ) and 64 -bytes input 
#! message ( in terms of 16 SHA256 words ) on stack top, this routine computes
#! whole message schedule of 64 message words and consumes them into hash state.
#!
#! Expected stack state:
#!
#! [state0, state1, state2, state3, state4, state5, state6, state7, msg0, msg1, msg2, msg3, msg4, msg5, msg6, msg7, msg8, msg9, msg10, msg11, msg12, msg13, msg14, msg15]
#!
#! Final stack state after completion of execution
#!
#! [state0', state1', state2', state3', state4', state5', state6', state7']
#!
#! Note, each SHA256 word is 32 -bit wide
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2.hpp#L89-L113
#! & https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2_256.hpp#L148-L187 ( loop body execution when i = 0 )
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

#! Given 32 -bytes hash state ( in terms of 8 SHA256 words ) and precomputed message 
#! schedule of padding bytes ( in terms of 64 message words ), this routine consumes
#! that into hash state, leaving final hash state, which is 32 -bytes SHA256 digest.
#!
#! Note, in SHA256 2-to-1 hashing, 64 -bytes are padded, which is processed as second message
#! block ( each SHA256 message block is 64 -bytes wide ). That message block is used for generating 
#! message schedule of 64 SHA256 words. That's exactly what can be precomputed & is consumed here 
#! ( in this routine ) into provided hash state.
#!
#! Expected stack state:
#!
#! [state0, state1, state2, state3, state4, state5, state6, state7, ...]
#!
#! Final stack state after completion of execution
#!
#! [state0', state1', state2', state3', state4', state5', state6', state7']
#!
#! Note, each SHA256 word is 32 -bit wide
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/8a2c006/include/sha2_256.hpp#L148-L187 ( loop 
#! body execution when i = 1 i.e. consuming padding bytes )
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

#! Given 64 -bytes input, this routine computes 32 -bytes SAH256 digest
#!
#! Expected stack state:
#!
#! [m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15] | m[0,16) = 32 -bit word
#!
#! Note, each SHA256 word is 32 -bit wide, so that's how input is expected.
#! If you've 64 -bytes, consider packing 4 consecutive bytes into single word, 
#! maintaining big endian byte order.
#!
#! Final stack state:
#!
#! [dig0, dig1, dig2, dig3, dig4, dig5, dig6, dig7]
#!
#! SHA256 digest is represented in terms of eight 32 -bit words ( big endian byte order ).
export.hash
    push.0x5be0cd19.0x1f83d9ab.0x9b05688c.0x510e527f
    push.0xa54ff53a.0x3c6ef372.0xbb67ae85.0x6a09e667

    exec.prepare_message_schedule_and_consume
    exec.consume_padding_message_schedule
end
",&[12, 0, 13, 115, 109, 97, 108, 108, 95, 115, 105, 103, 109, 97, 95, 48, 0, 0, 0, 0, 0, 9, 0, 110, 86, 7, 130, 110, 86, 18, 130, 78, 3, 73, 73, 13, 115, 109, 97, 108, 108, 95, 115, 105, 103, 109, 97, 95, 49, 0, 0, 0, 0, 0, 9, 0, 110, 86, 17, 130, 110, 86, 19, 130, 78, 10, 73, 73, 11, 99, 97, 112, 95, 115, 105, 103, 109, 97, 95, 48, 0, 0, 0, 0, 0, 9, 0, 110, 86, 2, 130, 110, 86, 13, 130, 86, 22, 73, 73, 11, 99, 97, 112, 95, 115, 105, 103, 109, 97, 95, 49, 0, 0, 0, 0, 0, 9, 0, 110, 86, 6, 130, 110, 86, 11, 130, 86, 25, 73, 73, 2, 99, 104, 0, 0, 0, 0, 0, 8, 0, 130, 111, 71, 130, 74, 149, 71, 73, 3, 109, 97, 106, 0, 0, 0, 0, 0, 11, 0, 111, 111, 71, 130, 113, 71, 149, 150, 71, 73, 73, 17, 114, 101, 118, 95, 101, 108, 101, 109, 101, 110, 116, 95, 111, 114, 100, 101, 114, 0, 0, 0, 0, 0, 3, 0, 130, 149, 150, 29, 99, 111, 109, 112, 117, 116, 101, 95, 109, 101, 115, 115, 97, 103, 101, 95, 115, 99, 104, 101, 100, 117, 108, 101, 95, 119, 111, 114, 100, 0, 0, 0, 0, 0, 6, 0, 211, 1, 0, 149, 211, 0, 0, 43, 107, 39, 20, 99, 111, 110, 115, 117, 109, 101, 95, 109, 101, 115, 115, 97, 103, 101, 95, 119, 111, 114, 100, 0, 0, 0, 0, 0, 25, 0, 116, 116, 116, 211, 4, 0, 156, 157, 43, 107, 115, 211, 3, 0, 156, 43, 107, 113, 113, 113, 211, 5, 0, 112, 211, 2, 0, 39, 152, 112, 39, 168, 39, 36, 112, 114, 101, 112, 97, 114, 101, 95, 109, 101, 115, 115, 97, 103, 101, 95, 115, 99, 104, 101, 100, 117, 108, 101, 95, 97, 110, 100, 95, 99, 111, 110, 115, 117, 109, 101, 0, 0, 0, 2, 0, 185, 2, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 125, 125, 121, 130, 114, 114, 166, 165, 211, 7, 0, 130, 122, 130, 115, 115, 166, 165, 211, 7, 0, 111, 124, 130, 117, 117, 166, 165, 211, 7, 0, 125, 112, 119, 119, 166, 165, 211, 7, 0, 145, 185, 1, 152, 47, 138, 66, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 145, 68, 55, 113, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 207, 251, 192, 181, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 165, 219, 181, 233, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 125, 125, 125, 114, 119, 119, 166, 165, 211, 7, 0, 130, 113, 120, 120, 166, 165, 211, 7, 0, 149, 112, 121, 121, 166, 165, 211, 7, 0, 116, 112, 123, 123, 166, 165, 211, 7, 0, 163, 185, 1, 91, 194, 86, 57, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 241, 17, 241, 89, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 164, 130, 63, 146, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 213, 94, 28, 171, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 116, 112, 121, 121, 166, 165, 211, 7, 0, 116, 112, 123, 123, 166, 165, 211, 7, 0, 116, 112, 125, 125, 166, 165, 211, 7, 0, 125, 125, 130, 118, 114, 211, 7, 0, 164, 185, 1, 152, 170, 7, 216, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 1, 91, 131, 18, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 190, 133, 49, 36, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 195, 125, 12, 85, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 164, 164, 124, 120, 117, 117, 166, 165, 211, 7, 0, 124, 120, 119, 119, 166, 165, 211, 7, 0, 124, 112, 121, 121, 166, 165, 211, 7, 0, 124, 112, 118, 123, 166, 165, 211, 7, 0, 163, 185, 1, 116, 93, 190, 114, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 254, 177, 222, 128, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 167, 6, 220, 155, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 116, 241, 155, 193, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 164, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 112, 123, 123, 166, 166, 211, 7, 0, 120, 112, 118, 124, 166, 165, 211, 7, 0, 164, 211, 6, 0, 185, 1, 193, 105, 155, 228, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 134, 71, 190, 239, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 198, 157, 193, 15, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 204, 161, 12, 36, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 164, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 112, 123, 123, 166, 166, 211, 7, 0, 120, 112, 118, 124, 166, 165, 211, 7, 0, 164, 211, 6, 0, 185, 1, 111, 44, 233, 45, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 170, 132, 116, 74, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 220, 169, 176, 92, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 218, 136, 249, 118, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 164, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 112, 123, 123, 166, 166, 211, 7, 0, 120, 112, 123, 119, 166, 166, 211, 7, 0, 164, 211, 6, 0, 185, 1, 82, 81, 62, 152, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 109, 198, 49, 168, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 200, 39, 3, 176, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 199, 127, 89, 191, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 164, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 112, 123, 123, 166, 166, 211, 7, 0, 120, 112, 118, 124, 166, 165, 211, 7, 0, 164, 211, 6, 0, 185, 1, 243, 11, 224, 198, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 71, 145, 167, 213, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 81, 99, 202, 6, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 103, 41, 41, 20, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 164, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 112, 123, 123, 166, 166, 211, 7, 0, 120, 112, 118, 124, 166, 165, 211, 7, 0, 164, 211, 6, 0, 185, 1, 133, 10, 183, 39, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 56, 33, 27, 46, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 252, 109, 44, 77, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 19, 13, 56, 83, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 164, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 112, 123, 123, 166, 166, 211, 7, 0, 120, 112, 118, 124, 166, 165, 211, 7, 0, 164, 211, 6, 0, 185, 1, 84, 115, 10, 101, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 187, 10, 106, 118, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 46, 201, 194, 129, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 133, 44, 114, 146, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 164, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 112, 123, 123, 166, 166, 211, 7, 0, 120, 112, 118, 124, 166, 165, 211, 7, 0, 164, 211, 6, 0, 185, 1, 161, 232, 191, 162, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 75, 102, 26, 168, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 112, 139, 75, 194, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 163, 81, 108, 199, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 164, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 116, 123, 123, 166, 166, 211, 7, 0, 124, 112, 123, 123, 166, 166, 211, 7, 0, 120, 112, 118, 124, 166, 165, 211, 7, 0, 164, 211, 6, 0, 185, 1, 25, 232, 146, 209, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 36, 6, 153, 214, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 133, 53, 14, 244, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 112, 160, 106, 16, 0, 0, 0, 0, 171, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 163, 164, 164, 211, 6, 0, 185, 1, 22, 193, 164, 25, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 185, 1, 8, 108, 55, 30, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 76, 119, 72, 39, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 181, 188, 176, 52, 0, 0, 0, 0, 171, 211, 8, 0, 163, 211, 6, 0, 179, 185, 1, 179, 12, 28, 57, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 74, 170, 216, 78, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 79, 202, 156, 91, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 243, 111, 46, 104, 0, 0, 0, 0, 171, 211, 8, 0, 163, 211, 6, 0, 179, 185, 1, 238, 130, 143, 116, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 111, 99, 165, 120, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 20, 120, 200, 132, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 8, 2, 199, 140, 0, 0, 0, 0, 171, 211, 8, 0, 163, 211, 6, 0, 179, 185, 1, 250, 255, 190, 144, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 235, 108, 80, 164, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 247, 163, 249, 190, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 242, 120, 113, 198, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 103, 230, 9, 106, 0, 0, 0, 0, 39, 130, 185, 1, 133, 174, 103, 187, 0, 0, 0, 0, 39, 130, 149, 185, 1, 114, 243, 110, 60, 0, 0, 0, 0, 39, 165, 150, 185, 1, 58, 245, 79, 165, 0, 0, 0, 0, 39, 166, 151, 185, 1, 127, 82, 14, 81, 0, 0, 0, 0, 39, 167, 152, 185, 1, 140, 104, 5, 155, 0, 0, 0, 0, 39, 168, 153, 185, 1, 171, 217, 131, 31, 0, 0, 0, 0, 39, 169, 154, 185, 1, 25, 205, 224, 91, 0, 0, 0, 0, 39, 170, 32, 99, 111, 110, 115, 117, 109, 101, 95, 112, 97, 100, 100, 105, 110, 103, 95, 109, 101, 115, 115, 97, 103, 101, 95, 115, 99, 104, 101, 100, 117, 108, 101, 0, 0, 0, 0, 0, 96, 1, 127, 127, 185, 1, 0, 0, 0, 128, 0, 0, 0, 0, 171, 185, 1, 152, 47, 138, 66, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 145, 68, 55, 113, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 207, 251, 192, 181, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 165, 219, 181, 233, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 91, 194, 86, 57, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 241, 17, 241, 89, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 164, 130, 63, 146, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 213, 94, 28, 171, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 152, 170, 7, 216, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 1, 91, 131, 18, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 190, 133, 49, 36, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 195, 125, 12, 85, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 116, 93, 190, 114, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 254, 177, 222, 128, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 171, 185, 1, 167, 6, 220, 155, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 2, 0, 0, 0, 0, 0, 0, 171, 185, 1, 116, 241, 155, 193, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 0, 128, 0, 0, 0, 0, 171, 185, 1, 193, 105, 155, 228, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 64, 1, 0, 0, 0, 0, 171, 185, 1, 134, 71, 190, 239, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 80, 32, 0, 0, 0, 0, 0, 171, 185, 1, 198, 157, 193, 15, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 136, 80, 0, 0, 0, 0, 0, 0, 171, 185, 1, 204, 161, 12, 36, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 8, 0, 34, 0, 0, 0, 0, 171, 185, 1, 111, 44, 233, 45, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 20, 0, 85, 34, 0, 0, 0, 0, 171, 185, 1, 170, 132, 116, 74, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 66, 151, 8, 5, 0, 0, 0, 0, 171, 185, 1, 220, 169, 176, 92, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 32, 0, 0, 160, 0, 0, 0, 0, 171, 185, 1, 218, 136, 249, 118, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 0, 136, 90, 0, 0, 0, 0, 171, 185, 1, 82, 81, 62, 152, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 148, 92, 0, 0, 0, 0, 0, 171, 185, 1, 109, 198, 49, 168, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 157, 212, 22, 0, 0, 0, 0, 0, 171, 185, 1, 200, 39, 3, 176, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 0, 31, 128, 250, 0, 0, 0, 0, 171, 185, 1, 199, 127, 89, 191, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 208, 37, 50, 211, 0, 0, 0, 0, 171, 185, 1, 243, 11, 224, 198, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 89, 89, 103, 17, 0, 0, 0, 0, 171, 185, 1, 71, 145, 167, 213, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 218, 191, 230, 246, 0, 0, 0, 0, 171, 185, 1, 81, 99, 202, 6, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 73, 21, 12, 179, 0, 0, 0, 0, 171, 185, 1, 103, 41, 41, 20, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 80, 176, 178, 8, 0, 0, 0, 0, 171, 185, 1, 133, 10, 183, 39, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 39, 76, 124, 157, 0, 0, 0, 0, 171, 185, 1, 56, 33, 27, 46, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 147, 163, 226, 12, 0, 0, 0, 0, 171, 185, 1, 252, 109, 44, 77, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 234, 225, 230, 136, 0, 0, 0, 0, 171, 185, 1, 19, 13, 56, 83, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 53, 67, 43, 165, 0, 0, 0, 0, 171, 185, 1, 84, 115, 10, 101, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 73, 111, 161, 103, 0, 0, 0, 0, 171, 185, 1, 187, 10, 106, 118, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 111, 1, 50, 215, 0, 0, 0, 0, 171, 185, 1, 46, 201, 194, 129, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 145, 46, 235, 78, 0, 0, 0, 0, 171, 185, 1, 133, 44, 114, 146, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 229, 85, 191, 93, 0, 0, 0, 0, 171, 185, 1, 161, 232, 191, 162, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 53, 35, 238, 142, 0, 0, 0, 0, 171, 185, 1, 75, 102, 26, 168, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 194, 94, 188, 226, 0, 0, 0, 0, 171, 185, 1, 112, 139, 75, 194, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 148, 67, 63, 168, 0, 0, 0, 0, 171, 185, 1, 163, 81, 108, 199, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 247, 120, 173, 69, 0, 0, 0, 0, 171, 185, 1, 25, 232, 146, 209, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 205, 208, 243, 54, 0, 0, 0, 0, 171, 185, 1, 36, 6, 153, 214, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 232, 5, 156, 217, 0, 0, 0, 0, 171, 185, 1, 133, 53, 14, 244, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 199, 29, 81, 176, 0, 0, 0, 0, 171, 185, 1, 112, 160, 106, 16, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 196, 122, 188, 105, 0, 0, 0, 0, 171, 185, 1, 22, 193, 164, 25, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 91, 55, 17, 189, 0, 0, 0, 0, 171, 185, 1, 8, 108, 55, 30, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 229, 113, 186, 227, 0, 0, 0, 0, 171, 185, 1, 76, 119, 72, 39, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 242, 159, 32, 59, 0, 0, 0, 0, 171, 185, 1, 181, 188, 176, 52, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 23, 238, 254, 24, 0, 0, 0, 0, 171, 185, 1, 179, 12, 28, 57, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 231, 217, 90, 226, 0, 0, 0, 0, 171, 185, 1, 74, 170, 216, 78, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 70, 80, 55, 19, 0, 0, 0, 0, 171, 185, 1, 79, 202, 156, 91, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 157, 8, 21, 5, 0, 0, 0, 0, 171, 185, 1, 243, 111, 46, 104, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 4, 15, 13, 79, 0, 0, 0, 0, 171, 185, 1, 238, 130, 143, 116, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 78, 72, 39, 38, 0, 0, 0, 0, 171, 185, 1, 111, 99, 165, 120, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 210, 40, 1, 49, 0, 0, 0, 0, 171, 185, 1, 20, 120, 200, 132, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 52, 180, 104, 198, 0, 0, 0, 0, 171, 185, 1, 8, 2, 199, 140, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 204, 65, 8, 66, 0, 0, 0, 0, 171, 185, 1, 250, 255, 190, 144, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 184, 17, 211, 98, 0, 0, 0, 0, 171, 185, 1, 235, 108, 80, 164, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 113, 167, 155, 229, 0, 0, 0, 0, 171, 185, 1, 247, 163, 249, 190, 0, 0, 0, 0, 171, 211, 8, 0, 185, 1, 132, 164, 167, 133, 0, 0, 0, 0, 171, 185, 1, 242, 120, 113, 198, 0, 0, 0, 0, 171, 211, 8, 0, 155, 39, 130, 155, 39, 130, 149, 155, 39, 165, 150, 155, 39, 166, 151, 155, 39, 167, 152, 155, 39, 168, 153, 155, 39, 169, 154, 155, 39, 170, 4, 104, 97, 115, 104, 14, 2, 71, 105, 118, 101, 110, 32, 54, 52, 32, 45, 98, 121, 116, 101, 115, 32, 105, 110, 112, 117, 116, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 51, 50, 32, 45, 98, 121, 116, 101, 115, 32, 83, 65, 72, 50, 53, 54, 32, 100, 105, 103, 101, 115, 116, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 58, 10, 91, 109, 48, 44, 32, 109, 49, 44, 32, 109, 50, 44, 32, 109, 51, 44, 32, 109, 52, 44, 32, 109, 53, 44, 32, 109, 54, 44, 32, 109, 55, 44, 32, 109, 56, 44, 32, 109, 57, 44, 32, 109, 49, 48, 44, 32, 109, 49, 49, 44, 32, 109, 49, 50, 44, 32, 109, 49, 51, 44, 32, 109, 49, 52, 44, 32, 109, 49, 53, 93, 32, 124, 32, 109, 91, 48, 44, 49, 54, 41, 32, 61, 32, 51, 50, 32, 45, 98, 105, 116, 32, 119, 111, 114, 100, 10, 78, 111, 116, 101, 44, 32, 101, 97, 99, 104, 32, 83, 72, 65, 50, 53, 54, 32, 119, 111, 114, 100, 32, 105, 115, 32, 51, 50, 32, 45, 98, 105, 116, 32, 119, 105, 100, 101, 44, 32, 115, 111, 32, 116, 104, 97, 116, 39, 115, 32, 104, 111, 119, 32, 105, 110, 112, 117, 116, 32, 105, 115, 32, 101, 120, 112, 101, 99, 116, 101, 100, 46, 10, 73, 102, 32, 121, 111, 117, 39, 118, 101, 32, 54, 52, 32, 45, 98, 121, 116, 101, 115, 44, 32, 99, 111, 110, 115, 105, 100, 101, 114, 32, 112, 97, 99, 107, 105, 110, 103, 32, 52, 32, 99, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 98, 121, 116, 101, 115, 32, 105, 110, 116, 111, 32, 115, 105, 110, 103, 108, 101, 32, 119, 111, 114, 100, 44, 10, 109, 97, 105, 110, 116, 97, 105, 110, 105, 110, 103, 32, 98, 105, 103, 32, 101, 110, 100, 105, 97, 110, 32, 98, 121, 116, 101, 32, 111, 114, 100, 101, 114, 46, 10, 70, 105, 110, 97, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 58, 10, 91, 100, 105, 103, 48, 44, 32, 100, 105, 103, 49, 44, 32, 100, 105, 103, 50, 44, 32, 100, 105, 103, 51, 44, 32, 100, 105, 103, 52, 44, 32, 100, 105, 103, 53, 44, 32, 100, 105, 103, 54, 44, 32, 100, 105, 103, 55, 93, 10, 83, 72, 65, 50, 53, 54, 32, 100, 105, 103, 101, 115, 116, 32, 105, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 116, 101, 114, 109, 115, 32, 111, 102, 32, 101, 105, 103, 104, 116, 32, 51, 50, 32, 45, 98, 105, 116, 32, 119, 111, 114, 100, 115, 32, 40, 32, 98, 105, 103, 32, 101, 110, 100, 105, 97, 110, 32, 98, 121, 116, 101, 32, 111, 114, 100, 101, 114, 32, 41, 46, 1, 0, 0, 4, 0, 185, 4, 25, 205, 224, 91, 0, 0, 0, 0, 171, 217, 131, 31, 0, 0, 0, 0, 140, 104, 5, 155, 0, 0, 0, 0, 127, 82, 14, 81, 0, 0, 0, 0, 185, 4, 58, 245, 79, 165, 0, 0, 0, 0, 114, 243, 110, 60, 0, 0, 0, 0, 133, 174, 103, 187, 0, 0, 0, 0, 103, 230, 9, 106, 0, 0, 0, 0, 211, 9, 0, 211, 10, 0]),
("std::crypto::hashes::keccak256",vm_assembly::ProcedureId([202, 157, 243, 12, 53, 127, 247, 159, 253, 64, 111, 145, 36, 163, 43, 222, 73, 148, 214, 86, 121, 230, 153, 70]),"#! Keccak-p[1600, 24] permutation's θ step mapping function, which is implemented 
#! in terms of 32 -bit word size ( bit interleaved representation )
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L55-L98 for original implementation
#!
#! Expected stack state :
#!
#! [state_addr, ...]
#!
#! Final stack state :
#!
#! [ ... ]
#!
#! Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
#! s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#!
#! Consecutive memory addresses can be computed by repeated application of `add.1`.
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

#! Keccak-p[1600, 24] permutation's ρ step mapping function, which is implemented 
#! in terms of 32 -bit word size ( bit interleaved representation )
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L115-L147 for original implementation
#!
#! Expected stack state :
#!
#! [state_addr, ...]
#!
#! Final stack state :
#!
#! [ ... ]
#!
#! Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
#! s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#!
#! Consecutive memory addresses can be computed by repeated application of `add.1`.
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

#! Keccak-p[1600, 24] permutation's π step mapping function, which is implemented 
#! in terms of 32 -bit word size ( bit interleaved representation )
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L169-L207 for original implementation
#!
#! Expected stack state :
#!
#! [state_addr, ...]
#!
#! Final stack state :
#!
#! [ ... ]
#!
#! Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
#! s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#!
#! Consecutive memory addresses can be computed by repeated application of `add.1`.
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

#! Keccak-p[1600, 24] permutation's χ step mapping function, which is implemented 
#! in terms of 32 -bit word size ( bit interleaved representation )
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L233-L271 for original implementation
#!
#! Expected stack state :
#!
#! [state_addr, ...]
#!
#! Final stack state :
#!
#! [ ... ]
#!
#! Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
#! s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#!
#! Consecutive memory addresses can be computed by repeated application of `add.1`.
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

#! Keccak-p[1600, 24] permutation's ι ( iota ) function, which is
#! implemented in terms of 32 -bit word size ( bit interleaved form ); 
#! imagine https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
#! invoked with (c0, c1) as template arguments
#!
#! Expected stack state :
#!
#! [state_addr, c0, c1, ...]
#!
#! Final stack state :
#!
#! [ ... ]
#!
#! All this routine does is
#!
#! state[0] ^= c0
#! state[1] ^= c1
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

#! Keccak-p[1600, 24] permutation round, without `iota` function ( all other 
#! functions i.e. `theta`, `rho`, `pi`, `chi` are applied in order )
#!
#! As `iota` function involves xoring constant factors with first lane of state array 
#! ( read state[0, 0] ), it's required to invoke them seperately after completion of
#! this procedure's execution.
#!
#! Expected stack state :
#!
#! [start_addr, ... ]
#!
#! After finishing execution, stack looks like
#!
#! [ ... ]
#!
#! Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
#! s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#!
#! Consecutive memory addresses can be computed by repeated application of `add.1`.
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L325-L340
proc.round
    dup
    exec.theta

    dup
    exec.rho

    dup
    exec.pi

    exec.chi
end

#! Keccak-p[1600, 24] permutation, applying 24 rounds on state array of size  5 x 5 x 64, 
#! where each 64 -bit lane is represented in bit interleaved form ( in terms of two 32 -bit words ).
#!
#! Expected stack state :
#!
#! [start_addr, ... ]
#!
#! After finishing execution, stack looks like
#!
#! [ ... ]
#!
#! Whole keccak-p[1600, 24] state can be represented using fifty u32 elements i.e. 13 absolute memory addresses
#! s.t. last two elements of 12 -th ( when indexed from zero ) memory address are zeroed.
#!
#! Consecutive memory addresses can be computed by repeated application of `add.1`.
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L379-L427
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

#! Given two 32 -bit unsigned integers ( standard form ), representing upper and lower
#! bits of a 64 -bit unsigned integer ( actually a keccak-[1600, 24] lane ),
#! this function converts them into bit interleaved representation, where two 32 -bit
#! unsigned integers ( even portion & then odd portion ) hold bits in even and odd
#! indices of 64 -bit unsigned integer ( remember it's represented in terms of
#! two 32 -bit elements )
#!
#! Input stack state :
#!
#! [hi, lo, ...]
#!
#! After application of bit interleaving, stack looks like
#!
#! [even, odd, ...]
#!
#! Read more about bit interleaved representation in section 2.1 of https://keccak.team/files/Keccak-implementation-3.2.pdf
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/utils.hpp#L123-L149
#! for reference implementation in higher level language.
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

#! Given two 32 -bit unsigned integers ( in bit interleaved form ), representing even and odd
#! positioned bits of a 64 -bit unsigned integer ( actually a keccak-[1600, 24] lane ),
#! this function converts them into standard representation, where two 32 -bit
#! unsigned integers hold higher ( 32 -bit ) and lower ( 32 -bit ) bits of standard
#! representation of 64 -bit unsigned integer
#!
#! Input stack state :
#!
#! [even, odd, ...]
#!
#! After application of logic, stack looks like
#!
#! [hi, lo, ...]
#!
#! This function reverts the action done by `to_bit_interleaved` function implemented above.
#!
#! Read more about bit interleaved representation in section 2.1 of https://keccak.team/files/Keccak-implementation-3.2.pdf
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/utils.hpp#L151-L175
#! for reference implementation in higher level language.
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

#! Given 64 -bytes input ( in terms of sixteen u32 elements on stack top ) to 2-to-1
#! keccak256 hash function, this function prepares 5 x 5 x 64 keccak-p[1600, 24] state
#! bit array such that each of twenty five 64 -bit wide lane is represented in bit
#! interleaved form, using two 32 -bit integers. After completion of execution of
#! this function, state array should live in allocated memory ( total fifty u32 elements, stored in
#! 13 consecutive memory addresses s.t. starting absolute address is provided ).
#!
#! Input stack state :
#!
#! [state_addr, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, ...]
#!
#! Note, state_addr is the starting absolute memory address where keccak-p[1600, 24] state
#! is kept. Consecutive addresses can be computed by repeated application of `add.1` instruction.
#!
#! Final stack state :
#!
#! [...]
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/keccak_256.hpp#L73-L153
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

#! Given 32 -bytes digest ( in terms of eight u32 elements on stack top ) in bit interleaved form,
#! this function attempts to convert those into standard representation, where eight u32 elements
#! live on stack top, each pair of them hold higher and lower bits of 64 -bit unsigned
#! integer ( lane of keccak-p[1600, 24] state array )
#!
#! Input stack state :
#!
#! [lane0_even, lane0_odd, lane1_even, lane1_odd, lane2_even, lane2_odd, lane3_even, lane3_odd, ...]
#!
#! Output stack state :
#!
#! [dig0_hi, dig0_lo, dig1_hi, dig1_lo, dig2_hi, dig2_lo, dig3_hi, dig3_lo, ...]
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/keccak_256.hpp#L180-L209
proc.to_digest
    repeat.4
        movup.7
        movup.7

        exec.from_bit_interleaved
    end
end

#! Given 64 -bytes input, in terms of sixteen 32 -bit unsigned integers, where each pair
#! of them holding higher & lower 32 -bits of 64 -bit unsigned integer ( reinterpreted on
#! host CPU from little endian byte array ) respectively, this function computes 32 -bytes
#! keccak256 digest, held on stack top, represented in terms of eight 32 -bit unsigned integers,
#! where each pair of them keeps higher and lower 32 -bits of 64 -bit unsigned integer respectively
#!
#! Expected stack state :
#!
#! [iword0, iword1, iword2, iword3, iword4, iword5, iword6, iword7, 
#!  iword8, iword9, iword10, iword11, iword12, iword13, iword14, iword15, ... ]
#!
#! Final stack state :
#!
#! [oword0, oword1, oword2, oword3, oword4, oword5, oword6, oword7, ... ]
#!
#! See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/keccak_256.hpp#L232-L257
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
end",&[12, 0, 5, 116, 104, 101, 116, 97, 0, 0, 0, 3, 0, 175, 2, 110, 186, 0, 0, 0, 0, 0, 0, 0, 0, 195, 107, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 150, 73, 130, 150, 73, 130, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 150, 73, 130, 150, 73, 130, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 150, 73, 130, 150, 73, 130, 149, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 149, 73, 130, 149, 73, 130, 186, 0, 0, 0, 0, 0, 0, 0, 0, 189, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 150, 73, 130, 150, 73, 130, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 150, 73, 130, 150, 73, 130, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 150, 73, 130, 150, 73, 130, 149, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 149, 73, 130, 149, 73, 130, 150, 150, 186, 1, 0, 0, 0, 0, 0, 0, 0, 198, 108, 186, 0, 0, 0, 0, 0, 0, 0, 0, 189, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 150, 73, 130, 150, 73, 130, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 150, 73, 130, 150, 73, 130, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 150, 73, 130, 150, 73, 130, 149, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 149, 73, 130, 149, 73, 130, 186, 0, 0, 0, 0, 0, 0, 0, 0, 189, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 150, 73, 130, 150, 73, 130, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 150, 73, 130, 150, 73, 130, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 150, 73, 130, 150, 73, 130, 149, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 149, 73, 130, 149, 73, 130, 150, 150, 186, 2, 0, 0, 0, 0, 0, 0, 0, 198, 108, 186, 0, 0, 0, 0, 0, 0, 0, 0, 189, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 150, 73, 130, 150, 73, 130, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 150, 73, 130, 150, 73, 130, 149, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 150, 73, 130, 150, 73, 130, 149, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 149, 73, 130, 149, 73, 130, 186, 2, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 186, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 118, 114, 90, 1, 73, 120, 114, 73, 112, 118, 90, 1, 73, 114, 118, 73, 153, 121, 90, 1, 73, 154, 120, 73, 155, 160, 90, 1, 73, 156, 159, 73, 157, 157, 90, 1, 73, 157, 157, 73, 130, 149, 150, 151, 152, 153, 154, 155, 156, 186, 0, 0, 0, 0, 0, 0, 0, 0, 189, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 115, 73, 130, 116, 73, 130, 149, 117, 73, 165, 150, 118, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 119, 73, 130, 120, 73, 130, 149, 121, 73, 165, 150, 122, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 123, 73, 130, 124, 73, 130, 149, 115, 73, 165, 150, 116, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 117, 73, 130, 118, 73, 130, 149, 119, 73, 165, 150, 120, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 121, 73, 130, 122, 73, 130, 149, 123, 73, 165, 150, 124, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 115, 73, 130, 116, 73, 130, 149, 117, 73, 165, 150, 118, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 119, 73, 130, 120, 73, 130, 149, 121, 73, 165, 150, 122, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 123, 73, 130, 124, 73, 130, 149, 115, 73, 165, 150, 116, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 117, 73, 130, 118, 73, 130, 149, 119, 73, 165, 150, 120, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 121, 73, 130, 122, 73, 130, 149, 123, 73, 165, 150, 124, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 152, 73, 130, 152, 73, 130, 149, 152, 73, 165, 150, 152, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 152, 73, 130, 152, 73, 130, 149, 152, 73, 165, 150, 152, 73, 166, 114, 198, 108, 3, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 152, 73, 130, 152, 73, 130, 151, 198, 108, 3, 114, 104, 111, 0, 0, 0, 1, 0, 203, 0, 110, 186, 0, 0, 0, 0, 0, 0, 0, 0, 195, 107, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 191, 150, 90, 1, 165, 151, 110, 3, 168, 198, 114, 191, 90, 31, 130, 90, 31, 130, 149, 90, 14, 165, 150, 90, 14, 166, 151, 110, 3, 168, 198, 114, 191, 90, 13, 130, 90, 14, 149, 90, 18, 165, 150, 90, 18, 166, 151, 110, 3, 168, 198, 114, 191, 90, 22, 130, 90, 22, 130, 149, 90, 3, 165, 150, 90, 3, 166, 151, 110, 3, 168, 198, 114, 191, 90, 27, 130, 90, 28, 149, 90, 10, 165, 150, 90, 10, 166, 151, 110, 3, 168, 198, 114, 191, 90, 1, 130, 90, 2, 149, 90, 5, 165, 150, 90, 5, 166, 151, 110, 3, 168, 198, 114, 191, 90, 21, 130, 90, 22, 149, 90, 12, 166, 149, 90, 13, 165, 151, 110, 3, 168, 198, 114, 191, 90, 19, 130, 90, 20, 149, 90, 20, 166, 149, 90, 21, 165, 151, 110, 3, 168, 198, 114, 191, 90, 22, 130, 90, 23, 149, 90, 7, 166, 149, 90, 8, 165, 151, 110, 3, 168, 198, 114, 191, 90, 10, 130, 90, 11, 149, 90, 4, 165, 150, 90, 4, 166, 151, 110, 3, 168, 198, 114, 191, 90, 9, 130, 90, 9, 130, 149, 90, 1, 165, 150, 90, 1, 166, 151, 110, 3, 168, 198, 114, 191, 90, 30, 130, 90, 31, 149, 90, 28, 165, 150, 90, 28, 166, 151, 110, 3, 168, 198, 114, 191, 90, 7, 130, 90, 7, 130, 151, 198, 108, 2, 112, 105, 0, 0, 0, 14, 0, 25, 1, 110, 186, 0, 0, 0, 0, 0, 0, 0, 0, 195, 107, 186, 1, 0, 0, 0, 0, 0, 0, 0, 130, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 191, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 166, 117, 198, 107, 107, 166, 166, 115, 3, 198, 151, 3, 167, 114, 191, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 166, 117, 3, 198, 107, 107, 115, 3, 198, 151, 3, 167, 114, 191, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 117, 3, 198, 149, 107, 149, 107, 166, 166, 115, 3, 198, 151, 3, 167, 114, 191, 115, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 117, 198, 117, 3, 191, 149, 107, 149, 107, 115, 3, 198, 151, 3, 167, 114, 191, 115, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 117, 3, 198, 108, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 166, 115, 3, 198, 151, 3, 167, 114, 191, 115, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 117, 3, 198, 117, 3, 191, 149, 107, 149, 107, 115, 3, 198, 151, 3, 167, 114, 191, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 166, 117, 3, 198, 107, 107, 166, 166, 115, 3, 198, 151, 3, 167, 114, 191, 115, 3, 198, 151, 3, 167, 114, 191, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 166, 117, 3, 198, 107, 107, 166, 166, 115, 3, 198, 151, 3, 167, 114, 191, 115, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 117, 3, 198, 117, 3, 191, 149, 107, 149, 107, 115, 3, 198, 151, 3, 167, 114, 191, 115, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 150, 150, 117, 3, 198, 108, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 166, 115, 3, 198, 151, 3, 167, 114, 191, 115, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 117, 3, 198, 117, 3, 191, 149, 107, 149, 107, 115, 3, 198, 151, 3, 167, 114, 191, 115, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 166, 166, 117, 3, 198, 107, 107, 151, 107, 186, 0, 0, 0, 0, 0, 0, 0, 0, 189, 167, 254, 148, 4, 10, 0, 115, 191, 114, 198, 151, 3, 167, 152, 3, 168, 108, 107, 107, 3, 99, 104, 105, 0, 0, 0, 4, 0, 82, 3, 110, 186, 0, 0, 0, 0, 0, 0, 0, 0, 195, 107, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 107, 107, 74, 130, 74, 130, 149, 3, 110, 166, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 111, 111, 153, 71, 130, 153, 71, 130, 150, 74, 150, 74, 151, 71, 130, 151, 71, 130, 150, 150, 186, 1, 0, 0, 0, 0, 0, 0, 0, 198, 114, 191, 107, 107, 74, 130, 74, 130, 149, 3, 110, 166, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 111, 111, 151, 71, 130, 151, 71, 130, 150, 150, 151, 5, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 152, 74, 152, 74, 112, 71, 130, 113, 71, 130, 154, 154, 186, 2, 0, 0, 0, 0, 0, 0, 0, 198, 108, 74, 130, 74, 130, 149, 71, 130, 149, 71, 130, 186, 0, 0, 0, 0, 0, 0, 0, 0, 189, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 191, 186, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 186, 2, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 152, 73, 130, 152, 73, 130, 114, 198, 151, 3, 167, 114, 191, 74, 130, 74, 130, 113, 113, 149, 71, 130, 149, 71, 130, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 186, 1, 0, 0, 0, 0, 0, 0, 0, 198, 153, 3, 110, 170, 191, 152, 152, 74, 130, 74, 130, 112, 71, 130, 113, 71, 130, 150, 150, 74, 130, 74, 130, 114, 71, 130, 115, 71, 130, 150, 150, 186, 2, 0, 0, 0, 0, 0, 0, 0, 198, 153, 5, 110, 170, 191, 107, 107, 111, 111, 151, 74, 152, 74, 130, 149, 71, 130, 149, 71, 130, 150, 150, 151, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 150, 150, 74, 130, 74, 130, 149, 71, 130, 149, 71, 130, 150, 150, 186, 3, 0, 0, 0, 0, 0, 0, 0, 198, 186, 0, 0, 0, 0, 0, 0, 0, 0, 189, 3, 110, 168, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 107, 107, 74, 130, 74, 130, 149, 3, 165, 112, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 111, 111, 153, 71, 130, 153, 71, 130, 150, 150, 74, 130, 74, 130, 114, 71, 130, 115, 71, 130, 150, 150, 200, 1, 0, 0, 0, 0, 0, 0, 0, 153, 3, 169, 116, 191, 149, 107, 149, 107, 111, 111, 152, 152, 74, 130, 74, 130, 149, 71, 130, 149, 71, 130, 151, 5, 167, 114, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 154, 154, 74, 130, 74, 130, 113, 113, 149, 71, 130, 149, 71, 130, 154, 154, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 74, 130, 74, 130, 149, 71, 130, 149, 71, 130, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 166, 200, 3, 0, 0, 0, 0, 0, 0, 0, 114, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 74, 130, 74, 130, 113, 113, 149, 71, 130, 149, 71, 130, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 1, 0, 0, 0, 0, 0, 0, 0, 153, 3, 169, 116, 191, 152, 152, 74, 130, 74, 130, 113, 113, 149, 71, 130, 149, 71, 130, 150, 150, 74, 130, 74, 130, 115, 115, 149, 71, 130, 149, 71, 130, 150, 150, 200, 2, 0, 0, 0, 0, 0, 0, 0, 153, 5, 169, 116, 191, 107, 107, 150, 150, 74, 130, 74, 130, 113, 113, 149, 71, 130, 149, 71, 130, 151, 3, 167, 114, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 107, 149, 107, 152, 152, 74, 130, 74, 130, 149, 71, 130, 149, 71, 130, 150, 150, 200, 3, 0, 0, 0, 0, 0, 0, 0, 151, 5, 167, 114, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 107, 107, 149, 3, 165, 112, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 152, 152, 74, 130, 74, 130, 113, 113, 149, 71, 130, 149, 71, 130, 150, 150, 74, 130, 74, 130, 115, 115, 149, 71, 130, 149, 71, 130, 150, 150, 200, 1, 0, 0, 0, 0, 0, 0, 0, 153, 3, 169, 116, 191, 149, 107, 149, 107, 150, 150, 74, 130, 74, 130, 113, 113, 149, 71, 130, 149, 71, 130, 151, 5, 167, 114, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 154, 154, 74, 130, 74, 130, 113, 113, 149, 71, 130, 149, 71, 130, 154, 154, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 74, 130, 74, 130, 149, 71, 130, 149, 71, 130, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 166, 200, 3, 0, 0, 0, 0, 0, 0, 0, 114, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 151, 3, 167, 114, 191, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 151, 73, 130, 151, 73, 130, 149, 151, 73, 165, 150, 151, 73, 166, 114, 198, 108, 107, 4, 105, 111, 116, 97, 0, 0, 0, 0, 0, 13, 0, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 152, 73, 130, 152, 73, 130, 151, 198, 108, 5, 114, 111, 117, 110, 100, 0, 0, 0, 0, 0, 7, 0, 110, 211, 0, 0, 110, 211, 1, 0, 110, 211, 2, 0, 211, 3, 0, 8, 107, 101, 99, 99, 97, 107, 95, 112, 0, 0, 0, 0, 0, 120, 0, 110, 211, 5, 0, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 139, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 128, 128, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 139, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 0, 128, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 136, 128, 0, 128, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 130, 0, 0, 128, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 130, 128, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 3, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 139, 128, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 11, 0, 0, 128, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 138, 0, 0, 128, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 129, 0, 0, 128, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 129, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 8, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 3, 128, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 136, 128, 0, 128, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 136, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 0, 128, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 211, 4, 0, 110, 211, 5, 0, 185, 2, 130, 128, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 149, 211, 4, 0, 18, 116, 111, 95, 98, 105, 116, 95, 105, 110, 116, 101, 114, 108, 101, 97, 118, 101, 100, 55, 3, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 51, 50, 32, 45, 98, 105, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 40, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 102, 111, 114, 109, 32, 41, 44, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 105, 110, 103, 32, 117, 112, 112, 101, 114, 32, 97, 110, 100, 32, 108, 111, 119, 101, 114, 10, 98, 105, 116, 115, 32, 111, 102, 32, 97, 32, 54, 52, 32, 45, 98, 105, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 40, 32, 97, 99, 116, 117, 97, 108, 108, 121, 32, 97, 32, 107, 101, 99, 99, 97, 107, 45, 91, 49, 54, 48, 48, 44, 32, 50, 52, 93, 32, 108, 97, 110, 101, 32, 41, 44, 10, 116, 104, 105, 115, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 99, 111, 110, 118, 101, 114, 116, 115, 32, 116, 104, 101, 109, 32, 105, 110, 116, 111, 32, 98, 105, 116, 32, 105, 110, 116, 101, 114, 108, 101, 97, 118, 101, 100, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 44, 32, 119, 104, 101, 114, 101, 32, 116, 119, 111, 32, 51, 50, 32, 45, 98, 105, 116, 10, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 40, 32, 101, 118, 101, 110, 32, 112, 111, 114, 116, 105, 111, 110, 32, 38, 32, 116, 104, 101, 110, 32, 111, 100, 100, 32, 112, 111, 114, 116, 105, 111, 110, 32, 41, 32, 104, 111, 108, 100, 32, 98, 105, 116, 115, 32, 105, 110, 32, 101, 118, 101, 110, 32, 97, 110, 100, 32, 111, 100, 100, 10, 105, 110, 100, 105, 99, 101, 115, 32, 111, 102, 32, 54, 52, 32, 45, 98, 105, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 40, 32, 114, 101, 109, 101, 109, 98, 101, 114, 32, 105, 116, 39, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 116, 101, 114, 109, 115, 32, 111, 102, 10, 116, 119, 111, 32, 51, 50, 32, 45, 98, 105, 116, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 41, 10, 73, 110, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 104, 105, 44, 32, 108, 111, 44, 32, 46, 46, 46, 93, 10, 65, 102, 116, 101, 114, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 98, 105, 116, 32, 105, 110, 116, 101, 114, 108, 101, 97, 118, 105, 110, 103, 44, 32, 115, 116, 97, 99, 107, 32, 108, 111, 111, 107, 115, 32, 108, 105, 107, 101, 10, 91, 101, 118, 101, 110, 44, 32, 111, 100, 100, 44, 32, 46, 46, 46, 93, 10, 82, 101, 97, 100, 32, 109, 111, 114, 101, 32, 97, 98, 111, 117, 116, 32, 98, 105, 116, 32, 105, 110, 116, 101, 114, 108, 101, 97, 118, 101, 100, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 115, 101, 99, 116, 105, 111, 110, 32, 50, 46, 49, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 107, 101, 99, 99, 97, 107, 46, 116, 101, 97, 109, 47, 102, 105, 108, 101, 115, 47, 75, 101, 99, 99, 97, 107, 45, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 45, 51, 46, 50, 46, 112, 100, 102, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 105, 116, 122, 109, 101, 97, 110, 106, 97, 110, 47, 109, 101, 114, 107, 108, 105, 122, 101, 45, 115, 104, 97, 47, 98, 108, 111, 98, 47, 49, 100, 51, 53, 97, 97, 101, 57, 100, 97, 55, 102, 101, 100, 50, 48, 49, 50, 55, 52, 56, 57, 102, 51, 54, 50, 98, 52, 98, 99, 57, 51, 50, 52, 50, 97, 53, 49, 54, 99, 47, 105, 110, 99, 108, 117, 100, 101, 47, 117, 116, 105, 108, 115, 46, 104, 112, 112, 35, 76, 49, 50, 51, 45, 76, 49, 52, 57, 10, 102, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 104, 105, 103, 104, 101, 114, 32, 108, 101, 118, 101, 108, 32, 108, 97, 110, 103, 117, 97, 103, 101, 46, 1, 0, 0, 6, 0, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 140, 8, 40, 0, 78, 1, 130, 78, 1, 130, 113, 113, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 71, 130, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 71, 130, 82, 31, 130, 82, 15, 130, 73, 73, 113, 113, 185, 1, 2, 0, 0, 0, 0, 0, 0, 0, 71, 130, 185, 1, 2, 0, 0, 0, 0, 0, 0, 0, 71, 130, 82, 30, 130, 82, 14, 130, 150, 73, 73, 130, 149, 78, 2, 165, 150, 78, 2, 166, 149, 107, 149, 107, 20, 102, 114, 111, 109, 95, 98, 105, 116, 95, 105, 110, 116, 101, 114, 108, 101, 97, 118, 101, 100, 90, 3, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 51, 50, 32, 45, 98, 105, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 40, 32, 105, 110, 32, 98, 105, 116, 32, 105, 110, 116, 101, 114, 108, 101, 97, 118, 101, 100, 32, 102, 111, 114, 109, 32, 41, 44, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 105, 110, 103, 32, 101, 118, 101, 110, 32, 97, 110, 100, 32, 111, 100, 100, 10, 112, 111, 115, 105, 116, 105, 111, 110, 101, 100, 32, 98, 105, 116, 115, 32, 111, 102, 32, 97, 32, 54, 52, 32, 45, 98, 105, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 40, 32, 97, 99, 116, 117, 97, 108, 108, 121, 32, 97, 32, 107, 101, 99, 99, 97, 107, 45, 91, 49, 54, 48, 48, 44, 32, 50, 52, 93, 32, 108, 97, 110, 101, 32, 41, 44, 10, 116, 104, 105, 115, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 99, 111, 110, 118, 101, 114, 116, 115, 32, 116, 104, 101, 109, 32, 105, 110, 116, 111, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 44, 32, 119, 104, 101, 114, 101, 32, 116, 119, 111, 32, 51, 50, 32, 45, 98, 105, 116, 10, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 104, 111, 108, 100, 32, 104, 105, 103, 104, 101, 114, 32, 40, 32, 51, 50, 32, 45, 98, 105, 116, 32, 41, 32, 97, 110, 100, 32, 108, 111, 119, 101, 114, 32, 40, 32, 51, 50, 32, 45, 98, 105, 116, 32, 41, 32, 98, 105, 116, 115, 32, 111, 102, 32, 115, 116, 97, 110, 100, 97, 114, 100, 10, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 111, 102, 32, 54, 52, 32, 45, 98, 105, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 10, 73, 110, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 101, 118, 101, 110, 44, 32, 111, 100, 100, 44, 32, 46, 46, 46, 93, 10, 65, 102, 116, 101, 114, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 108, 111, 103, 105, 99, 44, 32, 115, 116, 97, 99, 107, 32, 108, 111, 111, 107, 115, 32, 108, 105, 107, 101, 10, 91, 104, 105, 44, 32, 108, 111, 44, 32, 46, 46, 46, 93, 10, 84, 104, 105, 115, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 118, 101, 114, 116, 115, 32, 116, 104, 101, 32, 97, 99, 116, 105, 111, 110, 32, 100, 111, 110, 101, 32, 98, 121, 32, 96, 116, 111, 95, 98, 105, 116, 95, 105, 110, 116, 101, 114, 108, 101, 97, 118, 101, 100, 96, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 101, 100, 32, 97, 98, 111, 118, 101, 46, 10, 82, 101, 97, 100, 32, 109, 111, 114, 101, 32, 97, 98, 111, 117, 116, 32, 98, 105, 116, 32, 105, 110, 116, 101, 114, 108, 101, 97, 118, 101, 100, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 115, 101, 99, 116, 105, 111, 110, 32, 50, 46, 49, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 107, 101, 99, 99, 97, 107, 46, 116, 101, 97, 109, 47, 102, 105, 108, 101, 115, 47, 75, 101, 99, 99, 97, 107, 45, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 45, 51, 46, 50, 46, 112, 100, 102, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 105, 116, 122, 109, 101, 97, 110, 106, 97, 110, 47, 109, 101, 114, 107, 108, 105, 122, 101, 45, 115, 104, 97, 47, 98, 108, 111, 98, 47, 49, 100, 51, 53, 97, 97, 101, 57, 100, 97, 55, 102, 101, 100, 50, 48, 49, 50, 55, 52, 56, 57, 102, 51, 54, 50, 98, 52, 98, 99, 57, 51, 50, 52, 50, 97, 53, 49, 54, 99, 47, 105, 110, 99, 108, 117, 100, 101, 47, 117, 116, 105, 108, 115, 46, 104, 112, 112, 35, 76, 49, 53, 49, 45, 76, 49, 55, 53, 10, 102, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 104, 105, 103, 104, 101, 114, 32, 108, 101, 118, 101, 108, 32, 108, 97, 110, 103, 117, 97, 103, 101, 46, 1, 0, 0, 6, 0, 185, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 189, 8, 36, 0, 78, 2, 130, 78, 2, 130, 113, 113, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 71, 130, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 71, 82, 31, 130, 82, 30, 73, 149, 73, 130, 113, 113, 185, 1, 0, 0, 1, 0, 0, 0, 0, 0, 71, 130, 185, 1, 0, 0, 1, 0, 0, 0, 0, 0, 71, 82, 15, 130, 82, 14, 73, 73, 149, 78, 1, 165, 150, 78, 1, 166, 149, 107, 149, 107, 14, 116, 111, 95, 115, 116, 97, 116, 101, 95, 97, 114, 114, 97, 121, 0, 0, 0, 0, 0, 45, 0, 254, 233, 8, 11, 0, 167, 211, 7, 0, 150, 150, 211, 7, 0, 150, 150, 114, 198, 108, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 114, 198, 108, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 198, 108, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 198, 108, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 198, 108, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 198, 108, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 198, 108, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 198, 108, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 198, 108, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 9, 116, 111, 95, 100, 105, 103, 101, 115, 116, 0, 0, 0, 0, 0, 1, 0, 254, 36, 9, 3, 0, 154, 154, 211, 8, 0, 4, 104, 97, 115, 104, 64, 3, 71, 105, 118, 101, 110, 32, 54, 52, 32, 45, 98, 121, 116, 101, 115, 32, 105, 110, 112, 117, 116, 44, 32, 105, 110, 32, 116, 101, 114, 109, 115, 32, 111, 102, 32, 115, 105, 120, 116, 101, 101, 110, 32, 51, 50, 32, 45, 98, 105, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115, 44, 32, 119, 104, 101, 114, 101, 32, 101, 97, 99, 104, 32, 112, 97, 105, 114, 10, 111, 102, 32, 116, 104, 101, 109, 32, 104, 111, 108, 100, 105, 110, 103, 32, 104, 105, 103, 104, 101, 114, 32, 38, 32, 108, 111, 119, 101, 114, 32, 51, 50, 32, 45, 98, 105, 116, 115, 32, 111, 102, 32, 54, 52, 32, 45, 98, 105, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 40, 32, 114, 101, 105, 110, 116, 101, 114, 112, 114, 101, 116, 101, 100, 32, 111, 110, 10, 104, 111, 115, 116, 32, 67, 80, 85, 32, 102, 114, 111, 109, 32, 108, 105, 116, 116, 108, 101, 32, 101, 110, 100, 105, 97, 110, 32, 98, 121, 116, 101, 32, 97, 114, 114, 97, 121, 32, 41, 32, 114, 101, 115, 112, 101, 99, 116, 105, 118, 101, 108, 121, 44, 32, 116, 104, 105, 115, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 51, 50, 32, 45, 98, 121, 116, 101, 115, 10, 107, 101, 99, 99, 97, 107, 50, 53, 54, 32, 100, 105, 103, 101, 115, 116, 44, 32, 104, 101, 108, 100, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 116, 111, 112, 44, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 116, 101, 114, 109, 115, 32, 111, 102, 32, 101, 105, 103, 104, 116, 32, 51, 50, 32, 45, 98, 105, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115, 44, 10, 119, 104, 101, 114, 101, 32, 101, 97, 99, 104, 32, 112, 97, 105, 114, 32, 111, 102, 32, 116, 104, 101, 109, 32, 107, 101, 101, 112, 115, 32, 104, 105, 103, 104, 101, 114, 32, 97, 110, 100, 32, 108, 111, 119, 101, 114, 32, 51, 50, 32, 45, 98, 105, 116, 115, 32, 111, 102, 32, 54, 52, 32, 45, 98, 105, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 115, 112, 101, 99, 116, 105, 118, 101, 108, 121, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 105, 119, 111, 114, 100, 48, 44, 32, 105, 119, 111, 114, 100, 49, 44, 32, 105, 119, 111, 114, 100, 50, 44, 32, 105, 119, 111, 114, 100, 51, 44, 32, 105, 119, 111, 114, 100, 52, 44, 32, 105, 119, 111, 114, 100, 53, 44, 32, 105, 119, 111, 114, 100, 54, 44, 32, 105, 119, 111, 114, 100, 55, 44, 10, 105, 119, 111, 114, 100, 56, 44, 32, 105, 119, 111, 114, 100, 57, 44, 32, 105, 119, 111, 114, 100, 49, 48, 44, 32, 105, 119, 111, 114, 100, 49, 49, 44, 32, 105, 119, 111, 114, 100, 49, 50, 44, 32, 105, 119, 111, 114, 100, 49, 51, 44, 32, 105, 119, 111, 114, 100, 49, 52, 44, 32, 105, 119, 111, 114, 100, 49, 53, 44, 32, 46, 46, 46, 32, 93, 10, 70, 105, 110, 97, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 111, 119, 111, 114, 100, 48, 44, 32, 111, 119, 111, 114, 100, 49, 44, 32, 111, 119, 111, 114, 100, 50, 44, 32, 111, 119, 111, 114, 100, 51, 44, 32, 111, 119, 111, 114, 100, 52, 44, 32, 111, 119, 111, 114, 100, 53, 44, 32, 111, 119, 111, 114, 100, 54, 44, 32, 111, 119, 111, 114, 100, 55, 44, 32, 46, 46, 46, 32, 93, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 105, 116, 122, 109, 101, 97, 110, 106, 97, 110, 47, 109, 101, 114, 107, 108, 105, 122, 101, 45, 115, 104, 97, 47, 98, 108, 111, 98, 47, 49, 100, 51, 53, 97, 97, 101, 57, 100, 97, 55, 102, 101, 100, 50, 48, 49, 50, 55, 52, 56, 57, 102, 51, 54, 50, 98, 52, 98, 99, 57, 51, 50, 52, 50, 97, 53, 49, 54, 99, 47, 105, 110, 99, 108, 117, 100, 101, 47, 107, 101, 99, 99, 97, 107, 95, 50, 53, 54, 46, 104, 112, 112, 35, 76, 50, 51, 50, 45, 76, 50, 53, 55, 1, 13, 0, 9, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 211, 9, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 10, 0]),
("std::crypto::hashes::blake3",vm_assembly::ProcedureId([125, 62, 90, 38, 42, 214, 31, 95, 122, 216, 196, 243, 27, 252, 211, 140, 67, 212, 156, 35, 144, 101, 113, 94]),"#! Initializes four memory addresses, provided for storing initial 4x4 blake3 
#! state matrix ( i.e. 16 elements each of 32 -bit ), for computing blake3 2-to-1 hash
#!
#! Expected stack state:
#!
#! [state_0_3_addr, state_4_7_addr, state_8_11_addr, state_12_15_addr]
#!
#! Note, state_`i`_`j`_addr -> absolute address of {state[i], state[i+1], state[i+2], state[i+3]} in memory | j = i+3
#!
#! Final stack state:
#!
#! [...]
#!
#! Initialized stack state is written back to provided memory addresses.
#!
#! Functionally this routine is equivalent to https://github.com/itzmeanjan/blake3/blob/f07d32e/include/blake3.hpp#!L1709-L1713
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

#! Permutes ordered message words, kept on stack top ( = sixteen 32 -bit BLAKE3 words )
#!
#! Expected stack top: 
#!
#! [s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15]
#!
#! After permutation, stack top:
#!
#! [s2, s6, s3, s10, s7, s0, s4, s13, s1, s11, s12, s5, s9, s14, s15, s8]
#!
#! See https://github.com/itzmeanjan/blake3/blob/f07d32ec10cbc8a10663b7e6539e0b1dab3e453b/include/blake3.hpp#!L1623-L1639
#! and https://github.com/maticnetwork/miden/pull/313#!discussion_r922627984
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

#! Given blake3 state matrix on stack top ( in order ) as 16 elements ( each of 32 -bit ),
#! this routine computes output chaining value i.e. 2-to-1 hashing digest.
#!
#! Expected stack state:
#!
#! [state0, state1, state2, state3, state4, state5, state6, state7, state8, state9, state10, state11, state12, state13, state14, state15]
#!
#! After finalizing, stack should look like
#!
#! [dig0, dig1, dig2, dig3, dig4, dig5, dig6, dig7]
#!
#! See https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#!L116-L119 ,
#! you'll notice I've skipped executing second statement in loop body of above hyperlinked implementation,
#! that's because it doesn't dictate what output of 2-to-1 hash will be.
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

#! Given blake3 state matrix ( total 16 elements, each of 32 -bit ) and 
#! 8 message words ( each of 32 -bit ), this routine performs column-wise mixing
#! of message words into blake3 hash state.
#!
#! Functionality wise this routine is equivalent to https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#!L55-L59
#!
#! Expected stack state:
#!
#! [state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr, m0, m1, m2, m3, m4, m5, m6, m7]
#!
#! Note, state_`i`_`j`_addr -> absolute address of {state[i], state[i+1], state[i+2], state[i+3]} in memory | j = i+3
#!
#! Meaning four consecutive blake3 state words can be read from memory easily.
#!
#! Final stack state:
#!
#! [state0, state1, state2, state3, state4, state5, state6, state7, state8, state9, state10, state11, state12, state13, state14, state15]
#!
#! i.e. whole blake3 state is placed on stack ( in order ).
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

#! Given blake3 state matrix ( total 16 elements, each of 32 -bit ) and 
#! 8 message words ( each of 32 -bit ), this routine performs diagonal-wise mixing
#! of message words into blake3 hash state.
#!
#! Functionality wise this routine is equivalent to https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#!L61-L64
#!
#! Expected stack state:
#!
#! [state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr, m0, m1, m2, m3, m4, m5, m6, m7]
#!
#! Note, state_`i`_`j`_addr -> absolute address of {state[i], state[i+1], state[i+2], state[i+3]} in memory | j = i+3
#!
#! Meaning four consecutive blake3 state words can be read from memory easily.
#!
#! Final stack state:
#!
#! [state0, state1, state2, state3, state4, state5, state6, state7, state8, state9, state10, state11, state12, state13, state14, state15]
#!
#! i.e. whole blake3 state is placed on stack ( in order ).
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

#! Given blake3 state matrix ( total 16 elements, each of 32 -bit ) and 
#! 16 message words ( each of 32 -bit ), this routine applies single round of mixing
#! of message words into hash state i.e. msg_word[0..8] are mixed into hash state using
#! columnar mixing while remaining message words ( msg_word[8..16] ) are mixed into hash state
#! using diagonal mixing.
#!
#! Functionality wise this routine is equivalent to https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#!L54-L65
#!
#! Expected stack state:
#!
#! [state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr, m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15]
#!
#! Note, state_`i`_`j`_addr -> absolute address of {state[i], state[i+1], state[i+2], state[i+3]} in memory | j = i+3
#!
#! Meaning four consecutive blake3 state words can be read from memory easily.
#!
#! Final stack state:
#!
#! [...]
#!
#! i.e. mixed state matrix lives in memory addresses {state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr}, 
#! which were provided, on stack top, while invoking this routine.
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

#! Given blake3 state matrix ( total 16 elements, each of 32 -bit ) and a message block
#! i.e. 16 message words ( each of 32 -bit ), this routine applies 7 rounds of mixing
#! of (permuted) message words into hash state.
#!
#! Functionality wise this routine is equivalent to https://github.com/BLAKE3-team/BLAKE3/blob/da4c792/reference_impl/reference_impl.rs#!L75-L114
#!
#! Expected stack state:
#!
#! [state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr, m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15]
#!
#! Note, state_`i`_`j`_addr -> absolute address of {state[i], state[i+1], state[i+2], state[i+3]} in memory | j = i+3
#!
#! Meaning four consecutive blake3 state words can be read from memory easily.
#!
#! Final stack state:
#!
#! [...]
#!
#! i.e. 7 -round mixed state matrix lives in memory addresses {state0_3_addr, state4_7_addr, state8_11_addr, state12_15_addr}, 
#! which were provided, on stack top, while invoking this routine. So updated state matrix can be read by caller routine, by reading
#! the content of memory addresses where state was provided as routine input.
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

#! Blake3 2-to-1 hash function, which takes 64 -bytes input and produces 32 -bytes output digest
#!
#! Expected stack state:
#!
#! [msg0, msg1, msg2, msg3, msg4, msg5, msg6, msg7, msg8, msg9, msg10, msg11, msg12, msg13, msg14, msg15]
#!
#! msg`i` -> 32 -bit message word | i ∈ [0, 16)
#!
#! Output stack state:
#!
#! [dig0, dig1, dig2, dig3, dig4, dig5, dig6, dig7]
#!
#! dig`i` -> 32 -bit digest word | i ∈ [0, 8)
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
",&[8, 0, 10, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 0, 0, 0, 0, 0, 16, 0, 185, 4, 58, 245, 79, 165, 0, 0, 0, 0, 114, 243, 110, 60, 0, 0, 0, 0, 133, 174, 103, 187, 0, 0, 0, 0, 103, 230, 9, 106, 0, 0, 0, 0, 151, 198, 108, 185, 4, 25, 205, 224, 91, 0, 0, 0, 0, 171, 217, 131, 31, 0, 0, 0, 0, 140, 104, 5, 155, 0, 0, 0, 0, 127, 82, 14, 81, 0, 0, 0, 0, 151, 198, 108, 185, 4, 58, 245, 79, 165, 0, 0, 0, 0, 114, 243, 110, 60, 0, 0, 0, 0, 133, 174, 103, 187, 0, 0, 0, 0, 103, 230, 9, 106, 0, 0, 0, 0, 151, 198, 108, 185, 4, 11, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 17, 112, 101, 114, 109, 117, 116, 101, 95, 109, 115, 103, 95, 119, 111, 114, 100, 115, 0, 0, 0, 0, 0, 20, 0, 170, 152, 165, 151, 170, 147, 130, 170, 148, 149, 170, 145, 146, 150, 169, 168, 150, 145, 150, 148, 8, 102, 105, 110, 97, 108, 105, 122, 101, 0, 0, 0, 0, 0, 30, 0, 155, 73, 130, 155, 73, 130, 149, 155, 73, 165, 150, 155, 73, 166, 151, 155, 73, 167, 152, 155, 73, 168, 153, 155, 73, 169, 154, 155, 73, 170, 15, 99, 111, 108, 117, 109, 110, 97, 114, 95, 109, 105, 120, 105, 110, 103, 0, 0, 0, 1, 0, 174, 0, 146, 145, 154, 153, 152, 151, 200, 0, 0, 0, 0, 0, 0, 0, 0, 156, 191, 155, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 155, 115, 43, 107, 130, 155, 116, 43, 107, 130, 149, 116, 156, 43, 107, 165, 150, 117, 156, 43, 107, 166, 156, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 114, 73, 86, 16, 130, 115, 73, 86, 16, 130, 149, 116, 73, 86, 16, 165, 150, 117, 73, 86, 16, 166, 159, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 114, 39, 130, 115, 39, 130, 149, 116, 39, 165, 150, 117, 39, 166, 164, 114, 73, 86, 12, 130, 115, 73, 86, 12, 130, 149, 116, 73, 86, 12, 165, 150, 117, 73, 86, 12, 166, 164, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 145, 151, 118, 43, 107, 130, 151, 118, 43, 107, 130, 149, 151, 118, 43, 107, 165, 150, 151, 118, 43, 107, 166, 164, 114, 73, 86, 8, 130, 115, 73, 86, 8, 130, 149, 116, 73, 86, 8, 165, 150, 117, 73, 86, 8, 166, 164, 114, 39, 130, 115, 39, 130, 149, 116, 39, 165, 150, 117, 39, 166, 164, 114, 73, 86, 7, 130, 115, 73, 86, 7, 130, 149, 116, 73, 86, 7, 165, 150, 117, 73, 86, 7, 166, 164, 15, 100, 105, 97, 103, 111, 110, 97, 108, 95, 109, 105, 120, 105, 110, 103, 0, 0, 0, 1, 0, 174, 0, 146, 145, 154, 153, 152, 151, 200, 0, 0, 0, 0, 0, 0, 0, 0, 156, 191, 155, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 155, 116, 43, 107, 130, 155, 117, 43, 107, 130, 149, 155, 118, 43, 107, 165, 150, 155, 115, 43, 107, 166, 156, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 150, 114, 73, 86, 16, 166, 115, 73, 86, 16, 130, 116, 73, 86, 16, 130, 149, 117, 73, 86, 16, 165, 159, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 149, 117, 39, 165, 150, 114, 39, 166, 115, 39, 130, 116, 39, 130, 164, 130, 116, 73, 86, 12, 130, 149, 117, 73, 86, 12, 165, 150, 114, 73, 86, 12, 166, 115, 73, 86, 12, 164, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 145, 151, 119, 43, 107, 130, 151, 119, 43, 107, 130, 149, 151, 119, 43, 107, 165, 150, 151, 115, 43, 107, 166, 164, 150, 114, 73, 86, 8, 166, 115, 73, 86, 8, 130, 116, 73, 86, 8, 130, 149, 117, 73, 86, 8, 165, 164, 149, 117, 39, 165, 150, 114, 39, 166, 115, 39, 130, 116, 39, 130, 164, 130, 116, 73, 86, 7, 130, 149, 117, 73, 86, 7, 165, 150, 114, 73, 86, 7, 166, 115, 73, 86, 7, 164, 5, 114, 111, 117, 110, 100, 0, 0, 0, 5, 0, 23, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 211, 3, 0, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 200, 3, 0, 0, 0, 0, 0, 0, 0, 108, 200, 4, 0, 0, 0, 0, 0, 0, 0, 108, 186, 4, 0, 0, 0, 0, 0, 0, 0, 186, 3, 0, 0, 0, 0, 0, 0, 0, 186, 2, 0, 0, 0, 0, 0, 0, 0, 186, 1, 0, 0, 0, 0, 0, 0, 0, 211, 4, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 145, 151, 198, 108, 254, 190, 1, 6, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 166, 145, 151, 198, 108, 254, 198, 1, 1, 0, 107, 8, 99, 111, 109, 112, 114, 101, 115, 115, 0, 0, 0, 1, 0, 6, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 254, 205, 1, 5, 0, 254, 206, 1, 1, 0, 129, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 211, 1, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 4, 104, 97, 115, 104, 123, 1, 66, 108, 97, 107, 101, 51, 32, 50, 45, 116, 111, 45, 49, 32, 104, 97, 115, 104, 32, 102, 117, 110, 99, 116, 105, 111, 110, 44, 32, 119, 104, 105, 99, 104, 32, 116, 97, 107, 101, 115, 32, 54, 52, 32, 45, 98, 121, 116, 101, 115, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 112, 114, 111, 100, 117, 99, 101, 115, 32, 51, 50, 32, 45, 98, 121, 116, 101, 115, 32, 111, 117, 116, 112, 117, 116, 32, 100, 105, 103, 101, 115, 116, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 58, 10, 91, 109, 115, 103, 48, 44, 32, 109, 115, 103, 49, 44, 32, 109, 115, 103, 50, 44, 32, 109, 115, 103, 51, 44, 32, 109, 115, 103, 52, 44, 32, 109, 115, 103, 53, 44, 32, 109, 115, 103, 54, 44, 32, 109, 115, 103, 55, 44, 32, 109, 115, 103, 56, 44, 32, 109, 115, 103, 57, 44, 32, 109, 115, 103, 49, 48, 44, 32, 109, 115, 103, 49, 49, 44, 32, 109, 115, 103, 49, 50, 44, 32, 109, 115, 103, 49, 51, 44, 32, 109, 115, 103, 49, 52, 44, 32, 109, 115, 103, 49, 53, 93, 10, 109, 115, 103, 96, 105, 96, 32, 45, 62, 32, 51, 50, 32, 45, 98, 105, 116, 32, 109, 101, 115, 115, 97, 103, 101, 32, 119, 111, 114, 100, 32, 124, 32, 105, 32, 226, 136, 136, 32, 91, 48, 44, 32, 49, 54, 41, 10, 79, 117, 116, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 58, 10, 91, 100, 105, 103, 48, 44, 32, 100, 105, 103, 49, 44, 32, 100, 105, 103, 50, 44, 32, 100, 105, 103, 51, 44, 32, 100, 105, 103, 52, 44, 32, 100, 105, 103, 53, 44, 32, 100, 105, 103, 54, 44, 32, 100, 105, 103, 55, 93, 10, 100, 105, 103, 96, 105, 96, 32, 45, 62, 32, 51, 50, 32, 45, 98, 105, 116, 32, 100, 105, 103, 101, 115, 116, 32, 119, 111, 114, 100, 32, 124, 32, 105, 32, 226, 136, 136, 32, 91, 48, 44, 32, 56, 41, 1, 4, 0, 19, 0, 186, 3, 0, 0, 0, 0, 0, 0, 0, 186, 2, 0, 0, 0, 0, 0, 0, 0, 186, 1, 0, 0, 0, 0, 0, 0, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 211, 0, 0, 186, 3, 0, 0, 0, 0, 0, 0, 0, 186, 2, 0, 0, 0, 0, 0, 0, 0, 186, 1, 0, 0, 0, 0, 0, 0, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 2, 0]),
("std::crypto::dsa::falcon",vm_assembly::ProcedureId([189, 139, 200, 202, 100, 233, 83, 114, 77, 58, 86, 188, 150, 133, 31, 254, 25, 201, 51, 28, 11, 81, 40, 75]),"use.std::math::poly512

#! Given an element on stack top, this routine normalizes that element in 
#! interval (-q/2, q/2] | q = 12289
#!
#! Imagine, a is the provided element, which needs to be normalized
#!
#! b = normalize(a)
#!   = (a + (q >> 1)) % q - (q >> 1) | a ∈ [0, q), q = 12289
#!
#! Note, normalization requires that we can represent the number as signed integer,
#! which is not allowed inside Miden VM stack. But we can ignore the sign of integer and only
#! store the absolute value as field element. This can be safely done because after normalization
#! anyway `b` will be squared ( for computing norm of a vector i.e. polynomial, where b is a coefficient ).
#! That means we can just drop the sign, and that's what is done in this routine.
#!
#! To be more concrete, normalization of 12166 ( = a ) should result into -123, but absolute value 
#! 123 will be kept on stack. While normalization of 21, should result into 21, which has absolute
#! value 21 --- that's what is kept on stack.
#!
#! Expected stack state :
#!
#! [a, ...]
#!
#! After normalization ( represented using unsigned integer i.e. Miden field element ) stack looks like
#!
#! [b, ...]
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

#! Given four elements from Falcon prime field, on stack top, this routine 
#! normalizes each of them, using above defined `normalize()` routine.
#!
#! Expected stack state :
#!
#! [a0, a1, a2, a3, ...]
#!
#! Output stack state :
#!
#! [b0, b1, b2, b3, ...]
#!
#! b`i` = normalize(a`i`) | i ∈ [0..4)
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

#! Given a degree 512 polynomial on stack, using its starting (absolute) memory address, 
#! this routine normalizes each coefficient of the polynomial, using above defined 
#! `normalize()` routine
#!
#! Imagine, f is the given polynomial of degree 512. It can be normalized using
#!
#! g = [normalize(f[i]) for i in range(512)]
#!
#! Expected stack state :
#!
#! [f_start_addr, g_start_addr, ...] | next 127 absolute addresses can be computed using `INCR` instruction
#!
#! Post normalization stack state looks like
#!
#! [ ... ]
#!
#! Note, input polynomial which is provided using memory addresses, is not mutated.
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

#! Given four elements on stack top, this routine computes squared norm of that
#! vector ( read polynomial ) with four coefficients.
#!
#! Imagine, given vector is f, which is described as
#!
#! f = [a0, a1, a2, a3]
#!
#! Norm of that vector is
#!
#! √(a0 ^ 2 + a1 ^ 2 + a2 ^ 2 + a3 ^ 2)
#!
#! But we need squared norm, which is just skipping the final square root operation.
#!
#! Expected stack state :
#!
#! [a0, a1, a2, a3, ...]
#!
#! Final stack state :
#!
#! [b, ...] | b = a0 ^ 2 + a1 ^ 2 + a2 ^ 2 + a3 ^ 2
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

#! Given a degree 512 polynomial in coefficient form, as starting (absolute) memory address 
#! on stack, this routine computes squared norm of that vector, using following formula
#!
#! Say, f = [a0, a1, a2, ..., a510, a511]
#!      g = sq_norm(f) = a0 ^ 2 + a1 ^ 2 + ... + a510 ^ 2 + a511 ^ 2
#!
#! Expected input stack state :
#!
#! [f_start_addr, ...] | f_addr`i` holds f[(i << 2) .. ((i+1) << 2)]
#!
#! Consecutive 127 addresses on stack can be computed using `INCR` instruction, because memory 
#! addresses are consecutive i.e. monotonically increasing by 1.
#!
#! Final stack state :
#!
#! [g, ...] | g = sq_norm(f)
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

#! Falcon-512 Digital Signature Verification routine
#!
#! Given four degree-511 polynomials, using initial absolute memory addresses on stack, 
#! this routine checks whether it's a valid Falcon signature or not.
#!
#! Four degree-511 polynomials, which are provided ( in order )
#!
#! f = [f0, f1, ..., f510, f511] -> decompressed Falcon-512 signature
#! g = [g0, g1, ..., g510, g511] -> public key used for signing input message
#! h = [h0, h1, ..., h510, h511] -> input message hashed using SHAKE256 XOF and converted to polynomial
#! k = [k0, k1, ..., k510, k511] -> [abs(i) for i in f] | abs(a) = a < 0 ? 0 - a : a
#!
#! Each of these polynomials are represented using starting absolute memory address. Contiguous 127 
#! memory addresses can be computed by repeated application of INCR instruction ( read add.1 ) on previous
#! absolute memory address.
#!
#! f`i` holds f[(i << 2) .. ((i+1) << 2)] | i ∈ [0..128)
#! g`i` holds g[(i << 2) .. ((i+1) << 2)] | i ∈ [0..128)
#! h`i` holds h[(i << 2) .. ((i+1) << 2)] | i ∈ [0..128)
#! k`i` holds k[(i << 2) .. ((i+1) << 2)] | i ∈ [0..128)
#!
#! Expected stack state :
#!
#! [f_start_addr, g_start_addr, h_start_addr, k_start_addr, ...]
#!
#! After execution of verification routine, stack looks like
#!
#! [ ... ]
#!
#! If verification fails, program panics, due to failure in assertion !
#!
#! Note, input memory addresses are considered to be immutable.
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
",&[6, 0, 9, 110, 111, 114, 109, 97, 108, 105, 122, 101, 0, 0, 0, 0, 0, 4, 0, 110, 185, 1, 0, 24, 0, 0, 0, 0, 0, 0, 28, 253, 7, 0, 185, 1, 0, 24, 0, 0, 0, 0, 0, 0, 3, 212, 85, 132, 203, 155, 10, 43, 66, 153, 188, 247, 113, 182, 11, 149, 253, 89, 63, 20, 200, 120, 146, 57, 157, 137, 110, 185, 1, 0, 24, 0, 0, 0, 0, 0, 0, 29, 253, 2, 0, 185, 1, 0, 24, 0, 0, 0, 0, 0, 0, 5, 3, 0, 185, 1, 0, 24, 0, 0, 0, 0, 0, 0, 130, 5, 0, 0, 14, 110, 111, 114, 109, 97, 108, 105, 122, 101, 95, 119, 111, 114, 100, 0, 0, 0, 0, 0, 10, 0, 211, 0, 0, 130, 211, 0, 0, 130, 149, 211, 0, 0, 165, 150, 211, 0, 0, 166, 17, 110, 111, 114, 109, 97, 108, 105, 122, 101, 95, 112, 111, 108, 121, 53, 49, 50, 53, 2, 71, 105, 118, 101, 110, 32, 97, 32, 100, 101, 103, 114, 101, 101, 32, 53, 49, 50, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 117, 115, 105, 110, 103, 32, 105, 116, 115, 32, 115, 116, 97, 114, 116, 105, 110, 103, 32, 40, 97, 98, 115, 111, 108, 117, 116, 101, 41, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 44, 10, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 115, 32, 101, 97, 99, 104, 32, 99, 111, 101, 102, 102, 105, 99, 105, 101, 110, 116, 32, 111, 102, 32, 116, 104, 101, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 44, 32, 117, 115, 105, 110, 103, 32, 97, 98, 111, 118, 101, 32, 100, 101, 102, 105, 110, 101, 100, 10, 96, 110, 111, 114, 109, 97, 108, 105, 122, 101, 40, 41, 96, 32, 114, 111, 117, 116, 105, 110, 101, 10, 73, 109, 97, 103, 105, 110, 101, 44, 32, 102, 32, 105, 115, 32, 116, 104, 101, 32, 103, 105, 118, 101, 110, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 111, 102, 32, 100, 101, 103, 114, 101, 101, 32, 53, 49, 50, 46, 32, 73, 116, 32, 99, 97, 110, 32, 98, 101, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 117, 115, 105, 110, 103, 10, 103, 32, 61, 32, 91, 110, 111, 114, 109, 97, 108, 105, 122, 101, 40, 102, 91, 105, 93, 41, 32, 102, 111, 114, 32, 105, 32, 105, 110, 32, 114, 97, 110, 103, 101, 40, 53, 49, 50, 41, 93, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 102, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 103, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 46, 46, 46, 93, 32, 124, 32, 110, 101, 120, 116, 32, 49, 50, 55, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 96, 73, 78, 67, 82, 96, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 10, 80, 111, 115, 116, 32, 110, 111, 114, 109, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 108, 111, 111, 107, 115, 32, 108, 105, 107, 101, 10, 91, 32, 46, 46, 46, 32, 93, 10, 78, 111, 116, 101, 44, 32, 105, 110, 112, 117, 116, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 119, 104, 105, 99, 104, 32, 105, 115, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 117, 115, 105, 110, 103, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 44, 32, 105, 115, 32, 110, 111, 116, 32, 109, 117, 116, 97, 116, 101, 100, 46, 1, 0, 0, 5, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 36, 0, 11, 0, 114, 191, 211, 1, 0, 115, 198, 152, 3, 168, 151, 3, 167, 108, 107, 107, 17, 115, 113, 117, 97, 114, 101, 100, 95, 110, 111, 114, 109, 95, 119, 111, 114, 100, 0, 0, 0, 0, 0, 14, 0, 110, 7, 130, 110, 7, 3, 130, 110, 7, 3, 130, 110, 7, 3, 20, 115, 113, 117, 97, 114, 101, 100, 95, 110, 111, 114, 109, 95, 112, 111, 108, 121, 53, 49, 50, 56, 2, 71, 105, 118, 101, 110, 32, 97, 32, 100, 101, 103, 114, 101, 101, 32, 53, 49, 50, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 105, 110, 32, 99, 111, 101, 102, 102, 105, 99, 105, 101, 110, 116, 32, 102, 111, 114, 109, 44, 32, 97, 115, 32, 115, 116, 97, 114, 116, 105, 110, 103, 32, 40, 97, 98, 115, 111, 108, 117, 116, 101, 41, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 10, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 115, 113, 117, 97, 114, 101, 100, 32, 110, 111, 114, 109, 32, 111, 102, 32, 116, 104, 97, 116, 32, 118, 101, 99, 116, 111, 114, 44, 32, 117, 115, 105, 110, 103, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 102, 111, 114, 109, 117, 108, 97, 10, 83, 97, 121, 44, 32, 102, 32, 61, 32, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 46, 46, 46, 44, 32, 97, 53, 49, 48, 44, 32, 97, 53, 49, 49, 93, 10, 103, 32, 61, 32, 115, 113, 95, 110, 111, 114, 109, 40, 102, 41, 32, 61, 32, 97, 48, 32, 94, 32, 50, 32, 43, 32, 97, 49, 32, 94, 32, 50, 32, 43, 32, 46, 46, 46, 32, 43, 32, 97, 53, 49, 48, 32, 94, 32, 50, 32, 43, 32, 97, 53, 49, 49, 32, 94, 32, 50, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 105, 110, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 102, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 46, 46, 46, 93, 32, 124, 32, 102, 95, 97, 100, 100, 114, 96, 105, 96, 32, 104, 111, 108, 100, 115, 32, 102, 91, 40, 105, 32, 60, 60, 32, 50, 41, 32, 46, 46, 32, 40, 40, 105, 43, 49, 41, 32, 60, 60, 32, 50, 41, 93, 10, 67, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 49, 50, 55, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 96, 73, 78, 67, 82, 96, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 98, 101, 99, 97, 117, 115, 101, 32, 109, 101, 109, 111, 114, 121, 10, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 99, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 105, 46, 101, 46, 32, 109, 111, 110, 111, 116, 111, 110, 105, 99, 97, 108, 108, 121, 32, 105, 110, 99, 114, 101, 97, 115, 105, 110, 103, 32, 98, 121, 32, 49, 46, 10, 70, 105, 110, 97, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 103, 44, 32, 46, 46, 46, 93, 32, 124, 32, 103, 32, 61, 32, 115, 113, 95, 110, 111, 114, 109, 40, 102, 41, 1, 0, 0, 5, 0, 185, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 71, 0, 8, 0, 115, 191, 211, 3, 0, 3, 130, 3, 130, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 108, 130, 107, 6, 118, 101, 114, 105, 102, 121, 37, 5, 70, 97, 108, 99, 111, 110, 45, 53, 49, 50, 32, 68, 105, 103, 105, 116, 97, 108, 32, 83, 105, 103, 110, 97, 116, 117, 114, 101, 32, 86, 101, 114, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 114, 111, 117, 116, 105, 110, 101, 10, 71, 105, 118, 101, 110, 32, 102, 111, 117, 114, 32, 100, 101, 103, 114, 101, 101, 45, 53, 49, 49, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 115, 44, 32, 117, 115, 105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 10, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 104, 101, 99, 107, 115, 32, 119, 104, 101, 116, 104, 101, 114, 32, 105, 116, 39, 115, 32, 97, 32, 118, 97, 108, 105, 100, 32, 70, 97, 108, 99, 111, 110, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 111, 114, 32, 110, 111, 116, 46, 10, 70, 111, 117, 114, 32, 100, 101, 103, 114, 101, 101, 45, 53, 49, 49, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 115, 44, 32, 119, 104, 105, 99, 104, 32, 97, 114, 101, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 40, 32, 105, 110, 32, 111, 114, 100, 101, 114, 32, 41, 10, 102, 32, 61, 32, 91, 102, 48, 44, 32, 102, 49, 44, 32, 46, 46, 46, 44, 32, 102, 53, 49, 48, 44, 32, 102, 53, 49, 49, 93, 32, 45, 62, 32, 100, 101, 99, 111, 109, 112, 114, 101, 115, 115, 101, 100, 32, 70, 97, 108, 99, 111, 110, 45, 53, 49, 50, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 10, 103, 32, 61, 32, 91, 103, 48, 44, 32, 103, 49, 44, 32, 46, 46, 46, 44, 32, 103, 53, 49, 48, 44, 32, 103, 53, 49, 49, 93, 32, 45, 62, 32, 112, 117, 98, 108, 105, 99, 32, 107, 101, 121, 32, 117, 115, 101, 100, 32, 102, 111, 114, 32, 115, 105, 103, 110, 105, 110, 103, 32, 105, 110, 112, 117, 116, 32, 109, 101, 115, 115, 97, 103, 101, 10, 104, 32, 61, 32, 91, 104, 48, 44, 32, 104, 49, 44, 32, 46, 46, 46, 44, 32, 104, 53, 49, 48, 44, 32, 104, 53, 49, 49, 93, 32, 45, 62, 32, 105, 110, 112, 117, 116, 32, 109, 101, 115, 115, 97, 103, 101, 32, 104, 97, 115, 104, 101, 100, 32, 117, 115, 105, 110, 103, 32, 83, 72, 65, 75, 69, 50, 53, 54, 32, 88, 79, 70, 32, 97, 110, 100, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 10, 107, 32, 61, 32, 91, 107, 48, 44, 32, 107, 49, 44, 32, 46, 46, 46, 44, 32, 107, 53, 49, 48, 44, 32, 107, 53, 49, 49, 93, 32, 45, 62, 32, 91, 97, 98, 115, 40, 105, 41, 32, 102, 111, 114, 32, 105, 32, 105, 110, 32, 102, 93, 32, 124, 32, 97, 98, 115, 40, 97, 41, 32, 61, 32, 97, 32, 60, 32, 48, 32, 63, 32, 48, 32, 45, 32, 97, 32, 58, 32, 97, 10, 69, 97, 99, 104, 32, 111, 102, 32, 116, 104, 101, 115, 101, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 115, 32, 97, 114, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 115, 116, 97, 114, 116, 105, 110, 103, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 46, 32, 67, 111, 110, 116, 105, 103, 117, 111, 117, 115, 32, 49, 50, 55, 10, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 98, 121, 32, 114, 101, 112, 101, 97, 116, 101, 100, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 73, 78, 67, 82, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 40, 32, 114, 101, 97, 100, 32, 97, 100, 100, 46, 49, 32, 41, 32, 111, 110, 32, 112, 114, 101, 118, 105, 111, 117, 115, 10, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 46, 10, 102, 96, 105, 96, 32, 104, 111, 108, 100, 115, 32, 102, 91, 40, 105, 32, 60, 60, 32, 50, 41, 32, 46, 46, 32, 40, 40, 105, 43, 49, 41, 32, 60, 60, 32, 50, 41, 93, 32, 124, 32, 105, 32, 226, 136, 136, 32, 91, 48, 46, 46, 49, 50, 56, 41, 10, 103, 96, 105, 96, 32, 104, 111, 108, 100, 115, 32, 103, 91, 40, 105, 32, 60, 60, 32, 50, 41, 32, 46, 46, 32, 40, 40, 105, 43, 49, 41, 32, 60, 60, 32, 50, 41, 93, 32, 124, 32, 105, 32, 226, 136, 136, 32, 91, 48, 46, 46, 49, 50, 56, 41, 10, 104, 96, 105, 96, 32, 104, 111, 108, 100, 115, 32, 104, 91, 40, 105, 32, 60, 60, 32, 50, 41, 32, 46, 46, 32, 40, 40, 105, 43, 49, 41, 32, 60, 60, 32, 50, 41, 93, 32, 124, 32, 105, 32, 226, 136, 136, 32, 91, 48, 46, 46, 49, 50, 56, 41, 10, 107, 96, 105, 96, 32, 104, 111, 108, 100, 115, 32, 107, 91, 40, 105, 32, 60, 60, 32, 50, 41, 32, 46, 46, 32, 40, 40, 105, 43, 49, 41, 32, 60, 60, 32, 50, 41, 93, 32, 124, 32, 105, 32, 226, 136, 136, 32, 91, 48, 46, 46, 49, 50, 56, 41, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 102, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 103, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 104, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 107, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 46, 46, 46, 93, 10, 65, 102, 116, 101, 114, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 111, 102, 32, 118, 101, 114, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 114, 111, 117, 116, 105, 110, 101, 44, 32, 115, 116, 97, 99, 107, 32, 108, 111, 111, 107, 115, 32, 108, 105, 107, 101, 10, 91, 32, 46, 46, 46, 32, 93, 10, 73, 102, 32, 118, 101, 114, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 102, 97, 105, 108, 115, 44, 32, 112, 114, 111, 103, 114, 97, 109, 32, 112, 97, 110, 105, 99, 115, 44, 32, 100, 117, 101, 32, 116, 111, 32, 102, 97, 105, 108, 117, 114, 101, 32, 105, 110, 32, 97, 115, 115, 101, 114, 116, 105, 111, 110, 32, 33, 10, 78, 111, 116, 101, 44, 32, 105, 110, 112, 117, 116, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 99, 111, 110, 115, 105, 100, 101, 114, 101, 100, 32, 116, 111, 32, 98, 101, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 46, 1, 1, 1, 25, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 165, 212, 198, 4, 141, 102, 17, 204, 28, 154, 71, 189, 42, 106, 248, 32, 176, 63, 110, 187, 169, 166, 97, 67, 211, 29, 186, 128, 0, 0, 0, 0, 0, 0, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 212, 155, 180, 67, 30, 194, 168, 239, 166, 191, 42, 172, 120, 205, 218, 138, 159, 213, 146, 128, 132, 109, 111, 94, 112, 186, 0, 0, 0, 0, 0, 0, 0, 0, 130, 186, 128, 0, 0, 0, 0, 0, 0, 0, 212, 146, 126, 125, 245, 167, 6, 77, 144, 105, 242, 2, 197, 171, 93, 11, 100, 207, 223, 103, 221, 8, 17, 80, 124, 186, 128, 0, 0, 0, 0, 0, 0, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 211, 2, 0, 186, 128, 0, 0, 0, 0, 0, 0, 0, 211, 4, 0, 186, 0, 1, 0, 0, 0, 0, 0, 0, 195, 107, 211, 4, 0, 186, 0, 1, 0, 0, 0, 0, 0, 0, 189, 3, 185, 1, 38, 84, 7, 2, 0, 0, 0, 0, 27, 0]),
("std::math::ntt512",vm_assembly::ProcedureId([22, 123, 245, 116, 227, 10, 59, 231, 204, 174, 119, 65, 130, 90, 169, 173, 41, 172, 125, 73, 87, 127, 16, 74]),"#! Applies four NTT butterflies on four different indices, given following stack state
#!
#! [k0, k1, k2, k3, A0, B0, C0, D0, A1, B1, C1, D1]
#! 
#! Here k`i` => i-th constant i.e. ω raised to *some* power | ω => 2N -th primitive root of unity, N = 512
#!
#! A{0, 1} -> first butterfly will be applied on these two elements
#! B{0, 1} -> second butterfly will be applied on these two elements
#! C{0, 1} -> third butterfly will be applied on these two elements
#! D{0, 1} -> fourth butterfly will be applied on these two elements
#!
#! Four independent butterflies are applied in following way
#!
#! ζ = k0 * A0  | ζ = k1 * B0  | ζ = k2 * C0  | ζ = k3 * D0
#! --- --- --- --- --- --- --- --- --- --- --- --- --- --- -
#! A0' = A1 - ζ | B0' = B1 - ζ | C0' = C1 - ζ | D0' = D1 - ζ
#! A1' = A1 + ζ | B1' = B1 + ζ | C1' = C1 + ζ | D1' = D1 + ζ
#!
#! After four independent butterflies are applied, resulting stack state should look like
#!
#! [A0', B0', C0', D0', A1', B1', C1', D1']
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

#! Applies forward NTT on a vector of length 512, where each element ∈ Zp | p = 2^64 − 2^32 + 1,
#! producing elements in frequency domain in bit-reversed order.
#!
#! Expected stack state as input:
#!
#! [start_addr, ...] | Single absolute memory address, where polynomial starts
#!
#! Note, total 128 memory addresses are required for storing whole polynomial. Next 127
#! addresses are consecutive i.e. computable by using `add.1` instruction on previous address.
#!
#! addr{i} holds values V[(i << 2) .. ((i+1) << 2)] | i ∈ [0, 128) and addr0 = start_addr
#!
#! After applying NTT, bit-reversed order vector is returned back as single absolute memory
#! addresses on stack, where it begins storing the polynomial. Consecutive 127 addresses should be
#! computable using `add.1` instruction.
#!
#! [start_addr', ...] | Single absolute memory address, where resulting polynomial starts
#!
#! Note, input memory allocation is not mutated, instead output is stored in different memory allocation.
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

#! Applies four inverse NTT butterflies on four different indices, given following stack state
#!
#! [k0, k1, k2, k3, A0, B0, C0, D0, A1, B1, C1, D1]
#! 
#! Here k`i` => i-th constant i.e. negative of ω raised to *some* power | ω => 2N -th primitive root of unity, N = 512
#!
#! A{0, 1} -> first inverse butterfly will be applied on these two elements
#! B{0, 1} -> second inverse butterfly will be applied on these two elements
#! C{0, 1} -> third inverse butterfly will be applied on these two elements
#! D{0, 1} -> fourth inverse butterfly will be applied on these two elements
#!
#! Four independent inverse butterflies are applied in following way
#!
#! t0 = A1  			   | t1 = B1  			  | t2 = C1				 | t3 = D1
#! --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- -
#! A1' = t0 + A0		   | B1' = t1 + B0 		  | C1' = t2 + C0 		 | D1' = t3 + D0
#! A0' = (t0 - A0) * k0 | B0' = (t1 - B0) * k1 | C0' = (t2 - C0) * k2 | D0' = (t3 - D0) * k3
#!
#! After four independent butterflies are applied, resulting stack state should look like
#!
#! [A0', B0', C0', D0', A1', B1', C1', D1']
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

#! Given four elements on stack top, this routine multiplies each of them by invN = 18410715272404008961,
#! such that N = 512
#!
#! invN = (1/ 512) modulo q | q = 2^64 - 2^32 + 1
#!
#! Expected input stack state:
#!
#! [a0, a1, a2, a3]
#!
#! After applying routine, stack looks like
#!
#! [a0', a1', a2', a3']
#!
#! a{i}' = (a{i} * invN) modulo q | i ∈ [0, 4)
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

#! Applies inverse NTT on a vector of length 512, where each element ∈ Zp | p = 2^64 − 2^32 + 1,
#! producing elements in time domain in standard order, while input vector is expected to be in 
#! bit-reversed order.
#!
#! Expected stack state as input:
#!
#! [start_addr, ...] | Single absolute memory address, where polynomial starts
#!
#! Note, total 128 memory addresses are required for storing whole polynomial. Next 127
#! addresses are consecutive i.e. computable by using `add.1` instruction on previous address.
#!
#! addr{i} holds values V[(i << 2) .. ((i+1) << 2)] | i ∈ [0, 128) and addr0 = start_addr
#!
#! After applying iNTT, normal order vector is returned back as single absolute memory
#! addresses on stack, where it begins storing the polynomial. Consecutive 127 addresses should 
#! similarly be computable using `add.1` instruction.
#!
#! [start_addr', ...] | Single absolute memory address, where resulting polynomial starts
#!
#! Note, input memory allocation is not mutated, instead output is stored in different memory allocation.
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
",&[5, 0, 9, 98, 117, 116, 116, 101, 114, 102, 108, 121, 0, 0, 0, 0, 0, 46, 0, 151, 7, 130, 151, 7, 130, 149, 151, 7, 165, 150, 151, 7, 166, 126, 128, 151, 3, 130, 151, 3, 130, 149, 151, 3, 165, 150, 151, 3, 166, 145, 163, 151, 5, 130, 151, 5, 130, 149, 151, 5, 165, 150, 151, 5, 166, 7, 102, 111, 114, 119, 97, 114, 100, 177, 3, 65, 112, 112, 108, 105, 101, 115, 32, 102, 111, 114, 119, 97, 114, 100, 32, 78, 84, 84, 32, 111, 110, 32, 97, 32, 118, 101, 99, 116, 111, 114, 32, 111, 102, 32, 108, 101, 110, 103, 116, 104, 32, 53, 49, 50, 44, 32, 119, 104, 101, 114, 101, 32, 101, 97, 99, 104, 32, 101, 108, 101, 109, 101, 110, 116, 32, 226, 136, 136, 32, 90, 112, 32, 124, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 226, 136, 146, 32, 50, 94, 51, 50, 32, 43, 32, 49, 44, 10, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 105, 110, 32, 102, 114, 101, 113, 117, 101, 110, 99, 121, 32, 100, 111, 109, 97, 105, 110, 32, 105, 110, 32, 98, 105, 116, 45, 114, 101, 118, 101, 114, 115, 101, 100, 32, 111, 114, 100, 101, 114, 46, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 97, 115, 32, 105, 110, 112, 117, 116, 58, 10, 91, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 46, 46, 46, 93, 32, 124, 32, 83, 105, 110, 103, 108, 101, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 44, 32, 119, 104, 101, 114, 101, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 115, 116, 97, 114, 116, 115, 10, 78, 111, 116, 101, 44, 32, 116, 111, 116, 97, 108, 32, 49, 50, 56, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 102, 111, 114, 32, 115, 116, 111, 114, 105, 110, 103, 32, 119, 104, 111, 108, 101, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 46, 32, 78, 101, 120, 116, 32, 49, 50, 55, 10, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 99, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 105, 46, 101, 46, 32, 99, 111, 109, 112, 117, 116, 97, 98, 108, 101, 32, 98, 121, 32, 117, 115, 105, 110, 103, 32, 96, 97, 100, 100, 46, 49, 96, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 111, 110, 32, 112, 114, 101, 118, 105, 111, 117, 115, 32, 97, 100, 100, 114, 101, 115, 115, 46, 10, 97, 100, 100, 114, 123, 105, 125, 32, 104, 111, 108, 100, 115, 32, 118, 97, 108, 117, 101, 115, 32, 86, 91, 40, 105, 32, 60, 60, 32, 50, 41, 32, 46, 46, 32, 40, 40, 105, 43, 49, 41, 32, 60, 60, 32, 50, 41, 93, 32, 124, 32, 105, 32, 226, 136, 136, 32, 91, 48, 44, 32, 49, 50, 56, 41, 32, 97, 110, 100, 32, 97, 100, 100, 114, 48, 32, 61, 32, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 10, 65, 102, 116, 101, 114, 32, 97, 112, 112, 108, 121, 105, 110, 103, 32, 78, 84, 84, 44, 32, 98, 105, 116, 45, 114, 101, 118, 101, 114, 115, 101, 100, 32, 111, 114, 100, 101, 114, 32, 118, 101, 99, 116, 111, 114, 32, 105, 115, 32, 114, 101, 116, 117, 114, 110, 101, 100, 32, 98, 97, 99, 107, 32, 97, 115, 32, 115, 105, 110, 103, 108, 101, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 10, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 119, 104, 101, 114, 101, 32, 105, 116, 32, 98, 101, 103, 105, 110, 115, 32, 115, 116, 111, 114, 105, 110, 103, 32, 116, 104, 101, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 46, 32, 67, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 49, 50, 55, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 115, 104, 111, 117, 108, 100, 32, 98, 101, 10, 99, 111, 109, 112, 117, 116, 97, 98, 108, 101, 32, 117, 115, 105, 110, 103, 32, 96, 97, 100, 100, 46, 49, 96, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 46, 10, 91, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 39, 44, 32, 46, 46, 46, 93, 32, 124, 32, 83, 105, 110, 103, 108, 101, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 44, 32, 119, 104, 101, 114, 101, 32, 114, 101, 115, 117, 108, 116, 105, 110, 103, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 115, 116, 97, 114, 116, 115, 10, 78, 111, 116, 101, 44, 32, 105, 110, 112, 117, 116, 32, 109, 101, 109, 111, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 105, 115, 32, 110, 111, 116, 32, 109, 117, 116, 97, 116, 101, 100, 44, 32, 105, 110, 115, 116, 101, 97, 100, 32, 111, 117, 116, 112, 117, 116, 32, 105, 115, 32, 115, 116, 111, 114, 101, 100, 32, 105, 110, 32, 100, 105, 102, 102, 101, 114, 101, 110, 116, 32, 109, 101, 109, 111, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 46, 1, 128, 0, 167, 1, 186, 0, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 51, 0, 10, 0, 115, 191, 114, 198, 152, 3, 168, 151, 3, 167, 108, 107, 107, 185, 4, 1, 0, 0, 0, 255, 255, 254, 255, 1, 0, 0, 0, 255, 255, 254, 255, 1, 0, 0, 0, 255, 255, 254, 255, 1, 0, 0, 0, 255, 255, 254, 255, 254, 67, 0, 1, 0, 126, 185, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 186, 64, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 75, 0, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 0, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 254, 103, 0, 1, 0, 126, 185, 4, 0, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 254, 107, 0, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 32, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 116, 0, 7, 0, 254, 117, 0, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 0, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 1, 0, 0, 0, 255, 255, 255, 239, 1, 0, 0, 0, 255, 255, 255, 239, 1, 0, 0, 0, 255, 255, 255, 239, 1, 0, 0, 0, 255, 255, 255, 239, 254, 152, 0, 1, 0, 126, 185, 4, 0, 16, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 254, 156, 0, 1, 0, 126, 185, 4, 0, 0, 240, 255, 255, 255, 15, 0, 0, 0, 240, 255, 255, 255, 15, 0, 0, 0, 240, 255, 255, 255, 15, 0, 0, 0, 240, 255, 255, 255, 15, 0, 254, 160, 0, 1, 0, 126, 185, 4, 1, 0, 0, 0, 239, 255, 255, 255, 1, 0, 0, 0, 239, 255, 255, 255, 1, 0, 0, 0, 239, 255, 255, 255, 1, 0, 0, 0, 239, 255, 255, 255, 254, 164, 0, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 16, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 173, 0, 7, 0, 254, 174, 0, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 0, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 254, 209, 0, 1, 0, 126, 185, 4, 0, 192, 255, 255, 255, 63, 0, 0, 0, 192, 255, 255, 255, 63, 0, 0, 0, 192, 255, 255, 255, 63, 0, 0, 0, 192, 255, 255, 255, 63, 0, 0, 254, 213, 0, 1, 0, 126, 185, 4, 1, 0, 0, 0, 255, 255, 191, 255, 1, 0, 0, 0, 255, 255, 191, 255, 1, 0, 0, 0, 255, 255, 191, 255, 1, 0, 0, 0, 255, 255, 191, 255, 254, 217, 0, 1, 0, 126, 185, 4, 64, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 254, 221, 0, 1, 0, 126, 185, 4, 0, 0, 0, 252, 255, 255, 255, 3, 0, 0, 0, 252, 255, 255, 255, 3, 0, 0, 0, 252, 255, 255, 255, 3, 0, 0, 0, 252, 255, 255, 255, 3, 254, 225, 0, 1, 0, 126, 185, 4, 1, 0, 0, 0, 255, 251, 255, 255, 1, 0, 0, 0, 255, 251, 255, 255, 1, 0, 0, 0, 255, 251, 255, 255, 1, 0, 0, 0, 255, 251, 255, 255, 254, 229, 0, 1, 0, 126, 185, 4, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 254, 233, 0, 1, 0, 126, 185, 4, 252, 255, 255, 255, 3, 0, 0, 0, 252, 255, 255, 255, 3, 0, 0, 0, 252, 255, 255, 255, 3, 0, 0, 0, 252, 255, 255, 255, 3, 0, 0, 0, 254, 237, 0, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 8, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 246, 0, 7, 0, 254, 247, 0, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 0, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 1, 0, 0, 0, 255, 255, 255, 127, 1, 0, 0, 0, 255, 255, 255, 127, 1, 0, 0, 0, 255, 255, 255, 127, 1, 0, 0, 0, 255, 255, 255, 127, 254, 26, 1, 1, 0, 126, 185, 4, 0, 128, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 254, 30, 1, 1, 0, 126, 185, 4, 0, 0, 128, 255, 255, 255, 127, 0, 0, 0, 128, 255, 255, 255, 127, 0, 0, 0, 128, 255, 255, 255, 127, 0, 0, 0, 128, 255, 255, 255, 127, 0, 254, 34, 1, 1, 0, 126, 185, 4, 1, 0, 0, 0, 127, 255, 255, 255, 1, 0, 0, 0, 127, 255, 255, 255, 1, 0, 0, 0, 127, 255, 255, 255, 1, 0, 0, 0, 127, 255, 255, 255, 254, 38, 1, 1, 0, 126, 185, 4, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 254, 42, 1, 1, 0, 126, 185, 4, 0, 248, 255, 255, 255, 7, 0, 0, 0, 248, 255, 255, 255, 7, 0, 0, 0, 248, 255, 255, 255, 7, 0, 0, 0, 248, 255, 255, 255, 7, 0, 0, 254, 46, 1, 1, 0, 126, 185, 4, 1, 0, 0, 0, 255, 255, 247, 255, 1, 0, 0, 0, 255, 255, 247, 255, 1, 0, 0, 0, 255, 255, 247, 255, 1, 0, 0, 0, 255, 255, 247, 255, 254, 50, 1, 1, 0, 126, 185, 4, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 254, 54, 1, 1, 0, 126, 185, 4, 0, 0, 0, 224, 255, 255, 255, 31, 0, 0, 0, 224, 255, 255, 255, 31, 0, 0, 0, 224, 255, 255, 255, 31, 0, 0, 0, 224, 255, 255, 255, 31, 254, 58, 1, 1, 0, 126, 185, 4, 1, 0, 0, 0, 255, 223, 255, 255, 1, 0, 0, 0, 255, 223, 255, 255, 1, 0, 0, 0, 255, 223, 255, 255, 1, 0, 0, 0, 255, 223, 255, 255, 254, 62, 1, 1, 0, 126, 185, 4, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 254, 66, 1, 1, 0, 126, 185, 4, 224, 255, 255, 255, 31, 0, 0, 0, 224, 255, 255, 255, 31, 0, 0, 0, 224, 255, 255, 255, 31, 0, 0, 0, 224, 255, 255, 255, 31, 0, 0, 0, 254, 70, 1, 1, 0, 126, 185, 4, 1, 0, 0, 0, 255, 255, 255, 253, 1, 0, 0, 0, 255, 255, 255, 253, 1, 0, 0, 0, 255, 255, 255, 253, 1, 0, 0, 0, 255, 255, 255, 253, 254, 74, 1, 1, 0, 126, 185, 4, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 254, 78, 1, 1, 0, 126, 185, 4, 0, 0, 254, 255, 255, 255, 1, 0, 0, 0, 254, 255, 255, 255, 1, 0, 0, 0, 254, 255, 255, 255, 1, 0, 0, 0, 254, 255, 255, 255, 1, 0, 254, 82, 1, 1, 0, 126, 185, 4, 1, 0, 0, 0, 253, 255, 255, 255, 1, 0, 0, 0, 253, 255, 255, 255, 1, 0, 0, 0, 253, 255, 255, 255, 1, 0, 0, 0, 253, 255, 255, 255, 254, 86, 1, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 4, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 95, 1, 7, 0, 254, 96, 1, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 0, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 128, 0, 0, 0, 0, 0, 128, 0, 128, 0, 0, 0, 0, 0, 128, 0, 128, 0, 0, 0, 0, 0, 128, 0, 128, 0, 0, 0, 0, 0, 128, 0, 185, 4, 128, 0, 0, 0, 0, 0, 128, 0, 128, 0, 0, 0, 0, 0, 128, 0, 128, 0, 0, 0, 0, 0, 128, 0, 128, 0, 0, 0, 0, 0, 128, 0, 185, 4, 128, 255, 255, 255, 255, 255, 127, 0, 128, 255, 255, 255, 255, 255, 127, 0, 128, 255, 255, 255, 255, 255, 127, 0, 128, 255, 255, 255, 255, 255, 127, 0, 185, 4, 128, 255, 255, 255, 255, 255, 127, 0, 128, 255, 255, 255, 255, 255, 127, 0, 128, 255, 255, 255, 255, 255, 127, 0, 128, 255, 255, 255, 255, 255, 127, 0, 185, 4, 1, 128, 0, 128, 254, 127, 255, 255, 1, 128, 0, 128, 254, 127, 255, 255, 1, 128, 0, 128, 254, 127, 255, 255, 1, 128, 0, 128, 254, 127, 255, 255, 185, 4, 1, 128, 0, 128, 254, 127, 255, 255, 1, 128, 0, 128, 254, 127, 255, 255, 1, 128, 0, 128, 254, 127, 255, 255, 1, 128, 0, 128, 254, 127, 255, 255, 185, 4, 1, 128, 0, 128, 255, 127, 255, 255, 1, 128, 0, 128, 255, 127, 255, 255, 1, 128, 0, 128, 255, 127, 255, 255, 1, 128, 0, 128, 255, 127, 255, 255, 185, 4, 1, 128, 0, 128, 255, 127, 255, 255, 1, 128, 0, 128, 255, 127, 255, 255, 1, 128, 0, 128, 255, 127, 255, 255, 1, 128, 0, 128, 255, 127, 255, 255, 185, 4, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 185, 4, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 185, 4, 9, 0, 248, 255, 246, 255, 255, 255, 9, 0, 248, 255, 246, 255, 255, 255, 9, 0, 248, 255, 246, 255, 255, 255, 9, 0, 248, 255, 246, 255, 255, 255, 185, 4, 9, 0, 248, 255, 246, 255, 255, 255, 9, 0, 248, 255, 246, 255, 255, 255, 9, 0, 248, 255, 246, 255, 255, 255, 9, 0, 248, 255, 246, 255, 255, 255, 185, 4, 1, 0, 0, 8, 255, 7, 0, 248, 1, 0, 0, 8, 255, 7, 0, 248, 1, 0, 0, 8, 255, 7, 0, 248, 1, 0, 0, 8, 255, 7, 0, 248, 185, 4, 1, 0, 0, 8, 255, 7, 0, 248, 1, 0, 0, 8, 255, 7, 0, 248, 1, 0, 0, 8, 255, 7, 0, 248, 1, 0, 0, 8, 255, 7, 0, 248, 185, 4, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 185, 4, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 185, 4, 1, 0, 32, 0, 223, 255, 223, 255, 1, 0, 32, 0, 223, 255, 223, 255, 1, 0, 32, 0, 223, 255, 223, 255, 1, 0, 32, 0, 223, 255, 223, 255, 185, 4, 1, 0, 32, 0, 223, 255, 223, 255, 1, 0, 32, 0, 223, 255, 223, 255, 1, 0, 32, 0, 223, 255, 223, 255, 1, 0, 32, 0, 223, 255, 223, 255, 185, 4, 1, 0, 32, 0, 31, 0, 224, 255, 1, 0, 32, 0, 31, 0, 224, 255, 1, 0, 32, 0, 31, 0, 224, 255, 1, 0, 32, 0, 31, 0, 224, 255, 185, 4, 1, 0, 32, 0, 31, 0, 224, 255, 1, 0, 32, 0, 31, 0, 224, 255, 1, 0, 32, 0, 31, 0, 224, 255, 1, 0, 32, 0, 31, 0, 224, 255, 185, 4, 0, 224, 255, 255, 255, 255, 255, 31, 0, 224, 255, 255, 255, 255, 255, 31, 0, 224, 255, 255, 255, 255, 255, 31, 0, 224, 255, 255, 255, 255, 255, 31, 185, 4, 0, 224, 255, 255, 255, 255, 255, 31, 0, 224, 255, 255, 255, 255, 255, 31, 0, 224, 255, 255, 255, 255, 255, 31, 0, 224, 255, 255, 255, 255, 255, 31, 185, 4, 1, 224, 255, 255, 254, 255, 255, 223, 1, 224, 255, 255, 254, 255, 255, 223, 1, 224, 255, 255, 254, 255, 255, 223, 1, 224, 255, 255, 254, 255, 255, 223, 185, 4, 1, 224, 255, 255, 254, 255, 255, 223, 1, 224, 255, 255, 254, 255, 255, 223, 1, 224, 255, 255, 254, 255, 255, 223, 1, 224, 255, 255, 254, 255, 255, 223, 185, 4, 2, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 2, 0, 185, 4, 2, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 2, 0, 185, 4, 254, 255, 255, 255, 255, 255, 1, 0, 254, 255, 255, 255, 255, 255, 1, 0, 254, 255, 255, 255, 255, 255, 1, 0, 254, 255, 255, 255, 255, 255, 1, 0, 185, 4, 254, 255, 255, 255, 255, 255, 1, 0, 254, 255, 255, 255, 255, 255, 1, 0, 254, 255, 255, 255, 255, 255, 1, 0, 254, 255, 255, 255, 255, 255, 1, 0, 185, 4, 1, 2, 0, 254, 254, 253, 255, 255, 1, 2, 0, 254, 254, 253, 255, 255, 1, 2, 0, 254, 254, 253, 255, 255, 1, 2, 0, 254, 254, 253, 255, 255, 185, 4, 1, 2, 0, 254, 254, 253, 255, 255, 1, 2, 0, 254, 254, 253, 255, 255, 1, 2, 0, 254, 254, 253, 255, 255, 1, 2, 0, 254, 254, 253, 255, 255, 185, 4, 1, 2, 0, 2, 255, 253, 255, 255, 1, 2, 0, 2, 255, 253, 255, 255, 1, 2, 0, 2, 255, 253, 255, 255, 1, 2, 0, 2, 255, 253, 255, 255, 185, 4, 1, 2, 0, 2, 255, 253, 255, 255, 1, 2, 0, 2, 255, 253, 255, 255, 1, 2, 0, 2, 255, 253, 255, 255, 1, 2, 0, 2, 255, 253, 255, 255, 185, 4, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 185, 4, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 185, 4, 65, 0, 192, 255, 190, 255, 255, 255, 65, 0, 192, 255, 190, 255, 255, 255, 65, 0, 192, 255, 190, 255, 255, 255, 65, 0, 192, 255, 190, 255, 255, 255, 185, 4, 65, 0, 192, 255, 190, 255, 255, 255, 65, 0, 192, 255, 190, 255, 255, 255, 65, 0, 192, 255, 190, 255, 255, 255, 65, 0, 192, 255, 190, 255, 255, 255, 185, 4, 1, 0, 0, 64, 255, 63, 0, 192, 1, 0, 0, 64, 255, 63, 0, 192, 1, 0, 0, 64, 255, 63, 0, 192, 1, 0, 0, 64, 255, 63, 0, 192, 185, 4, 1, 0, 0, 64, 255, 63, 0, 192, 1, 0, 0, 64, 255, 63, 0, 192, 1, 0, 0, 64, 255, 63, 0, 192, 1, 0, 0, 64, 255, 63, 0, 192, 185, 4, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 185, 4, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 185, 4, 1, 0, 4, 0, 251, 255, 251, 255, 1, 0, 4, 0, 251, 255, 251, 255, 1, 0, 4, 0, 251, 255, 251, 255, 1, 0, 4, 0, 251, 255, 251, 255, 185, 4, 1, 0, 4, 0, 251, 255, 251, 255, 1, 0, 4, 0, 251, 255, 251, 255, 1, 0, 4, 0, 251, 255, 251, 255, 1, 0, 4, 0, 251, 255, 251, 255, 185, 4, 1, 0, 4, 0, 3, 0, 252, 255, 1, 0, 4, 0, 3, 0, 252, 255, 1, 0, 4, 0, 3, 0, 252, 255, 1, 0, 4, 0, 3, 0, 252, 255, 185, 4, 1, 0, 4, 0, 3, 0, 252, 255, 1, 0, 4, 0, 3, 0, 252, 255, 1, 0, 4, 0, 3, 0, 252, 255, 1, 0, 4, 0, 3, 0, 252, 255, 185, 4, 0, 252, 255, 255, 255, 255, 255, 3, 0, 252, 255, 255, 255, 255, 255, 3, 0, 252, 255, 255, 255, 255, 255, 3, 0, 252, 255, 255, 255, 255, 255, 3, 185, 4, 0, 252, 255, 255, 255, 255, 255, 3, 0, 252, 255, 255, 255, 255, 255, 3, 0, 252, 255, 255, 255, 255, 255, 3, 0, 252, 255, 255, 255, 255, 255, 3, 185, 4, 1, 252, 255, 255, 254, 255, 255, 251, 1, 252, 255, 255, 254, 255, 255, 251, 1, 252, 255, 255, 254, 255, 255, 251, 1, 252, 255, 255, 254, 255, 255, 251, 185, 4, 1, 252, 255, 255, 254, 255, 255, 251, 1, 252, 255, 255, 254, 255, 255, 251, 1, 252, 255, 255, 254, 255, 255, 251, 1, 252, 255, 255, 254, 255, 255, 251, 185, 4, 16, 0, 0, 0, 0, 0, 16, 0, 16, 0, 0, 0, 0, 0, 16, 0, 16, 0, 0, 0, 0, 0, 16, 0, 16, 0, 0, 0, 0, 0, 16, 0, 185, 4, 16, 0, 0, 0, 0, 0, 16, 0, 16, 0, 0, 0, 0, 0, 16, 0, 16, 0, 0, 0, 0, 0, 16, 0, 16, 0, 0, 0, 0, 0, 16, 0, 185, 4, 240, 255, 255, 255, 255, 255, 15, 0, 240, 255, 255, 255, 255, 255, 15, 0, 240, 255, 255, 255, 255, 255, 15, 0, 240, 255, 255, 255, 255, 255, 15, 0, 185, 4, 240, 255, 255, 255, 255, 255, 15, 0, 240, 255, 255, 255, 255, 255, 15, 0, 240, 255, 255, 255, 255, 255, 15, 0, 240, 255, 255, 255, 255, 255, 15, 0, 185, 4, 1, 16, 0, 240, 254, 239, 255, 255, 1, 16, 0, 240, 254, 239, 255, 255, 1, 16, 0, 240, 254, 239, 255, 255, 1, 16, 0, 240, 254, 239, 255, 255, 185, 4, 1, 16, 0, 240, 254, 239, 255, 255, 1, 16, 0, 240, 254, 239, 255, 255, 1, 16, 0, 240, 254, 239, 255, 255, 1, 16, 0, 240, 254, 239, 255, 255, 185, 4, 1, 16, 0, 16, 255, 239, 255, 255, 1, 16, 0, 16, 255, 239, 255, 255, 1, 16, 0, 16, 255, 239, 255, 255, 1, 16, 0, 16, 255, 239, 255, 255, 185, 4, 1, 16, 0, 16, 255, 239, 255, 255, 1, 16, 0, 16, 255, 239, 255, 255, 1, 16, 0, 16, 255, 239, 255, 255, 1, 16, 0, 16, 255, 239, 255, 255, 185, 4, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 185, 4, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 185, 4, 2, 0, 255, 255, 253, 255, 255, 255, 2, 0, 255, 255, 253, 255, 255, 255, 2, 0, 255, 255, 253, 255, 255, 255, 2, 0, 255, 255, 253, 255, 255, 255, 185, 4, 2, 0, 255, 255, 253, 255, 255, 255, 2, 0, 255, 255, 253, 255, 255, 255, 2, 0, 255, 255, 253, 255, 255, 255, 2, 0, 255, 255, 253, 255, 255, 255, 185, 4, 1, 0, 0, 1, 255, 0, 0, 255, 1, 0, 0, 1, 255, 0, 0, 255, 1, 0, 0, 1, 255, 0, 0, 255, 1, 0, 0, 1, 255, 0, 0, 255, 185, 4, 1, 0, 0, 1, 255, 0, 0, 255, 1, 0, 0, 1, 255, 0, 0, 255, 1, 0, 0, 1, 255, 0, 0, 255, 1, 0, 0, 1, 255, 0, 0, 255, 185, 4, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 185, 4, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 2, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 200, 1, 7, 0, 254, 201, 1, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 0, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 112, 97, 105, 15, 213, 124, 70, 151, 112, 97, 105, 15, 213, 124, 70, 151, 112, 97, 105, 15, 213, 124, 70, 151, 112, 97, 105, 15, 213, 124, 70, 151, 185, 4, 81, 89, 42, 131, 104, 15, 69, 222, 81, 89, 42, 131, 104, 15, 69, 222, 81, 89, 42, 131, 104, 15, 69, 222, 81, 89, 42, 131, 104, 15, 69, 222, 185, 4, 125, 70, 151, 144, 33, 80, 89, 42, 125, 70, 151, 144, 33, 80, 89, 42, 125, 70, 151, 144, 33, 80, 89, 42, 125, 70, 151, 144, 33, 80, 89, 42, 185, 4, 16, 69, 222, 175, 150, 144, 158, 150, 16, 69, 222, 175, 150, 144, 158, 150, 16, 69, 222, 175, 150, 144, 158, 150, 16, 69, 222, 175, 150, 144, 158, 150, 185, 4, 28, 2, 149, 165, 22, 150, 246, 80, 28, 2, 149, 165, 22, 150, 246, 80, 28, 2, 149, 165, 22, 150, 246, 80, 28, 2, 149, 165, 22, 150, 246, 80, 185, 4, 117, 9, 233, 105, 148, 165, 50, 152, 117, 9, 233, 105, 148, 165, 50, 152, 117, 9, 233, 105, 148, 165, 50, 152, 117, 9, 233, 105, 148, 165, 50, 152, 185, 4, 151, 246, 80, 228, 102, 116, 9, 233, 151, 246, 80, 228, 102, 116, 9, 233, 151, 246, 80, 228, 102, 116, 9, 233, 151, 246, 80, 228, 102, 116, 9, 233, 185, 4, 166, 50, 152, 139, 80, 228, 253, 106, 166, 50, 152, 139, 80, 228, 253, 106, 166, 50, 152, 139, 80, 228, 253, 106, 166, 50, 152, 139, 80, 228, 253, 106, 185, 4, 54, 159, 209, 37, 110, 8, 84, 150, 54, 159, 209, 37, 110, 8, 84, 150, 54, 159, 209, 37, 110, 8, 84, 150, 54, 159, 209, 37, 110, 8, 84, 150, 185, 4, 219, 67, 145, 247, 208, 37, 164, 167, 219, 67, 145, 247, 208, 37, 164, 167, 219, 67, 145, 247, 208, 37, 164, 167, 219, 67, 145, 247, 208, 37, 164, 167, 185, 4, 9, 84, 150, 202, 87, 218, 67, 145, 9, 84, 150, 202, 87, 218, 67, 145, 9, 84, 150, 202, 87, 218, 67, 145, 9, 84, 150, 202, 87, 218, 67, 145, 185, 4, 38, 164, 167, 37, 150, 202, 96, 46, 38, 164, 167, 37, 150, 202, 96, 46, 38, 164, 167, 37, 150, 202, 96, 46, 38, 164, 167, 37, 150, 202, 96, 46, 185, 4, 134, 165, 61, 20, 243, 25, 93, 66, 134, 165, 61, 20, 243, 25, 93, 66, 134, 165, 61, 20, 243, 25, 93, 66, 134, 165, 61, 20, 243, 25, 93, 66, 185, 4, 102, 169, 12, 230, 60, 20, 121, 191, 102, 169, 12, 230, 60, 20, 121, 191, 102, 169, 12, 230, 60, 20, 121, 191, 102, 169, 12, 230, 60, 20, 121, 191, 185, 4, 26, 93, 66, 122, 64, 101, 169, 12, 26, 93, 66, 122, 64, 101, 169, 12, 26, 93, 66, 122, 64, 101, 169, 12, 26, 93, 66, 122, 64, 101, 169, 12, 185, 4, 21, 121, 191, 154, 65, 122, 90, 194, 21, 121, 191, 154, 65, 122, 90, 194, 21, 121, 191, 154, 65, 122, 90, 194, 21, 121, 191, 154, 65, 122, 90, 194, 185, 4, 222, 16, 168, 44, 183, 176, 180, 135, 222, 16, 168, 44, 183, 176, 180, 135, 222, 16, 168, 44, 183, 176, 180, 135, 222, 16, 168, 44, 183, 176, 180, 135, 185, 4, 164, 75, 72, 79, 167, 44, 149, 193, 164, 75, 72, 79, 167, 44, 149, 193, 164, 75, 72, 79, 167, 44, 149, 193, 164, 75, 72, 79, 167, 44, 149, 193, 185, 4, 177, 180, 135, 34, 62, 163, 75, 72, 177, 180, 135, 34, 62, 163, 75, 72, 177, 180, 135, 34, 62, 163, 75, 72, 177, 180, 135, 34, 62, 163, 75, 72, 185, 4, 45, 149, 193, 92, 135, 34, 239, 87, 45, 149, 193, 92, 135, 34, 239, 87, 45, 149, 193, 92, 135, 34, 239, 87, 45, 149, 193, 92, 135, 34, 239, 87, 185, 4, 231, 51, 186, 132, 13, 129, 202, 82, 231, 51, 186, 132, 13, 129, 202, 82, 231, 51, 186, 132, 13, 129, 202, 82, 231, 51, 186, 132, 13, 129, 202, 82, 185, 4, 124, 40, 242, 126, 185, 132, 244, 180, 124, 40, 242, 126, 185, 132, 244, 180, 124, 40, 242, 126, 185, 132, 244, 180, 124, 40, 242, 126, 185, 132, 244, 180, 185, 4, 130, 202, 82, 25, 74, 123, 40, 242, 130, 202, 82, 25, 74, 123, 40, 242, 130, 202, 82, 25, 74, 123, 40, 242, 130, 202, 82, 25, 74, 123, 40, 242, 185, 4, 133, 244, 180, 132, 82, 25, 204, 69, 133, 244, 180, 132, 82, 25, 204, 69, 133, 244, 180, 132, 82, 25, 204, 69, 133, 244, 180, 132, 82, 25, 204, 69, 185, 4, 46, 44, 237, 161, 154, 207, 232, 18, 46, 44, 237, 161, 154, 207, 232, 18, 46, 44, 237, 161, 154, 207, 232, 18, 46, 44, 237, 161, 154, 207, 232, 18, 185, 4, 43, 75, 101, 48, 236, 161, 200, 251, 43, 75, 101, 48, 236, 161, 200, 251, 43, 75, 101, 48, 236, 161, 200, 251, 43, 75, 101, 48, 236, 161, 200, 251, 185, 4, 208, 232, 18, 210, 3, 42, 75, 101, 208, 232, 18, 210, 3, 42, 75, 101, 208, 232, 18, 210, 3, 42, 75, 101, 208, 232, 18, 210, 3, 42, 75, 101, 185, 4, 162, 200, 251, 213, 18, 210, 211, 18, 162, 200, 251, 213, 18, 210, 211, 18, 162, 200, 251, 213, 18, 210, 211, 18, 162, 200, 251, 213, 18, 210, 211, 18, 185, 4, 68, 160, 178, 84, 194, 210, 30, 138, 68, 160, 178, 84, 194, 210, 30, 138, 68, 160, 178, 84, 194, 210, 30, 138, 68, 160, 178, 84, 194, 210, 30, 138, 185, 4, 47, 33, 61, 45, 178, 84, 6, 115, 47, 33, 61, 45, 178, 84, 6, 115, 47, 33, 61, 45, 178, 84, 6, 115, 47, 33, 61, 45, 178, 84, 6, 115, 185, 4, 211, 30, 138, 188, 140, 46, 33, 61, 211, 30, 138, 188, 140, 46, 33, 61, 211, 30, 138, 188, 140, 46, 33, 61, 211, 30, 138, 188, 140, 46, 33, 61, 185, 4, 85, 6, 115, 209, 137, 188, 95, 77, 85, 6, 115, 209, 137, 188, 95, 77, 85, 6, 115, 209, 137, 188, 95, 77, 85, 6, 115, 209, 137, 188, 95, 77, 185, 4, 59, 160, 34, 182, 123, 224, 57, 58, 59, 160, 34, 182, 123, 224, 57, 58, 59, 160, 34, 182, 123, 224, 57, 58, 59, 160, 34, 182, 123, 224, 57, 58, 185, 4, 164, 15, 132, 31, 34, 182, 182, 128, 164, 15, 132, 31, 34, 182, 182, 128, 164, 15, 132, 31, 34, 182, 182, 128, 164, 15, 132, 31, 34, 182, 182, 128, 185, 4, 225, 57, 58, 197, 126, 163, 15, 132, 225, 57, 58, 197, 126, 163, 15, 132, 225, 57, 58, 197, 126, 163, 15, 132, 225, 57, 58, 197, 126, 163, 15, 132, 185, 4, 183, 182, 128, 92, 57, 197, 95, 221, 183, 182, 128, 92, 57, 197, 95, 221, 183, 182, 128, 92, 57, 197, 95, 221, 183, 182, 128, 92, 57, 197, 95, 221, 185, 4, 245, 55, 250, 64, 3, 42, 98, 107, 245, 55, 250, 64, 3, 42, 98, 107, 245, 55, 250, 64, 3, 42, 98, 107, 245, 55, 250, 64, 3, 42, 98, 107, 185, 4, 164, 83, 252, 213, 249, 64, 248, 97, 164, 83, 252, 213, 249, 64, 248, 97, 164, 83, 252, 213, 249, 64, 248, 97, 164, 83, 252, 213, 249, 64, 248, 97, 185, 4, 43, 98, 107, 11, 157, 163, 83, 252, 43, 98, 107, 11, 157, 163, 83, 252, 43, 98, 107, 11, 157, 163, 83, 252, 43, 98, 107, 11, 157, 163, 83, 252, 185, 4, 65, 248, 97, 92, 107, 11, 200, 5, 65, 248, 97, 92, 107, 11, 200, 5, 65, 248, 97, 92, 107, 11, 200, 5, 65, 248, 97, 92, 107, 11, 200, 5, 185, 4, 31, 120, 142, 78, 210, 223, 232, 3, 31, 120, 142, 78, 210, 223, 232, 3, 31, 120, 142, 78, 210, 223, 232, 3, 31, 120, 142, 78, 210, 223, 232, 3, 185, 4, 137, 173, 45, 32, 142, 78, 241, 87, 137, 173, 45, 32, 142, 78, 241, 87, 137, 173, 45, 32, 142, 78, 241, 87, 137, 173, 45, 32, 142, 78, 241, 87, 185, 4, 224, 232, 3, 225, 167, 136, 173, 45, 224, 232, 3, 225, 167, 136, 173, 45, 224, 232, 3, 225, 167, 136, 173, 45, 224, 232, 3, 225, 167, 136, 173, 45, 185, 4, 79, 241, 87, 119, 3, 225, 135, 113, 79, 241, 87, 119, 3, 225, 135, 113, 79, 241, 87, 119, 3, 225, 135, 113, 79, 241, 87, 119, 3, 225, 135, 113, 185, 4, 129, 138, 216, 218, 129, 231, 232, 20, 129, 138, 216, 218, 129, 231, 232, 20, 129, 138, 216, 218, 129, 231, 232, 20, 129, 138, 216, 218, 129, 231, 232, 20, 185, 4, 63, 16, 126, 24, 216, 218, 2, 114, 63, 16, 126, 24, 216, 218, 2, 114, 63, 16, 126, 24, 216, 218, 2, 114, 63, 16, 126, 24, 216, 218, 2, 114, 185, 4, 232, 232, 20, 127, 141, 62, 16, 126, 232, 232, 20, 127, 141, 62, 16, 126, 232, 232, 20, 127, 141, 62, 16, 126, 232, 232, 20, 127, 141, 62, 16, 126, 185, 4, 219, 2, 114, 193, 20, 127, 117, 39, 219, 2, 114, 193, 20, 127, 117, 39, 219, 2, 114, 193, 20, 127, 117, 39, 219, 2, 114, 193, 20, 127, 117, 39, 185, 4, 165, 191, 209, 7, 29, 80, 17, 91, 165, 191, 209, 7, 29, 80, 17, 91, 165, 191, 209, 7, 29, 80, 17, 91, 165, 191, 209, 7, 29, 80, 17, 91, 185, 4, 29, 157, 226, 175, 209, 7, 194, 15, 29, 157, 226, 175, 209, 7, 194, 15, 29, 157, 226, 175, 209, 7, 194, 15, 29, 157, 226, 175, 209, 7, 194, 15, 185, 4, 81, 17, 91, 91, 239, 28, 157, 226, 81, 17, 91, 91, 239, 28, 157, 226, 81, 17, 91, 91, 239, 28, 157, 226, 81, 17, 91, 91, 239, 28, 157, 226, 185, 4, 8, 194, 15, 227, 90, 91, 64, 46, 8, 194, 15, 227, 90, 91, 64, 46, 8, 194, 15, 227, 90, 91, 64, 46, 8, 194, 15, 227, 90, 91, 64, 46, 185, 4, 4, 207, 209, 41, 250, 27, 125, 32, 4, 207, 209, 41, 250, 27, 125, 32, 4, 207, 209, 41, 250, 27, 125, 32, 4, 207, 209, 41, 250, 27, 125, 32, 185, 4, 178, 181, 5, 228, 208, 41, 254, 234, 178, 181, 5, 228, 208, 41, 254, 234, 178, 181, 5, 228, 208, 41, 254, 234, 178, 181, 5, 228, 208, 41, 254, 234, 185, 4, 28, 125, 32, 252, 20, 177, 181, 5, 28, 125, 32, 252, 20, 177, 181, 5, 28, 125, 32, 252, 20, 177, 181, 5, 28, 125, 32, 252, 20, 177, 181, 5, 185, 4, 42, 254, 234, 78, 32, 252, 48, 46, 42, 254, 234, 78, 32, 252, 48, 46, 42, 254, 234, 78, 32, 252, 48, 46, 42, 254, 234, 78, 32, 252, 48, 46, 185, 4, 8, 84, 196, 214, 14, 60, 71, 167, 8, 84, 196, 214, 14, 60, 71, 167, 8, 84, 196, 214, 14, 60, 71, 167, 8, 84, 196, 214, 14, 60, 71, 167, 185, 4, 245, 129, 240, 195, 195, 214, 22, 144, 245, 129, 240, 195, 195, 214, 22, 144, 245, 129, 240, 195, 195, 214, 22, 144, 245, 129, 240, 195, 195, 214, 22, 144, 185, 4, 61, 71, 167, 248, 110, 244, 129, 240, 61, 71, 167, 248, 110, 244, 129, 240, 61, 71, 167, 248, 110, 244, 129, 240, 61, 71, 167, 248, 110, 244, 129, 240, 185, 4, 215, 22, 144, 11, 167, 248, 171, 59, 215, 22, 144, 11, 167, 248, 171, 59, 215, 22, 144, 11, 167, 248, 171, 59, 215, 22, 144, 11, 167, 248, 171, 59, 185, 4, 255, 70, 31, 8, 64, 69, 108, 109, 255, 70, 31, 8, 64, 69, 108, 109, 255, 70, 31, 8, 64, 69, 108, 109, 255, 70, 31, 8, 64, 69, 108, 109, 185, 4, 117, 138, 191, 186, 30, 8, 63, 140, 117, 138, 191, 186, 30, 8, 63, 140, 117, 138, 191, 186, 30, 8, 63, 140, 117, 138, 191, 186, 30, 8, 63, 140, 185, 4, 70, 108, 109, 1, 115, 116, 138, 191, 70, 108, 109, 1, 115, 116, 138, 191, 70, 108, 109, 1, 115, 116, 138, 191, 70, 108, 109, 1, 115, 116, 138, 191, 185, 4, 9, 63, 140, 139, 108, 1, 185, 224, 9, 63, 140, 139, 108, 1, 185, 224, 9, 63, 140, 139, 108, 1, 185, 224, 9, 63, 140, 139, 108, 1, 185, 224, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 1, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 49, 2, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 0, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 130, 46, 200, 144, 131, 200, 85, 172, 130, 46, 200, 144, 131, 200, 85, 172, 227, 194, 123, 55, 199, 144, 5, 247, 227, 194, 123, 55, 199, 144, 5, 247, 185, 4, 201, 85, 172, 126, 8, 226, 194, 123, 201, 85, 172, 126, 8, 226, 194, 123, 145, 5, 247, 29, 172, 126, 209, 55, 145, 5, 247, 29, 172, 126, 209, 55, 185, 4, 144, 32, 46, 188, 231, 130, 12, 89, 144, 32, 46, 188, 231, 130, 12, 89, 198, 234, 23, 125, 45, 188, 119, 163, 198, 234, 23, 125, 45, 188, 119, 163, 185, 4, 131, 12, 89, 112, 92, 197, 234, 23, 131, 12, 89, 112, 92, 197, 234, 23, 189, 119, 163, 58, 88, 112, 223, 209, 189, 119, 163, 58, 88, 112, 223, 209, 185, 4, 34, 114, 21, 171, 61, 130, 184, 240, 34, 114, 21, 171, 61, 130, 184, 240, 51, 100, 193, 125, 20, 171, 95, 244, 51, 100, 193, 125, 20, 171, 95, 244, 185, 4, 131, 184, 240, 222, 10, 50, 100, 193, 131, 184, 240, 222, 10, 50, 100, 193, 172, 95, 244, 205, 239, 222, 141, 234, 172, 95, 244, 205, 239, 222, 141, 234, 185, 4, 187, 32, 67, 22, 33, 87, 177, 250, 187, 32, 67, 22, 33, 87, 177, 250, 12, 239, 221, 168, 66, 22, 220, 119, 12, 239, 221, 168, 66, 22, 220, 119, 185, 4, 88, 177, 250, 69, 135, 11, 239, 221, 88, 177, 250, 69, 135, 11, 239, 221, 23, 220, 119, 244, 249, 69, 223, 188, 23, 220, 119, 244, 249, 69, 223, 188, 185, 4, 126, 4, 113, 225, 63, 23, 100, 200, 126, 4, 113, 225, 63, 23, 100, 200, 43, 86, 191, 232, 112, 225, 189, 27, 43, 86, 191, 232, 112, 225, 189, 27, 185, 4, 24, 100, 200, 130, 227, 42, 86, 191, 24, 100, 200, 130, 227, 42, 86, 191, 226, 189, 27, 213, 199, 130, 251, 142, 226, 189, 27, 213, 199, 130, 251, 142, 185, 4, 69, 174, 98, 245, 70, 16, 23, 222, 69, 174, 98, 245, 70, 16, 23, 222, 135, 44, 184, 239, 97, 245, 139, 190, 135, 44, 184, 239, 97, 245, 139, 190, 185, 4, 17, 23, 222, 187, 64, 134, 44, 184, 17, 23, 222, 187, 64, 134, 44, 184, 246, 139, 190, 121, 221, 187, 81, 157, 246, 139, 190, 121, 221, 187, 81, 157, 185, 4, 209, 5, 25, 178, 15, 185, 138, 213, 209, 5, 25, 178, 15, 185, 138, 213, 93, 120, 239, 70, 24, 178, 224, 190, 93, 120, 239, 70, 24, 178, 224, 190, 185, 4, 186, 138, 213, 47, 64, 92, 120, 239, 186, 138, 213, 47, 64, 92, 120, 239, 179, 224, 190, 163, 212, 47, 250, 230, 179, 224, 190, 163, 212, 47, 250, 230, 185, 4, 18, 196, 133, 247, 92, 144, 33, 11, 18, 196, 133, 247, 92, 144, 33, 11, 89, 253, 162, 111, 133, 247, 110, 84, 89, 253, 162, 111, 133, 247, 110, 84, 185, 4, 145, 33, 11, 238, 170, 88, 253, 162, 145, 33, 11, 238, 170, 88, 253, 162, 248, 110, 84, 167, 10, 238, 59, 122, 248, 110, 84, 167, 10, 238, 59, 122, 185, 4, 82, 78, 7, 222, 201, 89, 83, 82, 82, 78, 7, 222, 201, 89, 83, 82, 166, 207, 53, 166, 6, 222, 27, 168, 166, 207, 53, 166, 6, 222, 27, 168, 185, 4, 90, 83, 82, 174, 87, 165, 207, 53, 90, 83, 82, 174, 87, 165, 207, 53, 223, 27, 168, 90, 81, 174, 177, 248, 223, 27, 168, 90, 81, 174, 177, 248, 185, 4, 127, 85, 250, 92, 228, 116, 224, 189, 127, 85, 250, 92, 228, 116, 224, 189, 38, 229, 26, 139, 249, 92, 99, 202, 38, 229, 26, 139, 249, 92, 99, 202, 185, 4, 117, 224, 189, 129, 53, 37, 229, 26, 117, 224, 189, 129, 53, 37, 229, 26, 93, 99, 202, 218, 189, 129, 170, 5, 93, 99, 202, 218, 189, 129, 170, 5, 185, 4, 115, 214, 148, 148, 248, 85, 233, 115, 115, 214, 148, 148, 248, 85, 233, 115, 130, 247, 6, 170, 148, 148, 107, 44, 130, 247, 6, 170, 148, 148, 107, 44, 185, 4, 86, 233, 115, 141, 211, 129, 247, 6, 86, 233, 115, 141, 211, 129, 247, 6, 149, 107, 44, 126, 115, 141, 41, 107, 149, 107, 44, 126, 115, 141, 41, 107, 185, 4, 58, 29, 120, 111, 102, 77, 73, 185, 58, 29, 120, 111, 102, 77, 73, 185, 63, 215, 152, 178, 119, 111, 160, 106, 63, 215, 152, 178, 119, 111, 160, 106, 185, 4, 78, 73, 185, 198, 148, 62, 215, 152, 78, 73, 185, 198, 148, 62, 215, 152, 112, 160, 106, 193, 184, 198, 226, 135, 112, 160, 106, 193, 184, 198, 226, 135, 185, 4, 243, 171, 210, 231, 39, 167, 3, 239, 243, 171, 210, 231, 39, 167, 3, 239, 42, 41, 215, 88, 210, 231, 26, 83, 42, 41, 215, 88, 210, 231, 26, 83, 185, 4, 168, 3, 239, 13, 172, 41, 41, 215, 168, 3, 239, 13, 172, 41, 41, 215, 232, 26, 83, 214, 238, 13, 84, 45, 232, 26, 83, 214, 238, 13, 84, 45, 185, 4, 207, 154, 146, 114, 190, 42, 125, 174, 207, 154, 146, 114, 190, 42, 125, 174, 241, 222, 64, 213, 145, 114, 141, 197, 241, 222, 64, 213, 145, 114, 141, 197, 185, 4, 43, 125, 174, 49, 58, 240, 222, 64, 43, 125, 174, 49, 58, 240, 222, 64, 115, 141, 197, 15, 174, 49, 101, 109, 115, 141, 197, 15, 174, 49, 101, 109, 185, 4, 203, 233, 192, 123, 56, 107, 74, 202, 203, 233, 192, 123, 56, 107, 74, 202, 245, 185, 198, 148, 192, 123, 3, 85, 245, 185, 198, 148, 192, 123, 3, 85, 185, 4, 108, 74, 202, 53, 170, 244, 185, 198, 108, 74, 202, 53, 170, 244, 185, 198, 124, 3, 85, 11, 202, 53, 22, 63, 124, 3, 85, 11, 202, 53, 22, 63, 185, 4, 176, 74, 159, 107, 156, 14, 188, 55, 176, 74, 159, 107, 156, 14, 188, 55, 165, 92, 99, 241, 158, 107, 76, 89, 165, 92, 99, 241, 158, 107, 76, 89, 185, 4, 15, 188, 55, 80, 166, 164, 92, 99, 15, 188, 55, 80, 166, 164, 92, 99, 108, 76, 89, 91, 55, 80, 181, 96, 108, 76, 89, 91, 55, 80, 181, 96, 185, 4, 47, 163, 143, 236, 74, 116, 43, 234, 47, 163, 143, 236, 74, 116, 43, 234, 69, 41, 180, 139, 143, 236, 121, 23, 69, 41, 180, 139, 143, 236, 121, 23, 185, 4, 117, 43, 234, 209, 231, 68, 41, 180, 117, 43, 234, 209, 231, 68, 41, 180, 237, 121, 23, 187, 233, 209, 92, 112, 237, 121, 23, 187, 233, 209, 92, 112, 185, 4, 137, 78, 148, 66, 50, 250, 200, 158, 137, 78, 148, 66, 50, 250, 200, 158, 163, 30, 205, 5, 148, 66, 187, 72, 163, 30, 205, 5, 148, 66, 187, 72, 185, 4, 251, 200, 158, 119, 182, 162, 30, 205, 251, 200, 158, 119, 182, 162, 30, 205, 67, 187, 72, 93, 158, 119, 177, 107, 67, 187, 72, 93, 158, 119, 177, 107, 185, 4, 19, 221, 138, 122, 33, 58, 81, 10, 19, 221, 138, 122, 33, 58, 81, 10, 36, 123, 222, 197, 138, 122, 52, 23, 36, 123, 222, 197, 138, 122, 52, 23, 185, 4, 59, 81, 10, 237, 231, 35, 123, 222, 59, 81, 10, 237, 231, 35, 123, 222, 123, 52, 23, 220, 9, 237, 34, 117, 123, 52, 23, 220, 9, 237, 34, 117, 185, 4, 141, 62, 178, 231, 208, 173, 168, 71, 141, 62, 178, 231, 208, 173, 168, 71, 166, 208, 46, 82, 177, 231, 93, 236, 166, 208, 46, 82, 177, 231, 93, 236, 185, 4, 174, 168, 71, 115, 19, 165, 208, 46, 174, 168, 71, 115, 19, 165, 208, 46, 232, 93, 236, 90, 71, 115, 193, 77, 232, 93, 236, 90, 71, 115, 193, 77, 185, 4, 68, 116, 162, 20, 150, 209, 71, 246, 68, 116, 162, 20, 150, 209, 71, 246, 22, 245, 104, 46, 162, 20, 218, 69, 22, 245, 104, 46, 162, 20, 218, 69, 185, 4, 210, 71, 246, 188, 185, 21, 245, 104, 210, 71, 246, 188, 185, 21, 245, 104, 21, 218, 69, 234, 245, 188, 139, 93, 21, 218, 69, 234, 245, 188, 139, 93, 185, 4, 163, 91, 81, 143, 67, 39, 74, 161, 163, 91, 81, 143, 67, 39, 74, 161, 101, 207, 187, 216, 80, 143, 230, 130, 101, 207, 187, 216, 80, 143, 230, 130, 185, 4, 40, 74, 161, 93, 124, 100, 207, 187, 40, 74, 161, 93, 124, 100, 207, 187, 144, 230, 130, 155, 160, 93, 164, 174, 144, 230, 130, 155, 160, 93, 164, 174, 185, 4, 102, 244, 145, 61, 137, 110, 69, 61, 102, 244, 145, 61, 137, 110, 69, 61, 41, 133, 118, 145, 145, 61, 239, 98, 41, 133, 118, 145, 145, 61, 239, 98, 185, 4, 111, 69, 61, 154, 156, 40, 133, 118, 111, 69, 61, 154, 156, 40, 133, 118, 62, 239, 98, 215, 60, 154, 11, 110, 62, 239, 98, 215, 60, 154, 11, 110, 185, 4, 210, 137, 82, 104, 69, 31, 217, 243, 210, 137, 82, 104, 69, 31, 217, 243, 213, 163, 185, 224, 81, 104, 23, 169, 213, 163, 185, 224, 81, 104, 23, 169, 185, 4, 32, 217, 243, 46, 86, 212, 163, 185, 32, 217, 243, 46, 86, 212, 163, 185, 105, 23, 169, 43, 243, 46, 118, 173, 105, 23, 169, 43, 243, 46, 118, 173, 185, 4, 129, 198, 25, 222, 66, 146, 79, 45, 129, 198, 25, 222, 66, 146, 79, 45, 151, 244, 188, 109, 25, 222, 195, 88, 151, 244, 188, 109, 25, 222, 195, 88, 185, 4, 147, 79, 45, 127, 166, 150, 244, 188, 147, 79, 45, 127, 166, 150, 244, 188, 223, 195, 88, 105, 44, 127, 57, 230, 223, 195, 88, 105, 44, 127, 57, 230, 185, 4, 116, 106, 73, 207, 103, 156, 225, 61, 116, 106, 73, 207, 103, 156, 225, 61, 213, 242, 151, 99, 73, 207, 219, 6, 213, 242, 151, 99, 73, 207, 219, 6, 185, 4, 157, 225, 61, 140, 248, 212, 242, 151, 157, 225, 61, 140, 248, 212, 242, 151, 208, 219, 6, 43, 61, 140, 149, 182, 208, 219, 6, 43, 61, 140, 149, 182, 185, 4, 145, 228, 83, 203, 206, 169, 37, 61, 145, 228, 83, 203, 206, 169, 37, 61, 135, 247, 48, 86, 83, 203, 95, 142, 135, 247, 48, 86, 83, 203, 95, 142, 185, 4, 170, 37, 61, 111, 113, 134, 247, 48, 170, 37, 61, 111, 113, 134, 247, 48, 204, 95, 142, 121, 60, 111, 27, 172, 204, 95, 142, 121, 60, 111, 27, 172, 185, 4, 27, 103, 120, 15, 72, 62, 181, 252, 27, 103, 120, 15, 72, 62, 181, 252, 211, 243, 182, 193, 119, 15, 99, 165, 211, 243, 182, 193, 119, 15, 99, 165, 185, 4, 63, 181, 252, 229, 89, 210, 243, 182, 63, 181, 252, 229, 89, 210, 243, 182, 16, 99, 165, 45, 252, 229, 152, 135, 16, 99, 165, 45, 252, 229, 152, 135, 185, 4, 159, 83, 75, 122, 63, 227, 12, 239, 159, 83, 75, 122, 63, 227, 12, 239, 168, 150, 191, 28, 75, 122, 222, 54, 168, 150, 191, 28, 75, 122, 222, 54, 185, 4, 228, 12, 239, 97, 200, 167, 150, 191, 228, 12, 239, 97, 200, 167, 150, 191, 123, 222, 54, 88, 238, 97, 172, 180, 123, 222, 54, 88, 238, 97, 172, 180, 185, 4, 147, 124, 106, 249, 56, 181, 164, 231, 147, 124, 106, 249, 56, 181, 164, 231, 241, 30, 198, 74, 106, 249, 203, 49, 241, 30, 198, 74, 106, 249, 203, 49, 185, 4, 182, 164, 231, 109, 205, 240, 30, 198, 182, 164, 231, 109, 205, 240, 30, 198, 250, 203, 49, 15, 231, 109, 131, 149, 250, 203, 49, 15, 231, 109, 131, 149, 185, 4, 209, 56, 195, 123, 71, 242, 169, 229, 209, 56, 195, 123, 71, 242, 169, 229, 147, 158, 183, 13, 195, 123, 24, 43, 147, 158, 183, 13, 195, 123, 24, 43, 185, 4, 243, 169, 229, 47, 212, 146, 158, 183, 243, 169, 229, 47, 212, 146, 158, 183, 124, 24, 43, 109, 229, 47, 199, 60, 124, 24, 43, 109, 229, 47, 199, 60, 185, 4, 79, 45, 233, 121, 140, 51, 188, 135, 79, 45, 233, 121, 140, 51, 188, 135, 91, 254, 114, 204, 232, 121, 219, 96, 91, 254, 114, 204, 232, 121, 219, 96, 185, 4, 52, 188, 135, 177, 158, 90, 254, 114, 52, 188, 135, 177, 158, 90, 254, 114, 122, 219, 96, 165, 135, 177, 210, 22, 122, 219, 96, 165, 135, 177, 210, 22, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 1, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 146, 2, 29, 0, 118, 191, 145, 119, 191, 168, 168, 154, 154, 160, 160, 160, 160, 211, 0, 0, 168, 168, 154, 154, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 43, 219, 179, 40, 232, 196, 107, 252, 225, 218, 22, 59, 179, 40, 19, 160, 197, 107, 252, 213, 95, 224, 218, 22, 41, 19, 160, 31, 252, 213, 36, 76, 185, 4, 255, 5, 174, 109, 178, 61, 139, 50, 199, 95, 77, 194, 173, 109, 177, 67, 62, 139, 50, 1, 188, 198, 95, 77, 110, 177, 67, 57, 50, 1, 250, 81, 185, 4, 59, 241, 26, 127, 250, 23, 184, 182, 45, 202, 4, 232, 26, 127, 53, 9, 24, 184, 182, 197, 246, 44, 202, 4, 128, 53, 9, 211, 181, 197, 14, 229, 185, 4, 109, 207, 162, 76, 19, 175, 241, 87, 108, 91, 236, 80, 162, 76, 128, 126, 176, 241, 87, 147, 128, 107, 91, 236, 77, 128, 126, 148, 87, 147, 48, 93, 185, 4, 247, 47, 112, 109, 148, 237, 89, 148, 54, 254, 106, 18, 112, 109, 139, 29, 238, 89, 148, 9, 226, 53, 254, 106, 110, 139, 29, 202, 147, 9, 208, 143, 185, 4, 40, 94, 227, 175, 254, 2, 215, 182, 70, 153, 0, 253, 226, 175, 38, 97, 3, 215, 182, 216, 158, 69, 153, 0, 176, 38, 97, 186, 182, 216, 161, 28, 185, 4, 102, 123, 22, 101, 156, 120, 141, 191, 93, 219, 98, 135, 21, 101, 2, 244, 121, 141, 191, 154, 11, 92, 219, 98, 102, 2, 244, 163, 190, 154, 132, 233, 185, 4, 192, 192, 181, 45, 182, 103, 81, 38, 249, 171, 73, 152, 181, 45, 118, 40, 104, 81, 38, 64, 215, 248, 171, 73, 46, 118, 40, 7, 38, 64, 63, 74, 185, 4, 207, 250, 164, 26, 127, 49, 137, 192, 210, 36, 128, 206, 164, 26, 78, 44, 50, 137, 192, 49, 211, 209, 36, 128, 27, 78, 44, 46, 192, 49, 5, 91, 185, 4, 60, 29, 77, 2, 172, 79, 170, 225, 9, 28, 83, 176, 76, 2, 232, 108, 80, 170, 225, 196, 146, 8, 28, 83, 3, 232, 108, 247, 224, 196, 226, 178, 185, 4, 96, 76, 34, 112, 236, 116, 52, 9, 170, 134, 19, 139, 33, 112, 76, 193, 117, 52, 9, 160, 62, 169, 134, 19, 113, 76, 193, 86, 8, 160, 179, 221, 185, 4, 236, 147, 106, 56, 197, 36, 2, 199, 148, 0, 58, 219, 105, 56, 177, 184, 37, 2, 199, 20, 71, 147, 0, 58, 57, 177, 184, 108, 198, 20, 108, 149, 185, 4, 217, 233, 104, 18, 103, 125, 82, 13, 69, 224, 152, 130, 104, 18, 64, 103, 126, 82, 13, 39, 152, 68, 224, 152, 19, 64, 103, 187, 12, 39, 22, 151, 185, 4, 140, 73, 4, 142, 157, 142, 38, 1, 214, 112, 98, 113, 3, 142, 41, 216, 143, 38, 1, 116, 39, 213, 112, 98, 143, 41, 216, 42, 0, 116, 182, 251, 185, 4, 90, 159, 84, 195, 47, 38, 17, 56, 155, 4, 208, 217, 83, 195, 137, 197, 39, 17, 56, 166, 57, 154, 4, 208, 196, 137, 197, 101, 55, 166, 96, 171, 185, 4, 168, 163, 73, 0, 245, 73, 53, 156, 130, 99, 10, 182, 72, 0, 157, 237, 74, 53, 156, 88, 18, 129, 99, 10, 1, 157, 237, 126, 155, 88, 92, 182, 185, 4, 9, 157, 133, 70, 121, 191, 87, 7, 35, 178, 134, 64, 133, 70, 130, 92, 192, 87, 7, 247, 162, 34, 178, 134, 71, 130, 92, 221, 6, 247, 98, 122, 185, 4, 56, 42, 34, 107, 208, 89, 104, 36, 118, 112, 47, 166, 33, 107, 8, 132, 90, 104, 36, 200, 123, 117, 112, 47, 108, 8, 132, 138, 35, 200, 213, 221, 185, 4, 223, 239, 213, 193, 222, 168, 136, 172, 162, 145, 32, 87, 213, 193, 189, 152, 169, 136, 172, 33, 103, 161, 145, 32, 194, 189, 152, 94, 172, 33, 16, 42, 185, 4, 117, 22, 26, 9, 253, 94, 29, 220, 201, 26, 2, 161, 25, 9, 114, 117, 95, 29, 220, 139, 138, 200, 26, 2, 10, 114, 117, 55, 219, 139, 233, 229, 185, 4, 191, 81, 17, 89, 132, 206, 66, 35, 172, 131, 123, 49, 17, 89, 67, 32, 207, 66, 35, 65, 223, 171, 131, 123, 90, 67, 32, 84, 34, 65, 174, 238, 185, 4, 252, 189, 58, 184, 27, 21, 145, 53, 53, 18, 228, 234, 57, 184, 23, 211, 22, 145, 53, 4, 44, 52, 18, 228, 185, 23, 211, 203, 52, 4, 66, 197, 185, 4, 162, 179, 208, 72, 238, 247, 234, 224, 69, 214, 16, 8, 208, 72, 144, 171, 248, 234, 224, 94, 84, 68, 214, 16, 73, 144, 171, 187, 224, 94, 76, 47, 185, 4, 71, 69, 100, 13, 58, 11, 141, 4, 15, 238, 197, 244, 99, 13, 129, 80, 12, 141, 4, 185, 174, 14, 238, 197, 14, 129, 80, 241, 3, 185, 186, 155, 185, 4, 50, 106, 216, 121, 138, 0, 253, 18, 43, 115, 117, 255, 215, 121, 188, 106, 1, 253, 18, 206, 148, 42, 115, 117, 122, 188, 106, 213, 18, 206, 149, 39, 185, 4, 85, 169, 50, 87, 162, 134, 157, 199, 48, 225, 92, 121, 50, 87, 247, 47, 135, 157, 199, 171, 207, 47, 225, 92, 88, 247, 47, 208, 198, 171, 86, 205, 185, 4, 35, 64, 191, 132, 80, 165, 202, 92, 119, 30, 175, 90, 190, 132, 115, 229, 166, 202, 92, 221, 25, 118, 30, 175, 133, 115, 229, 137, 92, 221, 191, 64, 185, 4, 169, 97, 231, 241, 1, 244, 75, 56, 205, 213, 253, 11, 231, 241, 170, 85, 245, 75, 56, 87, 169, 204, 213, 253, 242, 170, 85, 51, 56, 87, 158, 24, 185, 4, 162, 74, 149, 185, 24, 53, 236, 60, 127, 9, 231, 202, 148, 185, 186, 127, 54, 236, 60, 94, 127, 126, 9, 231, 186, 186, 127, 129, 60, 94, 181, 106, 185, 4, 5, 232, 151, 112, 169, 84, 153, 171, 207, 227, 85, 171, 151, 112, 174, 60, 85, 153, 171, 251, 194, 206, 227, 85, 113, 174, 60, 49, 171, 251, 23, 104, 185, 4, 71, 13, 59, 143, 16, 160, 95, 194, 102, 174, 238, 95, 58, 143, 87, 173, 161, 95, 194, 185, 81, 101, 174, 238, 144, 87, 173, 154, 193, 185, 242, 196, 185, 4, 43, 85, 230, 234, 211, 176, 243, 120, 38, 156, 43, 79, 230, 234, 254, 5, 177, 243, 120, 213, 249, 37, 156, 43, 235, 254, 5, 218, 120, 213, 170, 25, 185, 4, 87, 118, 201, 139, 156, 21, 234, 190, 77, 181, 98, 234, 200, 139, 243, 139, 22, 234, 190, 169, 115, 76, 181, 98, 140, 243, 139, 179, 190, 169, 137, 54, 185, 4, 65, 199, 84, 43, 101, 151, 188, 56, 239, 155, 154, 104, 84, 43, 166, 94, 152, 188, 56, 191, 160, 238, 155, 154, 44, 166, 94, 17, 56, 191, 56, 171, 185, 4, 104, 133, 186, 111, 2, 29, 83, 173, 243, 226, 252, 226, 185, 111, 106, 162, 30, 83, 173, 152, 92, 242, 226, 252, 112, 106, 162, 13, 173, 152, 122, 69, 185, 4, 218, 37, 47, 206, 85, 168, 251, 166, 214, 138, 169, 87, 46, 206, 47, 206, 169, 251, 166, 38, 49, 213, 138, 169, 207, 47, 206, 42, 166, 38, 218, 208, 185, 4, 7, 58, 166, 90, 42, 187, 228, 197, 118, 223, 212, 68, 165, 90, 49, 245, 188, 228, 197, 249, 9, 117, 223, 212, 91, 49, 245, 138, 197, 249, 197, 89, 185, 4, 173, 80, 247, 77, 160, 99, 170, 21, 95, 156, 95, 156, 246, 77, 77, 180, 100, 170, 21, 83, 75, 94, 156, 95, 78, 77, 180, 161, 21, 83, 175, 8, 185, 4, 203, 46, 121, 113, 179, 66, 221, 55, 170, 86, 76, 189, 120, 113, 126, 113, 67, 221, 55, 53, 142, 169, 86, 76, 114, 126, 113, 86, 55, 53, 209, 134, 185, 4, 233, 152, 106, 197, 235, 146, 23, 231, 126, 83, 19, 109, 106, 197, 212, 43, 147, 23, 231, 23, 212, 125, 83, 19, 198, 212, 43, 130, 230, 23, 103, 149, 185, 4, 166, 46, 234, 233, 158, 76, 47, 145, 231, 132, 96, 179, 233, 233, 68, 123, 77, 47, 145, 90, 132, 230, 132, 96, 234, 68, 123, 25, 145, 90, 209, 21, 185, 4, 76, 104, 78, 8, 234, 162, 158, 78, 19, 169, 21, 93, 78, 8, 54, 11, 163, 158, 78, 180, 244, 18, 169, 21, 9, 54, 11, 237, 77, 180, 151, 177, 185, 4, 40, 211, 75, 164, 46, 161, 57, 33, 123, 58, 209, 94, 75, 164, 86, 116, 162, 57, 33, 216, 138, 122, 58, 209, 165, 86, 116, 133, 32, 216, 44, 180, 185, 4, 187, 168, 167, 19, 50, 189, 68, 106, 20, 130, 205, 66, 167, 19, 237, 101, 190, 68, 106, 69, 153, 19, 130, 205, 20, 237, 101, 236, 105, 69, 87, 88, 185, 4, 94, 66, 115, 66, 82, 23, 245, 116, 152, 72, 173, 232, 114, 66, 176, 89, 24, 245, 116, 162, 165, 151, 72, 173, 67, 176, 89, 104, 116, 162, 189, 140, 185, 4, 101, 122, 137, 212, 37, 52, 39, 4, 80, 39, 218, 203, 136, 212, 138, 174, 53, 39, 4, 155, 80, 79, 39, 218, 213, 138, 174, 176, 3, 155, 133, 118, 185, 4, 213, 69, 61, 157, 147, 233, 37, 82, 157, 16, 108, 22, 61, 157, 104, 47, 234, 37, 82, 43, 208, 156, 16, 108, 158, 104, 47, 99, 81, 43, 186, 194, 185, 4, 10, 205, 9, 193, 92, 212, 211, 137, 35, 181, 162, 43, 9, 193, 102, 161, 213, 211, 137, 246, 93, 34, 181, 162, 194, 102, 161, 221, 136, 246, 50, 246, 185, 4, 149, 17, 35, 214, 204, 81, 72, 167, 149, 130, 50, 174, 34, 214, 97, 99, 82, 72, 167, 107, 156, 148, 130, 50, 215, 97, 99, 107, 166, 107, 238, 220, 185, 4, 202, 73, 41, 40, 25, 49, 98, 29, 117, 186, 230, 206, 40, 40, 227, 122, 50, 98, 29, 54, 132, 116, 186, 230, 41, 227, 122, 139, 28, 54, 182, 214, 185, 4, 116, 20, 210, 233, 38, 39, 165, 160, 137, 117, 216, 216, 209, 233, 154, 59, 40, 165, 160, 140, 195, 136, 117, 216, 234, 154, 59, 119, 160, 140, 235, 45, 185, 4, 71, 140, 88, 135, 70, 33, 157, 174, 11, 202, 184, 222, 87, 135, 141, 173, 34, 157, 174, 185, 81, 10, 202, 184, 136, 141, 173, 245, 173, 185, 115, 167, 185, 4, 80, 78, 74, 65, 201, 136, 17, 235, 165, 211, 53, 119, 73, 65, 25, 215, 137, 17, 235, 176, 40, 164, 211, 53, 66, 25, 215, 91, 234, 176, 177, 181, 185, 4, 143, 66, 58, 93, 228, 164, 20, 148, 178, 14, 27, 91, 57, 93, 115, 231, 165, 20, 148, 113, 24, 177, 14, 27, 94, 115, 231, 78, 147, 113, 189, 197, 185, 4, 51, 98, 196, 58, 57, 10, 233, 116, 83, 80, 198, 245, 195, 58, 108, 108, 11, 233, 116, 205, 146, 82, 80, 198, 59, 108, 108, 173, 116, 205, 157, 59, 185, 4, 58, 41, 5, 101, 34, 70, 172, 195, 79, 215, 220, 185, 4, 101, 92, 111, 71, 172, 195, 198, 143, 78, 215, 220, 102, 92, 111, 177, 194, 198, 214, 250, 185, 4, 134, 235, 53, 213, 183, 7, 236, 170, 223, 127, 71, 248, 52, 213, 61, 243, 8, 236, 170, 122, 12, 222, 127, 71, 214, 61, 243, 33, 170, 122, 20, 202, 185, 4, 205, 224, 253, 119, 183, 94, 83, 221, 175, 170, 71, 161, 253, 119, 132, 63, 95, 83, 221, 51, 192, 174, 170, 71, 120, 132, 63, 81, 221, 51, 31, 2, 185, 4, 239, 1, 187, 170, 47, 131, 247, 223, 78, 117, 207, 124, 186, 170, 30, 133, 132, 247, 223, 17, 122, 77, 117, 207, 171, 30, 133, 178, 223, 17, 254, 68, 185, 4, 175, 215, 84, 247, 29, 176, 171, 234, 0, 30, 225, 79, 84, 247, 204, 135, 177, 171, 234, 81, 119, 255, 29, 225, 248, 204, 135, 0, 234, 81, 40, 171, 185, 4, 98, 6, 239, 191, 193, 245, 154, 234, 119, 85, 61, 10, 238, 191, 35, 252, 246, 154, 234, 158, 3, 118, 85, 61, 192, 35, 252, 137, 234, 158, 249, 16, 185, 4, 62, 96, 87, 213, 101, 240, 254, 59, 170, 238, 153, 15, 87, 213, 163, 80, 241, 254, 59, 194, 174, 169, 238, 153, 214, 163, 80, 86, 59, 194, 159, 168, 185, 4, 113, 189, 166, 186, 246, 128, 93, 85, 252, 239, 8, 127, 166, 186, 103, 62, 129, 93, 85, 143, 193, 251, 239, 8, 187, 103, 62, 4, 85, 143, 66, 89, 185, 4, 26, 188, 255, 142, 214, 107, 170, 123, 86, 245, 40, 148, 255, 142, 240, 39, 108, 170, 123, 230, 215, 85, 245, 40, 143, 240, 39, 170, 123, 230, 67, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 1, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 251, 2, 36, 0, 118, 191, 145, 119, 191, 149, 130, 153, 152, 152, 152, 154, 154, 160, 160, 160, 160, 211, 0, 0, 152, 130, 152, 152, 154, 153, 154, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 186, 0, 0, 0, 0, 0, 0, 0, 0, 10, 105, 98, 117, 116, 116, 101, 114, 102, 108, 121, 0, 0, 0, 0, 0, 47, 0, 128, 128, 151, 3, 130, 151, 3, 130, 149, 151, 3, 165, 150, 151, 3, 166, 163, 164, 151, 5, 130, 151, 5, 130, 149, 151, 5, 165, 150, 151, 5, 166, 163, 151, 7, 130, 151, 7, 130, 149, 151, 7, 165, 150, 151, 7, 166, 11, 109, 117, 108, 95, 98, 121, 95, 105, 110, 118, 78, 0, 0, 0, 0, 0, 14, 0, 185, 1, 1, 0, 128, 0, 255, 255, 127, 255, 7, 130, 185, 1, 1, 0, 128, 0, 255, 255, 127, 255, 7, 130, 149, 185, 1, 1, 0, 128, 0, 255, 255, 127, 255, 7, 165, 150, 185, 1, 1, 0, 128, 0, 255, 255, 127, 255, 7, 166, 8, 98, 97, 99, 107, 119, 97, 114, 100, 233, 3, 65, 112, 112, 108, 105, 101, 115, 32, 105, 110, 118, 101, 114, 115, 101, 32, 78, 84, 84, 32, 111, 110, 32, 97, 32, 118, 101, 99, 116, 111, 114, 32, 111, 102, 32, 108, 101, 110, 103, 116, 104, 32, 53, 49, 50, 44, 32, 119, 104, 101, 114, 101, 32, 101, 97, 99, 104, 32, 101, 108, 101, 109, 101, 110, 116, 32, 226, 136, 136, 32, 90, 112, 32, 124, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 226, 136, 146, 32, 50, 94, 51, 50, 32, 43, 32, 49, 44, 10, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 105, 110, 32, 116, 105, 109, 101, 32, 100, 111, 109, 97, 105, 110, 32, 105, 110, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 111, 114, 100, 101, 114, 44, 32, 119, 104, 105, 108, 101, 32, 105, 110, 112, 117, 116, 32, 118, 101, 99, 116, 111, 114, 32, 105, 115, 32, 101, 120, 112, 101, 99, 116, 101, 100, 32, 116, 111, 32, 98, 101, 32, 105, 110, 10, 98, 105, 116, 45, 114, 101, 118, 101, 114, 115, 101, 100, 32, 111, 114, 100, 101, 114, 46, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 97, 115, 32, 105, 110, 112, 117, 116, 58, 10, 91, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 46, 46, 46, 93, 32, 124, 32, 83, 105, 110, 103, 108, 101, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 44, 32, 119, 104, 101, 114, 101, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 115, 116, 97, 114, 116, 115, 10, 78, 111, 116, 101, 44, 32, 116, 111, 116, 97, 108, 32, 49, 50, 56, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 102, 111, 114, 32, 115, 116, 111, 114, 105, 110, 103, 32, 119, 104, 111, 108, 101, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 46, 32, 78, 101, 120, 116, 32, 49, 50, 55, 10, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 99, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 105, 46, 101, 46, 32, 99, 111, 109, 112, 117, 116, 97, 98, 108, 101, 32, 98, 121, 32, 117, 115, 105, 110, 103, 32, 96, 97, 100, 100, 46, 49, 96, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 111, 110, 32, 112, 114, 101, 118, 105, 111, 117, 115, 32, 97, 100, 100, 114, 101, 115, 115, 46, 10, 97, 100, 100, 114, 123, 105, 125, 32, 104, 111, 108, 100, 115, 32, 118, 97, 108, 117, 101, 115, 32, 86, 91, 40, 105, 32, 60, 60, 32, 50, 41, 32, 46, 46, 32, 40, 40, 105, 43, 49, 41, 32, 60, 60, 32, 50, 41, 93, 32, 124, 32, 105, 32, 226, 136, 136, 32, 91, 48, 44, 32, 49, 50, 56, 41, 32, 97, 110, 100, 32, 97, 100, 100, 114, 48, 32, 61, 32, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 10, 65, 102, 116, 101, 114, 32, 97, 112, 112, 108, 121, 105, 110, 103, 32, 105, 78, 84, 84, 44, 32, 110, 111, 114, 109, 97, 108, 32, 111, 114, 100, 101, 114, 32, 118, 101, 99, 116, 111, 114, 32, 105, 115, 32, 114, 101, 116, 117, 114, 110, 101, 100, 32, 98, 97, 99, 107, 32, 97, 115, 32, 115, 105, 110, 103, 108, 101, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 10, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 119, 104, 101, 114, 101, 32, 105, 116, 32, 98, 101, 103, 105, 110, 115, 32, 115, 116, 111, 114, 105, 110, 103, 32, 116, 104, 101, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 46, 32, 67, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 49, 50, 55, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 115, 104, 111, 117, 108, 100, 10, 115, 105, 109, 105, 108, 97, 114, 108, 121, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 97, 98, 108, 101, 32, 117, 115, 105, 110, 103, 32, 96, 97, 100, 100, 46, 49, 96, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 46, 10, 91, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 39, 44, 32, 46, 46, 46, 93, 32, 124, 32, 83, 105, 110, 103, 108, 101, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 44, 32, 119, 104, 101, 114, 101, 32, 114, 101, 115, 117, 108, 116, 105, 110, 103, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 115, 116, 97, 114, 116, 115, 10, 78, 111, 116, 101, 44, 32, 105, 110, 112, 117, 116, 32, 109, 101, 109, 111, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 105, 115, 32, 110, 111, 116, 32, 109, 117, 116, 97, 116, 101, 100, 44, 32, 105, 110, 115, 116, 101, 97, 100, 32, 111, 117, 116, 112, 117, 116, 32, 105, 115, 32, 115, 116, 111, 114, 101, 100, 32, 105, 110, 32, 100, 105, 102, 102, 101, 114, 101, 110, 116, 32, 109, 101, 109, 111, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 46, 1, 128, 0, 172, 1, 186, 0, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 107, 3, 10, 0, 115, 191, 114, 198, 152, 3, 168, 151, 3, 167, 108, 107, 107, 185, 4, 114, 15, 216, 85, 131, 25, 188, 255, 149, 85, 132, 25, 39, 170, 10, 215, 171, 10, 215, 107, 255, 112, 15, 216, 231, 67, 0, 113, 40, 148, 85, 132, 185, 4, 70, 152, 193, 251, 169, 112, 189, 166, 128, 162, 170, 112, 61, 4, 16, 247, 5, 16, 247, 128, 88, 69, 152, 193, 144, 66, 89, 69, 8, 127, 162, 170, 185, 4, 43, 92, 175, 169, 195, 61, 96, 87, 16, 1, 196, 61, 80, 86, 17, 102, 87, 17, 102, 240, 167, 42, 92, 175, 195, 159, 168, 42, 153, 15, 1, 196, 185, 4, 65, 220, 3, 118, 20, 97, 6, 239, 11, 101, 21, 97, 251, 137, 170, 194, 138, 170, 194, 245, 16, 64, 220, 3, 159, 249, 16, 64, 61, 10, 101, 21, 185, 4, 9, 51, 120, 255, 20, 174, 215, 84, 80, 84, 21, 174, 135, 0, 226, 30, 1, 226, 30, 176, 170, 8, 51, 120, 82, 40, 171, 8, 225, 79, 84, 21, 185, 4, 86, 225, 122, 77, 31, 238, 1, 187, 125, 8, 32, 238, 132, 178, 138, 48, 179, 138, 48, 131, 68, 85, 225, 122, 18, 254, 68, 85, 207, 124, 8, 32, 185, 4, 137, 123, 192, 174, 33, 204, 224, 253, 162, 172, 34, 204, 62, 81, 85, 184, 82, 85, 184, 94, 1, 136, 123, 192, 52, 31, 2, 136, 71, 161, 172, 34, 185, 4, 43, 194, 12, 222, 84, 133, 235, 53, 249, 19, 85, 133, 242, 33, 128, 184, 34, 128, 184, 7, 202, 42, 194, 12, 123, 20, 202, 42, 71, 248, 19, 85, 185, 4, 155, 163, 144, 78, 60, 57, 41, 5, 186, 83, 60, 57, 111, 177, 40, 35, 178, 40, 35, 70, 250, 154, 163, 144, 199, 214, 250, 154, 220, 185, 83, 60, 185, 4, 198, 147, 147, 82, 138, 50, 98, 196, 246, 22, 139, 50, 108, 173, 175, 57, 174, 175, 57, 10, 59, 197, 147, 147, 206, 157, 59, 197, 197, 245, 22, 139, 185, 4, 163, 140, 24, 177, 107, 142, 66, 58, 92, 235, 107, 142, 230, 78, 241, 228, 79, 241, 228, 164, 197, 162, 140, 24, 114, 189, 197, 162, 26, 91, 235, 107, 185, 4, 191, 230, 40, 164, 20, 79, 78, 74, 120, 238, 20, 79, 214, 91, 44, 202, 92, 44, 202, 136, 181, 190, 230, 40, 177, 177, 181, 190, 53, 119, 238, 20, 185, 4, 121, 114, 82, 10, 81, 70, 140, 88, 223, 98, 81, 70, 173, 245, 53, 71, 246, 53, 71, 33, 167, 120, 114, 82, 186, 115, 167, 120, 184, 222, 98, 81, 185, 4, 23, 101, 196, 136, 94, 115, 20, 210, 217, 90, 95, 115, 59, 119, 138, 39, 120, 138, 39, 39, 45, 22, 101, 196, 141, 235, 45, 22, 216, 216, 90, 95, 185, 4, 216, 28, 133, 116, 226, 201, 73, 41, 207, 157, 226, 201, 122, 139, 69, 25, 140, 69, 25, 49, 214, 215, 28, 133, 55, 182, 214, 215, 229, 206, 157, 226, 185, 4, 42, 158, 156, 148, 88, 148, 17, 35, 175, 183, 88, 148, 98, 107, 125, 205, 108, 125, 205, 81, 220, 41, 158, 156, 108, 238, 220, 41, 50, 174, 183, 88, 185, 4, 63, 153, 94, 34, 118, 9, 205, 9, 44, 44, 118, 9, 161, 221, 74, 93, 222, 74, 93, 212, 245, 62, 153, 94, 247, 50, 246, 62, 162, 43, 44, 118, 185, 4, 99, 151, 208, 156, 173, 212, 69, 61, 23, 218, 173, 212, 46, 99, 239, 147, 100, 239, 147, 233, 193, 98, 151, 208, 44, 186, 194, 98, 107, 22, 218, 173, 185, 4, 44, 117, 81, 79, 251, 100, 122, 137, 204, 216, 251, 100, 174, 176, 216, 37, 177, 216, 37, 52, 118, 43, 117, 81, 156, 133, 118, 43, 217, 203, 216, 251, 185, 4, 190, 79, 166, 151, 138, 93, 66, 115, 233, 10, 139, 93, 89, 104, 183, 82, 105, 183, 82, 23, 140, 189, 79, 166, 163, 189, 140, 189, 172, 232, 10, 139, 185, 4, 237, 18, 154, 19, 149, 186, 168, 167, 67, 187, 149, 186, 101, 236, 125, 50, 237, 125, 50, 189, 87, 236, 18, 154, 70, 87, 88, 236, 204, 66, 187, 149, 185, 4, 92, 169, 139, 122, 222, 39, 211, 75, 95, 198, 222, 39, 116, 133, 197, 46, 134, 197, 46, 161, 179, 91, 169, 139, 217, 44, 180, 91, 208, 94, 198, 222, 185, 4, 248, 201, 244, 18, 177, 75, 104, 78, 94, 97, 177, 75, 10, 237, 86, 234, 238, 86, 234, 162, 176, 247, 201, 244, 181, 151, 177, 247, 20, 93, 97, 177, 185, 4, 23, 187, 132, 230, 109, 165, 46, 234, 180, 208, 110, 165, 122, 25, 123, 159, 26, 123, 159, 76, 21, 22, 187, 132, 91, 209, 21, 22, 96, 179, 208, 110, 185, 4, 59, 43, 212, 125, 24, 232, 152, 106, 110, 232, 24, 232, 42, 130, 172, 236, 131, 172, 236, 146, 148, 58, 43, 212, 24, 103, 149, 58, 19, 109, 232, 24, 185, 4, 143, 129, 142, 169, 199, 202, 46, 121, 190, 34, 200, 202, 112, 86, 169, 179, 87, 169, 179, 66, 134, 142, 129, 142, 54, 209, 134, 142, 75, 189, 34, 200, 185, 4, 179, 178, 75, 94, 233, 172, 80, 247, 157, 85, 234, 172, 179, 161, 99, 160, 162, 99, 160, 99, 8, 178, 178, 75, 84, 175, 8, 178, 94, 156, 85, 234, 185, 4, 166, 206, 10, 117, 57, 6, 58, 166, 69, 27, 58, 6, 245, 138, 32, 43, 139, 32, 43, 187, 89, 165, 206, 10, 250, 197, 89, 165, 212, 68, 27, 58, 185, 4, 50, 208, 49, 213, 88, 217, 37, 47, 88, 4, 89, 217, 205, 42, 117, 86, 43, 117, 86, 168, 208, 49, 208, 49, 39, 218, 208, 49, 169, 87, 4, 89, 185, 4, 145, 149, 93, 242, 81, 103, 133, 186, 227, 172, 82, 103, 162, 13, 29, 3, 14, 29, 3, 29, 69, 144, 149, 93, 153, 122, 69, 144, 252, 226, 172, 82, 185, 4, 213, 89, 161, 238, 198, 64, 199, 84, 105, 67, 199, 64, 94, 17, 100, 101, 18, 100, 101, 151, 170, 212, 89, 161, 192, 56, 171, 212, 153, 104, 67, 199, 185, 4, 117, 12, 116, 76, 64, 86, 118, 201, 235, 21, 65, 86, 139, 179, 74, 157, 180, 74, 157, 21, 54, 116, 12, 116, 170, 137, 54, 116, 98, 234, 21, 65, 185, 4, 22, 1, 250, 37, 134, 42, 85, 230, 80, 12, 135, 42, 5, 218, 99, 212, 219, 99, 212, 176, 24, 21, 1, 250, 214, 170, 25, 21, 43, 79, 12, 135, 185, 4, 113, 168, 82, 101, 61, 70, 13, 59, 96, 160, 61, 70, 173, 154, 81, 17, 155, 81, 17, 160, 196, 112, 168, 82, 186, 242, 196, 112, 238, 95, 160, 61, 185, 4, 144, 81, 195, 206, 83, 4, 232, 151, 172, 102, 84, 4, 60, 49, 28, 170, 50, 28, 170, 84, 103, 143, 81, 195, 252, 23, 104, 143, 85, 171, 102, 84, 185, 4, 71, 69, 128, 126, 194, 161, 74, 149, 203, 19, 195, 161, 127, 129, 246, 24, 130, 246, 24, 53, 106, 70, 69, 128, 95, 181, 106, 70, 230, 202, 19, 195, 185, 4, 15, 85, 170, 204, 198, 168, 97, 231, 12, 180, 199, 168, 85, 51, 42, 2, 52, 42, 2, 244, 23, 14, 85, 170, 88, 158, 24, 14, 253, 11, 180, 199, 185, 4, 124, 140, 26, 118, 162, 34, 64, 191, 91, 53, 163, 34, 229, 137, 225, 80, 138, 225, 80, 165, 64, 123, 140, 26, 222, 191, 64, 123, 174, 90, 53, 163, 185, 4, 169, 8, 208, 47, 56, 84, 169, 50, 122, 98, 56, 84, 47, 208, 30, 163, 209, 30, 163, 134, 204, 168, 8, 208, 172, 86, 205, 168, 92, 121, 98, 56, 185, 4, 135, 67, 149, 42, 236, 49, 106, 216, 0, 3, 237, 49, 106, 213, 140, 138, 214, 140, 138, 0, 39, 134, 67, 149, 207, 149, 39, 134, 116, 255, 2, 237, 185, 4, 243, 126, 175, 14, 251, 70, 69, 100, 245, 114, 251, 70, 80, 241, 17, 58, 242, 17, 58, 11, 155, 242, 126, 175, 186, 186, 155, 242, 196, 244, 114, 251, 185, 4, 184, 111, 84, 68, 30, 161, 179, 208, 9, 21, 31, 161, 170, 187, 41, 239, 188, 41, 239, 247, 46, 183, 111, 84, 95, 76, 47, 183, 16, 8, 21, 31, 185, 4, 72, 232, 44, 52, 202, 251, 189, 58, 235, 110, 202, 251, 210, 203, 237, 27, 204, 237, 27, 21, 197, 71, 232, 44, 5, 66, 197, 71, 227, 234, 110, 202, 185, 4, 167, 188, 223, 171, 220, 190, 81, 17, 50, 189, 220, 190, 31, 84, 124, 132, 85, 124, 132, 206, 237, 166, 188, 223, 66, 174, 238, 166, 122, 49, 189, 220, 185, 4, 247, 141, 138, 200, 35, 116, 22, 26, 162, 226, 35, 116, 116, 55, 229, 253, 56, 229, 253, 94, 229, 246, 141, 138, 140, 233, 229, 246, 1, 161, 226, 35, 185, 4, 63, 66, 103, 161, 82, 222, 239, 213, 88, 119, 83, 222, 151, 94, 110, 223, 95, 110, 223, 168, 41, 62, 66, 103, 34, 16, 42, 62, 32, 87, 119, 83, 185, 4, 149, 247, 123, 117, 219, 55, 42, 34, 167, 151, 219, 55, 131, 138, 143, 208, 139, 143, 208, 89, 221, 148, 247, 123, 201, 213, 221, 148, 46, 166, 151, 219, 185, 4, 186, 125, 163, 34, 248, 8, 157, 133, 65, 168, 248, 8, 92, 221, 77, 121, 222, 77, 121, 191, 121, 185, 125, 163, 248, 98, 122, 185, 133, 64, 168, 248, 185, 4, 0, 99, 18, 129, 99, 167, 163, 73, 183, 202, 99, 167, 236, 126, 156, 245, 127, 156, 245, 73, 182, 255, 98, 18, 89, 92, 182, 255, 9, 182, 202, 99, 185, 4, 61, 118, 58, 154, 199, 89, 159, 84, 218, 238, 199, 89, 197, 101, 251, 47, 102, 251, 47, 38, 171, 60, 118, 58, 167, 96, 171, 60, 207, 217, 238, 199, 185, 4, 114, 214, 39, 213, 254, 139, 73, 4, 114, 217, 254, 139, 215, 42, 143, 157, 43, 143, 157, 142, 251, 113, 214, 39, 117, 182, 251, 113, 97, 113, 217, 254, 185, 4, 238, 191, 152, 68, 242, 216, 233, 104, 131, 173, 242, 216, 102, 187, 31, 103, 188, 31, 103, 125, 150, 237, 191, 152, 40, 22, 151, 237, 151, 130, 173, 242, 185, 4, 200, 78, 71, 147, 56, 235, 147, 106, 220, 253, 56, 235, 183, 108, 255, 197, 109, 255, 197, 36, 149, 199, 78, 71, 21, 108, 149, 199, 57, 219, 253, 56, 185, 4, 144, 179, 62, 169, 246, 95, 76, 34, 140, 203, 246, 95, 192, 86, 121, 236, 87, 121, 236, 116, 221, 143, 179, 62, 161, 179, 221, 143, 18, 139, 203, 246, 185, 4, 254, 23, 147, 8, 30, 59, 29, 77, 177, 85, 30, 59, 108, 247, 227, 172, 248, 227, 172, 79, 178, 253, 23, 147, 197, 226, 178, 253, 82, 176, 85, 30, 185, 4, 230, 177, 211, 209, 62, 206, 250, 164, 207, 118, 63, 206, 43, 46, 219, 127, 47, 219, 127, 49, 90, 229, 177, 211, 50, 5, 91, 229, 127, 206, 118, 63, 185, 4, 211, 137, 215, 248, 216, 191, 192, 181, 153, 174, 217, 191, 39, 7, 84, 182, 8, 84, 182, 103, 73, 210, 137, 215, 65, 63, 74, 210, 72, 152, 174, 217, 185, 4, 155, 253, 11, 92, 64, 101, 123, 22, 136, 114, 64, 101, 243, 163, 36, 157, 164, 36, 157, 120, 233, 154, 253, 11, 155, 132, 233, 154, 98, 135, 114, 64, 185, 4, 81, 217, 158, 69, 72, 39, 94, 227, 254, 40, 73, 39, 96, 186, 102, 255, 187, 102, 255, 2, 28, 80, 217, 158, 217, 161, 28, 80, 0, 253, 40, 73, 185, 4, 147, 116, 226, 53, 107, 246, 47, 112, 19, 166, 107, 246, 28, 202, 1, 149, 203, 1, 149, 237, 142, 146, 116, 226, 10, 208, 143, 146, 106, 18, 166, 107, 185, 4, 180, 127, 129, 107, 167, 108, 207, 162, 81, 14, 168, 108, 126, 148, 164, 19, 149, 164, 19, 175, 92, 179, 127, 129, 148, 48, 93, 179, 235, 80, 14, 168, 185, 4, 129, 202, 246, 44, 73, 58, 241, 26, 233, 71, 73, 58, 8, 211, 53, 251, 212, 53, 251, 23, 228, 128, 202, 246, 198, 14, 229, 128, 4, 232, 71, 73, 185, 4, 147, 78, 188, 198, 204, 254, 5, 174, 195, 116, 205, 254, 66, 57, 160, 178, 58, 160, 178, 61, 81, 146, 78, 188, 2, 250, 81, 146, 76, 194, 116, 205, 185, 4, 216, 236, 95, 224, 2, 42, 219, 179, 60, 148, 3, 42, 159, 31, 37, 233, 32, 37, 233, 196, 75, 215, 236, 95, 214, 36, 76, 215, 22, 59, 148, 3, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 1, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 192, 3, 36, 0, 118, 191, 145, 119, 191, 149, 130, 153, 152, 152, 152, 154, 154, 160, 160, 160, 160, 211, 2, 0, 152, 130, 152, 152, 154, 153, 154, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 135, 36, 159, 90, 119, 78, 45, 233, 135, 36, 159, 90, 119, 78, 45, 233, 205, 67, 120, 78, 96, 165, 1, 141, 205, 67, 120, 78, 96, 165, 1, 141, 185, 4, 166, 1, 141, 51, 22, 134, 36, 159, 166, 1, 141, 51, 22, 134, 36, 159, 178, 210, 22, 134, 114, 204, 67, 120, 178, 210, 22, 134, 114, 204, 67, 120, 185, 4, 133, 231, 212, 146, 25, 208, 56, 195, 133, 231, 212, 146, 25, 208, 56, 195, 14, 86, 26, 208, 42, 109, 97, 72, 14, 86, 26, 208, 42, 109, 97, 72, 185, 4, 110, 97, 72, 242, 59, 132, 231, 212, 110, 97, 72, 242, 59, 132, 231, 212, 48, 199, 60, 132, 183, 13, 86, 26, 48, 199, 60, 132, 183, 13, 86, 26, 185, 4, 7, 52, 206, 240, 23, 146, 124, 106, 7, 52, 206, 240, 23, 146, 124, 106, 75, 91, 24, 146, 49, 15, 225, 57, 75, 91, 24, 146, 49, 15, 225, 57, 185, 4, 16, 225, 57, 181, 148, 6, 52, 206, 16, 225, 57, 181, 148, 6, 52, 206, 110, 131, 149, 6, 198, 74, 91, 24, 110, 131, 149, 6, 198, 74, 91, 24, 185, 4, 134, 33, 201, 167, 16, 158, 83, 75, 134, 33, 201, 167, 16, 158, 83, 75, 29, 243, 16, 158, 54, 88, 105, 64, 29, 243, 16, 158, 54, 88, 105, 64, 185, 4, 89, 105, 64, 227, 179, 133, 33, 201, 89, 105, 64, 227, 179, 133, 33, 201, 98, 172, 180, 133, 191, 28, 243, 16, 98, 172, 180, 133, 191, 28, 243, 16, 185, 4, 241, 156, 90, 210, 2, 26, 103, 120, 241, 156, 90, 210, 2, 26, 103, 120, 194, 74, 3, 26, 165, 45, 12, 73, 194, 74, 3, 26, 165, 45, 12, 73, 185, 4, 46, 12, 73, 62, 135, 240, 156, 90, 46, 12, 73, 62, 135, 240, 156, 90, 230, 152, 135, 240, 182, 193, 74, 3, 230, 152, 135, 240, 182, 193, 74, 3, 185, 4, 53, 160, 113, 134, 194, 144, 228, 83, 53, 160, 113, 134, 194, 144, 228, 83, 87, 218, 194, 144, 141, 121, 8, 207, 87, 218, 194, 144, 141, 121, 8, 207, 185, 4, 122, 8, 207, 169, 171, 52, 160, 113, 122, 8, 207, 169, 171, 52, 160, 113, 112, 27, 172, 52, 48, 86, 218, 194, 112, 27, 172, 52, 48, 86, 218, 194, 185, 4, 49, 36, 249, 212, 193, 115, 106, 73, 49, 36, 249, 212, 193, 115, 106, 73, 100, 30, 194, 115, 6, 43, 13, 104, 100, 30, 194, 115, 6, 43, 13, 104, 185, 4, 44, 13, 104, 156, 181, 48, 36, 249, 44, 13, 104, 156, 181, 48, 36, 249, 141, 149, 182, 48, 151, 99, 30, 194, 141, 149, 182, 48, 151, 99, 30, 194, 185, 4, 34, 60, 167, 150, 210, 128, 198, 25, 34, 60, 167, 150, 210, 128, 198, 25, 110, 176, 210, 128, 88, 105, 11, 67, 110, 176, 210, 128, 88, 105, 11, 67, 185, 4, 106, 11, 67, 146, 229, 33, 60, 167, 106, 11, 67, 146, 229, 33, 60, 167, 128, 57, 230, 33, 188, 109, 176, 210, 128, 57, 230, 33, 188, 109, 176, 210, 185, 4, 152, 232, 86, 212, 11, 209, 137, 82, 152, 232, 86, 212, 11, 209, 137, 82, 225, 38, 12, 209, 168, 43, 92, 70, 225, 38, 12, 209, 168, 43, 92, 70, 185, 4, 44, 92, 70, 31, 173, 151, 232, 86, 44, 92, 70, 31, 173, 151, 232, 86, 47, 118, 173, 151, 185, 224, 38, 12, 47, 118, 173, 151, 185, 224, 38, 12, 185, 4, 195, 16, 157, 40, 194, 101, 244, 145, 195, 16, 157, 40, 194, 101, 244, 145, 146, 186, 194, 101, 98, 215, 122, 137, 146, 186, 194, 101, 98, 215, 122, 137, 185, 4, 216, 122, 137, 110, 109, 194, 16, 157, 216, 122, 137, 110, 109, 194, 16, 157, 155, 11, 110, 194, 117, 145, 186, 194, 155, 11, 110, 194, 117, 145, 186, 194, 185, 4, 113, 25, 125, 100, 94, 162, 91, 81, 113, 25, 125, 100, 94, 162, 91, 81, 217, 181, 94, 162, 130, 155, 48, 68, 217, 181, 94, 162, 130, 155, 48, 68, 185, 4, 156, 48, 68, 39, 174, 112, 25, 125, 156, 48, 68, 39, 174, 112, 25, 125, 94, 164, 174, 112, 187, 216, 181, 94, 94, 164, 174, 112, 187, 216, 181, 94, 185, 4, 236, 37, 186, 21, 9, 67, 116, 162, 236, 37, 186, 21, 9, 67, 116, 162, 47, 184, 9, 67, 69, 234, 10, 151, 47, 184, 9, 67, 69, 234, 10, 151, 185, 4, 235, 10, 151, 209, 92, 235, 37, 186, 235, 10, 151, 209, 92, 235, 37, 186, 189, 139, 93, 235, 104, 46, 184, 9, 189, 139, 93, 235, 104, 46, 184, 9, 185, 4, 25, 162, 19, 165, 183, 140, 62, 178, 25, 162, 19, 165, 183, 140, 62, 178, 83, 87, 184, 140, 235, 90, 47, 209, 83, 87, 184, 140, 235, 90, 47, 209, 185, 4, 91, 47, 209, 173, 77, 24, 162, 19, 91, 47, 209, 173, 77, 24, 162, 19, 116, 193, 77, 24, 46, 82, 87, 184, 116, 193, 77, 24, 46, 82, 87, 184, 185, 4, 134, 203, 232, 35, 245, 18, 221, 138, 134, 203, 232, 35, 245, 18, 221, 138, 198, 174, 245, 18, 23, 220, 132, 33, 198, 174, 245, 18, 23, 220, 132, 33, 185, 4, 221, 132, 33, 58, 116, 133, 203, 232, 221, 132, 33, 58, 116, 133, 203, 232, 238, 34, 117, 133, 221, 197, 174, 245, 238, 34, 117, 133, 221, 197, 174, 245, 185, 4, 190, 68, 183, 162, 96, 136, 78, 148, 190, 68, 183, 162, 96, 136, 78, 148, 6, 55, 97, 136, 72, 93, 225, 50, 6, 55, 97, 136, 72, 93, 225, 50, 185, 4, 94, 225, 50, 250, 106, 189, 68, 183, 94, 225, 50, 250, 106, 189, 68, 183, 120, 177, 107, 189, 204, 5, 55, 97, 120, 177, 107, 189, 204, 5, 55, 97, 185, 4, 20, 134, 232, 68, 21, 46, 163, 143, 20, 134, 232, 68, 21, 46, 163, 143, 140, 212, 21, 46, 23, 187, 214, 75, 140, 212, 21, 46, 23, 187, 214, 75, 185, 4, 188, 214, 75, 116, 111, 19, 134, 232, 188, 214, 75, 116, 111, 19, 134, 232, 210, 92, 112, 19, 180, 139, 212, 21, 210, 92, 112, 19, 180, 139, 212, 21, 185, 4, 149, 179, 166, 164, 199, 175, 74, 159, 149, 179, 166, 164, 199, 175, 74, 159, 242, 67, 200, 175, 88, 91, 163, 156, 242, 67, 200, 175, 88, 91, 163, 156, 185, 4, 92, 163, 156, 14, 96, 148, 179, 166, 92, 163, 156, 14, 96, 148, 179, 166, 81, 181, 96, 148, 98, 241, 67, 200, 81, 181, 96, 148, 98, 241, 67, 200, 185, 4, 133, 252, 170, 244, 52, 202, 233, 192, 133, 252, 170, 244, 52, 202, 233, 192, 149, 181, 53, 202, 84, 11, 70, 57, 149, 181, 53, 202, 84, 11, 70, 57, 185, 4, 12, 70, 57, 107, 62, 132, 252, 170, 12, 70, 57, 107, 62, 132, 252, 170, 54, 22, 63, 132, 198, 148, 181, 53, 54, 22, 63, 132, 198, 148, 181, 53, 185, 4, 142, 114, 58, 240, 80, 206, 154, 146, 142, 114, 58, 240, 80, 206, 154, 146, 214, 130, 81, 206, 196, 15, 33, 191, 214, 130, 81, 206, 196, 15, 33, 191, 185, 4, 16, 33, 191, 42, 109, 141, 114, 58, 16, 33, 191, 42, 109, 141, 114, 58, 50, 101, 109, 141, 64, 213, 130, 81, 50, 101, 109, 141, 64, 213, 130, 81, 185, 4, 25, 229, 172, 41, 16, 242, 171, 210, 25, 229, 172, 41, 16, 242, 171, 210, 89, 252, 16, 242, 82, 214, 214, 40, 89, 252, 16, 242, 82, 214, 214, 40, 185, 4, 215, 214, 40, 167, 44, 24, 229, 172, 215, 214, 40, 167, 44, 24, 229, 172, 14, 84, 45, 24, 215, 88, 252, 16, 14, 84, 45, 24, 215, 88, 252, 16, 185, 4, 145, 95, 149, 62, 70, 57, 29, 120, 145, 95, 149, 62, 70, 57, 29, 120, 179, 182, 70, 57, 106, 193, 40, 103, 179, 182, 70, 57, 106, 193, 40, 103, 185, 4, 194, 40, 103, 77, 135, 144, 95, 149, 194, 40, 103, 77, 135, 144, 95, 149, 199, 226, 135, 144, 152, 178, 182, 70, 199, 226, 135, 144, 152, 178, 182, 70, 185, 4, 108, 148, 211, 129, 139, 114, 214, 148, 108, 148, 211, 129, 139, 114, 214, 148, 171, 22, 140, 114, 43, 126, 8, 249, 171, 22, 140, 114, 43, 126, 8, 249, 185, 4, 127, 8, 249, 85, 106, 107, 148, 211, 127, 8, 249, 85, 106, 107, 148, 211, 142, 41, 107, 107, 6, 170, 22, 140, 142, 41, 107, 107, 6, 170, 22, 140, 185, 4, 164, 156, 53, 37, 65, 126, 85, 250, 164, 156, 53, 37, 65, 126, 85, 250, 140, 31, 66, 126, 201, 218, 26, 229, 140, 31, 66, 126, 201, 218, 26, 229, 185, 4, 219, 26, 229, 116, 5, 163, 156, 53, 219, 26, 229, 116, 5, 163, 156, 53, 130, 170, 5, 163, 26, 139, 31, 66, 130, 170, 5, 163, 26, 139, 31, 66, 185, 4, 34, 228, 87, 165, 173, 81, 78, 7, 34, 228, 87, 165, 173, 81, 78, 7, 167, 172, 173, 81, 167, 90, 48, 202, 167, 172, 173, 81, 167, 90, 48, 202, 185, 4, 91, 48, 202, 89, 248, 33, 228, 87, 91, 48, 202, 89, 248, 33, 228, 87, 175, 177, 248, 33, 53, 166, 172, 173, 175, 177, 248, 33, 53, 166, 172, 173, 185, 4, 9, 145, 171, 88, 244, 17, 196, 133, 9, 145, 171, 88, 244, 17, 196, 133, 112, 222, 244, 17, 84, 167, 2, 93, 112, 222, 244, 17, 84, 167, 2, 93, 185, 4, 168, 2, 93, 144, 121, 8, 145, 171, 168, 2, 93, 144, 121, 8, 145, 171, 239, 59, 122, 8, 162, 111, 222, 244, 239, 59, 122, 8, 162, 111, 222, 244, 185, 4, 78, 31, 65, 92, 42, 208, 5, 25, 78, 31, 65, 92, 42, 208, 5, 25, 71, 117, 42, 208, 190, 163, 135, 16, 71, 117, 42, 208, 190, 163, 135, 16, 185, 4, 164, 135, 16, 185, 230, 77, 31, 65, 164, 135, 16, 185, 230, 77, 31, 65, 48, 250, 230, 77, 239, 70, 117, 42, 48, 250, 230, 77, 239, 70, 117, 42, 185, 4, 11, 116, 65, 134, 33, 68, 174, 98, 11, 116, 65, 134, 33, 68, 174, 98, 240, 232, 33, 68, 190, 121, 211, 71, 240, 232, 33, 68, 190, 121, 211, 71, 185, 4, 122, 211, 71, 16, 157, 10, 116, 65, 122, 211, 71, 16, 157, 10, 116, 65, 188, 81, 157, 10, 184, 239, 232, 33, 188, 81, 157, 10, 184, 239, 232, 33, 185, 4, 31, 66, 228, 42, 55, 125, 4, 113, 31, 66, 228, 42, 55, 125, 4, 113, 233, 155, 55, 125, 27, 213, 169, 64, 233, 155, 55, 125, 27, 213, 169, 64, 185, 4, 214, 169, 64, 23, 142, 30, 66, 228, 214, 169, 64, 23, 142, 30, 66, 228, 131, 251, 142, 30, 191, 232, 155, 55, 131, 251, 142, 30, 191, 232, 155, 55, 185, 4, 234, 35, 136, 11, 5, 186, 32, 67, 234, 35, 136, 11, 5, 186, 32, 67, 169, 78, 5, 186, 119, 244, 16, 34, 169, 78, 5, 186, 119, 244, 16, 34, 185, 4, 245, 16, 34, 87, 188, 233, 35, 136, 245, 16, 34, 87, 188, 233, 35, 136, 70, 223, 188, 233, 221, 168, 78, 5, 70, 223, 188, 233, 221, 168, 78, 5, 185, 4, 85, 160, 11, 50, 15, 33, 114, 21, 85, 160, 11, 50, 15, 33, 114, 21, 126, 71, 15, 33, 244, 205, 155, 62, 126, 71, 15, 33, 244, 205, 155, 62, 185, 4, 206, 155, 62, 130, 234, 84, 160, 11, 206, 155, 62, 130, 234, 84, 160, 11, 223, 141, 234, 84, 193, 125, 71, 15, 223, 141, 234, 84, 193, 125, 71, 15, 185, 4, 68, 136, 92, 197, 166, 143, 32, 46, 68, 136, 92, 197, 166, 143, 32, 46, 126, 243, 166, 143, 162, 58, 21, 232, 126, 243, 166, 143, 162, 58, 21, 232, 185, 4, 59, 21, 232, 130, 209, 67, 136, 92, 59, 21, 232, 130, 209, 67, 136, 92, 113, 223, 209, 67, 23, 125, 243, 166, 113, 223, 209, 67, 23, 125, 243, 166, 185, 4, 112, 250, 8, 226, 82, 129, 46, 200, 112, 250, 8, 226, 82, 129, 46, 200, 56, 170, 83, 129, 246, 29, 61, 132, 56, 170, 83, 129, 246, 29, 61, 132, 185, 4, 30, 61, 132, 200, 55, 111, 250, 8, 30, 61, 132, 200, 55, 111, 250, 8, 127, 209, 55, 111, 123, 55, 170, 83, 127, 209, 55, 111, 123, 55, 170, 83, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 1, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 48, 4, 29, 0, 118, 191, 145, 119, 191, 168, 168, 154, 154, 160, 160, 160, 160, 211, 2, 0, 168, 168, 154, 154, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 248, 192, 115, 116, 146, 254, 70, 31, 248, 192, 115, 116, 146, 254, 70, 31, 248, 192, 115, 116, 146, 254, 70, 31, 248, 192, 115, 116, 146, 254, 70, 31, 185, 4, 187, 147, 146, 254, 139, 139, 117, 64, 187, 147, 146, 254, 139, 139, 117, 64, 187, 147, 146, 254, 139, 139, 117, 64, 187, 147, 146, 254, 139, 139, 117, 64, 185, 4, 140, 117, 64, 69, 224, 247, 192, 115, 140, 117, 64, 69, 224, 247, 192, 115, 140, 117, 64, 69, 224, 247, 192, 115, 140, 117, 64, 69, 224, 247, 192, 115, 185, 4, 2, 185, 224, 247, 190, 186, 147, 146, 2, 185, 224, 247, 190, 186, 147, 146, 2, 185, 224, 247, 190, 186, 147, 146, 2, 185, 224, 247, 190, 186, 147, 146, 185, 4, 42, 233, 111, 244, 87, 7, 84, 196, 42, 233, 111, 244, 87, 7, 84, 196, 42, 233, 111, 244, 87, 7, 84, 196, 42, 233, 111, 244, 87, 7, 84, 196, 185, 4, 196, 184, 88, 7, 144, 11, 126, 15, 196, 184, 88, 7, 144, 11, 126, 15, 196, 184, 88, 7, 144, 11, 126, 15, 196, 184, 88, 7, 144, 11, 126, 15, 185, 4, 12, 126, 15, 60, 59, 41, 233, 111, 12, 126, 15, 60, 59, 41, 233, 111, 12, 126, 15, 60, 59, 41, 233, 111, 12, 126, 15, 60, 59, 41, 233, 111, 185, 4, 249, 171, 59, 41, 240, 195, 184, 88, 249, 171, 59, 41, 240, 195, 184, 88, 249, 171, 59, 41, 240, 195, 184, 88, 249, 171, 59, 41, 240, 195, 184, 88, 185, 4, 215, 1, 21, 177, 222, 3, 207, 209, 215, 1, 21, 177, 222, 3, 207, 209, 215, 1, 21, 177, 222, 3, 207, 209, 215, 1, 21, 177, 222, 3, 207, 209, 185, 4, 229, 130, 223, 3, 234, 78, 74, 250, 229, 130, 223, 3, 234, 78, 74, 250, 229, 130, 223, 3, 234, 78, 74, 250, 229, 130, 223, 3, 234, 78, 74, 250, 185, 4, 79, 74, 250, 27, 46, 214, 1, 21, 79, 74, 250, 27, 46, 214, 1, 21, 79, 74, 250, 27, 46, 214, 1, 21, 79, 74, 250, 27, 46, 214, 1, 21, 185, 4, 253, 48, 46, 214, 4, 228, 130, 223, 253, 48, 46, 214, 4, 228, 130, 223, 253, 48, 46, 214, 4, 228, 130, 223, 253, 48, 46, 214, 4, 228, 130, 223, 185, 4, 249, 61, 240, 28, 164, 164, 191, 209, 249, 61, 240, 28, 164, 164, 191, 209, 249, 61, 240, 28, 164, 164, 191, 209, 249, 61, 240, 28, 164, 164, 191, 209, 185, 4, 176, 238, 164, 164, 15, 227, 98, 29, 176, 238, 164, 164, 15, 227, 98, 29, 176, 238, 164, 164, 15, 227, 98, 29, 176, 238, 164, 164, 15, 227, 98, 29, 185, 4, 228, 98, 29, 80, 45, 248, 61, 240, 228, 98, 29, 80, 45, 248, 61, 240, 228, 98, 29, 80, 45, 248, 61, 240, 228, 98, 29, 80, 45, 248, 61, 240, 185, 4, 92, 64, 46, 248, 225, 175, 238, 164, 92, 64, 46, 248, 225, 175, 238, 164, 92, 64, 46, 248, 225, 175, 238, 164, 92, 64, 46, 248, 225, 175, 238, 164, 185, 4, 38, 253, 141, 62, 234, 128, 138, 216, 38, 253, 141, 62, 234, 128, 138, 216, 38, 253, 141, 62, 234, 128, 138, 216, 38, 253, 141, 62, 234, 128, 138, 216, 185, 4, 25, 23, 235, 128, 113, 193, 239, 129, 25, 23, 235, 128, 113, 193, 239, 129, 25, 23, 235, 128, 113, 193, 239, 129, 25, 23, 235, 128, 113, 193, 239, 129, 185, 4, 194, 239, 129, 231, 38, 37, 253, 141, 194, 239, 129, 231, 38, 37, 253, 141, 194, 239, 129, 231, 38, 37, 253, 141, 194, 239, 129, 231, 38, 37, 253, 141, 185, 4, 128, 117, 39, 37, 125, 24, 23, 235, 128, 117, 39, 37, 125, 24, 23, 235, 128, 117, 39, 37, 125, 24, 23, 235, 128, 117, 39, 37, 125, 24, 23, 235, 185, 4, 178, 14, 168, 136, 251, 30, 120, 142, 178, 14, 168, 136, 251, 30, 120, 142, 178, 14, 168, 136, 251, 30, 120, 142, 178, 14, 168, 136, 251, 30, 120, 142, 185, 4, 33, 23, 252, 30, 87, 119, 82, 210, 33, 23, 252, 30, 87, 119, 82, 210, 33, 23, 252, 30, 87, 119, 82, 210, 33, 23, 252, 30, 87, 119, 82, 210, 185, 4, 120, 82, 210, 223, 112, 177, 14, 168, 120, 82, 210, 223, 112, 177, 14, 168, 120, 82, 210, 223, 112, 177, 14, 168, 120, 82, 210, 223, 112, 177, 14, 168, 185, 4, 226, 135, 113, 177, 44, 32, 23, 252, 226, 135, 113, 177, 44, 32, 23, 252, 226, 135, 113, 177, 44, 32, 23, 252, 226, 135, 113, 177, 44, 32, 23, 252, 185, 4, 192, 7, 158, 163, 147, 244, 55, 250, 192, 7, 158, 163, 147, 244, 55, 250, 192, 7, 158, 163, 147, 244, 55, 250, 192, 7, 158, 163, 147, 244, 55, 250, 185, 4, 214, 157, 148, 244, 97, 92, 172, 3, 214, 157, 148, 244, 97, 92, 172, 3, 214, 157, 148, 244, 97, 92, 172, 3, 214, 157, 148, 244, 97, 92, 172, 3, 185, 4, 93, 172, 3, 42, 5, 191, 7, 158, 93, 172, 3, 42, 5, 191, 7, 158, 93, 172, 3, 42, 5, 191, 7, 158, 93, 172, 3, 42, 5, 191, 7, 158, 185, 4, 12, 200, 5, 191, 251, 213, 157, 148, 12, 200, 5, 191, 251, 213, 157, 148, 12, 200, 5, 191, 251, 213, 157, 148, 12, 200, 5, 191, 251, 213, 157, 148, 185, 4, 74, 73, 127, 163, 197, 58, 160, 34, 74, 73, 127, 163, 197, 58, 160, 34, 74, 73, 127, 163, 197, 58, 160, 34, 74, 73, 127, 163, 197, 58, 160, 34, 185, 4, 32, 198, 197, 58, 128, 92, 240, 123, 32, 198, 197, 58, 128, 92, 240, 123, 32, 198, 197, 58, 128, 92, 240, 123, 32, 198, 197, 58, 128, 92, 240, 123, 185, 4, 93, 240, 123, 224, 220, 73, 73, 127, 93, 240, 123, 224, 220, 73, 73, 127, 93, 240, 123, 224, 220, 73, 73, 127, 93, 240, 123, 224, 220, 73, 73, 127, 185, 4, 198, 95, 221, 73, 131, 31, 198, 197, 198, 95, 221, 73, 131, 31, 198, 197, 198, 95, 221, 73, 131, 31, 198, 197, 198, 95, 221, 73, 131, 31, 198, 197, 185, 4, 172, 249, 140, 46, 117, 67, 160, 178, 172, 249, 140, 46, 117, 67, 160, 178, 172, 249, 140, 46, 117, 67, 160, 178, 172, 249, 140, 46, 117, 67, 160, 178, 185, 4, 46, 225, 117, 67, 114, 209, 222, 194, 46, 225, 117, 67, 114, 209, 222, 194, 46, 225, 117, 67, 114, 209, 222, 194, 46, 225, 117, 67, 114, 209, 222, 194, 185, 4, 210, 222, 194, 210, 76, 171, 249, 140, 210, 222, 194, 210, 76, 171, 249, 140, 210, 222, 194, 210, 76, 171, 249, 140, 210, 222, 194, 210, 76, 171, 249, 140, 185, 4, 189, 95, 77, 171, 60, 45, 225, 117, 189, 95, 77, 171, 60, 45, 225, 117, 189, 95, 77, 171, 60, 45, 225, 117, 189, 95, 77, 171, 60, 45, 225, 117, 185, 4, 95, 55, 4, 42, 236, 45, 44, 237, 95, 55, 4, 42, 236, 45, 44, 237, 95, 55, 4, 42, 236, 45, 44, 237, 95, 55, 4, 42, 236, 45, 44, 237, 185, 4, 49, 23, 237, 45, 251, 213, 180, 154, 49, 23, 237, 45, 251, 213, 180, 154, 49, 23, 237, 45, 251, 213, 180, 154, 49, 23, 237, 45, 251, 213, 180, 154, 185, 4, 214, 180, 154, 207, 18, 94, 55, 4, 214, 180, 154, 207, 18, 94, 55, 4, 214, 180, 154, 207, 18, 94, 55, 4, 214, 180, 154, 207, 18, 94, 55, 4, 185, 4, 211, 211, 18, 94, 100, 48, 23, 237, 211, 211, 18, 94, 100, 48, 23, 237, 211, 211, 18, 94, 100, 48, 23, 237, 211, 211, 18, 94, 100, 48, 23, 237, 185, 4, 124, 11, 75, 123, 172, 230, 51, 186, 124, 11, 75, 123, 172, 230, 51, 186, 124, 11, 75, 123, 172, 230, 51, 186, 124, 11, 75, 123, 172, 230, 51, 186, 185, 4, 127, 53, 173, 230, 180, 132, 215, 13, 127, 53, 173, 230, 180, 132, 215, 13, 127, 53, 173, 230, 180, 132, 215, 13, 127, 53, 173, 230, 180, 132, 215, 13, 185, 4, 133, 215, 13, 129, 69, 123, 11, 75, 133, 215, 13, 129, 69, 123, 11, 75, 133, 215, 13, 129, 69, 123, 11, 75, 133, 215, 13, 129, 69, 123, 11, 75, 185, 4, 26, 204, 69, 123, 241, 126, 53, 173, 26, 204, 69, 123, 241, 126, 53, 173, 26, 204, 69, 123, 241, 126, 53, 173, 26, 204, 69, 123, 241, 126, 53, 173, 185, 4, 212, 106, 62, 163, 119, 221, 16, 168, 212, 106, 62, 163, 119, 221, 16, 168, 212, 106, 62, 163, 119, 221, 16, 168, 212, 106, 62, 163, 119, 221, 16, 168, 185, 4, 80, 75, 120, 221, 192, 92, 180, 183, 80, 75, 120, 221, 192, 92, 180, 183, 80, 75, 120, 221, 192, 92, 180, 183, 80, 75, 120, 221, 192, 92, 180, 183, 185, 4, 93, 180, 183, 176, 87, 211, 106, 62, 93, 180, 183, 176, 87, 211, 106, 62, 93, 180, 183, 176, 87, 211, 106, 62, 93, 180, 183, 176, 87, 211, 106, 62, 185, 4, 35, 239, 87, 211, 71, 79, 75, 120, 35, 239, 87, 211, 71, 79, 75, 120, 35, 239, 87, 211, 71, 79, 75, 120, 35, 239, 87, 211, 71, 79, 75, 120, 185, 4, 236, 134, 64, 101, 189, 133, 165, 61, 236, 134, 64, 101, 189, 133, 165, 61, 236, 134, 64, 101, 189, 133, 165, 61, 236, 134, 64, 101, 189, 133, 165, 61, 185, 4, 231, 162, 189, 133, 190, 154, 86, 243, 231, 162, 189, 133, 190, 154, 86, 243, 231, 162, 189, 133, 190, 154, 86, 243, 231, 162, 189, 133, 190, 154, 86, 243, 185, 4, 155, 86, 243, 25, 194, 235, 134, 64, 155, 86, 243, 25, 194, 235, 134, 64, 155, 86, 243, 25, 194, 235, 134, 64, 155, 86, 243, 25, 194, 235, 134, 64, 185, 4, 123, 90, 194, 235, 11, 230, 162, 189, 123, 90, 194, 235, 11, 230, 162, 189, 123, 90, 194, 235, 11, 230, 162, 189, 123, 90, 194, 235, 11, 230, 162, 189, 185, 4, 219, 91, 88, 218, 104, 53, 159, 209, 219, 91, 88, 218, 104, 53, 159, 209, 219, 91, 88, 218, 104, 53, 159, 209, 219, 91, 88, 218, 104, 53, 159, 209, 185, 4, 248, 171, 105, 53, 167, 37, 188, 110, 248, 171, 105, 53, 167, 37, 188, 110, 248, 171, 105, 53, 167, 37, 188, 110, 248, 171, 105, 53, 167, 37, 188, 110, 185, 4, 38, 188, 110, 8, 46, 218, 91, 88, 38, 188, 110, 8, 46, 218, 91, 88, 38, 188, 110, 8, 46, 218, 91, 88, 38, 188, 110, 8, 46, 218, 91, 88, 185, 4, 203, 96, 46, 218, 144, 247, 171, 105, 203, 96, 46, 218, 144, 247, 171, 105, 203, 96, 46, 218, 144, 247, 171, 105, 203, 96, 46, 218, 144, 247, 171, 105, 185, 4, 91, 205, 103, 116, 174, 27, 2, 149, 91, 205, 103, 116, 174, 27, 2, 149, 91, 205, 103, 116, 174, 27, 2, 149, 91, 205, 103, 116, 174, 27, 2, 149, 185, 4, 106, 9, 175, 27, 152, 139, 246, 22, 106, 9, 175, 27, 152, 139, 246, 22, 106, 9, 175, 27, 152, 139, 246, 22, 106, 9, 175, 27, 152, 139, 246, 22, 185, 4, 140, 246, 22, 150, 106, 90, 205, 103, 140, 246, 22, 150, 106, 90, 205, 103, 140, 246, 22, 150, 106, 90, 205, 103, 140, 246, 22, 150, 106, 90, 205, 103, 185, 4, 229, 253, 106, 90, 232, 105, 9, 175, 229, 253, 106, 90, 232, 105, 9, 175, 229, 253, 106, 90, 232, 105, 9, 175, 229, 253, 106, 90, 232, 105, 9, 175, 185, 4, 241, 186, 33, 80, 104, 111, 97, 105, 241, 186, 33, 80, 104, 111, 97, 105, 241, 186, 33, 80, 104, 111, 97, 105, 241, 186, 33, 80, 104, 111, 97, 105, 185, 4, 132, 185, 104, 111, 221, 175, 166, 213, 132, 185, 104, 111, 221, 175, 166, 213, 132, 185, 104, 111, 221, 175, 166, 213, 132, 185, 104, 111, 221, 175, 166, 213, 185, 4, 176, 166, 213, 124, 150, 240, 186, 33, 176, 166, 213, 124, 150, 240, 186, 33, 176, 166, 213, 124, 150, 240, 186, 33, 176, 166, 213, 124, 150, 240, 186, 33, 185, 4, 145, 158, 150, 240, 41, 131, 185, 104, 145, 158, 150, 240, 41, 131, 185, 104, 145, 158, 150, 240, 41, 131, 185, 104, 145, 158, 150, 240, 41, 131, 185, 104, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 1, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 153, 4, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 2, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 1, 0, 0, 1, 255, 254, 255, 254, 1, 0, 0, 1, 255, 254, 255, 254, 1, 0, 0, 1, 255, 254, 255, 254, 1, 0, 0, 1, 255, 254, 255, 254, 185, 4, 1, 0, 0, 1, 255, 254, 255, 254, 1, 0, 0, 1, 255, 254, 255, 254, 1, 0, 0, 1, 255, 254, 255, 254, 1, 0, 0, 1, 255, 254, 255, 254, 185, 4, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 185, 4, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 0, 0, 0, 255, 255, 254, 255, 0, 185, 4, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 185, 4, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0, 185, 4, 2, 0, 1, 0, 254, 255, 255, 255, 2, 0, 1, 0, 254, 255, 255, 255, 2, 0, 1, 0, 254, 255, 255, 255, 2, 0, 1, 0, 254, 255, 255, 255, 185, 4, 2, 0, 1, 0, 254, 255, 255, 255, 2, 0, 1, 0, 254, 255, 255, 255, 2, 0, 1, 0, 254, 255, 255, 255, 2, 0, 1, 0, 254, 255, 255, 255, 185, 4, 0, 240, 255, 239, 255, 15, 0, 0, 0, 240, 255, 239, 255, 15, 0, 0, 0, 240, 255, 239, 255, 15, 0, 0, 0, 240, 255, 239, 255, 15, 0, 0, 185, 4, 0, 240, 255, 239, 255, 15, 0, 0, 0, 240, 255, 239, 255, 15, 0, 0, 0, 240, 255, 239, 255, 15, 0, 0, 0, 240, 255, 239, 255, 15, 0, 0, 185, 4, 0, 240, 255, 15, 0, 16, 0, 0, 0, 240, 255, 15, 0, 16, 0, 0, 0, 240, 255, 15, 0, 16, 0, 0, 0, 240, 255, 15, 0, 16, 0, 0, 185, 4, 0, 240, 255, 15, 0, 16, 0, 0, 0, 240, 255, 15, 0, 16, 0, 0, 0, 240, 255, 15, 0, 16, 0, 0, 0, 240, 255, 15, 0, 16, 0, 0, 185, 4, 17, 0, 0, 0, 255, 255, 239, 255, 17, 0, 0, 0, 255, 255, 239, 255, 17, 0, 0, 0, 255, 255, 239, 255, 17, 0, 0, 0, 255, 255, 239, 255, 185, 4, 17, 0, 0, 0, 255, 255, 239, 255, 17, 0, 0, 0, 255, 255, 239, 255, 17, 0, 0, 0, 255, 255, 239, 255, 17, 0, 0, 0, 255, 255, 239, 255, 185, 4, 241, 255, 255, 255, 254, 255, 239, 255, 241, 255, 255, 255, 254, 255, 239, 255, 241, 255, 255, 255, 254, 255, 239, 255, 241, 255, 255, 255, 254, 255, 239, 255, 185, 4, 241, 255, 255, 255, 254, 255, 239, 255, 241, 255, 255, 255, 254, 255, 239, 255, 241, 255, 255, 255, 254, 255, 239, 255, 241, 255, 255, 255, 254, 255, 239, 255, 185, 4, 0, 4, 0, 0, 0, 0, 0, 4, 0, 4, 0, 0, 0, 0, 0, 4, 0, 4, 0, 0, 0, 0, 0, 4, 0, 4, 0, 0, 0, 0, 0, 4, 185, 4, 0, 4, 0, 0, 0, 0, 0, 4, 0, 4, 0, 0, 0, 0, 0, 4, 0, 4, 0, 0, 0, 0, 0, 4, 0, 4, 0, 0, 0, 0, 0, 4, 185, 4, 1, 4, 0, 0, 255, 255, 255, 251, 1, 4, 0, 0, 255, 255, 255, 251, 1, 4, 0, 0, 255, 255, 255, 251, 1, 4, 0, 0, 255, 255, 255, 251, 185, 4, 1, 4, 0, 0, 255, 255, 255, 251, 1, 4, 0, 0, 255, 255, 255, 251, 1, 4, 0, 0, 255, 255, 255, 251, 1, 4, 0, 0, 255, 255, 255, 251, 185, 4, 0, 0, 252, 255, 251, 255, 3, 0, 0, 0, 252, 255, 251, 255, 3, 0, 0, 0, 252, 255, 251, 255, 3, 0, 0, 0, 252, 255, 251, 255, 3, 0, 185, 4, 0, 0, 252, 255, 251, 255, 3, 0, 0, 0, 252, 255, 251, 255, 3, 0, 0, 0, 252, 255, 251, 255, 3, 0, 0, 0, 252, 255, 251, 255, 3, 0, 185, 4, 0, 0, 252, 255, 3, 0, 4, 0, 0, 0, 252, 255, 3, 0, 4, 0, 0, 0, 252, 255, 3, 0, 4, 0, 0, 0, 252, 255, 3, 0, 4, 0, 185, 4, 0, 0, 252, 255, 3, 0, 4, 0, 0, 0, 252, 255, 3, 0, 4, 0, 0, 0, 252, 255, 3, 0, 4, 0, 0, 0, 252, 255, 3, 0, 4, 0, 185, 4, 1, 0, 0, 64, 255, 191, 255, 191, 1, 0, 0, 64, 255, 191, 255, 191, 1, 0, 0, 64, 255, 191, 255, 191, 1, 0, 0, 64, 255, 191, 255, 191, 185, 4, 1, 0, 0, 64, 255, 191, 255, 191, 1, 0, 0, 64, 255, 191, 255, 191, 1, 0, 0, 64, 255, 191, 255, 191, 1, 0, 0, 64, 255, 191, 255, 191, 185, 4, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 185, 4, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 0, 0, 0, 192, 255, 191, 255, 63, 185, 4, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 185, 4, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 192, 255, 63, 0, 64, 0, 0, 0, 185, 4, 65, 0, 64, 0, 191, 255, 255, 255, 65, 0, 64, 0, 191, 255, 255, 255, 65, 0, 64, 0, 191, 255, 255, 255, 65, 0, 64, 0, 191, 255, 255, 255, 185, 4, 65, 0, 64, 0, 191, 255, 255, 255, 65, 0, 64, 0, 191, 255, 255, 255, 65, 0, 64, 0, 191, 255, 255, 255, 65, 0, 64, 0, 191, 255, 255, 255, 185, 4, 0, 254, 255, 253, 255, 1, 0, 0, 0, 254, 255, 253, 255, 1, 0, 0, 0, 254, 255, 253, 255, 1, 0, 0, 0, 254, 255, 253, 255, 1, 0, 0, 185, 4, 0, 254, 255, 253, 255, 1, 0, 0, 0, 254, 255, 253, 255, 1, 0, 0, 0, 254, 255, 253, 255, 1, 0, 0, 0, 254, 255, 253, 255, 1, 0, 0, 185, 4, 0, 254, 255, 1, 0, 2, 0, 0, 0, 254, 255, 1, 0, 2, 0, 0, 0, 254, 255, 1, 0, 2, 0, 0, 0, 254, 255, 1, 0, 2, 0, 0, 185, 4, 0, 254, 255, 1, 0, 2, 0, 0, 0, 254, 255, 1, 0, 2, 0, 0, 0, 254, 255, 1, 0, 2, 0, 0, 0, 254, 255, 1, 0, 2, 0, 0, 185, 4, 3, 0, 0, 0, 255, 255, 253, 255, 3, 0, 0, 0, 255, 255, 253, 255, 3, 0, 0, 0, 255, 255, 253, 255, 3, 0, 0, 0, 255, 255, 253, 255, 185, 4, 3, 0, 0, 0, 255, 255, 253, 255, 3, 0, 0, 0, 255, 255, 253, 255, 3, 0, 0, 0, 255, 255, 253, 255, 3, 0, 0, 0, 255, 255, 253, 255, 185, 4, 255, 255, 255, 255, 254, 255, 253, 255, 255, 255, 255, 255, 254, 255, 253, 255, 255, 255, 255, 255, 254, 255, 253, 255, 255, 255, 255, 255, 254, 255, 253, 255, 185, 4, 255, 255, 255, 255, 254, 255, 253, 255, 255, 255, 255, 255, 254, 255, 253, 255, 255, 255, 255, 255, 254, 255, 253, 255, 255, 255, 255, 255, 254, 255, 253, 255, 185, 4, 0, 32, 0, 0, 0, 0, 0, 32, 0, 32, 0, 0, 0, 0, 0, 32, 0, 32, 0, 0, 0, 0, 0, 32, 0, 32, 0, 0, 0, 0, 0, 32, 185, 4, 0, 32, 0, 0, 0, 0, 0, 32, 0, 32, 0, 0, 0, 0, 0, 32, 0, 32, 0, 0, 0, 0, 0, 32, 0, 32, 0, 0, 0, 0, 0, 32, 185, 4, 1, 32, 0, 0, 255, 255, 255, 223, 1, 32, 0, 0, 255, 255, 255, 223, 1, 32, 0, 0, 255, 255, 255, 223, 1, 32, 0, 0, 255, 255, 255, 223, 185, 4, 1, 32, 0, 0, 255, 255, 255, 223, 1, 32, 0, 0, 255, 255, 255, 223, 1, 32, 0, 0, 255, 255, 255, 223, 1, 32, 0, 0, 255, 255, 255, 223, 185, 4, 0, 0, 224, 255, 223, 255, 31, 0, 0, 0, 224, 255, 223, 255, 31, 0, 0, 0, 224, 255, 223, 255, 31, 0, 0, 0, 224, 255, 223, 255, 31, 0, 185, 4, 0, 0, 224, 255, 223, 255, 31, 0, 0, 0, 224, 255, 223, 255, 31, 0, 0, 0, 224, 255, 223, 255, 31, 0, 0, 0, 224, 255, 223, 255, 31, 0, 185, 4, 0, 0, 224, 255, 31, 0, 32, 0, 0, 0, 224, 255, 31, 0, 32, 0, 0, 0, 224, 255, 31, 0, 32, 0, 0, 0, 224, 255, 31, 0, 32, 0, 185, 4, 0, 0, 224, 255, 31, 0, 32, 0, 0, 0, 224, 255, 31, 0, 32, 0, 0, 0, 224, 255, 31, 0, 32, 0, 0, 0, 224, 255, 31, 0, 32, 0, 185, 4, 1, 0, 0, 8, 255, 247, 255, 247, 1, 0, 0, 8, 255, 247, 255, 247, 1, 0, 0, 8, 255, 247, 255, 247, 1, 0, 0, 8, 255, 247, 255, 247, 185, 4, 1, 0, 0, 8, 255, 247, 255, 247, 1, 0, 0, 8, 255, 247, 255, 247, 1, 0, 0, 8, 255, 247, 255, 247, 1, 0, 0, 8, 255, 247, 255, 247, 185, 4, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 185, 4, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 0, 0, 0, 248, 255, 247, 255, 7, 185, 4, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 185, 4, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 248, 255, 7, 0, 8, 0, 0, 0, 185, 4, 9, 0, 8, 0, 247, 255, 255, 255, 9, 0, 8, 0, 247, 255, 255, 255, 9, 0, 8, 0, 247, 255, 255, 255, 9, 0, 8, 0, 247, 255, 255, 255, 185, 4, 9, 0, 8, 0, 247, 255, 255, 255, 9, 0, 8, 0, 247, 255, 255, 255, 9, 0, 8, 0, 247, 255, 255, 255, 9, 0, 8, 0, 247, 255, 255, 255, 185, 4, 0, 128, 255, 127, 255, 127, 0, 0, 0, 128, 255, 127, 255, 127, 0, 0, 0, 128, 255, 127, 255, 127, 0, 0, 0, 128, 255, 127, 255, 127, 0, 0, 185, 4, 0, 128, 255, 127, 255, 127, 0, 0, 0, 128, 255, 127, 255, 127, 0, 0, 0, 128, 255, 127, 255, 127, 0, 0, 0, 128, 255, 127, 255, 127, 0, 0, 185, 4, 0, 128, 255, 127, 0, 128, 0, 0, 0, 128, 255, 127, 0, 128, 0, 0, 0, 128, 255, 127, 0, 128, 0, 0, 0, 128, 255, 127, 0, 128, 0, 0, 185, 4, 0, 128, 255, 127, 0, 128, 0, 0, 0, 128, 255, 127, 0, 128, 0, 0, 0, 128, 255, 127, 0, 128, 0, 0, 0, 128, 255, 127, 0, 128, 0, 0, 185, 4, 129, 0, 0, 0, 255, 255, 127, 255, 129, 0, 0, 0, 255, 255, 127, 255, 129, 0, 0, 0, 255, 255, 127, 255, 129, 0, 0, 0, 255, 255, 127, 255, 185, 4, 129, 0, 0, 0, 255, 255, 127, 255, 129, 0, 0, 0, 255, 255, 127, 255, 129, 0, 0, 0, 255, 255, 127, 255, 129, 0, 0, 0, 255, 255, 127, 255, 185, 4, 129, 255, 255, 255, 254, 255, 127, 255, 129, 255, 255, 255, 254, 255, 127, 255, 129, 255, 255, 255, 254, 255, 127, 255, 129, 255, 255, 255, 254, 255, 127, 255, 185, 4, 129, 255, 255, 255, 254, 255, 127, 255, 129, 255, 255, 255, 254, 255, 127, 255, 129, 255, 255, 255, 254, 255, 127, 255, 129, 255, 255, 255, 254, 255, 127, 255, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 2, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 250, 4, 7, 0, 254, 251, 4, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 2, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 254, 30, 5, 1, 0, 126, 185, 4, 1, 0, 2, 0, 255, 255, 253, 255, 1, 0, 2, 0, 255, 255, 253, 255, 1, 0, 2, 0, 255, 255, 253, 255, 1, 0, 2, 0, 255, 255, 253, 255, 254, 34, 5, 1, 0, 126, 185, 4, 1, 254, 255, 255, 254, 255, 255, 255, 1, 254, 255, 255, 254, 255, 255, 255, 1, 254, 255, 255, 254, 255, 255, 255, 1, 254, 255, 255, 254, 255, 255, 255, 254, 38, 5, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 254, 42, 5, 1, 0, 126, 185, 4, 33, 0, 0, 0, 223, 255, 255, 255, 33, 0, 0, 0, 223, 255, 255, 255, 33, 0, 0, 0, 223, 255, 255, 255, 33, 0, 0, 0, 223, 255, 255, 255, 254, 46, 5, 1, 0, 126, 185, 4, 1, 0, 224, 255, 254, 255, 255, 255, 1, 0, 224, 255, 254, 255, 255, 255, 1, 0, 224, 255, 254, 255, 255, 255, 1, 0, 224, 255, 254, 255, 255, 255, 254, 50, 5, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 254, 54, 5, 1, 0, 126, 185, 4, 1, 0, 0, 32, 255, 255, 255, 223, 1, 0, 0, 32, 255, 255, 255, 223, 1, 0, 0, 32, 255, 255, 255, 223, 1, 0, 0, 32, 255, 255, 255, 223, 254, 58, 5, 1, 0, 126, 185, 4, 249, 255, 255, 255, 254, 255, 255, 255, 249, 255, 255, 255, 254, 255, 255, 255, 249, 255, 255, 255, 254, 255, 255, 255, 249, 255, 255, 255, 254, 255, 255, 255, 254, 62, 5, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 254, 66, 5, 1, 0, 126, 185, 4, 1, 8, 0, 0, 255, 247, 255, 255, 1, 8, 0, 0, 255, 247, 255, 255, 1, 8, 0, 0, 255, 247, 255, 255, 1, 8, 0, 0, 255, 247, 255, 255, 254, 70, 5, 1, 0, 126, 185, 4, 1, 0, 0, 248, 254, 255, 255, 255, 1, 0, 0, 248, 254, 255, 255, 255, 1, 0, 0, 248, 254, 255, 255, 255, 1, 0, 0, 248, 254, 255, 255, 255, 254, 74, 5, 1, 0, 126, 185, 4, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 254, 78, 5, 1, 0, 126, 185, 4, 1, 0, 128, 0, 255, 255, 127, 255, 1, 0, 128, 0, 255, 255, 127, 255, 1, 0, 128, 0, 255, 255, 127, 255, 1, 0, 128, 0, 255, 255, 127, 255, 254, 82, 5, 1, 0, 126, 185, 4, 1, 128, 255, 255, 254, 255, 255, 255, 1, 128, 255, 255, 254, 255, 255, 255, 1, 128, 255, 255, 254, 255, 255, 255, 1, 128, 255, 255, 254, 255, 255, 255, 254, 86, 5, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 128, 254, 90, 5, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 4, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 99, 5, 7, 0, 254, 100, 5, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 2, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 5, 0, 0, 0, 251, 255, 255, 255, 5, 0, 0, 0, 251, 255, 255, 255, 5, 0, 0, 0, 251, 255, 255, 255, 5, 0, 0, 0, 251, 255, 255, 255, 254, 135, 5, 1, 0, 126, 185, 4, 1, 0, 252, 255, 254, 255, 255, 255, 1, 0, 252, 255, 254, 255, 255, 255, 1, 0, 252, 255, 254, 255, 255, 255, 1, 0, 252, 255, 254, 255, 255, 255, 254, 139, 5, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 254, 143, 5, 1, 0, 126, 185, 4, 1, 0, 0, 4, 255, 255, 255, 251, 1, 0, 0, 4, 255, 255, 255, 251, 1, 0, 0, 4, 255, 255, 255, 251, 1, 0, 0, 4, 255, 255, 255, 251, 254, 147, 5, 1, 0, 126, 185, 4, 193, 255, 255, 255, 254, 255, 255, 255, 193, 255, 255, 255, 254, 255, 255, 255, 193, 255, 255, 255, 254, 255, 255, 255, 193, 255, 255, 255, 254, 255, 255, 255, 254, 151, 5, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 0, 254, 155, 5, 1, 0, 126, 185, 4, 1, 64, 0, 0, 255, 191, 255, 255, 1, 64, 0, 0, 255, 191, 255, 255, 1, 64, 0, 0, 255, 191, 255, 255, 1, 64, 0, 0, 255, 191, 255, 255, 254, 159, 5, 1, 0, 126, 185, 4, 1, 0, 0, 192, 254, 255, 255, 255, 1, 0, 0, 192, 254, 255, 255, 255, 1, 0, 0, 192, 254, 255, 255, 255, 1, 0, 0, 192, 254, 255, 255, 255, 254, 163, 5, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 8, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 172, 5, 7, 0, 254, 173, 5, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 2, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 254, 208, 5, 1, 0, 126, 185, 4, 1, 0, 16, 0, 255, 255, 239, 255, 1, 0, 16, 0, 255, 255, 239, 255, 1, 0, 16, 0, 255, 255, 239, 255, 1, 0, 16, 0, 255, 255, 239, 255, 254, 212, 5, 1, 0, 126, 185, 4, 1, 240, 255, 255, 254, 255, 255, 255, 1, 240, 255, 255, 254, 255, 255, 255, 1, 240, 255, 255, 254, 255, 255, 255, 1, 240, 255, 255, 254, 255, 255, 255, 254, 216, 5, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 16, 254, 220, 5, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 16, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 229, 5, 7, 0, 254, 230, 5, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 2, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 1, 1, 0, 0, 255, 254, 255, 255, 1, 1, 0, 0, 255, 254, 255, 255, 1, 1, 0, 0, 255, 254, 255, 255, 1, 1, 0, 0, 255, 254, 255, 255, 254, 9, 6, 1, 0, 126, 185, 4, 1, 0, 0, 255, 254, 255, 255, 255, 1, 0, 0, 255, 254, 255, 255, 255, 1, 0, 0, 255, 254, 255, 255, 255, 1, 0, 0, 255, 254, 255, 255, 255, 254, 13, 6, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 32, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 22, 6, 7, 0, 254, 23, 6, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 2, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 155, 3, 171, 156, 3, 172, 107, 107, 108, 108, 185, 4, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 254, 58, 6, 1, 0, 126, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 186, 64, 0, 0, 0, 0, 0, 0, 0, 171, 186, 0, 0, 0, 0, 0, 0, 0, 0, 171, 254, 67, 6, 21, 0, 118, 191, 145, 119, 191, 160, 160, 160, 160, 211, 2, 0, 119, 198, 145, 118, 198, 155, 3, 171, 156, 3, 172, 107, 107, 108, 186, 0, 0, 0, 0, 0, 0, 0, 0, 167, 254, 95, 6, 8, 0, 114, 191, 211, 3, 0, 114, 198, 151, 3, 167, 108, 107, 186, 0, 0, 0, 0, 0, 0, 0, 0]),
("std::math::u64",vm_assembly::ProcedureId([16, 215, 171, 37, 69, 128, 45, 73, 105, 167, 152, 221, 87, 169, 212, 183, 244, 98, 84, 253, 101, 84, 148, 213]),"# ===== HELPER FUNCTIONS ==========================================================================

#! Asserts that both values at the top of the stack are u64 values.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
proc.u32assert4
    u32assert.2
    movup.3
    movup.3
    u32assert.2
    movup.3
    movup.3
end

# ===== ADDITION ==================================================================================

#! Performs addition of two unsigned 64 bit integers preserving the overflow.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [overflowing_flag, c_hi, c_lo, ...], where c = (a + b) % 2^64
export.overflowing_add
    swap
    movup.3
    u32overflowing_add
    movup.3
    movup.3
    u32overflowing_add3
end

#! Performs addition of two unsigned 64 bit integers discarding the overflow.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a + b) % 2^64
export.wrapping_add
    exec.overflowing_add
    drop
end

#! Performs addition of two unsigned 64 bit integers, fails when overflowing.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a + b) % 2^64
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

#! Performs subtraction of two unsigned 64 bit integers discarding the overflow.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a - b) % 2^64
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

#! Performs subtraction of two unsigned 64 bit integers, fails when underflowing.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a - b) % 2^64
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

#! Performs subtraction of two unsigned 64 bit integers preserving the overflow.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [underflowing_flag, c_hi, c_lo, ...], where c = (a - b) % 2^64
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

#! Performs multiplication of two unsigned 64 bit integers discarding the overflow.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a * b) % 2^64
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

#! Performs multiplication of two unsigned 64 bit integers preserving the overflow.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_mid_hi, c_mid_lo, c_lo, ...], where c = (a * b) % 2^64
#! This takes 18 cycles.
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

#! Performs multiplication of two unsigned 64 bit integers, fails when overflowing.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = (a * b) % 2^64
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

#! Performs less-than comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a < b, and 0 otherwise.
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

#! Performs less-than comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a < b, and 0 otherwise.
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

#! Performs greater-than comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a > b, and 0 otherwise.
#! This takes 11 cycles.
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

#! Performs greater-than comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a > b, and 0 otherwise.
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

#! Performs less-than-or-equal comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a <= b, and 0 otherwise.
export.unchecked_lte
    exec.unchecked_gt
    not
end

#! Performs less-than-or-equal comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a <= b, and 0 otherwise.
export.checked_lte
    exec.checked_gt
    not
end

#! Performs greater-than-or-equal comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a >= b, and 0 otherwise.
export.unchecked_gte
    exec.unchecked_lt
    not
end

#! Performs greater-than-or-equal comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a >= b, and 0 otherwise.
export.checked_gte
    exec.checked_lt
    not
end

#! Performs equality comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == b, and 0 otherwise.
export.unchecked_eq
    movup.2
    u32checked_eq
    swap
    movup.2
    u32checked_eq
    and
end

#! Performs equality comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == b, and 0 otherwise.
export.checked_eq
    movup.2
    u32checked_eq
    swap
    movup.2
    u32checked_eq
    and
end

#! Performs inequality comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a != b, and 0 otherwise.
export.unchecked_neq
    movup.2
    u32checked_neq
    swap
    movup.2
    u32checked_neq
    or
end

#! Performs inequality comparison of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == b, and 0 otherwise.
export.checked_neq
    exec.checked_eq
    not
end

#! Performs comparison to zero of an unsigned 64 bit integer.
#! The input value is assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == 0, and 0 otherwise.
export.unchecked_eqz
    eq.0
    swap
    eq.0
    and
end

#! Performs comparison to zero of an unsigned 64 bit integer.
#! The input value is assumed to be represented using 32 bit limbs, fails if it is not.
#! Stack transition looks as follows:
#! [a_hi, a_lo, ...] -> [c, ...], where c = 1 when a == 0, and 0 otherwise.
export.checked_eqz
    u32assert.2
    eq.0
    swap
    eq.0
    and
end

#! Compares two unsigned 64 bit integers and drop the larger one from the stack.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a when a < b, and b otherwise.
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

#! Compares two unsigned 64 bit integers and drop the larger one from the stack.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a when a < b, and b otherwise.
export.checked_min
    exec.u32assert4
    exec.unchecked_min
end

#! Compares two unsigned 64 bit integers and drop the smaller one from the stack.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a when a > b, and b otherwise.
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

#! Compares two unsigned 64 bit integers and drop the smaller one from the stack.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a when a > b, and b otherwise.
export.checked_max
    exec.u32assert4
    exec.unchecked_max
end


# ===== DIVISION ==================================================================================

#! Performs division of two unsigned 64 bit integers discarding the remainder.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a // b
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

#! Performs division of two unsigned 64 bit integers discarding the remainder.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a // b
export.checked_div
    exec.u32assert4
    exec.unchecked_div
end

# ===== MODULO OPERATION ==========================================================================

#! Performs modulo operation of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a % b
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

#! Performs modulo operation of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a % b
export.checked_mod
    exec.u32assert4
    exec.unchecked_mod
end

# ===== DIVMOD OPERATION ==========================================================================

#! Performs divmod operation of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [r_hi, r_lo, q_hi, q_lo ...], where r = a % b, q = a / b
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

#! Performs divmod operation of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [r_hi, r_lo, q_hi, q_lo ...], where r = a % b, q = a / b
export.checked_divmod
    exec.u32assert4
    exec.unchecked_divmod
end

# ===== BITWISE OPERATIONS ========================================================================

#! Performs bitwise AND of two unsigned 64-bit integers.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a AND b.
export.checked_and
    swap
    movup.3
    u32checked_and
    swap
    movup.2
    u32checked_and
end

#! Performs bitwise OR of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a OR b.
export.checked_or
    swap
    movup.3
    u32checked_or
    swap
    movup.2
    u32checked_or
end

#! Performs bitwise XOR of two unsigned 64 bit integers.
#! The input values are assumed to be represented using 32 bit limbs, fails if they are not.
#! Stack transition looks as follows:
#! [b_hi, b_lo, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a XOR b.
export.checked_xor
    swap
    movup.3
    u32checked_xor
    swap
    movup.2
    u32checked_xor
end

#! Performs left shift of one unsigned 64-bit integer using the pow2 operation.
#! The input value to be shifted is assumed to be represented using 32 bit limbs.
#! The shift value should be in the range [0, 64), otherwise it will result in an
#! error.
#! Stack transition looks as follows:
#! [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a << b mod 2^64.
#! This takes 28 cycles.
export.unchecked_shl
    pow2
    u32split
    exec.wrapping_mul
end


#! Performs right shift of one unsigned 64-bit integer using the pow2 operation.
#! The input value to be shifted is assumed to be represented using 32 bit limbs.
#! The shift value should be in the range [0, 64), otherwise it will result in an
#! error.
#! Stack transition looks as follows:
#! [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a >> b.
#! This takes 44 cycles.
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

#! Performs left shift of one unsigned 64-bit integer preserving the overflow and
#! using the pow2 operation.
#! The input value to be shifted is assumed to be represented using 32 bit limbs.
#! The shift value should be in the range [0, 64), otherwise it will result in an
#! error.
#! Stack transition looks as follows:
#! [b, a_hi, a_lo, ...] -> [d_hi, d_lo, c_hi, c_lo, ...], where (d,c) = a << b,
#! which d contains the bits shifted out.
#! This takes 35 cycles.
export.overflowing_shl
    pow2
    u32split
    exec.overflowing_mul
end

#! Performs right shift of one unsigned 64-bit integer preserving the overflow and
#! using the pow2 operation.
#! The input value to be shifted is assumed to be represented using 32 bit limbs.
#! The shift value should be in the range [0, 64), otherwise it will result in an
#! error.
#! Stack transition looks as follows:
#! [b, a_hi, a_lo, ...] -> [d_hi, d_lo, c_hi, c_lo, ...], where c = a >> b, d = a << (64 - b).
#! This takes 94 cycles.
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

#! Performs left rotation of one unsigned 64-bit integer using the pow2 operation.
#! The input value to be shifted is assumed to be represented using 32 bit limbs.
#! The shift value should be in the range [0, 64), otherwise it will result in an
#! error.
#! Stack transition looks as follows:
#! [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a << b mod 2^64.
#! This takes 35 cycles.
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

#! Performs right rotation of one unsigned 64-bit integer using the pow2 operation.
#! The input value to be shifted is assumed to be represented using 32 bit limbs.
#! The shift value should be in the range [0, 64), otherwise it will result in an
#! error.
#! Stack transition looks as follows:
#! [b, a_hi, a_lo, ...] -> [c_hi, c_lo, ...], where c = a << b mod 2^64.
#! This takes 40 cycles.
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
",&[43, 0, 10, 117, 51, 50, 97, 115, 115, 101, 114, 116, 52, 0, 0, 0, 0, 0, 6, 0, 33, 150, 150, 33, 150, 150, 15, 111, 118, 101, 114, 102, 108, 111, 119, 105, 110, 103, 95, 97, 100, 100, 40, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 112, 114, 101, 115, 101, 114, 118, 105, 110, 103, 32, 116, 104, 101, 32, 111, 118, 101, 114, 102, 108, 111, 119, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 111, 118, 101, 114, 102, 108, 111, 119, 105, 110, 103, 95, 102, 108, 97, 103, 44, 32, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 40, 97, 32, 43, 32, 98, 41, 32, 37, 32, 50, 94, 54, 52, 1, 0, 0, 6, 0, 130, 150, 41, 150, 150, 43, 12, 119, 114, 97, 112, 112, 105, 110, 103, 95, 97, 100, 100, 22, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 100, 105, 115, 99, 97, 114, 100, 105, 110, 103, 32, 116, 104, 101, 32, 111, 118, 101, 114, 102, 108, 111, 119, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 40, 97, 32, 43, 32, 98, 41, 32, 37, 32, 50, 94, 54, 52, 1, 0, 0, 2, 0, 211, 1, 0, 107, 11, 99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100, 20, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 44, 32, 102, 97, 105, 108, 115, 32, 119, 104, 101, 110, 32, 111, 118, 101, 114, 102, 108, 111, 119, 105, 110, 103, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 40, 97, 32, 43, 32, 98, 41, 32, 37, 32, 50, 94, 54, 52, 1, 0, 0, 10, 0, 130, 150, 33, 41, 150, 150, 33, 43, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 119, 114, 97, 112, 112, 105, 110, 103, 95, 115, 117, 98, 25, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 115, 117, 98, 116, 114, 97, 99, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 100, 105, 115, 99, 97, 114, 100, 105, 110, 103, 32, 116, 104, 101, 32, 111, 118, 101, 114, 102, 108, 111, 119, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 40, 97, 32, 45, 32, 98, 41, 32, 37, 32, 50, 94, 54, 52, 1, 0, 0, 10, 0, 150, 149, 49, 150, 150, 49, 107, 130, 49, 107, 11, 99, 104, 101, 99, 107, 101, 100, 95, 115, 117, 98, 24, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 115, 117, 98, 116, 114, 97, 99, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 44, 32, 102, 97, 105, 108, 115, 32, 119, 104, 101, 110, 32, 117, 110, 100, 101, 114, 102, 108, 111, 119, 105, 110, 103, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 40, 97, 32, 45, 32, 98, 41, 32, 37, 32, 50, 94, 54, 52, 1, 0, 0, 14, 0, 150, 149, 33, 49, 150, 150, 33, 49, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 49, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 111, 118, 101, 114, 102, 108, 111, 119, 105, 110, 103, 95, 115, 117, 98, 44, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 115, 117, 98, 116, 114, 97, 99, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 112, 114, 101, 115, 101, 114, 118, 105, 110, 103, 32, 116, 104, 101, 32, 111, 118, 101, 114, 102, 108, 111, 119, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 117, 110, 100, 101, 114, 102, 108, 111, 119, 105, 110, 103, 95, 102, 108, 97, 103, 44, 32, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 40, 97, 32, 45, 32, 98, 41, 32, 37, 32, 50, 94, 54, 52, 1, 0, 0, 11, 0, 150, 149, 49, 150, 150, 49, 130, 149, 49, 149, 19, 12, 119, 114, 97, 112, 112, 105, 110, 103, 95, 109, 117, 108, 28, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 100, 105, 115, 99, 97, 114, 100, 105, 110, 103, 32, 116, 104, 101, 32, 111, 118, 101, 114, 102, 108, 111, 119, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 40, 97, 32, 42, 32, 98, 41, 32, 37, 32, 50, 94, 54, 52, 1, 0, 0, 11, 0, 113, 112, 55, 151, 151, 57, 107, 150, 150, 57, 107, 15, 111, 118, 101, 114, 102, 108, 111, 119, 105, 110, 103, 95, 109, 117, 108, 70, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 112, 114, 101, 115, 101, 114, 118, 105, 110, 103, 32, 116, 104, 101, 32, 111, 118, 101, 114, 102, 108, 111, 119, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 109, 105, 100, 95, 104, 105, 44, 32, 99, 95, 109, 105, 100, 95, 108, 111, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 40, 97, 32, 42, 32, 98, 41, 32, 37, 32, 50, 94, 54, 52, 10, 84, 104, 105, 115, 32, 116, 97, 107, 101, 115, 32, 49, 56, 32, 99, 121, 99, 108, 101, 115, 46, 1, 0, 0, 18, 0, 113, 112, 55, 114, 151, 57, 130, 152, 114, 57, 152, 152, 57, 150, 149, 41, 149, 3, 11, 99, 104, 101, 99, 107, 101, 100, 95, 109, 117, 108, 26, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 44, 32, 102, 97, 105, 108, 115, 32, 119, 104, 101, 110, 32, 111, 118, 101, 114, 102, 108, 111, 119, 105, 110, 103, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 40, 97, 32, 42, 32, 98, 41, 32, 37, 32, 50, 94, 54, 52, 1, 0, 0, 22, 0, 113, 112, 33, 55, 114, 151, 57, 130, 152, 114, 57, 152, 152, 33, 57, 150, 149, 41, 3, 3, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 108, 116, 17, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 108, 101, 115, 115, 45, 116, 104, 97, 110, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 60, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 11, 0, 150, 149, 49, 166, 107, 49, 130, 22, 0, 0, 0, 0, 0, 0, 0, 0, 149, 18, 19, 10, 99, 104, 101, 99, 107, 101, 100, 95, 108, 116, 15, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 108, 101, 115, 115, 45, 116, 104, 97, 110, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 60, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 13, 0, 150, 149, 33, 49, 166, 107, 33, 49, 130, 22, 0, 0, 0, 0, 0, 0, 0, 0, 149, 18, 19, 12, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 103, 116, 42, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 103, 114, 101, 97, 116, 101, 114, 45, 116, 104, 97, 110, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 62, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 10, 84, 104, 105, 115, 32, 116, 97, 107, 101, 115, 32, 49, 49, 32, 99, 121, 99, 108, 101, 115, 46, 1, 0, 0, 11, 0, 149, 49, 149, 150, 49, 130, 107, 149, 22, 0, 0, 0, 0, 0, 0, 0, 0, 18, 19, 10, 99, 104, 101, 99, 107, 101, 100, 95, 103, 116, 18, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 103, 114, 101, 97, 116, 101, 114, 45, 116, 104, 97, 110, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 62, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 13, 0, 149, 33, 49, 149, 150, 33, 49, 130, 107, 149, 22, 0, 0, 0, 0, 0, 0, 0, 0, 18, 19, 13, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 108, 116, 101, 27, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 108, 101, 115, 115, 45, 116, 104, 97, 110, 45, 111, 114, 45, 101, 113, 117, 97, 108, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 60, 61, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 2, 0, 211, 12, 0, 17, 11, 99, 104, 101, 99, 107, 101, 100, 95, 108, 116, 101, 25, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 108, 101, 115, 115, 45, 116, 104, 97, 110, 45, 111, 114, 45, 101, 113, 117, 97, 108, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 60, 61, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 2, 0, 211, 13, 0, 17, 13, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 103, 116, 101, 30, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 103, 114, 101, 97, 116, 101, 114, 45, 116, 104, 97, 110, 45, 111, 114, 45, 101, 113, 117, 97, 108, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 62, 61, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 2, 0, 211, 10, 0, 17, 11, 99, 104, 101, 99, 107, 101, 100, 95, 103, 116, 101, 28, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 103, 114, 101, 97, 116, 101, 114, 45, 116, 104, 97, 110, 45, 111, 114, 45, 101, 113, 117, 97, 108, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 62, 61, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 2, 0, 211, 11, 0, 17, 12, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 101, 113, 17, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 61, 61, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 6, 0, 149, 91, 130, 149, 91, 18, 10, 99, 104, 101, 99, 107, 101, 100, 95, 101, 113, 15, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 61, 61, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 6, 0, 149, 91, 130, 149, 91, 18, 13, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 110, 101, 113, 19, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 105, 110, 101, 113, 117, 97, 108, 105, 116, 121, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 33, 61, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 6, 0, 149, 93, 130, 149, 93, 19, 11, 99, 104, 101, 99, 107, 101, 100, 95, 110, 101, 113, 17, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 105, 110, 101, 113, 117, 97, 108, 105, 116, 121, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 61, 61, 32, 98, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 2, 0, 211, 19, 0, 17, 13, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 101, 113, 122, 0, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 116, 111, 32, 122, 101, 114, 111, 32, 111, 102, 32, 97, 110, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 32, 105, 115, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 61, 61, 32, 48, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 4, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 130, 22, 0, 0, 0, 0, 0, 0, 0, 0, 18, 11, 99, 104, 101, 99, 107, 101, 100, 95, 101, 113, 122, 251, 0, 80, 101, 114, 102, 111, 114, 109, 115, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 116, 111, 32, 122, 101, 114, 111, 32, 111, 102, 32, 97, 110, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 32, 105, 115, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 49, 32, 119, 104, 101, 110, 32, 97, 32, 61, 61, 32, 48, 44, 32, 97, 110, 100, 32, 48, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 5, 0, 33, 22, 0, 0, 0, 0, 0, 0, 0, 0, 130, 22, 0, 0, 0, 0, 0, 0, 0, 0, 18, 13, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 109, 105, 110, 41, 1, 67, 111, 109, 112, 97, 114, 101, 115, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 97, 110, 100, 32, 100, 114, 111, 112, 32, 116, 104, 101, 32, 108, 97, 114, 103, 101, 114, 32, 111, 110, 101, 32, 102, 114, 111, 109, 32, 116, 104, 101, 32, 115, 116, 97, 99, 107, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 119, 104, 101, 110, 32, 97, 32, 60, 32, 98, 44, 32, 97, 110, 100, 32, 98, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 8, 0, 126, 211, 12, 0, 151, 150, 112, 183, 166, 183, 11, 99, 104, 101, 99, 107, 101, 100, 95, 109, 105, 110, 39, 1, 67, 111, 109, 112, 97, 114, 101, 115, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 97, 110, 100, 32, 100, 114, 111, 112, 32, 116, 104, 101, 32, 108, 97, 114, 103, 101, 114, 32, 111, 110, 101, 32, 102, 114, 111, 109, 32, 116, 104, 101, 32, 115, 116, 97, 99, 107, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 119, 104, 101, 110, 32, 97, 32, 60, 32, 98, 44, 32, 97, 110, 100, 32, 98, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 2, 0, 211, 0, 0, 211, 24, 0, 13, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 109, 97, 120, 42, 1, 67, 111, 109, 112, 97, 114, 101, 115, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 97, 110, 100, 32, 100, 114, 111, 112, 32, 116, 104, 101, 32, 115, 109, 97, 108, 108, 101, 114, 32, 111, 110, 101, 32, 102, 114, 111, 109, 32, 116, 104, 101, 32, 115, 116, 97, 99, 107, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 119, 104, 101, 110, 32, 97, 32, 62, 32, 98, 44, 32, 97, 110, 100, 32, 98, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 8, 0, 126, 211, 10, 0, 151, 150, 112, 183, 166, 183, 11, 99, 104, 101, 99, 107, 101, 100, 95, 109, 97, 120, 40, 1, 67, 111, 109, 112, 97, 114, 101, 115, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 97, 110, 100, 32, 100, 114, 111, 112, 32, 116, 104, 101, 32, 115, 109, 97, 108, 108, 101, 114, 32, 111, 110, 101, 32, 102, 114, 111, 109, 32, 116, 104, 101, 32, 115, 116, 97, 99, 107, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 119, 104, 101, 110, 32, 97, 32, 62, 32, 98, 44, 32, 97, 110, 100, 32, 98, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 46, 1, 0, 0, 2, 0, 211, 0, 0, 211, 26, 0, 13, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118, 15, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 100, 105, 118, 105, 115, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 100, 105, 115, 99, 97, 114, 100, 105, 110, 103, 32, 116, 104, 101, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 47, 47, 32, 98, 1, 0, 0, 41, 0, 205, 186, 2, 0, 0, 0, 0, 0, 0, 0, 33, 113, 112, 55, 114, 114, 57, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 113, 57, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 113, 7, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 186, 2, 0, 0, 0, 0, 0, 0, 0, 33, 154, 154, 113, 113, 211, 12, 0, 0, 130, 150, 41, 150, 150, 43, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 1, 150, 1, 11, 99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118, 13, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 100, 105, 118, 105, 115, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 100, 105, 115, 99, 97, 114, 100, 105, 110, 103, 32, 116, 104, 101, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 47, 47, 32, 98, 1, 0, 0, 2, 0, 211, 0, 0, 211, 28, 0, 13, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 109, 111, 100, 253, 0, 80, 101, 114, 102, 111, 114, 109, 115, 32, 109, 111, 100, 117, 108, 111, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 37, 32, 98, 1, 0, 0, 41, 0, 205, 186, 2, 0, 0, 0, 0, 0, 0, 0, 33, 113, 112, 55, 114, 151, 57, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 113, 57, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 113, 150, 7, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 186, 2, 0, 0, 0, 0, 0, 0, 0, 33, 152, 152, 113, 113, 211, 12, 0, 0, 111, 151, 41, 151, 113, 43, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 1, 150, 1, 11, 99, 104, 101, 99, 107, 101, 100, 95, 109, 111, 100, 251, 0, 80, 101, 114, 102, 111, 114, 109, 115, 32, 109, 111, 100, 117, 108, 111, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 37, 32, 98, 1, 0, 0, 2, 0, 211, 0, 0, 211, 30, 0, 16, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118, 109, 111, 100, 19, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 100, 105, 118, 109, 111, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 114, 95, 104, 105, 44, 32, 114, 95, 108, 111, 44, 32, 113, 95, 104, 105, 44, 32, 113, 95, 108, 111, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 114, 32, 61, 32, 97, 32, 37, 32, 98, 44, 32, 113, 32, 61, 32, 97, 32, 47, 32, 98, 1, 0, 0, 41, 0, 205, 186, 2, 0, 0, 0, 0, 0, 0, 0, 33, 113, 112, 55, 114, 114, 57, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 113, 57, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 113, 7, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 186, 2, 0, 0, 0, 0, 0, 0, 0, 33, 154, 154, 113, 113, 211, 12, 0, 0, 111, 151, 41, 151, 113, 43, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 1, 152, 1, 14, 99, 104, 101, 99, 107, 101, 100, 95, 100, 105, 118, 109, 111, 100, 17, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 100, 105, 118, 109, 111, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 114, 95, 104, 105, 44, 32, 114, 95, 108, 111, 44, 32, 113, 95, 104, 105, 44, 32, 113, 95, 108, 111, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 114, 32, 61, 32, 97, 32, 37, 32, 98, 44, 32, 113, 32, 61, 32, 97, 32, 47, 32, 98, 1, 0, 0, 2, 0, 211, 0, 0, 211, 32, 0, 11, 99, 104, 101, 99, 107, 101, 100, 95, 97, 110, 100, 251, 0, 80, 101, 114, 102, 111, 114, 109, 115, 32, 98, 105, 116, 119, 105, 115, 101, 32, 65, 78, 68, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 45, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 65, 78, 68, 32, 98, 46, 1, 0, 0, 6, 0, 130, 150, 71, 130, 149, 71, 10, 99, 104, 101, 99, 107, 101, 100, 95, 111, 114, 247, 0, 80, 101, 114, 102, 111, 114, 109, 115, 32, 98, 105, 116, 119, 105, 115, 101, 32, 79, 82, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 79, 82, 32, 98, 46, 1, 0, 0, 6, 0, 130, 150, 72, 130, 149, 72, 11, 99, 104, 101, 99, 107, 101, 100, 95, 120, 111, 114, 249, 0, 80, 101, 114, 102, 111, 114, 109, 115, 32, 98, 105, 116, 119, 105, 115, 101, 32, 88, 79, 82, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 102, 97, 105, 108, 115, 32, 105, 102, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 95, 104, 105, 44, 32, 98, 95, 108, 111, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 88, 79, 82, 32, 98, 46, 1, 0, 0, 6, 0, 130, 150, 73, 130, 149, 73, 13, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 108, 112, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 108, 101, 102, 116, 32, 115, 104, 105, 102, 116, 32, 111, 102, 32, 111, 110, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 45, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 117, 115, 105, 110, 103, 32, 116, 104, 101, 32, 112, 111, 119, 50, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 98, 101, 32, 115, 104, 105, 102, 116, 101, 100, 32, 105, 115, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 46, 10, 84, 104, 101, 32, 115, 104, 105, 102, 116, 32, 118, 97, 108, 117, 101, 32, 115, 104, 111, 117, 108, 100, 32, 98, 101, 32, 105, 110, 32, 116, 104, 101, 32, 114, 97, 110, 103, 101, 32, 91, 48, 44, 32, 54, 52, 41, 44, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 32, 105, 116, 32, 119, 105, 108, 108, 32, 114, 101, 115, 117, 108, 116, 32, 105, 110, 32, 97, 110, 10, 101, 114, 114, 111, 114, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 60, 60, 32, 98, 32, 109, 111, 100, 32, 50, 94, 54, 52, 46, 10, 84, 104, 105, 115, 32, 116, 97, 107, 101, 115, 32, 50, 56, 32, 99, 121, 99, 108, 101, 115, 46, 1, 0, 0, 3, 0, 13, 35, 211, 7, 0, 13, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 115, 104, 114, 104, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 114, 105, 103, 104, 116, 32, 115, 104, 105, 102, 116, 32, 111, 102, 32, 111, 110, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 45, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 117, 115, 105, 110, 103, 32, 116, 104, 101, 32, 112, 111, 119, 50, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 98, 101, 32, 115, 104, 105, 102, 116, 101, 100, 32, 105, 115, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 46, 10, 84, 104, 101, 32, 115, 104, 105, 102, 116, 32, 118, 97, 108, 117, 101, 32, 115, 104, 111, 117, 108, 100, 32, 98, 101, 32, 105, 110, 32, 116, 104, 101, 32, 114, 97, 110, 103, 101, 32, 91, 48, 44, 32, 54, 52, 41, 44, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 32, 105, 116, 32, 119, 105, 108, 108, 32, 114, 101, 115, 117, 108, 116, 32, 105, 110, 32, 97, 110, 10, 101, 114, 114, 111, 114, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 62, 62, 32, 98, 46, 10, 84, 104, 105, 115, 32, 116, 97, 107, 101, 115, 32, 52, 52, 32, 99, 121, 99, 108, 101, 115, 46, 1, 0, 0, 28, 0, 13, 35, 111, 3, 149, 130, 69, 150, 150, 110, 22, 0, 0, 0, 0, 0, 0, 0, 0, 49, 17, 167, 110, 167, 69, 107, 185, 1, 0, 0, 0, 0, 1, 0, 0, 0, 115, 7, 151, 9, 149, 7, 3, 149, 181, 15, 111, 118, 101, 114, 102, 108, 111, 119, 105, 110, 103, 95, 115, 104, 108, 186, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 108, 101, 102, 116, 32, 115, 104, 105, 102, 116, 32, 111, 102, 32, 111, 110, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 45, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 114, 101, 115, 101, 114, 118, 105, 110, 103, 32, 116, 104, 101, 32, 111, 118, 101, 114, 102, 108, 111, 119, 32, 97, 110, 100, 10, 117, 115, 105, 110, 103, 32, 116, 104, 101, 32, 112, 111, 119, 50, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 98, 101, 32, 115, 104, 105, 102, 116, 101, 100, 32, 105, 115, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 46, 10, 84, 104, 101, 32, 115, 104, 105, 102, 116, 32, 118, 97, 108, 117, 101, 32, 115, 104, 111, 117, 108, 100, 32, 98, 101, 32, 105, 110, 32, 116, 104, 101, 32, 114, 97, 110, 103, 101, 32, 91, 48, 44, 32, 54, 52, 41, 44, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 32, 105, 116, 32, 119, 105, 108, 108, 32, 114, 101, 115, 117, 108, 116, 32, 105, 110, 32, 97, 110, 10, 101, 114, 114, 111, 114, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 100, 95, 104, 105, 44, 32, 100, 95, 108, 111, 44, 32, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 40, 100, 44, 99, 41, 32, 61, 32, 97, 32, 60, 60, 32, 98, 44, 10, 119, 104, 105, 99, 104, 32, 100, 32, 99, 111, 110, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 98, 105, 116, 115, 32, 115, 104, 105, 102, 116, 101, 100, 32, 111, 117, 116, 46, 10, 84, 104, 105, 115, 32, 116, 97, 107, 101, 115, 32, 51, 53, 32, 99, 121, 99, 108, 101, 115, 46, 1, 0, 0, 3, 0, 13, 35, 211, 8, 0, 15, 111, 118, 101, 114, 102, 108, 111, 119, 105, 110, 103, 95, 115, 104, 114, 163, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 114, 105, 103, 104, 116, 32, 115, 104, 105, 102, 116, 32, 111, 102, 32, 111, 110, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 45, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 114, 101, 115, 101, 114, 118, 105, 110, 103, 32, 116, 104, 101, 32, 111, 118, 101, 114, 102, 108, 111, 119, 32, 97, 110, 100, 10, 117, 115, 105, 110, 103, 32, 116, 104, 101, 32, 112, 111, 119, 50, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 98, 101, 32, 115, 104, 105, 102, 116, 101, 100, 32, 105, 115, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 46, 10, 84, 104, 101, 32, 115, 104, 105, 102, 116, 32, 118, 97, 108, 117, 101, 32, 115, 104, 111, 117, 108, 100, 32, 98, 101, 32, 105, 110, 32, 116, 104, 101, 32, 114, 97, 110, 103, 101, 32, 91, 48, 44, 32, 54, 52, 41, 44, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 32, 105, 116, 32, 119, 105, 108, 108, 32, 114, 101, 115, 117, 108, 116, 32, 105, 110, 32, 97, 110, 10, 101, 114, 114, 111, 114, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 100, 95, 104, 105, 44, 32, 100, 95, 108, 111, 44, 32, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 62, 62, 32, 98, 44, 32, 100, 32, 61, 32, 97, 32, 60, 60, 32, 40, 54, 52, 32, 45, 32, 98, 41, 46, 10, 84, 104, 105, 115, 32, 116, 97, 107, 101, 115, 32, 57, 52, 32, 99, 121, 99, 108, 101, 115, 46, 1, 0, 0, 16, 0, 185, 1, 64, 0, 0, 0, 0, 0, 0, 0, 111, 5, 113, 113, 113, 211, 38, 0, 168, 168, 109, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 153, 22, 0, 0, 0, 0, 0, 0, 0, 0, 184, 107, 211, 37, 0, 14, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 114, 111, 116, 108, 115, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 108, 101, 102, 116, 32, 114, 111, 116, 97, 116, 105, 111, 110, 32, 111, 102, 32, 111, 110, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 45, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 117, 115, 105, 110, 103, 32, 116, 104, 101, 32, 112, 111, 119, 50, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 98, 101, 32, 115, 104, 105, 102, 116, 101, 100, 32, 105, 115, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 46, 10, 84, 104, 101, 32, 115, 104, 105, 102, 116, 32, 118, 97, 108, 117, 101, 32, 115, 104, 111, 117, 108, 100, 32, 98, 101, 32, 105, 110, 32, 116, 104, 101, 32, 114, 97, 110, 103, 101, 32, 91, 48, 44, 32, 54, 52, 41, 44, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 32, 105, 116, 32, 119, 105, 108, 108, 32, 114, 101, 115, 117, 108, 116, 32, 105, 110, 32, 97, 110, 10, 101, 114, 114, 111, 114, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 60, 60, 32, 98, 32, 109, 111, 100, 32, 50, 94, 54, 52, 46, 10, 84, 104, 105, 115, 32, 116, 97, 107, 101, 115, 32, 51, 53, 32, 99, 121, 99, 108, 101, 115, 46, 1, 0, 0, 20, 0, 185, 1, 31, 0, 0, 0, 0, 0, 0, 0, 111, 49, 130, 107, 166, 185, 1, 31, 0, 0, 0, 0, 0, 0, 0, 71, 13, 110, 150, 55, 150, 150, 57, 149, 3, 130, 149, 181, 14, 117, 110, 99, 104, 101, 99, 107, 101, 100, 95, 114, 111, 116, 114, 116, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 114, 105, 103, 104, 116, 32, 114, 111, 116, 97, 116, 105, 111, 110, 32, 111, 102, 32, 111, 110, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 54, 52, 45, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 117, 115, 105, 110, 103, 32, 116, 104, 101, 32, 112, 111, 119, 50, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 98, 101, 32, 115, 104, 105, 102, 116, 101, 100, 32, 105, 115, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 46, 10, 84, 104, 101, 32, 115, 104, 105, 102, 116, 32, 118, 97, 108, 117, 101, 32, 115, 104, 111, 117, 108, 100, 32, 98, 101, 32, 105, 110, 32, 116, 104, 101, 32, 114, 97, 110, 103, 101, 32, 91, 48, 44, 32, 54, 52, 41, 44, 32, 111, 116, 104, 101, 114, 119, 105, 115, 101, 32, 105, 116, 32, 119, 105, 108, 108, 32, 114, 101, 115, 117, 108, 116, 32, 105, 110, 32, 97, 110, 10, 101, 114, 114, 111, 114, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 44, 32, 97, 95, 104, 105, 44, 32, 97, 95, 108, 111, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 95, 104, 105, 44, 32, 99, 95, 108, 111, 44, 32, 46, 46, 46, 93, 44, 32, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 97, 32, 60, 60, 32, 98, 32, 109, 111, 100, 32, 50, 94, 54, 52, 46, 10, 84, 104, 105, 115, 32, 116, 97, 107, 101, 115, 32, 52, 48, 32, 99, 121, 99, 108, 101, 115, 46, 1, 0, 0, 25, 0, 185, 1, 31, 0, 0, 0, 0, 0, 0, 0, 111, 49, 130, 107, 166, 185, 1, 31, 0, 0, 0, 0, 0, 0, 0, 71, 185, 1, 32, 0, 0, 0, 0, 0, 0, 0, 130, 49, 107, 13, 110, 150, 55, 150, 150, 57, 149, 3, 130, 149, 17, 181]),
("std::math::ec_ext5",vm_assembly::ProcedureId([11, 200, 248, 75, 39, 96, 162, 164, 213, 142, 214, 213, 245, 227, 223, 119, 59, 60, 179, 122, 210, 200, 93, 102]),"use.std::math::ext5

#! Given an encoded elliptic curve point `w` s.t. it's expressed using
#! an element ∈ GF(p^5) | p = 2^64 - 2^32 + 1, this routine verifies whether
#! given point can be successfully decoded or not
#!
#! Expected stack state 
#!
#! [w0, w1, w2, w3, w4, ...]
#!
#! Final stack state 
#!
#! [flg, ...]
#!
#! If w can be decoded, flg = 1
#! Else flg = 0
#!
#! Note, if w = (0, 0, 0, 0, 0), it can be successfully decoded to point 
#! at infinity i.e. flg = 1, in that case.
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1043-L1052
#! for reference implementation
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

#! Given an encoded elliptic curve point `w` s.t. it's expressed using
#! an element ∈ GF(p^5) | p = 2^64 - 2^32 + 1, this routine attempts to decode
#! it into x, y coordinates, along with boolean field element denoting whether it's
#! point-at-infinity or not.
#!
#! Expected stack state 
#!
#! [w0, w1, w2, w3, w4, ...]
#!
#! Final state state 
#!
#! [x0, x1, x2, x3, x4, y0, y1, y2, y3, y4, inf, flg, ...]
#!
#! If `w` has be decoded, flg = 1
#! Else flg = 0 and x, y = (0, 0)
#!
#! Note, when w = (0, 0, 0, 0, 0), it will be successfully decoded to
#! point-at-infinity i.e. x, y = (0, 0) and flg = 1
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1022-L1041
#! for reference implementation
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

#! Given an elliptic curve point as Weierstraß coordinates (X, Y) along with
#! boolean field element `inf`, denoting whether this is point-at-infinity or not, 
#! this routine encodes it to a single element ∈ GF(p^5) | p = 2^64 - 2^32 + 1
#!
#! Expected stack state 
#!
#! [x0, x1, x2, x3, x4, y0, y1, y2, y3, y4, inf, ...]
#!
#! Final stack state 
#!
#! [w0, w1, w2, w3, w4, ...]
#!
#! Note, when inf = 1, encoded point w = (0, 0, 0, 0, 0)
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1214-L1216
#! for reference implementation.
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

#! Given two elliptic curve points ( say a, b ) as Weierstraß coordinates (X, Y) on stack,
#! this routine computes elliptic curve point c, resulting from a + b.
#!
#! Following point addition formula is complete and it works when two points are 
#! same/ different or input operands are point-at-infinity.
#!
#! Expected stack state
#!
#! [x1_0, x1_1, x1_2, x1_3, x1_4, y1_0, y1_1, y1_2, y1_3, y1_4, inf1, x2_0, x2_1, x2_2, x2_3, x2_4, y2_0, y2_1, y2_2, y2_3, y2_4, inf2, ...]
#!
#! s.t. x1_{0..5} -> x1, y1_{0..5} -> y1 |> a = (x1, y1, inf1)
#!      x2_{0..5} -> x2, y2_{0..5} -> y2 |> b = (x2, y2, inf2)
#!
#! Final stack state
#!
#! [x3_0, x3_1, x3_2, x3_3, x3_4, y3_0, y3_1, y3_2, y3_3, y3_4, inf3, ...]
#!
#! Read point addition section ( on page 8 ) of https://ia.cr/2022/274
#! For reference implementation see https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1228-L1255
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

#! Given one elliptic curve point ( say a ) as Weierstraß coordinates (X, Y) on stack,
#! this routine computes elliptic curve point b s.t. b = 2 * a.
#!
#! Following point doubling formula is complete and it works only when input operand is
#! a non-infinity point, then resulting point b should also be non-infinity.
#!
#! Note, result of add(a, b) = double(a) | a = b
#!
#! Expected stack state
#!
#! [x0, x1, x2, x3, x4, y0, y1, y2, y3, y4, inf, ...]
#!
#! s.t. x{0..5} -> x, y{0..5} -> y |> a = (x, y, inf)
#!
#! Final stack state
#!
#! [x'0, x'1, x'2, x'3, x'4, y'0, y'1, y'2, y'3, y'4, inf, ...]
#!
#! Read point addition section ( on page 8 ) of https://ia.cr/2022/274
#! For reference implementation see https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1270-L1280
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

#! Given an elliptic curve point ( say a ) as Weierstraß coordinates (X, Y) and a 319 -bit scalar ( say e )
#! on stack, this routine computes elliptic curve point b s.t. b =  e * a, using double-and-add technique.
#!
#! Scalar e should be lesser than 1067993516717146951041484916571792702745057740581727230159139685185762082554198619328292418486241 ( prime number ).
#! Note, scalar e should be provided as 10 limbs on stack, each of 32 -bit ( in little endian byte order ).
#! 
#! Given a scalar e ( as arbitrary width big integer ), following python code snippet should convert it to desired input form
#!
#! [(a >> (32*i)) & 0xffff_ffff for i in range(10)]
#!
#! Expected stack state
#!
#! [x0, x1, x2, x3, x4, y0, y1, y2, y3, y4, inf, e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, ...]
#!
#! Point a = (x, y, inf)
#! Scalar e = (e0, e1, e2, e3, e4, e5, e6, e7, e8, e9)
#!
#! Final stack state
#!
#! [x'0, x'1, x'2, x'3, x'4, y'0, y'1, y'2, y'3, y'4, inf, ...]
#!
#! Point b = (x', y' inf') | b = e * a
#!
#! See https://github.com/itzmeanjan/secp256k1/blob/cbbe199/point.py#L174-L186 for source of inpiration.
export.mul.10
    loc_storew.0
    dropw
    loc_store.1 # cached base_x
    drop

    loc_storew.2
    dropw
    loc_store.3 # cached base_y
    drop

    loc_store.4 # cached base_inf
    drop

    push.0.0.0.0
    loc_storew.5
    dropw
    push.0
    loc_store.6 # initialize and cache res_x
    drop

    push.0.0.0.0
    loc_storew.7
    dropw
    push.0
    loc_store.8 # initialize and cache res_y
    drop

    push.1
    loc_store.9 # initialize and cache res_inf
    drop

    repeat.10
        repeat.32
            dup
            push.1
            u32checked_and

            if.true
                # bring base
                loc_load.4

                loc_load.3
                push.0.0.0.0
                loc_loadw.2

                loc_load.1
                push.0.0.0.0
                loc_loadw.0

                # bring res
                loc_load.9

                loc_load.8
                push.0.0.0.0
                loc_loadw.7

                loc_load.6
                push.0.0.0.0
                loc_loadw.5

                exec.add

                # write back res
                loc_storew.5
                dropw
                loc_store.6
                drop

                loc_storew.7
                dropw
                loc_store.8
                drop

                loc_store.9
                drop
            end

            # bring base
            loc_load.4

            loc_load.3
            push.0.0.0.0
            loc_loadw.2

            loc_load.1
            push.0.0.0.0
            loc_loadw.0

            exec.double

            # write back base
            loc_storew.0
            dropw
            loc_store.1
            drop

            loc_storew.2
            dropw
            loc_store.3
            drop

            loc_store.4
            drop

            u32unchecked_shr.1
        end

        drop
    end

    # bring res
    loc_load.9

    loc_load.8
    push.0.0.0.0
    loc_loadw.7

    loc_load.6
    push.0.0.0.0
    loc_loadw.5
end
",&[6, 0, 8, 118, 97, 108, 105, 100, 97, 116, 101, 14, 2, 71, 105, 118, 101, 110, 32, 97, 110, 32, 101, 110, 99, 111, 100, 101, 100, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 32, 96, 119, 96, 32, 115, 46, 116, 46, 32, 105, 116, 39, 115, 32, 101, 120, 112, 114, 101, 115, 115, 101, 100, 32, 117, 115, 105, 110, 103, 10, 97, 110, 32, 101, 108, 101, 109, 101, 110, 116, 32, 226, 136, 136, 32, 71, 70, 40, 112, 94, 53, 41, 32, 124, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 45, 32, 50, 94, 51, 50, 32, 43, 32, 49, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 118, 101, 114, 105, 102, 105, 101, 115, 32, 119, 104, 101, 116, 104, 101, 114, 10, 103, 105, 118, 101, 110, 32, 112, 111, 105, 110, 116, 32, 99, 97, 110, 32, 98, 101, 32, 115, 117, 99, 99, 101, 115, 115, 102, 117, 108, 108, 121, 32, 100, 101, 99, 111, 100, 101, 100, 32, 111, 114, 32, 110, 111, 116, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 119, 48, 44, 32, 119, 49, 44, 32, 119, 50, 44, 32, 119, 51, 44, 32, 119, 52, 44, 32, 46, 46, 46, 93, 10, 70, 105, 110, 97, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 102, 108, 103, 44, 32, 46, 46, 46, 93, 10, 73, 102, 32, 119, 32, 99, 97, 110, 32, 98, 101, 32, 100, 101, 99, 111, 100, 101, 100, 44, 32, 102, 108, 103, 32, 61, 32, 49, 10, 69, 108, 115, 101, 32, 102, 108, 103, 32, 61, 32, 48, 10, 78, 111, 116, 101, 44, 32, 105, 102, 32, 119, 32, 61, 32, 40, 48, 44, 32, 48, 44, 32, 48, 44, 32, 48, 44, 32, 48, 41, 44, 32, 105, 116, 32, 99, 97, 110, 32, 98, 101, 32, 115, 117, 99, 99, 101, 115, 115, 102, 117, 108, 108, 121, 32, 100, 101, 99, 111, 100, 101, 100, 32, 116, 111, 32, 112, 111, 105, 110, 116, 10, 97, 116, 32, 105, 110, 102, 105, 110, 105, 116, 121, 32, 105, 46, 101, 46, 32, 102, 108, 103, 32, 61, 32, 49, 44, 32, 105, 110, 32, 116, 104, 97, 116, 32, 99, 97, 115, 101, 46, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 49, 48, 52, 51, 45, 76, 49, 48, 53, 50, 10, 102, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 1, 0, 0, 13, 0, 254, 2, 0, 1, 0, 114, 212, 123, 11, 18, 96, 79, 68, 216, 164, 216, 9, 106, 225, 21, 75, 189, 202, 248, 94, 41, 244, 239, 175, 61, 63, 5, 212, 123, 11, 18, 96, 79, 68, 216, 164, 216, 9, 106, 225, 21, 75, 189, 202, 248, 94, 41, 244, 239, 175, 61, 63, 130, 5, 130, 212, 38, 46, 26, 203, 191, 187, 185, 14, 167, 98, 82, 183, 68, 13, 49, 144, 251, 135, 82, 199, 138, 11, 168, 197, 22, 1, 0, 0, 0, 0, 0, 0, 0, 168, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 254, 15, 0, 3, 0, 130, 22, 0, 0, 0, 0, 0, 0, 0, 0, 18, 19, 6, 100, 101, 99, 111, 100, 101, 147, 2, 71, 105, 118, 101, 110, 32, 97, 110, 32, 101, 110, 99, 111, 100, 101, 100, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 32, 96, 119, 96, 32, 115, 46, 116, 46, 32, 105, 116, 39, 115, 32, 101, 120, 112, 114, 101, 115, 115, 101, 100, 32, 117, 115, 105, 110, 103, 10, 97, 110, 32, 101, 108, 101, 109, 101, 110, 116, 32, 226, 136, 136, 32, 71, 70, 40, 112, 94, 53, 41, 32, 124, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 45, 32, 50, 94, 51, 50, 32, 43, 32, 49, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 97, 116, 116, 101, 109, 112, 116, 115, 32, 116, 111, 32, 100, 101, 99, 111, 100, 101, 10, 105, 116, 32, 105, 110, 116, 111, 32, 120, 44, 32, 121, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 44, 32, 97, 108, 111, 110, 103, 32, 119, 105, 116, 104, 32, 98, 111, 111, 108, 101, 97, 110, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 32, 100, 101, 110, 111, 116, 105, 110, 103, 32, 119, 104, 101, 116, 104, 101, 114, 32, 105, 116, 39, 115, 10, 112, 111, 105, 110, 116, 45, 97, 116, 45, 105, 110, 102, 105, 110, 105, 116, 121, 32, 111, 114, 32, 110, 111, 116, 46, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 119, 48, 44, 32, 119, 49, 44, 32, 119, 50, 44, 32, 119, 51, 44, 32, 119, 52, 44, 32, 46, 46, 46, 93, 10, 70, 105, 110, 97, 108, 32, 115, 116, 97, 116, 101, 32, 115, 116, 97, 116, 101, 10, 91, 120, 48, 44, 32, 120, 49, 44, 32, 120, 50, 44, 32, 120, 51, 44, 32, 120, 52, 44, 32, 121, 48, 44, 32, 121, 49, 44, 32, 121, 50, 44, 32, 121, 51, 44, 32, 121, 52, 44, 32, 105, 110, 102, 44, 32, 102, 108, 103, 44, 32, 46, 46, 46, 93, 10, 73, 102, 32, 96, 119, 96, 32, 104, 97, 115, 32, 98, 101, 32, 100, 101, 99, 111, 100, 101, 100, 44, 32, 102, 108, 103, 32, 61, 32, 49, 10, 69, 108, 115, 101, 32, 102, 108, 103, 32, 61, 32, 48, 32, 97, 110, 100, 32, 120, 44, 32, 121, 32, 61, 32, 40, 48, 44, 32, 48, 41, 10, 78, 111, 116, 101, 44, 32, 119, 104, 101, 110, 32, 119, 32, 61, 32, 40, 48, 44, 32, 48, 44, 32, 48, 44, 32, 48, 44, 32, 48, 41, 44, 32, 105, 116, 32, 119, 105, 108, 108, 32, 98, 101, 32, 115, 117, 99, 99, 101, 115, 115, 102, 117, 108, 108, 121, 32, 100, 101, 99, 111, 100, 101, 100, 32, 116, 111, 10, 112, 111, 105, 110, 116, 45, 97, 116, 45, 105, 110, 102, 105, 110, 105, 116, 121, 32, 105, 46, 101, 46, 32, 120, 44, 32, 121, 32, 61, 32, 40, 48, 44, 32, 48, 41, 32, 97, 110, 100, 32, 102, 108, 103, 32, 61, 32, 49, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 49, 48, 50, 50, 45, 76, 49, 48, 52, 49, 10, 102, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 1, 0, 0, 38, 0, 254, 23, 0, 1, 0, 114, 212, 123, 11, 18, 96, 79, 68, 216, 164, 216, 9, 106, 225, 21, 75, 189, 202, 248, 94, 41, 244, 239, 175, 61, 63, 5, 254, 28, 0, 1, 0, 114, 212, 123, 11, 18, 96, 79, 68, 216, 164, 216, 9, 106, 225, 21, 75, 189, 202, 248, 94, 41, 244, 239, 175, 61, 63, 130, 5, 130, 212, 93, 7, 187, 74, 22, 167, 186, 255, 99, 31, 216, 35, 169, 124, 128, 212, 245, 66, 175, 202, 208, 178, 23, 1, 254, 36, 0, 1, 0, 120, 254, 39, 0, 1, 0, 119, 212, 86, 49, 45, 244, 136, 130, 11, 45, 233, 25, 243, 66, 103, 68, 132, 243, 239, 79, 45, 193, 115, 9, 17, 149, 185, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 128, 255, 255, 255, 127, 212, 14, 28, 171, 83, 83, 32, 255, 54, 152, 60, 86, 23, 119, 109, 189, 213, 184, 10, 82, 204, 116, 172, 34, 97, 254, 45, 0, 1, 0, 156, 254, 48, 0, 1, 0, 162, 212, 234, 95, 121, 92, 168, 18, 198, 185, 94, 230, 82, 197, 3, 198, 51, 170, 89, 168, 192, 247, 130, 80, 66, 63, 185, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 128, 255, 255, 255, 127, 212, 14, 28, 171, 83, 83, 32, 255, 54, 152, 60, 86, 23, 119, 109, 189, 213, 184, 10, 82, 204, 116, 172, 34, 97, 254, 54, 0, 1, 0, 156, 254, 57, 0, 1, 0, 114, 212, 38, 46, 26, 203, 191, 187, 185, 14, 167, 98, 82, 183, 68, 13, 49, 144, 251, 135, 82, 199, 138, 11, 168, 197, 22, 1, 0, 0, 0, 0, 0, 0, 0, 253, 1, 0, 254, 63, 0, 2, 0, 152, 107, 1, 0, 254, 68, 0, 1, 0, 107, 254, 72, 0, 1, 0, 120, 254, 75, 0, 1, 0, 119, 212, 14, 28, 171, 83, 83, 32, 255, 54, 152, 60, 86, 23, 119, 109, 189, 213, 184, 10, 82, 204, 116, 172, 34, 97, 254, 79, 0, 2, 0, 151, 11, 120, 17, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 254, 86, 0, 3, 0, 160, 22, 0, 0, 0, 0, 0, 0, 0, 0, 18, 159, 19, 130, 254, 94, 0, 1, 0, 153, 254, 97, 0, 1, 0, 158, 3, 6, 101, 110, 99, 111, 100, 101, 255, 1, 71, 105, 118, 101, 110, 32, 97, 110, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 32, 97, 115, 32, 87, 101, 105, 101, 114, 115, 116, 114, 97, 195, 159, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 40, 88, 44, 32, 89, 41, 32, 97, 108, 111, 110, 103, 32, 119, 105, 116, 104, 10, 98, 111, 111, 108, 101, 97, 110, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 32, 96, 105, 110, 102, 96, 44, 32, 100, 101, 110, 111, 116, 105, 110, 103, 32, 119, 104, 101, 116, 104, 101, 114, 32, 116, 104, 105, 115, 32, 105, 115, 32, 112, 111, 105, 110, 116, 45, 97, 116, 45, 105, 110, 102, 105, 110, 105, 116, 121, 32, 111, 114, 32, 110, 111, 116, 44, 10, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 101, 110, 99, 111, 100, 101, 115, 32, 105, 116, 32, 116, 111, 32, 97, 32, 115, 105, 110, 103, 108, 101, 32, 101, 108, 101, 109, 101, 110, 116, 32, 226, 136, 136, 32, 71, 70, 40, 112, 94, 53, 41, 32, 124, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 45, 32, 50, 94, 51, 50, 32, 43, 32, 49, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 120, 48, 44, 32, 120, 49, 44, 32, 120, 50, 44, 32, 120, 51, 44, 32, 120, 52, 44, 32, 121, 48, 44, 32, 121, 49, 44, 32, 121, 50, 44, 32, 121, 51, 44, 32, 121, 52, 44, 32, 105, 110, 102, 44, 32, 46, 46, 46, 93, 10, 70, 105, 110, 97, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 119, 48, 44, 32, 119, 49, 44, 32, 119, 50, 44, 32, 119, 51, 44, 32, 119, 52, 44, 32, 46, 46, 46, 93, 10, 78, 111, 116, 101, 44, 32, 119, 104, 101, 110, 32, 105, 110, 102, 32, 61, 32, 49, 44, 32, 101, 110, 99, 111, 100, 101, 100, 32, 112, 111, 105, 110, 116, 32, 119, 32, 61, 32, 40, 48, 44, 32, 48, 44, 32, 48, 44, 32, 48, 44, 32, 48, 41, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 49, 50, 49, 52, 45, 76, 49, 50, 49, 54, 10, 102, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 46, 1, 0, 0, 6, 0, 185, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 85, 85, 85, 85, 212, 234, 95, 121, 92, 168, 18, 198, 185, 94, 230, 82, 197, 3, 198, 51, 170, 89, 168, 192, 247, 130, 80, 66, 63, 254, 105, 0, 1, 0, 156, 212, 104, 121, 166, 177, 157, 137, 124, 238, 20, 148, 237, 36, 234, 92, 222, 219, 22, 119, 84, 176, 24, 174, 59, 229, 152, 253, 2, 0, 254, 111, 0, 1, 0, 107, 185, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 97, 100, 100, 63, 3, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 115, 32, 40, 32, 115, 97, 121, 32, 97, 44, 32, 98, 32, 41, 32, 97, 115, 32, 87, 101, 105, 101, 114, 115, 116, 114, 97, 195, 159, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 40, 88, 44, 32, 89, 41, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 10, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 32, 99, 44, 32, 114, 101, 115, 117, 108, 116, 105, 110, 103, 32, 102, 114, 111, 109, 32, 97, 32, 43, 32, 98, 46, 10, 70, 111, 108, 108, 111, 119, 105, 110, 103, 32, 112, 111, 105, 110, 116, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 102, 111, 114, 109, 117, 108, 97, 32, 105, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 110, 100, 32, 105, 116, 32, 119, 111, 114, 107, 115, 32, 119, 104, 101, 110, 32, 116, 119, 111, 32, 112, 111, 105, 110, 116, 115, 32, 97, 114, 101, 10, 115, 97, 109, 101, 47, 32, 100, 105, 102, 102, 101, 114, 101, 110, 116, 32, 111, 114, 32, 105, 110, 112, 117, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 114, 101, 32, 112, 111, 105, 110, 116, 45, 97, 116, 45, 105, 110, 102, 105, 110, 105, 116, 121, 46, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 120, 49, 95, 48, 44, 32, 120, 49, 95, 49, 44, 32, 120, 49, 95, 50, 44, 32, 120, 49, 95, 51, 44, 32, 120, 49, 95, 52, 44, 32, 121, 49, 95, 48, 44, 32, 121, 49, 95, 49, 44, 32, 121, 49, 95, 50, 44, 32, 121, 49, 95, 51, 44, 32, 121, 49, 95, 52, 44, 32, 105, 110, 102, 49, 44, 32, 120, 50, 95, 48, 44, 32, 120, 50, 95, 49, 44, 32, 120, 50, 95, 50, 44, 32, 120, 50, 95, 51, 44, 32, 120, 50, 95, 52, 44, 32, 121, 50, 95, 48, 44, 32, 121, 50, 95, 49, 44, 32, 121, 50, 95, 50, 44, 32, 121, 50, 95, 51, 44, 32, 121, 50, 95, 52, 44, 32, 105, 110, 102, 50, 44, 32, 46, 46, 46, 93, 10, 115, 46, 116, 46, 32, 120, 49, 95, 123, 48, 46, 46, 53, 125, 32, 45, 62, 32, 120, 49, 44, 32, 121, 49, 95, 123, 48, 46, 46, 53, 125, 32, 45, 62, 32, 121, 49, 32, 124, 62, 32, 97, 32, 61, 32, 40, 120, 49, 44, 32, 121, 49, 44, 32, 105, 110, 102, 49, 41, 10, 120, 50, 95, 123, 48, 46, 46, 53, 125, 32, 45, 62, 32, 120, 50, 44, 32, 121, 50, 95, 123, 48, 46, 46, 53, 125, 32, 45, 62, 32, 121, 50, 32, 124, 62, 32, 98, 32, 61, 32, 40, 120, 50, 44, 32, 121, 50, 44, 32, 105, 110, 102, 50, 41, 10, 70, 105, 110, 97, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 120, 51, 95, 48, 44, 32, 120, 51, 95, 49, 44, 32, 120, 51, 95, 50, 44, 32, 120, 51, 95, 51, 44, 32, 120, 51, 95, 52, 44, 32, 121, 51, 95, 48, 44, 32, 121, 51, 95, 49, 44, 32, 121, 51, 95, 50, 44, 32, 121, 51, 95, 51, 44, 32, 121, 51, 95, 52, 44, 32, 105, 110, 102, 51, 44, 32, 46, 46, 46, 93, 10, 82, 101, 97, 100, 32, 112, 111, 105, 110, 116, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 115, 101, 99, 116, 105, 111, 110, 32, 40, 32, 111, 110, 32, 112, 97, 103, 101, 32, 56, 32, 41, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 105, 97, 46, 99, 114, 47, 50, 48, 50, 50, 47, 50, 55, 52, 10, 70, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 115, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 49, 50, 50, 56, 45, 76, 49, 50, 53, 53, 1, 10, 0, 95, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 197, 1, 0, 0, 0, 0, 0, 0, 0, 107, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 197, 3, 0, 0, 0, 0, 0, 0, 0, 107, 197, 4, 0, 0, 0, 0, 0, 0, 0, 107, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 197, 6, 0, 0, 0, 0, 0, 0, 0, 107, 200, 7, 0, 0, 0, 0, 0, 0, 0, 108, 197, 8, 0, 0, 0, 0, 0, 0, 0, 107, 197, 9, 0, 0, 0, 0, 0, 0, 0, 107, 193, 6, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 193, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 212, 193, 152, 91, 237, 37, 69, 70, 135, 11, 228, 220, 113, 147, 49, 149, 81, 16, 118, 120, 87, 87, 12, 226, 234, 110, 253, 9, 0, 193, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 212, 123, 11, 18, 96, 79, 68, 216, 164, 216, 9, 106, 225, 21, 75, 189, 202, 248, 94, 41, 244, 239, 175, 61, 63, 254, 151, 0, 2, 0, 151, 7, 3, 130, 3, 130, 7, 0, 193, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 193, 8, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 212, 234, 95, 121, 92, 168, 18, 198, 185, 94, 230, 82, 197, 3, 198, 51, 170, 89, 168, 192, 247, 130, 80, 66, 63, 115, 253, 4, 0, 193, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 254, 173, 0, 2, 0, 151, 7, 7, 0, 193, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 193, 6, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 212, 234, 95, 121, 92, 168, 18, 198, 185, 94, 230, 82, 197, 3, 198, 51, 170, 89, 168, 192, 247, 130, 80, 66, 63, 254, 186, 0, 1, 0, 156, 212, 104, 121, 166, 177, 157, 137, 124, 238, 20, 148, 237, 36, 234, 92, 222, 219, 22, 119, 84, 176, 24, 174, 59, 229, 254, 190, 0, 1, 0, 114, 212, 123, 11, 18, 96, 79, 68, 216, 164, 216, 9, 106, 225, 21, 75, 189, 202, 248, 94, 41, 244, 239, 175, 61, 63, 193, 6, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 193, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 212, 86, 49, 45, 244, 136, 130, 11, 45, 233, 25, 243, 66, 103, 68, 132, 243, 239, 79, 45, 193, 115, 9, 17, 149, 254, 201, 0, 1, 0, 156, 212, 234, 95, 121, 92, 168, 18, 198, 185, 94, 230, 82, 197, 3, 198, 51, 170, 89, 168, 192, 247, 130, 80, 66, 63, 254, 205, 0, 1, 0, 114, 193, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 212, 234, 95, 121, 92, 168, 18, 198, 185, 94, 230, 82, 197, 3, 198, 51, 170, 89, 168, 192, 247, 130, 80, 66, 63, 254, 212, 0, 1, 0, 161, 212, 14, 28, 171, 83, 83, 32, 255, 54, 152, 60, 86, 23, 119, 109, 189, 213, 184, 10, 82, 204, 116, 172, 34, 97, 193, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 254, 219, 0, 1, 0, 156, 212, 234, 95, 121, 92, 168, 18, 198, 185, 94, 230, 82, 197, 3, 198, 51, 170, 89, 168, 192, 247, 130, 80, 66, 63, 157, 193, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 193, 8, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 212, 72, 210, 24, 136, 250, 93, 123, 6, 162, 112, 191, 4, 4, 57, 236, 129, 193, 39, 125, 108, 25, 68, 223, 210, 18, 168, 193, 8, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 193, 4, 0, 0, 0, 0, 0, 0, 0, 253, 1, 0, 254, 238, 0, 2, 0, 152, 107, 1, 0, 254, 243, 0, 1, 0, 107, 193, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 193, 9, 0, 0, 0, 0, 0, 0, 0, 253, 1, 0, 254, 252, 0, 2, 0, 152, 107, 1, 0, 254, 1, 1, 1, 0, 107, 254, 5, 1, 1, 0, 157, 193, 6, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 193, 4, 0, 0, 0, 0, 0, 0, 0, 253, 1, 0, 254, 13, 1, 2, 0, 152, 107, 1, 0, 254, 18, 1, 1, 0, 107, 193, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 193, 9, 0, 0, 0, 0, 0, 0, 0, 253, 1, 0, 254, 27, 1, 2, 0, 152, 107, 1, 0, 254, 32, 1, 1, 0, 107, 157, 193, 9, 0, 0, 0, 0, 0, 0, 0, 193, 4, 0, 0, 0, 0, 0, 0, 0, 183, 193, 4, 0, 0, 0, 0, 0, 0, 0, 193, 9, 0, 0, 0, 0, 0, 0, 0, 183, 173, 6, 100, 111, 117, 98, 108, 101, 216, 2, 71, 105, 118, 101, 110, 32, 111, 110, 101, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 32, 40, 32, 115, 97, 121, 32, 97, 32, 41, 32, 97, 115, 32, 87, 101, 105, 101, 114, 115, 116, 114, 97, 195, 159, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 40, 88, 44, 32, 89, 41, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 10, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 32, 98, 32, 115, 46, 116, 46, 32, 98, 32, 61, 32, 50, 32, 42, 32, 97, 46, 10, 70, 111, 108, 108, 111, 119, 105, 110, 103, 32, 112, 111, 105, 110, 116, 32, 100, 111, 117, 98, 108, 105, 110, 103, 32, 102, 111, 114, 109, 117, 108, 97, 32, 105, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 110, 100, 32, 105, 116, 32, 119, 111, 114, 107, 115, 32, 111, 110, 108, 121, 32, 119, 104, 101, 110, 32, 105, 110, 112, 117, 116, 32, 111, 112, 101, 114, 97, 110, 100, 32, 105, 115, 10, 97, 32, 110, 111, 110, 45, 105, 110, 102, 105, 110, 105, 116, 121, 32, 112, 111, 105, 110, 116, 44, 32, 116, 104, 101, 110, 32, 114, 101, 115, 117, 108, 116, 105, 110, 103, 32, 112, 111, 105, 110, 116, 32, 98, 32, 115, 104, 111, 117, 108, 100, 32, 97, 108, 115, 111, 32, 98, 101, 32, 110, 111, 110, 45, 105, 110, 102, 105, 110, 105, 116, 121, 46, 10, 78, 111, 116, 101, 44, 32, 114, 101, 115, 117, 108, 116, 32, 111, 102, 32, 97, 100, 100, 40, 97, 44, 32, 98, 41, 32, 61, 32, 100, 111, 117, 98, 108, 101, 40, 97, 41, 32, 124, 32, 97, 32, 61, 32, 98, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 120, 48, 44, 32, 120, 49, 44, 32, 120, 50, 44, 32, 120, 51, 44, 32, 120, 52, 44, 32, 121, 48, 44, 32, 121, 49, 44, 32, 121, 50, 44, 32, 121, 51, 44, 32, 121, 52, 44, 32, 105, 110, 102, 44, 32, 46, 46, 46, 93, 10, 115, 46, 116, 46, 32, 120, 123, 48, 46, 46, 53, 125, 32, 45, 62, 32, 120, 44, 32, 121, 123, 48, 46, 46, 53, 125, 32, 45, 62, 32, 121, 32, 124, 62, 32, 97, 32, 61, 32, 40, 120, 44, 32, 121, 44, 32, 105, 110, 102, 41, 10, 70, 105, 110, 97, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 120, 39, 48, 44, 32, 120, 39, 49, 44, 32, 120, 39, 50, 44, 32, 120, 39, 51, 44, 32, 120, 39, 52, 44, 32, 121, 39, 48, 44, 32, 121, 39, 49, 44, 32, 121, 39, 50, 44, 32, 121, 39, 51, 44, 32, 121, 39, 52, 44, 32, 105, 110, 102, 44, 32, 46, 46, 46, 93, 10, 82, 101, 97, 100, 32, 112, 111, 105, 110, 116, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 115, 101, 99, 116, 105, 111, 110, 32, 40, 32, 111, 110, 32, 112, 97, 103, 101, 32, 56, 32, 41, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 105, 97, 46, 99, 114, 47, 50, 48, 50, 50, 47, 50, 55, 52, 10, 70, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 115, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 49, 50, 55, 48, 45, 76, 49, 50, 56, 48, 1, 5, 0, 46, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 197, 1, 0, 0, 0, 0, 0, 0, 0, 107, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 197, 3, 0, 0, 0, 0, 0, 0, 0, 107, 197, 4, 0, 0, 0, 0, 0, 0, 0, 107, 193, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 254, 59, 1, 2, 0, 151, 7, 193, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 212, 123, 11, 18, 96, 79, 68, 216, 164, 216, 9, 106, 225, 21, 75, 189, 202, 248, 94, 41, 244, 239, 175, 61, 63, 254, 67, 1, 2, 0, 151, 7, 3, 130, 3, 130, 212, 104, 121, 166, 177, 157, 137, 124, 238, 20, 148, 237, 36, 234, 92, 222, 219, 22, 119, 84, 176, 24, 174, 59, 229, 193, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 254, 79, 1, 2, 0, 151, 7, 254, 83, 1, 1, 0, 119, 212, 123, 11, 18, 96, 79, 68, 216, 164, 216, 9, 106, 225, 21, 75, 189, 202, 248, 94, 41, 244, 239, 175, 61, 63, 212, 234, 95, 121, 92, 168, 18, 198, 185, 94, 230, 82, 197, 3, 198, 51, 170, 89, 168, 192, 247, 130, 80, 66, 63, 254, 88, 1, 1, 0, 114, 193, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 212, 234, 95, 121, 92, 168, 18, 198, 185, 94, 230, 82, 197, 3, 198, 51, 170, 89, 168, 192, 247, 130, 80, 66, 63, 254, 95, 1, 1, 0, 161, 212, 14, 28, 171, 83, 83, 32, 255, 54, 152, 60, 86, 23, 119, 109, 189, 213, 184, 10, 82, 204, 116, 172, 34, 97, 193, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 254, 102, 1, 1, 0, 156, 212, 234, 95, 121, 92, 168, 18, 198, 185, 94, 230, 82, 197, 3, 198, 51, 170, 89, 168, 192, 247, 130, 80, 66, 63, 254, 106, 1, 1, 0, 156, 193, 4, 0, 0, 0, 0, 0, 0, 0, 173, 3, 109, 117, 108, 12, 4, 71, 105, 118, 101, 110, 32, 97, 110, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 32, 40, 32, 115, 97, 121, 32, 97, 32, 41, 32, 97, 115, 32, 87, 101, 105, 101, 114, 115, 116, 114, 97, 195, 159, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 40, 88, 44, 32, 89, 41, 32, 97, 110, 100, 32, 97, 32, 51, 49, 57, 32, 45, 98, 105, 116, 32, 115, 99, 97, 108, 97, 114, 32, 40, 32, 115, 97, 121, 32, 101, 32, 41, 10, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 32, 98, 32, 115, 46, 116, 46, 32, 98, 32, 61, 32, 32, 101, 32, 42, 32, 97, 44, 32, 117, 115, 105, 110, 103, 32, 100, 111, 117, 98, 108, 101, 45, 97, 110, 100, 45, 97, 100, 100, 32, 116, 101, 99, 104, 110, 105, 113, 117, 101, 46, 10, 83, 99, 97, 108, 97, 114, 32, 101, 32, 115, 104, 111, 117, 108, 100, 32, 98, 101, 32, 108, 101, 115, 115, 101, 114, 32, 116, 104, 97, 110, 32, 49, 48, 54, 55, 57, 57, 51, 53, 49, 54, 55, 49, 55, 49, 52, 54, 57, 53, 49, 48, 52, 49, 52, 56, 52, 57, 49, 54, 53, 55, 49, 55, 57, 50, 55, 48, 50, 55, 52, 53, 48, 53, 55, 55, 52, 48, 53, 56, 49, 55, 50, 55, 50, 51, 48, 49, 53, 57, 49, 51, 57, 54, 56, 53, 49, 56, 53, 55, 54, 50, 48, 56, 50, 53, 53, 52, 49, 57, 56, 54, 49, 57, 51, 50, 56, 50, 57, 50, 52, 49, 56, 52, 56, 54, 50, 52, 49, 32, 40, 32, 112, 114, 105, 109, 101, 32, 110, 117, 109, 98, 101, 114, 32, 41, 46, 10, 78, 111, 116, 101, 44, 32, 115, 99, 97, 108, 97, 114, 32, 101, 32, 115, 104, 111, 117, 108, 100, 32, 98, 101, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 97, 115, 32, 49, 48, 32, 108, 105, 109, 98, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 101, 97, 99, 104, 32, 111, 102, 32, 51, 50, 32, 45, 98, 105, 116, 32, 40, 32, 105, 110, 32, 108, 105, 116, 116, 108, 101, 32, 101, 110, 100, 105, 97, 110, 32, 98, 121, 116, 101, 32, 111, 114, 100, 101, 114, 32, 41, 46, 10, 71, 105, 118, 101, 110, 32, 97, 32, 115, 99, 97, 108, 97, 114, 32, 101, 32, 40, 32, 97, 115, 32, 97, 114, 98, 105, 116, 114, 97, 114, 121, 32, 119, 105, 100, 116, 104, 32, 98, 105, 103, 32, 105, 110, 116, 101, 103, 101, 114, 32, 41, 44, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 112, 121, 116, 104, 111, 110, 32, 99, 111, 100, 101, 32, 115, 110, 105, 112, 112, 101, 116, 32, 115, 104, 111, 117, 108, 100, 32, 99, 111, 110, 118, 101, 114, 116, 32, 105, 116, 32, 116, 111, 32, 100, 101, 115, 105, 114, 101, 100, 32, 105, 110, 112, 117, 116, 32, 102, 111, 114, 109, 10, 91, 40, 97, 32, 62, 62, 32, 40, 51, 50, 42, 105, 41, 41, 32, 38, 32, 48, 120, 102, 102, 102, 102, 95, 102, 102, 102, 102, 32, 102, 111, 114, 32, 105, 32, 105, 110, 32, 114, 97, 110, 103, 101, 40, 49, 48, 41, 93, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 120, 48, 44, 32, 120, 49, 44, 32, 120, 50, 44, 32, 120, 51, 44, 32, 120, 52, 44, 32, 121, 48, 44, 32, 121, 49, 44, 32, 121, 50, 44, 32, 121, 51, 44, 32, 121, 52, 44, 32, 105, 110, 102, 44, 32, 101, 48, 44, 32, 101, 49, 44, 32, 101, 50, 44, 32, 101, 51, 44, 32, 101, 52, 44, 32, 101, 53, 44, 32, 101, 54, 44, 32, 101, 55, 44, 32, 101, 56, 44, 32, 101, 57, 44, 32, 46, 46, 46, 93, 10, 80, 111, 105, 110, 116, 32, 97, 32, 61, 32, 40, 120, 44, 32, 121, 44, 32, 105, 110, 102, 41, 10, 83, 99, 97, 108, 97, 114, 32, 101, 32, 61, 32, 40, 101, 48, 44, 32, 101, 49, 44, 32, 101, 50, 44, 32, 101, 51, 44, 32, 101, 52, 44, 32, 101, 53, 44, 32, 101, 54, 44, 32, 101, 55, 44, 32, 101, 56, 44, 32, 101, 57, 41, 10, 70, 105, 110, 97, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 120, 39, 48, 44, 32, 120, 39, 49, 44, 32, 120, 39, 50, 44, 32, 120, 39, 51, 44, 32, 120, 39, 52, 44, 32, 121, 39, 48, 44, 32, 121, 39, 49, 44, 32, 121, 39, 50, 44, 32, 121, 39, 51, 44, 32, 121, 39, 52, 44, 32, 105, 110, 102, 44, 32, 46, 46, 46, 93, 10, 80, 111, 105, 110, 116, 32, 98, 32, 61, 32, 40, 120, 39, 44, 32, 121, 39, 32, 105, 110, 102, 39, 41, 32, 124, 32, 98, 32, 61, 32, 101, 32, 42, 32, 97, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 105, 116, 122, 109, 101, 97, 110, 106, 97, 110, 47, 115, 101, 99, 112, 50, 53, 54, 107, 49, 47, 98, 108, 111, 98, 47, 99, 98, 98, 101, 49, 57, 57, 47, 112, 111, 105, 110, 116, 46, 112, 121, 35, 76, 49, 55, 52, 45, 76, 49, 56, 54, 32, 102, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 111, 102, 32, 105, 110, 112, 105, 114, 97, 116, 105, 111, 110, 46, 1, 10, 0, 33, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 197, 1, 0, 0, 0, 0, 0, 0, 0, 107, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 197, 3, 0, 0, 0, 0, 0, 0, 0, 107, 197, 4, 0, 0, 0, 0, 0, 0, 0, 107, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 197, 6, 0, 0, 0, 0, 0, 0, 0, 107, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 7, 0, 0, 0, 0, 0, 0, 0, 108, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 197, 8, 0, 0, 0, 0, 0, 0, 0, 107, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 197, 9, 0, 0, 0, 0, 0, 0, 0, 107, 254, 138, 1, 2, 0, 254, 139, 1, 23, 0, 110, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 71, 253, 25, 0, 193, 4, 0, 0, 0, 0, 0, 0, 0, 193, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 193, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 193, 9, 0, 0, 0, 0, 0, 0, 0, 193, 8, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 193, 6, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 211, 3, 0, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 197, 6, 0, 0, 0, 0, 0, 0, 0, 107, 200, 7, 0, 0, 0, 0, 0, 0, 0, 108, 197, 8, 0, 0, 0, 0, 0, 0, 0, 107, 197, 9, 0, 0, 0, 0, 0, 0, 0, 107, 0, 0, 193, 4, 0, 0, 0, 0, 0, 0, 0, 193, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 193, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 4, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 197, 1, 0, 0, 0, 0, 0, 0, 0, 107, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 197, 3, 0, 0, 0, 0, 0, 0, 0, 107, 197, 4, 0, 0, 0, 0, 0, 0, 0, 107, 78, 1, 107, 193, 9, 0, 0, 0, 0, 0, 0, 0, 193, 8, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 193, 6, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0]),
("std::math::u256",vm_assembly::ProcedureId([103, 72, 7, 211, 66, 172, 161, 203, 22, 238, 56, 207, 243, 100, 193, 210, 195, 158, 175, 210, 228, 25, 74, 59]),"export.add_unsafe
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

#! Performs addition of two unsigned 256 bit integers discarding the overflow.
#! The input values are assumed to be represented using 32 bit limbs, but this is not checked.
#! Stack transition looks as follows:
#! [b7, b6, b5, b4, b3, b2, b1, b0, a7, a6, a5, a4, a3, a2, a1, a0, ...] -> [c7, c6, c5, c4, c3, c2, c1, c0, ...]
#! where c = (a * b) % 2^256, and a0, b0, and c0 are least significant 32-bit limbs of a, b, and c respectively.
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
end",&[10, 0, 10, 97, 100, 100, 95, 117, 110, 115, 97, 102, 101, 0, 0, 1, 0, 0, 29, 0, 147, 150, 154, 41, 151, 154, 43, 151, 153, 43, 151, 152, 43, 175, 146, 159, 151, 155, 43, 151, 154, 43, 151, 153, 43, 151, 152, 43, 107, 10, 115, 117, 98, 95, 117, 110, 115, 97, 102, 101, 0, 0, 1, 0, 0, 56, 0, 147, 150, 154, 49, 154, 41, 152, 149, 49, 149, 3, 153, 41, 152, 149, 49, 149, 3, 152, 41, 152, 149, 49, 149, 3, 175, 146, 159, 151, 41, 155, 149, 49, 149, 3, 151, 41, 154, 149, 49, 149, 3, 151, 41, 153, 149, 49, 149, 3, 152, 152, 149, 41, 107, 49, 107, 3, 97, 110, 100, 0, 0, 1, 0, 0, 26, 0, 147, 150, 154, 71, 150, 153, 71, 150, 152, 71, 150, 151, 71, 146, 150, 154, 71, 150, 153, 71, 150, 152, 71, 150, 151, 71, 2, 111, 114, 0, 0, 1, 0, 0, 26, 0, 147, 150, 154, 72, 150, 153, 72, 150, 152, 72, 150, 151, 72, 146, 150, 154, 72, 150, 153, 72, 150, 152, 72, 150, 151, 72, 3, 120, 111, 114, 0, 0, 1, 0, 0, 26, 0, 147, 150, 154, 73, 150, 153, 73, 150, 152, 73, 150, 151, 73, 146, 150, 154, 73, 150, 153, 73, 150, 152, 73, 150, 151, 73, 13, 105, 115, 122, 101, 114, 111, 95, 117, 110, 115, 97, 102, 101, 0, 0, 1, 0, 0, 2, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 254, 175, 0, 3, 0, 130, 22, 0, 0, 0, 0, 0, 0, 0, 0, 18, 9, 101, 113, 95, 117, 110, 115, 97, 102, 101, 0, 0, 1, 0, 0, 11, 0, 147, 25, 171, 108, 108, 171, 25, 171, 108, 108, 18, 7, 109, 117, 108, 115, 116, 101, 112, 0, 0, 0, 0, 0, 6, 0, 165, 57, 165, 41, 149, 3, 8, 109, 117, 108, 115, 116, 101, 112, 52, 0, 0, 0, 0, 0, 28, 0, 159, 111, 157, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 211, 7, 0, 130, 172, 111, 156, 160, 132, 211, 7, 0, 130, 171, 111, 155, 159, 132, 211, 7, 0, 130, 170, 111, 154, 158, 132, 211, 7, 0, 130, 169, 10, 109, 117, 108, 95, 117, 110, 115, 97, 102, 101, 167, 1, 80, 101, 114, 102, 111, 114, 109, 115, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 50, 53, 54, 32, 98, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 100, 105, 115, 99, 97, 114, 100, 105, 110, 103, 32, 116, 104, 101, 32, 111, 118, 101, 114, 102, 108, 111, 119, 46, 10, 84, 104, 101, 32, 105, 110, 112, 117, 116, 32, 118, 97, 108, 117, 101, 115, 32, 97, 114, 101, 32, 97, 115, 115, 117, 109, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 117, 115, 105, 110, 103, 32, 51, 50, 32, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 98, 117, 116, 32, 116, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 99, 104, 101, 99, 107, 101, 100, 46, 10, 83, 116, 97, 99, 107, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 108, 111, 111, 107, 115, 32, 97, 115, 32, 102, 111, 108, 108, 111, 119, 115, 58, 10, 91, 98, 55, 44, 32, 98, 54, 44, 32, 98, 53, 44, 32, 98, 52, 44, 32, 98, 51, 44, 32, 98, 50, 44, 32, 98, 49, 44, 32, 98, 48, 44, 32, 97, 55, 44, 32, 97, 54, 44, 32, 97, 53, 44, 32, 97, 52, 44, 32, 97, 51, 44, 32, 97, 50, 44, 32, 97, 49, 44, 32, 97, 48, 44, 32, 46, 46, 46, 93, 32, 45, 62, 32, 91, 99, 55, 44, 32, 99, 54, 44, 32, 99, 53, 44, 32, 99, 52, 44, 32, 99, 51, 44, 32, 99, 50, 44, 32, 99, 49, 44, 32, 99, 48, 44, 32, 46, 46, 46, 93, 10, 119, 104, 101, 114, 101, 32, 99, 32, 61, 32, 40, 97, 32, 42, 32, 98, 41, 32, 37, 32, 50, 94, 50, 53, 54, 44, 32, 97, 110, 100, 32, 97, 48, 44, 32, 98, 48, 44, 32, 97, 110, 100, 32, 99, 48, 32, 97, 114, 101, 32, 108, 101, 97, 115, 116, 32, 115, 105, 103, 110, 105, 102, 105, 99, 97, 110, 116, 32, 51, 50, 45, 98, 105, 116, 32, 108, 105, 109, 98, 115, 32, 111, 102, 32, 97, 44, 32, 98, 44, 32, 97, 110, 100, 32, 99, 32, 114, 101, 115, 112, 101, 99, 116, 105, 118, 101, 108, 121, 46, 1, 6, 0, 53, 1, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 171, 200, 2, 0, 0, 0, 0, 0, 0, 0, 145, 200, 3, 0, 0, 0, 0, 0, 0, 0, 109, 200, 4, 0, 0, 0, 0, 0, 0, 0, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 145, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 179, 159, 211, 8, 0, 172, 172, 145, 200, 4, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 145, 156, 156, 111, 153, 157, 132, 211, 7, 0, 130, 168, 111, 152, 156, 132, 211, 7, 0, 130, 167, 111, 151, 155, 132, 211, 7, 0, 130, 166, 130, 149, 153, 132, 211, 7, 0, 107, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 154, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 149, 166, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 211, 8, 0, 172, 172, 145, 166, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 166, 200, 4, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 150, 107, 145, 156, 156, 111, 153, 156, 132, 211, 7, 0, 130, 170, 111, 152, 154, 132, 211, 7, 0, 130, 168, 130, 150, 151, 132, 211, 7, 0, 107, 130, 107, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 154, 154, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 130, 166, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 211, 8, 0, 172, 172, 145, 166, 166, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 107, 107, 166, 166, 200, 4, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 150, 150, 107, 107, 145, 156, 156, 111, 153, 155, 132, 211, 7, 0, 130, 169, 111, 152, 153, 132, 211, 7, 0, 130, 130, 107, 166, 107, 107, 107, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 154, 154, 154, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 166, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 211, 8, 0, 172, 172, 145, 150, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 107, 150, 200, 4, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 166, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 145, 156, 156, 130, 152, 153, 132, 211, 7, 0, 107, 166, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 211, 8, 0, 108, 107, 107, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 149, 166, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 154, 111, 153, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 211, 7, 0, 130, 170, 151, 112, 154, 132, 211, 7, 0, 130, 168, 130, 150, 151, 132, 211, 7, 0, 107, 130, 107, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 130, 166, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 153, 111, 153, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 211, 7, 0, 130, 169, 130, 151, 152, 132, 211, 7, 0, 107, 165, 107, 107, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 166, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 151, 152, 165, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 211, 7, 0, 107, 166, 107, 107, 107, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 145]),
("std::math::ext2",vm_assembly::ProcedureId([61, 169, 56, 236, 75, 27, 240, 183, 219, 34, 14, 208, 61, 214, 66, 187, 190, 97, 89, 172, 102, 59, 222, 238]),"#! Given a stack with initial configuration given by [a1,a0,b1,b0,...] where a = (a0,a1) and
#! b = (b0,b1) represent elements in the extension field of degree 2, the procedure outputs the 
#! product c = (c1,c0) where c0 = a0b0 - 2(a1b1) and c1 = (a0 + a1)(b0 + b1) - a0b0
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

#! Given a stack with initial configuration given by [x,a1,a0,...] where a = (a0,a1) is an element
#! in the field extension and x is an element of the base field, this procedure computes the multiplication
#! of x, when looked at as (x,0), with a in the extension field. The output is [xa1,xa0,...]
export.mul_base
    dup         #[x,x,a1,a0,...]
    movdn.3     #[x,a1,a0,x,...]
    mul         #[xa1,a0,x,...]
    movdn.2     #[a0,x,xa1,...]
    mul         #[xa0,xa1,...]
    swap        #[xa1,xa0,...]
end

#! Given a stack in the following initial configuration [a1,a0,b1,b0,...] the following
#! procedure computes [a1+b1,a0+b0,...]
export.add
    swap        #[a0,a1,b1,b0,...]
    movup.3     #[b0,a0,a1,b1,...]
    add         #[b0+a0,a1,b1,...]
    movdn.2     #[a1,b1,b0+a0,...]
    add         #[a1+b1,b0+a0,...]
end

#! Given a stack in the following initial configuration [a1,a0,b1,b0,...] the following
#! procedure computes [a1-b1,a0-b0,...]
export.sub
    swap        #[a0,a1,b1,b0,...]
    movup.3     #[b0,a0,a1,b1,...]
    sub         #[a0-b0,a1,b1,...]
    movdn.2     #[a1,b1,a0-b0,...]
    swap        #[b1,a1,a0-b0,...]
    sub         #[a1-b1,a0-b0,...]
end",&[4, 0, 3, 109, 117, 108, 7, 1, 71, 105, 118, 101, 110, 32, 97, 32, 115, 116, 97, 99, 107, 32, 119, 105, 116, 104, 32, 105, 110, 105, 116, 105, 97, 108, 32, 99, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105, 111, 110, 32, 103, 105, 118, 101, 110, 32, 98, 121, 32, 91, 97, 49, 44, 97, 48, 44, 98, 49, 44, 98, 48, 44, 46, 46, 46, 93, 32, 119, 104, 101, 114, 101, 32, 97, 32, 61, 32, 40, 97, 48, 44, 97, 49, 41, 32, 97, 110, 100, 10, 98, 32, 61, 32, 40, 98, 48, 44, 98, 49, 41, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 105, 110, 32, 116, 104, 101, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 102, 105, 101, 108, 100, 32, 111, 102, 32, 100, 101, 103, 114, 101, 101, 32, 50, 44, 32, 116, 104, 101, 32, 112, 114, 111, 99, 101, 100, 117, 114, 101, 32, 111, 117, 116, 112, 117, 116, 115, 32, 116, 104, 101, 10, 112, 114, 111, 100, 117, 99, 116, 32, 99, 32, 61, 32, 40, 99, 49, 44, 99, 48, 41, 32, 119, 104, 101, 114, 101, 32, 99, 48, 32, 61, 32, 97, 48, 98, 48, 32, 45, 32, 50, 40, 97, 49, 98, 49, 41, 32, 97, 110, 100, 32, 99, 49, 32, 61, 32, 40, 97, 48, 32, 43, 32, 97, 49, 41, 40, 98, 48, 32, 43, 32, 98, 49, 41, 32, 45, 32, 97, 48, 98, 48, 1, 0, 0, 16, 0, 126, 132, 7, 110, 170, 165, 7, 7, 5, 168, 3, 131, 3, 7, 130, 5, 8, 109, 117, 108, 95, 98, 97, 115, 101, 34, 1, 71, 105, 118, 101, 110, 32, 97, 32, 115, 116, 97, 99, 107, 32, 119, 105, 116, 104, 32, 105, 110, 105, 116, 105, 97, 108, 32, 99, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105, 111, 110, 32, 103, 105, 118, 101, 110, 32, 98, 121, 32, 91, 120, 44, 97, 49, 44, 97, 48, 44, 46, 46, 46, 93, 32, 119, 104, 101, 114, 101, 32, 97, 32, 61, 32, 40, 97, 48, 44, 97, 49, 41, 32, 105, 115, 32, 97, 110, 32, 101, 108, 101, 109, 101, 110, 116, 10, 105, 110, 32, 116, 104, 101, 32, 102, 105, 101, 108, 100, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 97, 110, 100, 32, 120, 32, 105, 115, 32, 97, 110, 32, 101, 108, 101, 109, 101, 110, 116, 32, 111, 102, 32, 116, 104, 101, 32, 98, 97, 115, 101, 32, 102, 105, 101, 108, 100, 44, 32, 116, 104, 105, 115, 32, 112, 114, 111, 99, 101, 100, 117, 114, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 116, 104, 101, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 10, 111, 102, 32, 120, 44, 32, 119, 104, 101, 110, 32, 108, 111, 111, 107, 101, 100, 32, 97, 116, 32, 97, 115, 32, 40, 120, 44, 48, 41, 44, 32, 119, 105, 116, 104, 32, 97, 32, 105, 110, 32, 116, 104, 101, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 102, 105, 101, 108, 100, 46, 32, 84, 104, 101, 32, 111, 117, 116, 112, 117, 116, 32, 105, 115, 32, 91, 120, 97, 49, 44, 120, 97, 48, 44, 46, 46, 46, 93, 1, 0, 0, 6, 0, 110, 166, 7, 165, 7, 130, 3, 97, 100, 100, 121, 0, 71, 105, 118, 101, 110, 32, 97, 32, 115, 116, 97, 99, 107, 32, 105, 110, 32, 116, 104, 101, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 32, 99, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105, 111, 110, 32, 91, 97, 49, 44, 97, 48, 44, 98, 49, 44, 98, 48, 44, 46, 46, 46, 93, 32, 116, 104, 101, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 10, 112, 114, 111, 99, 101, 100, 117, 114, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 91, 97, 49, 43, 98, 49, 44, 97, 48, 43, 98, 48, 44, 46, 46, 46, 93, 1, 0, 0, 5, 0, 130, 150, 3, 165, 3, 3, 115, 117, 98, 121, 0, 71, 105, 118, 101, 110, 32, 97, 32, 115, 116, 97, 99, 107, 32, 105, 110, 32, 116, 104, 101, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 32, 99, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105, 111, 110, 32, 91, 97, 49, 44, 97, 48, 44, 98, 49, 44, 98, 48, 44, 46, 46, 46, 93, 32, 116, 104, 101, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 10, 112, 114, 111, 99, 101, 100, 117, 114, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 91, 97, 49, 45, 98, 49, 44, 97, 48, 45, 98, 48, 44, 46, 46, 46, 93, 1, 0, 0, 6, 0, 130, 150, 5, 165, 130, 5]),
("std::math::poly512",vm_assembly::ProcedureId([246, 254, 122, 17, 172, 48, 42, 219, 199, 126, 234, 204, 22, 172, 49, 139, 159, 7, 169, 35, 6, 217, 58, 54]),"use.std::math::ntt512
use.std::math::u64

#! Given two consecutive words on stack, this routine performs 
#! element wise multiplication, while keeping resulting single
#! word on stack.
#!
#! Expected stack state looks like
#!
#! [a0, a1, a2, a3, b0, b1, b2, b3]
#!
#! What this routine does is
#!
#! c`i` = a`i` * b`i` mod P | i ∈ [0, 4), P = 2 ^ 64 - 2 ^ 32 + 1
#!
#! Output stack state looks like
#!
#! [c0, c1, c2, c3]
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

#! Given two consecutive words on stack, this routine performs 
#! element wise addition, while keeping resulting single
#! word on stack.
#!
#! Expected stack state looks like
#!
#! [a0, a1, a2, a3, b0, b1, b2, b3]
#!
#! What this routine does is
#!
#! c`i` = a`i` + b`i` mod P | i ∈ [0, 4), P = 2 ^ 64 - 2 ^ 32 + 1
#!
#! Output stack state looks like
#!
#! [c0, c1, c2, c3]
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

#! Given dividend ( i.e. field element a ) on stack top, this routine computes c = a % 12289
#!
#! Expected stack state
#!
#! [a, ...]
#!
#! Output stack state looks like
#!
#! [c, ...] | c = a % 12289
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

#! Given four elements on stack top, this routine reduces them by applying
#! modular division by 12289 ( = Falcon Signature Algorithm's Prime Number )
#!
#! Input stack state :
#!
#! [a0, a1, a3, a3, ...]
#!
#! Operated such that
#!
#! b`i` = a`i` % 12289 | i ∈ [0..4)
#!
#! Output stack state :
#!
#! [b0, b1, b2, b3, ...]
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

#! Given an operand on stack, this routine negates the element, using modular arithmetic
#! over Falcon Digital Signature Algorithm's prime field = 12289.
#!
#! All this routine does is
#!
#! b = (0 - a) % Q
#!   = Q - a % Q | Q = 12289
#!
#! Input stack state
#!
#! [a,  ...]
#!
#! Output stack state looks like
#!
#! [b, ...] | b ∈ [0..12289)
proc.neg
    exec.mod_12289

    push.12289
    swap
    sub
end

#! Given four elements on stack, this routine negates those, using modular arithmetic
#! over Falcon Digital Signature Algorithm's prime field = 12289.
#!
#! All this routine does is
#!
#! b`i` = (0 - a`i`) % Q
#!   = Q - a`i` % Q | Q = 12289 & i ∈ [0..4)
#!
#! Input stack state
#!
#! [a0, a1, a2, a3, ...]
#!
#! Output stack state looks like
#!
#! [b0, b1, b2, b3 ...] | b`i` ∈ [0..12289)
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

#! Given a field element, this routine does centered reduction using Miden VM
#! prime ( say Q ) and then reduces it using Falcon Post Quantum Digital 
#! Signature Algorithm prime ( say Q' )
#!
#! Q = 2 ^ 64 - 2 ^ 32 + 1
#! Q' = 12289
#!
#! Expected stack state
#!
#! [a, ...]
#!
#! All this routine does is
#!
#! if a > (Q >> 1):
#!   b = (a - Q) % Q'
#! else:
#!   b = a % Q'
#!
#! Final stack state looks like
#!
#! [b, ...]
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

#! Reduces four consecutive elements living on stack top using `reduce` routine ( defined above )
#!
#! Expected stack state
#!
#! [a0, a1, a2, a3, ...]
#!
#! What this routine does is
#!
#! b`i` = reduce(a`i`)
#!
#! Final stack state looks like
#!
#! [b0, b1, b2, b3, ...]
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

#! Given two polynomials of degree 512 on stack as absolute memory addresses,
#! this routine computes polynomial multiplication, using NTT and iNTT.
#!
#! Imagine, two polynomials are f, g
#!
#! h = f . g, can be computed using
#!
#! iNTT(NTT(f) * NTT(g))
#!
#! Note, * -> element wise multiplication of polynomial coefficients in NTT domain
#!
#! Input stack state :
#!
#! [f_start_addr, g_start_addr, h_start_addr, ...]
#!
#! - {f, g, h}_addr`i` -> {f, g, h}[ (i << 2) .. ((i+1) << 2) ), address holding four consecutive coefficients
#! - {f, g, h}_addr0 -> {f, g, h}_start_addr
#!
#! Output stack state :
#!
#! [ ... ]
#!
#! Consecutive 127 memory addresses can be computed from starting memory address ( living on stack top ) by 
#! continuing to apply `INCR` ( = add.1 ) instruction on previous absolute memory address.
#!
#! Note, input memory addresses are considered to be read-only, they are not mutated.
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

#! Given two polynomials of degree 512 on stack as absolute memory addresses,
#! this routine computes polynomial addition.
#!
#! Imagine, two polynomials f, g
#!
#! h = f + g, can be computed as
#!
#! [(f[i] + g[i]) % Q for i in range(512)] | Q = 12289 ( = Falcon Digital Signature Algorithm's Prime Number )
#!
#! Input stack state :
#!
#! [f_start_addr, g_start_addr, h_start_addr, ...]
#!
#! - {f, g, h}_addr`i` -> {f, g, h}[ (i << 2) .. ((i+1) << 2) ), address holding four consecutive coefficients
#! - {f, g, h}_addr0 -> {f, g, h}_start_addr
#!
#! Output stack state :
#!
#! [ ... ]
#!
#! Consecutive 127 memory addresses can be computed from starting memory address ( living on stack top ) by 
#! continuing to apply `INCR` ( = add.1 ) instruction on previous absolute memory address.
#!
#! Note, input memory addresses are considered to be read-only, they are not mutated.
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

#! Given one polynomial of degree 512 on stack as absolute memory addresses,
#! this routine negates each coefficient of that polynomial.
#!
#! Imagine, polynomial f
#!
#! g = -f, can be computed as
#!
#! [(-f[i]) % Q for i in range(512)] | Q = 12289 ( = Falcon Digital Signature Algorithm's Prime Number )
#!
#! Input stack state :
#!
#! [f_start_addr, g_start_addr, ...]
#!
#! - {f,g}_addr`i` -> {f,g}[ (i << 2) .. ((i+1) << 2) ), address holding four consecutive coefficients
#! - {f,g}_addr0 -> {f,g}_start_addr
#!
#! Output stack state :
#!
#! [ ... ]
#!
#! Consecutive 127 memory addresses can be computed from starting memory address ( living on stack top ) by 
#! continuing to apply `INCR` ( = add.1 ) instruction on previous absolute memory address.
#!
#! Note, input memory addresses are considered to be read-only, they are not mutated.
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

#! Given two polynomials of degree 512 on stack as absolute memory addresses,
#! this routine subtracts second polynomial from first one.
#!
#! Imagine, two polynomials f, g
#!
#! h = f - g, can be computed as
#!
#! [(f[i] - g[i]) % Q for i in range(512)] | Q = 12289 ( = Falcon Digital Signature Algorithm's Prime Number )
#!
#! Input stack state :
#!
#! [f_start_addr, g_start_addr, h_start_addr ...]
#!
#! - {f, g, h}_addr`i` -> {f, g, h}[ (i << 2) .. ((i+1) << 2) ), address holding four consecutive coefficients
#! - {f, g, h}_addr0 -> {f, g, h}_start_addr
#!
#! Output stack state :
#!
#! [ ... ]
#!
#! Consecutive 127 memory addresses can be computed from starting memory address ( living on stack top ) by 
#! continuing to apply `INCR` ( = add.1 ) instruction on previous absolute memory address.
#!
#! Note, input memory addresses are considered to be read-only, they are not mutated.
export.sub_zq.128
    locaddr.0
    movup.2
    exec.neg_zq

    locaddr.0
    exec.add_zq
end
",&[12, 0, 8, 109, 117, 108, 95, 119, 111, 114, 100, 0, 0, 0, 0, 0, 11, 0, 151, 7, 169, 150, 7, 168, 149, 7, 167, 7, 166, 8, 97, 100, 100, 95, 119, 111, 114, 100, 0, 0, 0, 0, 0, 11, 0, 151, 3, 169, 150, 3, 168, 149, 3, 167, 3, 166, 9, 109, 111, 100, 95, 49, 50, 50, 56, 57, 174, 0, 71, 105, 118, 101, 110, 32, 100, 105, 118, 105, 100, 101, 110, 100, 32, 40, 32, 105, 46, 101, 46, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 32, 97, 32, 41, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 116, 111, 112, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 99, 32, 61, 32, 97, 32, 37, 32, 49, 50, 50, 56, 57, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 97, 44, 32, 46, 46, 46, 93, 10, 79, 117, 116, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 108, 111, 111, 107, 115, 32, 108, 105, 107, 101, 10, 91, 99, 44, 32, 46, 46, 46, 93, 32, 124, 32, 99, 32, 61, 32, 97, 32, 37, 32, 49, 50, 50, 56, 57, 1, 0, 0, 29, 0, 35, 185, 2, 1, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 205, 186, 2, 0, 0, 0, 0, 0, 0, 0, 33, 130, 185, 1, 1, 48, 0, 0, 0, 0, 0, 0, 55, 149, 185, 1, 1, 48, 0, 0, 0, 0, 0, 0, 57, 107, 186, 2, 0, 0, 0, 0, 0, 0, 0, 107, 32, 110, 150, 41, 150, 41, 107, 152, 1, 151, 1, 130, 107, 130, 107, 14, 109, 111, 100, 95, 49, 50, 50, 56, 57, 95, 119, 111, 114, 100, 0, 0, 0, 0, 0, 10, 0, 211, 2, 0, 130, 211, 2, 0, 130, 149, 211, 2, 0, 165, 150, 211, 2, 0, 166, 3, 110, 101, 103, 0, 0, 0, 0, 0, 4, 0, 211, 2, 0, 185, 1, 1, 48, 0, 0, 0, 0, 0, 0, 130, 5, 8, 110, 101, 103, 95, 119, 111, 114, 100, 0, 0, 0, 0, 0, 10, 0, 211, 4, 0, 130, 211, 4, 0, 130, 149, 211, 4, 0, 165, 150, 211, 4, 0, 166, 6, 114, 101, 100, 117, 99, 101, 0, 0, 0, 0, 0, 4, 0, 110, 185, 1, 0, 0, 0, 128, 255, 255, 255, 127, 28, 253, 5, 0, 211, 2, 0, 110, 185, 1, 90, 27, 0, 0, 0, 0, 0, 0, 102, 253, 1, 0, 5, 6, 0, 185, 1, 90, 27, 0, 0, 0, 0, 0, 0, 130, 5, 185, 1, 1, 48, 0, 0, 0, 0, 0, 0, 130, 5, 1, 0, 211, 2, 0, 11, 114, 101, 100, 117, 99, 101, 95, 119, 111, 114, 100, 0, 0, 0, 0, 0, 10, 0, 211, 6, 0, 130, 211, 6, 0, 130, 149, 211, 6, 0, 165, 150, 211, 6, 0, 166, 6, 109, 117, 108, 95, 122, 113, 67, 3, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 115, 32, 111, 102, 32, 100, 101, 103, 114, 101, 101, 32, 53, 49, 50, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 97, 115, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 44, 10, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 44, 32, 117, 115, 105, 110, 103, 32, 78, 84, 84, 32, 97, 110, 100, 32, 105, 78, 84, 84, 46, 10, 73, 109, 97, 103, 105, 110, 101, 44, 32, 116, 119, 111, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 115, 32, 97, 114, 101, 32, 102, 44, 32, 103, 10, 104, 32, 61, 32, 102, 32, 46, 32, 103, 44, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 117, 115, 105, 110, 103, 10, 105, 78, 84, 84, 40, 78, 84, 84, 40, 102, 41, 32, 42, 32, 78, 84, 84, 40, 103, 41, 41, 10, 78, 111, 116, 101, 44, 32, 42, 32, 45, 62, 32, 101, 108, 101, 109, 101, 110, 116, 32, 119, 105, 115, 101, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 99, 111, 101, 102, 102, 105, 99, 105, 101, 110, 116, 115, 32, 105, 110, 32, 78, 84, 84, 32, 100, 111, 109, 97, 105, 110, 10, 73, 110, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 102, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 103, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 104, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 46, 46, 46, 93, 10, 45, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 95, 97, 100, 100, 114, 96, 105, 96, 32, 45, 62, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 91, 32, 40, 105, 32, 60, 60, 32, 50, 41, 32, 46, 46, 32, 40, 40, 105, 43, 49, 41, 32, 60, 60, 32, 50, 41, 32, 41, 44, 32, 97, 100, 100, 114, 101, 115, 115, 32, 104, 111, 108, 100, 105, 110, 103, 32, 102, 111, 117, 114, 32, 99, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 99, 111, 101, 102, 102, 105, 99, 105, 101, 110, 116, 115, 10, 45, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 95, 97, 100, 100, 114, 48, 32, 45, 62, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 10, 79, 117, 116, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 32, 46, 46, 46, 32, 93, 10, 67, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 49, 50, 55, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 102, 114, 111, 109, 32, 115, 116, 97, 114, 116, 105, 110, 103, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 32, 40, 32, 108, 105, 118, 105, 110, 103, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 116, 111, 112, 32, 41, 32, 98, 121, 10, 99, 111, 110, 116, 105, 110, 117, 105, 110, 103, 32, 116, 111, 32, 97, 112, 112, 108, 121, 32, 96, 73, 78, 67, 82, 96, 32, 40, 32, 61, 32, 97, 100, 100, 46, 49, 32, 41, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 111, 110, 32, 112, 114, 101, 118, 105, 111, 117, 115, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 46, 10, 78, 111, 116, 101, 44, 32, 105, 110, 112, 117, 116, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 99, 111, 110, 115, 105, 100, 101, 114, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 97, 100, 45, 111, 110, 108, 121, 44, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 32, 109, 117, 116, 97, 116, 101, 100, 46, 1, 128, 0, 22, 0, 212, 121, 42, 204, 249, 233, 220, 62, 115, 134, 57, 253, 74, 205, 30, 94, 131, 235, 18, 101, 84, 52, 222, 139, 77, 186, 0, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 128, 0, 10, 0, 115, 191, 114, 198, 152, 3, 168, 151, 3, 167, 108, 107, 107, 212, 121, 42, 204, 249, 233, 220, 62, 115, 134, 57, 253, 74, 205, 30, 94, 131, 235, 18, 101, 84, 52, 222, 139, 77, 186, 0, 0, 0, 0, 0, 0, 0, 0, 185, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 146, 0, 15, 0, 119, 191, 145, 118, 191, 211, 0, 0, 114, 198, 152, 3, 168, 151, 3, 167, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 108, 108, 107, 107, 186, 0, 0, 0, 0, 0, 0, 0, 0, 212, 210, 191, 35, 22, 225, 99, 65, 123, 88, 60, 150, 249, 33, 160, 178, 175, 104, 179, 18, 22, 53, 132, 108, 84, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 170, 0, 11, 0, 114, 191, 211, 7, 0, 115, 198, 152, 3, 168, 151, 3, 167, 108, 107, 107, 6, 97, 100, 100, 95, 122, 113, 40, 3, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 115, 32, 111, 102, 32, 100, 101, 103, 114, 101, 101, 32, 53, 49, 50, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 97, 115, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 44, 10, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 97, 100, 100, 105, 116, 105, 111, 110, 46, 10, 73, 109, 97, 103, 105, 110, 101, 44, 32, 116, 119, 111, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 115, 32, 102, 44, 32, 103, 10, 104, 32, 61, 32, 102, 32, 43, 32, 103, 44, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 97, 115, 10, 91, 40, 102, 91, 105, 93, 32, 43, 32, 103, 91, 105, 93, 41, 32, 37, 32, 81, 32, 102, 111, 114, 32, 105, 32, 105, 110, 32, 114, 97, 110, 103, 101, 40, 53, 49, 50, 41, 93, 32, 124, 32, 81, 32, 61, 32, 49, 50, 50, 56, 57, 32, 40, 32, 61, 32, 70, 97, 108, 99, 111, 110, 32, 68, 105, 103, 105, 116, 97, 108, 32, 83, 105, 103, 110, 97, 116, 117, 114, 101, 32, 65, 108, 103, 111, 114, 105, 116, 104, 109, 39, 115, 32, 80, 114, 105, 109, 101, 32, 78, 117, 109, 98, 101, 114, 32, 41, 10, 73, 110, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 102, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 103, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 104, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 46, 46, 46, 93, 10, 45, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 95, 97, 100, 100, 114, 96, 105, 96, 32, 45, 62, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 91, 32, 40, 105, 32, 60, 60, 32, 50, 41, 32, 46, 46, 32, 40, 40, 105, 43, 49, 41, 32, 60, 60, 32, 50, 41, 32, 41, 44, 32, 97, 100, 100, 114, 101, 115, 115, 32, 104, 111, 108, 100, 105, 110, 103, 32, 102, 111, 117, 114, 32, 99, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 99, 111, 101, 102, 102, 105, 99, 105, 101, 110, 116, 115, 10, 45, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 95, 97, 100, 100, 114, 48, 32, 45, 62, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 10, 79, 117, 116, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 32, 46, 46, 46, 32, 93, 10, 67, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 49, 50, 55, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 102, 114, 111, 109, 32, 115, 116, 97, 114, 116, 105, 110, 103, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 32, 40, 32, 108, 105, 118, 105, 110, 103, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 116, 111, 112, 32, 41, 32, 98, 121, 10, 99, 111, 110, 116, 105, 110, 117, 105, 110, 103, 32, 116, 111, 32, 97, 112, 112, 108, 121, 32, 96, 73, 78, 67, 82, 96, 32, 40, 32, 61, 32, 97, 100, 100, 46, 49, 32, 41, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 111, 110, 32, 112, 114, 101, 118, 105, 111, 117, 115, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 46, 10, 78, 111, 116, 101, 44, 32, 105, 110, 112, 117, 116, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 99, 111, 110, 115, 105, 100, 101, 114, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 97, 100, 45, 111, 110, 108, 121, 44, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 32, 109, 117, 116, 97, 116, 101, 100, 46, 1, 0, 0, 6, 0, 185, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 189, 0, 19, 0, 118, 191, 145, 119, 191, 211, 1, 0, 211, 3, 0, 116, 198, 153, 3, 169, 152, 3, 168, 151, 3, 167, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 108, 108, 108, 6, 110, 101, 103, 95, 122, 113, 7, 3, 71, 105, 118, 101, 110, 32, 111, 110, 101, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 111, 102, 32, 100, 101, 103, 114, 101, 101, 32, 53, 49, 50, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 97, 115, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 44, 10, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 110, 101, 103, 97, 116, 101, 115, 32, 101, 97, 99, 104, 32, 99, 111, 101, 102, 102, 105, 99, 105, 101, 110, 116, 32, 111, 102, 32, 116, 104, 97, 116, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 46, 10, 73, 109, 97, 103, 105, 110, 101, 44, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 102, 10, 103, 32, 61, 32, 45, 102, 44, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 97, 115, 10, 91, 40, 45, 102, 91, 105, 93, 41, 32, 37, 32, 81, 32, 102, 111, 114, 32, 105, 32, 105, 110, 32, 114, 97, 110, 103, 101, 40, 53, 49, 50, 41, 93, 32, 124, 32, 81, 32, 61, 32, 49, 50, 50, 56, 57, 32, 40, 32, 61, 32, 70, 97, 108, 99, 111, 110, 32, 68, 105, 103, 105, 116, 97, 108, 32, 83, 105, 103, 110, 97, 116, 117, 114, 101, 32, 65, 108, 103, 111, 114, 105, 116, 104, 109, 39, 115, 32, 80, 114, 105, 109, 101, 32, 78, 117, 109, 98, 101, 114, 32, 41, 10, 73, 110, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 102, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 103, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 46, 46, 46, 93, 10, 45, 32, 123, 102, 44, 103, 125, 95, 97, 100, 100, 114, 96, 105, 96, 32, 45, 62, 32, 123, 102, 44, 103, 125, 91, 32, 40, 105, 32, 60, 60, 32, 50, 41, 32, 46, 46, 32, 40, 40, 105, 43, 49, 41, 32, 60, 60, 32, 50, 41, 32, 41, 44, 32, 97, 100, 100, 114, 101, 115, 115, 32, 104, 111, 108, 100, 105, 110, 103, 32, 102, 111, 117, 114, 32, 99, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 99, 111, 101, 102, 102, 105, 99, 105, 101, 110, 116, 115, 10, 45, 32, 123, 102, 44, 103, 125, 95, 97, 100, 100, 114, 48, 32, 45, 62, 32, 123, 102, 44, 103, 125, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 10, 79, 117, 116, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 32, 46, 46, 46, 32, 93, 10, 67, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 49, 50, 55, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 102, 114, 111, 109, 32, 115, 116, 97, 114, 116, 105, 110, 103, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 32, 40, 32, 108, 105, 118, 105, 110, 103, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 116, 111, 112, 32, 41, 32, 98, 121, 10, 99, 111, 110, 116, 105, 110, 117, 105, 110, 103, 32, 116, 111, 32, 97, 112, 112, 108, 121, 32, 96, 73, 78, 67, 82, 96, 32, 40, 32, 61, 32, 97, 100, 100, 46, 49, 32, 41, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 111, 110, 32, 112, 114, 101, 118, 105, 111, 117, 115, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 46, 10, 78, 111, 116, 101, 44, 32, 105, 110, 112, 117, 116, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 99, 111, 110, 115, 105, 100, 101, 114, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 97, 100, 45, 111, 110, 108, 121, 44, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 32, 109, 117, 116, 97, 116, 101, 100, 46, 1, 0, 0, 5, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254, 217, 0, 11, 0, 114, 191, 211, 5, 0, 115, 198, 152, 3, 168, 151, 3, 167, 108, 107, 107, 6, 115, 117, 98, 95, 122, 113, 53, 3, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 115, 32, 111, 102, 32, 100, 101, 103, 114, 101, 101, 32, 53, 49, 50, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 97, 115, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 44, 10, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 115, 117, 98, 116, 114, 97, 99, 116, 115, 32, 115, 101, 99, 111, 110, 100, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 102, 114, 111, 109, 32, 102, 105, 114, 115, 116, 32, 111, 110, 101, 46, 10, 73, 109, 97, 103, 105, 110, 101, 44, 32, 116, 119, 111, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 115, 32, 102, 44, 32, 103, 10, 104, 32, 61, 32, 102, 32, 45, 32, 103, 44, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 97, 115, 10, 91, 40, 102, 91, 105, 93, 32, 45, 32, 103, 91, 105, 93, 41, 32, 37, 32, 81, 32, 102, 111, 114, 32, 105, 32, 105, 110, 32, 114, 97, 110, 103, 101, 40, 53, 49, 50, 41, 93, 32, 124, 32, 81, 32, 61, 32, 49, 50, 50, 56, 57, 32, 40, 32, 61, 32, 70, 97, 108, 99, 111, 110, 32, 68, 105, 103, 105, 116, 97, 108, 32, 83, 105, 103, 110, 97, 116, 117, 114, 101, 32, 65, 108, 103, 111, 114, 105, 116, 104, 109, 39, 115, 32, 80, 114, 105, 109, 101, 32, 78, 117, 109, 98, 101, 114, 32, 41, 10, 73, 110, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 102, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 103, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 44, 32, 104, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 32, 46, 46, 46, 93, 10, 45, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 95, 97, 100, 100, 114, 96, 105, 96, 32, 45, 62, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 91, 32, 40, 105, 32, 60, 60, 32, 50, 41, 32, 46, 46, 32, 40, 40, 105, 43, 49, 41, 32, 60, 60, 32, 50, 41, 32, 41, 44, 32, 97, 100, 100, 114, 101, 115, 115, 32, 104, 111, 108, 100, 105, 110, 103, 32, 102, 111, 117, 114, 32, 99, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 99, 111, 101, 102, 102, 105, 99, 105, 101, 110, 116, 115, 10, 45, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 95, 97, 100, 100, 114, 48, 32, 45, 62, 32, 123, 102, 44, 32, 103, 44, 32, 104, 125, 95, 115, 116, 97, 114, 116, 95, 97, 100, 100, 114, 10, 79, 117, 116, 112, 117, 116, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 32, 46, 46, 46, 32, 93, 10, 67, 111, 110, 115, 101, 99, 117, 116, 105, 118, 101, 32, 49, 50, 55, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 99, 97, 110, 32, 98, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 102, 114, 111, 109, 32, 115, 116, 97, 114, 116, 105, 110, 103, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 32, 40, 32, 108, 105, 118, 105, 110, 103, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 116, 111, 112, 32, 41, 32, 98, 121, 10, 99, 111, 110, 116, 105, 110, 117, 105, 110, 103, 32, 116, 111, 32, 97, 112, 112, 108, 121, 32, 96, 73, 78, 67, 82, 96, 32, 40, 32, 61, 32, 97, 100, 100, 46, 49, 32, 41, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 111, 110, 32, 112, 114, 101, 118, 105, 111, 117, 115, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 46, 10, 78, 111, 116, 101, 44, 32, 105, 110, 112, 117, 116, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 99, 111, 110, 115, 105, 100, 101, 114, 101, 100, 32, 116, 111, 32, 98, 101, 32, 114, 101, 97, 100, 45, 111, 110, 108, 121, 44, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 110, 111, 116, 32, 109, 117, 116, 97, 116, 101, 100, 46, 1, 128, 0, 5, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 149, 211, 10, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 211, 9, 0]),
("std::math::secp256k1",vm_assembly::ProcedureId([222, 223, 0, 199, 92, 172, 8, 99, 250, 250, 88, 55, 52, 247, 239, 239, 23, 59, 237, 206, 18, 129, 229, 57]),"#! Given [b, c, a, carry] on stack top, following function computes
#!
#!  tmp = a + (b * c) + carry
#!  hi = tmp >> 32
#!  lo = tmp & 0xffff_ffff
#!  return (hi, lo)
#!
#! At end of execution of this function, stack top should look like [hi, lo]
#! See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/utils.py#L75-L80
proc.mac
  u32overflowing_madd

  movdn.2
  u32overflowing_add

  movup.2
  add
end

#! Given [a, b, borrow] on stack top, following function computes
#!
#!  tmp = a - (b + borrow)
#!  hi = tmp >> 32
#!  lo = tmp & 0xffff_ffff
#!  return (hi, lo)
#!
#! At end of execution of this function, stack top should look like [hi, lo]
#! See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/utils.py#L83-L89
proc.sbb
  movdn.2
  add
  u32overflowing_sub
end

#! Given a secp256k1 field element in radix-2^32 representation and 32 -bit unsigned integer,
#! this routine computes a 288 -bit number.
#!
#! Input via stack is expected in this form
#!
#! [a0, a1, a2, a3, a4, a5, a6, a7, b] | a[0..8] -> 256 -bit number, b = 32 -bit number
#!
#! Computed output looks like below, on stack
#!
#! [carry, b7, b6, b5, b4, b3, b2, b1, b0]
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

#! Given a 288 -bit number and 256 -bit number on stack ( in order ), this routine
#! computes a 288 -bit number
#!
#! Expected stack state during routine invocation
#!
#! [carry, b7, b6, b5, b4, b3, b2, b1, b0, c0, c1, c2, c3, c4, c5, c6, c7]
#!
#! While after execution of this routine, stack should look like
#!
#! [d0, d1, d2, d3, d4, d5, d6, d7, carry]
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

#! Given [c0, c1, c2, c3, c4, c5, c6, c7, c8, pc] on stack top,
#! this function attempts to reduce 288 -bit number to 256 -bit number
#! along with carry, using montgomery reduction method
#!
#! In stack top content c[0..9] i.e. first 9 elements, holding 288 -bit
#! number. Stack element `pc` ( at stack[9] ) is previous reduction round's
#! carry ( for first reduction round, it'll be set to 0 ).
#!
#! After finishing execution of this function, stack top should look like
#!
#! [c0, c1, c2, c3, c4, c5, c6, c7, pc] | pc = next round's carry
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

#! Given two 256 -bit numbers on stack, where each number is represented in
#! radix-2^32 form ( i.e. each number having eight 32 -bit limbs ), following function
#! computes modular multiplication of those two operands, computing 256 -bit result.
#!
#! Stack expected as below, holding input
#!
#! [a0, a1, a2, a3, a4, a5, a6, a7, b0, b1, b2, b3, b4, b5, b6, b7] | a[0..8], b[0..8] are 256 -bit numbers
#!
#! After finishing execution of this function, stack should look like
#!
#! [c0, c1, c2, c3, c4, c5, c6, c7] | c[0..8] is a 256 -bit number
#!
#! Note, for computing modular multiplication of a[0..8] & b[0..8],
#! school book multiplication equipped with montgomery reduction technique
#! is used, which is why a[0..8], b[0..8] are expected to be in montgomery form,
#! while computed c[0..8] will also be in montgomery form.
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

#! Given two 256 -bit numbers on stack, where each number is represented in
#! radix-2^32 form ( i.e. each number having eight 32 -bit limbs ), following function
#! computes modular addition of those two operands, in secp256k1 prime field.
#!
#! Stack expected as below, holding input
#!
#! [a0, a1, a2, a3, a4, a5, a6, a7, b0, b1, b2, b3, b4, b5, b6, b7] | a[0..8], b[0..8] are 256 -bit numbers
#!
#! After finishing execution of this function, stack should look like
#!
#! [c0, c1, c2, c3, c4, c5, c6, c7] | c[0..8] is a 256 -bit number
#!
#! This implementation takes inspiration from https://gist.github.com/itzmeanjan/d4853347dfdfa853993f5ea059824de6#file-test_montgomery_arithmetic-py-L236-L256
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

#! Given a secp256k1 field element ( say `a` ) on stack, represented in Montgomery form 
#! ( i.e. number having eight 32 -bit limbs ), following function negates it to
#! field element `a'` | a' + a = 0
#!
#! Stack expected as below, holding input
#!
#! [a0, a1, a2, a3, a4, a5, a6, a7] | a[0..8] is a secp256k1 field element
#!
#! After finishing execution of this function, stack should look like
#!
#! [c0, c1, c2, c3, c4, c5, c6, c7] | c[0..8] is a secp256k1 field element
#!
#! See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/field.py#L77-L95
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

#! Given two secp256k1 field elements, say a, b, ( represented in Montgomery form, each number having 
#! eight 32 -bit limbs ) on stack, following function computes modular subtraction of those 
#! two operands c = a + (-b) = a - b
#!
#! Stack expected as below, holding input
#!
#! [a0, a1, a2, a3, a4, a5, a6, a7, b0, b1, b2, b3, b4, b5, b6, b7] | a[0..8], b[0..8] are secp256k1 field elements
#!
#! After finishing execution of this function, stack should look like
#!
#! [c0, c1, c2, c3, c4, c5, c6, c7] | c[0..8] is a secp256k1 field element
#!
#! See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/field.py#L97-L101
export.u256_mod_sub
  movupw.3
  movupw.3

  exec.u256_mod_neg
  exec.u256_mod_add
end

#! Given a 256 -bit number on stack, represented in radix-2^32 
#! form i.e. eight 32 -bit limbs, this routine computes Montgomery
#! representation of provided radix-2^32 number.
#!
#! - u256 radix-2^32 form input expected on stack as
#!
#!  [a0, a1, a2, a3, a4, a5, a6, a7]
#!
#! - u256 montgomery form output on stack
#!
#! [a0`, a1`, a2`, a3`, a4`, a5`, a6`, a7`]
#!
#! See section 2.2 of https://eprint.iacr.org/2017/1057.pdf
export.to_mont
  push.0.0.0.0
  push.0.1.1954.954529 # pushed R2's radix-2^32 form;
                       # see https://gist.github.com/itzmeanjan/d4853347dfdfa853993f5ea059824de6

  exec.u256_mod_mul
end

#! Given a 256 -bit number on stack, represented in Montgomery 
#! form i.e. eight 32 -bit limbs, this routine computes radix-2^32
#! representation of provided u256 number.
#!
#! - u256 montgomery form input on stack expected
#!
#!  [a0, a1, a2, a3, a4, a5, a6, a7]
#!
#! - u256 radix-2^32 form output on stack as
#!
#! [a0`, a1`, a2`, a3`, a4`, a5`, a6`, a7`]
#!
#! See section 2.2 of https://eprint.iacr.org/2017/1057.pdf
export.from_mont
  push.0.0.0.0
  push.0.0.0.1 # pushed 1's radix-2^32 form;
               # see https://gist.github.com/itzmeanjan/d4853347dfdfa853993f5ea059824de6

  exec.u256_mod_mul
end

#! Given a secp256k1 point in projective coordinate system ( i.e. with x, y, z -coordinates
#! as secp256k1 prime field elements, represented in Montgomery form ), this routine adds 
#! that point with self i.e. does point doubling on elliptic curve, using exception-free 
#! doubling formula from algorithm 9 of https://eprint.iacr.org/2015/1060.pdf, while 
#! following prototype implementation https://github.com/itzmeanjan/secp256k1/blob/ec3652a/point.py#L131-L165
#! 
#! Input:
#!
#! 12 memory addresses on stack such that first 6 memory addresses are for input point &
#! last 6 are for storing resulting point.
#!
#! First 6 addresses hold input elliptic curve point's x, y, z -coordinates, where each coordinate
#! is represented in Montgomery form, as eight 32 -bit limbs.
#!
#! Similarly, last 6 addresses hold resulting (doubled) point's x, y, z -coordinates, where each
#! coordinate is represented in Montgomery form, as eight 32 -bit limbs. Note, this is where
#! output will be written, so called is expected to read doubled point from last 6 memory addresses.
#!
#! Expected stack during invocation of this routine:
#!
#!   [x_addr[0..4], x_addr[4..8], y_addr[0..4], y_addr[4..8], z_addr[0..4], z_addr[4..8], 
#!     x3_addr[0..4], x3_addr[4..8], y3_addr[0..4], y3_addr[4..8], z3_addr[0..4], z3_addr[4..8]]
#!
#! Note, (X, Y, Z)    => input point
#!       (X3, Y3, Z3) => output point
#!
#! Output:
#!
#! Last 6 memory addresses of 12 memory addresses which were provided during invocation, where resulting doubled
#! point is kept in similar form. For seeing X3, Y3, Z3 -coordinates of doubled point, one needs to read from
#! those 6 memory addresses.
#!
#! Stack at end of execution of routine looks like
#!
#!   [x3_addr[0..4], x3_addr[4..8], y3_addr[0..4], y3_addr[4..8], z3_addr[0..4], z3_addr[4..8]]
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

#! Given two secp256k1 points in projective coordinate system ( i.e. with x, y, z -coordinates
#! as secp256k1 prime field elements, represented in Montgomery form, each coordinate using eight 32 -bit limbs ),
#! this routine adds those two points on elliptic curve, using exception-free addition formula from
#! algorithm 7 of https://eprint.iacr.org/2015/1060.pdf, while following prototype
#! implementation https://github.com/itzmeanjan/secp256k1/blob/ec3652a/point.py#L60-L115
#! 
#! Input:
#!
#! 18 memory addresses on stack such that first 6 memory addresses are for first input point, next 6
#! memory addresses holding x, y, z -coordinates of second input point & last 6 addresses are for storing 
#! resulting point ( addition of two input points ).
#!
#! Expected stack during invocation of this routine:
#!
#!   [x1_addr[0..4], x1_addr[4..8], y1_addr[0..4], y1_addr[4..8], z1_addr[0..4], z1_addr[4..8], 
#!     x2_addr[0..4], x2_addr[4..8], y2_addr[0..4], y2_addr[4..8], z2_addr[0..4], z2_addr[4..8],
#!       x3_addr[0..4], x3_addr[4..8], y3_addr[0..4], y3_addr[4..8], z3_addr[0..4], z3_addr[4..8]]
#!
#! Note, (X1, Y1, Z1)    => input point 1
#!       (X2, Y2, Z2)    => input point 2
#!       (X3, Y3, Z3)    => output point
#!
#! Output:
#!
#! Last 6 memory addresses of 18 input memory addresses which were provided during invocation, where resulting elliptic curve
#! point is kept in similar form. For seeing X3, Y3, Z3 -coordinates of doubled point, one needs to read from
#! those 6 memory addresses.
#!
#! Stack at end of execution of routine looks like
#!
#!   [x3_addr[0..4], x3_addr[4..8], y3_addr[0..4], y3_addr[4..8], z3_addr[0..4], z3_addr[4..8]]
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

#! Given an elliptic curve point in projective coordinate system ( total 24 field elements 
#! required for representing x, y, z coordinate values s.t. they are provided by 6 distinct 
#! memory addresses ) and a 256 -bit scalar, in radix-2^32 representation ( such that it 
#! takes 8 stack elements to represent whole scalar, where each limb is of 32 -bit width ), 
#! this routine multiplies elliptic curve point by given scalar, producing another point 
#! on secp256k1 curve, which will also be presented in projective coordinate system.
#!
#! Input:
#!
#! During invocation, this routine expects stack in following form
#!
#! [X_addr_0, X_addr_1, Y_addr_0, Y_addr_1, Z_addr_0, Z_addr_1, Sc0, Sc1, Sc2, Sc3, Sc4, Sc5, Sc6, Sc7, X'_addr_0, X'_addr_1, Y'_addr_0, Y'_addr_1, Z'_addr_0, Z'_addr_1, ...]
#!
#! X_addr_0, X_addr_1 -> Input secp256k1 point's X -coordinate to be placed, in Montgomery form, in given addresses
#! Y_addr_0, Y_addr_1 -> Input secp256k1 point's Y -coordinate to be placed, in Montgomery form, in given addresses
#! Z_addr_1, Z_addr_1 -> Input secp256k1 point's Z -coordinate to be placed, in Montgomery form, in given addresses
#! Sc{0..8}           -> 256 -bit scalar in radix-2^32 form | Sc0 is least significant limb & Sc7 is most significant limb
#! X'_addr_0, X'_addr_1 -> Resulting secp256k1 point's X -coordinate to be placed, in Montgomery form, in given addresses
#! Y'_addr_0, Y'_addr_1 -> Resulting secp256k1 point's Y -coordinate to be placed, in Montgomery form, in given addresses
#! Z'_addr_1, Z'_addr_1 -> Resulting secp256k1 point's Z -coordinate to be placed, in Montgomery form, in given addresses
#!
#! Output:
#!
#! At end of execution of this routine, stack should look like below
#!
#! [X_addr_0, X_addr_1, Y_addr_0, Y_addr_1, Z_addr_0, Z_addr_1, ...]
#!
#! X_addr_0, X_addr_1 -> Resulting secp256k1 point's X -coordinate written, in Montgomery form, in given addresses
#! Y_addr_0, Y_addr_1 -> Resulting secp256k1 point's Y -coordinate written, in Montgomery form, in given addresses
#! Z_addr_0, Z_addr_1 -> Resulting secp256k1 point's Z -coordinate written, in Montgomery form, in given addresses
#!
#! One interested in resulting point, should read from provided addresses on stack.
#! 
#! This routine implements double-and-add algorithm, while following 
#! https://github.com/itzmeanjan/secp256k1/blob/d23ea7d/point.py#L174-L186
#!
#! If base point being multiplied is secp256k1 curve generator point, one should use `gen_point` routine,
#! which is almost 2x faster !
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

#! Given a 256 -bit scalar, in radix-2^32 representation ( such that it takes 8 stack elements
#! to represent whole scalar, where each limb is of 32 -bit width ), this routine multiplies
#! secp256k1 generator point ( in projective coordinate system ) with given scalar, producing
#! another point on secp256k1 curve, which will also be presented in projective coordinate
#! system.
#!
#! Input:
#!
#! During invocation, this routine expects stack in following form
#!
#! [Sc0, Sc1, Sc2, Sc3, Sc4, Sc5, Sc6, Sc7, X_addr_0, X_addr_1, Y_addr_0, Y_addr_1, Z_addr_0, Z_addr_1, ...]
#!
#! Sc{0..8}           -> 256 -bit scalar in radix-2^32 form | Sc0 is least significant limb & Sc7 is most significant limb
#! X_addr_0, X_addr_1 -> Resulting secp256k1 point's X -coordinate to be placed, in Montgomery form, in given addresses
#! Y_addr_0, Y_addr_1 -> Resulting secp256k1 point's Y -coordinate to be placed, in Montgomery form, in given addresses
#! Z_addr_1, Z_addr_1 -> Resulting secp256k1 point's Z -coordinate to be placed, in Montgomery form, in given addresses
#!
#! Output:
#!
#! At end of execution of this routine, stack should look like below
#!
#! [X_addr_0, X_addr_1, Y_addr_0, Y_addr_1, Z_addr_0, Z_addr_1, ...]
#!
#! X_addr_0, X_addr_1 -> Resulting secp256k1 point's X -coordinate written, in Montgomery form, in given addresses
#! Y_addr_0, Y_addr_1 -> Resulting secp256k1 point's Y -coordinate written, in Montgomery form, in given addresses
#! Z_addr_0, Z_addr_1 -> Resulting secp256k1 point's Z -coordinate written, in Montgomery form, in given addresses
#!
#! One interested in resulting point, should read from provided address on stack.
#! 
#! This routine implements double-and-add algorithm, while following 
#! https://github.com/itzmeanjan/secp256k1/blob/d23ea7d/point.py#L174-L186 
#!
#! Note, this routine is a specialised instantiation of secp256k1 point multiplication, where we know what the base
#! point is, so we enjoy faster computation ( because all point doublings can be precomputed, saving us 256 point doublings ! ).
export.gen_mul.20
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
",&[15, 0, 3, 109, 97, 99, 0, 0, 0, 0, 0, 5, 0, 57, 165, 41, 149, 3, 3, 115, 98, 98, 0, 0, 0, 0, 0, 3, 0, 165, 3, 49, 8, 117, 50, 53, 54, 120, 117, 51, 50, 0, 0, 0, 0, 0, 26, 0, 155, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 111, 150, 57, 112, 151, 57, 113, 152, 57, 114, 153, 57, 115, 154, 57, 116, 155, 57, 117, 156, 57, 155, 156, 57, 13, 117, 50, 56, 56, 95, 97, 100, 100, 95, 117, 50, 53, 54, 0, 0, 0, 0, 0, 42, 0, 145, 163, 41, 149, 154, 43, 150, 153, 43, 151, 152, 163, 149, 151, 153, 43, 152, 152, 43, 150, 151, 163, 149, 151, 153, 43, 152, 152, 43, 157, 152, 43, 151, 3, 130, 149, 150, 151, 152, 153, 154, 155, 11, 117, 50, 56, 56, 95, 114, 101, 100, 117, 99, 101, 0, 0, 0, 0, 0, 50, 0, 110, 185, 1, 49, 53, 37, 210, 0, 0, 0, 0, 53, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 149, 185, 1, 47, 252, 255, 255, 0, 0, 0, 0, 113, 211, 0, 0, 130, 107, 149, 185, 1, 254, 255, 255, 255, 0, 0, 0, 0, 113, 211, 0, 0, 150, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 114, 211, 0, 0, 151, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 115, 211, 0, 0, 152, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 116, 211, 0, 0, 153, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 117, 211, 0, 0, 154, 117, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 211, 0, 0, 154, 155, 130, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 211, 0, 0, 156, 156, 43, 130, 149, 150, 151, 152, 153, 154, 155, 12, 117, 50, 53, 54, 95, 109, 111, 100, 95, 109, 117, 108, 16, 3, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 110, 117, 109, 98, 101, 114, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 119, 104, 101, 114, 101, 32, 101, 97, 99, 104, 32, 110, 117, 109, 98, 101, 114, 32, 105, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 10, 114, 97, 100, 105, 120, 45, 50, 94, 51, 50, 32, 102, 111, 114, 109, 32, 40, 32, 105, 46, 101, 46, 32, 101, 97, 99, 104, 32, 110, 117, 109, 98, 101, 114, 32, 104, 97, 118, 105, 110, 103, 32, 101, 105, 103, 104, 116, 32, 51, 50, 32, 45, 98, 105, 116, 32, 108, 105, 109, 98, 115, 32, 41, 44, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 102, 117, 110, 99, 116, 105, 111, 110, 10, 99, 111, 109, 112, 117, 116, 101, 115, 32, 109, 111, 100, 117, 108, 97, 114, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 111, 115, 101, 32, 116, 119, 111, 32, 111, 112, 101, 114, 97, 110, 100, 115, 44, 32, 99, 111, 109, 112, 117, 116, 105, 110, 103, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 114, 101, 115, 117, 108, 116, 46, 10, 83, 116, 97, 99, 107, 32, 101, 120, 112, 101, 99, 116, 101, 100, 32, 97, 115, 32, 98, 101, 108, 111, 119, 44, 32, 104, 111, 108, 100, 105, 110, 103, 32, 105, 110, 112, 117, 116, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 97, 53, 44, 32, 97, 54, 44, 32, 97, 55, 44, 32, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 98, 53, 44, 32, 98, 54, 44, 32, 98, 55, 93, 32, 124, 32, 97, 91, 48, 46, 46, 56, 93, 44, 32, 98, 91, 48, 46, 46, 56, 93, 32, 97, 114, 101, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 110, 117, 109, 98, 101, 114, 115, 10, 65, 102, 116, 101, 114, 32, 102, 105, 110, 105, 115, 104, 105, 110, 103, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 105, 115, 32, 102, 117, 110, 99, 116, 105, 111, 110, 44, 32, 115, 116, 97, 99, 107, 32, 115, 104, 111, 117, 108, 100, 32, 108, 111, 111, 107, 32, 108, 105, 107, 101, 10, 91, 99, 48, 44, 32, 99, 49, 44, 32, 99, 50, 44, 32, 99, 51, 44, 32, 99, 52, 44, 32, 99, 53, 44, 32, 99, 54, 44, 32, 99, 55, 93, 32, 124, 32, 99, 91, 48, 46, 46, 56, 93, 32, 105, 115, 32, 97, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 110, 117, 109, 98, 101, 114, 10, 78, 111, 116, 101, 44, 32, 102, 111, 114, 32, 99, 111, 109, 112, 117, 116, 105, 110, 103, 32, 109, 111, 100, 117, 108, 97, 114, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 97, 91, 48, 46, 46, 56, 93, 32, 38, 32, 98, 91, 48, 46, 46, 56, 93, 44, 10, 115, 99, 104, 111, 111, 108, 32, 98, 111, 111, 107, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 101, 113, 117, 105, 112, 112, 101, 100, 32, 119, 105, 116, 104, 32, 109, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 114, 101, 100, 117, 99, 116, 105, 111, 110, 32, 116, 101, 99, 104, 110, 105, 113, 117, 101, 10, 105, 115, 32, 117, 115, 101, 100, 44, 32, 119, 104, 105, 99, 104, 32, 105, 115, 32, 119, 104, 121, 32, 97, 91, 48, 46, 46, 56, 93, 44, 32, 98, 91, 48, 46, 46, 56, 93, 32, 97, 114, 101, 32, 101, 120, 112, 101, 99, 116, 101, 100, 32, 116, 111, 32, 98, 101, 32, 105, 110, 32, 109, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 10, 119, 104, 105, 108, 101, 32, 99, 111, 109, 112, 117, 116, 101, 100, 32, 99, 91, 48, 46, 46, 56, 93, 32, 119, 105, 108, 108, 32, 97, 108, 115, 111, 32, 98, 101, 32, 105, 110, 32, 109, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 46, 1, 2, 0, 81, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 145, 200, 1, 0, 0, 0, 0, 0, 0, 0, 145, 211, 2, 0, 130, 149, 150, 151, 152, 153, 154, 155, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 172, 211, 4, 0, 156, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 2, 0, 211, 3, 0, 211, 4, 0, 156, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 2, 0, 211, 3, 0, 211, 4, 0, 156, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 2, 0, 211, 3, 0, 211, 4, 0, 156, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 2, 0, 211, 3, 0, 211, 4, 0, 156, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 2, 0, 211, 3, 0, 211, 4, 0, 156, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 2, 0, 211, 3, 0, 211, 4, 0, 156, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 2, 0, 211, 3, 0, 211, 4, 0, 155, 149, 111, 3, 149, 149, 185, 1, 209, 3, 0, 0, 0, 0, 0, 0, 57, 107, 12, 117, 50, 53, 54, 95, 109, 111, 100, 95, 97, 100, 100, 150, 2, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 110, 117, 109, 98, 101, 114, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 119, 104, 101, 114, 101, 32, 101, 97, 99, 104, 32, 110, 117, 109, 98, 101, 114, 32, 105, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 10, 114, 97, 100, 105, 120, 45, 50, 94, 51, 50, 32, 102, 111, 114, 109, 32, 40, 32, 105, 46, 101, 46, 32, 101, 97, 99, 104, 32, 110, 117, 109, 98, 101, 114, 32, 104, 97, 118, 105, 110, 103, 32, 101, 105, 103, 104, 116, 32, 51, 50, 32, 45, 98, 105, 116, 32, 108, 105, 109, 98, 115, 32, 41, 44, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 102, 117, 110, 99, 116, 105, 111, 110, 10, 99, 111, 109, 112, 117, 116, 101, 115, 32, 109, 111, 100, 117, 108, 97, 114, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 111, 115, 101, 32, 116, 119, 111, 32, 111, 112, 101, 114, 97, 110, 100, 115, 44, 32, 105, 110, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 114, 105, 109, 101, 32, 102, 105, 101, 108, 100, 46, 10, 83, 116, 97, 99, 107, 32, 101, 120, 112, 101, 99, 116, 101, 100, 32, 97, 115, 32, 98, 101, 108, 111, 119, 44, 32, 104, 111, 108, 100, 105, 110, 103, 32, 105, 110, 112, 117, 116, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 97, 53, 44, 32, 97, 54, 44, 32, 97, 55, 44, 32, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 98, 53, 44, 32, 98, 54, 44, 32, 98, 55, 93, 32, 124, 32, 97, 91, 48, 46, 46, 56, 93, 44, 32, 98, 91, 48, 46, 46, 56, 93, 32, 97, 114, 101, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 110, 117, 109, 98, 101, 114, 115, 10, 65, 102, 116, 101, 114, 32, 102, 105, 110, 105, 115, 104, 105, 110, 103, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 105, 115, 32, 102, 117, 110, 99, 116, 105, 111, 110, 44, 32, 115, 116, 97, 99, 107, 32, 115, 104, 111, 117, 108, 100, 32, 108, 111, 111, 107, 32, 108, 105, 107, 101, 10, 91, 99, 48, 44, 32, 99, 49, 44, 32, 99, 50, 44, 32, 99, 51, 44, 32, 99, 52, 44, 32, 99, 53, 44, 32, 99, 54, 44, 32, 99, 55, 93, 32, 124, 32, 99, 91, 48, 46, 46, 56, 93, 32, 105, 115, 32, 97, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 110, 117, 109, 98, 101, 114, 10, 84, 104, 105, 115, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 116, 97, 107, 101, 115, 32, 105, 110, 115, 112, 105, 114, 97, 116, 105, 111, 110, 32, 102, 114, 111, 109, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 115, 116, 46, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 105, 116, 122, 109, 101, 97, 110, 106, 97, 110, 47, 100, 52, 56, 53, 51, 51, 52, 55, 100, 102, 100, 102, 97, 56, 53, 51, 57, 57, 51, 102, 53, 101, 97, 48, 53, 57, 56, 50, 52, 100, 101, 54, 35, 102, 105, 108, 101, 45, 116, 101, 115, 116, 95, 109, 111, 110, 116, 103, 111, 109, 101, 114, 121, 95, 97, 114, 105, 116, 104, 109, 101, 116, 105, 99, 45, 112, 121, 45, 76, 50, 51, 54, 45, 76, 50, 53, 54, 1, 0, 0, 41, 0, 163, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 152, 43, 149, 152, 43, 150, 152, 43, 151, 152, 43, 152, 156, 43, 153, 156, 43, 154, 156, 43, 155, 156, 43, 155, 111, 185, 1, 209, 3, 0, 0, 0, 0, 0, 0, 57, 107, 130, 155, 3, 149, 150, 151, 152, 153, 154, 153, 154, 12, 117, 50, 53, 54, 95, 109, 111, 100, 95, 110, 101, 103, 38, 2, 71, 105, 118, 101, 110, 32, 97, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 32, 40, 32, 115, 97, 121, 32, 96, 97, 96, 32, 41, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 10, 40, 32, 105, 46, 101, 46, 32, 110, 117, 109, 98, 101, 114, 32, 104, 97, 118, 105, 110, 103, 32, 101, 105, 103, 104, 116, 32, 51, 50, 32, 45, 98, 105, 116, 32, 108, 105, 109, 98, 115, 32, 41, 44, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 101, 103, 97, 116, 101, 115, 32, 105, 116, 32, 116, 111, 10, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 32, 96, 97, 39, 96, 32, 124, 32, 97, 39, 32, 43, 32, 97, 32, 61, 32, 48, 10, 83, 116, 97, 99, 107, 32, 101, 120, 112, 101, 99, 116, 101, 100, 32, 97, 115, 32, 98, 101, 108, 111, 119, 44, 32, 104, 111, 108, 100, 105, 110, 103, 32, 105, 110, 112, 117, 116, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 97, 53, 44, 32, 97, 54, 44, 32, 97, 55, 93, 32, 124, 32, 97, 91, 48, 46, 46, 56, 93, 32, 105, 115, 32, 97, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 10, 65, 102, 116, 101, 114, 32, 102, 105, 110, 105, 115, 104, 105, 110, 103, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 105, 115, 32, 102, 117, 110, 99, 116, 105, 111, 110, 44, 32, 115, 116, 97, 99, 107, 32, 115, 104, 111, 117, 108, 100, 32, 108, 111, 111, 107, 32, 108, 105, 107, 101, 10, 91, 99, 48, 44, 32, 99, 49, 44, 32, 99, 50, 44, 32, 99, 51, 44, 32, 99, 52, 44, 32, 99, 53, 44, 32, 99, 54, 44, 32, 99, 55, 93, 32, 124, 32, 99, 91, 48, 46, 46, 56, 93, 32, 105, 115, 32, 97, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 105, 116, 122, 109, 101, 97, 110, 106, 97, 110, 47, 115, 101, 99, 112, 50, 53, 54, 107, 49, 47, 98, 108, 111, 98, 47, 101, 99, 51, 54, 53, 50, 97, 102, 101, 56, 101, 100, 55, 50, 98, 50, 57, 98, 48, 101, 51, 57, 50, 55, 51, 97, 56, 55, 54, 97, 56, 57, 56, 51, 49, 54, 102, 98, 57, 97, 47, 102, 105, 101, 108, 100, 46, 112, 121, 35, 76, 55, 55, 45, 76, 57, 53, 1, 0, 0, 33, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 130, 185, 1, 47, 252, 255, 255, 0, 0, 0, 0, 211, 1, 0, 149, 185, 1, 254, 255, 255, 255, 0, 0, 0, 0, 211, 1, 0, 150, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 211, 1, 0, 151, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 211, 1, 0, 152, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 211, 1, 0, 153, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 211, 1, 0, 154, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 211, 1, 0, 155, 185, 1, 255, 255, 255, 255, 0, 0, 0, 0, 211, 1, 0, 107, 130, 149, 150, 151, 152, 153, 154, 12, 117, 50, 53, 54, 95, 109, 111, 100, 95, 115, 117, 98, 108, 2, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 115, 44, 32, 115, 97, 121, 32, 97, 44, 32, 98, 44, 32, 40, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 101, 97, 99, 104, 32, 110, 117, 109, 98, 101, 114, 32, 104, 97, 118, 105, 110, 103, 10, 101, 105, 103, 104, 116, 32, 51, 50, 32, 45, 98, 105, 116, 32, 108, 105, 109, 98, 115, 32, 41, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 109, 111, 100, 117, 108, 97, 114, 32, 115, 117, 98, 116, 114, 97, 99, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 111, 115, 101, 10, 116, 119, 111, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 99, 32, 61, 32, 97, 32, 43, 32, 40, 45, 98, 41, 32, 61, 32, 97, 32, 45, 32, 98, 10, 83, 116, 97, 99, 107, 32, 101, 120, 112, 101, 99, 116, 101, 100, 32, 97, 115, 32, 98, 101, 108, 111, 119, 44, 32, 104, 111, 108, 100, 105, 110, 103, 32, 105, 110, 112, 117, 116, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 97, 53, 44, 32, 97, 54, 44, 32, 97, 55, 44, 32, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 98, 53, 44, 32, 98, 54, 44, 32, 98, 55, 93, 32, 124, 32, 97, 91, 48, 46, 46, 56, 93, 44, 32, 98, 91, 48, 46, 46, 56, 93, 32, 97, 114, 101, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 115, 10, 65, 102, 116, 101, 114, 32, 102, 105, 110, 105, 115, 104, 105, 110, 103, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 105, 115, 32, 102, 117, 110, 99, 116, 105, 111, 110, 44, 32, 115, 116, 97, 99, 107, 32, 115, 104, 111, 117, 108, 100, 32, 108, 111, 111, 107, 32, 108, 105, 107, 101, 10, 91, 99, 48, 44, 32, 99, 49, 44, 32, 99, 50, 44, 32, 99, 51, 44, 32, 99, 52, 44, 32, 99, 53, 44, 32, 99, 54, 44, 32, 99, 55, 93, 32, 124, 32, 99, 91, 48, 46, 46, 56, 93, 32, 105, 115, 32, 97, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 105, 116, 122, 109, 101, 97, 110, 106, 97, 110, 47, 115, 101, 99, 112, 50, 53, 54, 107, 49, 47, 98, 108, 111, 98, 47, 101, 99, 51, 54, 53, 50, 97, 102, 101, 56, 101, 100, 55, 50, 98, 50, 57, 98, 48, 101, 51, 57, 50, 55, 51, 97, 56, 55, 54, 97, 56, 57, 56, 51, 49, 54, 102, 98, 57, 97, 47, 102, 105, 101, 108, 100, 46, 112, 121, 35, 76, 57, 55, 45, 76, 49, 48, 49, 1, 0, 0, 4, 0, 164, 164, 211, 7, 0, 211, 6, 0, 7, 116, 111, 95, 109, 111, 110, 116, 133, 1, 71, 105, 118, 101, 110, 32, 97, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 110, 117, 109, 98, 101, 114, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 114, 97, 100, 105, 120, 45, 50, 94, 51, 50, 10, 102, 111, 114, 109, 32, 105, 46, 101, 46, 32, 101, 105, 103, 104, 116, 32, 51, 50, 32, 45, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 10, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 111, 102, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 114, 97, 100, 105, 120, 45, 50, 94, 51, 50, 32, 110, 117, 109, 98, 101, 114, 46, 10, 45, 32, 117, 50, 53, 54, 32, 114, 97, 100, 105, 120, 45, 50, 94, 51, 50, 32, 102, 111, 114, 109, 32, 105, 110, 112, 117, 116, 32, 101, 120, 112, 101, 99, 116, 101, 100, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 97, 115, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 97, 53, 44, 32, 97, 54, 44, 32, 97, 55, 93, 10, 45, 32, 117, 50, 53, 54, 32, 109, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 32, 111, 117, 116, 112, 117, 116, 32, 111, 110, 32, 115, 116, 97, 99, 107, 10, 91, 97, 48, 96, 44, 32, 97, 49, 96, 44, 32, 97, 50, 96, 44, 32, 97, 51, 96, 44, 32, 97, 52, 96, 44, 32, 97, 53, 96, 44, 32, 97, 54, 96, 44, 32, 97, 55, 96, 93, 10, 83, 101, 101, 32, 115, 101, 99, 116, 105, 111, 110, 32, 50, 46, 50, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 101, 112, 114, 105, 110, 116, 46, 105, 97, 99, 114, 46, 111, 114, 103, 47, 50, 48, 49, 55, 47, 49, 48, 53, 55, 46, 112, 100, 102, 1, 0, 0, 3, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 162, 7, 0, 0, 0, 0, 0, 0, 161, 144, 14, 0, 0, 0, 0, 0, 211, 5, 0, 9, 102, 114, 111, 109, 95, 109, 111, 110, 116, 127, 1, 71, 105, 118, 101, 110, 32, 97, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 110, 117, 109, 98, 101, 114, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 10, 102, 111, 114, 109, 32, 105, 46, 101, 46, 32, 101, 105, 103, 104, 116, 32, 51, 50, 32, 45, 98, 105, 116, 32, 108, 105, 109, 98, 115, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 114, 97, 100, 105, 120, 45, 50, 94, 51, 50, 10, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 111, 102, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 117, 50, 53, 54, 32, 110, 117, 109, 98, 101, 114, 46, 10, 45, 32, 117, 50, 53, 54, 32, 109, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 32, 105, 110, 112, 117, 116, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 101, 120, 112, 101, 99, 116, 101, 100, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 97, 53, 44, 32, 97, 54, 44, 32, 97, 55, 93, 10, 45, 32, 117, 50, 53, 54, 32, 114, 97, 100, 105, 120, 45, 50, 94, 51, 50, 32, 102, 111, 114, 109, 32, 111, 117, 116, 112, 117, 116, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 97, 115, 10, 91, 97, 48, 96, 44, 32, 97, 49, 96, 44, 32, 97, 50, 96, 44, 32, 97, 51, 96, 44, 32, 97, 52, 96, 44, 32, 97, 53, 96, 44, 32, 97, 54, 96, 44, 32, 97, 55, 96, 93, 10, 83, 101, 101, 32, 115, 101, 99, 116, 105, 111, 110, 32, 50, 46, 50, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 101, 112, 114, 105, 110, 116, 46, 105, 97, 99, 114, 46, 111, 114, 103, 47, 50, 48, 49, 55, 47, 49, 48, 53, 55, 46, 112, 100, 102, 1, 0, 0, 3, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 14, 112, 111, 105, 110, 116, 95, 100, 111, 117, 98, 108, 105, 110, 103, 162, 6, 71, 105, 118, 101, 110, 32, 97, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 32, 105, 110, 32, 112, 114, 111, 106, 101, 99, 116, 105, 118, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 115, 121, 115, 116, 101, 109, 32, 40, 32, 105, 46, 101, 46, 32, 119, 105, 116, 104, 32, 120, 44, 32, 121, 44, 32, 122, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 10, 97, 115, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 114, 105, 109, 101, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 115, 44, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 32, 41, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 97, 100, 100, 115, 10, 116, 104, 97, 116, 32, 112, 111, 105, 110, 116, 32, 119, 105, 116, 104, 32, 115, 101, 108, 102, 32, 105, 46, 101, 46, 32, 100, 111, 101, 115, 32, 112, 111, 105, 110, 116, 32, 100, 111, 117, 98, 108, 105, 110, 103, 32, 111, 110, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 44, 32, 117, 115, 105, 110, 103, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 45, 102, 114, 101, 101, 10, 100, 111, 117, 98, 108, 105, 110, 103, 32, 102, 111, 114, 109, 117, 108, 97, 32, 102, 114, 111, 109, 32, 97, 108, 103, 111, 114, 105, 116, 104, 109, 32, 57, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 101, 112, 114, 105, 110, 116, 46, 105, 97, 99, 114, 46, 111, 114, 103, 47, 50, 48, 49, 53, 47, 49, 48, 54, 48, 46, 112, 100, 102, 44, 32, 119, 104, 105, 108, 101, 10, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 105, 116, 122, 109, 101, 97, 110, 106, 97, 110, 47, 115, 101, 99, 112, 50, 53, 54, 107, 49, 47, 98, 108, 111, 98, 47, 101, 99, 51, 54, 53, 50, 97, 47, 112, 111, 105, 110, 116, 46, 112, 121, 35, 76, 49, 51, 49, 45, 76, 49, 54, 53, 10, 73, 110, 112, 117, 116, 58, 10, 49, 50, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 115, 117, 99, 104, 32, 116, 104, 97, 116, 32, 102, 105, 114, 115, 116, 32, 54, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 102, 111, 114, 32, 105, 110, 112, 117, 116, 32, 112, 111, 105, 110, 116, 32, 38, 10, 108, 97, 115, 116, 32, 54, 32, 97, 114, 101, 32, 102, 111, 114, 32, 115, 116, 111, 114, 105, 110, 103, 32, 114, 101, 115, 117, 108, 116, 105, 110, 103, 32, 112, 111, 105, 110, 116, 46, 10, 70, 105, 114, 115, 116, 32, 54, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 104, 111, 108, 100, 32, 105, 110, 112, 117, 116, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 39, 115, 32, 120, 44, 32, 121, 44, 32, 122, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 44, 32, 119, 104, 101, 114, 101, 32, 101, 97, 99, 104, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 10, 105, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 97, 115, 32, 101, 105, 103, 104, 116, 32, 51, 50, 32, 45, 98, 105, 116, 32, 108, 105, 109, 98, 115, 46, 10, 83, 105, 109, 105, 108, 97, 114, 108, 121, 44, 32, 108, 97, 115, 116, 32, 54, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 104, 111, 108, 100, 32, 114, 101, 115, 117, 108, 116, 105, 110, 103, 32, 40, 100, 111, 117, 98, 108, 101, 100, 41, 32, 112, 111, 105, 110, 116, 39, 115, 32, 120, 44, 32, 121, 44, 32, 122, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 44, 32, 119, 104, 101, 114, 101, 32, 101, 97, 99, 104, 10, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 105, 115, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 97, 115, 32, 101, 105, 103, 104, 116, 32, 51, 50, 32, 45, 98, 105, 116, 32, 108, 105, 109, 98, 115, 46, 32, 78, 111, 116, 101, 44, 32, 116, 104, 105, 115, 32, 105, 115, 32, 119, 104, 101, 114, 101, 10, 111, 117, 116, 112, 117, 116, 32, 119, 105, 108, 108, 32, 98, 101, 32, 119, 114, 105, 116, 116, 101, 110, 44, 32, 115, 111, 32, 99, 97, 108, 108, 101, 100, 32, 105, 115, 32, 101, 120, 112, 101, 99, 116, 101, 100, 32, 116, 111, 32, 114, 101, 97, 100, 32, 100, 111, 117, 98, 108, 101, 100, 32, 112, 111, 105, 110, 116, 32, 102, 114, 111, 109, 32, 108, 97, 115, 116, 32, 54, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 46, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 100, 117, 114, 105, 110, 103, 32, 105, 110, 118, 111, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 58, 10, 91, 120, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 120, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 121, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 121, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 122, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 122, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 10, 120, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 120, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 121, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 121, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 122, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 122, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 93, 10, 78, 111, 116, 101, 44, 32, 40, 88, 44, 32, 89, 44, 32, 90, 41, 32, 32, 32, 32, 61, 62, 32, 105, 110, 112, 117, 116, 32, 112, 111, 105, 110, 116, 10, 40, 88, 51, 44, 32, 89, 51, 44, 32, 90, 51, 41, 32, 61, 62, 32, 111, 117, 116, 112, 117, 116, 32, 112, 111, 105, 110, 116, 10, 79, 117, 116, 112, 117, 116, 58, 10, 76, 97, 115, 116, 32, 54, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 111, 102, 32, 49, 50, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 119, 104, 105, 99, 104, 32, 119, 101, 114, 101, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 100, 117, 114, 105, 110, 103, 32, 105, 110, 118, 111, 99, 97, 116, 105, 111, 110, 44, 32, 119, 104, 101, 114, 101, 32, 114, 101, 115, 117, 108, 116, 105, 110, 103, 32, 100, 111, 117, 98, 108, 101, 100, 10, 112, 111, 105, 110, 116, 32, 105, 115, 32, 107, 101, 112, 116, 32, 105, 110, 32, 115, 105, 109, 105, 108, 97, 114, 32, 102, 111, 114, 109, 46, 32, 70, 111, 114, 32, 115, 101, 101, 105, 110, 103, 32, 88, 51, 44, 32, 89, 51, 44, 32, 90, 51, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 111, 102, 32, 100, 111, 117, 98, 108, 101, 100, 32, 112, 111, 105, 110, 116, 44, 32, 111, 110, 101, 32, 110, 101, 101, 100, 115, 32, 116, 111, 32, 114, 101, 97, 100, 32, 102, 114, 111, 109, 10, 116, 104, 111, 115, 101, 32, 54, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 46, 10, 83, 116, 97, 99, 107, 32, 97, 116, 32, 101, 110, 100, 32, 111, 102, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 111, 102, 32, 114, 111, 117, 116, 105, 110, 101, 32, 108, 111, 111, 107, 115, 32, 108, 105, 107, 101, 10, 91, 120, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 120, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 121, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 121, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 122, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 122, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 93, 1, 12, 0, 205, 0, 113, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 116, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 127, 127, 211, 5, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 145, 200, 1, 0, 0, 0, 0, 0, 0, 0, 145, 127, 127, 211, 6, 0, 127, 127, 211, 6, 0, 127, 127, 211, 6, 0, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 200, 3, 0, 0, 0, 0, 0, 0, 0, 108, 115, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 118, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 121, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 124, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 211, 5, 0, 200, 4, 0, 0, 0, 0, 0, 0, 0, 108, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 115, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 118, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 127, 127, 211, 5, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 37, 80, 0, 0, 0, 0, 0, 0, 211, 5, 0, 200, 6, 0, 0, 0, 0, 0, 0, 0, 145, 200, 7, 0, 0, 0, 0, 0, 0, 0, 145, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 200, 8, 0, 0, 0, 0, 0, 0, 0, 108, 200, 9, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 6, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 200, 10, 0, 0, 0, 0, 0, 0, 0, 108, 200, 11, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 200, 3, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 6, 0, 0, 0, 0, 0, 0, 0, 127, 127, 211, 6, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 6, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 145, 200, 1, 0, 0, 0, 0, 0, 0, 0, 145, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 11, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 10, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 9, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 8, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 200, 10, 0, 0, 0, 0, 0, 0, 0, 108, 200, 11, 0, 0, 0, 0, 0, 0, 0, 108, 113, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 116, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 119, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 122, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 211, 5, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 127, 127, 211, 6, 0, 200, 8, 0, 0, 0, 0, 0, 0, 0, 108, 200, 9, 0, 0, 0, 0, 0, 0, 0, 108, 108, 107, 107, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 8, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 111, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 9, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 112, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 10, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 113, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 11, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 114, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 115, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 14, 112, 111, 105, 110, 116, 95, 97, 100, 100, 105, 116, 105, 111, 110, 16, 6, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 115, 32, 105, 110, 32, 112, 114, 111, 106, 101, 99, 116, 105, 118, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 115, 121, 115, 116, 101, 109, 32, 40, 32, 105, 46, 101, 46, 32, 119, 105, 116, 104, 32, 120, 44, 32, 121, 44, 32, 122, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 10, 97, 115, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 114, 105, 109, 101, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 115, 44, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 101, 97, 99, 104, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 117, 115, 105, 110, 103, 32, 101, 105, 103, 104, 116, 32, 51, 50, 32, 45, 98, 105, 116, 32, 108, 105, 109, 98, 115, 32, 41, 44, 10, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 97, 100, 100, 115, 32, 116, 104, 111, 115, 101, 32, 116, 119, 111, 32, 112, 111, 105, 110, 116, 115, 32, 111, 110, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 44, 32, 117, 115, 105, 110, 103, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 45, 102, 114, 101, 101, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 102, 111, 114, 109, 117, 108, 97, 32, 102, 114, 111, 109, 10, 97, 108, 103, 111, 114, 105, 116, 104, 109, 32, 55, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 101, 112, 114, 105, 110, 116, 46, 105, 97, 99, 114, 46, 111, 114, 103, 47, 50, 48, 49, 53, 47, 49, 48, 54, 48, 46, 112, 100, 102, 44, 32, 119, 104, 105, 108, 101, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 10, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 105, 116, 122, 109, 101, 97, 110, 106, 97, 110, 47, 115, 101, 99, 112, 50, 53, 54, 107, 49, 47, 98, 108, 111, 98, 47, 101, 99, 51, 54, 53, 50, 97, 47, 112, 111, 105, 110, 116, 46, 112, 121, 35, 76, 54, 48, 45, 76, 49, 49, 53, 10, 73, 110, 112, 117, 116, 58, 10, 49, 56, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 32, 115, 117, 99, 104, 32, 116, 104, 97, 116, 32, 102, 105, 114, 115, 116, 32, 54, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 102, 111, 114, 32, 102, 105, 114, 115, 116, 32, 105, 110, 112, 117, 116, 32, 112, 111, 105, 110, 116, 44, 32, 110, 101, 120, 116, 32, 54, 10, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 104, 111, 108, 100, 105, 110, 103, 32, 120, 44, 32, 121, 44, 32, 122, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 111, 102, 32, 115, 101, 99, 111, 110, 100, 32, 105, 110, 112, 117, 116, 32, 112, 111, 105, 110, 116, 32, 38, 32, 108, 97, 115, 116, 32, 54, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 97, 114, 101, 32, 102, 111, 114, 32, 115, 116, 111, 114, 105, 110, 103, 10, 114, 101, 115, 117, 108, 116, 105, 110, 103, 32, 112, 111, 105, 110, 116, 32, 40, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 111, 102, 32, 116, 119, 111, 32, 105, 110, 112, 117, 116, 32, 112, 111, 105, 110, 116, 115, 32, 41, 46, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 100, 117, 114, 105, 110, 103, 32, 105, 110, 118, 111, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 58, 10, 91, 120, 49, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 120, 49, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 121, 49, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 121, 49, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 122, 49, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 122, 49, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 10, 120, 50, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 120, 50, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 121, 50, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 121, 50, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 122, 50, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 122, 50, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 10, 120, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 120, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 121, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 121, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 122, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 122, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 93, 10, 78, 111, 116, 101, 44, 32, 40, 88, 49, 44, 32, 89, 49, 44, 32, 90, 49, 41, 32, 32, 32, 32, 61, 62, 32, 105, 110, 112, 117, 116, 32, 112, 111, 105, 110, 116, 32, 49, 10, 40, 88, 50, 44, 32, 89, 50, 44, 32, 90, 50, 41, 32, 32, 32, 32, 61, 62, 32, 105, 110, 112, 117, 116, 32, 112, 111, 105, 110, 116, 32, 50, 10, 40, 88, 51, 44, 32, 89, 51, 44, 32, 90, 51, 41, 32, 32, 32, 32, 61, 62, 32, 111, 117, 116, 112, 117, 116, 32, 112, 111, 105, 110, 116, 10, 79, 117, 116, 112, 117, 116, 58, 10, 76, 97, 115, 116, 32, 54, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 111, 102, 32, 49, 56, 32, 105, 110, 112, 117, 116, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 119, 104, 105, 99, 104, 32, 119, 101, 114, 101, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 100, 117, 114, 105, 110, 103, 32, 105, 110, 118, 111, 99, 97, 116, 105, 111, 110, 44, 32, 119, 104, 101, 114, 101, 32, 114, 101, 115, 117, 108, 116, 105, 110, 103, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 10, 112, 111, 105, 110, 116, 32, 105, 115, 32, 107, 101, 112, 116, 32, 105, 110, 32, 115, 105, 109, 105, 108, 97, 114, 32, 102, 111, 114, 109, 46, 32, 70, 111, 114, 32, 115, 101, 101, 105, 110, 103, 32, 88, 51, 44, 32, 89, 51, 44, 32, 90, 51, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 111, 102, 32, 100, 111, 117, 98, 108, 101, 100, 32, 112, 111, 105, 110, 116, 44, 32, 111, 110, 101, 32, 110, 101, 101, 100, 115, 32, 116, 111, 32, 114, 101, 97, 100, 32, 102, 114, 111, 109, 10, 116, 104, 111, 115, 101, 32, 54, 32, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 46, 10, 83, 116, 97, 99, 107, 32, 97, 116, 32, 101, 110, 100, 32, 111, 102, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 111, 102, 32, 114, 111, 117, 116, 105, 110, 101, 32, 108, 111, 111, 107, 115, 32, 108, 105, 107, 101, 10, 91, 120, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 120, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 121, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 121, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 44, 32, 122, 51, 95, 97, 100, 100, 114, 91, 48, 46, 46, 52, 93, 44, 32, 122, 51, 95, 97, 100, 100, 114, 91, 52, 46, 46, 56, 93, 93, 1, 16, 0, 211, 1, 116, 118, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 118, 120, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 211, 5, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 118, 120, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 120, 122, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 211, 5, 0, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 200, 3, 0, 0, 0, 0, 0, 0, 0, 108, 120, 122, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 122, 124, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 211, 5, 0, 200, 4, 0, 0, 0, 0, 0, 0, 0, 108, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 112, 114, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 118, 120, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 211, 6, 0, 200, 6, 0, 0, 0, 0, 0, 0, 0, 108, 200, 7, 0, 0, 0, 0, 0, 0, 0, 108, 118, 120, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 125, 125, 130, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 211, 6, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 6, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 200, 6, 0, 0, 0, 0, 0, 0, 0, 108, 200, 7, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 6, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 200, 6, 0, 0, 0, 0, 0, 0, 0, 108, 200, 7, 0, 0, 0, 0, 0, 0, 0, 108, 112, 114, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 122, 124, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 211, 6, 0, 200, 8, 0, 0, 0, 0, 0, 0, 0, 108, 200, 9, 0, 0, 0, 0, 0, 0, 0, 108, 121, 121, 120, 122, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 155, 156, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 211, 6, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 9, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 8, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 200, 8, 0, 0, 0, 0, 0, 0, 0, 108, 200, 9, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 9, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 8, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 200, 8, 0, 0, 0, 0, 0, 0, 0, 108, 200, 9, 0, 0, 0, 0, 0, 0, 0, 108, 114, 116, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 118, 120, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 211, 6, 0, 200, 10, 0, 0, 0, 0, 0, 0, 0, 108, 200, 11, 0, 0, 0, 0, 0, 0, 0, 108, 120, 122, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 125, 125, 130, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 151, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 211, 6, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 11, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 10, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 200, 10, 0, 0, 0, 0, 0, 0, 0, 108, 200, 11, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 11, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 10, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 200, 12, 0, 0, 0, 0, 0, 0, 0, 108, 200, 13, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 127, 127, 211, 6, 0, 200, 10, 0, 0, 0, 0, 0, 0, 0, 145, 200, 11, 0, 0, 0, 0, 0, 0, 0, 145, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 37, 80, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 200, 4, 0, 0, 0, 0, 0, 0, 0, 145, 200, 5, 0, 0, 0, 0, 0, 0, 0, 145, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 200, 14, 0, 0, 0, 0, 0, 0, 0, 108, 200, 15, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 211, 8, 0, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 200, 3, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 37, 80, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 13, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 12, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 200, 12, 0, 0, 0, 0, 0, 0, 0, 145, 200, 13, 0, 0, 0, 0, 0, 0, 0, 145, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 9, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 8, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 200, 10, 0, 0, 0, 0, 0, 0, 0, 108, 200, 11, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 6, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 11, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 10, 0, 0, 0, 0, 0, 0, 0, 211, 7, 0, 211, 6, 0, 200, 10, 0, 0, 0, 0, 0, 0, 0, 108, 200, 11, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 13, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 12, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 200, 12, 0, 0, 0, 0, 0, 0, 0, 108, 200, 13, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 15, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 14, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 13, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 12, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 200, 12, 0, 0, 0, 0, 0, 0, 0, 108, 200, 13, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 6, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 9, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 8, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 15, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 14, 0, 0, 0, 0, 0, 0, 0, 211, 5, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 211, 6, 0, 200, 14, 0, 0, 0, 0, 0, 0, 0, 108, 200, 15, 0, 0, 0, 0, 0, 0, 0, 108, 108, 108, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 10, 0, 0, 0, 0, 0, 0, 0, 114, 198, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 11, 0, 0, 0, 0, 0, 0, 0, 115, 198, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 12, 0, 0, 0, 0, 0, 0, 0, 116, 198, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 13, 0, 0, 0, 0, 0, 0, 0, 117, 198, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 14, 0, 0, 0, 0, 0, 0, 0, 118, 198, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 15, 0, 0, 0, 0, 0, 0, 0, 119, 198, 108, 9, 112, 111, 105, 110, 116, 95, 109, 117, 108, 100, 9, 71, 105, 118, 101, 110, 32, 97, 110, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 32, 105, 110, 32, 112, 114, 111, 106, 101, 99, 116, 105, 118, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 115, 121, 115, 116, 101, 109, 32, 40, 32, 116, 111, 116, 97, 108, 32, 50, 52, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 115, 10, 114, 101, 113, 117, 105, 114, 101, 100, 32, 102, 111, 114, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 105, 110, 103, 32, 120, 44, 32, 121, 44, 32, 122, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 118, 97, 108, 117, 101, 115, 32, 115, 46, 116, 46, 32, 116, 104, 101, 121, 32, 97, 114, 101, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 98, 121, 32, 54, 32, 100, 105, 115, 116, 105, 110, 99, 116, 10, 109, 101, 109, 111, 114, 121, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 41, 32, 97, 110, 100, 32, 97, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 115, 99, 97, 108, 97, 114, 44, 32, 105, 110, 32, 114, 97, 100, 105, 120, 45, 50, 94, 51, 50, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 40, 32, 115, 117, 99, 104, 32, 116, 104, 97, 116, 32, 105, 116, 10, 116, 97, 107, 101, 115, 32, 56, 32, 115, 116, 97, 99, 107, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 116, 111, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 32, 119, 104, 111, 108, 101, 32, 115, 99, 97, 108, 97, 114, 44, 32, 119, 104, 101, 114, 101, 32, 101, 97, 99, 104, 32, 108, 105, 109, 98, 32, 105, 115, 32, 111, 102, 32, 51, 50, 32, 45, 98, 105, 116, 32, 119, 105, 100, 116, 104, 32, 41, 44, 10, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 109, 117, 108, 116, 105, 112, 108, 105, 101, 115, 32, 101, 108, 108, 105, 112, 116, 105, 99, 32, 99, 117, 114, 118, 101, 32, 112, 111, 105, 110, 116, 32, 98, 121, 32, 103, 105, 118, 101, 110, 32, 115, 99, 97, 108, 97, 114, 44, 32, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 97, 110, 111, 116, 104, 101, 114, 32, 112, 111, 105, 110, 116, 10, 111, 110, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 99, 117, 114, 118, 101, 44, 32, 119, 104, 105, 99, 104, 32, 119, 105, 108, 108, 32, 97, 108, 115, 111, 32, 98, 101, 32, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 112, 114, 111, 106, 101, 99, 116, 105, 118, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 115, 121, 115, 116, 101, 109, 46, 10, 73, 110, 112, 117, 116, 58, 10, 68, 117, 114, 105, 110, 103, 32, 105, 110, 118, 111, 99, 97, 116, 105, 111, 110, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 101, 120, 112, 101, 99, 116, 115, 32, 115, 116, 97, 99, 107, 32, 105, 110, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 102, 111, 114, 109, 10, 91, 88, 95, 97, 100, 100, 114, 95, 48, 44, 32, 88, 95, 97, 100, 100, 114, 95, 49, 44, 32, 89, 95, 97, 100, 100, 114, 95, 48, 44, 32, 89, 95, 97, 100, 100, 114, 95, 49, 44, 32, 90, 95, 97, 100, 100, 114, 95, 48, 44, 32, 90, 95, 97, 100, 100, 114, 95, 49, 44, 32, 83, 99, 48, 44, 32, 83, 99, 49, 44, 32, 83, 99, 50, 44, 32, 83, 99, 51, 44, 32, 83, 99, 52, 44, 32, 83, 99, 53, 44, 32, 83, 99, 54, 44, 32, 83, 99, 55, 44, 32, 88, 39, 95, 97, 100, 100, 114, 95, 48, 44, 32, 88, 39, 95, 97, 100, 100, 114, 95, 49, 44, 32, 89, 39, 95, 97, 100, 100, 114, 95, 48, 44, 32, 89, 39, 95, 97, 100, 100, 114, 95, 49, 44, 32, 90, 39, 95, 97, 100, 100, 114, 95, 48, 44, 32, 90, 39, 95, 97, 100, 100, 114, 95, 49, 44, 32, 46, 46, 46, 93, 10, 88, 95, 97, 100, 100, 114, 95, 48, 44, 32, 88, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 73, 110, 112, 117, 116, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 88, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 116, 111, 32, 98, 101, 32, 112, 108, 97, 99, 101, 100, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 89, 95, 97, 100, 100, 114, 95, 48, 44, 32, 89, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 73, 110, 112, 117, 116, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 89, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 116, 111, 32, 98, 101, 32, 112, 108, 97, 99, 101, 100, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 90, 95, 97, 100, 100, 114, 95, 49, 44, 32, 90, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 73, 110, 112, 117, 116, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 90, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 116, 111, 32, 98, 101, 32, 112, 108, 97, 99, 101, 100, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 83, 99, 123, 48, 46, 46, 56, 125, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 45, 62, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 115, 99, 97, 108, 97, 114, 32, 105, 110, 32, 114, 97, 100, 105, 120, 45, 50, 94, 51, 50, 32, 102, 111, 114, 109, 32, 124, 32, 83, 99, 48, 32, 105, 115, 32, 108, 101, 97, 115, 116, 32, 115, 105, 103, 110, 105, 102, 105, 99, 97, 110, 116, 32, 108, 105, 109, 98, 32, 38, 32, 83, 99, 55, 32, 105, 115, 32, 109, 111, 115, 116, 32, 115, 105, 103, 110, 105, 102, 105, 99, 97, 110, 116, 32, 108, 105, 109, 98, 10, 88, 39, 95, 97, 100, 100, 114, 95, 48, 44, 32, 88, 39, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 88, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 116, 111, 32, 98, 101, 32, 112, 108, 97, 99, 101, 100, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 89, 39, 95, 97, 100, 100, 114, 95, 48, 44, 32, 89, 39, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 89, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 116, 111, 32, 98, 101, 32, 112, 108, 97, 99, 101, 100, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 90, 39, 95, 97, 100, 100, 114, 95, 49, 44, 32, 90, 39, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 90, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 116, 111, 32, 98, 101, 32, 112, 108, 97, 99, 101, 100, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 79, 117, 116, 112, 117, 116, 58, 10, 65, 116, 32, 101, 110, 100, 32, 111, 102, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 44, 32, 115, 116, 97, 99, 107, 32, 115, 104, 111, 117, 108, 100, 32, 108, 111, 111, 107, 32, 108, 105, 107, 101, 32, 98, 101, 108, 111, 119, 10, 91, 88, 95, 97, 100, 100, 114, 95, 48, 44, 32, 88, 95, 97, 100, 100, 114, 95, 49, 44, 32, 89, 95, 97, 100, 100, 114, 95, 48, 44, 32, 89, 95, 97, 100, 100, 114, 95, 49, 44, 32, 90, 95, 97, 100, 100, 114, 95, 48, 44, 32, 90, 95, 97, 100, 100, 114, 95, 49, 44, 32, 46, 46, 46, 93, 10, 88, 95, 97, 100, 100, 114, 95, 48, 44, 32, 88, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 88, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 119, 114, 105, 116, 116, 101, 110, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 89, 95, 97, 100, 100, 114, 95, 48, 44, 32, 89, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 89, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 119, 114, 105, 116, 116, 101, 110, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 90, 95, 97, 100, 100, 114, 95, 48, 44, 32, 90, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 90, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 119, 114, 105, 116, 116, 101, 110, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 79, 110, 101, 32, 105, 110, 116, 101, 114, 101, 115, 116, 101, 100, 32, 105, 110, 32, 114, 101, 115, 117, 108, 116, 105, 110, 103, 32, 112, 111, 105, 110, 116, 44, 32, 115, 104, 111, 117, 108, 100, 32, 114, 101, 97, 100, 32, 102, 114, 111, 109, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 46, 10, 84, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 115, 32, 100, 111, 117, 98, 108, 101, 45, 97, 110, 100, 45, 97, 100, 100, 32, 97, 108, 103, 111, 114, 105, 116, 104, 109, 44, 32, 119, 104, 105, 108, 101, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 10, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 105, 116, 122, 109, 101, 97, 110, 106, 97, 110, 47, 115, 101, 99, 112, 50, 53, 54, 107, 49, 47, 98, 108, 111, 98, 47, 100, 50, 51, 101, 97, 55, 100, 47, 112, 111, 105, 110, 116, 46, 112, 121, 35, 76, 49, 55, 52, 45, 76, 49, 56, 54, 10, 73, 102, 32, 98, 97, 115, 101, 32, 112, 111, 105, 110, 116, 32, 98, 101, 105, 110, 103, 32, 109, 117, 108, 116, 105, 112, 108, 105, 101, 100, 32, 105, 115, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 99, 117, 114, 118, 101, 32, 103, 101, 110, 101, 114, 97, 116, 111, 114, 32, 112, 111, 105, 110, 116, 44, 32, 111, 110, 101, 32, 115, 104, 111, 117, 108, 100, 32, 117, 115, 101, 32, 96, 103, 101, 110, 95, 112, 111, 105, 110, 116, 96, 32, 114, 111, 117, 116, 105, 110, 101, 44, 10, 119, 104, 105, 99, 104, 32, 105, 115, 32, 97, 108, 109, 111, 115, 116, 32, 50, 120, 32, 102, 97, 115, 116, 101, 114, 32, 33, 1, 18, 0, 53, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 1, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 2, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 3, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 4, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 6, 0, 0, 0, 0, 0, 0, 0, 200, 7, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 209, 3, 0, 0, 0, 0, 0, 0, 200, 8, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 9, 0, 0, 0, 0, 0, 0, 0, 200, 10, 0, 0, 0, 0, 0, 0, 0, 200, 11, 0, 0, 0, 0, 0, 0, 0, 108, 254, 254, 3, 2, 0, 254, 255, 3, 38, 0, 110, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 71, 253, 39, 0, 186, 17, 0, 0, 0, 0, 0, 0, 0, 186, 16, 0, 0, 0, 0, 0, 0, 0, 186, 15, 0, 0, 0, 0, 0, 0, 0, 186, 14, 0, 0, 0, 0, 0, 0, 0, 186, 13, 0, 0, 0, 0, 0, 0, 0, 186, 12, 0, 0, 0, 0, 0, 0, 0, 186, 11, 0, 0, 0, 0, 0, 0, 0, 186, 10, 0, 0, 0, 0, 0, 0, 0, 186, 9, 0, 0, 0, 0, 0, 0, 0, 186, 8, 0, 0, 0, 0, 0, 0, 0, 186, 7, 0, 0, 0, 0, 0, 0, 0, 186, 6, 0, 0, 0, 0, 0, 0, 0, 186, 5, 0, 0, 0, 0, 0, 0, 0, 186, 4, 0, 0, 0, 0, 0, 0, 0, 186, 3, 0, 0, 0, 0, 0, 0, 0, 186, 2, 0, 0, 0, 0, 0, 0, 0, 186, 1, 0, 0, 0, 0, 0, 0, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 211, 12, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 6, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 7, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 8, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 9, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 10, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 11, 0, 0, 0, 0, 0, 0, 0, 108, 0, 0, 186, 17, 0, 0, 0, 0, 0, 0, 0, 186, 16, 0, 0, 0, 0, 0, 0, 0, 186, 15, 0, 0, 0, 0, 0, 0, 0, 186, 14, 0, 0, 0, 0, 0, 0, 0, 186, 13, 0, 0, 0, 0, 0, 0, 0, 186, 12, 0, 0, 0, 0, 0, 0, 0, 186, 5, 0, 0, 0, 0, 0, 0, 0, 186, 4, 0, 0, 0, 0, 0, 0, 0, 186, 3, 0, 0, 0, 0, 0, 0, 0, 186, 2, 0, 0, 0, 0, 0, 0, 0, 186, 1, 0, 0, 0, 0, 0, 0, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 211, 11, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 0, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 1, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 2, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 3, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 4, 0, 0, 0, 0, 0, 0, 0, 151, 191, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 78, 1, 107, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 6, 0, 0, 0, 0, 0, 0, 0, 114, 198, 194, 7, 0, 0, 0, 0, 0, 0, 0, 115, 198, 194, 8, 0, 0, 0, 0, 0, 0, 0, 116, 198, 194, 9, 0, 0, 0, 0, 0, 0, 0, 117, 198, 194, 10, 0, 0, 0, 0, 0, 0, 0, 118, 198, 194, 11, 0, 0, 0, 0, 0, 0, 0, 119, 198, 108, 7, 103, 101, 110, 95, 109, 117, 108, 156, 7, 71, 105, 118, 101, 110, 32, 97, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 115, 99, 97, 108, 97, 114, 44, 32, 105, 110, 32, 114, 97, 100, 105, 120, 45, 50, 94, 51, 50, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 32, 40, 32, 115, 117, 99, 104, 32, 116, 104, 97, 116, 32, 105, 116, 32, 116, 97, 107, 101, 115, 32, 56, 32, 115, 116, 97, 99, 107, 32, 101, 108, 101, 109, 101, 110, 116, 115, 10, 116, 111, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 32, 119, 104, 111, 108, 101, 32, 115, 99, 97, 108, 97, 114, 44, 32, 119, 104, 101, 114, 101, 32, 101, 97, 99, 104, 32, 108, 105, 109, 98, 32, 105, 115, 32, 111, 102, 32, 51, 50, 32, 45, 98, 105, 116, 32, 119, 105, 100, 116, 104, 32, 41, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 109, 117, 108, 116, 105, 112, 108, 105, 101, 115, 10, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 103, 101, 110, 101, 114, 97, 116, 111, 114, 32, 112, 111, 105, 110, 116, 32, 40, 32, 105, 110, 32, 112, 114, 111, 106, 101, 99, 116, 105, 118, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 115, 121, 115, 116, 101, 109, 32, 41, 32, 119, 105, 116, 104, 32, 103, 105, 118, 101, 110, 32, 115, 99, 97, 108, 97, 114, 44, 32, 112, 114, 111, 100, 117, 99, 105, 110, 103, 10, 97, 110, 111, 116, 104, 101, 114, 32, 112, 111, 105, 110, 116, 32, 111, 110, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 99, 117, 114, 118, 101, 44, 32, 119, 104, 105, 99, 104, 32, 119, 105, 108, 108, 32, 97, 108, 115, 111, 32, 98, 101, 32, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 105, 110, 32, 112, 114, 111, 106, 101, 99, 116, 105, 118, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 10, 115, 121, 115, 116, 101, 109, 46, 10, 73, 110, 112, 117, 116, 58, 10, 68, 117, 114, 105, 110, 103, 32, 105, 110, 118, 111, 99, 97, 116, 105, 111, 110, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 101, 120, 112, 101, 99, 116, 115, 32, 115, 116, 97, 99, 107, 32, 105, 110, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 32, 102, 111, 114, 109, 10, 91, 83, 99, 48, 44, 32, 83, 99, 49, 44, 32, 83, 99, 50, 44, 32, 83, 99, 51, 44, 32, 83, 99, 52, 44, 32, 83, 99, 53, 44, 32, 83, 99, 54, 44, 32, 83, 99, 55, 44, 32, 88, 95, 97, 100, 100, 114, 95, 48, 44, 32, 88, 95, 97, 100, 100, 114, 95, 49, 44, 32, 89, 95, 97, 100, 100, 114, 95, 48, 44, 32, 89, 95, 97, 100, 100, 114, 95, 49, 44, 32, 90, 95, 97, 100, 100, 114, 95, 48, 44, 32, 90, 95, 97, 100, 100, 114, 95, 49, 44, 32, 46, 46, 46, 93, 10, 83, 99, 123, 48, 46, 46, 56, 125, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 45, 62, 32, 50, 53, 54, 32, 45, 98, 105, 116, 32, 115, 99, 97, 108, 97, 114, 32, 105, 110, 32, 114, 97, 100, 105, 120, 45, 50, 94, 51, 50, 32, 102, 111, 114, 109, 32, 124, 32, 83, 99, 48, 32, 105, 115, 32, 108, 101, 97, 115, 116, 32, 115, 105, 103, 110, 105, 102, 105, 99, 97, 110, 116, 32, 108, 105, 109, 98, 32, 38, 32, 83, 99, 55, 32, 105, 115, 32, 109, 111, 115, 116, 32, 115, 105, 103, 110, 105, 102, 105, 99, 97, 110, 116, 32, 108, 105, 109, 98, 10, 88, 95, 97, 100, 100, 114, 95, 48, 44, 32, 88, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 88, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 116, 111, 32, 98, 101, 32, 112, 108, 97, 99, 101, 100, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 89, 95, 97, 100, 100, 114, 95, 48, 44, 32, 89, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 89, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 116, 111, 32, 98, 101, 32, 112, 108, 97, 99, 101, 100, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 90, 95, 97, 100, 100, 114, 95, 49, 44, 32, 90, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 90, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 116, 111, 32, 98, 101, 32, 112, 108, 97, 99, 101, 100, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 79, 117, 116, 112, 117, 116, 58, 10, 65, 116, 32, 101, 110, 100, 32, 111, 102, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 44, 32, 115, 116, 97, 99, 107, 32, 115, 104, 111, 117, 108, 100, 32, 108, 111, 111, 107, 32, 108, 105, 107, 101, 32, 98, 101, 108, 111, 119, 10, 91, 88, 95, 97, 100, 100, 114, 95, 48, 44, 32, 88, 95, 97, 100, 100, 114, 95, 49, 44, 32, 89, 95, 97, 100, 100, 114, 95, 48, 44, 32, 89, 95, 97, 100, 100, 114, 95, 49, 44, 32, 90, 95, 97, 100, 100, 114, 95, 48, 44, 32, 90, 95, 97, 100, 100, 114, 95, 49, 44, 32, 46, 46, 46, 93, 10, 88, 95, 97, 100, 100, 114, 95, 48, 44, 32, 88, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 88, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 119, 114, 105, 116, 116, 101, 110, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 89, 95, 97, 100, 100, 114, 95, 48, 44, 32, 89, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 89, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 119, 114, 105, 116, 116, 101, 110, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 90, 95, 97, 100, 100, 114, 95, 48, 44, 32, 90, 95, 97, 100, 100, 114, 95, 49, 32, 45, 62, 32, 82, 101, 115, 117, 108, 116, 105, 110, 103, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 39, 115, 32, 90, 32, 45, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 119, 114, 105, 116, 116, 101, 110, 44, 32, 105, 110, 32, 77, 111, 110, 116, 103, 111, 109, 101, 114, 121, 32, 102, 111, 114, 109, 44, 32, 105, 110, 32, 103, 105, 118, 101, 110, 32, 97, 100, 100, 114, 101, 115, 115, 101, 115, 10, 79, 110, 101, 32, 105, 110, 116, 101, 114, 101, 115, 116, 101, 100, 32, 105, 110, 32, 114, 101, 115, 117, 108, 116, 105, 110, 103, 32, 112, 111, 105, 110, 116, 44, 32, 115, 104, 111, 117, 108, 100, 32, 114, 101, 97, 100, 32, 102, 114, 111, 109, 32, 112, 114, 111, 118, 105, 100, 101, 100, 32, 97, 100, 100, 114, 101, 115, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 46, 10, 84, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 115, 32, 100, 111, 117, 98, 108, 101, 45, 97, 110, 100, 45, 97, 100, 100, 32, 97, 108, 103, 111, 114, 105, 116, 104, 109, 44, 32, 119, 104, 105, 108, 101, 32, 102, 111, 108, 108, 111, 119, 105, 110, 103, 10, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 105, 116, 122, 109, 101, 97, 110, 106, 97, 110, 47, 115, 101, 99, 112, 50, 53, 54, 107, 49, 47, 98, 108, 111, 98, 47, 100, 50, 51, 101, 97, 55, 100, 47, 112, 111, 105, 110, 116, 46, 112, 121, 35, 76, 49, 55, 52, 45, 76, 49, 56, 54, 10, 78, 111, 116, 101, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 105, 115, 32, 97, 32, 115, 112, 101, 99, 105, 97, 108, 105, 115, 101, 100, 32, 105, 110, 115, 116, 97, 110, 116, 105, 97, 116, 105, 111, 110, 32, 111, 102, 32, 115, 101, 99, 112, 50, 53, 54, 107, 49, 32, 112, 111, 105, 110, 116, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 44, 32, 119, 104, 101, 114, 101, 32, 119, 101, 32, 107, 110, 111, 119, 32, 119, 104, 97, 116, 32, 116, 104, 101, 32, 98, 97, 115, 101, 10, 112, 111, 105, 110, 116, 32, 105, 115, 44, 32, 115, 111, 32, 119, 101, 32, 101, 110, 106, 111, 121, 32, 102, 97, 115, 116, 101, 114, 32, 99, 111, 109, 112, 117, 116, 97, 116, 105, 111, 110, 32, 40, 32, 98, 101, 99, 97, 117, 115, 101, 32, 97, 108, 108, 32, 112, 111, 105, 110, 116, 32, 100, 111, 117, 98, 108, 105, 110, 103, 115, 32, 99, 97, 110, 32, 98, 101, 32, 112, 114, 101, 99, 111, 109, 112, 117, 116, 101, 100, 44, 32, 115, 97, 118, 105, 110, 103, 32, 117, 115, 32, 50, 53, 54, 32, 112, 111, 105, 110, 116, 32, 100, 111, 117, 98, 108, 105, 110, 103, 115, 32, 33, 32, 41, 46, 1, 20, 0, 59, 6, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 1, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 209, 3, 0, 0, 0, 0, 0, 0, 200, 2, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 3, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 4, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 200, 18, 0, 0, 0, 0, 0, 0, 0, 108, 200, 19, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 155, 130, 82, 105, 0, 0, 0, 0, 219, 165, 58, 210, 0, 0, 0, 0, 193, 171, 19, 222, 0, 0, 0, 0, 152, 81, 57, 27, 0, 0, 0, 0, 185, 4, 56, 156, 118, 142, 0, 0, 0, 0, 106, 240, 105, 76, 0, 0, 0, 0, 133, 44, 147, 130, 0, 0, 0, 0, 110, 37, 47, 205, 0, 0, 0, 0, 185, 4, 99, 28, 59, 226, 0, 0, 0, 0, 2, 126, 156, 204, 0, 0, 0, 0, 247, 181, 130, 206, 0, 0, 0, 0, 172, 46, 129, 105, 0, 0, 0, 0, 185, 4, 1, 101, 183, 6, 0, 0, 0, 0, 6, 34, 194, 91, 0, 0, 0, 0, 127, 32, 85, 1, 0, 0, 0, 0, 203, 142, 178, 49, 0, 0, 0, 0, 185, 4, 229, 243, 150, 88, 0, 0, 0, 0, 202, 66, 27, 179, 0, 0, 0, 0, 93, 87, 245, 65, 0, 0, 0, 0, 255, 146, 96, 46, 0, 0, 0, 0, 185, 4, 77, 68, 35, 150, 0, 0, 0, 0, 139, 109, 104, 86, 0, 0, 0, 0, 222, 233, 158, 252, 0, 0, 0, 0, 143, 224, 131, 93, 0, 0, 0, 0, 185, 4, 128, 240, 253, 197, 0, 0, 0, 0, 106, 8, 178, 203, 0, 0, 0, 0, 44, 228, 177, 5, 0, 0, 0, 0, 127, 67, 107, 33, 0, 0, 0, 0, 185, 4, 159, 202, 158, 56, 0, 0, 0, 0, 163, 185, 200, 110, 0, 0, 0, 0, 151, 173, 10, 194, 0, 0, 0, 0, 201, 184, 236, 37, 0, 0, 0, 0, 185, 4, 44, 77, 203, 164, 0, 0, 0, 0, 65, 79, 112, 134, 0, 0, 0, 0, 30, 67, 215, 31, 0, 0, 0, 0, 189, 89, 154, 75, 0, 0, 0, 0, 185, 4, 8, 201, 71, 169, 0, 0, 0, 0, 226, 252, 112, 200, 0, 0, 0, 0, 250, 237, 185, 32, 0, 0, 0, 0, 130, 175, 65, 101, 0, 0, 0, 0, 185, 4, 184, 26, 166, 233, 0, 0, 0, 0, 118, 40, 94, 23, 0, 0, 0, 0, 159, 53, 120, 72, 0, 0, 0, 0, 104, 231, 0, 113, 0, 0, 0, 0, 185, 4, 178, 69, 2, 130, 0, 0, 0, 0, 112, 215, 50, 112, 0, 0, 0, 0, 218, 176, 33, 116, 0, 0, 0, 0, 6, 106, 188, 137, 0, 0, 0, 0, 185, 4, 199, 158, 32, 253, 0, 0, 0, 0, 22, 160, 171, 211, 0, 0, 0, 0, 165, 115, 153, 228, 0, 0, 0, 0, 17, 237, 217, 12, 0, 0, 0, 0, 185, 4, 94, 109, 93, 89, 0, 0, 0, 0, 175, 84, 183, 236, 0, 0, 0, 0, 119, 204, 47, 132, 0, 0, 0, 0, 122, 123, 79, 195, 0, 0, 0, 0, 185, 4, 74, 53, 6, 38, 0, 0, 0, 0, 40, 251, 20, 254, 0, 0, 0, 0, 49, 107, 111, 171, 0, 0, 0, 0, 142, 204, 201, 228, 0, 0, 0, 0, 185, 4, 249, 40, 26, 156, 0, 0, 0, 0, 70, 195, 84, 195, 0, 0, 0, 0, 17, 155, 225, 151, 0, 0, 0, 0, 114, 119, 67, 137, 0, 0, 0, 0, 185, 4, 33, 109, 126, 88, 0, 0, 0, 0, 187, 201, 188, 67, 0, 0, 0, 0, 24, 117, 195, 244, 0, 0, 0, 0, 32, 240, 188, 155, 0, 0, 0, 0, 185, 4, 119, 199, 28, 81, 0, 0, 0, 0, 129, 160, 99, 222, 0, 0, 0, 0, 240, 173, 178, 36, 0, 0, 0, 0, 207, 60, 195, 116, 0, 0, 0, 0, 185, 4, 84, 117, 21, 244, 0, 0, 0, 0, 148, 114, 159, 127, 0, 0, 0, 0, 231, 15, 155, 118, 0, 0, 0, 0, 123, 212, 123, 65, 0, 0, 0, 0, 185, 4, 7, 59, 106, 168, 0, 0, 0, 0, 149, 41, 214, 77, 0, 0, 0, 0, 122, 193, 50, 50, 0, 0, 0, 0, 171, 82, 186, 55, 0, 0, 0, 0, 185, 4, 69, 105, 138, 212, 0, 0, 0, 0, 255, 48, 125, 154, 0, 0, 0, 0, 67, 114, 104, 147, 0, 0, 0, 0, 211, 41, 43, 241, 0, 0, 0, 0, 185, 4, 108, 150, 198, 78, 0, 0, 0, 0, 146, 224, 24, 79, 0, 0, 0, 0, 20, 2, 249, 172, 0, 0, 0, 0, 142, 8, 150, 241, 0, 0, 0, 0, 185, 4, 202, 90, 156, 224, 0, 0, 0, 0, 252, 87, 127, 19, 0, 0, 0, 0, 19, 27, 20, 147, 0, 0, 0, 0, 211, 178, 221, 91, 0, 0, 0, 0, 185, 4, 74, 62, 155, 188, 0, 0, 0, 0, 111, 210, 155, 149, 0, 0, 0, 0, 94, 116, 148, 80, 0, 0, 0, 0, 27, 247, 100, 16, 0, 0, 0, 0, 185, 4, 146, 118, 46, 203, 0, 0, 0, 0, 35, 58, 212, 0, 0, 0, 0, 0, 23, 144, 188, 168, 0, 0, 0, 0, 152, 49, 190, 114, 0, 0, 0, 0, 185, 4, 55, 195, 211, 32, 0, 0, 0, 0, 139, 113, 143, 71, 0, 0, 0, 0, 221, 227, 71, 185, 0, 0, 0, 0, 199, 6, 214, 101, 0, 0, 0, 0, 185, 4, 91, 101, 208, 0, 0, 0, 0, 0, 98, 106, 209, 27, 0, 0, 0, 0, 24, 133, 242, 138, 0, 0, 0, 0, 178, 108, 209, 183, 0, 0, 0, 0, 185, 4, 27, 107, 41, 22, 0, 0, 0, 0, 133, 50, 211, 250, 0, 0, 0, 0, 2, 214, 55, 68, 0, 0, 0, 0, 194, 152, 28, 126, 0, 0, 0, 0, 185, 4, 85, 228, 56, 206, 0, 0, 0, 0, 83, 203, 5, 212, 0, 0, 0, 0, 34, 80, 8, 200, 0, 0, 0, 0, 199, 26, 168, 12, 0, 0, 0, 0, 185, 4, 86, 126, 205, 177, 0, 0, 0, 0, 20, 213, 187, 156, 0, 0, 0, 0, 29, 98, 95, 116, 0, 0, 0, 0, 167, 253, 87, 248, 0, 0, 0, 0, 185, 4, 132, 192, 195, 138, 0, 0, 0, 0, 161, 79, 235, 24, 0, 0, 0, 0, 243, 51, 148, 77, 0, 0, 0, 0, 65, 169, 22, 85, 0, 0, 0, 0, 185, 4, 230, 249, 230, 24, 0, 0, 0, 0, 120, 81, 33, 138, 0, 0, 0, 0, 138, 25, 245, 235, 0, 0, 0, 0, 10, 47, 51, 66, 0, 0, 0, 0, 185, 4, 227, 152, 157, 173, 0, 0, 0, 0, 69, 107, 243, 103, 0, 0, 0, 0, 213, 41, 99, 205, 0, 0, 0, 0, 234, 253, 22, 64, 0, 0, 0, 0, 185, 4, 143, 214, 101, 50, 0, 0, 0, 0, 228, 243, 223, 255, 0, 0, 0, 0, 171, 30, 126, 18, 0, 0, 0, 0, 154, 14, 23, 180, 0, 0, 0, 0, 185, 4, 70, 243, 209, 51, 0, 0, 0, 0, 180, 238, 142, 5, 0, 0, 0, 0, 225, 42, 35, 207, 0, 0, 0, 0, 39, 241, 176, 81, 0, 0, 0, 0, 185, 4, 7, 238, 165, 234, 0, 0, 0, 0, 120, 246, 58, 165, 0, 0, 0, 0, 66, 42, 86, 71, 0, 0, 0, 0, 30, 125, 110, 88, 0, 0, 0, 0, 185, 4, 29, 203, 5, 206, 0, 0, 0, 0, 229, 57, 47, 220, 0, 0, 0, 0, 129, 76, 197, 138, 0, 0, 0, 0, 208, 201, 18, 174, 0, 0, 0, 0, 185, 4, 125, 162, 21, 150, 0, 0, 0, 0, 81, 147, 72, 180, 0, 0, 0, 0, 131, 73, 219, 93, 0, 0, 0, 0, 139, 58, 118, 91, 0, 0, 0, 0, 185, 4, 112, 231, 243, 145, 0, 0, 0, 0, 92, 17, 109, 203, 0, 0, 0, 0, 31, 120, 89, 198, 0, 0, 0, 0, 238, 166, 120, 141, 0, 0, 0, 0, 185, 4, 117, 158, 0, 217, 0, 0, 0, 0, 86, 162, 29, 139, 0, 0, 0, 0, 45, 75, 67, 96, 0, 0, 0, 0, 192, 233, 66, 149, 0, 0, 0, 0, 185, 4, 195, 214, 243, 221, 0, 0, 0, 0, 200, 10, 46, 104, 0, 0, 0, 0, 153, 127, 81, 197, 0, 0, 0, 0, 27, 30, 29, 17, 0, 0, 0, 0, 185, 4, 112, 255, 74, 171, 0, 0, 0, 0, 109, 46, 97, 33, 0, 0, 0, 0, 184, 233, 225, 82, 0, 0, 0, 0, 151, 136, 81, 38, 0, 0, 0, 0, 185, 4, 135, 166, 234, 123, 0, 0, 0, 0, 134, 204, 191, 215, 0, 0, 0, 0, 112, 33, 108, 232, 0, 0, 0, 0, 251, 220, 208, 135, 0, 0, 0, 0, 185, 4, 109, 40, 168, 130, 0, 0, 0, 0, 152, 68, 92, 221, 0, 0, 0, 0, 41, 96, 188, 111, 0, 0, 0, 0, 248, 105, 53, 15, 0, 0, 0, 0, 185, 4, 239, 138, 36, 24, 0, 0, 0, 0, 179, 79, 200, 205, 0, 0, 0, 0, 185, 185, 92, 254, 0, 0, 0, 0, 135, 145, 131, 250, 0, 0, 0, 0, 185, 4, 186, 80, 37, 15, 0, 0, 0, 0, 97, 199, 3, 237, 0, 0, 0, 0, 137, 233, 24, 63, 0, 0, 0, 0, 194, 204, 132, 184, 0, 0, 0, 0, 185, 4, 146, 88, 14, 78, 0, 0, 0, 0, 222, 101, 69, 130, 0, 0, 0, 0, 87, 220, 255, 106, 0, 0, 0, 0, 24, 76, 33, 224, 0, 0, 0, 0, 185, 4, 234, 123, 238, 8, 0, 0, 0, 0, 184, 249, 154, 2, 0, 0, 0, 0, 110, 138, 46, 34, 0, 0, 0, 0, 1, 160, 229, 23, 0, 0, 0, 0, 185, 4, 252, 154, 206, 179, 0, 0, 0, 0, 26, 117, 144, 71, 0, 0, 0, 0, 107, 160, 12, 157, 0, 0, 0, 0, 87, 149, 189, 163, 0, 0, 0, 0, 185, 4, 41, 216, 243, 181, 0, 0, 0, 0, 93, 187, 92, 103, 0, 0, 0, 0, 229, 93, 15, 112, 0, 0, 0, 0, 56, 12, 148, 0, 0, 0, 0, 0, 185, 4, 144, 29, 61, 164, 0, 0, 0, 0, 100, 16, 166, 190, 0, 0, 0, 0, 191, 148, 118, 243, 0, 0, 0, 0, 228, 125, 45, 66, 0, 0, 0, 0, 185, 4, 186, 70, 123, 91, 0, 0, 0, 0, 253, 152, 24, 156, 0, 0, 0, 0, 93, 171, 177, 216, 0, 0, 0, 0, 71, 224, 225, 235, 0, 0, 0, 0, 185, 4, 80, 2, 182, 199, 0, 0, 0, 0, 116, 106, 28, 207, 0, 0, 0, 0, 143, 141, 139, 36, 0, 0, 0, 0, 209, 191, 149, 138, 0, 0, 0, 0, 185, 4, 211, 74, 64, 71, 0, 0, 0, 0, 24, 216, 126, 50, 0, 0, 0, 0, 159, 40, 31, 62, 0, 0, 0, 0, 55, 172, 64, 140, 0, 0, 0, 0, 185, 4, 180, 67, 219, 10, 0, 0, 0, 0, 218, 8, 61, 235, 0, 0, 0, 0, 127, 212, 151, 139, 0, 0, 0, 0, 245, 243, 165, 227, 0, 0, 0, 0, 185, 4, 8, 126, 127, 108, 0, 0, 0, 0, 239, 24, 72, 223, 0, 0, 0, 0, 201, 168, 222, 245, 0, 0, 0, 0, 146, 49, 204, 163, 0, 0, 0, 0, 185, 4, 79, 83, 25, 100, 0, 0, 0, 0, 144, 9, 111, 150, 0, 0, 0, 0, 250, 38, 34, 183, 0, 0, 0, 0, 101, 91, 14, 210, 0, 0, 0, 0, 185, 4, 112, 107, 210, 38, 0, 0, 0, 0, 68, 185, 26, 249, 0, 0, 0, 0, 14, 39, 166, 204, 0, 0, 0, 0, 156, 105, 195, 146, 0, 0, 0, 0, 185, 4, 102, 173, 164, 255, 0, 0, 0, 0, 186, 180, 146, 72, 0, 0, 0, 0, 61, 227, 85, 145, 0, 0, 0, 0, 219, 69, 87, 66, 0, 0, 0, 0, 185, 4, 30, 183, 78, 43, 0, 0, 0, 0, 113, 47, 152, 254, 0, 0, 0, 0, 10, 6, 160, 39, 0, 0, 0, 0, 74, 99, 200, 160, 0, 0, 0, 0, 185, 4, 244, 221, 28, 65, 0, 0, 0, 0, 224, 89, 172, 114, 0, 0, 0, 0, 27, 111, 118, 255, 0, 0, 0, 0, 58, 202, 202, 162, 0, 0, 0, 0, 185, 4, 43, 23, 89, 72, 0, 0, 0, 0, 234, 201, 25, 227, 0, 0, 0, 0, 8, 102, 233, 208, 0, 0, 0, 0, 156, 86, 65, 179, 0, 0, 0, 0, 185, 4, 55, 240, 255, 237, 0, 0, 0, 0, 40, 143, 13, 129, 0, 0, 0, 0, 36, 42, 102, 149, 0, 0, 0, 0, 169, 133, 24, 130, 0, 0, 0, 0, 185, 4, 55, 141, 217, 99, 0, 0, 0, 0, 90, 155, 117, 147, 0, 0, 0, 0, 65, 202, 144, 96, 0, 0, 0, 0, 183, 49, 44, 109, 0, 0, 0, 0, 185, 4, 115, 35, 218, 143, 0, 0, 0, 0, 198, 96, 225, 243, 0, 0, 0, 0, 105, 31, 130, 252, 0, 0, 0, 0, 221, 155, 62, 244, 0, 0, 0, 0, 185, 4, 71, 7, 101, 35, 0, 0, 0, 0, 71, 157, 38, 214, 0, 0, 0, 0, 220, 132, 180, 12, 0, 0, 0, 0, 124, 1, 111, 49, 0, 0, 0, 0, 185, 4, 99, 7, 78, 22, 0, 0, 0, 0, 173, 248, 27, 80, 0, 0, 0, 0, 255, 240, 174, 226, 0, 0, 0, 0, 0, 35, 234, 118, 0, 0, 0, 0, 185, 4, 94, 152, 201, 65, 0, 0, 0, 0, 94, 37, 111, 109, 0, 0, 0, 0, 219, 202, 76, 213, 0, 0, 0, 0, 92, 254, 123, 246, 0, 0, 0, 0, 185, 4, 32, 15, 197, 82, 0, 0, 0, 0, 148, 18, 239, 26, 0, 0, 0, 0, 212, 135, 136, 199, 0, 0, 0, 0, 85, 244, 199, 137, 0, 0, 0, 0, 185, 4, 37, 249, 32, 98, 0, 0, 0, 0, 8, 153, 244, 68, 0, 0, 0, 0, 172, 222, 142, 68, 0, 0, 0, 0, 207, 66, 112, 190, 0, 0, 0, 0, 185, 4, 240, 65, 254, 57, 0, 0, 0, 0, 169, 57, 24, 80, 0, 0, 0, 0, 176, 200, 171, 109, 0, 0, 0, 0, 5, 64, 82, 153, 0, 0, 0, 0, 185, 4, 18, 96, 180, 49, 0, 0, 0, 0, 145, 71, 174, 173, 0, 0, 0, 0, 249, 78, 205, 183, 0, 0, 0, 0, 147, 17, 0, 179, 0, 0, 0, 0, 185, 4, 5, 213, 208, 83, 0, 0, 0, 0, 173, 253, 134, 47, 0, 0, 0, 0, 197, 137, 50, 250, 0, 0, 0, 0, 59, 186, 91, 4, 0, 0, 0, 0, 185, 4, 9, 12, 178, 53, 0, 0, 0, 0, 114, 59, 195, 196, 0, 0, 0, 0, 52, 44, 62, 6, 0, 0, 0, 0, 97, 192, 3, 105, 0, 0, 0, 0, 185, 4, 65, 3, 244, 141, 0, 0, 0, 0, 186, 12, 35, 246, 0, 0, 0, 0, 27, 155, 121, 204, 0, 0, 0, 0, 152, 224, 11, 120, 0, 0, 0, 0, 185, 4, 202, 158, 17, 48, 0, 0, 0, 0, 138, 185, 10, 216, 0, 0, 0, 0, 229, 232, 139, 173, 0, 0, 0, 0, 116, 160, 73, 190, 0, 0, 0, 0, 185, 4, 242, 30, 51, 6, 0, 0, 0, 0, 221, 151, 208, 104, 0, 0, 0, 0, 46, 121, 225, 203, 0, 0, 0, 0, 168, 58, 117, 239, 0, 0, 0, 0, 185, 4, 5, 175, 200, 117, 0, 0, 0, 0, 81, 73, 194, 10, 0, 0, 0, 0, 90, 47, 56, 91, 0, 0, 0, 0, 157, 93, 249, 86, 0, 0, 0, 0, 185, 4, 129, 245, 249, 113, 0, 0, 0, 0, 76, 41, 58, 87, 0, 0, 0, 0, 81, 49, 102, 215, 0, 0, 0, 0, 12, 178, 156, 1, 0, 0, 0, 0, 185, 4, 77, 27, 224, 150, 0, 0, 0, 0, 76, 161, 75, 15, 0, 0, 0, 0, 233, 98, 212, 18, 0, 0, 0, 0, 158, 230, 20, 9, 0, 0, 0, 0, 185, 4, 179, 112, 83, 111, 0, 0, 0, 0, 225, 61, 117, 2, 0, 0, 0, 0, 205, 245, 126, 121, 0, 0, 0, 0, 212, 175, 114, 145, 0, 0, 0, 0, 185, 4, 9, 219, 64, 125, 0, 0, 0, 0, 200, 66, 250, 15, 0, 0, 0, 0, 249, 144, 6, 216, 0, 0, 0, 0, 4, 145, 149, 206, 0, 0, 0, 0, 185, 4, 186, 33, 125, 74, 0, 0, 0, 0, 214, 141, 115, 44, 0, 0, 0, 0, 132, 115, 34, 9, 0, 0, 0, 0, 93, 185, 20, 168, 0, 0, 0, 0, 185, 4, 177, 42, 109, 187, 0, 0, 0, 0, 134, 183, 249, 141, 0, 0, 0, 0, 201, 67, 183, 156, 0, 0, 0, 0, 92, 5, 83, 170, 0, 0, 0, 0, 185, 4, 8, 233, 56, 35, 0, 0, 0, 0, 126, 129, 255, 204, 0, 0, 0, 0, 187, 14, 157, 174, 0, 0, 0, 0, 38, 245, 125, 151, 0, 0, 0, 0, 185, 4, 191, 94, 53, 81, 0, 0, 0, 0, 175, 89, 74, 126, 0, 0, 0, 0, 21, 14, 36, 191, 0, 0, 0, 0, 107, 104, 241, 156, 0, 0, 0, 0, 185, 4, 173, 218, 145, 199, 0, 0, 0, 0, 188, 44, 209, 182, 0, 0, 0, 0, 238, 112, 120, 197, 0, 0, 0, 0, 78, 160, 74, 154, 0, 0, 0, 0, 185, 4, 115, 113, 115, 224, 0, 0, 0, 0, 206, 90, 106, 146, 0, 0, 0, 0, 27, 112, 129, 50, 0, 0, 0, 0, 71, 76, 241, 145, 0, 0, 0, 0, 185, 4, 78, 25, 193, 48, 0, 0, 0, 0, 96, 76, 236, 13, 0, 0, 0, 0, 60, 175, 222, 235, 0, 0, 0, 0, 187, 39, 215, 12, 0, 0, 0, 0, 185, 4, 241, 91, 53, 242, 0, 0, 0, 0, 70, 181, 192, 27, 0, 0, 0, 0, 115, 111, 95, 207, 0, 0, 0, 0, 39, 53, 201, 201, 0, 0, 0, 0, 185, 4, 127, 23, 246, 90, 0, 0, 0, 0, 15, 36, 238, 158, 0, 0, 0, 0, 2, 255, 60, 85, 0, 0, 0, 0, 239, 1, 188, 17, 0, 0, 0, 0, 185, 4, 250, 41, 88, 220, 0, 0, 0, 0, 101, 73, 251, 131, 0, 0, 0, 0, 227, 246, 34, 31, 0, 0, 0, 0, 78, 202, 210, 76, 0, 0, 0, 0, 185, 4, 14, 69, 133, 218, 0, 0, 0, 0, 20, 250, 16, 230, 0, 0, 0, 0, 69, 11, 238, 105, 0, 0, 0, 0, 181, 69, 89, 25, 0, 0, 0, 0, 185, 4, 162, 34, 122, 59, 0, 0, 0, 0, 211, 26, 236, 131, 0, 0, 0, 0, 82, 26, 54, 225, 0, 0, 0, 0, 224, 189, 97, 83, 0, 0, 0, 0, 185, 4, 70, 138, 175, 206, 0, 0, 0, 0, 158, 106, 228, 138, 0, 0, 0, 0, 23, 202, 57, 214, 0, 0, 0, 0, 238, 246, 88, 43, 0, 0, 0, 0, 185, 4, 234, 109, 252, 61, 0, 0, 0, 0, 110, 147, 119, 253, 0, 0, 0, 0, 1, 143, 128, 56, 0, 0, 0, 0, 124, 74, 175, 3, 0, 0, 0, 0, 185, 4, 142, 125, 18, 157, 0, 0, 0, 0, 107, 79, 210, 14, 0, 0, 0, 0, 207, 174, 104, 73, 0, 0, 0, 0, 230, 231, 178, 190, 0, 0, 0, 0, 185, 4, 223, 182, 253, 119, 0, 0, 0, 0, 74, 123, 51, 56, 0, 0, 0, 0, 163, 178, 19, 31, 0, 0, 0, 0, 74, 68, 157, 227, 0, 0, 0, 0, 185, 4, 245, 39, 12, 22, 0, 0, 0, 0, 4, 148, 215, 216, 0, 0, 0, 0, 89, 184, 143, 189, 0, 0, 0, 0, 198, 3, 206, 50, 0, 0, 0, 0, 185, 4, 193, 99, 203, 170, 0, 0, 0, 0, 108, 167, 200, 220, 0, 0, 0, 0, 89, 175, 207, 198, 0, 0, 0, 0, 16, 106, 240, 124, 0, 0, 0, 0, 185, 4, 18, 235, 43, 201, 0, 0, 0, 0, 60, 244, 133, 38, 0, 0, 0, 0, 121, 118, 244, 106, 0, 0, 0, 0, 244, 151, 3, 145, 0, 0, 0, 0, 185, 4, 16, 66, 132, 148, 0, 0, 0, 0, 94, 154, 129, 109, 0, 0, 0, 0, 151, 201, 69, 109, 0, 0, 0, 0, 161, 63, 156, 105, 0, 0, 0, 0, 185, 4, 39, 28, 240, 209, 0, 0, 0, 0, 0, 85, 153, 120, 0, 0, 0, 0, 167, 1, 34, 20, 0, 0, 0, 0, 224, 205, 19, 202, 0, 0, 0, 0, 185, 4, 33, 138, 71, 117, 0, 0, 0, 0, 204, 186, 166, 63, 0, 0, 0, 0, 165, 99, 19, 47, 0, 0, 0, 0, 114, 6, 161, 113, 0, 0, 0, 0, 185, 4, 31, 47, 29, 235, 0, 0, 0, 0, 147, 1, 28, 167, 0, 0, 0, 0, 157, 5, 173, 59, 0, 0, 0, 0, 134, 251, 181, 155, 0, 0, 0, 0, 185, 4, 62, 58, 245, 195, 0, 0, 0, 0, 4, 70, 138, 186, 0, 0, 0, 0, 134, 165, 89, 31, 0, 0, 0, 0, 102, 10, 54, 160, 0, 0, 0, 0, 185, 4, 87, 238, 144, 219, 0, 0, 0, 0, 85, 30, 21, 219, 0, 0, 0, 0, 19, 217, 108, 96, 0, 0, 0, 0, 249, 17, 13, 202, 0, 0, 0, 0, 185, 4, 0, 16, 213, 170, 0, 0, 0, 0, 101, 158, 192, 108, 0, 0, 0, 0, 210, 251, 56, 236, 0, 0, 0, 0, 114, 44, 8, 224, 0, 0, 0, 0, 185, 4, 11, 27, 165, 132, 0, 0, 0, 0, 9, 136, 136, 174, 0, 0, 0, 0, 237, 93, 101, 77, 0, 0, 0, 0, 142, 128, 105, 45, 0, 0, 0, 0, 185, 4, 205, 188, 119, 234, 0, 0, 0, 0, 139, 123, 63, 236, 0, 0, 0, 0, 139, 20, 91, 52, 0, 0, 0, 0, 175, 6, 15, 225, 0, 0, 0, 0, 185, 4, 80, 107, 234, 61, 0, 0, 0, 0, 52, 220, 121, 114, 0, 0, 0, 0, 2, 223, 236, 74, 0, 0, 0, 0, 67, 209, 172, 59, 0, 0, 0, 0, 185, 4, 140, 234, 197, 62, 0, 0, 0, 0, 64, 56, 87, 99, 0, 0, 0, 0, 103, 73, 10, 72, 0, 0, 0, 0, 24, 10, 202, 192, 0, 0, 0, 0, 185, 4, 200, 198, 77, 230, 0, 0, 0, 0, 233, 34, 42, 145, 0, 0, 0, 0, 154, 228, 91, 141, 0, 0, 0, 0, 129, 4, 150, 194, 0, 0, 0, 0, 185, 4, 230, 100, 97, 41, 0, 0, 0, 0, 109, 91, 170, 131, 0, 0, 0, 0, 217, 193, 18, 147, 0, 0, 0, 0, 199, 186, 232, 170, 0, 0, 0, 0, 185, 4, 198, 110, 86, 232, 0, 0, 0, 0, 83, 234, 24, 147, 0, 0, 0, 0, 147, 207, 33, 61, 0, 0, 0, 0, 116, 223, 209, 186, 0, 0, 0, 0, 185, 4, 102, 102, 229, 43, 0, 0, 0, 0, 44, 34, 226, 142, 0, 0, 0, 0, 121, 147, 15, 53, 0, 0, 0, 0, 191, 228, 87, 177, 0, 0, 0, 0, 185, 4, 102, 171, 54, 227, 0, 0, 0, 0, 99, 209, 85, 143, 0, 0, 0, 0, 169, 136, 57, 29, 0, 0, 0, 0, 130, 237, 173, 191, 0, 0, 0, 0, 185, 4, 233, 187, 234, 96, 0, 0, 0, 0, 179, 19, 215, 49, 0, 0, 0, 0, 112, 254, 148, 225, 0, 0, 0, 0, 6, 127, 92, 8, 0, 0, 0, 0, 185, 4, 134, 11, 144, 31, 0, 0, 0, 0, 40, 121, 55, 38, 0, 0, 0, 0, 229, 62, 66, 234, 0, 0, 0, 0, 226, 115, 124, 177, 0, 0, 0, 0, 185, 4, 40, 196, 121, 214, 0, 0, 0, 0, 5, 228, 13, 100, 0, 0, 0, 0, 56, 10, 214, 29, 0, 0, 0, 0, 14, 176, 182, 97, 0, 0, 0, 0, 185, 4, 175, 152, 0, 182, 0, 0, 0, 0, 66, 211, 201, 192, 0, 0, 0, 0, 44, 27, 10, 222, 0, 0, 0, 0, 91, 182, 188, 183, 0, 0, 0, 0, 185, 4, 131, 58, 93, 152, 0, 0, 0, 0, 165, 131, 124, 5, 0, 0, 0, 0, 56, 245, 198, 217, 0, 0, 0, 0, 197, 28, 95, 131, 0, 0, 0, 0, 185, 4, 254, 56, 73, 146, 0, 0, 0, 0, 20, 251, 123, 164, 0, 0, 0, 0, 90, 241, 56, 113, 0, 0, 0, 0, 17, 103, 119, 166, 0, 0, 0, 0, 185, 4, 165, 236, 127, 102, 0, 0, 0, 0, 50, 75, 204, 26, 0, 0, 0, 0, 81, 254, 10, 108, 0, 0, 0, 0, 227, 25, 56, 72, 0, 0, 0, 0, 185, 4, 194, 101, 200, 239, 0, 0, 0, 0, 56, 15, 217, 202, 0, 0, 0, 0, 202, 162, 135, 9, 0, 0, 0, 0, 24, 189, 92, 83, 0, 0, 0, 0, 185, 4, 201, 180, 114, 146, 0, 0, 0, 0, 46, 66, 14, 208, 0, 0, 0, 0, 12, 201, 114, 145, 0, 0, 0, 0, 2, 231, 16, 11, 0, 0, 0, 0, 185, 4, 216, 10, 185, 224, 0, 0, 0, 0, 115, 56, 172, 145, 0, 0, 0, 0, 198, 109, 237, 225, 0, 0, 0, 0, 223, 53, 243, 88, 0, 0, 0, 0, 185, 4, 241, 194, 208, 197, 0, 0, 0, 0, 193, 197, 158, 245, 0, 0, 0, 0, 36, 167, 149, 104, 0, 0, 0, 0, 35, 57, 219, 16, 0, 0, 0, 0, 185, 4, 234, 25, 162, 134, 0, 0, 0, 0, 35, 112, 25, 224, 0, 0, 0, 0, 179, 215, 142, 217, 0, 0, 0, 0, 118, 62, 97, 44, 0, 0, 0, 0, 185, 4, 165, 81, 106, 27, 0, 0, 0, 0, 65, 18, 84, 132, 0, 0, 0, 0, 253, 134, 160, 76, 0, 0, 0, 0, 215, 243, 224, 250, 0, 0, 0, 0, 185, 4, 248, 29, 91, 236, 0, 0, 0, 0, 177, 117, 114, 24, 0, 0, 0, 0, 170, 77, 230, 210, 0, 0, 0, 0, 5, 1, 91, 102, 0, 0, 0, 0, 185, 4, 133, 81, 0, 47, 0, 0, 0, 0, 223, 212, 104, 82, 0, 0, 0, 0, 175, 159, 63, 152, 0, 0, 0, 0, 66, 130, 34, 213, 0, 0, 0, 0, 185, 4, 154, 74, 97, 114, 0, 0, 0, 0, 166, 243, 82, 235, 0, 0, 0, 0, 209, 227, 228, 148, 0, 0, 0, 0, 248, 38, 182, 228, 0, 0, 0, 0, 185, 4, 96, 16, 10, 34, 0, 0, 0, 0, 174, 29, 204, 158, 0, 0, 0, 0, 22, 206, 182, 71, 0, 0, 0, 0, 84, 175, 242, 91, 0, 0, 0, 0, 185, 4, 37, 68, 183, 49, 0, 0, 0, 0, 156, 52, 135, 31, 0, 0, 0, 0, 166, 88, 210, 87, 0, 0, 0, 0, 124, 170, 173, 89, 0, 0, 0, 0, 185, 4, 247, 198, 132, 9, 0, 0, 0, 0, 55, 247, 145, 75, 0, 0, 0, 0, 22, 127, 15, 69, 0, 0, 0, 0, 83, 7, 1, 101, 0, 0, 0, 0, 185, 4, 154, 216, 234, 62, 0, 0, 0, 0, 26, 142, 236, 4, 0, 0, 0, 0, 121, 107, 162, 217, 0, 0, 0, 0, 59, 119, 252, 77, 0, 0, 0, 0, 185, 4, 19, 244, 224, 115, 0, 0, 0, 0, 120, 43, 82, 74, 0, 0, 0, 0, 94, 234, 145, 202, 0, 0, 0, 0, 25, 32, 183, 239, 0, 0, 0, 0, 185, 4, 133, 122, 166, 79, 0, 0, 0, 0, 162, 41, 241, 84, 0, 0, 0, 0, 218, 140, 218, 19, 0, 0, 0, 0, 235, 166, 202, 189, 0, 0, 0, 0, 185, 4, 60, 113, 191, 0, 0, 0, 0, 0, 126, 31, 147, 58, 0, 0, 0, 0, 224, 192, 71, 192, 0, 0, 0, 0, 243, 210, 77, 80, 0, 0, 0, 0, 185, 4, 88, 198, 86, 32, 0, 0, 0, 0, 225, 19, 14, 247, 0, 0, 0, 0, 177, 169, 98, 151, 0, 0, 0, 0, 10, 222, 221, 2, 0, 0, 0, 0, 185, 4, 151, 136, 95, 207, 0, 0, 0, 0, 34, 29, 131, 64, 0, 0, 0, 0, 155, 255, 214, 150, 0, 0, 0, 0, 116, 116, 63, 12, 0, 0, 0, 0, 185, 4, 244, 31, 241, 91, 0, 0, 0, 0, 59, 14, 175, 233, 0, 0, 0, 0, 250, 131, 46, 206, 0, 0, 0, 0, 221, 129, 29, 46, 0, 0, 0, 0, 185, 4, 46, 33, 50, 37, 0, 0, 0, 0, 79, 231, 76, 227, 0, 0, 0, 0, 95, 147, 46, 241, 0, 0, 0, 0, 212, 94, 62, 68, 0, 0, 0, 0, 185, 4, 48, 183, 40, 19, 0, 0, 0, 0, 162, 107, 193, 99, 0, 0, 0, 0, 242, 194, 200, 182, 0, 0, 0, 0, 153, 196, 77, 38, 0, 0, 0, 0, 185, 4, 177, 178, 160, 246, 0, 0, 0, 0, 234, 251, 118, 49, 0, 0, 0, 0, 49, 202, 15, 60, 0, 0, 0, 0, 65, 89, 97, 228, 0, 0, 0, 0, 185, 4, 221, 50, 242, 45, 0, 0, 0, 0, 236, 251, 185, 46, 0, 0, 0, 0, 79, 106, 8, 187, 0, 0, 0, 0, 40, 229, 121, 82, 0, 0, 0, 0, 185, 4, 44, 219, 160, 106, 0, 0, 0, 0, 249, 16, 131, 127, 0, 0, 0, 0, 229, 86, 176, 249, 0, 0, 0, 0, 4, 224, 123, 232, 0, 0, 0, 0, 185, 4, 48, 210, 164, 137, 0, 0, 0, 0, 118, 217, 163, 190, 0, 0, 0, 0, 2, 141, 173, 71, 0, 0, 0, 0, 86, 112, 187, 41, 0, 0, 0, 0, 185, 4, 196, 14, 99, 124, 0, 0, 0, 0, 172, 97, 16, 195, 0, 0, 0, 0, 137, 84, 230, 244, 0, 0, 0, 0, 77, 8, 150, 17, 0, 0, 0, 0, 185, 4, 183, 51, 176, 238, 0, 0, 0, 0, 151, 56, 209, 248, 0, 0, 0, 0, 59, 131, 191, 237, 0, 0, 0, 0, 34, 208, 76, 9, 0, 0, 0, 0, 185, 4, 227, 84, 188, 161, 0, 0, 0, 0, 80, 111, 158, 46, 0, 0, 0, 0, 171, 75, 98, 122, 0, 0, 0, 0, 47, 217, 151, 230, 0, 0, 0, 0, 185, 4, 22, 111, 114, 146, 0, 0, 0, 0, 203, 189, 215, 126, 0, 0, 0, 0, 13, 217, 186, 122, 0, 0, 0, 0, 105, 211, 3, 69, 0, 0, 0, 0, 185, 4, 244, 52, 192, 148, 0, 0, 0, 0, 127, 37, 49, 98, 0, 0, 0, 0, 66, 36, 29, 90, 0, 0, 0, 0, 254, 25, 179, 16, 0, 0, 0, 0, 185, 4, 67, 232, 154, 60, 0, 0, 0, 0, 10, 254, 126, 24, 0, 0, 0, 0, 97, 25, 99, 13, 0, 0, 0, 0, 202, 226, 213, 153, 0, 0, 0, 0, 185, 4, 70, 74, 39, 79, 0, 0, 0, 0, 213, 40, 42, 85, 0, 0, 0, 0, 235, 23, 98, 211, 0, 0, 0, 0, 136, 166, 118, 75, 0, 0, 0, 0, 185, 4, 215, 239, 238, 177, 0, 0, 0, 0, 175, 243, 12, 134, 0, 0, 0, 0, 237, 180, 75, 27, 0, 0, 0, 0, 165, 132, 206, 92, 0, 0, 0, 0, 185, 4, 204, 217, 18, 116, 0, 0, 0, 0, 106, 86, 80, 112, 0, 0, 0, 0, 69, 238, 139, 129, 0, 0, 0, 0, 241, 152, 250, 190, 0, 0, 0, 0, 185, 4, 141, 163, 26, 47, 0, 0, 0, 0, 190, 66, 21, 214, 0, 0, 0, 0, 189, 57, 104, 62, 0, 0, 0, 0, 23, 29, 17, 62, 0, 0, 0, 0, 185, 4, 57, 181, 207, 120, 0, 0, 0, 0, 246, 213, 87, 173, 0, 0, 0, 0, 216, 69, 55, 227, 0, 0, 0, 0, 15, 148, 84, 77, 0, 0, 0, 0, 185, 4, 89, 119, 174, 234, 0, 0, 0, 0, 181, 228, 243, 43, 0, 0, 0, 0, 203, 106, 196, 249, 0, 0, 0, 0, 32, 115, 155, 28, 0, 0, 0, 0, 185, 4, 168, 212, 93, 12, 0, 0, 0, 0, 225, 220, 217, 27, 0, 0, 0, 0, 200, 246, 116, 108, 0, 0, 0, 0, 158, 125, 105, 94, 0, 0, 0, 0, 185, 4, 92, 144, 215, 180, 0, 0, 0, 0, 188, 239, 168, 200, 0, 0, 0, 0, 71, 17, 204, 128, 0, 0, 0, 0, 183, 172, 96, 179, 0, 0, 0, 0, 185, 4, 103, 118, 91, 191, 0, 0, 0, 0, 78, 86, 87, 41, 0, 0, 0, 0, 99, 147, 108, 206, 0, 0, 0, 0, 51, 133, 105, 196, 0, 0, 0, 0, 185, 4, 185, 102, 89, 1, 0, 0, 0, 0, 24, 241, 106, 46, 0, 0, 0, 0, 217, 177, 192, 181, 0, 0, 0, 0, 70, 225, 65, 43, 0, 0, 0, 0, 185, 4, 7, 214, 115, 196, 0, 0, 0, 0, 153, 164, 59, 235, 0, 0, 0, 0, 59, 104, 89, 78, 0, 0, 0, 0, 71, 158, 126, 111, 0, 0, 0, 0, 185, 4, 80, 189, 243, 25, 0, 0, 0, 0, 16, 208, 131, 31, 0, 0, 0, 0, 129, 57, 208, 22, 0, 0, 0, 0, 51, 143, 4, 77, 0, 0, 0, 0, 185, 4, 57, 80, 245, 1, 0, 0, 0, 0, 213, 79, 143, 171, 0, 0, 0, 0, 255, 49, 67, 212, 0, 0, 0, 0, 93, 45, 155, 235, 0, 0, 0, 0, 185, 4, 196, 43, 0, 181, 0, 0, 0, 0, 166, 228, 111, 86, 0, 0, 0, 0, 220, 89, 248, 236, 0, 0, 0, 0, 200, 75, 184, 90, 0, 0, 0, 0, 185, 4, 174, 31, 172, 77, 0, 0, 0, 0, 239, 174, 64, 39, 0, 0, 0, 0, 248, 243, 71, 222, 0, 0, 0, 0, 172, 148, 197, 26, 0, 0, 0, 0, 185, 4, 121, 72, 83, 80, 0, 0, 0, 0, 62, 7, 77, 5, 0, 0, 0, 0, 17, 192, 40, 50, 0, 0, 0, 0, 131, 41, 254, 216, 0, 0, 0, 0, 185, 4, 167, 49, 23, 145, 0, 0, 0, 0, 113, 142, 148, 29, 0, 0, 0, 0, 66, 117, 68, 115, 0, 0, 0, 0, 12, 101, 129, 107, 0, 0, 0, 0, 185, 4, 10, 46, 50, 28, 0, 0, 0, 0, 145, 70, 30, 79, 0, 0, 0, 0, 235, 237, 22, 68, 0, 0, 0, 0, 29, 144, 173, 22, 0, 0, 0, 0, 185, 4, 222, 152, 165, 243, 0, 0, 0, 0, 248, 192, 9, 159, 0, 0, 0, 0, 165, 116, 92, 31, 0, 0, 0, 0, 194, 32, 249, 52, 0, 0, 0, 0, 185, 4, 49, 5, 176, 151, 0, 0, 0, 0, 218, 176, 200, 81, 0, 0, 0, 0, 107, 136, 54, 36, 0, 0, 0, 0, 227, 233, 43, 93, 0, 0, 0, 0, 185, 4, 188, 49, 252, 209, 0, 0, 0, 0, 47, 202, 150, 45, 0, 0, 0, 0, 95, 1, 97, 35, 0, 0, 0, 0, 38, 37, 50, 228, 0, 0, 0, 0, 185, 4, 131, 113, 48, 151, 0, 0, 0, 0, 203, 9, 35, 30, 0, 0, 0, 0, 74, 80, 242, 122, 0, 0, 0, 0, 175, 39, 114, 180, 0, 0, 0, 0, 185, 4, 152, 88, 164, 174, 0, 0, 0, 0, 38, 46, 12, 182, 0, 0, 0, 0, 141, 66, 220, 123, 0, 0, 0, 0, 83, 109, 92, 193, 0, 0, 0, 0, 185, 4, 181, 200, 70, 209, 0, 0, 0, 0, 220, 89, 89, 23, 0, 0, 0, 0, 64, 105, 15, 126, 0, 0, 0, 0, 184, 179, 78, 69, 0, 0, 0, 0, 185, 4, 198, 240, 116, 176, 0, 0, 0, 0, 83, 245, 185, 131, 0, 0, 0, 0, 68, 204, 23, 62, 0, 0, 0, 0, 63, 173, 190, 118, 0, 0, 0, 0, 185, 4, 13, 11, 114, 92, 0, 0, 0, 0, 125, 22, 168, 255, 0, 0, 0, 0, 97, 124, 126, 26, 0, 0, 0, 0, 216, 5, 28, 78, 0, 0, 0, 0, 185, 4, 55, 77, 248, 51, 0, 0, 0, 0, 234, 214, 171, 203, 0, 0, 0, 0, 225, 138, 131, 221, 0, 0, 0, 0, 91, 102, 181, 0, 0, 0, 0, 0, 185, 4, 199, 209, 77, 156, 0, 0, 0, 0, 42, 209, 129, 214, 0, 0, 0, 0, 176, 9, 114, 205, 0, 0, 0, 0, 74, 60, 151, 50, 0, 0, 0, 0, 185, 4, 128, 89, 222, 216, 0, 0, 0, 0, 135, 11, 89, 21, 0, 0, 0, 0, 80, 241, 248, 217, 0, 0, 0, 0, 204, 170, 61, 234, 0, 0, 0, 0, 185, 4, 129, 165, 252, 244, 0, 0, 0, 0, 164, 94, 208, 15, 0, 0, 0, 0, 244, 26, 113, 61, 0, 0, 0, 0, 95, 105, 58, 188, 0, 0, 0, 0, 185, 4, 116, 74, 231, 9, 0, 0, 0, 0, 28, 31, 138, 7, 0, 0, 0, 0, 107, 99, 98, 201, 0, 0, 0, 0, 150, 47, 64, 222, 0, 0, 0, 0, 185, 4, 56, 56, 132, 174, 0, 0, 0, 0, 65, 130, 145, 106, 0, 0, 0, 0, 89, 175, 113, 218, 0, 0, 0, 0, 84, 209, 211, 172, 0, 0, 0, 0, 185, 4, 250, 22, 161, 146, 0, 0, 0, 0, 255, 202, 193, 117, 0, 0, 0, 0, 134, 62, 145, 254, 0, 0, 0, 0, 41, 32, 24, 29, 0, 0, 0, 0, 185, 4, 67, 89, 191, 155, 0, 0, 0, 0, 135, 196, 144, 143, 0, 0, 0, 0, 41, 186, 160, 46, 0, 0, 0, 0, 57, 249, 214, 155, 0, 0, 0, 0, 185, 4, 197, 243, 121, 72, 0, 0, 0, 0, 33, 99, 78, 52, 0, 0, 0, 0, 248, 130, 92, 87, 0, 0, 0, 0, 109, 122, 89, 200, 0, 0, 0, 0, 185, 4, 24, 106, 210, 33, 0, 0, 0, 0, 110, 28, 140, 178, 0, 0, 0, 0, 195, 100, 181, 169, 0, 0, 0, 0, 24, 246, 79, 156, 0, 0, 0, 0, 185, 4, 42, 18, 188, 177, 0, 0, 0, 0, 90, 77, 28, 151, 0, 0, 0, 0, 250, 56, 29, 0, 0, 0, 0, 0, 222, 67, 240, 138, 0, 0, 0, 0, 185, 4, 238, 206, 131, 76, 0, 0, 0, 0, 229, 45, 108, 196, 0, 0, 0, 0, 215, 229, 179, 212, 0, 0, 0, 0, 9, 69, 249, 110, 0, 0, 0, 0, 185, 4, 90, 94, 17, 120, 0, 0, 0, 0, 157, 93, 132, 112, 0, 0, 0, 0, 232, 42, 37, 138, 0, 0, 0, 0, 128, 221, 126, 80, 0, 0, 0, 0, 185, 4, 169, 224, 221, 235, 0, 0, 0, 0, 154, 105, 160, 57, 0, 0, 0, 0, 213, 91, 37, 59, 0, 0, 0, 0, 132, 7, 73, 8, 0, 0, 0, 0, 185, 4, 226, 56, 34, 169, 0, 0, 0, 0, 81, 221, 41, 12, 0, 0, 0, 0, 93, 99, 213, 83, 0, 0, 0, 0, 126, 148, 202, 115, 0, 0, 0, 0, 185, 4, 37, 144, 238, 204, 0, 0, 0, 0, 125, 194, 161, 190, 0, 0, 0, 0, 180, 169, 40, 199, 0, 0, 0, 0, 172, 175, 140, 100, 0, 0, 0, 0, 185, 4, 156, 161, 141, 139, 0, 0, 0, 0, 51, 25, 188, 196, 0, 0, 0, 0, 171, 108, 4, 232, 0, 0, 0, 0, 178, 104, 125, 147, 0, 0, 0, 0, 185, 4, 235, 81, 121, 199, 0, 0, 0, 0, 136, 230, 190, 191, 0, 0, 0, 0, 77, 60, 229, 123, 0, 0, 0, 0, 249, 231, 26, 164, 0, 0, 0, 0, 185, 4, 180, 9, 2, 76, 0, 0, 0, 0, 218, 211, 207, 77, 0, 0, 0, 0, 190, 73, 134, 219, 0, 0, 0, 0, 45, 251, 58, 86, 0, 0, 0, 0, 185, 4, 220, 200, 254, 18, 0, 0, 0, 0, 214, 172, 249, 201, 0, 0, 0, 0, 43, 209, 236, 17, 0, 0, 0, 0, 83, 134, 86, 25, 0, 0, 0, 0, 185, 4, 252, 228, 200, 187, 0, 0, 0, 0, 116, 84, 92, 254, 0, 0, 0, 0, 150, 129, 160, 146, 0, 0, 0, 0, 240, 245, 83, 203, 0, 0, 0, 0, 185, 4, 79, 208, 175, 37, 0, 0, 0, 0, 215, 109, 182, 81, 0, 0, 0, 0, 84, 97, 200, 240, 0, 0, 0, 0, 51, 162, 231, 42, 0, 0, 0, 0, 185, 4, 208, 99, 104, 230, 0, 0, 0, 0, 72, 221, 245, 35, 0, 0, 0, 0, 15, 144, 114, 193, 0, 0, 0, 0, 23, 251, 161, 6, 0, 0, 0, 0, 185, 4, 130, 74, 167, 126, 0, 0, 0, 0, 185, 72, 215, 172, 0, 0, 0, 0, 73, 170, 116, 141, 0, 0, 0, 0, 27, 14, 33, 103, 0, 0, 0, 0, 185, 4, 47, 178, 21, 110, 0, 0, 0, 0, 84, 73, 211, 147, 0, 0, 0, 0, 45, 88, 63, 213, 0, 0, 0, 0, 253, 132, 161, 72, 0, 0, 0, 0, 185, 4, 112, 210, 228, 104, 0, 0, 0, 0, 151, 162, 227, 18, 0, 0, 0, 0, 155, 111, 119, 228, 0, 0, 0, 0, 78, 142, 113, 222, 0, 0, 0, 0, 185, 4, 206, 168, 166, 255, 0, 0, 0, 0, 152, 212, 23, 4, 0, 0, 0, 0, 196, 191, 109, 1, 0, 0, 0, 0, 108, 37, 109, 230, 0, 0, 0, 0, 185, 4, 40, 100, 115, 98, 0, 0, 0, 0, 21, 175, 105, 222, 0, 0, 0, 0, 82, 165, 145, 151, 0, 0, 0, 0, 225, 3, 31, 41, 0, 0, 0, 0, 185, 4, 89, 81, 200, 229, 0, 0, 0, 0, 50, 10, 155, 187, 0, 0, 0, 0, 64, 248, 192, 90, 0, 0, 0, 0, 79, 93, 126, 186, 0, 0, 0, 0, 185, 4, 206, 168, 234, 36, 0, 0, 0, 0, 235, 218, 153, 39, 0, 0, 0, 0, 152, 248, 127, 30, 0, 0, 0, 0, 79, 46, 227, 54, 0, 0, 0, 0, 185, 4, 167, 21, 83, 218, 0, 0, 0, 0, 174, 224, 67, 185, 0, 0, 0, 0, 144, 19, 222, 40, 0, 0, 0, 0, 99, 143, 126, 156, 0, 0, 0, 0, 185, 4, 218, 142, 141, 84, 0, 0, 0, 0, 81, 192, 106, 78, 0, 0, 0, 0, 2, 220, 62, 115, 0, 0, 0, 0, 18, 58, 232, 89, 0, 0, 0, 0, 185, 4, 164, 250, 240, 59, 0, 0, 0, 0, 164, 1, 10, 46, 0, 0, 0, 0, 204, 171, 99, 246, 0, 0, 0, 0, 215, 121, 10, 245, 0, 0, 0, 0, 185, 4, 46, 59, 69, 57, 0, 0, 0, 0, 110, 124, 125, 109, 0, 0, 0, 0, 23, 15, 84, 25, 0, 0, 0, 0, 223, 236, 222, 169, 0, 0, 0, 0, 185, 4, 129, 239, 179, 27, 0, 0, 0, 0, 191, 79, 22, 87, 0, 0, 0, 0, 79, 130, 93, 163, 0, 0, 0, 0, 189, 178, 202, 70, 0, 0, 0, 0, 185, 4, 5, 39, 30, 57, 0, 0, 0, 0, 158, 43, 204, 21, 0, 0, 0, 0, 5, 53, 206, 254, 0, 0, 0, 0, 175, 61, 166, 141, 0, 0, 0, 0, 185, 4, 209, 4, 245, 67, 0, 0, 0, 0, 54, 147, 205, 158, 0, 0, 0, 0, 117, 210, 246, 240, 0, 0, 0, 0, 71, 122, 133, 77, 0, 0, 0, 0, 185, 4, 31, 235, 201, 179, 0, 0, 0, 0, 40, 108, 170, 2, 0, 0, 0, 0, 153, 138, 116, 146, 0, 0, 0, 0, 57, 71, 193, 80, 0, 0, 0, 0, 185, 4, 4, 116, 141, 242, 0, 0, 0, 0, 85, 196, 54, 147, 0, 0, 0, 0, 20, 151, 217, 223, 0, 0, 0, 0, 45, 156, 222, 16, 0, 0, 0, 0, 185, 4, 212, 53, 211, 166, 0, 0, 0, 0, 100, 216, 191, 72, 0, 0, 0, 0, 25, 240, 216, 138, 0, 0, 0, 0, 65, 130, 240, 7, 0, 0, 0, 0, 185, 4, 171, 151, 14, 229, 0, 0, 0, 0, 40, 138, 19, 249, 0, 0, 0, 0, 150, 96, 160, 168, 0, 0, 0, 0, 187, 77, 19, 196, 0, 0, 0, 0, 185, 4, 54, 213, 255, 27, 0, 0, 0, 0, 52, 212, 98, 220, 0, 0, 0, 0, 160, 142, 118, 92, 0, 0, 0, 0, 46, 164, 48, 34, 0, 0, 0, 0, 185, 4, 111, 211, 191, 167, 0, 0, 0, 0, 118, 39, 42, 20, 0, 0, 0, 0, 3, 149, 232, 191, 0, 0, 0, 0, 13, 48, 33, 103, 0, 0, 0, 0, 185, 4, 20, 19, 137, 15, 0, 0, 0, 0, 239, 146, 211, 148, 0, 0, 0, 0, 133, 4, 61, 207, 0, 0, 0, 0, 99, 54, 114, 222, 0, 0, 0, 0, 185, 4, 196, 77, 115, 207, 0, 0, 0, 0, 248, 142, 249, 202, 0, 0, 0, 0, 116, 130, 170, 158, 0, 0, 0, 0, 73, 155, 238, 27, 0, 0, 0, 0, 185, 4, 84, 104, 167, 6, 0, 0, 0, 0, 45, 146, 183, 84, 0, 0, 0, 0, 101, 108, 197, 85, 0, 0, 0, 0, 66, 41, 70, 184, 0, 0, 0, 0, 185, 4, 117, 209, 65, 182, 0, 0, 0, 0, 139, 87, 207, 239, 0, 0, 0, 0, 153, 90, 48, 77, 0, 0, 0, 0, 49, 85, 192, 96, 0, 0, 0, 0, 185, 4, 168, 136, 125, 77, 0, 0, 0, 0, 55, 74, 117, 250, 0, 0, 0, 0, 230, 33, 197, 119, 0, 0, 0, 0, 98, 244, 187, 183, 0, 0, 0, 0, 185, 4, 39, 183, 156, 211, 0, 0, 0, 0, 174, 30, 241, 23, 0, 0, 0, 0, 118, 117, 202, 7, 0, 0, 0, 0, 91, 132, 239, 70, 0, 0, 0, 0, 185, 4, 207, 220, 3, 149, 0, 0, 0, 0, 131, 113, 246, 13, 0, 0, 0, 0, 12, 40, 183, 193, 0, 0, 0, 0, 104, 83, 146, 72, 0, 0, 0, 0, 185, 4, 224, 188, 197, 14, 0, 0, 0, 0, 251, 37, 71, 105, 0, 0, 0, 0, 168, 173, 24, 131, 0, 0, 0, 0, 246, 112, 215, 156, 0, 0, 0, 0, 185, 4, 45, 7, 208, 4, 0, 0, 0, 0, 63, 253, 219, 103, 0, 0, 0, 0, 163, 80, 75, 148, 0, 0, 0, 0, 221, 122, 82, 32, 0, 0, 0, 0, 185, 4, 205, 196, 113, 23, 0, 0, 0, 0, 189, 231, 81, 73, 0, 0, 0, 0, 112, 56, 107, 118, 0, 0, 0, 0, 236, 94, 207, 40, 0, 0, 0, 0, 185, 4, 169, 211, 248, 122, 0, 0, 0, 0, 50, 102, 155, 132, 0, 0, 0, 0, 238, 47, 76, 93, 0, 0, 0, 0, 105, 236, 83, 211, 0, 0, 0, 0, 185, 4, 159, 23, 211, 41, 0, 0, 0, 0, 25, 124, 120, 148, 0, 0, 0, 0, 91, 214, 108, 101, 0, 0, 0, 0, 221, 245, 45, 110, 0, 0, 0, 0, 185, 4, 25, 103, 135, 141, 0, 0, 0, 0, 174, 75, 44, 43, 0, 0, 0, 0, 224, 132, 91, 21, 0, 0, 0, 0, 190, 173, 92, 127, 0, 0, 0, 0, 185, 4, 11, 100, 220, 64, 0, 0, 0, 0, 93, 196, 9, 45, 0, 0, 0, 0, 92, 110, 250, 44, 0, 0, 0, 0, 51, 249, 37, 29, 0, 0, 0, 0, 185, 4, 194, 85, 3, 93, 0, 0, 0, 0, 193, 194, 27, 87, 0, 0, 0, 0, 170, 129, 101, 96, 0, 0, 0, 0, 21, 207, 67, 235, 0, 0, 0, 0, 185, 4, 158, 125, 85, 162, 0, 0, 0, 0, 187, 205, 208, 46, 0, 0, 0, 0, 188, 143, 110, 255, 0, 0, 0, 0, 10, 187, 202, 158, 0, 0, 0, 0, 185, 4, 97, 59, 234, 85, 0, 0, 0, 0, 141, 229, 158, 235, 0, 0, 0, 0, 173, 66, 165, 74, 0, 0, 0, 0, 186, 202, 7, 130, 0, 0, 0, 0, 185, 4, 7, 236, 247, 82, 0, 0, 0, 0, 131, 72, 171, 209, 0, 0, 0, 0, 212, 216, 124, 155, 0, 0, 0, 0, 74, 56, 210, 29, 0, 0, 0, 0, 185, 4, 174, 26, 196, 9, 0, 0, 0, 0, 72, 218, 221, 177, 0, 0, 0, 0, 15, 128, 56, 145, 0, 0, 0, 0, 158, 13, 34, 43, 0, 0, 0, 0, 185, 4, 211, 175, 173, 135, 0, 0, 0, 0, 177, 107, 185, 254, 0, 0, 0, 0, 244, 180, 15, 187, 0, 0, 0, 0, 139, 98, 211, 133, 0, 0, 0, 0, 185, 4, 144, 119, 53, 60, 0, 0, 0, 0, 236, 96, 190, 83, 0, 0, 0, 0, 215, 228, 94, 147, 0, 0, 0, 0, 126, 192, 107, 137, 0, 0, 0, 0, 185, 4, 205, 195, 183, 246, 0, 0, 0, 0, 122, 91, 40, 81, 0, 0, 0, 0, 126, 136, 59, 2, 0, 0, 0, 0, 102, 239, 202, 190, 0, 0, 0, 0, 185, 4, 170, 209, 206, 194, 0, 0, 0, 0, 97, 32, 12, 34, 0, 0, 0, 0, 73, 91, 180, 232, 0, 0, 0, 0, 169, 112, 101, 202, 0, 0, 0, 0, 185, 4, 140, 45, 222, 167, 0, 0, 0, 0, 107, 15, 146, 163, 0, 0, 0, 0, 92, 1, 230, 190, 0, 0, 0, 0, 14, 205, 76, 16, 0, 0, 0, 0, 185, 4, 101, 77, 111, 70, 0, 0, 0, 0, 47, 37, 104, 43, 0, 0, 0, 0, 33, 71, 105, 151, 0, 0, 0, 0, 220, 117, 60, 173, 0, 0, 0, 0, 185, 4, 217, 167, 24, 242, 0, 0, 0, 0, 0, 40, 194, 147, 0, 0, 0, 0, 112, 32, 132, 226, 0, 0, 0, 0, 183, 89, 242, 238, 0, 0, 0, 0, 185, 4, 154, 68, 110, 140, 0, 0, 0, 0, 82, 200, 216, 86, 0, 0, 0, 0, 59, 93, 145, 30, 0, 0, 0, 0, 10, 15, 140, 90, 0, 0, 0, 0, 185, 4, 35, 136, 217, 91, 0, 0, 0, 0, 179, 137, 156, 33, 0, 0, 0, 0, 89, 202, 66, 156, 0, 0, 0, 0, 116, 237, 86, 159, 0, 0, 0, 0, 185, 4, 177, 223, 252, 13, 0, 0, 0, 0, 58, 189, 79, 15, 0, 0, 0, 0, 5, 140, 28, 3, 0, 0, 0, 0, 189, 109, 49, 157, 0, 0, 0, 0, 185, 4, 106, 105, 154, 160, 0, 0, 0, 0, 221, 178, 109, 152, 0, 0, 0, 0, 190, 187, 2, 187, 0, 0, 0, 0, 151, 1, 237, 73, 0, 0, 0, 0, 185, 4, 210, 10, 34, 236, 0, 0, 0, 0, 125, 172, 197, 228, 0, 0, 0, 0, 31, 121, 10, 177, 0, 0, 0, 0, 48, 237, 215, 32, 0, 0, 0, 0, 185, 4, 200, 45, 73, 144, 0, 0, 0, 0, 102, 142, 177, 148, 0, 0, 0, 0, 80, 25, 54, 43, 0, 0, 0, 0, 113, 11, 123, 99, 0, 0, 0, 0, 185, 4, 36, 217, 17, 100, 0, 0, 0, 0, 228, 47, 221, 48, 0, 0, 0, 0, 157, 124, 218, 54, 0, 0, 0, 0, 80, 84, 132, 208, 0, 0, 0, 0, 185, 4, 82, 90, 23, 204, 0, 0, 0, 0, 189, 200, 58, 243, 0, 0, 0, 0, 253, 133, 212, 178, 0, 0, 0, 0, 140, 148, 62, 213, 0, 0, 0, 0, 185, 4, 215, 110, 13, 22, 0, 0, 0, 0, 56, 88, 57, 199, 0, 0, 0, 0, 234, 171, 240, 149, 0, 0, 0, 0, 16, 143, 164, 83, 0, 0, 0, 0, 185, 4, 2, 148, 174, 9, 0, 0, 0, 0, 163, 52, 60, 165, 0, 0, 0, 0, 44, 105, 9, 77, 0, 0, 0, 0, 64, 163, 137, 8, 0, 0, 0, 0, 185, 4, 95, 73, 82, 99, 0, 0, 0, 0, 138, 138, 176, 110, 0, 0, 0, 0, 213, 253, 252, 114, 0, 0, 0, 0, 79, 173, 74, 209, 0, 0, 0, 0, 185, 4, 106, 250, 179, 60, 0, 0, 0, 0, 47, 133, 250, 255, 0, 0, 0, 0, 53, 41, 203, 173, 0, 0, 0, 0, 67, 54, 34, 22, 0, 0, 0, 0, 185, 4, 158, 49, 31, 24, 0, 0, 0, 0, 191, 17, 170, 251, 0, 0, 0, 0, 64, 50, 233, 132, 0, 0, 0, 0, 131, 35, 23, 126, 0, 0, 0, 0, 185, 4, 1, 170, 114, 68, 0, 0, 0, 0, 70, 132, 0, 117, 0, 0, 0, 0, 207, 6, 74, 188, 0, 0, 0, 0, 101, 168, 31, 198, 0, 0, 0, 0, 185, 4, 39, 6, 165, 46, 0, 0, 0, 0, 37, 156, 233, 143, 0, 0, 0, 0, 195, 215, 248, 252, 0, 0, 0, 0, 189, 206, 253, 20, 0, 0, 0, 0, 185, 4, 38, 77, 3, 241, 0, 0, 0, 0, 141, 147, 209, 143, 0, 0, 0, 0, 113, 223, 76, 45, 0, 0, 0, 0, 222, 123, 134, 156, 0, 0, 0, 0, 185, 4, 139, 29, 215, 171, 0, 0, 0, 0, 175, 182, 191, 183, 0, 0, 0, 0, 62, 240, 63, 53, 0, 0, 0, 0, 60, 254, 229, 115, 0, 0, 0, 0, 185, 4, 166, 199, 172, 200, 0, 0, 0, 0, 219, 225, 150, 23, 0, 0, 0, 0, 177, 248, 223, 56, 0, 0, 0, 0, 175, 135, 162, 237, 0, 0, 0, 0, 185, 4, 136, 210, 40, 247, 0, 0, 0, 0, 103, 147, 34, 19, 0, 0, 0, 0, 144, 34, 69, 209, 0, 0, 0, 0, 102, 227, 57, 76, 0, 0, 0, 0, 185, 4, 54, 119, 81, 57, 0, 0, 0, 0, 253, 185, 22, 3, 0, 0, 0, 0, 66, 30, 71, 8, 0, 0, 0, 0, 20, 139, 75, 23, 0, 0, 0, 0, 185, 4, 149, 250, 174, 192, 0, 0, 0, 0, 50, 69, 33, 52, 0, 0, 0, 0, 65, 25, 236, 115, 0, 0, 0, 0, 159, 107, 125, 155, 0, 0, 0, 0, 185, 4, 209, 133, 25, 10, 0, 0, 0, 0, 11, 227, 204, 232, 0, 0, 0, 0, 233, 50, 174, 157, 0, 0, 0, 0, 244, 172, 155, 180, 0, 0, 0, 0, 185, 4, 159, 147, 222, 163, 0, 0, 0, 0, 4, 168, 84, 28, 0, 0, 0, 0, 150, 252, 74, 117, 0, 0, 0, 0, 147, 206, 200, 235, 0, 0, 0, 0, 185, 4, 29, 135, 243, 125, 0, 0, 0, 0, 81, 201, 115, 218, 0, 0, 0, 0, 48, 241, 174, 225, 0, 0, 0, 0, 186, 10, 251, 136, 0, 0, 0, 0, 185, 4, 121, 228, 97, 153, 0, 0, 0, 0, 232, 83, 210, 215, 0, 0, 0, 0, 117, 54, 184, 191, 0, 0, 0, 0, 124, 221, 201, 109, 0, 0, 0, 0, 185, 4, 81, 53, 84, 250, 0, 0, 0, 0, 29, 99, 211, 77, 0, 0, 0, 0, 25, 187, 155, 156, 0, 0, 0, 0, 6, 3, 149, 106, 0, 0, 0, 0, 185, 4, 96, 32, 198, 67, 0, 0, 0, 0, 193, 33, 196, 204, 0, 0, 0, 0, 98, 25, 206, 206, 0, 0, 0, 0, 210, 170, 11, 240, 0, 0, 0, 0, 185, 4, 25, 64, 113, 222, 0, 0, 0, 0, 110, 54, 124, 138, 0, 0, 0, 0, 254, 81, 207, 157, 0, 0, 0, 0, 251, 39, 48, 220, 0, 0, 0, 0, 185, 4, 163, 150, 81, 94, 0, 0, 0, 0, 96, 209, 98, 75, 0, 0, 0, 0, 222, 230, 89, 175, 0, 0, 0, 0, 49, 60, 202, 210, 0, 0, 0, 0, 185, 4, 222, 42, 199, 152, 0, 0, 0, 0, 99, 64, 25, 67, 0, 0, 0, 0, 204, 72, 164, 30, 0, 0, 0, 0, 5, 70, 4, 67, 0, 0, 0, 0, 185, 4, 199, 43, 70, 107, 0, 0, 0, 0, 53, 248, 128, 183, 0, 0, 0, 0, 164, 116, 85, 15, 0, 0, 0, 0, 221, 52, 63, 224, 0, 0, 0, 0, 185, 4, 195, 98, 29, 221, 0, 0, 0, 0, 36, 218, 198, 32, 0, 0, 0, 0, 203, 44, 165, 133, 0, 0, 0, 0, 39, 44, 225, 222, 0, 0, 0, 0, 185, 4, 62, 217, 186, 26, 0, 0, 0, 0, 66, 218, 177, 205, 0, 0, 0, 0, 54, 228, 131, 32, 0, 0, 0, 0, 132, 73, 155, 71, 0, 0, 0, 0, 185, 4, 252, 100, 157, 216, 0, 0, 0, 0, 253, 73, 42, 190, 0, 0, 0, 0, 59, 127, 147, 52, 0, 0, 0, 0, 175, 49, 119, 234, 0, 0, 0, 0, 185, 4, 238, 5, 116, 80, 0, 0, 0, 0, 20, 254, 75, 192, 0, 0, 0, 0, 148, 178, 82, 108, 0, 0, 0, 0, 179, 89, 114, 252, 0, 0, 0, 0, 185, 4, 48, 41, 137, 80, 0, 0, 0, 0, 240, 83, 39, 250, 0, 0, 0, 0, 126, 42, 134, 253, 0, 0, 0, 0, 127, 183, 49, 41, 0, 0, 0, 0, 185, 4, 234, 247, 162, 163, 0, 0, 0, 0, 236, 247, 1, 47, 0, 0, 0, 0, 189, 129, 217, 252, 0, 0, 0, 0, 2, 189, 54, 97, 0, 0, 0, 0, 185, 4, 224, 137, 168, 159, 0, 0, 0, 0, 225, 251, 203, 182, 0, 0, 0, 0, 51, 244, 10, 196, 0, 0, 0, 0, 177, 29, 102, 74, 0, 0, 0, 0, 185, 4, 102, 240, 58, 217, 0, 0, 0, 0, 33, 207, 126, 222, 0, 0, 0, 0, 140, 236, 126, 140, 0, 0, 0, 0, 47, 1, 80, 40, 0, 0, 0, 0, 185, 4, 235, 57, 52, 114, 0, 0, 0, 0, 189, 116, 153, 220, 0, 0, 0, 0, 47, 72, 167, 78, 0, 0, 0, 0, 22, 0, 175, 34, 0, 0, 0, 0, 185, 4, 108, 37, 27, 183, 0, 0, 0, 0, 91, 82, 98, 187, 0, 0, 0, 0, 50, 9, 68, 79, 0, 0, 0, 0, 5, 174, 84, 160, 0, 0, 0, 0, 185, 4, 237, 237, 154, 96, 0, 0, 0, 0, 197, 88, 162, 57, 0, 0, 0, 0, 194, 116, 112, 193, 0, 0, 0, 0, 189, 104, 108, 112, 0, 0, 0, 0, 185, 4, 22, 109, 146, 181, 0, 0, 0, 0, 30, 54, 96, 21, 0, 0, 0, 0, 66, 215, 61, 228, 0, 0, 0, 0, 150, 187, 195, 27, 0, 0, 0, 0, 185, 4, 118, 50, 74, 56, 0, 0, 0, 0, 130, 47, 121, 154, 0, 0, 0, 0, 63, 152, 67, 149, 0, 0, 0, 0, 66, 98, 204, 33, 0, 0, 0, 0, 185, 4, 150, 8, 212, 233, 0, 0, 0, 0, 195, 94, 124, 6, 0, 0, 0, 0, 200, 168, 43, 225, 0, 0, 0, 0, 112, 79, 131, 88, 0, 0, 0, 0, 185, 4, 101, 12, 147, 161, 0, 0, 0, 0, 89, 151, 82, 96, 0, 0, 0, 0, 176, 220, 153, 32, 0, 0, 0, 0, 133, 215, 3, 193, 0, 0, 0, 0, 185, 4, 253, 249, 244, 151, 0, 0, 0, 0, 195, 246, 185, 244, 0, 0, 0, 0, 212, 8, 51, 207, 0, 0, 0, 0, 159, 79, 133, 177, 0, 0, 0, 0, 185, 4, 128, 147, 64, 168, 0, 0, 0, 0, 66, 177, 196, 55, 0, 0, 0, 0, 41, 131, 223, 85, 0, 0, 0, 0, 225, 203, 70, 30, 0, 0, 0, 0, 185, 4, 210, 115, 92, 181, 0, 0, 0, 0, 106, 224, 245, 169, 0, 0, 0, 0, 220, 193, 91, 46, 0, 0, 0, 0, 179, 161, 156, 251, 0, 0, 0, 0, 185, 4, 139, 185, 31, 179, 0, 0, 0, 0, 21, 251, 95, 20, 0, 0, 0, 0, 219, 28, 100, 222, 0, 0, 0, 0, 214, 250, 26, 9, 0, 0, 0, 0, 185, 4, 230, 181, 56, 219, 0, 0, 0, 0, 36, 199, 87, 95, 0, 0, 0, 0, 184, 211, 172, 128, 0, 0, 0, 0, 127, 192, 228, 62, 0, 0, 0, 0, 185, 4, 111, 142, 110, 186, 0, 0, 0, 0, 71, 229, 106, 180, 0, 0, 0, 0, 146, 179, 203, 117, 0, 0, 0, 0, 244, 90, 212, 92, 0, 0, 0, 0, 185, 4, 198, 224, 194, 60, 0, 0, 0, 0, 35, 245, 63, 66, 0, 0, 0, 0, 22, 69, 11, 32, 0, 0, 0, 0, 102, 64, 66, 227, 0, 0, 0, 0, 185, 4, 255, 227, 45, 136, 0, 0, 0, 0, 64, 178, 132, 107, 0, 0, 0, 0, 171, 203, 84, 211, 0, 0, 0, 0, 131, 231, 71, 81, 0, 0, 0, 0, 185, 4, 79, 95, 10, 115, 0, 0, 0, 0, 98, 97, 218, 236, 0, 0, 0, 0, 202, 53, 44, 185, 0, 0, 0, 0, 121, 29, 46, 237, 0, 0, 0, 0, 185, 4, 160, 176, 138, 104, 0, 0, 0, 0, 248, 152, 249, 206, 0, 0, 0, 0, 223, 94, 212, 187, 0, 0, 0, 0, 224, 74, 111, 201, 0, 0, 0, 0, 185, 4, 194, 147, 219, 90, 0, 0, 0, 0, 171, 221, 141, 209, 0, 0, 0, 0, 163, 47, 239, 232, 0, 0, 0, 0, 220, 128, 68, 98, 0, 0, 0, 0, 185, 4, 2, 179, 68, 57, 0, 0, 0, 0, 1, 226, 118, 41, 0, 0, 0, 0, 214, 228, 13, 214, 0, 0, 0, 0, 23, 116, 140, 39, 0, 0, 0, 0, 185, 4, 155, 216, 213, 118, 0, 0, 0, 0, 75, 201, 78, 31, 0, 0, 0, 0, 22, 93, 93, 50, 0, 0, 0, 0, 13, 160, 251, 232, 0, 0, 0, 0, 185, 4, 240, 242, 29, 161, 0, 0, 0, 0, 111, 122, 13, 158, 0, 0, 0, 0, 147, 129, 227, 53, 0, 0, 0, 0, 105, 63, 87, 37, 0, 0, 0, 0, 185, 4, 75, 181, 11, 210, 0, 0, 0, 0, 14, 123, 90, 19, 0, 0, 0, 0, 239, 152, 194, 208, 0, 0, 0, 0, 64, 203, 117, 67, 0, 0, 0, 0, 185, 4, 162, 149, 79, 226, 0, 0, 0, 0, 8, 45, 213, 184, 0, 0, 0, 0, 144, 240, 225, 97, 0, 0, 0, 0, 182, 109, 137, 238, 0, 0, 0, 0, 185, 4, 160, 28, 248, 82, 0, 0, 0, 0, 151, 242, 248, 24, 0, 0, 0, 0, 36, 209, 211, 67, 0, 0, 0, 0, 106, 241, 14, 102, 0, 0, 0, 0, 185, 4, 127, 32, 220, 134, 0, 0, 0, 0, 46, 111, 99, 253, 0, 0, 0, 0, 151, 75, 238, 103, 0, 0, 0, 0, 81, 117, 194, 51, 0, 0, 0, 0, 185, 4, 208, 219, 139, 91, 0, 0, 0, 0, 117, 86, 173, 198, 0, 0, 0, 0, 207, 129, 205, 121, 0, 0, 0, 0, 120, 76, 153, 178, 0, 0, 0, 0, 185, 4, 211, 88, 66, 137, 0, 0, 0, 0, 113, 150, 74, 55, 0, 0, 0, 0, 54, 249, 13, 201, 0, 0, 0, 0, 102, 173, 53, 14, 0, 0, 0, 0, 185, 4, 74, 121, 164, 70, 0, 0, 0, 0, 232, 101, 2, 202, 0, 0, 0, 0, 249, 24, 167, 146, 0, 0, 0, 0, 20, 184, 215, 149, 0, 0, 0, 0, 185, 4, 198, 156, 103, 166, 0, 0, 0, 0, 10, 117, 75, 50, 0, 0, 0, 0, 203, 184, 222, 110, 0, 0, 0, 0, 247, 149, 65, 153, 0, 0, 0, 0, 185, 4, 201, 254, 128, 224, 0, 0, 0, 0, 44, 118, 70, 27, 0, 0, 0, 0, 22, 156, 152, 106, 0, 0, 0, 0, 46, 50, 1, 248, 0, 0, 0, 0, 185, 4, 85, 232, 55, 200, 0, 0, 0, 0, 77, 222, 69, 179, 0, 0, 0, 0, 128, 211, 214, 188, 0, 0, 0, 0, 1, 107, 113, 255, 0, 0, 0, 0, 185, 4, 83, 29, 171, 44, 0, 0, 0, 0, 233, 116, 73, 233, 0, 0, 0, 0, 18, 102, 23, 54, 0, 0, 0, 0, 218, 23, 33, 226, 0, 0, 0, 0, 185, 4, 249, 228, 8, 112, 0, 0, 0, 0, 236, 124, 194, 56, 0, 0, 0, 0, 250, 124, 98, 165, 0, 0, 0, 0, 134, 32, 246, 197, 0, 0, 0, 0, 185, 4, 218, 111, 76, 21, 0, 0, 0, 0, 218, 2, 2, 166, 0, 0, 0, 0, 27, 155, 37, 36, 0, 0, 0, 0, 99, 107, 209, 161, 0, 0, 0, 0, 185, 4, 70, 235, 60, 113, 0, 0, 0, 0, 61, 148, 6, 34, 0, 0, 0, 0, 131, 130, 172, 132, 0, 0, 0, 0, 190, 233, 174, 62, 0, 0, 0, 0, 185, 4, 229, 103, 32, 244, 0, 0, 0, 0, 18, 255, 98, 91, 0, 0, 0, 0, 224, 27, 170, 149, 0, 0, 0, 0, 103, 185, 152, 101, 0, 0, 0, 0, 185, 4, 74, 115, 121, 30, 0, 0, 0, 0, 113, 46, 71, 189, 0, 0, 0, 0, 121, 84, 134, 3, 0, 0, 0, 0, 190, 31, 192, 76, 0, 0, 0, 0, 185, 4, 35, 230, 89, 46, 0, 0, 0, 0, 164, 4, 128, 89, 0, 0, 0, 0, 103, 68, 64, 229, 0, 0, 0, 0, 52, 102, 11, 91, 0, 0, 0, 0, 185, 4, 189, 63, 235, 243, 0, 0, 0, 0, 68, 87, 138, 17, 0, 0, 0, 0, 3, 66, 236, 128, 0, 0, 0, 0, 109, 252, 172, 28, 0, 0, 0, 0, 185, 4, 126, 138, 61, 164, 0, 0, 0, 0, 89, 182, 203, 209, 0, 0, 0, 0, 166, 246, 206, 54, 0, 0, 0, 0, 92, 52, 227, 157, 0, 0, 0, 0, 185, 4, 47, 214, 176, 234, 0, 0, 0, 0, 151, 223, 157, 246, 0, 0, 0, 0, 69, 177, 33, 59, 0, 0, 0, 0, 36, 75, 91, 190, 0, 0, 0, 0, 185, 4, 104, 99, 69, 173, 0, 0, 0, 0, 97, 230, 191, 208, 0, 0, 0, 0, 0, 187, 246, 38, 0, 0, 0, 0, 115, 219, 109, 55, 0, 0, 0, 0, 185, 4, 88, 211, 188, 17, 0, 0, 0, 0, 0, 181, 90, 47, 0, 0, 0, 0, 4, 54, 63, 215, 0, 0, 0, 0, 31, 177, 154, 242, 0, 0, 0, 0, 185, 4, 179, 150, 195, 176, 0, 0, 0, 0, 23, 40, 71, 243, 0, 0, 0, 0, 195, 23, 233, 2, 0, 0, 0, 0, 227, 72, 76, 239, 0, 0, 0, 0, 185, 4, 121, 145, 129, 81, 0, 0, 0, 0, 143, 92, 231, 152, 0, 0, 0, 0, 139, 90, 40, 187, 0, 0, 0, 0, 111, 160, 45, 69, 0, 0, 0, 0, 185, 4, 139, 51, 134, 246, 0, 0, 0, 0, 200, 0, 42, 155, 0, 0, 0, 0, 151, 120, 20, 63, 0, 0, 0, 0, 141, 27, 145, 112, 0, 0, 0, 0, 185, 4, 186, 255, 191, 119, 0, 0, 0, 0, 244, 62, 144, 139, 0, 0, 0, 0, 55, 164, 235, 149, 0, 0, 0, 0, 51, 67, 131, 170, 0, 0, 0, 0, 185, 4, 233, 180, 117, 43, 0, 0, 0, 0, 110, 89, 118, 129, 0, 0, 0, 0, 230, 96, 139, 6, 0, 0, 0, 0, 42, 49, 67, 30, 0, 0, 0, 0, 185, 4, 44, 129, 181, 17, 0, 0, 0, 0, 65, 71, 81, 187, 0, 0, 0, 0, 183, 40, 226, 95, 0, 0, 0, 0, 9, 138, 101, 153, 0, 0, 0, 0, 185, 4, 225, 194, 196, 81, 0, 0, 0, 0, 232, 209, 207, 160, 0, 0, 0, 0, 219, 111, 116, 75, 0, 0, 0, 0, 45, 155, 33, 73, 0, 0, 0, 0, 185, 4, 111, 83, 92, 93, 0, 0, 0, 0, 208, 33, 30, 200, 0, 0, 0, 0, 38, 30, 111, 135, 0, 0, 0, 0, 245, 217, 236, 46, 0, 0, 0, 0, 185, 4, 23, 91, 250, 125, 0, 0, 0, 0, 87, 60, 206, 134, 0, 0, 0, 0, 238, 164, 98, 236, 0, 0, 0, 0, 212, 114, 27, 2, 0, 0, 0, 0, 185, 4, 226, 98, 22, 91, 0, 0, 0, 0, 96, 96, 84, 160, 0, 0, 0, 0, 163, 246, 31, 83, 0, 0, 0, 0, 163, 180, 173, 135, 0, 0, 0, 0, 185, 4, 54, 96, 254, 54, 0, 0, 0, 0, 77, 60, 70, 198, 0, 0, 0, 0, 105, 251, 75, 247, 0, 0, 0, 0, 165, 11, 91, 60, 0, 0, 0, 0, 185, 4, 230, 37, 107, 62, 0, 0, 0, 0, 196, 154, 168, 31, 0, 0, 0, 0, 209, 115, 208, 166, 0, 0, 0, 0, 99, 32, 143, 110, 0, 0, 0, 0, 185, 4, 135, 218, 90, 65, 0, 0, 0, 0, 255, 78, 85, 6, 0, 0, 0, 0, 50, 27, 245, 209, 0, 0, 0, 0, 62, 216, 143, 249, 0, 0, 0, 0, 185, 4, 183, 148, 72, 71, 0, 0, 0, 0, 177, 124, 159, 39, 0, 0, 0, 0, 25, 141, 173, 127, 0, 0, 0, 0, 81, 131, 1, 75, 0, 0, 0, 0, 185, 4, 33, 56, 169, 4, 0, 0, 0, 0, 39, 161, 82, 165, 0, 0, 0, 0, 56, 18, 63, 17, 0, 0, 0, 0, 205, 37, 219, 240, 0, 0, 0, 0, 185, 4, 89, 228, 146, 2, 0, 0, 0, 0, 43, 29, 154, 254, 0, 0, 0, 0, 129, 130, 86, 178, 0, 0, 0, 0, 232, 241, 2, 32, 0, 0, 0, 0, 185, 4, 112, 243, 55, 179, 0, 0, 0, 0, 138, 67, 211, 233, 0, 0, 0, 0, 48, 49, 170, 118, 0, 0, 0, 0, 193, 143, 189, 128, 0, 0, 0, 0, 185, 4, 184, 247, 242, 196, 0, 0, 0, 0, 60, 246, 64, 151, 0, 0, 0, 0, 138, 97, 94, 209, 0, 0, 0, 0, 125, 43, 187, 126, 0, 0, 0, 0, 185, 4, 145, 25, 214, 146, 0, 0, 0, 0, 47, 224, 179, 44, 0, 0, 0, 0, 193, 85, 152, 99, 0, 0, 0, 0, 84, 217, 116, 17, 0, 0, 0, 0, 185, 4, 79, 9, 161, 135, 0, 0, 0, 0, 112, 41, 38, 169, 0, 0, 0, 0, 156, 242, 179, 98, 0, 0, 0, 0, 179, 120, 60, 229, 0, 0, 0, 0, 185, 4, 42, 175, 166, 89, 0, 0, 0, 0, 8, 44, 169, 167, 0, 0, 0, 0, 251, 107, 96, 64, 0, 0, 0, 0, 91, 108, 90, 7, 0, 0, 0, 0, 185, 4, 136, 22, 140, 166, 0, 0, 0, 0, 128, 31, 207, 31, 0, 0, 0, 0, 235, 48, 101, 45, 0, 0, 0, 0, 236, 2, 147, 76, 0, 0, 0, 0, 185, 4, 202, 52, 223, 215, 0, 0, 0, 0, 67, 153, 170, 174, 0, 0, 0, 0, 186, 65, 190, 131, 0, 0, 0, 0, 71, 120, 123, 208, 0, 0, 0, 0, 185, 4, 178, 217, 59, 183, 0, 0, 0, 0, 245, 41, 139, 123, 0, 0, 0, 0, 139, 233, 201, 94, 0, 0, 0, 0, 190, 227, 16, 139, 0, 0, 0, 0, 185, 4, 76, 64, 106, 16, 0, 0, 0, 0, 40, 33, 76, 151, 0, 0, 0, 0, 231, 188, 252, 170, 0, 0, 0, 0, 175, 55, 40, 228, 0, 0, 0, 0, 185, 4, 171, 0, 213, 9, 0, 0, 0, 0, 116, 110, 204, 4, 0, 0, 0, 0, 197, 172, 11, 27, 0, 0, 0, 0, 65, 109, 55, 86, 0, 0, 0, 0, 185, 4, 212, 31, 153, 168, 0, 0, 0, 0, 118, 1, 119, 26, 0, 0, 0, 0, 127, 247, 192, 68, 0, 0, 0, 0, 177, 47, 57, 84, 0, 0, 0, 0, 185, 4, 206, 57, 123, 44, 0, 0, 0, 0, 211, 223, 34, 152, 0, 0, 0, 0, 178, 28, 140, 23, 0, 0, 0, 0, 217, 196, 140, 155, 0, 0, 0, 0, 185, 4, 71, 72, 98, 57, 0, 0, 0, 0, 20, 180, 96, 39, 0, 0, 0, 0, 133, 185, 240, 160, 0, 0, 0, 0, 252, 96, 161, 161, 0, 0, 0, 0, 185, 4, 38, 185, 151, 143, 0, 0, 0, 0, 185, 193, 93, 144, 0, 0, 0, 0, 160, 246, 230, 74, 0, 0, 0, 0, 122, 8, 134, 90, 0, 0, 0, 0, 185, 4, 131, 78, 159, 126, 0, 0, 0, 0, 27, 14, 237, 20, 0, 0, 0, 0, 172, 148, 133, 218, 0, 0, 0, 0, 181, 79, 14, 137, 0, 0, 0, 0, 185, 4, 98, 219, 26, 234, 0, 0, 0, 0, 146, 64, 67, 174, 0, 0, 0, 0, 208, 2, 106, 132, 0, 0, 0, 0, 170, 67, 29, 49, 0, 0, 0, 0, 185, 4, 71, 75, 16, 240, 0, 0, 0, 0, 60, 3, 143, 226, 0, 0, 0, 0, 244, 121, 195, 105, 0, 0, 0, 0, 244, 67, 207, 9, 0, 0, 0, 0, 185, 4, 86, 233, 64, 189, 0, 0, 0, 0, 51, 226, 54, 222, 0, 0, 0, 0, 212, 126, 59, 118, 0, 0, 0, 0, 63, 63, 73, 21, 0, 0, 0, 0, 185, 4, 13, 109, 4, 250, 0, 0, 0, 0, 101, 2, 148, 181, 0, 0, 0, 0, 69, 107, 172, 36, 0, 0, 0, 0, 92, 214, 9, 144, 0, 0, 0, 0, 185, 4, 116, 182, 48, 138, 0, 0, 0, 0, 238, 130, 138, 143, 0, 0, 0, 0, 7, 209, 42, 3, 0, 0, 0, 0, 150, 252, 60, 157, 0, 0, 0, 0, 185, 4, 23, 64, 212, 49, 0, 0, 0, 0, 230, 178, 35, 14, 0, 0, 0, 0, 70, 60, 1, 131, 0, 0, 0, 0, 244, 218, 113, 21, 0, 0, 0, 0, 185, 4, 96, 51, 109, 14, 0, 0, 0, 0, 143, 232, 218, 225, 0, 0, 0, 0, 208, 19, 205, 53, 0, 0, 0, 0, 145, 90, 253, 87, 0, 0, 0, 0, 185, 4, 162, 207, 208, 109, 0, 0, 0, 0, 147, 189, 232, 45, 0, 0, 0, 0, 60, 116, 131, 168, 0, 0, 0, 0, 167, 95, 89, 192, 0, 0, 0, 0, 185, 4, 99, 35, 138, 223, 0, 0, 0, 0, 110, 38, 79, 121, 0, 0, 0, 0, 197, 183, 168, 203, 0, 0, 0, 0, 7, 119, 224, 35, 0, 0, 0, 0, 185, 4, 228, 17, 254, 135, 0, 0, 0, 0, 151, 39, 183, 206, 0, 0, 0, 0, 53, 203, 52, 227, 0, 0, 0, 0, 96, 155, 217, 36, 0, 0, 0, 0, 185, 4, 123, 22, 80, 44, 0, 0, 0, 0, 117, 35, 228, 74, 0, 0, 0, 0, 40, 129, 191, 89, 0, 0, 0, 0, 144, 22, 38, 48, 0, 0, 0, 0, 185, 4, 232, 165, 6, 81, 0, 0, 0, 0, 182, 10, 55, 32, 0, 0, 0, 0, 175, 170, 6, 93, 0, 0, 0, 0, 247, 134, 16, 30, 0, 0, 0, 0, 185, 4, 227, 159, 66, 84, 0, 0, 0, 0, 44, 86, 235, 120, 0, 0, 0, 0, 219, 183, 130, 54, 0, 0, 0, 0, 81, 166, 93, 218, 0, 0, 0, 0, 185, 4, 239, 157, 48, 90, 0, 0, 0, 0, 84, 92, 84, 154, 0, 0, 0, 0, 162, 200, 107, 167, 0, 0, 0, 0, 211, 58, 226, 96, 0, 0, 0, 0, 185, 4, 157, 39, 91, 72, 0, 0, 0, 0, 85, 220, 85, 84, 0, 0, 0, 0, 148, 83, 65, 224, 0, 0, 0, 0, 9, 100, 114, 142, 0, 0, 0, 0, 185, 4, 107, 90, 143, 245, 0, 0, 0, 0, 140, 3, 200, 178, 0, 0, 0, 0, 83, 242, 199, 10, 0, 0, 0, 0, 181, 112, 218, 78, 0, 0, 0, 0, 185, 4, 165, 152, 44, 172, 0, 0, 0, 0, 246, 167, 14, 235, 0, 0, 0, 0, 21, 100, 147, 183, 0, 0, 0, 0, 95, 54, 160, 62, 0, 0, 0, 0, 185, 4, 163, 118, 201, 119, 0, 0, 0, 0, 83, 44, 208, 171, 0, 0, 0, 0, 252, 38, 108, 246, 0, 0, 0, 0, 116, 162, 141, 142, 0, 0, 0, 0, 185, 4, 196, 162, 28, 185, 0, 0, 0, 0, 199, 23, 66, 248, 0, 0, 0, 0, 166, 23, 54, 143, 0, 0, 0, 0, 116, 60, 56, 161, 0, 0, 0, 0, 185, 4, 233, 57, 98, 245, 0, 0, 0, 0, 30, 124, 128, 203, 0, 0, 0, 0, 199, 38, 193, 101, 0, 0, 0, 0, 153, 178, 148, 127, 0, 0, 0, 0, 185, 4, 85, 229, 57, 171, 0, 0, 0, 0, 94, 201, 253, 64, 0, 0, 0, 0, 34, 139, 217, 246, 0, 0, 0, 0, 165, 42, 222, 65, 0, 0, 0, 0, 185, 4, 173, 78, 25, 175, 0, 0, 0, 0, 75, 250, 111, 91, 0, 0, 0, 0, 32, 114, 35, 55, 0, 0, 0, 0, 213, 74, 120, 55, 0, 0, 0, 0, 185, 4, 23, 156, 29, 16, 0, 0, 0, 0, 24, 133, 52, 5, 0, 0, 0, 0, 161, 212, 244, 176, 0, 0, 0, 0, 242, 69, 196, 131, 0, 0, 0, 0, 185, 4, 27, 244, 35, 157, 0, 0, 0, 0, 0, 230, 211, 111, 0, 0, 0, 0, 179, 195, 115, 5, 0, 0, 0, 0, 216, 248, 83, 81, 0, 0, 0, 0, 185, 4, 223, 13, 146, 143, 0, 0, 0, 0, 117, 58, 223, 95, 0, 0, 0, 0, 138, 209, 30, 175, 0, 0, 0, 0, 32, 133, 15, 192, 0, 0, 0, 0, 185, 4, 106, 70, 65, 252, 0, 0, 0, 0, 104, 139, 214, 29, 0, 0, 0, 0, 154, 71, 8, 118, 0, 0, 0, 0, 18, 97, 137, 83, 0, 0, 0, 0, 185, 4, 64, 65, 142, 9, 0, 0, 0, 0, 33, 193, 223, 57, 0, 0, 0, 0, 247, 141, 205, 36, 0, 0, 0, 0, 54, 184, 49, 253, 0, 0, 0, 0, 185, 4, 167, 145, 27, 160, 0, 0, 0, 0, 161, 27, 22, 41, 0, 0, 0, 0, 94, 198, 39, 129, 0, 0, 0, 0, 106, 253, 186, 109, 0, 0, 0, 0, 185, 4, 186, 93, 207, 201, 0, 0, 0, 0, 59, 158, 134, 198, 0, 0, 0, 0, 147, 188, 121, 242, 0, 0, 0, 0, 170, 144, 153, 70, 0, 0, 0, 0, 185, 4, 217, 184, 9, 66, 0, 0, 0, 0, 247, 53, 247, 81, 0, 0, 0, 0, 218, 172, 127, 106, 0, 0, 0, 0, 210, 175, 78, 237, 0, 0, 0, 0, 185, 4, 80, 136, 119, 233, 0, 0, 0, 0, 228, 65, 117, 111, 0, 0, 0, 0, 182, 81, 114, 230, 0, 0, 0, 0, 102, 220, 159, 90, 0, 0, 0, 0, 185, 4, 35, 65, 198, 46, 0, 0, 0, 0, 27, 37, 179, 29, 0, 0, 0, 0, 175, 125, 229, 21, 0, 0, 0, 0, 184, 88, 228, 18, 0, 0, 0, 0, 185, 4, 186, 190, 141, 234, 0, 0, 0, 0, 51, 148, 49, 94, 0, 0, 0, 0, 206, 26, 38, 134, 0, 0, 0, 0, 173, 95, 218, 125, 0, 0, 0, 0, 185, 4, 182, 108, 72, 42, 0, 0, 0, 0, 89, 115, 229, 101, 0, 0, 0, 0, 174, 132, 145, 156, 0, 0, 0, 0, 150, 121, 176, 139, 0, 0, 0, 0, 185, 4, 74, 206, 162, 139, 0, 0, 0, 0, 124, 90, 7, 163, 0, 0, 0, 0, 206, 48, 73, 138, 0, 0, 0, 0, 205, 176, 83, 50, 0, 0, 0, 0, 185, 4, 146, 173, 29, 39, 0, 0, 0, 0, 216, 58, 128, 80, 0, 0, 0, 0, 29, 194, 12, 236, 0, 0, 0, 0, 101, 153, 36, 29, 0, 0, 0, 0, 185, 4, 21, 19, 119, 169, 0, 0, 0, 0, 64, 65, 253, 175, 0, 0, 0, 0, 82, 248, 49, 115, 0, 0, 0, 0, 169, 245, 32, 183, 0, 0, 0, 0, 185, 4, 108, 6, 56, 7, 0, 0, 0, 0, 35, 130, 136, 23, 0, 0, 0, 0, 187, 224, 103, 82, 0, 0, 0, 0, 191, 220, 65, 15, 0, 0, 0, 0, 185, 4, 86, 224, 231, 69, 0, 0, 0, 0, 46, 167, 42, 175, 0, 0, 0, 0, 75, 197, 221, 49, 0, 0, 0, 0, 27, 27, 18, 125, 0, 0, 0, 0, 185, 4, 185, 98, 204, 192, 0, 0, 0, 0, 33, 123, 246, 225, 0, 0, 0, 0, 189, 146, 236, 210, 0, 0, 0, 0, 185, 155, 28, 10, 0, 0, 0, 0, 185, 4, 203, 247, 104, 62, 0, 0, 0, 0, 153, 137, 56, 196, 0, 0, 0, 0, 125, 226, 34, 245, 0, 0, 0, 0, 198, 131, 158, 179, 0, 0, 0, 0, 185, 4, 122, 70, 137, 233, 0, 0, 0, 0, 11, 200, 80, 0, 0, 0, 0, 0, 105, 168, 213, 79, 0, 0, 0, 0, 60, 187, 191, 161, 0, 0, 0, 0, 185, 4, 62, 81, 2, 30, 0, 0, 0, 0, 194, 40, 39, 163, 0, 0, 0, 0, 116, 191, 116, 61, 0, 0, 0, 0, 137, 52, 173, 88, 0, 0, 0, 0, 185, 4, 218, 178, 37, 238, 0, 0, 0, 0, 100, 36, 242, 63, 0, 0, 0, 0, 199, 27, 221, 94, 0, 0, 0, 0, 236, 108, 165, 10, 0, 0, 0, 0, 185, 4, 107, 21, 187, 223, 0, 0, 0, 0, 161, 84, 203, 160, 0, 0, 0, 0, 1, 86, 127, 81, 0, 0, 0, 0, 182, 193, 234, 162, 0, 0, 0, 0, 185, 4, 73, 103, 112, 26, 0, 0, 0, 0, 238, 172, 210, 149, 0, 0, 0, 0, 95, 63, 234, 26, 0, 0, 0, 0, 196, 30, 250, 107, 0, 0, 0, 0, 185, 4, 189, 29, 219, 54, 0, 0, 0, 0, 129, 243, 140, 64, 0, 0, 0, 0, 80, 97, 127, 59, 0, 0, 0, 0, 75, 128, 147, 188, 0, 0, 0, 0, 185, 4, 235, 184, 8, 69, 0, 0, 0, 0, 52, 142, 121, 226, 0, 0, 0, 0, 241, 191, 177, 229, 0, 0, 0, 0, 249, 55, 178, 49, 0, 0, 0, 0, 185, 4, 53, 242, 59, 189, 0, 0, 0, 0, 137, 55, 20, 59, 0, 0, 0, 0, 66, 44, 73, 161, 0, 0, 0, 0, 1, 15, 134, 22, 0, 0, 0, 0, 185, 4, 136, 227, 143, 81, 0, 0, 0, 0, 137, 67, 185, 138, 0, 0, 0, 0, 113, 93, 78, 4, 0, 0, 0, 0, 0, 95, 96, 101, 0, 0, 0, 0, 185, 4, 36, 3, 249, 196, 0, 0, 0, 0, 96, 200, 180, 99, 0, 0, 0, 0, 132, 74, 42, 198, 0, 0, 0, 0, 145, 192, 47, 92, 0, 0, 0, 0, 185, 4, 252, 3, 230, 40, 0, 0, 0, 0, 130, 201, 92, 124, 0, 0, 0, 0, 247, 171, 184, 160, 0, 0, 0, 0, 137, 152, 36, 219, 0, 0, 0, 0, 185, 4, 218, 74, 214, 176, 0, 0, 0, 0, 99, 217, 43, 140, 0, 0, 0, 0, 22, 200, 70, 66, 0, 0, 0, 0, 79, 91, 213, 184, 0, 0, 0, 0, 185, 4, 243, 215, 194, 73, 0, 0, 0, 0, 59, 196, 68, 254, 0, 0, 0, 0, 225, 22, 138, 124, 0, 0, 0, 0, 43, 31, 245, 234, 0, 0, 0, 0, 185, 4, 72, 222, 145, 12, 0, 0, 0, 0, 95, 123, 148, 209, 0, 0, 0, 0, 18, 95, 216, 110, 0, 0, 0, 0, 242, 162, 78, 53, 0, 0, 0, 0, 185, 4, 94, 220, 150, 65, 0, 0, 0, 0, 1, 254, 61, 78, 0, 0, 0, 0, 4, 71, 166, 3, 0, 0, 0, 0, 86, 60, 45, 225, 0, 0, 0, 0, 185, 4, 173, 53, 152, 206, 0, 0, 0, 0, 117, 233, 127, 128, 0, 0, 0, 0, 20, 63, 60, 25, 0, 0, 0, 0, 85, 177, 167, 213, 0, 0, 0, 0, 185, 4, 82, 213, 226, 197, 0, 0, 0, 0, 15, 171, 181, 59, 0, 0, 0, 0, 115, 243, 226, 56, 0, 0, 0, 0, 93, 92, 152, 41, 0, 0, 0, 0, 185, 4, 58, 175, 252, 202, 0, 0, 0, 0, 238, 67, 220, 251, 0, 0, 0, 0, 205, 34, 185, 223, 0, 0, 0, 0, 148, 89, 236, 207, 0, 0, 0, 0, 185, 4, 214, 209, 155, 234, 0, 0, 0, 0, 173, 10, 99, 247, 0, 0, 0, 0, 93, 202, 10, 185, 0, 0, 0, 0, 83, 225, 54, 51, 0, 0, 0, 0, 185, 4, 196, 123, 117, 201, 0, 0, 0, 0, 228, 231, 195, 53, 0, 0, 0, 0, 119, 153, 0, 246, 0, 0, 0, 0, 32, 125, 171, 63, 0, 0, 0, 0, 185, 4, 3, 3, 102, 31, 0, 0, 0, 0, 141, 160, 183, 140, 0, 0, 0, 0, 170, 106, 186, 64, 0, 0, 0, 0, 245, 66, 240, 123, 0, 0, 0, 0, 185, 4, 11, 154, 219, 162, 0, 0, 0, 0, 226, 255, 172, 61, 0, 0, 0, 0, 155, 47, 90, 116, 0, 0, 0, 0, 64, 69, 236, 228, 0, 0, 0, 0, 185, 4, 134, 123, 115, 45, 0, 0, 0, 0, 246, 109, 239, 100, 0, 0, 0, 0, 228, 174, 197, 23, 0, 0, 0, 0, 167, 89, 111, 59, 0, 0, 0, 0, 185, 4, 209, 224, 159, 38, 0, 0, 0, 0, 113, 132, 130, 172, 0, 0, 0, 0, 234, 86, 113, 49, 0, 0, 0, 0, 38, 87, 93, 226, 0, 0, 0, 0, 185, 4, 180, 223, 3, 190, 0, 0, 0, 0, 53, 21, 230, 46, 0, 0, 0, 0, 88, 72, 158, 192, 0, 0, 0, 0, 54, 20, 115, 220, 0, 0, 0, 0, 185, 4, 87, 12, 230, 77, 0, 0, 0, 0, 204, 39, 138, 5, 0, 0, 0, 0, 208, 211, 61, 105, 0, 0, 0, 0, 140, 227, 29, 253, 0, 0, 0, 0, 185, 4, 66, 195, 95, 101, 0, 0, 0, 0, 251, 123, 101, 241, 0, 0, 0, 0, 233, 163, 239, 62, 0, 0, 0, 0, 109, 89, 227, 187, 0, 0, 0, 0, 185, 4, 26, 38, 6, 155, 0, 0, 0, 0, 255, 67, 48, 220, 0, 0, 0, 0, 231, 239, 55, 134, 0, 0, 0, 0, 232, 162, 198, 6, 0, 0, 0, 0, 185, 4, 93, 241, 221, 157, 0, 0, 0, 0, 190, 247, 212, 47, 0, 0, 0, 0, 210, 205, 236, 109, 0, 0, 0, 0, 101, 191, 44, 67, 0, 0, 0, 0, 185, 4, 219, 114, 98, 51, 0, 0, 0, 0, 22, 210, 241, 30, 0, 0, 0, 0, 7, 43, 179, 147, 0, 0, 0, 0, 194, 60, 153, 75, 0, 0, 0, 0, 185, 4, 203, 3, 107, 144, 0, 0, 0, 0, 28, 184, 35, 74, 0, 0, 0, 0, 6, 172, 53, 51, 0, 0, 0, 0, 59, 184, 106, 66, 0, 0, 0, 0, 185, 4, 61, 60, 20, 193, 0, 0, 0, 0, 212, 174, 74, 23, 0, 0, 0, 0, 223, 181, 96, 183, 0, 0, 0, 0, 254, 178, 34, 69, 0, 0, 0, 0, 185, 4, 124, 8, 181, 118, 0, 0, 0, 0, 39, 192, 137, 229, 0, 0, 0, 0, 99, 72, 125, 253, 0, 0, 0, 0, 145, 149, 168, 44, 0, 0, 0, 0, 185, 4, 135, 181, 148, 245, 0, 0, 0, 0, 157, 156, 108, 12, 0, 0, 0, 0, 84, 9, 23, 195, 0, 0, 0, 0, 53, 204, 212, 142, 0, 0, 0, 0, 185, 4, 222, 153, 160, 138, 0, 0, 0, 0, 97, 41, 180, 83, 0, 0, 0, 0, 63, 50, 9, 61, 0, 0, 0, 0, 193, 26, 125, 104, 0, 0, 0, 0, 185, 4, 64, 27, 207, 53, 0, 0, 0, 0, 67, 66, 97, 88, 0, 0, 0, 0, 241, 44, 47, 4, 0, 0, 0, 0, 76, 204, 206, 42, 0, 0, 0, 0, 185, 4, 123, 255, 91, 6, 0, 0, 0, 0, 247, 11, 127, 29, 0, 0, 0, 0, 25, 146, 30, 140, 0, 0, 0, 0, 206, 125, 159, 130, 0, 0, 0, 0, 185, 4, 53, 1, 169, 22, 0, 0, 0, 0, 130, 186, 238, 212, 0, 0, 0, 0, 76, 211, 91, 231, 0, 0, 0, 0, 225, 200, 141, 243, 0, 0, 0, 0, 185, 4, 119, 197, 131, 210, 0, 0, 0, 0, 237, 132, 157, 7, 0, 0, 0, 0, 170, 241, 241, 180, 0, 0, 0, 0, 214, 119, 211, 119, 0, 0, 0, 0, 185, 4, 236, 114, 77, 21, 0, 0, 0, 0, 144, 243, 122, 143, 0, 0, 0, 0, 181, 7, 182, 68, 0, 0, 0, 0, 62, 34, 98, 31, 0, 0, 0, 0, 185, 4, 165, 171, 164, 146, 0, 0, 0, 0, 82, 161, 7, 247, 0, 0, 0, 0, 176, 131, 41, 119, 0, 0, 0, 0, 72, 103, 213, 2, 0, 0, 0, 0, 185, 4, 242, 242, 152, 110, 0, 0, 0, 0, 160, 176, 156, 66, 0, 0, 0, 0, 71, 90, 101, 208, 0, 0, 0, 0, 119, 251, 236, 215, 0, 0, 0, 0, 185, 4, 250, 19, 33, 178, 0, 0, 0, 0, 219, 164, 233, 92, 0, 0, 0, 0, 241, 55, 24, 237, 0, 0, 0, 0, 142, 115, 79, 33, 0, 0, 0, 0, 185, 4, 195, 226, 186, 88, 0, 0, 0, 0, 168, 36, 120, 107, 0, 0, 0, 0, 125, 92, 186, 52, 0, 0, 0, 0, 61, 237, 209, 95, 0, 0, 0, 0, 185, 4, 43, 146, 131, 80, 0, 0, 0, 0, 232, 202, 193, 245, 0, 0, 0, 0, 46, 85, 70, 133, 0, 0, 0, 0, 192, 56, 250, 37, 0, 0, 0, 0, 185, 4, 91, 23, 114, 164, 0, 0, 0, 0, 90, 96, 236, 212, 0, 0, 0, 0, 158, 241, 125, 103, 0, 0, 0, 0, 3, 254, 55, 86, 0, 0, 0, 0, 185, 4, 140, 151, 94, 148, 0, 0, 0, 0, 124, 177, 19, 144, 0, 0, 0, 0, 244, 44, 174, 150, 0, 0, 0, 0, 132, 218, 192, 133, 0, 0, 0, 0, 185, 4, 208, 82, 175, 249, 0, 0, 0, 0, 7, 239, 169, 82, 0, 0, 0, 0, 117, 193, 38, 120, 0, 0, 0, 0, 43, 25, 104, 184, 0, 0, 0, 0, 185, 4, 189, 111, 90, 198, 0, 0, 0, 0, 171, 26, 88, 3, 0, 0, 0, 0, 150, 83, 254, 92, 0, 0, 0, 0, 229, 247, 111, 130, 0, 0, 0, 0, 185, 4, 179, 157, 177, 149, 0, 0, 0, 0, 157, 184, 185, 40, 0, 0, 0, 0, 254, 249, 47, 31, 0, 0, 0, 0, 102, 238, 252, 21, 0, 0, 0, 0, 185, 4, 172, 198, 24, 156, 0, 0, 0, 0, 16, 183, 210, 239, 0, 0, 0, 0, 109, 170, 193, 67, 0, 0, 0, 0, 25, 116, 128, 120, 0, 0, 0, 0, 185, 4, 57, 22, 44, 156, 0, 0, 0, 0, 203, 182, 121, 9, 0, 0, 0, 0, 244, 241, 8, 83, 0, 0, 0, 0, 250, 223, 118, 112, 0, 0, 0, 0, 185, 4, 116, 234, 28, 213, 0, 0, 0, 0, 122, 125, 181, 136, 0, 0, 0, 0, 144, 210, 254, 51, 0, 0, 0, 0, 104, 223, 92, 39, 0, 0, 0, 0, 185, 4, 198, 201, 104, 22, 0, 0, 0, 0, 241, 144, 147, 249, 0, 0, 0, 0, 232, 100, 14, 138, 0, 0, 0, 0, 44, 207, 189, 245, 0, 0, 0, 0, 185, 4, 29, 33, 66, 127, 0, 0, 0, 0, 175, 50, 227, 212, 0, 0, 0, 0, 182, 191, 76, 223, 0, 0, 0, 0, 82, 29, 24, 100, 0, 0, 0, 0, 185, 4, 133, 38, 134, 76, 0, 0, 0, 0, 186, 26, 143, 89, 0, 0, 0, 0, 83, 147, 35, 27, 0, 0, 0, 0, 57, 76, 250, 205, 0, 0, 0, 0, 185, 4, 206, 23, 51, 16, 0, 0, 0, 0, 43, 170, 226, 39, 0, 0, 0, 0, 33, 72, 166, 88, 0, 0, 0, 0, 60, 102, 233, 90, 0, 0, 0, 0, 185, 4, 232, 144, 255, 31, 0, 0, 0, 0, 68, 181, 124, 212, 0, 0, 0, 0, 97, 158, 234, 96, 0, 0, 0, 0, 201, 8, 245, 227, 0, 0, 0, 0, 185, 4, 235, 127, 4, 99, 0, 0, 0, 0, 133, 26, 25, 222, 0, 0, 0, 0, 150, 113, 171, 97, 0, 0, 0, 0, 251, 26, 142, 13, 0, 0, 0, 0, 185, 4, 154, 153, 220, 170, 0, 0, 0, 0, 99, 123, 101, 175, 0, 0, 0, 0, 41, 127, 203, 181, 0, 0, 0, 0, 82, 52, 154, 37, 0, 0, 0, 0, 185, 4, 107, 0, 234, 196, 0, 0, 0, 0, 167, 8, 200, 198, 0, 0, 0, 0, 171, 246, 103, 186, 0, 0, 0, 0, 104, 245, 129, 45, 0, 0, 0, 0, 185, 4, 170, 104, 55, 129, 0, 0, 0, 0, 24, 40, 141, 200, 0, 0, 0, 0, 116, 201, 106, 53, 0, 0, 0, 0, 99, 215, 87, 72, 0, 0, 0, 0, 185, 4, 201, 158, 113, 50, 0, 0, 0, 0, 32, 201, 40, 62, 0, 0, 0, 0, 113, 47, 82, 3, 0, 0, 0, 0, 189, 255, 116, 76, 0, 0, 0, 0, 185, 4, 52, 61, 237, 86, 0, 0, 0, 0, 254, 0, 28, 194, 0, 0, 0, 0, 156, 5, 241, 9, 0, 0, 0, 0, 118, 166, 227, 143, 0, 0, 0, 0, 185, 4, 247, 225, 61, 220, 0, 0, 0, 0, 201, 237, 186, 221, 0, 0, 0, 0, 169, 246, 162, 49, 0, 0, 0, 0, 47, 209, 21, 24, 0, 0, 0, 0, 185, 4, 224, 201, 126, 225, 0, 0, 0, 0, 225, 68, 125, 159, 0, 0, 0, 0, 93, 217, 100, 200, 0, 0, 0, 0, 62, 106, 251, 221, 0, 0, 0, 0, 185, 4, 109, 182, 178, 254, 0, 0, 0, 0, 196, 195, 245, 238, 0, 0, 0, 0, 149, 41, 80, 8, 0, 0, 0, 0, 166, 20, 144, 138, 0, 0, 0, 0, 185, 4, 234, 143, 212, 250, 0, 0, 0, 0, 172, 221, 14, 167, 0, 0, 0, 0, 124, 186, 37, 41, 0, 0, 0, 0, 144, 212, 236, 229, 0, 0, 0, 0, 185, 4, 25, 120, 238, 86, 0, 0, 0, 0, 241, 12, 246, 215, 0, 0, 0, 0, 239, 155, 187, 240, 0, 0, 0, 0, 7, 123, 226, 205, 0, 0, 0, 0, 185, 4, 53, 16, 91, 194, 0, 0, 0, 0, 102, 93, 40, 30, 0, 0, 0, 0, 78, 243, 131, 57, 0, 0, 0, 0, 138, 29, 188, 95, 0, 0, 0, 0, 185, 4, 188, 193, 120, 63, 0, 0, 0, 0, 159, 22, 208, 232, 0, 0, 0, 0, 137, 227, 129, 170, 0, 0, 0, 0, 63, 250, 42, 16, 0, 0, 0, 0, 185, 4, 218, 200, 186, 213, 0, 0, 0, 0, 149, 29, 121, 108, 0, 0, 0, 0, 54, 115, 177, 124, 0, 0, 0, 0, 35, 222, 103, 40, 0, 0, 0, 0, 185, 4, 199, 221, 3, 21, 0, 0, 0, 0, 180, 201, 84, 235, 0, 0, 0, 0, 77, 237, 137, 5, 0, 0, 0, 0, 128, 87, 163, 4, 0, 0, 0, 0, 185, 4, 194, 38, 144, 128, 0, 0, 0, 0, 74, 140, 174, 86, 0, 0, 0, 0, 93, 14, 109, 133, 0, 0, 0, 0, 131, 13, 22, 88, 0, 0, 0, 0, 185, 4, 60, 212, 209, 69, 0, 0, 0, 0, 222, 88, 164, 11, 0, 0, 0, 0, 221, 74, 117, 241, 0, 0, 0, 0, 100, 123, 48, 56, 0, 0, 0, 0, 185, 4, 84, 155, 78, 219, 0, 0, 0, 0, 78, 41, 125, 106, 0, 0, 0, 0, 60, 164, 127, 198, 0, 0, 0, 0, 1, 160, 55, 96, 0, 0, 0, 0, 185, 4, 116, 18, 49, 61, 0, 0, 0, 0, 144, 210, 168, 41, 0, 0, 0, 0, 176, 116, 156, 211, 0, 0, 0, 0, 190, 115, 196, 70, 0, 0, 0, 0, 185, 4, 150, 116, 56, 170, 0, 0, 0, 0, 62, 14, 220, 78, 0, 0, 0, 0, 183, 202, 173, 219, 0, 0, 0, 0, 204, 4, 123, 120, 0, 0, 0, 0, 185, 4, 88, 97, 247, 245, 0, 0, 0, 0, 113, 123, 110, 145, 0, 0, 0, 0, 192, 209, 214, 48, 0, 0, 0, 0, 177, 191, 55, 164, 0, 0, 0, 0, 185, 4, 233, 10, 113, 11, 0, 0, 0, 0, 189, 27, 142, 128, 0, 0, 0, 0, 244, 252, 125, 146, 0, 0, 0, 0, 147, 177, 251, 86, 0, 0, 0, 0, 185, 4, 77, 132, 206, 245, 0, 0, 0, 0, 20, 7, 246, 14, 0, 0, 0, 0, 232, 151, 87, 204, 0, 0, 0, 0, 200, 136, 239, 43, 0, 0, 0, 0, 185, 4, 205, 7, 36, 224, 0, 0, 0, 0, 202, 216, 49, 223, 0, 0, 0, 0, 139, 56, 173, 177, 0, 0, 0, 0, 170, 109, 222, 216, 0, 0, 0, 0, 185, 4, 177, 58, 55, 81, 0, 0, 0, 0, 27, 38, 153, 146, 0, 0, 0, 0, 165, 108, 146, 165, 0, 0, 0, 0, 195, 47, 174, 248, 0, 0, 0, 0, 185, 4, 112, 61, 47, 213, 0, 0, 0, 0, 30, 228, 195, 99, 0, 0, 0, 0, 51, 28, 219, 209, 0, 0, 0, 0, 96, 115, 93, 112, 0, 0, 0, 0, 185, 4, 219, 22, 70, 241, 0, 0, 0, 0, 100, 230, 255, 96, 0, 0, 0, 0, 211, 253, 40, 137, 0, 0, 0, 0, 10, 238, 20, 99, 0, 0, 0, 0, 185, 4, 142, 245, 3, 194, 0, 0, 0, 0, 255, 166, 139, 219, 0, 0, 0, 0, 149, 253, 70, 132, 0, 0, 0, 0, 216, 46, 218, 8, 0, 0, 0, 0, 185, 4, 90, 240, 254, 97, 0, 0, 0, 0, 164, 197, 165, 87, 0, 0, 0, 0, 214, 18, 156, 213, 0, 0, 0, 0, 193, 120, 49, 63, 0, 0, 0, 0, 185, 4, 51, 94, 186, 20, 0, 0, 0, 0, 251, 34, 187, 121, 0, 0, 0, 0, 245, 151, 136, 200, 0, 0, 0, 0, 3, 75, 255, 170, 0, 0, 0, 0, 185, 4, 20, 136, 104, 208, 0, 0, 0, 0, 17, 116, 49, 0, 0, 0, 0, 0, 173, 34, 81, 122, 0, 0, 0, 0, 61, 68, 128, 72, 0, 0, 0, 0, 185, 4, 177, 3, 11, 155, 0, 0, 0, 0, 130, 35, 231, 108, 0, 0, 0, 0, 233, 206, 85, 224, 0, 0, 0, 0, 231, 38, 222, 201, 0, 0, 0, 0, 185, 4, 155, 175, 60, 236, 0, 0, 0, 0, 220, 135, 40, 167, 0, 0, 0, 0, 51, 194, 93, 17, 0, 0, 0, 0, 171, 128, 84, 80, 0, 0, 0, 0, 185, 4, 167, 161, 222, 233, 0, 0, 0, 0, 180, 202, 125, 225, 0, 0, 0, 0, 55, 115, 124, 118, 0, 0, 0, 0, 235, 189, 167, 126, 0, 0, 0, 0, 185, 4, 183, 124, 12, 238, 0, 0, 0, 0, 183, 136, 247, 67, 0, 0, 0, 0, 174, 9, 111, 115, 0, 0, 0, 0, 63, 236, 0, 112, 0, 0, 0, 0, 185, 4, 59, 187, 151, 56, 0, 0, 0, 0, 52, 240, 217, 217, 0, 0, 0, 0, 107, 159, 210, 134, 0, 0, 0, 0, 0, 184, 217, 215, 0, 0, 0, 0, 185, 4, 193, 63, 52, 14, 0, 0, 0, 0, 11, 238, 127, 94, 0, 0, 0, 0, 96, 244, 192, 24, 0, 0, 0, 0, 178, 92, 43, 173, 0, 0, 0, 0, 185, 4, 120, 106, 18, 79, 0, 0, 0, 0, 158, 11, 173, 164, 0, 0, 0, 0, 28, 139, 202, 163, 0, 0, 0, 0, 27, 213, 65, 250, 0, 0, 0, 0, 185, 4, 23, 174, 28, 157, 0, 0, 0, 0, 29, 255, 150, 254, 0, 0, 0, 0, 133, 14, 234, 62, 0, 0, 0, 0, 139, 43, 240, 163, 0, 0, 0, 0, 185, 4, 33, 236, 206, 252, 0, 0, 0, 0, 130, 152, 199, 190, 0, 0, 0, 0, 100, 81, 124, 101, 0, 0, 0, 0, 178, 221, 78, 125, 0, 0, 0, 0, 185, 4, 228, 86, 200, 186, 0, 0, 0, 0, 56, 229, 199, 146, 0, 0, 0, 0, 158, 112, 104, 150, 0, 0, 0, 0, 97, 232, 150, 98, 0, 0, 0, 0, 185, 4, 18, 121, 96, 133, 0, 0, 0, 0, 32, 191, 64, 223, 0, 0, 0, 0, 61, 79, 134, 113, 0, 0, 0, 0, 172, 29, 166, 211, 0, 0, 0, 0, 185, 4, 21, 151, 248, 145, 0, 0, 0, 0, 146, 71, 59, 195, 0, 0, 0, 0, 229, 14, 236, 108, 0, 0, 0, 0, 144, 164, 203, 99, 0, 0, 0, 0, 185, 4, 232, 42, 162, 195, 0, 0, 0, 0, 106, 81, 78, 42, 0, 0, 0, 0, 70, 0, 248, 90, 0, 0, 0, 0, 202, 185, 173, 185, 0, 0, 0, 0, 185, 4, 146, 168, 134, 233, 0, 0, 0, 0, 183, 225, 136, 159, 0, 0, 0, 0, 75, 63, 207, 77, 0, 0, 0, 0, 82, 17, 120, 154, 0, 0, 0, 0, 185, 4, 125, 151, 199, 132, 0, 0, 0, 0, 43, 124, 218, 59, 0, 0, 0, 0, 149, 165, 161, 195, 0, 0, 0, 0, 9, 37, 111, 160, 0, 0, 0, 0, 185, 4, 109, 132, 181, 5, 0, 0, 0, 0, 173, 17, 250, 247, 0, 0, 0, 0, 78, 191, 4, 65, 0, 0, 0, 0, 65, 4, 242, 64, 0, 0, 0, 0, 185, 4, 115, 69, 38, 235, 0, 0, 0, 0, 53, 94, 139, 2, 0, 0, 0, 0, 126, 96, 87, 95, 0, 0, 0, 0, 214, 173, 118, 125, 0, 0, 0, 0, 185, 4, 52, 7, 179, 143, 0, 0, 0, 0, 224, 109, 3, 241, 0, 0, 0, 0, 110, 135, 135, 83, 0, 0, 0, 0, 101, 197, 60, 34, 0, 0, 0, 0, 185, 4, 56, 140, 109, 163, 0, 0, 0, 0, 236, 193, 81, 215, 0, 0, 0, 0, 249, 12, 70, 151, 0, 0, 0, 0, 230, 194, 14, 123, 0, 0, 0, 0, 185, 4, 47, 161, 150, 140, 0, 0, 0, 0, 135, 31, 200, 230, 0, 0, 0, 0, 118, 217, 202, 240, 0, 0, 0, 0, 191, 11, 212, 67, 0, 0, 0, 0, 185, 4, 248, 56, 110, 209, 0, 0, 0, 0, 200, 224, 131, 120, 0, 0, 0, 0, 161, 170, 43, 18, 0, 0, 0, 0, 25, 163, 169, 238, 0, 0, 0, 0, 185, 4, 191, 125, 148, 209, 0, 0, 0, 0, 35, 105, 203, 75, 0, 0, 0, 0, 180, 189, 144, 148, 0, 0, 0, 0, 146, 177, 157, 49, 0, 0, 0, 0, 185, 4, 150, 173, 78, 213, 0, 0, 0, 0, 77, 12, 3, 208, 0, 0, 0, 0, 170, 237, 11, 227, 0, 0, 0, 0, 149, 58, 235, 88, 0, 0, 0, 0, 185, 4, 154, 220, 202, 7, 0, 0, 0, 0, 96, 221, 0, 242, 0, 0, 0, 0, 189, 118, 60, 112, 0, 0, 0, 0, 194, 183, 104, 188, 0, 0, 0, 0, 185, 4, 45, 238, 247, 129, 0, 0, 0, 0, 40, 94, 240, 138, 0, 0, 0, 0, 123, 14, 88, 61, 0, 0, 0, 0, 54, 186, 243, 2, 0, 0, 0, 0, 185, 4, 67, 46, 222, 186, 0, 0, 0, 0, 130, 2, 45, 47, 0, 0, 0, 0, 198, 49, 42, 188, 0, 0, 0, 0, 185, 70, 74, 18, 0, 0, 0, 0, 185, 4, 150, 242, 132, 217, 0, 0, 0, 0, 197, 244, 227, 70, 0, 0, 0, 0, 15, 42, 164, 101, 0, 0, 0, 0, 209, 64, 10, 134, 0, 0, 0, 0, 185, 4, 203, 155, 201, 37, 0, 0, 0, 0, 206, 191, 230, 38, 0, 0, 0, 0, 239, 207, 80, 180, 0, 0, 0, 0, 65, 97, 24, 15, 0, 0, 0, 0, 185, 4, 250, 209, 145, 155, 0, 0, 0, 0, 111, 12, 56, 10, 0, 0, 0, 0, 129, 223, 63, 246, 0, 0, 0, 0, 55, 117, 137, 132, 0, 0, 0, 0, 185, 4, 176, 128, 89, 70, 0, 0, 0, 0, 51, 86, 164, 242, 0, 0, 0, 0, 109, 118, 191, 60, 0, 0, 0, 0, 223, 242, 10, 66, 0, 0, 0, 0, 185, 4, 180, 183, 200, 14, 0, 0, 0, 0, 174, 3, 151, 116, 0, 0, 0, 0, 44, 166, 88, 88, 0, 0, 0, 0, 229, 203, 11, 12, 0, 0, 0, 0, 185, 4, 92, 169, 173, 95, 0, 0, 0, 0, 119, 61, 228, 207, 0, 0, 0, 0, 50, 196, 222, 218, 0, 0, 0, 0, 219, 9, 152, 72, 0, 0, 0, 0, 185, 4, 126, 240, 93, 50, 0, 0, 0, 0, 200, 7, 239, 90, 0, 0, 0, 0, 83, 134, 186, 147, 0, 0, 0, 0, 172, 143, 200, 171, 0, 0, 0, 0, 185, 4, 162, 125, 14, 144, 0, 0, 0, 0, 188, 227, 62, 50, 0, 0, 0, 0, 146, 40, 30, 113, 0, 0, 0, 0, 23, 88, 12, 20, 0, 0, 0, 0, 185, 4, 54, 55, 155, 164, 0, 0, 0, 0, 229, 185, 24, 149, 0, 0, 0, 0, 186, 127, 180, 84, 0, 0, 0, 0, 135, 106, 62, 32, 0, 0, 0, 0, 185, 4, 244, 93, 126, 112, 0, 0, 0, 0, 83, 82, 238, 209, 0, 0, 0, 0, 137, 237, 201, 160, 0, 0, 0, 0, 105, 151, 144, 159, 0, 0, 0, 0, 185, 4, 48, 196, 39, 248, 0, 0, 0, 0, 167, 68, 56, 152, 0, 0, 0, 0, 1, 4, 185, 37, 0, 0, 0, 0, 53, 47, 12, 87, 0, 0, 0, 0, 185, 4, 178, 61, 55, 93, 0, 0, 0, 0, 2, 169, 204, 146, 0, 0, 0, 0, 99, 244, 144, 74, 0, 0, 0, 0, 6, 6, 19, 114, 0, 0, 0, 0, 185, 4, 146, 96, 49, 112, 0, 0, 0, 0, 76, 38, 142, 135, 0, 0, 0, 0, 25, 58, 168, 3, 0, 0, 0, 0, 165, 254, 214, 5, 0, 0, 0, 0, 185, 4, 75, 239, 212, 223, 0, 0, 0, 0, 216, 110, 49, 65, 0, 0, 0, 0, 214, 198, 216, 168, 0, 0, 0, 0, 8, 51, 98, 161, 0, 0, 0, 0, 185, 4, 101, 134, 151, 189, 0, 0, 0, 0, 53, 172, 180, 166, 0, 0, 0, 0, 17, 75, 65, 23, 0, 0, 0, 0, 5, 59, 217, 193, 0, 0, 0, 0, 185, 4, 20, 214, 192, 48, 0, 0, 0, 0, 178, 98, 54, 174, 0, 0, 0, 0, 129, 130, 42, 117, 0, 0, 0, 0, 246, 228, 228, 182, 0, 0, 0, 0, 185, 4, 122, 171, 14, 8, 0, 0, 0, 0, 82, 187, 40, 24, 0, 0, 0, 0, 33, 177, 90, 7, 0, 0, 0, 0, 41, 56, 121, 228, 0, 0, 0, 0, 185, 4, 219, 9, 123, 106, 0, 0, 0, 0, 69, 11, 41, 186, 0, 0, 0, 0, 149, 238, 239, 246, 0, 0, 0, 0, 171, 163, 62, 119, 0, 0, 0, 0, 185, 4, 176, 250, 127, 57, 0, 0, 0, 0, 112, 30, 190, 147, 0, 0, 0, 0, 216, 83, 254, 10, 0, 0, 0, 0, 181, 61, 198, 181, 0, 0, 0, 0, 185, 4, 7, 54, 109, 92, 0, 0, 0, 0, 93, 123, 41, 245, 0, 0, 0, 0, 155, 47, 29, 246, 0, 0, 0, 0, 101, 236, 173, 205, 0, 0, 0, 0, 185, 4, 252, 109, 233, 101, 0, 0, 0, 0, 151, 166, 44, 186, 0, 0, 0, 0, 128, 24, 161, 75, 0, 0, 0, 0, 115, 157, 50, 166, 0, 0, 0, 0, 185, 4, 179, 228, 151, 191, 0, 0, 0, 0, 172, 45, 194, 134, 0, 0, 0, 0, 219, 70, 144, 25, 0, 0, 0, 0, 215, 115, 178, 78, 0, 0, 0, 0, 185, 4, 176, 104, 132, 184, 0, 0, 0, 0, 91, 133, 9, 142, 0, 0, 0, 0, 223, 176, 103, 224, 0, 0, 0, 0, 217, 102, 99, 121, 0, 0, 0, 0, 185, 4, 188, 245, 107, 18, 0, 0, 0, 0, 222, 244, 59, 72, 0, 0, 0, 0, 110, 150, 93, 136, 0, 0, 0, 0, 121, 249, 176, 132, 0, 0, 0, 0, 185, 4, 177, 24, 8, 123, 0, 0, 0, 0, 227, 143, 113, 224, 0, 0, 0, 0, 250, 5, 210, 195, 0, 0, 0, 0, 98, 177, 217, 64, 0, 0, 0, 0, 185, 4, 114, 152, 1, 132, 0, 0, 0, 0, 148, 134, 237, 240, 0, 0, 0, 0, 7, 203, 206, 111, 0, 0, 0, 0, 56, 238, 228, 56, 0, 0, 0, 0, 185, 4, 187, 7, 185, 132, 0, 0, 0, 0, 8, 182, 7, 70, 0, 0, 0, 0, 253, 174, 92, 92, 0, 0, 0, 0, 206, 225, 217, 172, 0, 0, 0, 0, 185, 4, 224, 84, 9, 253, 0, 0, 0, 0, 72, 239, 37, 78, 0, 0, 0, 0, 234, 251, 48, 133, 0, 0, 0, 0, 83, 165, 209, 41, 0, 0, 0, 0, 185, 4, 38, 174, 214, 147, 0, 0, 0, 0, 13, 71, 10, 72, 0, 0, 0, 0, 73, 76, 228, 10, 0, 0, 0, 0, 211, 143, 191, 146, 0, 0, 0, 0, 185, 4, 129, 14, 75, 124, 0, 0, 0, 0, 249, 161, 198, 92, 0, 0, 0, 0, 193, 84, 148, 99, 0, 0, 0, 0, 163, 194, 160, 175, 0, 0, 0, 0, 185, 4, 170, 10, 71, 245, 0, 0, 0, 0, 152, 196, 210, 121, 0, 0, 0, 0, 113, 113, 26, 183, 0, 0, 0, 0, 179, 102, 70, 26, 0, 0, 0, 0, 185, 4, 207, 149, 209, 237, 0, 0, 0, 0, 53, 248, 93, 224, 0, 0, 0, 0, 45, 49, 36, 85, 0, 0, 0, 0, 103, 44, 176, 35, 0, 0, 0, 0, 185, 4, 51, 109, 54, 110, 0, 0, 0, 0, 53, 231, 142, 76, 0, 0, 0, 0, 217, 37, 89, 145, 0, 0, 0, 0, 67, 160, 131, 103, 0, 0, 0, 0, 185, 4, 248, 161, 165, 33, 0, 0, 0, 0, 205, 14, 106, 184, 0, 0, 0, 0, 166, 220, 105, 101, 0, 0, 0, 0, 190, 166, 78, 30, 0, 0, 0, 0, 185, 4, 195, 185, 54, 32, 0, 0, 0, 0, 68, 236, 153, 14, 0, 0, 0, 0, 7, 156, 243, 123, 0, 0, 0, 0, 74, 34, 169, 9, 0, 0, 0, 0, 185, 4, 205, 91, 230, 245, 0, 0, 0, 0, 18, 31, 225, 16, 0, 0, 0, 0, 102, 73, 61, 137, 0, 0, 0, 0, 205, 192, 125, 207, 0, 0, 0, 0, 185, 4, 136, 226, 226, 91, 0, 0, 0, 0, 56, 2, 123, 87, 0, 0, 0, 0, 59, 222, 248, 243, 0, 0, 0, 0, 63, 96, 140, 89, 0, 0, 0, 0, 185, 4, 170, 33, 79, 204, 0, 0, 0, 0, 98, 195, 243, 128, 0, 0, 0, 0, 206, 36, 14, 151, 0, 0, 0, 0, 156, 29, 240, 39, 0, 0, 0, 0, 185, 4, 134, 25, 107, 77, 0, 0, 0, 0, 225, 204, 199, 245, 0, 0, 0, 0, 184, 146, 64, 122, 0, 0, 0, 0, 152, 71, 164, 78, 0, 0, 0, 0, 185, 4, 77, 1, 140, 156, 0, 0, 0, 0, 189, 208, 164, 179, 0, 0, 0, 0, 60, 233, 106, 238, 0, 0, 0, 0, 29, 89, 39, 222, 0, 0, 0, 0, 185, 4, 161, 158, 190, 129, 0, 0, 0, 0, 141, 12, 143, 98, 0, 0, 0, 0, 69, 63, 253, 91, 0, 0, 0, 0, 43, 24, 243, 124, 0, 0, 0, 0, 185, 4, 48, 69, 88, 85, 0, 0, 0, 0, 23, 107, 65, 88, 0, 0, 0, 0, 203, 7, 157, 55, 0, 0, 0, 0, 114, 23, 219, 29, 0, 0, 0, 0, 185, 4, 201, 210, 135, 178, 0, 0, 0, 0, 208, 234, 142, 151, 0, 0, 0, 0, 36, 99, 182, 176, 0, 0, 0, 0, 45, 132, 171, 72, 0, 0, 0, 0, 185, 4, 229, 218, 244, 215, 0, 0, 0, 0, 1, 216, 62, 187, 0, 0, 0, 0, 166, 162, 55, 66, 0, 0, 0, 0, 182, 50, 214, 107, 0, 0, 0, 0, 185, 4, 202, 30, 42, 82, 0, 0, 0, 0, 140, 212, 207, 149, 0, 0, 0, 0, 111, 95, 242, 121, 0, 0, 0, 0, 140, 153, 38, 215, 0, 0, 0, 0, 185, 4, 235, 177, 19, 48, 0, 0, 0, 0, 235, 255, 112, 0, 0, 0, 0, 0, 62, 5, 44, 244, 0, 0, 0, 0, 20, 128, 82, 112, 0, 0, 0, 0, 185, 4, 219, 151, 119, 30, 0, 0, 0, 0, 83, 16, 181, 97, 0, 0, 0, 0, 145, 125, 113, 209, 0, 0, 0, 0, 56, 230, 180, 83, 0, 0, 0, 0, 185, 4, 154, 1, 158, 41, 0, 0, 0, 0, 4, 70, 74, 138, 0, 0, 0, 0, 255, 61, 144, 224, 0, 0, 0, 0, 129, 49, 202, 186, 0, 0, 0, 0, 185, 4, 122, 190, 99, 172, 0, 0, 0, 0, 53, 166, 199, 133, 0, 0, 0, 0, 73, 26, 160, 107, 0, 0, 0, 0, 130, 233, 76, 124, 0, 0, 0, 0, 185, 4, 14, 148, 158, 113, 0, 0, 0, 0, 241, 89, 179, 13, 0, 0, 0, 0, 65, 138, 122, 52, 0, 0, 0, 0, 117, 217, 114, 252, 0, 0, 0, 0, 185, 4, 130, 173, 156, 202, 0, 0, 0, 0, 160, 46, 254, 215, 0, 0, 0, 0, 226, 168, 108, 169, 0, 0, 0, 0, 226, 59, 190, 45, 0, 0, 0, 0, 185, 4, 159, 26, 195, 3, 0, 0, 0, 0, 176, 36, 200, 70, 0, 0, 0, 0, 110, 237, 64, 160, 0, 0, 0, 0, 128, 227, 54, 54, 0, 0, 0, 0, 185, 4, 171, 215, 31, 127, 0, 0, 0, 0, 64, 35, 30, 107, 0, 0, 0, 0, 158, 10, 41, 238, 0, 0, 0, 0, 61, 152, 118, 236, 0, 0, 0, 0, 185, 4, 222, 42, 26, 140, 0, 0, 0, 0, 204, 204, 12, 37, 0, 0, 0, 0, 189, 103, 78, 9, 0, 0, 0, 0, 55, 27, 45, 132, 0, 0, 0, 0, 185, 4, 22, 252, 147, 57, 0, 0, 0, 0, 255, 153, 40, 229, 0, 0, 0, 0, 177, 205, 124, 131, 0, 0, 0, 0, 152, 185, 12, 201, 0, 0, 0, 0, 185, 4, 78, 26, 226, 222, 0, 0, 0, 0, 131, 197, 26, 52, 0, 0, 0, 0, 174, 83, 197, 131, 0, 0, 0, 0, 184, 198, 6, 170, 0, 0, 0, 0, 185, 4, 68, 224, 109, 78, 0, 0, 0, 0, 245, 198, 143, 166, 0, 0, 0, 0, 103, 108, 194, 106, 0, 0, 0, 0, 255, 105, 165, 67, 0, 0, 0, 0, 185, 4, 17, 137, 69, 189, 0, 0, 0, 0, 139, 80, 34, 21, 0, 0, 0, 0, 155, 87, 178, 65, 0, 0, 0, 0, 40, 78, 135, 70, 0, 0, 0, 0, 185, 4, 195, 60, 111, 34, 0, 0, 0, 0, 131, 156, 174, 207, 0, 0, 0, 0, 130, 233, 85, 69, 0, 0, 0, 0, 115, 89, 217, 137, 0, 0, 0, 0, 185, 4, 226, 89, 189, 64, 0, 0, 0, 0, 129, 20, 85, 99, 0, 0, 0, 0, 185, 187, 18, 250, 0, 0, 0, 0, 79, 194, 63, 100, 0, 0, 0, 0, 185, 4, 46, 152, 103, 88, 0, 0, 0, 0, 157, 105, 101, 111, 0, 0, 0, 0, 57, 217, 179, 160, 0, 0, 0, 0, 238, 214, 52, 241, 0, 0, 0, 0, 185, 4, 97, 201, 88, 72, 0, 0, 0, 0, 230, 11, 153, 85, 0, 0, 0, 0, 180, 190, 15, 2, 0, 0, 0, 0, 57, 89, 172, 236, 0, 0, 0, 0, 185, 4, 201, 163, 232, 32, 0, 0, 0, 0, 38, 240, 106, 82, 0, 0, 0, 0, 159, 254, 46, 232, 0, 0, 0, 0, 141, 183, 144, 104, 0, 0, 0, 0, 185, 4, 53, 55, 148, 147, 0, 0, 0, 0, 146, 65, 156, 88, 0, 0, 0, 0, 78, 46, 79, 220, 0, 0, 0, 0, 115, 58, 203, 23, 0, 0, 0, 0, 185, 4, 130, 220, 165, 55, 0, 0, 0, 0, 193, 20, 36, 255, 0, 0, 0, 0, 11, 130, 186, 60, 0, 0, 0, 0, 162, 155, 125, 100, 0, 0, 0, 0, 185, 4, 158, 138, 8, 86, 0, 0, 0, 0, 102, 169, 52, 130, 0, 0, 0, 0, 200, 218, 254, 80, 0, 0, 0, 0, 157, 197, 179, 52, 0, 0, 0, 0, 185, 4, 89, 36, 45, 56, 0, 0, 0, 0, 91, 94, 142, 126, 0, 0, 0, 0, 203, 238, 144, 53, 0, 0, 0, 0, 223, 16, 56, 253, 0, 0, 0, 0, 185, 4, 236, 31, 216, 60, 0, 0, 0, 0, 141, 101, 36, 195, 0, 0, 0, 0, 91, 9, 188, 65, 0, 0, 0, 0, 84, 218, 82, 227, 0, 0, 0, 0, 185, 4, 80, 188, 116, 254, 0, 0, 0, 0, 207, 255, 251, 202, 0, 0, 0, 0, 88, 137, 206, 179, 0, 0, 0, 0, 145, 168, 39, 139, 0, 0, 0, 0, 185, 4, 180, 180, 68, 84, 0, 0, 0, 0, 119, 123, 114, 223, 0, 0, 0, 0, 65, 112, 23, 167, 0, 0, 0, 0, 97, 91, 191, 148, 0, 0, 0, 0, 185, 4, 5, 217, 211, 215, 0, 0, 0, 0, 241, 141, 51, 209, 0, 0, 0, 0, 140, 211, 154, 228, 0, 0, 0, 0, 50, 30, 243, 33, 0, 0, 0, 0, 185, 4, 92, 31, 10, 200, 0, 0, 0, 0, 248, 124, 140, 44, 0, 0, 0, 0, 107, 231, 51, 130, 0, 0, 0, 0, 152, 15, 86, 186, 0, 0, 0, 0, 185, 4, 43, 192, 33, 167, 0, 0, 0, 0, 114, 224, 19, 88, 0, 0, 0, 0, 76, 72, 88, 237, 0, 0, 0, 0, 191, 225, 187, 213, 0, 0, 0, 0, 185, 4, 3, 86, 198, 158, 0, 0, 0, 0, 127, 58, 130, 223, 0, 0, 0, 0, 51, 29, 199, 245, 0, 0, 0, 0, 54, 113, 151, 20, 0, 0, 0, 0, 185, 4, 159, 69, 118, 71, 0, 0, 0, 0, 151, 147, 24, 139, 0, 0, 0, 0, 93, 237, 18, 0, 0, 0, 0, 0, 193, 24, 196, 242, 0, 0, 0, 0, 185, 4, 114, 34, 187, 133, 0, 0, 0, 0, 77, 88, 0, 91, 0, 0, 0, 0, 201, 200, 154, 201, 0, 0, 0, 0, 230, 101, 90, 242, 0, 0, 0, 0, 185, 4, 9, 45, 56, 48, 0, 0, 0, 0, 228, 230, 9, 223, 0, 0, 0, 0, 194, 216, 76, 62, 0, 0, 0, 0, 45, 45, 10, 60, 0, 0, 0, 0, 185, 4, 51, 99, 113, 113, 0, 0, 0, 0, 220, 108, 253, 177, 0, 0, 0, 0, 24, 2, 205, 175, 0, 0, 0, 0, 14, 235, 213, 249, 0, 0, 0, 0, 185, 4, 122, 9, 174, 113, 0, 0, 0, 0, 6, 147, 106, 200, 0, 0, 0, 0, 75, 86, 202, 82, 0, 0, 0, 0, 197, 68, 94, 225, 0, 0, 0, 0, 185, 4, 233, 165, 216, 97, 0, 0, 0, 0, 45, 74, 16, 119, 0, 0, 0, 0, 141, 173, 239, 181, 0, 0, 0, 0, 12, 47, 125, 84, 0, 0, 0, 0, 185, 4, 171, 236, 51, 43, 0, 0, 0, 0, 193, 114, 23, 101, 0, 0, 0, 0, 155, 150, 205, 83, 0, 0, 0, 0, 51, 230, 163, 89, 0, 0, 0, 0, 185, 4, 26, 228, 202, 57, 0, 0, 0, 0, 235, 149, 53, 250, 0, 0, 0, 0, 90, 198, 241, 151, 0, 0, 0, 0, 91, 150, 126, 142, 0, 0, 0, 0, 185, 4, 36, 138, 77, 165, 0, 0, 0, 0, 85, 109, 247, 250, 0, 0, 0, 0, 225, 151, 70, 240, 0, 0, 0, 0, 140, 174, 119, 78, 0, 0, 0, 0, 185, 4, 198, 42, 54, 32, 0, 0, 0, 0, 178, 243, 185, 64, 0, 0, 0, 0, 2, 159, 248, 206, 0, 0, 0, 0, 169, 151, 43, 78, 0, 0, 0, 0, 185, 4, 152, 85, 74, 176, 0, 0, 0, 0, 106, 91, 104, 105, 0, 0, 0, 0, 211, 231, 145, 123, 0, 0, 0, 0, 113, 24, 72, 16, 0, 0, 0, 0, 185, 4, 177, 29, 166, 33, 0, 0, 0, 0, 46, 0, 5, 214, 0, 0, 0, 0, 207, 234, 160, 38, 0, 0, 0, 0, 33, 241, 28, 7, 0, 0, 0, 0, 185, 4, 246, 1, 39, 184, 0, 0, 0, 0, 151, 224, 150, 223, 0, 0, 0, 0, 123, 212, 115, 76, 0, 0, 0, 0, 191, 161, 201, 187, 0, 0, 0, 0, 185, 4, 244, 242, 153, 67, 0, 0, 0, 0, 40, 254, 34, 93, 0, 0, 0, 0, 165, 88, 43, 190, 0, 0, 0, 0, 194, 109, 169, 84, 0, 0, 0, 0, 185, 4, 116, 85, 134, 56, 0, 0, 0, 0, 246, 162, 39, 219, 0, 0, 0, 0, 199, 185, 217, 179, 0, 0, 0, 0, 50, 5, 2, 57, 0, 0, 0, 0, 185, 4, 171, 172, 89, 223, 0, 0, 0, 0, 199, 205, 80, 130, 0, 0, 0, 0, 134, 205, 65, 71, 0, 0, 0, 0, 29, 14, 106, 222, 0, 0, 0, 0, 185, 4, 97, 17, 41, 207, 0, 0, 0, 0, 200, 81, 77, 226, 0, 0, 0, 0, 114, 161, 249, 3, 0, 0, 0, 0, 165, 140, 250, 105, 0, 0, 0, 0, 185, 4, 212, 122, 91, 100, 0, 0, 0, 0, 163, 74, 141, 221, 0, 0, 0, 0, 93, 234, 59, 136, 0, 0, 0, 0, 189, 174, 117, 254, 0, 0, 0, 0, 185, 4, 217, 54, 153, 118, 0, 0, 0, 0, 36, 127, 142, 156, 0, 0, 0, 0, 113, 138, 159, 86, 0, 0, 0, 0, 55, 56, 180, 196, 0, 0, 0, 0, 185, 4, 202, 23, 56, 50, 0, 0, 0, 0, 201, 24, 247, 27, 0, 0, 0, 0, 239, 176, 144, 73, 0, 0, 0, 0, 241, 121, 53, 9, 0, 0, 0, 0, 185, 4, 181, 55, 104, 133, 0, 0, 0, 0, 172, 8, 7, 58, 0, 0, 0, 0, 194, 78, 192, 93, 0, 0, 0, 0, 237, 78, 87, 132, 0, 0, 0, 0, 185, 4, 198, 234, 55, 133, 0, 0, 0, 0, 83, 115, 100, 216, 0, 0, 0, 0, 155, 14, 87, 63, 0, 0, 0, 0, 208, 216, 197, 20, 0, 0, 0, 0, 185, 4, 243, 193, 250, 221, 0, 0, 0, 0, 221, 200, 219, 57, 0, 0, 0, 0, 253, 190, 20, 125, 0, 0, 0, 0, 191, 216, 199, 211, 0, 0, 0, 0, 185, 4, 216, 11, 90, 161, 0, 0, 0, 0, 66, 94, 61, 170, 0, 0, 0, 0, 85, 84, 216, 102, 0, 0, 0, 0, 114, 123, 151, 146, 0, 0, 0, 0, 185, 4, 93, 151, 81, 84, 0, 0, 0, 0, 65, 148, 77, 153, 0, 0, 0, 0, 172, 112, 143, 205, 0, 0, 0, 0, 80, 99, 171, 152, 0, 0, 0, 0, 185, 4, 109, 254, 20, 59, 0, 0, 0, 0, 117, 98, 20, 56, 0, 0, 0, 0, 37, 126, 143, 226, 0, 0, 0, 0, 35, 253, 251, 231, 0, 0, 0, 0, 185, 4, 2, 199, 34, 130, 0, 0, 0, 0, 70, 116, 39, 70, 0, 0, 0, 0, 88, 124, 193, 58, 0, 0, 0, 0, 196, 255, 188, 229, 0, 0, 0, 0, 185, 4, 255, 203, 188, 22, 0, 0, 0, 0, 252, 126, 126, 98, 0, 0, 0, 0, 243, 104, 163, 241, 0, 0, 0, 0, 218, 142, 36, 113, 0, 0, 0, 0, 185, 4, 74, 247, 220, 162, 0, 0, 0, 0, 107, 112, 152, 160, 0, 0, 0, 0, 153, 9, 146, 28, 0, 0, 0, 0, 128, 116, 26, 84, 0, 0, 0, 0, 185, 4, 65, 198, 218, 178, 0, 0, 0, 0, 53, 79, 96, 170, 0, 0, 0, 0, 214, 46, 230, 115, 0, 0, 0, 0, 166, 159, 87, 85, 0, 0, 0, 0, 185, 4, 59, 41, 194, 151, 0, 0, 0, 0, 177, 44, 22, 215, 0, 0, 0, 0, 48, 103, 94, 143, 0, 0, 0, 0, 195, 42, 182, 236, 0, 0, 0, 0, 185, 4, 6, 253, 162, 78, 0, 0, 0, 0, 149, 106, 150, 19, 0, 0, 0, 0, 152, 156, 192, 173, 0, 0, 0, 0, 207, 39, 131, 148, 0, 0, 0, 0, 185, 4, 168, 5, 63, 109, 0, 0, 0, 0, 177, 179, 183, 111, 0, 0, 0, 0, 113, 230, 35, 245, 0, 0, 0, 0, 2, 82, 71, 4, 0, 0, 0, 0, 185, 4, 192, 106, 72, 236, 0, 0, 0, 0, 86, 128, 220, 225, 0, 0, 0, 0, 141, 211, 232, 3, 0, 0, 0, 0, 89, 150, 82, 9, 0, 0, 0, 0, 185, 4, 156, 17, 159, 127, 0, 0, 0, 0, 147, 36, 124, 142, 0, 0, 0, 0, 157, 13, 225, 253, 0, 0, 0, 0, 168, 122, 140, 149, 0, 0, 0, 0, 185, 4, 218, 199, 205, 153, 0, 0, 0, 0, 198, 79, 174, 238, 0, 0, 0, 0, 48, 119, 41, 47, 0, 0, 0, 0, 84, 246, 80, 213, 0, 0, 0, 0, 185, 4, 202, 79, 239, 97, 0, 0, 0, 0, 26, 89, 62, 169, 0, 0, 0, 0, 66, 183, 247, 179, 0, 0, 0, 0, 117, 133, 62, 11, 0, 0, 0, 0, 185, 4, 188, 198, 197, 107, 0, 0, 0, 0, 72, 8, 94, 229, 0, 0, 0, 0, 28, 158, 6, 38, 0, 0, 0, 0, 0, 79, 90, 179, 0, 0, 0, 0, 185, 4, 107, 9, 103, 239, 0, 0, 0, 0, 252, 54, 109, 17, 0, 0, 0, 0, 238, 53, 142, 112, 0, 0, 0, 0, 191, 129, 101, 51, 0, 0, 0, 0, 185, 4, 203, 225, 120, 0, 0, 0, 0, 0, 55, 174, 65, 165, 0, 0, 0, 0, 94, 119, 110, 119, 0, 0, 0, 0, 6, 172, 4, 190, 0, 0, 0, 0, 185, 4, 204, 23, 155, 83, 0, 0, 0, 0, 142, 28, 64, 99, 0, 0, 0, 0, 153, 97, 36, 144, 0, 0, 0, 0, 70, 253, 130, 244, 0, 0, 0, 0, 185, 4, 193, 155, 137, 34, 0, 0, 0, 0, 97, 223, 46, 12, 0, 0, 0, 0, 230, 207, 61, 127, 0, 0, 0, 0, 109, 208, 33, 211, 0, 0, 0, 0, 185, 4, 188, 43, 188, 96, 0, 0, 0, 0, 230, 132, 139, 140, 0, 0, 0, 0, 96, 64, 116, 43, 0, 0, 0, 0, 189, 251, 107, 87, 0, 0, 0, 0, 185, 4, 15, 123, 118, 132, 0, 0, 0, 0, 246, 187, 34, 5, 0, 0, 0, 0, 29, 95, 47, 211, 0, 0, 0, 0, 196, 213, 113, 99, 0, 0, 0, 0, 185, 4, 218, 255, 76, 166, 0, 0, 0, 0, 54, 140, 141, 212, 0, 0, 0, 0, 72, 76, 42, 100, 0, 0, 0, 0, 126, 103, 236, 224, 0, 0, 0, 0, 185, 4, 247, 130, 95, 90, 0, 0, 0, 0, 0, 65, 47, 193, 0, 0, 0, 0, 212, 148, 158, 87, 0, 0, 0, 0, 110, 102, 43, 176, 0, 0, 0, 0, 185, 4, 129, 61, 107, 27, 0, 0, 0, 0, 160, 65, 197, 105, 0, 0, 0, 0, 31, 179, 5, 126, 0, 0, 0, 0, 104, 10, 147, 146, 0, 0, 0, 0, 185, 4, 231, 162, 23, 115, 0, 0, 0, 0, 228, 80, 168, 253, 0, 0, 0, 0, 99, 211, 109, 160, 0, 0, 0, 0, 148, 214, 8, 23, 0, 0, 0, 0, 185, 4, 202, 179, 243, 239, 0, 0, 0, 0, 248, 77, 150, 108, 0, 0, 0, 0, 253, 34, 209, 142, 0, 0, 0, 0, 2, 197, 162, 253, 0, 0, 0, 0, 185, 4, 12, 201, 142, 211, 0, 0, 0, 0, 115, 91, 0, 0, 0, 0, 0, 0, 124, 162, 138, 107, 0, 0, 0, 0, 163, 155, 45, 137, 0, 0, 0, 0, 185, 4, 195, 225, 52, 188, 0, 0, 0, 0, 81, 101, 24, 175, 0, 0, 0, 0, 37, 201, 226, 254, 0, 0, 0, 0, 17, 137, 83, 70, 0, 0, 0, 0, 185, 4, 36, 159, 46, 3, 0, 0, 0, 0, 208, 114, 233, 18, 0, 0, 0, 0, 194, 66, 109, 201, 0, 0, 0, 0, 195, 96, 171, 177, 0, 0, 0, 0, 185, 4, 16, 234, 91, 71, 0, 0, 0, 0, 20, 183, 212, 149, 0, 0, 0, 0, 163, 16, 109, 193, 0, 0, 0, 0, 72, 183, 106, 102, 0, 0, 0, 0, 185, 4, 90, 54, 204, 152, 0, 0, 0, 0, 75, 78, 88, 80, 0, 0, 0, 0, 186, 39, 58, 96, 0, 0, 0, 0, 115, 86, 174, 103, 0, 0, 0, 0, 185, 4, 111, 122, 9, 123, 0, 0, 0, 0, 71, 179, 229, 214, 0, 0, 0, 0, 99, 237, 139, 10, 0, 0, 0, 0, 36, 9, 96, 42, 0, 0, 0, 0, 185, 4, 149, 177, 211, 249, 0, 0, 0, 0, 217, 7, 174, 71, 0, 0, 0, 0, 122, 125, 127, 56, 0, 0, 0, 0, 146, 89, 29, 170, 0, 0, 0, 0, 185, 4, 149, 136, 203, 7, 0, 0, 0, 0, 3, 65, 249, 88, 0, 0, 0, 0, 141, 241, 167, 120, 0, 0, 0, 0, 167, 187, 221, 205, 0, 0, 0, 0, 185, 4, 147, 208, 213, 93, 0, 0, 0, 0, 118, 11, 228, 205, 0, 0, 0, 0, 117, 231, 41, 183, 0, 0, 0, 0, 118, 173, 110, 214, 0, 0, 0, 0, 185, 4, 8, 204, 234, 152, 0, 0, 0, 0, 23, 194, 86, 72, 0, 0, 0, 0, 24, 163, 156, 78, 0, 0, 0, 0, 7, 220, 79, 138, 0, 0, 0, 0, 185, 4, 117, 91, 174, 250, 0, 0, 0, 0, 223, 110, 195, 39, 0, 0, 0, 0, 248, 183, 98, 75, 0, 0, 0, 0, 4, 232, 111, 111, 0, 0, 0, 0, 185, 4, 183, 49, 155, 105, 0, 0, 0, 0, 174, 214, 121, 101, 0, 0, 0, 0, 217, 107, 52, 150, 0, 0, 0, 0, 160, 211, 6, 125, 0, 0, 0, 0, 185, 4, 7, 100, 189, 218, 0, 0, 0, 0, 61, 26, 205, 130, 0, 0, 0, 0, 120, 95, 183, 140, 0, 0, 0, 0, 67, 210, 91, 163, 0, 0, 0, 0, 185, 4, 159, 22, 210, 84, 0, 0, 0, 0, 254, 221, 215, 220, 0, 0, 0, 0, 70, 190, 102, 46, 0, 0, 0, 0, 223, 234, 218, 219, 0, 0, 0, 0, 185, 4, 43, 201, 3, 58, 0, 0, 0, 0, 135, 190, 251, 137, 0, 0, 0, 0, 2, 210, 233, 59, 0, 0, 0, 0, 57, 187, 249, 16, 0, 0, 0, 0, 185, 4, 197, 244, 61, 115, 0, 0, 0, 0, 117, 215, 61, 15, 0, 0, 0, 0, 201, 105, 201, 251, 0, 0, 0, 0, 18, 73, 130, 166, 0, 0, 0, 0, 185, 4, 155, 177, 88, 165, 0, 0, 0, 0, 140, 170, 171, 56, 0, 0, 0, 0, 53, 230, 113, 52, 0, 0, 0, 0, 80, 243, 230, 149, 0, 0, 0, 0, 185, 4, 211, 171, 65, 123, 0, 0, 0, 0, 191, 185, 210, 16, 0, 0, 0, 0, 27, 107, 17, 58, 0, 0, 0, 0, 245, 151, 77, 155, 0, 0, 0, 0, 185, 4, 2, 218, 32, 137, 0, 0, 0, 0, 77, 199, 252, 164, 0, 0, 0, 0, 243, 227, 187, 186, 0, 0, 0, 0, 213, 143, 41, 205, 0, 0, 0, 0, 185, 4, 106, 191, 119, 212, 0, 0, 0, 0, 79, 244, 69, 228, 0, 0, 0, 0, 190, 158, 19, 173, 0, 0, 0, 0, 133, 171, 12, 207, 0, 0, 0, 0, 185, 4, 43, 164, 248, 69, 0, 0, 0, 0, 188, 232, 208, 150, 0, 0, 0, 0, 120, 219, 120, 205, 0, 0, 0, 0, 48, 47, 236, 99, 0, 0, 0, 0, 185, 4, 121, 35, 117, 254, 0, 0, 0, 0, 194, 243, 125, 9, 0, 0, 0, 0, 2, 174, 18, 144, 0, 0, 0, 0, 85, 251, 9, 36, 0, 0, 0, 0, 185, 4, 4, 94, 194, 103, 0, 0, 0, 0, 116, 47, 163, 10, 0, 0, 0, 0, 173, 248, 62, 45, 0, 0, 0, 0, 243, 23, 60, 33, 0, 0, 0, 0, 185, 4, 43, 49, 195, 250, 0, 0, 0, 0, 162, 4, 86, 50, 0, 0, 0, 0, 179, 255, 143, 11, 0, 0, 0, 0, 229, 7, 5, 138, 0, 0, 0, 0, 185, 4, 72, 222, 0, 93, 0, 0, 0, 0, 72, 111, 86, 127, 0, 0, 0, 0, 211, 30, 27, 177, 0, 0, 0, 0, 154, 133, 80, 156, 0, 0, 0, 0, 185, 4, 65, 139, 149, 204, 0, 0, 0, 0, 194, 175, 150, 138, 0, 0, 0, 0, 231, 96, 238, 64, 0, 0, 0, 0, 24, 196, 132, 109, 0, 0, 0, 0, 185, 4, 103, 32, 193, 173, 0, 0, 0, 0, 221, 246, 81, 109, 0, 0, 0, 0, 201, 127, 232, 49, 0, 0, 0, 0, 76, 134, 189, 34, 0, 0, 0, 0, 185, 4, 169, 133, 197, 67, 0, 0, 0, 0, 218, 161, 125, 74, 0, 0, 0, 0, 209, 85, 177, 140, 0, 0, 0, 0, 155, 121, 176, 31, 0, 0, 0, 0, 185, 4, 247, 99, 118, 193, 0, 0, 0, 0, 8, 249, 158, 124, 0, 0, 0, 0, 143, 196, 183, 226, 0, 0, 0, 0, 126, 171, 50, 171, 0, 0, 0, 0, 185, 4, 118, 253, 202, 98, 0, 0, 0, 0, 104, 205, 65, 51, 0, 0, 0, 0, 220, 128, 136, 13, 0, 0, 0, 0, 182, 249, 76, 114, 0, 0, 0, 0, 185, 4, 147, 183, 194, 137, 0, 0, 0, 0, 187, 157, 3, 88, 0, 0, 0, 0, 28, 224, 155, 198, 0, 0, 0, 0, 157, 26, 255, 152, 0, 0, 0, 0, 185, 4, 25, 144, 221, 89, 0, 0, 0, 0, 116, 172, 99, 76, 0, 0, 0, 0, 172, 199, 38, 192, 0, 0, 0, 0, 195, 145, 66, 9, 0, 0, 0, 0, 185, 4, 203, 127, 84, 238, 0, 0, 0, 0, 161, 173, 25, 180, 0, 0, 0, 0, 218, 166, 160, 123, 0, 0, 0, 0, 31, 35, 214, 92, 0, 0, 0, 0, 185, 4, 203, 145, 75, 135, 0, 0, 0, 0, 150, 85, 28, 127, 0, 0, 0, 0, 225, 151, 64, 149, 0, 0, 0, 0, 139, 26, 122, 117, 0, 0, 0, 0, 185, 4, 254, 28, 196, 132, 0, 0, 0, 0, 52, 182, 205, 187, 0, 0, 0, 0, 241, 25, 223, 229, 0, 0, 0, 0, 237, 148, 169, 148, 0, 0, 0, 0, 185, 4, 119, 247, 138, 70, 0, 0, 0, 0, 33, 39, 252, 148, 0, 0, 0, 0, 108, 157, 67, 27, 0, 0, 0, 0, 17, 52, 223, 43, 0, 0, 0, 0, 185, 4, 42, 123, 132, 110, 0, 0, 0, 0, 8, 254, 222, 227, 0, 0, 0, 0, 249, 79, 68, 76, 0, 0, 0, 0, 61, 208, 7, 48, 0, 0, 0, 0, 185, 4, 27, 77, 253, 219, 0, 0, 0, 0, 11, 119, 210, 55, 0, 0, 0, 0, 32, 36, 212, 220, 0, 0, 0, 0, 87, 182, 185, 243, 0, 0, 0, 0, 185, 4, 162, 169, 21, 33, 0, 0, 0, 0, 250, 239, 160, 175, 0, 0, 0, 0, 32, 206, 0, 57, 0, 0, 0, 0, 61, 169, 5, 110, 0, 0, 0, 0, 185, 4, 208, 77, 43, 216, 0, 0, 0, 0, 107, 235, 224, 189, 0, 0, 0, 0, 139, 84, 188, 160, 0, 0, 0, 0, 127, 24, 69, 15, 0, 0, 0, 0, 185, 4, 127, 235, 101, 239, 0, 0, 0, 0, 104, 6, 25, 232, 0, 0, 0, 0, 72, 25, 234, 188, 0, 0, 0, 0, 181, 85, 62, 39, 0, 0, 0, 0, 185, 4, 195, 121, 248, 12, 0, 0, 0, 0, 220, 136, 134, 67, 0, 0, 0, 0, 121, 8, 174, 233, 0, 0, 0, 0, 244, 235, 245, 49, 0, 0, 0, 0, 185, 4, 45, 126, 248, 224, 0, 0, 0, 0, 164, 125, 206, 160, 0, 0, 0, 0, 223, 160, 79, 157, 0, 0, 0, 0, 79, 89, 50, 144, 0, 0, 0, 0, 185, 4, 4, 215, 245, 0, 0, 0, 0, 0, 250, 39, 183, 170, 0, 0, 0, 0, 194, 40, 11, 88, 0, 0, 0, 0, 59, 19, 228, 138, 0, 0, 0, 0, 185, 4, 18, 171, 238, 101, 0, 0, 0, 0, 186, 223, 163, 225, 0, 0, 0, 0, 244, 81, 54, 76, 0, 0, 0, 0, 138, 48, 96, 50, 0, 0, 0, 0, 185, 4, 0, 220, 16, 253, 0, 0, 0, 0, 18, 232, 200, 168, 0, 0, 0, 0, 227, 255, 105, 2, 0, 0, 0, 0, 93, 200, 204, 141, 0, 0, 0, 0, 185, 4, 152, 95, 75, 4, 0, 0, 0, 0, 185, 180, 4, 121, 0, 0, 0, 0, 248, 245, 250, 31, 0, 0, 0, 0, 6, 14, 158, 144, 0, 0, 0, 0, 185, 4, 122, 44, 219, 65, 0, 0, 0, 0, 220, 229, 8, 108, 0, 0, 0, 0, 35, 37, 110, 86, 0, 0, 0, 0, 214, 199, 156, 119, 0, 0, 0, 0, 185, 4, 5, 142, 109, 192, 0, 0, 0, 0, 58, 126, 27, 173, 0, 0, 0, 0, 49, 156, 188, 38, 0, 0, 0, 0, 228, 198, 198, 8, 0, 0, 0, 0, 185, 4, 86, 194, 48, 250, 0, 0, 0, 0, 155, 27, 33, 173, 0, 0, 0, 0, 194, 131, 196, 243, 0, 0, 0, 0, 248, 103, 2, 86, 0, 0, 0, 0, 185, 4, 206, 178, 21, 130, 0, 0, 0, 0, 209, 167, 233, 29, 0, 0, 0, 0, 36, 172, 106, 158, 0, 0, 0, 0, 31, 105, 209, 223, 0, 0, 0, 0, 185, 4, 41, 147, 223, 82, 0, 0, 0, 0, 119, 125, 80, 250, 0, 0, 0, 0, 10, 200, 112, 154, 0, 0, 0, 0, 117, 187, 64, 47, 0, 0, 0, 0, 185, 4, 68, 158, 251, 54, 0, 0, 0, 0, 67, 186, 231, 199, 0, 0, 0, 0, 2, 228, 97, 182, 0, 0, 0, 0, 243, 189, 11, 85, 0, 0, 0, 0, 185, 4, 79, 128, 39, 228, 0, 0, 0, 0, 56, 7, 225, 123, 0, 0, 0, 0, 94, 153, 247, 51, 0, 0, 0, 0, 161, 98, 245, 173, 0, 0, 0, 0, 185, 4, 254, 68, 202, 55, 0, 0, 0, 0, 14, 145, 221, 104, 0, 0, 0, 0, 130, 24, 40, 68, 0, 0, 0, 0, 187, 153, 202, 174, 0, 0, 0, 0, 185, 4, 23, 126, 47, 75, 0, 0, 0, 0, 136, 189, 222, 186, 0, 0, 0, 0, 238, 80, 50, 97, 0, 0, 0, 0, 93, 117, 82, 6, 0, 0, 0, 0, 185, 4, 80, 137, 184, 55, 0, 0, 0, 0, 187, 9, 22, 138, 0, 0, 0, 0, 101, 199, 116, 146, 0, 0, 0, 0, 18, 157, 80, 249, 0, 0, 0, 0, 185, 4, 149, 68, 91, 27, 0, 0, 0, 0, 100, 96, 8, 4, 0, 0, 0, 0, 0, 212, 167, 121, 0, 0, 0, 0, 109, 145, 89, 82, 0, 0, 0, 0, 185, 4, 122, 15, 72, 181, 0, 0, 0, 0, 156, 194, 25, 181, 0, 0, 0, 0, 187, 50, 126, 36, 0, 0, 0, 0, 242, 50, 136, 36, 0, 0, 0, 0, 185, 4, 93, 1, 4, 211, 0, 0, 0, 0, 71, 167, 16, 135, 0, 0, 0, 0, 251, 43, 102, 147, 0, 0, 0, 0, 150, 6, 186, 40, 0, 0, 0, 0, 185, 4, 216, 100, 10, 150, 0, 0, 0, 0, 134, 134, 247, 240, 0, 0, 0, 0, 197, 43, 223, 88, 0, 0, 0, 0, 97, 120, 143, 172, 0, 0, 0, 0, 185, 4, 248, 244, 158, 133, 0, 0, 0, 0, 245, 100, 135, 127, 0, 0, 0, 0, 233, 104, 161, 204, 0, 0, 0, 0, 199, 222, 109, 191, 0, 0, 0, 0, 185, 4, 11, 185, 29, 30, 0, 0, 0, 0, 56, 49, 161, 62, 0, 0, 0, 0, 75, 80, 39, 231, 0, 0, 0, 0, 234, 122, 237, 45, 0, 0, 0, 0, 185, 4, 196, 154, 163, 210, 0, 0, 0, 0, 29, 184, 66, 195, 0, 0, 0, 0, 102, 109, 99, 110, 0, 0, 0, 0, 173, 150, 213, 51, 0, 0, 0, 0, 185, 4, 67, 152, 173, 23, 0, 0, 0, 0, 44, 238, 145, 59, 0, 0, 0, 0, 46, 81, 182, 224, 0, 0, 0, 0, 153, 209, 71, 232, 0, 0, 0, 0, 185, 4, 172, 176, 138, 220, 0, 0, 0, 0, 75, 141, 179, 191, 0, 0, 0, 0, 216, 210, 204, 91, 0, 0, 0, 0, 92, 46, 153, 13, 0, 0, 0, 0, 185, 4, 149, 37, 54, 152, 0, 0, 0, 0, 151, 87, 9, 188, 0, 0, 0, 0, 218, 102, 154, 218, 0, 0, 0, 0, 228, 188, 143, 242, 0, 0, 0, 0, 185, 4, 118, 48, 83, 160, 0, 0, 0, 0, 102, 167, 191, 7, 0, 0, 0, 0, 116, 31, 233, 221, 0, 0, 0, 0, 161, 69, 190, 80, 0, 0, 0, 0, 185, 4, 161, 110, 196, 190, 0, 0, 0, 0, 195, 106, 73, 37, 0, 0, 0, 0, 74, 18, 54, 139, 0, 0, 0, 0, 201, 70, 132, 60, 0, 0, 0, 0, 185, 4, 13, 25, 182, 71, 0, 0, 0, 0, 222, 15, 20, 114, 0, 0, 0, 0, 25, 32, 153, 174, 0, 0, 0, 0, 211, 140, 166, 242, 0, 0, 0, 0, 185, 4, 181, 185, 17, 39, 0, 0, 0, 0, 29, 152, 58, 96, 0, 0, 0, 0, 235, 231, 12, 43, 0, 0, 0, 0, 8, 31, 118, 71, 0, 0, 0, 0, 185, 4, 150, 192, 76, 181, 0, 0, 0, 0, 123, 162, 130, 187, 0, 0, 0, 0, 121, 251, 22, 245, 0, 0, 0, 0, 145, 14, 211, 52, 0, 0, 0, 0, 185, 4, 34, 76, 214, 26, 0, 0, 0, 0, 8, 163, 86, 55, 0, 0, 0, 0, 33, 124, 34, 161, 0, 0, 0, 0, 13, 108, 198, 152, 0, 0, 0, 0, 185, 4, 43, 57, 55, 3, 0, 0, 0, 0, 231, 64, 12, 159, 0, 0, 0, 0, 146, 2, 8, 40, 0, 0, 0, 0, 19, 175, 164, 79, 0, 0, 0, 0, 185, 4, 237, 30, 63, 206, 0, 0, 0, 0, 244, 88, 44, 229, 0, 0, 0, 0, 48, 8, 184, 192, 0, 0, 0, 0, 95, 100, 49, 195, 0, 0, 0, 0, 185, 4, 239, 42, 40, 131, 0, 0, 0, 0, 149, 10, 23, 149, 0, 0, 0, 0, 101, 90, 229, 110, 0, 0, 0, 0, 14, 195, 50, 60, 0, 0, 0, 0, 185, 4, 87, 29, 59, 140, 0, 0, 0, 0, 130, 126, 211, 153, 0, 0, 0, 0, 107, 205, 97, 45, 0, 0, 0, 0, 156, 224, 64, 1, 0, 0, 0, 0, 185, 4, 80, 150, 192, 117, 0, 0, 0, 0, 219, 219, 182, 140, 0, 0, 0, 0, 79, 21, 74, 11, 0, 0, 0, 0, 26, 21, 68, 186, 0, 0, 0, 0, 185, 4, 172, 49, 110, 190, 0, 0, 0, 0, 166, 147, 152, 51, 0, 0, 0, 0, 187, 225, 211, 164, 0, 0, 0, 0, 220, 117, 114, 6, 0, 0, 0, 0, 185, 4, 107, 243, 51, 89, 0, 0, 0, 0, 65, 166, 15, 20, 0, 0, 0, 0, 245, 121, 48, 49, 0, 0, 0, 0, 176, 191, 156, 5, 0, 0, 0, 0, 185, 4, 168, 189, 217, 140, 0, 0, 0, 0, 83, 157, 68, 99, 0, 0, 0, 0, 43, 160, 90, 242, 0, 0, 0, 0, 141, 231, 183, 125, 0, 0, 0, 0, 185, 4, 199, 41, 141, 48, 0, 0, 0, 0, 191, 17, 230, 242, 0, 0, 0, 0, 226, 207, 7, 9, 0, 0, 0, 0, 127, 123, 166, 226, 0, 0, 0, 0, 185, 4, 92, 241, 183, 141, 0, 0, 0, 0, 64, 245, 29, 185, 0, 0, 0, 0, 41, 167, 70, 39, 0, 0, 0, 0, 143, 51, 96, 229, 0, 0, 0, 0, 185, 4, 191, 116, 84, 73, 0, 0, 0, 0, 211, 178, 219, 152, 0, 0, 0, 0, 96, 206, 177, 91, 0, 0, 0, 0, 194, 125, 231, 228, 0, 0, 0, 0, 185, 4, 200, 101, 56, 94, 0, 0, 0, 0, 241, 34, 144, 11, 0, 0, 0, 0, 201, 240, 138, 116, 0, 0, 0, 0, 6, 47, 189, 12, 0, 0, 0, 0, 185, 4, 73, 242, 180, 58, 0, 0, 0, 0, 246, 197, 78, 118, 0, 0, 0, 0, 221, 55, 25, 254, 0, 0, 0, 0, 96, 231, 25, 22, 0, 0, 0, 0, 185, 4, 17, 56, 193, 226, 0, 0, 0, 0, 14, 76, 23, 161, 0, 0, 0, 0, 252, 119, 152, 120, 0, 0, 0, 0, 217, 62, 32, 109, 0, 0, 0, 0, 185, 4, 150, 179, 206, 203, 0, 0, 0, 0, 38, 72, 173, 53, 0, 0, 0, 0, 81, 141, 14, 189, 0, 0, 0, 0, 55, 164, 83, 97, 0, 0, 0, 0, 185, 4, 36, 224, 136, 62, 0, 0, 0, 0, 88, 213, 80, 241, 0, 0, 0, 0, 213, 179, 222, 219, 0, 0, 0, 0, 138, 101, 97, 15, 0, 0, 0, 0, 185, 4, 175, 129, 248, 205, 0, 0, 0, 0, 203, 205, 115, 32, 0, 0, 0, 0, 203, 252, 76, 255, 0, 0, 0, 0, 48, 228, 105, 167, 0, 0, 0, 0, 185, 4, 159, 129, 210, 97, 0, 0, 0, 0, 167, 40, 66, 212, 0, 0, 0, 0, 150, 189, 101, 13, 0, 0, 0, 0, 219, 10, 47, 91, 0, 0, 0, 0, 185, 4, 45, 92, 198, 109, 0, 0, 0, 0, 106, 162, 62, 29, 0, 0, 0, 0, 101, 210, 247, 224, 0, 0, 0, 0, 250, 166, 97, 96, 0, 0, 0, 0, 185, 4, 178, 50, 6, 20, 0, 0, 0, 0, 111, 84, 178, 67, 0, 0, 0, 0, 170, 1, 82, 81, 0, 0, 0, 0, 180, 87, 79, 235, 0, 0, 0, 0, 185, 4, 70, 3, 253, 85, 0, 0, 0, 0, 203, 198, 167, 96, 0, 0, 0, 0, 128, 6, 99, 8, 0, 0, 0, 0, 234, 3, 115, 188, 0, 0, 0, 0, 185, 4, 118, 94, 193, 65, 0, 0, 0, 0, 225, 90, 24, 134, 0, 0, 0, 0, 85, 117, 29, 112, 0, 0, 0, 0, 70, 128, 223, 124, 0, 0, 0, 0, 185, 4, 56, 91, 16, 216, 0, 0, 0, 0, 89, 113, 109, 91, 0, 0, 0, 0, 187, 172, 128, 122, 0, 0, 0, 0, 36, 234, 231, 29, 0, 0, 0, 0, 185, 4, 49, 229, 36, 49, 0, 0, 0, 0, 14, 127, 239, 230, 0, 0, 0, 0, 135, 183, 78, 66, 0, 0, 0, 0, 170, 251, 250, 100, 0, 0, 0, 0, 185, 4, 149, 99, 44, 28, 0, 0, 0, 0, 71, 172, 219, 99, 0, 0, 0, 0, 64, 14, 202, 129, 0, 0, 0, 0, 161, 241, 156, 80, 0, 0, 0, 0, 185, 4, 253, 246, 243, 32, 0, 0, 0, 0, 112, 195, 8, 221, 0, 0, 0, 0, 130, 123, 115, 241, 0, 0, 0, 0, 113, 144, 211, 99, 0, 0, 0, 0, 185, 4, 195, 182, 36, 40, 0, 0, 0, 0, 40, 43, 84, 19, 0, 0, 0, 0, 24, 205, 118, 229, 0, 0, 0, 0, 133, 204, 128, 122, 0, 0, 0, 0, 185, 4, 89, 171, 81, 80, 0, 0, 0, 0, 53, 223, 5, 167, 0, 0, 0, 0, 153, 253, 15, 171, 0, 0, 0, 0, 189, 248, 153, 91, 0, 0, 0, 0, 185, 4, 57, 218, 231, 231, 0, 0, 0, 0, 171, 188, 65, 27, 0, 0, 0, 0, 81, 57, 109, 2, 0, 0, 0, 0, 28, 92, 38, 23, 0, 0, 0, 0, 185, 4, 225, 53, 60, 104, 0, 0, 0, 0, 94, 206, 105, 179, 0, 0, 0, 0, 120, 172, 13, 171, 0, 0, 0, 0, 101, 40, 24, 30, 0, 0, 0, 0, 185, 4, 84, 15, 190, 41, 0, 0, 0, 0, 214, 195, 209, 205, 0, 0, 0, 0, 194, 243, 65, 53, 0, 0, 0, 0, 47, 48, 32, 36, 0, 0, 0, 0, 185, 4, 2, 224, 175, 255, 0, 0, 0, 0, 40, 22, 116, 187, 0, 0, 0, 0, 18, 77, 114, 142, 0, 0, 0, 0, 215, 228, 166, 177, 0, 0, 0, 0, 185, 4, 163, 3, 45, 70, 0, 0, 0, 0, 196, 158, 74, 200, 0, 0, 0, 0, 114, 232, 167, 82, 0, 0, 0, 0, 185, 224, 147, 116, 0, 0, 0, 0, 185, 4, 111, 233, 198, 178, 0, 0, 0, 0, 96, 243, 99, 220, 0, 0, 0, 0, 10, 139, 167, 31, 0, 0, 0, 0, 174, 240, 21, 164, 0, 0, 0, 0, 185, 4, 219, 221, 38, 22, 0, 0, 0, 0, 111, 6, 149, 125, 0, 0, 0, 0, 40, 145, 120, 188, 0, 0, 0, 0, 61, 35, 232, 69, 0, 0, 0, 0, 185, 4, 80, 167, 60, 115, 0, 0, 0, 0, 49, 160, 230, 74, 0, 0, 0, 0, 223, 85, 234, 12, 0, 0, 0, 0, 67, 88, 247, 41, 0, 0, 0, 0, 185, 4, 36, 236, 25, 254, 0, 0, 0, 0, 226, 175, 18, 29, 0, 0, 0, 0, 23, 219, 100, 185, 0, 0, 0, 0, 140, 154, 10, 33, 0, 0, 0, 0, 185, 4, 167, 63, 99, 148, 0, 0, 0, 0, 180, 171, 237, 45, 0, 0, 0, 0, 227, 118, 128, 134, 0, 0, 0, 0, 207, 245, 6, 57, 0, 0, 0, 0, 185, 4, 184, 142, 189, 113, 0, 0, 0, 0, 51, 228, 102, 151, 0, 0, 0, 0, 236, 35, 103, 225, 0, 0, 0, 0, 192, 94, 90, 3, 0, 0, 0, 0, 185, 4, 98, 70, 112, 197, 0, 0, 0, 0, 58, 19, 97, 136, 0, 0, 0, 0, 246, 7, 188, 228, 0, 0, 0, 0, 59, 101, 223, 11, 0, 0, 0, 0, 185, 4, 205, 42, 163, 124, 0, 0, 0, 0, 188, 226, 1, 98, 0, 0, 0, 0, 28, 198, 7, 179, 0, 0, 0, 0, 233, 201, 59, 7, 0, 0, 0, 0, 185, 4, 107, 79, 232, 114, 0, 0, 0, 0, 103, 10, 82, 171, 0, 0, 0, 0, 84, 30, 186, 6, 0, 0, 0, 0, 112, 98, 61, 101, 0, 0, 0, 0, 185, 4, 105, 33, 48, 207, 0, 0, 0, 0, 243, 205, 20, 182, 0, 0, 0, 0, 158, 117, 222, 5, 0, 0, 0, 0, 39, 220, 142, 235, 0, 0, 0, 0, 185, 4, 212, 12, 238, 85, 0, 0, 0, 0, 240, 199, 108, 121, 0, 0, 0, 0, 4, 70, 2, 230, 0, 0, 0, 0, 140, 165, 245, 19, 0, 0, 0, 0, 185, 4, 107, 95, 188, 222, 0, 0, 0, 0, 38, 75, 47, 36, 0, 0, 0, 0, 83, 36, 158, 122, 0, 0, 0, 0, 214, 84, 58, 160, 0, 0, 0, 0, 185, 4, 202, 154, 69, 190, 0, 0, 0, 0, 122, 102, 189, 117, 0, 0, 0, 0, 214, 151, 7, 255, 0, 0, 0, 0, 251, 146, 7, 62, 0, 0, 0, 0, 185, 4, 217, 55, 59, 214, 0, 0, 0, 0, 93, 153, 81, 207, 0, 0, 0, 0, 249, 12, 234, 102, 0, 0, 0, 0, 41, 122, 188, 5, 0, 0, 0, 0, 185, 4, 44, 242, 148, 172, 0, 0, 0, 0, 40, 232, 7, 132, 0, 0, 0, 0, 174, 110, 74, 180, 0, 0, 0, 0, 159, 94, 24, 120, 0, 0, 0, 0, 185, 4, 140, 249, 237, 58, 0, 0, 0, 0, 157, 21, 10, 90, 0, 0, 0, 0, 215, 103, 88, 59, 0, 0, 0, 0, 140, 222, 110, 50, 0, 0, 0, 0, 185, 4, 121, 56, 157, 133, 0, 0, 0, 0, 56, 210, 241, 6, 0, 0, 0, 0, 229, 96, 168, 176, 0, 0, 0, 0, 17, 126, 198, 189, 0, 0, 0, 0, 185, 4, 75, 178, 163, 235, 0, 0, 0, 0, 105, 176, 206, 130, 0, 0, 0, 0, 57, 122, 227, 127, 0, 0, 0, 0, 48, 215, 74, 162, 0, 0, 0, 0, 185, 4, 123, 24, 188, 127, 0, 0, 0, 0, 163, 129, 126, 3, 0, 0, 0, 0, 241, 221, 36, 1, 0, 0, 0, 0, 11, 181, 247, 25, 0, 0, 0, 0, 185, 4, 237, 37, 54, 47, 0, 0, 0, 0, 46, 232, 107, 229, 0, 0, 0, 0, 41, 128, 40, 199, 0, 0, 0, 0, 135, 135, 250, 136, 0, 0, 0, 0, 185, 4, 187, 49, 134, 232, 0, 0, 0, 0, 178, 248, 130, 177, 0, 0, 0, 0, 150, 52, 149, 127, 0, 0, 0, 0, 40, 110, 112, 147, 0, 0, 0, 0, 185, 4, 64, 191, 17, 173, 0, 0, 0, 0, 158, 38, 229, 3, 0, 0, 0, 0, 45, 213, 156, 103, 0, 0, 0, 0, 251, 249, 250, 92, 0, 0, 0, 0, 185, 4, 128, 186, 216, 76, 0, 0, 0, 0, 16, 132, 38, 84, 0, 0, 0, 0, 132, 35, 34, 96, 0, 0, 0, 0, 211, 6, 11, 36, 0, 0, 0, 0, 185, 4, 47, 156, 23, 111, 0, 0, 0, 0, 82, 54, 142, 139, 0, 0, 0, 0, 208, 248, 148, 122, 0, 0, 0, 0, 85, 37, 219, 134, 0, 0, 0, 0, 185, 4, 88, 184, 254, 39, 0, 0, 0, 0, 126, 117, 228, 5, 0, 0, 0, 0, 198, 141, 173, 78, 0, 0, 0, 0, 115, 153, 6, 90, 0, 0, 0, 0, 185, 4, 203, 208, 84, 129, 0, 0, 0, 0, 15, 67, 165, 206, 0, 0, 0, 0, 11, 19, 137, 162, 0, 0, 0, 0, 19, 250, 150, 221, 0, 0, 0, 0, 185, 4, 203, 252, 16, 203, 0, 0, 0, 0, 236, 129, 36, 107, 0, 0, 0, 0, 239, 44, 196, 134, 0, 0, 0, 0, 82, 191, 195, 95, 0, 0, 0, 0, 185, 4, 188, 109, 89, 83, 0, 0, 0, 0, 74, 177, 26, 192, 0, 0, 0, 0, 102, 50, 11, 43, 0, 0, 0, 0, 132, 30, 105, 148, 0, 0, 0, 0, 185, 4, 227, 152, 28, 76, 0, 0, 0, 0, 152, 106, 226, 183, 0, 0, 0, 0, 101, 46, 99, 228, 0, 0, 0, 0, 59, 121, 66, 44, 0, 0, 0, 0, 185, 4, 161, 18, 36, 164, 0, 0, 0, 0, 164, 74, 161, 240, 0, 0, 0, 0, 235, 162, 15, 22, 0, 0, 0, 0, 200, 198, 113, 74, 0, 0, 0, 0, 185, 4, 127, 61, 227, 170, 0, 0, 0, 0, 15, 19, 217, 35, 0, 0, 0, 0, 215, 58, 173, 30, 0, 0, 0, 0, 144, 204, 26, 3, 0, 0, 0, 0, 185, 4, 32, 246, 123, 182, 0, 0, 0, 0, 253, 19, 233, 95, 0, 0, 0, 0, 141, 71, 155, 78, 0, 0, 0, 0, 202, 204, 183, 34, 0, 0, 0, 0, 185, 4, 141, 220, 224, 247, 0, 0, 0, 0, 27, 24, 7, 74, 0, 0, 0, 0, 29, 171, 21, 94, 0, 0, 0, 0, 238, 68, 241, 202, 0, 0, 0, 0, 185, 4, 23, 232, 203, 126, 0, 0, 0, 0, 144, 46, 149, 87, 0, 0, 0, 0, 92, 246, 96, 35, 0, 0, 0, 0, 176, 208, 42, 5, 0, 0, 0, 0, 185, 4, 22, 27, 241, 237, 0, 0, 0, 0, 194, 181, 228, 131, 0, 0, 0, 0, 246, 118, 3, 151, 0, 0, 0, 0, 241, 130, 43, 16, 0, 0, 0, 0, 185, 4, 184, 6, 195, 231, 0, 0, 0, 0, 38, 54, 230, 251, 0, 0, 0, 0, 225, 47, 218, 203, 0, 0, 0, 0, 17, 243, 127, 40, 0, 0, 0, 0, 185, 4, 192, 132, 253, 202, 0, 0, 0, 0, 78, 3, 233, 126, 0, 0, 0, 0, 60, 19, 79, 169, 0, 0, 0, 0, 163, 197, 188, 189, 0, 0, 0, 0, 185, 4, 34, 45, 115, 112, 0, 0, 0, 0, 254, 233, 171, 233, 0, 0, 0, 0, 177, 140, 96, 174, 0, 0, 0, 0, 128, 19, 26, 36, 0, 0, 0, 0, 185, 4, 166, 70, 107, 62, 0, 0, 0, 0, 115, 195, 160, 83, 0, 0, 0, 0, 221, 254, 72, 133, 0, 0, 0, 0, 169, 199, 42, 144, 0, 0, 0, 0, 185, 4, 198, 166, 244, 190, 0, 0, 0, 0, 174, 243, 91, 15, 0, 0, 0, 0, 218, 147, 245, 86, 0, 0, 0, 0, 1, 137, 131, 236, 0, 0, 0, 0, 185, 4, 151, 193, 27, 68, 0, 0, 0, 0, 227, 18, 234, 213, 0, 0, 0, 0, 235, 120, 247, 211, 0, 0, 0, 0, 225, 148, 236, 48, 0, 0, 0, 0, 185, 4, 151, 105, 220, 160, 0, 0, 0, 0, 225, 81, 84, 29, 0, 0, 0, 0, 145, 75, 15, 59, 0, 0, 0, 0, 40, 255, 60, 246, 0, 0, 0, 0, 185, 4, 3, 78, 147, 240, 0, 0, 0, 0, 195, 107, 217, 169, 0, 0, 0, 0, 14, 124, 112, 98, 0, 0, 0, 0, 205, 32, 7, 184, 0, 0, 0, 0, 185, 4, 241, 80, 53, 5, 0, 0, 0, 0, 93, 1, 31, 243, 0, 0, 0, 0, 2, 191, 182, 97, 0, 0, 0, 0, 201, 107, 120, 5, 0, 0, 0, 0, 185, 4, 5, 204, 17, 125, 0, 0, 0, 0, 122, 157, 146, 153, 0, 0, 0, 0, 211, 185, 141, 145, 0, 0, 0, 0, 236, 189, 170, 196, 0, 0, 0, 0, 185, 4, 6, 47, 155, 227, 0, 0, 0, 0, 157, 150, 173, 86, 0, 0, 0, 0, 173, 160, 20, 161, 0, 0, 0, 0, 192, 64, 16, 251, 0, 0, 0, 0, 185, 4, 8, 247, 182, 221, 0, 0, 0, 0, 163, 93, 250, 119, 0, 0, 0, 0, 216, 195, 94, 207, 0, 0, 0, 0, 150, 33, 175, 23, 0, 0, 0, 0, 185, 4, 61, 149, 197, 145, 0, 0, 0, 0, 252, 11, 103, 13, 0, 0, 0, 0, 87, 255, 127, 246, 0, 0, 0, 0, 37, 202, 171, 38, 0, 0, 0, 0, 185, 4, 129, 75, 126, 66, 0, 0, 0, 0, 51, 82, 32, 114, 0, 0, 0, 0, 40, 27, 18, 215, 0, 0, 0, 0, 73, 186, 123, 109, 0, 0, 0, 0, 185, 4, 224, 182, 75, 185, 0, 0, 0, 0, 138, 79, 24, 155, 0, 0, 0, 0, 60, 129, 4, 180, 0, 0, 0, 0, 232, 246, 165, 199, 0, 0, 0, 0, 185, 4, 165, 213, 160, 32, 0, 0, 0, 0, 1, 159, 195, 111, 0, 0, 0, 0, 213, 156, 188, 215, 0, 0, 0, 0, 129, 203, 167, 252, 0, 0, 0, 0, 185, 4, 122, 27, 51, 49, 0, 0, 0, 0, 129, 140, 187, 181, 0, 0, 0, 0, 117, 195, 245, 90, 0, 0, 0, 0, 28, 27, 248, 40, 0, 0, 0, 0, 185, 4, 120, 155, 125, 251, 0, 0, 0, 0, 56, 77, 240, 6, 0, 0, 0, 0, 42, 224, 181, 132, 0, 0, 0, 0, 206, 158, 61, 116, 0, 0, 0, 0, 185, 4, 49, 2, 56, 158, 0, 0, 0, 0, 170, 144, 232, 101, 0, 0, 0, 0, 233, 67, 37, 22, 0, 0, 0, 0, 66, 158, 234, 165, 0, 0, 0, 0, 185, 4, 8, 97, 139, 19, 0, 0, 0, 0, 112, 132, 105, 40, 0, 0, 0, 0, 51, 39, 177, 219, 0, 0, 0, 0, 142, 157, 223, 237, 0, 0, 0, 0, 185, 4, 64, 178, 233, 102, 0, 0, 0, 0, 212, 152, 96, 125, 0, 0, 0, 0, 234, 43, 172, 7, 0, 0, 0, 0, 106, 92, 47, 99, 0, 0, 0, 0, 185, 4, 69, 152, 239, 141, 0, 0, 0, 0, 116, 195, 75, 183, 0, 0, 0, 0, 97, 126, 130, 97, 0, 0, 0, 0, 68, 121, 76, 197, 0, 0, 0, 0, 185, 4, 144, 231, 51, 248, 0, 0, 0, 0, 83, 145, 131, 93, 0, 0, 0, 0, 42, 55, 209, 241, 0, 0, 0, 0, 154, 208, 240, 200, 0, 0, 0, 0, 185, 4, 121, 3, 36, 41, 0, 0, 0, 0, 37, 158, 122, 20, 0, 0, 0, 0, 207, 21, 251, 42, 0, 0, 0, 0, 212, 176, 118, 62, 0, 0, 0, 0, 185, 4, 253, 217, 134, 123, 0, 0, 0, 0, 251, 250, 44, 0, 0, 0, 0, 0, 210, 81, 192, 105, 0, 0, 0, 0, 34, 25, 13, 76, 0, 0, 0, 0, 185, 4, 86, 144, 34, 4, 0, 0, 0, 0, 196, 171, 148, 140, 0, 0, 0, 0, 186, 150, 211, 16, 0, 0, 0, 0, 214, 241, 177, 88, 0, 0, 0, 0, 185, 4, 221, 10, 243, 67, 0, 0, 0, 0, 125, 42, 14, 17, 0, 0, 0, 0, 17, 67, 19, 102, 0, 0, 0, 0, 163, 70, 113, 135, 0, 0, 0, 0, 185, 4, 87, 219, 250, 194, 0, 0, 0, 0, 95, 255, 200, 167, 0, 0, 0, 0, 69, 177, 139, 67, 0, 0, 0, 0, 191, 80, 156, 197, 0, 0, 0, 0, 185, 4, 155, 219, 54, 160, 0, 0, 0, 0, 201, 40, 95, 198, 0, 0, 0, 0, 60, 252, 59, 74, 0, 0, 0, 0, 244, 47, 204, 141, 0, 0, 0, 0, 185, 4, 170, 168, 180, 166, 0, 0, 0, 0, 207, 121, 210, 37, 0, 0, 0, 0, 83, 2, 31, 208, 0, 0, 0, 0, 145, 198, 74, 96, 0, 0, 0, 0, 185, 4, 198, 61, 227, 83, 0, 0, 0, 0, 19, 147, 251, 245, 0, 0, 0, 0, 181, 174, 109, 128, 0, 0, 0, 0, 21, 195, 196, 24, 0, 0, 0, 0, 185, 4, 21, 68, 135, 107, 0, 0, 0, 0, 199, 3, 194, 135, 0, 0, 0, 0, 219, 61, 109, 150, 0, 0, 0, 0, 20, 233, 208, 164, 0, 0, 0, 0, 185, 4, 61, 115, 170, 124, 0, 0, 0, 0, 194, 64, 129, 68, 0, 0, 0, 0, 237, 18, 40, 154, 0, 0, 0, 0, 247, 78, 37, 5, 0, 0, 0, 0, 185, 4, 19, 190, 76, 8, 0, 0, 0, 0, 13, 20, 117, 183, 0, 0, 0, 0, 83, 120, 5, 228, 0, 0, 0, 0, 76, 174, 215, 220, 0, 0, 0, 0, 185, 4, 250, 172, 11, 226, 0, 0, 0, 0, 156, 12, 193, 162, 0, 0, 0, 0, 52, 84, 61, 229, 0, 0, 0, 0, 210, 213, 206, 201, 0, 0, 0, 0, 185, 4, 37, 17, 163, 247, 0, 0, 0, 0, 65, 231, 34, 194, 0, 0, 0, 0, 228, 154, 167, 223, 0, 0, 0, 0, 226, 234, 187, 134, 0, 0, 0, 0, 185, 4, 25, 249, 148, 236, 0, 0, 0, 0, 97, 15, 118, 223, 0, 0, 0, 0, 6, 68, 129, 85, 0, 0, 0, 0, 92, 62, 175, 103, 0, 0, 0, 0, 185, 4, 120, 76, 190, 14, 0, 0, 0, 0, 92, 6, 161, 210, 0, 0, 0, 0, 25, 29, 216, 242, 0, 0, 0, 0, 251, 244, 214, 227, 0, 0, 0, 0, 185, 4, 165, 10, 100, 37, 0, 0, 0, 0, 146, 3, 144, 138, 0, 0, 0, 0, 220, 244, 245, 49, 0, 0, 0, 0, 49, 132, 119, 61, 0, 0, 0, 0, 185, 4, 229, 20, 152, 123, 0, 0, 0, 0, 73, 206, 75, 160, 0, 0, 0, 0, 76, 155, 121, 218, 0, 0, 0, 0, 18, 60, 124, 206, 0, 0, 0, 0, 185, 4, 49, 130, 183, 95, 0, 0, 0, 0, 88, 178, 110, 70, 0, 0, 0, 0, 20, 246, 17, 124, 0, 0, 0, 0, 253, 182, 18, 190, 0, 0, 0, 0, 185, 4, 166, 62, 136, 139, 0, 0, 0, 0, 173, 228, 150, 11, 0, 0, 0, 0, 80, 253, 190, 155, 0, 0, 0, 0, 64, 219, 215, 157, 0, 0, 0, 0, 185, 4, 187, 206, 89, 253, 0, 0, 0, 0, 236, 143, 191, 140, 0, 0, 0, 0, 251, 117, 205, 103, 0, 0, 0, 0, 238, 6, 25, 231, 0, 0, 0, 0, 185, 4, 184, 85, 52, 162, 0, 0, 0, 0, 97, 253, 251, 103, 0, 0, 0, 0, 96, 132, 111, 147, 0, 0, 0, 0, 119, 110, 201, 9, 0, 0, 0, 0, 185, 4, 194, 57, 15, 139, 0, 0, 0, 0, 105, 146, 105, 73, 0, 0, 0, 0, 183, 244, 22, 151, 0, 0, 0, 0, 249, 101, 254, 238, 0, 0, 0, 0, 185, 4, 114, 49, 135, 190, 0, 0, 0, 0, 138, 136, 30, 74, 0, 0, 0, 0, 112, 116, 63, 72, 0, 0, 0, 0, 213, 112, 175, 87, 0, 0, 0, 0, 185, 4, 59, 77, 36, 97, 0, 0, 0, 0, 248, 235, 133, 179, 0, 0, 0, 0, 203, 88, 226, 40, 0, 0, 0, 0, 211, 216, 100, 47, 0, 0, 0, 0, 185, 4, 78, 219, 129, 66, 0, 0, 0, 0, 246, 68, 218, 175, 0, 0, 0, 0, 215, 20, 55, 151, 0, 0, 0, 0, 41, 195, 59, 182, 0, 0, 0, 0, 185, 4, 11, 90, 13, 201, 0, 0, 0, 0, 207, 242, 247, 161, 0, 0, 0, 0, 172, 117, 10, 98, 0, 0, 0, 0, 98, 166, 100, 95, 0, 0, 0, 0, 185, 4, 10, 190, 114, 198, 0, 0, 0, 0, 221, 38, 96, 223, 0, 0, 0, 0, 58, 13, 99, 108, 0, 0, 0, 0, 146, 99, 151, 238, 0, 0, 0, 0, 185, 4, 124, 102, 165, 193, 0, 0, 0, 0, 129, 150, 162, 167, 0, 0, 0, 0, 8, 0, 105, 228, 0, 0, 0, 0, 180, 164, 26, 48, 0, 0, 0, 0, 185, 4, 93, 199, 93, 67, 0, 0, 0, 0, 85, 222, 237, 111, 0, 0, 0, 0, 201, 19, 9, 234, 0, 0, 0, 0, 203, 207, 14, 39, 0, 0, 0, 0, 185, 4, 203, 89, 245, 243, 0, 0, 0, 0, 185, 43, 212, 60, 0, 0, 0, 0, 78, 167, 93, 78, 0, 0, 0, 0, 123, 240, 78, 147, 0, 0, 0, 0, 185, 4, 160, 209, 117, 131, 0, 0, 0, 0, 153, 228, 94, 116, 0, 0, 0, 0, 70, 58, 38, 218, 0, 0, 0, 0, 139, 54, 55, 255, 0, 0, 0, 0, 185, 4, 177, 231, 116, 23, 0, 0, 0, 0, 12, 169, 59, 80, 0, 0, 0, 0, 20, 37, 228, 197, 0, 0, 0, 0, 54, 91, 25, 30, 0, 0, 0, 0, 185, 4, 96, 130, 171, 23, 0, 0, 0, 0, 102, 191, 61, 132, 0, 0, 0, 0, 179, 109, 215, 168, 0, 0, 0, 0, 122, 56, 159, 215, 0, 0, 0, 0, 185, 4, 168, 89, 241, 150, 0, 0, 0, 0, 235, 100, 140, 40, 0, 0, 0, 0, 107, 10, 172, 130, 0, 0, 0, 0, 243, 78, 100, 169, 0, 0, 0, 0, 185, 4, 14, 11, 47, 177, 0, 0, 0, 0, 3, 21, 152, 50, 0, 0, 0, 0, 131, 1, 227, 221, 0, 0, 0, 0, 80, 29, 153, 232, 0, 0, 0, 0, 185, 4, 76, 184, 206, 40, 0, 0, 0, 0, 89, 207, 6, 170, 0, 0, 0, 0, 111, 116, 136, 202, 0, 0, 0, 0, 249, 169, 118, 175, 0, 0, 0, 0, 185, 4, 250, 145, 153, 184, 0, 0, 0, 0, 252, 124, 72, 134, 0, 0, 0, 0, 201, 227, 149, 130, 0, 0, 0, 0, 135, 158, 62, 237, 0, 0, 0, 0, 185, 4, 50, 155, 133, 103, 0, 0, 0, 0, 242, 49, 24, 183, 0, 0, 0, 0, 80, 134, 168, 81, 0, 0, 0, 0, 220, 23, 253, 110, 0, 0, 0, 0, 185, 4, 229, 232, 178, 235, 0, 0, 0, 0, 248, 91, 15, 170, 0, 0, 0, 0, 209, 181, 122, 190, 0, 0, 0, 0, 72, 17, 91, 214, 0, 0, 0, 0, 185, 4, 239, 92, 205, 48, 0, 0, 0, 0, 175, 242, 79, 184, 0, 0, 0, 0, 99, 242, 42, 35, 0, 0, 0, 0, 76, 59, 137, 226, 0, 0, 0, 0, 185, 4, 11, 153, 195, 229, 0, 0, 0, 0, 79, 114, 198, 46, 0, 0, 0, 0, 151, 28, 255, 200, 0, 0, 0, 0, 181, 53, 150, 187, 0, 0, 0, 0, 185, 4, 10, 51, 49, 213, 0, 0, 0, 0, 77, 142, 79, 196, 0, 0, 0, 0, 102, 196, 166, 213, 0, 0, 0, 0, 156, 142, 180, 194, 0, 0, 0, 0, 185, 4, 170, 30, 220, 88, 0, 0, 0, 0, 254, 205, 113, 150, 0, 0, 0, 0, 56, 221, 198, 175, 0, 0, 0, 0, 215, 110, 188, 45, 0, 0, 0, 0, 185, 4, 79, 191, 83, 73, 0, 0, 0, 0, 49, 219, 170, 8, 0, 0, 0, 0, 122, 243, 129, 170, 0, 0, 0, 0, 174, 1, 89, 120, 0, 0, 0, 0, 185, 4, 7, 212, 147, 69, 0, 0, 0, 0, 204, 213, 202, 165, 0, 0, 0, 0, 238, 196, 48, 231, 0, 0, 0, 0, 225, 150, 204, 9, 0, 0, 0, 0, 185, 4, 146, 73, 36, 90, 0, 0, 0, 0, 94, 241, 248, 14, 0, 0, 0, 0, 117, 87, 233, 218, 0, 0, 0, 0, 245, 64, 176, 201, 0, 0, 0, 0, 185, 4, 111, 125, 168, 39, 0, 0, 0, 0, 65, 2, 154, 7, 0, 0, 0, 0, 203, 185, 80, 103, 0, 0, 0, 0, 126, 203, 236, 146, 0, 0, 0, 0, 185, 4, 82, 57, 89, 142, 0, 0, 0, 0, 77, 200, 1, 25, 0, 0, 0, 0, 33, 101, 156, 216, 0, 0, 0, 0, 37, 10, 33, 153, 0, 0, 0, 0, 185, 4, 42, 81, 135, 99, 0, 0, 0, 0, 89, 6, 164, 11, 0, 0, 0, 0, 223, 134, 32, 166, 0, 0, 0, 0, 109, 188, 156, 185, 0, 0, 0, 0, 185, 4, 130, 19, 220, 82, 0, 0, 0, 0, 18, 242, 104, 158, 0, 0, 0, 0, 179, 181, 190, 238, 0, 0, 0, 0, 44, 154, 39, 116, 0, 0, 0, 0, 185, 4, 145, 236, 154, 21, 0, 0, 0, 0, 243, 86, 112, 88, 0, 0, 0, 0, 227, 3, 97, 88, 0, 0, 0, 0, 224, 105, 167, 255, 0, 0, 0, 0, 185, 4, 225, 42, 221, 88, 0, 0, 0, 0, 246, 143, 248, 4, 0, 0, 0, 0, 220, 112, 37, 34, 0, 0, 0, 0, 201, 11, 27, 22, 0, 0, 0, 0, 185, 4, 192, 164, 112, 206, 0, 0, 0, 0, 173, 139, 118, 73, 0, 0, 0, 0, 250, 177, 168, 119, 0, 0, 0, 0, 253, 83, 13, 240, 0, 0, 0, 0, 185, 4, 48, 211, 151, 78, 0, 0, 0, 0, 105, 25, 107, 209, 0, 0, 0, 0, 90, 141, 63, 109, 0, 0, 0, 0, 21, 194, 113, 161, 0, 0, 0, 0, 185, 4, 237, 56, 167, 1, 0, 0, 0, 0, 197, 197, 227, 254, 0, 0, 0, 0, 140, 159, 120, 243, 0, 0, 0, 0, 1, 12, 252, 182, 0, 0, 0, 0, 185, 4, 169, 152, 169, 125, 0, 0, 0, 0, 62, 144, 138, 185, 0, 0, 0, 0, 200, 173, 113, 112, 0, 0, 0, 0, 216, 205, 226, 8, 0, 0, 0, 0, 185, 4, 225, 40, 122, 203, 0, 0, 0, 0, 166, 222, 198, 80, 0, 0, 0, 0, 246, 244, 11, 142, 0, 0, 0, 0, 111, 124, 125, 229, 0, 0, 0, 0, 185, 4, 80, 48, 45, 7, 0, 0, 0, 0, 117, 139, 54, 119, 0, 0, 0, 0, 118, 217, 40, 28, 0, 0, 0, 0, 112, 133, 161, 238, 0, 0, 0, 0, 185, 4, 210, 31, 228, 40, 0, 0, 0, 0, 212, 167, 136, 55, 0, 0, 0, 0, 207, 41, 198, 8, 0, 0, 0, 0, 93, 125, 101, 89, 0, 0, 0, 0, 185, 4, 239, 46, 190, 172, 0, 0, 0, 0, 118, 88, 154, 212, 0, 0, 0, 0, 29, 123, 244, 208, 0, 0, 0, 0, 117, 96, 54, 183, 0, 0, 0, 0, 185, 4, 224, 201, 163, 140, 0, 0, 0, 0, 137, 51, 253, 96, 0, 0, 0, 0, 15, 18, 118, 160, 0, 0, 0, 0, 153, 82, 74, 22, 0, 0, 0, 0, 185, 4, 20, 228, 230, 181, 0, 0, 0, 0, 44, 245, 199, 37, 0, 0, 0, 0, 16, 186, 193, 84, 0, 0, 0, 0, 221, 239, 188, 85, 0, 0, 0, 0, 185, 4, 162, 244, 110, 125, 0, 0, 0, 0, 88, 135, 61, 70, 0, 0, 0, 0, 134, 17, 122, 182, 0, 0, 0, 0, 145, 238, 208, 165, 0, 0, 0, 0, 185, 4, 171, 41, 24, 155, 0, 0, 0, 0, 47, 14, 52, 185, 0, 0, 0, 0, 217, 34, 251, 56, 0, 0, 0, 0, 91, 121, 249, 136, 0, 0, 0, 0, 185, 4, 62, 75, 183, 144, 0, 0, 0, 0, 177, 61, 1, 205, 0, 0, 0, 0, 89, 130, 230, 156, 0, 0, 0, 0, 142, 187, 168, 87, 0, 0, 0, 0, 185, 4, 83, 163, 13, 241, 0, 0, 0, 0, 187, 68, 134, 177, 0, 0, 0, 0, 138, 24, 183, 169, 0, 0, 0, 0, 187, 17, 11, 156, 0, 0, 0, 0, 185, 4, 123, 247, 52, 55, 0, 0, 0, 0, 187, 168, 4, 172, 0, 0, 0, 0, 62, 76, 178, 232, 0, 0, 0, 0, 173, 99, 0, 45, 0, 0, 0, 0, 185, 4, 132, 121, 59, 60, 0, 0, 0, 0, 25, 141, 163, 1, 0, 0, 0, 0, 16, 16, 138, 222, 0, 0, 0, 0, 139, 120, 237, 3, 0, 0, 0, 0, 185, 4, 185, 98, 100, 159, 0, 0, 0, 0, 135, 217, 148, 87, 0, 0, 0, 0, 157, 170, 190, 199, 0, 0, 0, 0, 81, 0, 71, 192, 0, 0, 0, 0, 185, 4, 233, 122, 86, 148, 0, 0, 0, 0, 68, 36, 90, 218, 0, 0, 0, 0, 122, 186, 150, 0, 0, 0, 0, 0, 15, 84, 140, 225, 0, 0, 0, 0, 185, 4, 19, 144, 200, 137, 0, 0, 0, 0, 158, 19, 82, 205, 0, 0, 0, 0, 104, 171, 161, 22, 0, 0, 0, 0, 137, 246, 82, 87, 0, 0, 0, 0, 185, 4, 134, 23, 166, 114, 0, 0, 0, 0, 238, 57, 102, 14, 0, 0, 0, 0, 181, 86, 34, 123, 0, 0, 0, 0, 34, 50, 3, 93, 0, 0, 0, 0, 185, 4, 221, 60, 184, 48, 0, 0, 0, 0, 55, 0, 26, 149, 0, 0, 0, 0, 90, 251, 248, 66, 0, 0, 0, 0, 210, 99, 124, 217, 0, 0, 0, 0, 185, 4, 62, 29, 215, 220, 0, 0, 0, 0, 94, 150, 140, 181, 0, 0, 0, 0, 230, 223, 16, 210, 0, 0, 0, 0, 245, 227, 138, 90, 0, 0, 0, 0, 185, 4, 236, 145, 105, 22, 0, 0, 0, 0, 53, 224, 244, 54, 0, 0, 0, 0, 55, 132, 187, 213, 0, 0, 0, 0, 122, 234, 45, 235, 0, 0, 0, 0, 185, 4, 24, 48, 65, 208, 0, 0, 0, 0, 72, 231, 233, 164, 0, 0, 0, 0, 80, 202, 8, 85, 0, 0, 0, 0, 114, 123, 84, 195, 0, 0, 0, 0, 185, 4, 21, 41, 191, 156, 0, 0, 0, 0, 246, 214, 117, 223, 0, 0, 0, 0, 54, 10, 131, 200, 0, 0, 0, 0, 104, 193, 200, 17, 0, 0, 0, 0, 185, 4, 194, 221, 175, 216, 0, 0, 0, 0, 156, 199, 190, 177, 0, 0, 0, 0, 240, 140, 6, 86, 0, 0, 0, 0, 183, 23, 4, 158, 0, 0, 0, 0, 185, 4, 230, 128, 166, 48, 0, 0, 0, 0, 14, 194, 73, 210, 0, 0, 0, 0, 46, 182, 78, 114, 0, 0, 0, 0, 86, 47, 123, 89, 0, 0, 0, 0, 185, 4, 45, 7, 52, 251, 0, 0, 0, 0, 158, 213, 163, 113, 0, 0, 0, 0, 37, 42, 92, 116, 0, 0, 0, 0, 105, 226, 78, 95, 0, 0, 0, 0, 185, 4, 84, 122, 62, 52, 0, 0, 0, 0, 198, 208, 174, 64, 0, 0, 0, 0, 16, 109, 177, 65, 0, 0, 0, 0, 254, 149, 55, 58, 0, 0, 0, 0, 185, 4, 115, 105, 17, 148, 0, 0, 0, 0, 155, 202, 147, 251, 0, 0, 0, 0, 207, 176, 98, 63, 0, 0, 0, 0, 68, 233, 228, 172, 0, 0, 0, 0, 185, 4, 110, 97, 8, 107, 0, 0, 0, 0, 51, 169, 129, 152, 0, 0, 0, 0, 4, 92, 148, 183, 0, 0, 0, 0, 169, 215, 202, 253, 0, 0, 0, 0, 185, 4, 194, 221, 179, 158, 0, 0, 0, 0, 29, 121, 197, 74, 0, 0, 0, 0, 253, 132, 0, 57, 0, 0, 0, 0, 42, 211, 76, 92, 0, 0, 0, 0, 185, 4, 16, 86, 76, 10, 0, 0, 0, 0, 149, 214, 46, 242, 0, 0, 0, 0, 64, 240, 181, 91, 0, 0, 0, 0, 26, 103, 174, 196, 0, 0, 0, 0, 185, 4, 184, 208, 202, 78, 0, 0, 0, 0, 74, 13, 205, 132, 0, 0, 0, 0, 156, 179, 71, 90, 0, 0, 0, 0, 247, 245, 210, 39, 0, 0, 0, 0, 185, 4, 133, 106, 126, 222, 0, 0, 0, 0, 26, 146, 170, 135, 0, 0, 0, 0, 79, 213, 54, 82, 0, 0, 0, 0, 227, 25, 46, 210, 0, 0, 0, 0, 185, 4, 192, 123, 58, 47, 0, 0, 0, 0, 88, 187, 231, 17, 0, 0, 0, 0, 94, 30, 79, 210, 0, 0, 0, 0, 52, 7, 226, 39, 0, 0, 0, 0, 185, 4, 129, 101, 168, 125, 0, 0, 0, 0, 143, 244, 249, 114, 0, 0, 0, 0, 75, 98, 5, 235, 0, 0, 0, 0, 103, 31, 215, 139, 0, 0, 0, 0, 185, 4, 180, 157, 114, 201, 0, 0, 0, 0, 94, 169, 88, 172, 0, 0, 0, 0, 231, 180, 251, 160, 0, 0, 0, 0, 99, 246, 41, 94, 0, 0, 0, 0, 185, 4, 19, 18, 170, 247, 0, 0, 0, 0, 249, 54, 133, 176, 0, 0, 0, 0, 216, 78, 49, 157, 0, 0, 0, 0, 203, 217, 67, 223, 0, 0, 0, 0, 185, 4, 80, 196, 242, 142, 0, 0, 0, 0, 113, 204, 200, 248, 0, 0, 0, 0, 254, 192, 251, 185, 0, 0, 0, 0, 165, 93, 142, 153, 0, 0, 0, 0, 185, 4, 79, 53, 199, 239, 0, 0, 0, 0, 139, 114, 71, 44, 0, 0, 0, 0, 222, 147, 137, 89, 0, 0, 0, 0, 152, 225, 68, 69, 0, 0, 0, 0, 185, 4, 219, 120, 8, 179, 0, 0, 0, 0, 20, 252, 160, 7, 0, 0, 0, 0, 181, 7, 159, 44, 0, 0, 0, 0, 168, 90, 113, 177, 0, 0, 0, 0, 185, 4, 23, 224, 196, 184, 0, 0, 0, 0, 145, 117, 225, 122, 0, 0, 0, 0, 53, 129, 74, 91, 0, 0, 0, 0, 67, 97, 98, 9, 0, 0, 0, 0, 185, 4, 203, 152, 27, 61, 0, 0, 0, 0, 9, 183, 117, 77, 0, 0, 0, 0, 103, 61, 149, 246, 0, 0, 0, 0, 30, 26, 154, 184, 0, 0, 0, 0, 185, 4, 181, 24, 137, 72, 0, 0, 0, 0, 125, 164, 90, 75, 0, 0, 0, 0, 122, 67, 193, 50, 0, 0, 0, 0, 140, 57, 175, 48, 0, 0, 0, 0, 185, 4, 179, 6, 220, 127, 0, 0, 0, 0, 145, 136, 102, 122, 0, 0, 0, 0, 250, 134, 121, 48, 0, 0, 0, 0, 121, 169, 123, 22, 0, 0, 0, 0, 185, 4, 152, 127, 54, 127, 0, 0, 0, 0, 5, 117, 118, 27, 0, 0, 0, 0, 110, 51, 73, 90, 0, 0, 0, 0, 33, 149, 2, 166, 0, 0, 0, 0, 185, 4, 95, 163, 204, 215, 0, 0, 0, 0, 211, 16, 244, 40, 0, 0, 0, 0, 139, 9, 168, 229, 0, 0, 0, 0, 114, 44, 186, 135, 0, 0, 0, 0, 185, 4, 203, 133, 176, 193, 0, 0, 0, 0, 79, 126, 151, 164, 0, 0, 0, 0, 0, 79, 137, 180, 0, 0, 0, 0, 32, 109, 20, 98, 0, 0, 0, 0, 185, 4, 87, 241, 171, 0, 0, 0, 0, 0, 25, 132, 68, 73, 0, 0, 0, 0, 195, 206, 50, 110, 0, 0, 0, 0, 104, 201, 191, 181, 0, 0, 0, 0, 185, 4, 253, 38, 72, 72, 0, 0, 0, 0, 59, 149, 86, 212, 0, 0, 0, 0, 197, 25, 97, 140, 0, 0, 0, 0, 233, 53, 95, 148, 0, 0, 0, 0, 185, 4, 106, 15, 116, 125, 0, 0, 0, 0, 201, 148, 131, 75, 0, 0, 0, 0, 44, 13, 244, 40, 0, 0, 0, 0, 244, 200, 90, 130, 0, 0, 0, 0, 185, 4, 220, 253, 232, 163, 0, 0, 0, 0, 234, 22, 245, 85, 0, 0, 0, 0, 95, 41, 138, 252, 0, 0, 0, 0, 57, 240, 248, 5, 0, 0, 0, 0, 185, 4, 36, 180, 145, 134, 0, 0, 0, 0, 245, 254, 204, 187, 0, 0, 0, 0, 101, 106, 161, 236, 0, 0, 0, 0, 171, 242, 115, 248, 0, 0, 0, 0, 185, 4, 230, 124, 73, 218, 0, 0, 0, 0, 190, 232, 21, 136, 0, 0, 0, 0, 146, 178, 93, 102, 0, 0, 0, 0, 172, 64, 126, 127, 0, 0, 0, 0, 185, 4, 40, 104, 50, 161, 0, 0, 0, 0, 244, 128, 53, 111, 0, 0, 0, 0, 235, 80, 160, 190, 0, 0, 0, 0, 223, 90, 7, 110, 0, 0, 0, 0, 185, 4, 110, 74, 15, 55, 0, 0, 0, 0, 125, 210, 178, 44, 0, 0, 0, 0, 192, 149, 202, 83, 0, 0, 0, 0, 74, 23, 157, 199, 0, 0, 0, 0, 185, 4, 250, 247, 170, 65, 0, 0, 0, 0, 53, 83, 210, 212, 0, 0, 0, 0, 232, 148, 57, 163, 0, 0, 0, 0, 233, 40, 209, 30, 0, 0, 0, 0, 185, 4, 56, 101, 99, 189, 0, 0, 0, 0, 118, 34, 132, 117, 0, 0, 0, 0, 106, 234, 115, 204, 0, 0, 0, 0, 30, 193, 123, 73, 0, 0, 0, 0, 185, 4, 98, 65, 28, 77, 0, 0, 0, 0, 223, 122, 71, 223, 0, 0, 0, 0, 220, 184, 17, 243, 0, 0, 0, 0, 246, 34, 198, 134, 0, 0, 0, 0, 185, 4, 185, 47, 54, 235, 0, 0, 0, 0, 225, 231, 251, 3, 0, 0, 0, 0, 247, 190, 139, 253, 0, 0, 0, 0, 170, 69, 205, 96, 0, 0, 0, 0, 185, 4, 91, 167, 70, 230, 0, 0, 0, 0, 66, 90, 246, 96, 0, 0, 0, 0, 39, 169, 161, 221, 0, 0, 0, 0, 133, 199, 18, 17, 0, 0, 0, 0, 185, 4, 32, 202, 29, 235, 0, 0, 0, 0, 192, 194, 56, 210, 0, 0, 0, 0, 119, 25, 234, 114, 0, 0, 0, 0, 62, 249, 147, 38, 0, 0, 0, 0, 185, 4, 61, 216, 109, 89, 0, 0, 0, 0, 147, 251, 90, 145, 0, 0, 0, 0, 113, 126, 244, 203, 0, 0, 0, 0, 178, 229, 183, 212, 0, 0, 0, 0, 185, 4, 240, 213, 215, 247, 0, 0, 0, 0, 96, 182, 17, 240, 0, 0, 0, 0, 50, 31, 54, 190, 0, 0, 0, 0, 88, 247, 182, 156, 0, 0, 0, 0, 185, 4, 247, 200, 101, 87, 0, 0, 0, 0, 49, 7, 44, 54, 0, 0, 0, 0, 163, 82, 46, 198, 0, 0, 0, 0, 36, 239, 231, 31, 0, 0, 0, 0, 185, 4, 45, 71, 194, 143, 0, 0, 0, 0, 225, 175, 232, 159, 0, 0, 0, 0, 22, 153, 11, 128, 0, 0, 0, 0, 219, 148, 93, 187, 0, 0, 0, 0, 185, 4, 38, 182, 45, 9, 0, 0, 0, 0, 253, 120, 66, 239, 0, 0, 0, 0, 0, 22, 24, 95, 0, 0, 0, 0, 237, 103, 1, 46, 0, 0, 0, 0, 185, 4, 237, 253, 51, 17, 0, 0, 0, 0, 162, 40, 163, 194, 0, 0, 0, 0, 130, 8, 119, 41, 0, 0, 0, 0, 40, 84, 209, 231, 0, 0, 0, 0, 185, 4, 163, 135, 199, 212, 0, 0, 0, 0, 20, 98, 127, 40, 0, 0, 0, 0, 136, 14, 197, 16, 0, 0, 0, 0, 13, 12, 233, 215, 0, 0, 0, 0, 185, 4, 196, 170, 148, 228, 0, 0, 0, 0, 111, 7, 138, 195, 0, 0, 0, 0, 196, 16, 86, 115, 0, 0, 0, 0, 88, 125, 98, 98, 0, 0, 0, 0, 185, 4, 124, 19, 65, 215, 0, 0, 0, 0, 19, 3, 50, 109, 0, 0, 0, 0, 111, 126, 118, 102, 0, 0, 0, 0, 35, 124, 130, 116, 0, 0, 0, 0, 185, 4, 126, 161, 0, 76, 0, 0, 0, 0, 25, 11, 205, 52, 0, 0, 0, 0, 242, 189, 146, 174, 0, 0, 0, 0, 20, 187, 5, 175, 0, 0, 0, 0, 185, 4, 96, 135, 161, 37, 0, 0, 0, 0, 254, 152, 232, 129, 0, 0, 0, 0, 216, 146, 45, 14, 0, 0, 0, 0, 122, 190, 47, 86, 0, 0, 0, 0, 185, 4, 74, 118, 202, 62, 0, 0, 0, 0, 115, 71, 73, 27, 0, 0, 0, 0, 154, 61, 67, 134, 0, 0, 0, 0, 129, 127, 117, 82, 0, 0, 0, 0, 185, 4, 230, 213, 247, 110, 0, 0, 0, 0, 59, 233, 7, 203, 0, 0, 0, 0, 254, 184, 184, 183, 0, 0, 0, 0, 61, 1, 223, 254, 0, 0, 0, 0, 185, 4, 3, 26, 132, 152, 0, 0, 0, 0, 18, 252, 24, 157, 0, 0, 0, 0, 75, 124, 35, 172, 0, 0, 0, 0, 207, 70, 101, 191, 0, 0, 0, 0, 185, 4, 177, 166, 58, 131, 0, 0, 0, 0, 84, 117, 122, 171, 0, 0, 0, 0, 182, 123, 27, 123, 0, 0, 0, 0, 214, 215, 159, 176, 0, 0, 0, 0, 185, 4, 110, 106, 163, 235, 0, 0, 0, 0, 68, 187, 77, 132, 0, 0, 0, 0, 8, 255, 205, 152, 0, 0, 0, 0, 126, 204, 199, 95, 0, 0, 0, 0, 185, 4, 249, 27, 0, 39, 0, 0, 0, 0, 115, 81, 5, 45, 0, 0, 0, 0, 108, 250, 25, 70, 0, 0, 0, 0, 140, 75, 95, 117, 0, 0, 0, 0, 185, 4, 188, 174, 39, 236, 0, 0, 0, 0, 146, 86, 189, 40, 0, 0, 0, 0, 104, 57, 111, 50, 0, 0, 0, 0, 250, 250, 75, 137, 0, 0, 0, 0, 185, 4, 14, 77, 233, 200, 0, 0, 0, 0, 88, 199, 77, 145, 0, 0, 0, 0, 238, 117, 136, 218, 0, 0, 0, 0, 40, 78, 157, 2, 0, 0, 0, 0, 185, 4, 15, 207, 62, 66, 0, 0, 0, 0, 23, 212, 106, 50, 0, 0, 0, 0, 56, 92, 169, 7, 0, 0, 0, 0, 213, 244, 61, 172, 0, 0, 0, 0, 185, 4, 66, 134, 30, 55, 0, 0, 0, 0, 81, 10, 127, 234, 0, 0, 0, 0, 76, 7, 136, 99, 0, 0, 0, 0, 240, 88, 51, 145, 0, 0, 0, 0, 185, 4, 137, 36, 120, 167, 0, 0, 0, 0, 225, 216, 237, 241, 0, 0, 0, 0, 170, 233, 169, 9, 0, 0, 0, 0, 26, 223, 192, 66, 0, 0, 0, 0, 185, 4, 106, 35, 140, 47, 0, 0, 0, 0, 10, 23, 44, 130, 0, 0, 0, 0, 56, 167, 145, 154, 0, 0, 0, 0, 220, 116, 193, 128, 0, 0, 0, 0, 185, 4, 235, 165, 144, 49, 0, 0, 0, 0, 79, 173, 175, 92, 0, 0, 0, 0, 219, 102, 131, 33, 0, 0, 0, 0, 113, 171, 10, 42, 0, 0, 0, 0, 185, 4, 139, 14, 58, 38, 0, 0, 0, 0, 72, 176, 1, 179, 0, 0, 0, 0, 51, 6, 35, 228, 0, 0, 0, 0, 177, 131, 135, 1, 0, 0, 0, 0, 185, 4, 105, 208, 184, 17, 0, 0, 0, 0, 166, 225, 77, 177, 0, 0, 0, 0, 66, 42, 227, 86, 0, 0, 0, 0, 170, 226, 213, 67, 0, 0, 0, 0, 185, 4, 160, 158, 16, 132, 0, 0, 0, 0, 197, 38, 67, 109, 0, 0, 0, 0, 85, 226, 186, 62, 0, 0, 0, 0, 147, 136, 243, 219, 0, 0, 0, 0, 185, 4, 119, 38, 246, 65, 0, 0, 0, 0, 23, 149, 225, 232, 0, 0, 0, 0, 237, 46, 248, 217, 0, 0, 0, 0, 53, 178, 194, 178, 0, 0, 0, 0, 185, 4, 12, 152, 164, 151, 0, 0, 0, 0, 222, 211, 235, 221, 0, 0, 0, 0, 131, 59, 10, 106, 0, 0, 0, 0, 247, 216, 244, 241, 0, 0, 0, 0, 185, 4, 58, 133, 27, 239, 0, 0, 0, 0, 190, 52, 104, 178, 0, 0, 0, 0, 188, 79, 34, 68, 0, 0, 0, 0, 149, 58, 199, 232, 0, 0, 0, 0, 185, 4, 245, 96, 237, 238, 0, 0, 0, 0, 229, 43, 189, 233, 0, 0, 0, 0, 18, 138, 51, 212, 0, 0, 0, 0, 135, 255, 106, 25, 0, 0, 0, 0, 185, 4, 126, 225, 130, 210, 0, 0, 0, 0, 57, 180, 146, 17, 0, 0, 0, 0, 180, 94, 149, 102, 0, 0, 0, 0, 167, 204, 133, 191, 0, 0, 0, 0, 185, 4, 237, 179, 105, 73, 0, 0, 0, 0, 195, 93, 218, 33, 0, 0, 0, 0, 21, 182, 48, 222, 0, 0, 0, 0, 244, 23, 148, 153, 0, 0, 0, 0, 185, 4, 89, 70, 149, 59, 0, 0, 0, 0, 149, 209, 237, 92, 0, 0, 0, 0, 150, 242, 193, 224, 0, 0, 0, 0, 0, 220, 126, 201, 0, 0, 0, 0, 185, 4, 136, 28, 20, 245, 0, 0, 0, 0, 248, 129, 24, 6, 0, 0, 0, 0, 38, 140, 223, 27, 0, 0, 0, 0, 38, 95, 12, 94, 0, 0, 0, 0, 185, 4, 173, 48, 252, 147, 0, 0, 0, 0, 40, 189, 48, 137, 0, 0, 0, 0, 132, 75, 30, 33, 0, 0, 0, 0, 158, 48, 14, 140, 0, 0, 0, 0, 185, 4, 245, 123, 113, 210, 0, 0, 0, 0, 141, 186, 14, 212, 0, 0, 0, 0, 88, 221, 153, 79, 0, 0, 0, 0, 117, 124, 29, 207, 0, 0, 0, 0, 185, 4, 84, 117, 64, 118, 0, 0, 0, 0, 222, 224, 187, 188, 0, 0, 0, 0, 197, 213, 75, 159, 0, 0, 0, 0, 155, 16, 15, 91, 0, 0, 0, 0, 185, 4, 186, 176, 57, 235, 0, 0, 0, 0, 15, 41, 101, 167, 0, 0, 0, 0, 174, 6, 152, 248, 0, 0, 0, 0, 120, 27, 6, 92, 0, 0, 0, 0, 185, 4, 156, 197, 15, 37, 0, 0, 0, 0, 192, 65, 113, 115, 0, 0, 0, 0, 236, 205, 36, 91, 0, 0, 0, 0, 41, 25, 170, 49, 0, 0, 0, 0, 185, 4, 111, 7, 99, 3, 0, 0, 0, 0, 172, 51, 16, 124, 0, 0, 0, 0, 150, 163, 47, 65, 0, 0, 0, 0, 123, 4, 248, 105, 0, 0, 0, 0, 185, 4, 89, 182, 21, 204, 0, 0, 0, 0, 198, 10, 236, 89, 0, 0, 0, 0, 51, 205, 48, 34, 0, 0, 0, 0, 101, 13, 182, 117, 0, 0, 0, 0, 185, 4, 208, 79, 82, 17, 0, 0, 0, 0, 61, 198, 188, 135, 0, 0, 0, 0, 65, 98, 89, 13, 0, 0, 0, 0, 202, 108, 4, 132, 0, 0, 0, 0, 185, 4, 65, 35, 30, 123, 0, 0, 0, 0, 142, 74, 227, 5, 0, 0, 0, 0, 229, 51, 7, 14, 0, 0, 0, 0, 85, 186, 42, 203, 0, 0, 0, 0, 185, 4, 149, 128, 118, 90, 0, 0, 0, 0, 147, 171, 91, 30, 0, 0, 0, 0, 97, 14, 226, 63, 0, 0, 0, 0, 229, 189, 165, 83, 0, 0, 0, 0, 185, 4, 136, 120, 165, 64, 0, 0, 0, 0, 176, 253, 219, 41, 0, 0, 0, 0, 189, 191, 54, 180, 0, 0, 0, 0, 18, 91, 204, 212, 0, 0, 0, 0, 185, 4, 233, 69, 100, 111, 0, 0, 0, 0, 237, 91, 26, 28, 0, 0, 0, 0, 48, 15, 37, 193, 0, 0, 0, 0, 48, 106, 55, 126, 0, 0, 0, 0, 185, 4, 42, 196, 71, 196, 0, 0, 0, 0, 135, 46, 147, 237, 0, 0, 0, 0, 195, 148, 72, 204, 0, 0, 0, 0, 82, 13, 156, 158, 0, 0, 0, 0, 185, 4, 137, 50, 20, 40, 0, 0, 0, 0, 33, 39, 185, 21, 0, 0, 0, 0, 115, 177, 208, 187, 0, 0, 0, 0, 124, 72, 230, 108, 0, 0, 0, 0, 185, 4, 183, 210, 92, 105, 0, 0, 0, 0, 95, 158, 246, 8, 0, 0, 0, 0, 212, 60, 185, 101, 0, 0, 0, 0, 228, 81, 168, 105, 0, 0, 0, 0, 185, 4, 174, 18, 228, 138, 0, 0, 0, 0, 37, 209, 30, 226, 0, 0, 0, 0, 229, 95, 82, 99, 0, 0, 0, 0, 157, 193, 113, 67, 0, 0, 0, 0, 185, 4, 224, 218, 79, 199, 0, 0, 0, 0, 15, 144, 246, 237, 0, 0, 0, 0, 56, 158, 224, 170, 0, 0, 0, 0, 252, 99, 228, 156, 0, 0, 0, 0, 185, 4, 65, 46, 23, 25, 0, 0, 0, 0, 152, 60, 51, 253, 0, 0, 0, 0, 172, 63, 213, 210, 0, 0, 0, 0, 226, 93, 79, 78, 0, 0, 0, 0, 185, 4, 200, 82, 92, 232, 0, 0, 0, 0, 119, 56, 99, 36, 0, 0, 0, 0, 171, 93, 215, 60, 0, 0, 0, 0, 215, 59, 85, 164, 0, 0, 0, 0, 185, 4, 85, 82, 89, 231, 0, 0, 0, 0, 213, 156, 167, 141, 0, 0, 0, 0, 179, 6, 173, 156, 0, 0, 0, 0, 82, 5, 249, 45, 0, 0, 0, 0, 185, 4, 188, 81, 164, 118, 0, 0, 0, 0, 200, 209, 70, 185, 0, 0, 0, 0, 120, 58, 224, 28, 0, 0, 0, 0, 158, 207, 209, 206, 0, 0, 0, 0, 185, 4, 46, 16, 194, 244, 0, 0, 0, 0, 24, 153, 167, 158, 0, 0, 0, 0, 16, 28, 26, 225, 0, 0, 0, 0, 125, 103, 32, 246, 0, 0, 0, 0, 185, 4, 214, 58, 11, 9, 0, 0, 0, 0, 202, 96, 135, 61, 0, 0, 0, 0, 107, 215, 48, 174, 0, 0, 0, 0, 97, 110, 68, 83, 0, 0, 0, 0, 185, 4, 209, 168, 115, 139, 0, 0, 0, 0, 25, 31, 129, 208, 0, 0, 0, 0, 41, 33, 119, 81, 0, 0, 0, 0, 200, 178, 96, 199, 0, 0, 0, 0, 185, 4, 222, 153, 97, 158, 0, 0, 0, 0, 159, 243, 57, 217, 0, 0, 0, 0, 207, 154, 51, 148, 0, 0, 0, 0, 248, 138, 194, 240, 0, 0, 0, 0, 185, 4, 73, 144, 182, 68, 0, 0, 0, 0, 129, 235, 144, 192, 0, 0, 0, 0, 82, 9, 221, 153, 0, 0, 0, 0, 60, 12, 82, 242, 0, 0, 0, 0, 185, 4, 158, 58, 250, 48, 0, 0, 0, 0, 98, 218, 184, 179, 0, 0, 0, 0, 60, 58, 213, 54, 0, 0, 0, 0, 159, 87, 223, 221, 0, 0, 0, 0, 185, 4, 158, 85, 229, 124, 0, 0, 0, 0, 223, 197, 7, 79, 0, 0, 0, 0, 108, 56, 46, 222, 0, 0, 0, 0, 87, 88, 97, 55, 0, 0, 0, 0, 185, 4, 26, 118, 202, 44, 0, 0, 0, 0, 162, 124, 141, 167, 0, 0, 0, 0, 0, 163, 152, 56, 0, 0, 0, 0, 154, 71, 34, 136, 0, 0, 0, 0, 185, 4, 96, 141, 126, 29, 0, 0, 0, 0, 45, 153, 246, 88, 0, 0, 0, 0, 52, 227, 246, 40, 0, 0, 0, 0, 203, 64, 0, 61, 0, 0, 0, 0, 185, 4, 187, 153, 3, 162, 0, 0, 0, 0, 163, 241, 98, 98, 0, 0, 0, 0, 28, 142, 183, 148, 0, 0, 0, 0, 82, 238, 151, 50, 0, 0, 0, 0, 185, 4, 104, 235, 221, 15, 0, 0, 0, 0, 132, 46, 216, 210, 0, 0, 0, 0, 41, 227, 236, 208, 0, 0, 0, 0, 80, 179, 165, 10, 0, 0, 0, 0, 185, 4, 198, 28, 74, 148, 0, 0, 0, 0, 121, 112, 91, 49, 0, 0, 0, 0, 53, 62, 124, 18, 0, 0, 0, 0, 26, 7, 111, 245, 0, 0, 0, 0, 185, 4, 87, 176, 211, 205, 0, 0, 0, 0, 247, 223, 32, 93, 0, 0, 0, 0, 226, 29, 112, 255, 0, 0, 0, 0, 185, 241, 127, 158, 0, 0, 0, 0, 185, 4, 125, 95, 94, 109, 0, 0, 0, 0, 140, 25, 7, 156, 0, 0, 0, 0, 137, 168, 215, 39, 0, 0, 0, 0, 155, 23, 11, 236, 0, 0, 0, 0, 185, 4, 172, 113, 201, 123, 0, 0, 0, 0, 0, 124, 250, 194, 0, 0, 0, 0, 232, 30, 151, 195, 0, 0, 0, 0, 187, 33, 120, 85, 0, 0, 0, 0, 185, 4, 25, 61, 220, 245, 0, 0, 0, 0, 37, 34, 82, 51, 0, 0, 0, 0, 76, 51, 59, 150, 0, 0, 0, 0, 171, 198, 45, 107, 0, 0, 0, 0, 185, 4, 114, 198, 40, 69, 0, 0, 0, 0, 95, 239, 230, 102, 0, 0, 0, 0, 127, 185, 234, 2, 0, 0, 0, 0, 43, 80, 101, 34, 0, 0, 0, 0, 185, 4, 118, 3, 5, 69, 0, 0, 0, 0, 145, 57, 207, 197, 0, 0, 0, 0, 101, 205, 247, 85, 0, 0, 0, 0, 226, 48, 147, 2, 0, 0, 0, 0, 185, 4, 89, 84, 225, 25, 0, 0, 0, 0, 87, 101, 153, 85, 0, 0, 0, 0, 164, 35, 124, 136, 0, 0, 0, 0, 96, 185, 11, 217, 0, 0, 0, 0, 185, 4, 141, 162, 75, 144, 0, 0, 0, 0, 220, 161, 72, 184, 0, 0, 0, 0, 170, 153, 154, 56, 0, 0, 0, 0, 43, 46, 119, 145, 0, 0, 0, 0, 185, 4, 183, 207, 215, 163, 0, 0, 0, 0, 81, 180, 33, 27, 0, 0, 0, 0, 216, 35, 107, 40, 0, 0, 0, 0, 35, 227, 207, 229, 0, 0, 0, 0, 185, 4, 2, 67, 47, 150, 0, 0, 0, 0, 59, 174, 78, 253, 0, 0, 0, 0, 32, 108, 183, 92, 0, 0, 0, 0, 68, 156, 22, 210, 0, 0, 0, 0, 185, 4, 195, 65, 255, 109, 0, 0, 0, 0, 90, 247, 239, 4, 0, 0, 0, 0, 249, 85, 165, 185, 0, 0, 0, 0, 211, 121, 129, 174, 0, 0, 0, 0, 185, 4, 95, 190, 109, 57, 0, 0, 0, 0, 197, 162, 28, 4, 0, 0, 0, 0, 86, 71, 57, 62, 0, 0, 0, 0, 199, 66, 35, 216, 0, 0, 0, 0, 185, 4, 7, 177, 194, 45, 0, 0, 0, 0, 36, 26, 13, 255, 0, 0, 0, 0, 248, 152, 254, 68, 0, 0, 0, 0, 241, 207, 112, 129, 0, 0, 0, 0, 185, 4, 34, 109, 71, 176, 0, 0, 0, 0, 15, 32, 19, 210, 0, 0, 0, 0, 84, 96, 146, 40, 0, 0, 0, 0, 221, 228, 136, 205, 0, 0, 0, 0, 185, 4, 69, 77, 189, 168, 0, 0, 0, 0, 229, 147, 250, 180, 0, 0, 0, 0, 120, 192, 9, 211, 0, 0, 0, 0, 156, 34, 231, 91, 0, 0, 0, 0, 185, 4, 198, 86, 20, 23, 0, 0, 0, 0, 202, 145, 161, 177, 0, 0, 0, 0, 69, 127, 72, 157, 0, 0, 0, 0, 221, 29, 6, 128, 0, 0, 0, 0, 185, 4, 161, 236, 95, 123, 0, 0, 0, 0, 97, 142, 32, 171, 0, 0, 0, 0, 192, 141, 130, 249, 0, 0, 0, 0, 6, 241, 85, 135, 0, 0, 0, 0, 185, 4, 101, 51, 162, 231, 0, 0, 0, 0, 42, 185, 184, 98, 0, 0, 0, 0, 203, 135, 43, 90, 0, 0, 0, 0, 82, 166, 29, 3, 0, 0, 0, 0, 185, 4, 183, 201, 231, 140, 0, 0, 0, 0, 100, 82, 72, 133, 0, 0, 0, 0, 133, 63, 178, 79, 0, 0, 0, 0, 144, 141, 125, 194, 0, 0, 0, 0, 185, 4, 107, 208, 177, 68, 0, 0, 0, 0, 104, 213, 241, 244, 0, 0, 0, 0, 133, 109, 68, 161, 0, 0, 0, 0, 216, 86, 251, 8, 0, 0, 0, 0, 185, 4, 221, 35, 222, 119, 0, 0, 0, 0, 92, 86, 41, 42, 0, 0, 0, 0, 88, 3, 8, 169, 0, 0, 0, 0, 188, 115, 189, 116, 0, 0, 0, 0, 185, 4, 103, 21, 19, 117, 0, 0, 0, 0, 171, 51, 185, 23, 0, 0, 0, 0, 70, 115, 46, 178, 0, 0, 0, 0, 252, 135, 148, 48, 0, 0, 0, 0, 185, 4, 60, 30, 10, 77, 0, 0, 0, 0, 221, 22, 45, 140, 0, 0, 0, 0, 56, 70, 194, 239, 0, 0, 0, 0, 39, 194, 108, 108, 0, 0, 0, 0, 185, 4, 145, 45, 9, 116, 0, 0, 0, 0, 48, 7, 139, 177, 0, 0, 0, 0, 58, 56, 125, 93, 0, 0, 0, 0, 155, 146, 27, 172, 0, 0, 0, 0, 185, 4, 207, 129, 111, 122, 0, 0, 0, 0, 148, 241, 249, 132, 0, 0, 0, 0, 224, 50, 54, 67, 0, 0, 0, 0, 4, 160, 34, 231, 0, 0, 0, 0, 185, 4, 135, 102, 142, 97, 0, 0, 0, 0, 185, 27, 54, 92, 0, 0, 0, 0, 200, 182, 47, 69, 0, 0, 0, 0, 50, 67, 91, 47, 0, 0, 0, 0, 185, 4, 22, 39, 140, 201, 0, 0, 0, 0, 47, 185, 230, 33, 0, 0, 0, 0, 201, 119, 127, 114, 0, 0, 0, 0, 16, 151, 211, 70, 0, 0, 0, 0, 185, 4, 244, 55, 5, 241, 0, 0, 0, 0, 64, 0, 89, 208, 0, 0, 0, 0, 33, 251, 160, 74, 0, 0, 0, 0, 163, 107, 137, 152, 0, 0, 0, 0, 185, 4, 43, 63, 10, 232, 0, 0, 0, 0, 253, 171, 201, 65, 0, 0, 0, 0, 68, 198, 45, 188, 0, 0, 0, 0, 182, 17, 210, 175, 0, 0, 0, 0, 185, 4, 133, 19, 6, 6, 0, 0, 0, 0, 56, 58, 110, 112, 0, 0, 0, 0, 1, 84, 124, 216, 0, 0, 0, 0, 190, 247, 85, 121, 0, 0, 0, 0, 185, 4, 117, 0, 187, 2, 0, 0, 0, 0, 66, 120, 219, 45, 0, 0, 0, 0, 73, 82, 34, 134, 0, 0, 0, 0, 179, 238, 174, 139, 0, 0, 0, 0, 185, 4, 17, 16, 91, 105, 0, 0, 0, 0, 82, 21, 240, 252, 0, 0, 0, 0, 16, 3, 178, 230, 0, 0, 0, 0, 3, 107, 23, 255, 0, 0, 0, 0, 185, 4, 29, 121, 73, 62, 0, 0, 0, 0, 105, 196, 171, 174, 0, 0, 0, 0, 199, 154, 72, 167, 0, 0, 0, 0, 105, 163, 39, 252, 0, 0, 0, 0, 185, 4, 241, 58, 185, 8, 0, 0, 0, 0, 87, 52, 150, 53, 0, 0, 0, 0, 11, 166, 183, 16, 0, 0, 0, 0, 18, 168, 232, 89, 0, 0, 0, 0, 185, 4, 44, 43, 81, 1, 0, 0, 0, 0, 162, 77, 101, 12, 0, 0, 0, 0, 76, 230, 253, 250, 0, 0, 0, 0, 6, 148, 35, 199, 0, 0, 0, 0, 185, 4, 111, 228, 152, 181, 0, 0, 0, 0, 100, 20, 55, 238, 0, 0, 0, 0, 149, 239, 26, 245, 0, 0, 0, 0, 215, 5, 247, 208, 0, 0, 0, 0, 185, 4, 81, 57, 102, 133, 0, 0, 0, 0, 37, 247, 138, 117, 0, 0, 0, 0, 5, 153, 118, 215, 0, 0, 0, 0, 61, 64, 193, 204, 0, 0, 0, 0, 185, 4, 171, 140, 179, 132, 0, 0, 0, 0, 136, 141, 153, 73, 0, 0, 0, 0, 187, 184, 224, 49, 0, 0, 0, 0, 33, 128, 11, 60, 0, 0, 0, 0, 185, 4, 230, 100, 227, 246, 0, 0, 0, 0, 226, 36, 94, 140, 0, 0, 0, 0, 140, 69, 48, 122, 0, 0, 0, 0, 40, 109, 5, 103, 0, 0, 0, 0, 185, 4, 4, 46, 218, 121, 0, 0, 0, 0, 209, 78, 247, 109, 0, 0, 0, 0, 11, 19, 247, 156, 0, 0, 0, 0, 21, 143, 116, 88, 0, 0, 0, 0, 185, 4, 18, 201, 184, 64, 0, 0, 0, 0, 109, 221, 81, 128, 0, 0, 0, 0, 223, 102, 75, 197, 0, 0, 0, 0, 179, 58, 244, 111, 0, 0, 0, 0, 185, 4, 70, 242, 226, 18, 0, 0, 0, 0, 122, 102, 101, 141, 0, 0, 0, 0, 25, 109, 23, 32, 0, 0, 0, 0, 254, 126, 160, 211, 0, 0, 0, 0, 185, 4, 209, 33, 169, 174, 0, 0, 0, 0, 123, 198, 73, 0, 0, 0, 0, 0, 131, 141, 4, 252, 0, 0, 0, 0, 106, 123, 66, 187, 0, 0, 0, 0, 185, 4, 125, 202, 117, 233, 0, 0, 0, 0, 6, 103, 110, 77, 0, 0, 0, 0, 74, 47, 87, 254, 0, 0, 0, 0, 66, 244, 112, 246, 0, 0, 0, 0, 185, 4, 111, 213, 122, 76, 0, 0, 0, 0, 58, 118, 40, 43, 0, 0, 0, 0, 213, 64, 21, 27, 0, 0, 0, 0, 151, 157, 99, 137, 0, 0, 0, 0, 185, 4, 65, 128, 146, 86, 0, 0, 0, 0, 96, 220, 3, 128, 0, 0, 0, 0, 253, 175, 119, 188, 0, 0, 0, 0, 199, 97, 206, 143, 0, 0, 0, 0, 185, 4, 216, 112, 208, 88, 0, 0, 0, 0, 203, 86, 249, 124, 0, 0, 0, 0, 144, 145, 118, 245, 0, 0, 0, 0, 39, 73, 212, 162, 0, 0, 0, 0, 185, 4, 251, 10, 66, 89, 0, 0, 0, 0, 222, 83, 251, 84, 0, 0, 0, 0, 105, 82, 70, 76, 0, 0, 0, 0, 158, 36, 133, 72, 0, 0, 0, 0, 185, 4, 229, 70, 55, 62, 0, 0, 0, 0, 80, 114, 252, 9, 0, 0, 0, 0, 1, 185, 189, 17, 0, 0, 0, 0, 118, 7, 244, 219, 0, 0, 0, 0, 185, 4, 222, 215, 244, 101, 0, 0, 0, 0, 23, 45, 142, 187, 0, 0, 0, 0, 71, 137, 196, 125, 0, 0, 0, 0, 102, 38, 237, 21, 0, 0, 0, 0, 185, 4, 133, 246, 192, 249, 0, 0, 0, 0, 205, 227, 232, 234, 0, 0, 0, 0, 80, 197, 67, 231, 0, 0, 0, 0, 139, 180, 18, 72, 0, 0, 0, 0, 185, 4, 17, 210, 245, 11, 0, 0, 0, 0, 88, 66, 10, 120, 0, 0, 0, 0, 47, 56, 43, 240, 0, 0, 0, 0, 51, 14, 170, 77, 0, 0, 0, 0, 185, 4, 8, 124, 240, 162, 0, 0, 0, 0, 36, 176, 210, 107, 0, 0, 0, 0, 176, 229, 186, 151, 0, 0, 0, 0, 19, 233, 255, 24, 0, 0, 0, 0, 185, 4, 55, 35, 114, 166, 0, 0, 0, 0, 167, 138, 58, 232, 0, 0, 0, 0, 135, 22, 17, 155, 0, 0, 0, 0, 230, 117, 91, 146, 0, 0, 0, 0, 185, 4, 31, 147, 181, 253, 0, 0, 0, 0, 54, 213, 238, 57, 0, 0, 0, 0, 66, 61, 89, 160, 0, 0, 0, 0, 231, 68, 135, 206, 0, 0, 0, 0, 185, 4, 93, 154, 87, 213, 0, 0, 0, 0, 210, 178, 181, 136, 0, 0, 0, 0, 179, 237, 123, 175, 0, 0, 0, 0, 164, 184, 86, 110, 0, 0, 0, 0, 185, 4, 56, 29, 223, 137, 0, 0, 0, 0, 157, 55, 98, 162, 0, 0, 0, 0, 254, 229, 255, 106, 0, 0, 0, 0, 86, 213, 14, 17, 0, 0, 0, 0, 185, 4, 213, 198, 221, 219, 0, 0, 0, 0, 88, 98, 140, 42, 0, 0, 0, 0, 95, 128, 0, 251, 0, 0, 0, 0, 104, 254, 8, 254, 0, 0, 0, 0, 185, 4, 148, 21, 27, 28, 0, 0, 0, 0, 74, 74, 122, 241, 0, 0, 0, 0, 167, 16, 91, 110, 0, 0, 0, 0, 72, 16, 253, 80, 0, 0, 0, 0, 185, 4, 231, 30, 41, 237, 0, 0, 0, 0, 177, 32, 246, 56, 0, 0, 0, 0, 137, 149, 58, 95, 0, 0, 0, 0, 186, 106, 122, 133, 0, 0, 0, 0, 185, 4, 4, 91, 144, 57, 0, 0, 0, 0, 172, 175, 59, 54, 0, 0, 0, 0, 232, 56, 57, 25, 0, 0, 0, 0, 99, 205, 30, 75, 0, 0, 0, 0, 185, 4, 87, 85, 175, 2, 0, 0, 0, 0, 147, 19, 130, 197, 0, 0, 0, 0, 50, 103, 78, 242, 0, 0, 0, 0, 209, 100, 37, 138, 0, 0, 0, 0, 185, 4, 198, 117, 52, 151, 0, 0, 0, 0, 135, 210, 52, 224, 0, 0, 0, 0, 143, 48, 145, 21, 0, 0, 0, 0, 76, 67, 151, 11, 0, 0, 0, 0, 185, 4, 237, 109, 93, 32, 0, 0, 0, 0, 96, 243, 187, 107, 0, 0, 0, 0, 247, 131, 122, 50, 0, 0, 0, 0, 221, 216, 62, 191, 0, 0, 0, 0, 185, 4, 84, 255, 65, 145, 0, 0, 0, 0, 72, 64, 206, 127, 0, 0, 0, 0, 88, 224, 229, 92, 0, 0, 0, 0, 133, 57, 102, 138, 0, 0, 0, 0, 185, 4, 21, 238, 66, 153, 0, 0, 0, 0, 37, 229, 29, 170, 0, 0, 0, 0, 166, 213, 226, 71, 0, 0, 0, 0, 140, 100, 222, 25, 0, 0, 0, 0, 185, 4, 36, 106, 251, 127, 0, 0, 0, 0, 113, 183, 48, 145, 0, 0, 0, 0, 22, 49, 2, 67, 0, 0, 0, 0, 40, 161, 75, 41, 0, 0, 0, 0, 185, 4, 71, 88, 220, 188, 0, 0, 0, 0, 190, 124, 91, 95, 0, 0, 0, 0, 62, 134, 196, 113, 0, 0, 0, 0, 13, 62, 133, 213, 0, 0, 0, 0, 185, 4, 175, 188, 235, 176, 0, 0, 0, 0, 53, 230, 137, 161, 0, 0, 0, 0, 196, 254, 199, 59, 0, 0, 0, 0, 4, 186, 205, 120, 0, 0, 0, 0, 185, 4, 193, 176, 0, 38, 0, 0, 0, 0, 96, 96, 255, 84, 0, 0, 0, 0, 101, 74, 183, 211, 0, 0, 0, 0, 112, 94, 232, 209, 0, 0, 0, 0, 185, 4, 217, 165, 216, 251, 0, 0, 0, 0, 4, 235, 65, 197, 0, 0, 0, 0, 138, 186, 149, 40, 0, 0, 0, 0, 32, 176, 135, 182, 0, 0, 0, 0, 185, 4, 32, 110, 93, 190, 0, 0, 0, 0, 28, 88, 206, 235, 0, 0, 0, 0, 62, 165, 18, 143, 0, 0, 0, 0, 169, 213, 160, 155, 0, 0, 0, 0, 185, 4, 60, 248, 163, 111, 0, 0, 0, 0, 127, 3, 87, 173, 0, 0, 0, 0, 109, 179, 255, 104, 0, 0, 0, 0, 30, 68, 55, 246, 0, 0, 0, 0, 185, 4, 224, 107, 92, 129, 0, 0, 0, 0, 163, 250, 125, 74, 0, 0, 0, 0, 21, 45, 249, 95, 0, 0, 0, 0, 59, 198, 154, 0, 0, 0, 0, 0, 185, 4, 17, 250, 242, 131, 0, 0, 0, 0, 204, 189, 22, 141, 0, 0, 0, 0, 243, 105, 153, 245, 0, 0, 0, 0, 173, 235, 147, 3, 0, 0, 0, 0, 185, 4, 192, 71, 138, 24, 0, 0, 0, 0, 216, 59, 226, 145, 0, 0, 0, 0, 140, 102, 183, 137, 0, 0, 0, 0, 53, 162, 98, 225, 0, 0, 0, 0, 185, 4, 220, 77, 5, 69, 0, 0, 0, 0, 72, 57, 207, 147, 0, 0, 0, 0, 66, 123, 176, 0, 0, 0, 0, 0, 225, 133, 218, 147, 0, 0, 0, 0, 185, 4, 232, 189, 75, 77, 0, 0, 0, 0, 103, 151, 152, 74, 0, 0, 0, 0, 13, 113, 156, 55, 0, 0, 0, 0, 164, 201, 71, 241, 0, 0, 0, 0, 185, 4, 206, 78, 42, 176, 0, 0, 0, 0, 117, 142, 53, 123, 0, 0, 0, 0, 8, 223, 243, 149, 0, 0, 0, 0, 198, 240, 48, 188, 0, 0, 0, 0, 185, 4, 238, 63, 74, 226, 0, 0, 0, 0, 144, 172, 74, 27, 0, 0, 0, 0, 8, 142, 67, 226, 0, 0, 0, 0, 28, 95, 243, 7, 0, 0, 0, 0, 185, 4, 153, 64, 206, 44, 0, 0, 0, 0, 202, 55, 36, 174, 0, 0, 0, 0, 45, 69, 176, 161, 0, 0, 0, 0, 15, 164, 95, 159, 0, 0, 0, 0, 185, 4, 106, 22, 65, 113, 0, 0, 0, 0, 138, 208, 195, 171, 0, 0, 0, 0, 185, 108, 130, 22, 0, 0, 0, 0, 210, 6, 1, 45, 0, 0, 0, 0, 185, 4, 148, 200, 195, 109, 0, 0, 0, 0, 114, 127, 211, 188, 0, 0, 0, 0, 107, 104, 85, 50, 0, 0, 0, 0, 96, 124, 26, 21, 0, 0, 0, 0, 185, 4, 7, 83, 17, 237, 0, 0, 0, 0, 50, 101, 32, 26, 0, 0, 0, 0, 228, 10, 246, 236, 0, 0, 0, 0, 152, 209, 89, 200, 0, 0, 0, 0, 185, 4, 210, 72, 115, 156, 0, 0, 0, 0, 224, 37, 241, 93, 0, 0, 0, 0, 16, 94, 37, 8, 0, 0, 0, 0, 37, 108, 52, 85, 0, 0, 0, 0, 185, 4, 106, 15, 112, 3, 0, 0, 0, 0, 204, 228, 48, 238, 0, 0, 0, 0, 47, 105, 166, 26, 0, 0, 0, 0, 129, 109, 57, 198, 0, 0, 0, 0, 185, 4, 203, 39, 152, 96, 0, 0, 0, 0, 206, 112, 99, 163, 0, 0, 0, 0, 229, 153, 130, 49, 0, 0, 0, 0, 130, 73, 139, 40, 0, 0, 0, 0, 185, 4, 254, 222, 195, 239, 0, 0, 0, 0, 47, 183, 56, 169, 0, 0, 0, 0, 101, 77, 245, 123, 0, 0, 0, 0, 102, 104, 153, 31, 0, 0, 0, 0, 185, 4, 29, 120, 46, 144, 0, 0, 0, 0, 37, 53, 128, 194, 0, 0, 0, 0, 64, 198, 149, 205, 0, 0, 0, 0, 99, 159, 56, 159, 0, 0, 0, 0, 185, 4, 113, 200, 114, 16, 0, 0, 0, 0, 179, 239, 69, 235, 0, 0, 0, 0, 201, 237, 105, 128, 0, 0, 0, 0, 23, 179, 186, 112, 0, 0, 0, 0, 185, 4, 53, 92, 98, 243, 0, 0, 0, 0, 177, 216, 182, 80, 0, 0, 0, 0, 28, 50, 107, 199, 0, 0, 0, 0, 170, 76, 38, 189, 0, 0, 0, 0, 185, 4, 109, 47, 234, 239, 0, 0, 0, 0, 202, 143, 239, 178, 0, 0, 0, 0, 124, 198, 159, 155, 0, 0, 0, 0, 110, 218, 185, 57, 0, 0, 0, 0, 185, 4, 202, 113, 160, 199, 0, 0, 0, 0, 133, 80, 248, 16, 0, 0, 0, 0, 40, 196, 110, 61, 0, 0, 0, 0, 241, 169, 200, 50, 0, 0, 0, 0, 185, 4, 67, 188, 244, 106, 0, 0, 0, 0, 24, 50, 37, 218, 0, 0, 0, 0, 134, 135, 78, 80, 0, 0, 0, 0, 166, 22, 213, 1, 0, 0, 0, 0, 185, 4, 251, 69, 174, 183, 0, 0, 0, 0, 177, 21, 119, 111, 0, 0, 0, 0, 104, 70, 52, 251, 0, 0, 0, 0, 53, 184, 193, 175, 0, 0, 0, 0, 185, 4, 16, 238, 77, 55, 0, 0, 0, 0, 36, 22, 204, 203, 0, 0, 0, 0, 240, 15, 69, 84, 0, 0, 0, 0, 106, 165, 72, 238, 0, 0, 0, 0, 185, 4, 178, 229, 236, 243, 0, 0, 0, 0, 232, 226, 7, 64, 0, 0, 0, 0, 138, 251, 22, 251, 0, 0, 0, 0, 222, 60, 69, 168, 0, 0, 0, 0, 185, 4, 211, 236, 117, 224, 0, 0, 0, 0, 204, 18, 109, 184, 0, 0, 0, 0, 123, 37, 98, 212, 0, 0, 0, 0, 40, 73, 145, 145, 0, 0, 0, 0, 185, 4, 92, 189, 159, 95, 0, 0, 0, 0, 86, 214, 133, 191, 0, 0, 0, 0, 147, 240, 44, 139, 0, 0, 0, 0, 184, 10, 190, 211, 0, 0, 0, 0, 185, 4, 172, 253, 245, 85, 0, 0, 0, 0, 156, 76, 15, 104, 0, 0, 0, 0, 155, 22, 54, 199, 0, 0, 0, 0, 195, 54, 235, 40, 0, 0, 0, 0, 185, 4, 14, 56, 186, 56, 0, 0, 0, 0, 193, 126, 124, 51, 0, 0, 0, 0, 89, 207, 16, 48, 0, 0, 0, 0, 180, 148, 114, 80, 0, 0, 0, 0, 185, 4, 10, 62, 198, 40, 0, 0, 0, 0, 238, 211, 3, 180, 0, 0, 0, 0, 169, 150, 135, 153, 0, 0, 0, 0, 37, 5, 106, 248, 0, 0, 0, 0, 185, 4, 205, 233, 205, 231, 0, 0, 0, 0, 70, 125, 127, 211, 0, 0, 0, 0, 94, 51, 25, 91, 0, 0, 0, 0, 0, 24, 20, 81, 0, 0, 0, 0, 185, 4, 74, 49, 121, 120, 0, 0, 0, 0, 113, 133, 179, 249, 0, 0, 0, 0, 143, 226, 88, 13, 0, 0, 0, 0, 220, 164, 213, 183, 0, 0, 0, 0, 185, 4, 167, 51, 171, 59, 0, 0, 0, 0, 215, 199, 0, 88, 0, 0, 0, 0, 202, 38, 43, 126, 0, 0, 0, 0, 122, 86, 41, 39, 0, 0, 0, 0, 185, 4, 201, 197, 187, 205, 0, 0, 0, 0, 7, 2, 98, 202, 0, 0, 0, 0, 106, 162, 214, 48, 0, 0, 0, 0, 65, 182, 189, 189, 0, 0, 0, 0, 185, 4, 217, 246, 104, 144, 0, 0, 0, 0, 28, 174, 28, 36, 0, 0, 0, 0, 186, 225, 111, 167, 0, 0, 0, 0, 12, 1, 92, 112, 0, 0, 0, 0, 185, 4, 32, 46, 191, 99, 0, 0, 0, 0, 4, 22, 128, 193, 0, 0, 0, 0, 86, 151, 181, 192, 0, 0, 0, 0, 80, 134, 57, 220, 0, 0, 0, 0, 185, 4, 30, 104, 81, 181, 0, 0, 0, 0, 212, 111, 71, 237, 0, 0, 0, 0, 123, 250, 19, 143, 0, 0, 0, 0, 63, 160, 28, 45, 0, 0, 0, 0, 185, 4, 39, 96, 32, 160, 0, 0, 0, 0, 32, 64, 245, 188, 0, 0, 0, 0, 107, 143, 45, 147, 0, 0, 0, 0, 246, 62, 136, 32, 0, 0, 0, 0, 185, 4, 4, 222, 128, 161, 0, 0, 0, 0, 32, 20, 145, 134, 0, 0, 0, 0, 112, 118, 117, 161, 0, 0, 0, 0, 109, 69, 130, 217, 0, 0, 0, 0, 185, 4, 102, 113, 151, 233, 0, 0, 0, 0, 69, 160, 106, 78, 0, 0, 0, 0, 116, 63, 6, 161, 0, 0, 0, 0, 125, 159, 115, 185, 0, 0, 0, 0, 185, 4, 66, 125, 197, 107, 0, 0, 0, 0, 189, 196, 173, 140, 0, 0, 0, 0, 182, 15, 250, 147, 0, 0, 0, 0, 168, 239, 44, 193, 0, 0, 0, 0, 185, 4, 44, 58, 138, 7, 0, 0, 0, 0, 91, 0, 99, 170, 0, 0, 0, 0, 136, 46, 128, 108, 0, 0, 0, 0, 142, 15, 154, 210, 0, 0, 0, 0, 185, 4, 21, 227, 53, 117, 0, 0, 0, 0, 10, 47, 10, 36, 0, 0, 0, 0, 188, 81, 37, 250, 0, 0, 0, 0, 165, 109, 18, 47, 0, 0, 0, 0, 185, 4, 195, 5, 174, 20, 0, 0, 0, 0, 37, 217, 63, 213, 0, 0, 0, 0, 10, 154, 54, 218, 0, 0, 0, 0, 209, 10, 213, 135, 0, 0, 0, 0, 185, 4, 211, 188, 239, 155, 0, 0, 0, 0, 79, 250, 42, 156, 0, 0, 0, 0, 150, 16, 137, 50, 0, 0, 0, 0, 217, 170, 158, 197, 0, 0, 0, 0, 185, 4, 89, 89, 36, 47, 0, 0, 0, 0, 123, 242, 54, 117, 0, 0, 0, 0, 218, 131, 52, 250, 0, 0, 0, 0, 104, 249, 81, 226, 0, 0, 0, 0, 185, 4, 200, 149, 22, 31, 0, 0, 0, 0, 114, 56, 155, 123, 0, 0, 0, 0, 77, 9, 131, 9, 0, 0, 0, 0, 61, 8, 26, 94, 0, 0, 0, 0, 185, 4, 137, 122, 184, 78, 0, 0, 0, 0, 146, 148, 73, 11, 0, 0, 0, 0, 23, 141, 192, 146, 0, 0, 0, 0, 116, 40, 191, 24, 0, 0, 0, 0, 185, 4, 247, 90, 121, 143, 0, 0, 0, 0, 3, 10, 172, 84, 0, 0, 0, 0, 93, 156, 116, 159, 0, 0, 0, 0, 215, 87, 44, 137, 0, 0, 0, 0, 185, 4, 236, 113, 160, 45, 0, 0, 0, 0, 188, 51, 152, 250, 0, 0, 0, 0, 127, 97, 21, 25, 0, 0, 0, 0, 82, 243, 231, 40, 0, 0, 0, 0, 185, 4, 235, 149, 58, 247, 0, 0, 0, 0, 242, 122, 69, 227, 0, 0, 0, 0, 124, 5, 149, 131, 0, 0, 0, 0, 38, 222, 238, 189, 0, 0, 0, 0, 185, 4, 53, 114, 222, 202, 0, 0, 0, 0, 101, 237, 133, 203, 0, 0, 0, 0, 185, 129, 153, 169, 0, 0, 0, 0, 46, 66, 52, 160, 0, 0, 0, 0, 185, 4, 227, 103, 184, 19, 0, 0, 0, 0, 172, 120, 14, 129, 0, 0, 0, 0, 186, 183, 132, 249, 0, 0, 0, 0, 64, 70, 33, 63, 0, 0, 0, 0, 185, 4, 30, 254, 225, 6, 0, 0, 0, 0, 154, 4, 6, 49, 0, 0, 0, 0, 154, 120, 230, 62, 0, 0, 0, 0, 92, 60, 165, 171, 0, 0, 0, 0, 185, 4, 138, 189, 26, 57, 0, 0, 0, 0, 71, 14, 87, 34, 0, 0, 0, 0, 55, 173, 174, 247, 0, 0, 0, 0, 128, 18, 146, 106, 0, 0, 0, 0, 185, 4, 71, 116, 143, 11, 0, 0, 0, 0, 178, 29, 8, 224, 0, 0, 0, 0, 245, 221, 239, 116, 0, 0, 0, 0, 84, 40, 212, 62, 0, 0, 0, 0, 185, 4, 183, 52, 120, 227, 0, 0, 0, 0, 92, 61, 100, 143, 0, 0, 0, 0, 208, 152, 35, 157, 0, 0, 0, 0, 70, 124, 220, 126, 0, 0, 0, 0, 185, 4, 53, 203, 142, 175, 0, 0, 0, 0, 213, 187, 46, 6, 0, 0, 0, 0, 15, 153, 87, 177, 0, 0, 0, 0, 17, 70, 3, 99, 0, 0, 0, 0, 185, 4, 34, 217, 232, 244, 0, 0, 0, 0, 9, 105, 106, 58, 0, 0, 0, 0, 57, 38, 249, 139, 0, 0, 0, 0, 169, 58, 253, 162, 0, 0, 0, 0, 185, 4, 60, 29, 80, 148, 0, 0, 0, 0, 136, 55, 39, 29, 0, 0, 0, 0, 72, 49, 179, 168, 0, 0, 0, 0, 44, 242, 153, 54, 0, 0, 0, 0, 185, 4, 204, 214, 223, 28, 0, 0, 0, 0, 213, 140, 120, 148, 0, 0, 0, 0, 238, 213, 186, 111, 0, 0, 0, 0, 160, 254, 231, 240, 0, 0, 0, 0, 185, 4, 145, 74, 63, 26, 0, 0, 0, 0, 137, 24, 177, 84, 0, 0, 0, 0, 175, 208, 9, 242, 0, 0, 0, 0, 17, 68, 53, 6, 0, 0, 0, 0, 185, 4, 132, 47, 37, 13, 0, 0, 0, 0, 242, 183, 134, 229, 0, 0, 0, 0, 122, 216, 133, 152, 0, 0, 0, 0, 85, 207, 60, 145, 0, 0, 0, 0, 185, 4, 21, 68, 42, 255, 0, 0, 0, 0, 128, 29, 163, 116, 0, 0, 0, 0, 135, 43, 242, 8, 0, 0, 0, 0, 148, 191, 118, 12, 0, 0, 0, 0, 185, 4, 132, 27, 116, 69, 0, 0, 0, 0, 146, 0, 150, 93, 0, 0, 0, 0, 122, 188, 126, 116, 0, 0, 0, 0, 247, 100, 39, 180, 0, 0, 0, 0, 185, 4, 248, 144, 162, 36, 0, 0, 0, 0, 216, 219, 44, 213, 0, 0, 0, 0, 41, 170, 166, 108, 0, 0, 0, 0, 202, 119, 10, 87, 0, 0, 0, 0, 185, 4, 6, 36, 87, 125, 0, 0, 0, 0, 57, 234, 146, 254, 0, 0, 0, 0, 168, 122, 81, 147, 0, 0, 0, 0, 150, 69, 167, 83, 0, 0, 0, 0, 185, 4, 170, 89, 87, 166, 0, 0, 0, 0, 211, 104, 56, 98, 0, 0, 0, 0, 182, 232, 194, 217, 0, 0, 0, 0, 74, 126, 177, 132, 0, 0, 0, 0, 185, 4, 175, 211, 43, 178, 0, 0, 0, 0, 219, 89, 166, 107, 0, 0, 0, 0, 69, 170, 91, 70, 0, 0, 0, 0, 135, 60, 240, 191, 0, 0, 0, 0, 185, 4, 32, 51, 42, 86, 0, 0, 0, 0, 203, 150, 42, 152, 0, 0, 0, 0, 51, 176, 66, 92, 0, 0, 0, 0, 166, 122, 202, 27, 0, 0, 0, 0, 185, 4, 197, 156, 22, 73, 0, 0, 0, 0, 200, 128, 101, 186, 0, 0, 0, 0, 29, 93, 199, 231, 0, 0, 0, 0, 206, 141, 239, 50, 0, 0, 0, 0, 185, 4, 131, 41, 247, 1, 0, 0, 0, 0, 108, 144, 81, 113, 0, 0, 0, 0, 179, 83, 122, 137, 0, 0, 0, 0, 255, 200, 188, 154, 0, 0, 0, 0, 185, 4, 121, 182, 12, 238, 0, 0, 0, 0, 66, 130, 86, 222, 0, 0, 0, 0, 74, 190, 124, 115, 0, 0, 0, 0, 68, 156, 76, 133, 0, 0, 0, 0, 185, 4, 209, 50, 218, 130, 0, 0, 0, 0, 213, 86, 176, 46, 0, 0, 0, 0, 108, 77, 112, 136, 0, 0, 0, 0, 100, 64, 1, 46, 0, 0, 0, 0, 185, 4, 54, 226, 91, 182, 0, 0, 0, 0, 89, 220, 222, 66, 0, 0, 0, 0, 124, 35, 198, 60, 0, 0, 0, 0, 62, 75, 68, 230, 0, 0, 0, 0, 185, 4, 76, 159, 143, 238, 0, 0, 0, 0, 95, 134, 112, 186, 0, 0, 0, 0, 217, 184, 207, 214, 0, 0, 0, 0, 243, 123, 29, 179, 0, 0, 0, 0, 185, 4, 163, 127, 252, 176, 0, 0, 0, 0, 106, 150, 68, 87, 0, 0, 0, 0, 153, 168, 2, 46, 0, 0, 0, 0, 63, 139, 225, 166, 0, 0, 0, 0, 185, 4, 185, 24, 232, 151, 0, 0, 0, 0, 60, 68, 55, 255, 0, 0, 0, 0, 188, 110, 226, 2, 0, 0, 0, 0, 156, 114, 85, 238, 0, 0, 0, 0, 185, 4, 31, 249, 204, 163, 0, 0, 0, 0, 80, 232, 154, 96, 0, 0, 0, 0, 37, 143, 160, 175, 0, 0, 0, 0, 146, 205, 196, 88, 0, 0, 0, 0, 185, 4, 164, 12, 95, 225, 0, 0, 0, 0, 205, 153, 196, 13, 0, 0, 0, 0, 27, 219, 146, 175, 0, 0, 0, 0, 146, 105, 157, 129, 0, 0, 0, 0, 185, 4, 115, 80, 234, 191, 0, 0, 0, 0, 118, 26, 202, 245, 0, 0, 0, 0, 5, 247, 16, 218, 0, 0, 0, 0, 30, 36, 78, 196, 0, 0, 0, 0, 185, 4, 205, 247, 175, 128, 0, 0, 0, 0, 149, 131, 58, 62, 0, 0, 0, 0, 38, 199, 202, 167, 0, 0, 0, 0, 63, 216, 70, 132, 0, 0, 0, 0, 185, 4, 44, 134, 38, 51, 0, 0, 0, 0, 30, 234, 55, 233, 0, 0, 0, 0, 174, 253, 226, 171, 0, 0, 0, 0, 27, 157, 71, 11, 0, 0, 0, 0, 185, 4, 1, 134, 181, 165, 0, 0, 0, 0, 155, 78, 188, 232, 0, 0, 0, 0, 245, 44, 158, 94, 0, 0, 0, 0, 160, 194, 30, 158, 0, 0, 0, 0, 185, 4, 209, 242, 91, 104, 0, 0, 0, 0, 0, 227, 241, 50, 0, 0, 0, 0, 251, 217, 163, 190, 0, 0, 0, 0, 23, 230, 76, 23, 0, 0, 0, 0, 185, 4, 203, 247, 123, 255, 0, 0, 0, 0, 248, 112, 137, 52, 0, 0, 0, 0, 248, 134, 200, 104, 0, 0, 0, 0, 234, 138, 143, 169, 0, 0, 0, 0, 185, 4, 248, 147, 107, 37, 0, 0, 0, 0, 119, 109, 20, 113, 0, 0, 0, 0, 40, 69, 82, 238, 0, 0, 0, 0, 173, 58, 156, 193, 0, 0, 0, 0, 185, 4, 168, 20, 37, 77, 0, 0, 0, 0, 62, 215, 195, 248, 0, 0, 0, 0, 50, 151, 231, 26, 0, 0, 0, 0, 34, 42, 240, 67, 0, 0, 0, 0, 185, 4, 11, 49, 111, 62, 0, 0, 0, 0, 148, 224, 66, 147, 0, 0, 0, 0, 65, 157, 250, 146, 0, 0, 0, 0, 195, 227, 137, 100, 0, 0, 0, 0, 185, 4, 115, 60, 224, 181, 0, 0, 0, 0, 32, 57, 107, 162, 0, 0, 0, 0, 153, 171, 178, 140, 0, 0, 0, 0, 102, 134, 102, 182, 0, 0, 0, 0, 185, 4, 189, 161, 128, 125, 0, 0, 0, 0, 178, 32, 31, 240, 0, 0, 0, 0, 159, 42, 94, 83, 0, 0, 0, 0, 111, 180, 95, 55, 0, 0, 0, 0, 185, 4, 123, 117, 241, 195, 0, 0, 0, 0, 147, 68, 126, 209, 0, 0, 0, 0, 75, 136, 177, 194, 0, 0, 0, 0, 243, 190, 118, 228, 0, 0, 0, 0, 185, 4, 12, 192, 59, 1, 0, 0, 0, 0, 121, 141, 41, 149, 0, 0, 0, 0, 17, 72, 153, 143, 0, 0, 0, 0, 178, 2, 65, 30, 0, 0, 0, 0, 185, 4, 255, 166, 7, 48, 0, 0, 0, 0, 101, 138, 224, 206, 0, 0, 0, 0, 225, 46, 59, 152, 0, 0, 0, 0, 82, 129, 89, 229, 0, 0, 0, 0, 185, 4, 155, 5, 120, 122, 0, 0, 0, 0, 70, 176, 160, 14, 0, 0, 0, 0, 212, 200, 231, 196, 0, 0, 0, 0, 69, 106, 55, 117, 0, 0, 0, 0, 185, 4, 217, 113, 223, 185, 0, 0, 0, 0, 137, 162, 104, 176, 0, 0, 0, 0, 141, 23, 252, 188, 0, 0, 0, 0, 113, 61, 5, 201, 0, 0, 0, 0, 185, 4, 20, 82, 210, 206, 0, 0, 0, 0, 155, 50, 57, 235, 0, 0, 0, 0, 31, 144, 127, 30, 0, 0, 0, 0, 239, 158, 34, 197, 0, 0, 0, 0, 185, 4, 5, 164, 3, 131, 0, 0, 0, 0, 54, 158, 190, 245, 0, 0, 0, 0, 160, 108, 35, 185, 0, 0, 0, 0, 112, 146, 71, 4, 0, 0, 0, 0, 185, 4, 162, 202, 233, 243, 0, 0, 0, 0, 135, 75, 189, 50, 0, 0, 0, 0, 19, 40, 60, 10, 0, 0, 0, 0, 56, 6, 149, 226, 0, 0, 0, 0, 185, 4, 66, 146, 1, 5, 0, 0, 0, 0, 9, 38, 87, 174, 0, 0, 0, 0, 52, 215, 148, 207, 0, 0, 0, 0, 253, 125, 229, 2, 0, 0, 0, 0, 185, 4, 82, 132, 73, 171, 0, 0, 0, 0, 241, 218, 51, 199, 0, 0, 0, 0, 247, 39, 100, 20, 0, 0, 0, 0, 53, 81, 16, 204, 0, 0, 0, 0, 185, 4, 155, 216, 244, 248, 0, 0, 0, 0, 70, 173, 227, 179, 0, 0, 0, 0, 19, 126, 240, 230, 0, 0, 0, 0, 2, 157, 172, 112, 0, 0, 0, 0, 185, 4, 163, 238, 16, 165, 0, 0, 0, 0, 223, 188, 231, 182, 0, 0, 0, 0, 143, 186, 237, 100, 0, 0, 0, 0, 200, 165, 121, 91, 0, 0, 0, 0, 185, 4, 148, 83, 76, 105, 0, 0, 0, 0, 243, 153, 123, 155, 0, 0, 0, 0, 51, 175, 181, 81, 0, 0, 0, 0, 192, 248, 181, 196, 0, 0, 0, 0, 185, 4, 9, 128, 30, 62, 0, 0, 0, 0, 55, 176, 36, 235, 0, 0, 0, 0, 248, 21, 201, 147, 0, 0, 0, 0, 167, 232, 183, 211, 0, 0, 0, 0, 185, 4, 0, 58, 235, 131, 0, 0, 0, 0, 197, 182, 248, 136, 0, 0, 0, 0, 126, 158, 213, 109, 0, 0, 0, 0, 220, 90, 211, 63, 0, 0, 0, 0, 185, 4, 147, 226, 117, 56, 0, 0, 0, 0, 103, 253, 97, 130, 0, 0, 0, 0, 121, 2, 102, 224, 0, 0, 0, 0, 188, 101, 4, 70, 0, 0, 0, 0, 185, 4, 81, 18, 167, 55, 0, 0, 0, 0, 51, 236, 99, 145, 0, 0, 0, 0, 0, 141, 226, 155, 0, 0, 0, 0, 244, 196, 120, 54, 0, 0, 0, 0, 185, 4, 137, 243, 139, 249, 0, 0, 0, 0, 190, 127, 73, 88, 0, 0, 0, 0, 169, 29, 193, 81, 0, 0, 0, 0, 42, 245, 189, 138, 0, 0, 0, 0, 185, 4, 133, 27, 70, 43, 0, 0, 0, 0, 198, 128, 143, 152, 0, 0, 0, 0, 27, 230, 48, 40, 0, 0, 0, 0, 7, 229, 208, 41, 0, 0, 0, 0, 185, 4, 78, 152, 122, 182, 0, 0, 0, 0, 47, 141, 55, 247, 0, 0, 0, 0, 239, 112, 58, 36, 0, 0, 0, 0, 227, 53, 237, 64, 0, 0, 0, 0, 185, 4, 30, 53, 17, 98, 0, 0, 0, 0, 45, 58, 113, 59, 0, 0, 0, 0, 50, 29, 206, 37, 0, 0, 0, 0, 74, 13, 46, 203, 0, 0, 0, 0, 185, 4, 169, 26, 221, 162, 0, 0, 0, 0, 255, 17, 77, 100, 0, 0, 0, 0, 137, 71, 111, 202, 0, 0, 0, 0, 125, 115, 243, 168, 0, 0, 0, 0, 185, 4, 53, 170, 123, 158, 0, 0, 0, 0, 232, 64, 225, 205, 0, 0, 0, 0, 123, 174, 90, 167, 0, 0, 0, 0, 53, 197, 199, 184, 0, 0, 0, 0, 185, 4, 237, 203, 133, 229, 0, 0, 0, 0, 39, 205, 231, 130, 0, 0, 0, 0, 119, 48, 139, 83, 0, 0, 0, 0, 219, 5, 122, 191, 0, 0, 0, 0, 185, 4, 121, 34, 95, 175, 0, 0, 0, 0, 124, 221, 206, 58, 0, 0, 0, 0, 18, 217, 166, 62, 0, 0, 0, 0, 160, 23, 80, 255, 0, 0, 0, 0, 185, 4, 124, 44, 211, 238, 0, 0, 0, 0, 107, 149, 155, 75, 0, 0, 0, 0, 201, 17, 221, 54, 0, 0, 0, 0, 100, 53, 229, 28, 0, 0, 0, 0, 185, 4, 173, 147, 184, 0, 0, 0, 0, 0, 2, 225, 227, 55, 0, 0, 0, 0, 163, 189, 167, 143, 0, 0, 0, 0, 13, 240, 11, 236, 0, 0, 0, 0, 185, 4, 115, 205, 239, 82, 0, 0, 0, 0, 231, 167, 76, 90, 0, 0, 0, 0, 53, 212, 243, 233, 0, 0, 0, 0, 9, 63, 110, 221, 0, 0, 0, 0, 185, 4, 215, 128, 239, 108, 0, 0, 0, 0, 28, 230, 168, 250, 0, 0, 0, 0, 142, 238, 216, 37, 0, 0, 0, 0, 41, 44, 124, 157, 0, 0, 0, 0, 185, 4, 160, 198, 212, 21, 0, 0, 0, 0, 188, 0, 229, 129, 0, 0, 0, 0, 50, 56, 51, 236, 0, 0, 0, 0, 232, 143, 4, 171, 0, 0, 0, 0, 185, 4, 35, 34, 117, 183, 0, 0, 0, 0, 123, 67, 227, 52, 0, 0, 0, 0, 153, 100, 221, 68, 0, 0, 0, 0, 36, 222, 122, 196, 0, 0, 0, 0, 185, 4, 169, 219, 203, 213, 0, 0, 0, 0, 155, 104, 160, 181, 0, 0, 0, 0, 107, 83, 87, 199, 0, 0, 0, 0, 237, 244, 104, 80, 0, 0, 0, 0, 185, 4, 39, 26, 4, 158, 0, 0, 0, 0, 88, 82, 182, 54, 0, 0, 0, 0, 78, 124, 142, 50, 0, 0, 0, 0, 45, 6, 243, 99, 0, 0, 0, 0, 185, 4, 116, 200, 182, 119, 0, 0, 0, 0, 15, 144, 11, 147, 0, 0, 0, 0, 210, 201, 164, 0, 0, 0, 0, 0, 191, 56, 205, 8, 0, 0, 0, 0, 185, 4, 84, 52, 171, 146, 0, 0, 0, 0, 148, 228, 143, 143, 0, 0, 0, 0, 121, 3, 125, 62, 0, 0, 0, 0, 205, 19, 144, 12, 0, 0, 0, 0, 185, 4, 80, 63, 26, 95, 0, 0, 0, 0, 202, 117, 98, 33, 0, 0, 0, 0, 136, 230, 231, 215, 0, 0, 0, 0, 23, 120, 45, 130, 0, 0, 0, 0, 185, 4, 74, 198, 241, 26, 0, 0, 0, 0, 10, 246, 171, 160, 0, 0, 0, 0, 65, 223, 179, 160, 0, 0, 0, 0, 149, 157, 48, 198, 0, 0, 0, 0, 185, 4, 13, 92, 154, 78, 0, 0, 0, 0, 226, 253, 179, 224, 0, 0, 0, 0, 199, 251, 203, 224, 0, 0, 0, 0, 175, 3, 111, 115, 0, 0, 0, 0, 185, 4, 214, 70, 173, 143, 0, 0, 0, 0, 115, 24, 73, 138, 0, 0, 0, 0, 96, 139, 211, 7, 0, 0, 0, 0, 103, 181, 56, 48, 0, 0, 0, 0, 185, 4, 77, 126, 1, 14, 0, 0, 0, 0, 241, 50, 189, 229, 0, 0, 0, 0, 202, 208, 3, 200, 0, 0, 0, 0, 52, 184, 150, 170, 0, 0, 0, 0, 185, 4, 198, 225, 54, 139, 0, 0, 0, 0, 63, 103, 66, 190, 0, 0, 0, 0, 139, 46, 141, 88, 0, 0, 0, 0, 176, 77, 55, 245, 0, 0, 0, 0, 185, 4, 24, 142, 73, 28, 0, 0, 0, 0, 250, 99, 179, 8, 0, 0, 0, 0, 78, 123, 180, 28, 0, 0, 0, 0, 200, 199, 230, 231, 0, 0, 0, 0, 185, 4, 22, 164, 105, 38, 0, 0, 0, 0, 27, 134, 145, 17, 0, 0, 0, 0, 37, 73, 15, 229, 0, 0, 0, 0, 51, 239, 37, 210, 0, 0, 0, 0, 185, 4, 252, 235, 163, 71, 0, 0, 0, 0, 74, 220, 189, 91, 0, 0, 0, 0, 29, 131, 188, 55, 0, 0, 0, 0, 78, 211, 171, 29, 0, 0, 0, 0, 185, 4, 10, 239, 201, 94, 0, 0, 0, 0, 140, 132, 97, 237, 0, 0, 0, 0, 239, 27, 199, 125, 0, 0, 0, 0, 217, 67, 54, 139, 0, 0, 0, 0, 185, 4, 83, 157, 157, 119, 0, 0, 0, 0, 177, 228, 174, 132, 0, 0, 0, 0, 183, 163, 46, 203, 0, 0, 0, 0, 251, 112, 166, 211, 0, 0, 0, 0, 185, 4, 149, 43, 157, 98, 0, 0, 0, 0, 146, 84, 94, 1, 0, 0, 0, 0, 73, 73, 130, 179, 0, 0, 0, 0, 5, 220, 38, 114, 0, 0, 0, 0, 185, 4, 112, 130, 36, 200, 0, 0, 0, 0, 243, 137, 121, 71, 0, 0, 0, 0, 253, 237, 202, 50, 0, 0, 0, 0, 116, 24, 205, 95, 0, 0, 0, 0, 185, 4, 225, 190, 136, 23, 0, 0, 0, 0, 91, 115, 56, 60, 0, 0, 0, 0, 187, 38, 43, 188, 0, 0, 0, 0, 120, 27, 191, 127, 0, 0, 0, 0, 185, 4, 235, 13, 241, 181, 0, 0, 0, 0, 196, 250, 245, 192, 0, 0, 0, 0, 226, 226, 188, 251, 0, 0, 0, 0, 200, 117, 186, 247, 0, 0, 0, 0, 185, 4, 188, 240, 122, 238, 0, 0, 0, 0, 224, 32, 158, 197, 0, 0, 0, 0, 9, 187, 106, 241, 0, 0, 0, 0, 35, 237, 181, 100, 0, 0, 0, 0, 185, 4, 62, 182, 209, 209, 0, 0, 0, 0, 248, 234, 234, 96, 0, 0, 0, 0, 241, 146, 2, 212, 0, 0, 0, 0, 2, 147, 98, 182, 0, 0, 0, 0, 185, 4, 23, 224, 103, 143, 0, 0, 0, 0, 130, 23, 129, 36, 0, 0, 0, 0, 218, 123, 107, 71, 0, 0, 0, 0, 56, 17, 234, 76, 0, 0, 0, 0, 185, 4, 63, 236, 180, 79, 0, 0, 0, 0, 194, 241, 9, 125, 0, 0, 0, 0, 85, 109, 90, 130, 0, 0, 0, 0, 71, 160, 37, 4, 0, 0, 0, 0, 185, 4, 223, 135, 144, 243, 0, 0, 0, 0, 8, 6, 212, 248, 0, 0, 0, 0, 48, 252, 31, 144, 0, 0, 0, 0, 249, 80, 140, 110, 0, 0, 0, 0, 185, 4, 190, 69, 111, 186, 0, 0, 0, 0, 149, 189, 141, 234, 0, 0, 0, 0, 50, 124, 244, 236, 0, 0, 0, 0, 1, 28, 224, 198, 0, 0, 0, 0, 185, 4, 251, 230, 253, 239, 0, 0, 0, 0, 223, 227, 59, 219, 0, 0, 0, 0, 167, 59, 177, 36, 0, 0, 0, 0, 109, 159, 6, 152, 0, 0, 0, 0, 185, 4, 127, 10, 8, 66, 0, 0, 0, 0, 172, 178, 244, 35, 0, 0, 0, 0, 196, 162, 197, 22, 0, 0, 0, 0, 180, 243, 11, 216, 0, 0, 0, 0, 185, 4, 119, 189, 209, 216, 0, 0, 0, 0, 163, 190, 248, 92, 0, 0, 0, 0, 154, 127, 117, 79, 0, 0, 0, 0, 242, 163, 6, 48, 0, 0, 0, 0, 185, 4, 203, 238, 90, 75, 0, 0, 0, 0, 61, 56, 152, 140, 0, 0, 0, 0, 140, 198, 244, 15, 0, 0, 0, 0, 176, 104, 175, 23, 0, 0, 0, 0, 185, 4, 106, 145, 20, 209, 0, 0, 0, 0, 7, 149, 138, 155, 0, 0, 0, 0, 102, 159, 186, 157, 0, 0, 0, 0, 23, 61, 51, 197, 0, 0, 0, 0, 185, 4, 129, 14, 81, 225, 0, 0, 0, 0, 80, 172, 120, 54, 0, 0, 0, 0, 18, 126, 80, 7, 0, 0, 0, 0, 242, 125, 128, 179, 0, 0, 0, 0, 185, 4, 237, 111, 144, 151, 0, 0, 0, 0, 210, 15, 192, 54, 0, 0, 0, 0, 158, 135, 37, 173, 0, 0, 0, 0, 162, 227, 64, 139, 0, 0, 0, 0, 185, 4, 65, 171, 231, 0, 0, 0, 0, 0, 111, 144, 229, 32, 0, 0, 0, 0, 8, 15, 220, 134, 0, 0, 0, 0, 215, 54, 66, 120, 0, 0, 0, 0, 185, 4, 93, 249, 39, 94, 0, 0, 0, 0, 198, 209, 106, 144, 0, 0, 0, 0, 145, 231, 104, 255, 0, 0, 0, 0, 177, 65, 250, 250, 0, 0, 0, 0, 185, 4, 233, 32, 182, 250, 0, 0, 0, 0, 90, 0, 8, 54, 0, 0, 0, 0, 216, 153, 249, 180, 0, 0, 0, 0, 150, 57, 250, 27, 0, 0, 0, 0, 185, 4, 136, 26, 62, 252, 0, 0, 0, 0, 54, 203, 220, 136, 0, 0, 0, 0, 64, 101, 180, 220, 0, 0, 0, 0, 198, 105, 230, 44, 0, 0, 0, 0, 185, 4, 248, 3, 91, 10, 0, 0, 0, 0, 201, 170, 32, 77, 0, 0, 0, 0, 181, 19, 85, 127, 0, 0, 0, 0, 211, 162, 139, 163, 0, 0, 0, 0, 185, 4, 16, 39, 164, 151, 0, 0, 0, 0, 201, 170, 155, 225, 0, 0, 0, 0, 187, 251, 206, 46, 0, 0, 0, 0, 41, 24, 145, 198, 0, 0, 0, 0, 185, 4, 144, 30, 63, 52, 0, 0, 0, 0, 255, 5, 128, 253, 0, 0, 0, 0, 222, 19, 224, 200, 0, 0, 0, 0, 169, 11, 63, 99, 0, 0, 0, 0, 185, 4, 45, 48, 211, 245, 0, 0, 0, 0, 192, 112, 112, 221, 0, 0, 0, 0, 94, 101, 175, 217, 0, 0, 0, 0, 247, 114, 122, 23, 0, 0, 0, 0, 185, 4, 243, 234, 88, 90, 0, 0, 0, 0, 241, 192, 138, 99, 0, 0, 0, 0, 37, 21, 231, 217, 0, 0, 0, 0, 230, 15, 129, 25, 0, 0, 0, 0, 185, 4, 47, 104, 180, 36, 0, 0, 0, 0, 6, 142, 154, 196, 0, 0, 0, 0, 5, 203, 164, 178, 0, 0, 0, 0, 18, 246, 208, 188, 0, 0, 0, 0, 185, 4, 158, 106, 189, 43, 0, 0, 0, 0, 191, 18, 227, 3, 0, 0, 0, 0, 193, 149, 228, 113, 0, 0, 0, 0, 134, 148, 230, 194, 0, 0, 0, 0, 185, 4, 90, 207, 67, 155, 0, 0, 0, 0, 79, 12, 80, 108, 0, 0, 0, 0, 19, 104, 94, 102, 0, 0, 0, 0, 30, 240, 79, 116, 0, 0, 0, 0, 185, 4, 43, 154, 249, 135, 0, 0, 0, 0, 88, 165, 77, 147, 0, 0, 0, 0, 162, 184, 240, 217, 0, 0, 0, 0, 213, 251, 228, 20, 0, 0, 0, 0, 185, 4, 33, 235, 40, 182, 0, 0, 0, 0, 174, 141, 19, 252, 0, 0, 0, 0, 23, 110, 252, 55, 0, 0, 0, 0, 74, 178, 15, 3, 0, 0, 0, 0, 185, 4, 228, 76, 42, 4, 0, 0, 0, 0, 148, 148, 150, 61, 0, 0, 0, 0, 65, 35, 149, 229, 0, 0, 0, 0, 175, 74, 169, 200, 0, 0, 0, 0, 185, 4, 28, 69, 11, 47, 0, 0, 0, 0, 241, 237, 92, 216, 0, 0, 0, 0, 48, 204, 48, 99, 0, 0, 0, 0, 22, 232, 21, 125, 0, 0, 0, 0, 185, 4, 82, 196, 213, 168, 0, 0, 0, 0, 51, 75, 73, 61, 0, 0, 0, 0, 184, 104, 159, 25, 0, 0, 0, 0, 53, 52, 44, 232, 0, 0, 0, 0, 185, 4, 250, 26, 66, 28, 0, 0, 0, 0, 171, 222, 185, 129, 0, 0, 0, 0, 44, 215, 160, 82, 0, 0, 0, 0, 109, 96, 48, 45, 0, 0, 0, 0, 185, 4, 151, 90, 121, 72, 0, 0, 0, 0, 37, 149, 203, 57, 0, 0, 0, 0, 164, 53, 225, 134, 0, 0, 0, 0, 24, 175, 100, 221, 0, 0, 0, 0, 185, 4, 167, 211, 178, 233, 0, 0, 0, 0, 203, 110, 199, 157, 0, 0, 0, 0, 250, 174, 205, 239, 0, 0, 0, 0, 212, 37, 48, 50, 0, 0, 0, 0, 185, 4, 180, 246, 224, 231, 0, 0, 0, 0, 109, 49, 180, 124, 0, 0, 0, 0, 184, 253, 156, 190, 0, 0, 0, 0, 178, 245, 150, 54, 0, 0, 0, 0, 185, 4, 208, 27, 40, 205, 0, 0, 0, 0, 128, 48, 63, 25, 0, 0, 0, 0, 166, 101, 228, 116, 0, 0, 0, 0, 91, 229, 127, 227, 0, 0, 0, 0, 185, 4, 64, 86, 184, 126, 0, 0, 0, 0, 118, 219, 49, 40, 0, 0, 0, 0, 136, 240, 33, 222, 0, 0, 0, 0, 129, 25, 208, 192, 0, 0, 0, 0, 185, 4, 146, 109, 4, 146, 0, 0, 0, 0, 112, 31, 185, 142, 0, 0, 0, 0, 155, 91, 255, 43, 0, 0, 0, 0, 248, 217, 134, 252, 0, 0, 0, 0, 185, 4, 42, 200, 213, 168, 0, 0, 0, 0, 78, 175, 109, 49, 0, 0, 0, 0, 146, 190, 239, 187, 0, 0, 0, 0, 96, 11, 228, 117, 0, 0, 0, 0, 185, 4, 121, 34, 170, 62, 0, 0, 0, 0, 165, 75, 209, 132, 0, 0, 0, 0, 180, 179, 252, 46, 0, 0, 0, 0, 113, 143, 170, 74, 0, 0, 0, 0, 185, 4, 161, 152, 113, 180, 0, 0, 0, 0, 70, 202, 87, 17, 0, 0, 0, 0, 204, 136, 127, 136, 0, 0, 0, 0, 157, 222, 67, 4, 0, 0, 0, 0, 185, 4, 47, 226, 13, 200, 0, 0, 0, 0, 152, 155, 148, 215, 0, 0, 0, 0, 76, 1, 145, 85, 0, 0, 0, 0, 153, 50, 7, 65, 0, 0, 0, 0, 185, 4, 36, 132, 133, 112, 0, 0, 0, 0, 236, 237, 233, 0, 0, 0, 0, 0, 122, 247, 140, 123, 0, 0, 0, 0, 191, 227, 155, 234, 0, 0, 0, 0, 185, 4, 47, 173, 42, 23, 0, 0, 0, 0, 251, 250, 7, 169, 0, 0, 0, 0, 42, 1, 151, 80, 0, 0, 0, 0, 123, 35, 130, 157, 0, 0, 0, 0, 185, 4, 188, 216, 146, 51, 0, 0, 0, 0, 35, 166, 117, 37, 0, 0, 0, 0, 88, 64, 175, 210, 0, 0, 0, 0, 124, 251, 95, 15, 0, 0, 0, 0, 185, 4, 137, 118, 100, 140, 0, 0, 0, 0, 119, 204, 144, 151, 0, 0, 0, 0, 85, 184, 116, 138, 0, 0, 0, 0, 115, 93, 223, 217, 0, 0, 0, 0, 185, 4, 21, 238, 124, 201, 0, 0, 0, 0, 139, 204, 19, 117, 0, 0, 0, 0, 16, 102, 42, 142, 0, 0, 0, 0, 185, 151, 7, 214, 0, 0, 0, 0, 185, 4, 167, 93, 124, 12, 0, 0, 0, 0, 189, 59, 102, 184, 0, 0, 0, 0, 132, 211, 128, 250, 0, 0, 0, 0, 151, 249, 223, 213, 0, 0, 0, 0, 185, 4, 64, 137, 134, 156, 0, 0, 0, 0, 131, 0, 6, 177, 0, 0, 0, 0, 184, 110, 99, 47, 0, 0, 0, 0, 179, 194, 82, 94, 0, 0, 0, 0, 185, 4, 8, 9, 172, 184, 0, 0, 0, 0, 129, 183, 150, 141, 0, 0, 0, 0, 100, 127, 24, 164, 0, 0, 0, 0, 170, 18, 144, 184, 0, 0, 0, 0, 185, 4, 202, 168, 208, 69, 0, 0, 0, 0, 88, 182, 230, 112, 0, 0, 0, 0, 189, 38, 205, 231, 0, 0, 0, 0, 37, 245, 232, 185, 0, 0, 0, 0, 185, 4, 38, 142, 77, 219, 0, 0, 0, 0, 29, 255, 151, 203, 0, 0, 0, 0, 6, 156, 183, 253, 0, 0, 0, 0, 57, 222, 213, 89, 0, 0, 0, 0, 185, 4, 200, 97, 69, 151, 0, 0, 0, 0, 46, 148, 186, 230, 0, 0, 0, 0, 119, 21, 118, 53, 0, 0, 0, 0, 117, 160, 132, 160, 0, 0, 0, 0, 185, 4, 208, 138, 230, 0, 0, 0, 0, 0, 31, 253, 40, 135, 0, 0, 0, 0, 5, 50, 243, 231, 0, 0, 0, 0, 103, 37, 205, 73, 0, 0, 0, 0, 185, 4, 239, 150, 105, 68, 0, 0, 0, 0, 37, 71, 208, 163, 0, 0, 0, 0, 171, 127, 30, 165, 0, 0, 0, 0, 147, 45, 100, 177, 0, 0, 0, 0, 185, 4, 204, 151, 211, 26, 0, 0, 0, 0, 41, 182, 137, 94, 0, 0, 0, 0, 81, 48, 13, 63, 0, 0, 0, 0, 167, 200, 175, 231, 0, 0, 0, 0, 185, 4, 123, 7, 165, 126, 0, 0, 0, 0, 52, 46, 227, 139, 0, 0, 0, 0, 155, 183, 95, 124, 0, 0, 0, 0, 215, 30, 114, 219, 0, 0, 0, 0, 185, 4, 72, 14, 7, 143, 0, 0, 0, 0, 154, 27, 222, 10, 0, 0, 0, 0, 115, 88, 239, 104, 0, 0, 0, 0, 59, 137, 166, 111, 0, 0, 0, 0, 185, 4, 196, 158, 241, 210, 0, 0, 0, 0, 246, 234, 40, 106, 0, 0, 0, 0, 105, 210, 149, 2, 0, 0, 0, 0, 128, 38, 175, 1, 0, 0, 0, 0, 185, 4, 223, 108, 72, 242, 0, 0, 0, 0, 67, 9, 248, 210, 0, 0, 0, 0, 16, 117, 237, 235, 0, 0, 0, 0, 29, 211, 49, 233, 0, 0, 0, 0, 185, 4, 231, 71, 202, 9, 0, 0, 0, 0, 237, 244, 226, 123, 0, 0, 0, 0, 40, 0, 103, 198, 0, 0, 0, 0, 3, 112, 237, 178, 0, 0, 0, 0, 185, 4, 255, 135, 147, 240, 0, 0, 0, 0, 143, 201, 90, 88, 0, 0, 0, 0, 9, 26, 197, 18, 0, 0, 0, 0, 55, 89, 54, 128, 0, 0, 0, 0, 185, 4, 145, 163, 235, 113, 0, 0, 0, 0, 82, 177, 63, 88, 0, 0, 0, 0, 91, 101, 167, 213, 0, 0, 0, 0, 56, 70, 196, 109, 0, 0, 0, 0, 185, 4, 233, 92, 177, 83, 0, 0, 0, 0, 192, 176, 157, 68, 0, 0, 0, 0, 5, 115, 188, 45, 0, 0, 0, 0, 60, 124, 18, 139, 0, 0, 0, 0, 185, 4, 54, 127, 192, 86, 0, 0, 0, 0, 55, 141, 255, 44, 0, 0, 0, 0, 227, 250, 89, 40, 0, 0, 0, 0, 251, 156, 183, 173, 0, 0, 0, 0, 185, 4, 98, 161, 249, 64, 0, 0, 0, 0, 170, 36, 224, 71, 0, 0, 0, 0, 230, 216, 1, 39, 0, 0, 0, 0, 118, 83, 72, 192, 0, 0, 0, 0, 185, 4, 27, 109, 57, 236, 0, 0, 0, 0, 169, 57, 57, 78, 0, 0, 0, 0, 114, 193, 113, 233, 0, 0, 0, 0, 119, 35, 163, 36, 0, 0, 0, 0, 185, 4, 153, 135, 250, 218, 0, 0, 0, 0, 7, 209, 123, 66, 0, 0, 0, 0, 3, 177, 6, 222, 0, 0, 0, 0, 147, 21, 218, 227, 0, 0, 0, 0, 185, 4, 42, 195, 2, 81, 0, 0, 0, 0, 111, 22, 222, 178, 0, 0, 0, 0, 149, 145, 118, 145, 0, 0, 0, 0, 209, 250, 57, 4, 0, 0, 0, 0, 185, 4, 183, 228, 194, 100, 0, 0, 0, 0, 76, 114, 15, 38, 0, 0, 0, 0, 199, 170, 105, 0, 0, 0, 0, 0, 197, 193, 3, 168, 0, 0, 0, 0, 185, 4, 168, 219, 45, 22, 0, 0, 0, 0, 211, 38, 112, 94, 0, 0, 0, 0, 196, 54, 131, 175, 0, 0, 0, 0, 205, 94, 195, 79, 0, 0, 0, 0, 185, 4, 15, 198, 36, 64, 0, 0, 0, 0, 236, 245, 105, 111, 0, 0, 0, 0, 162, 73, 67, 179, 0, 0, 0, 0, 182, 142, 194, 117, 0, 0, 0, 0, 185, 4, 85, 154, 143, 179, 0, 0, 0, 0, 176, 42, 169, 51, 0, 0, 0, 0, 176, 90, 116, 102, 0, 0, 0, 0, 38, 134, 146, 8, 0, 0, 0, 0, 185, 4, 61, 133, 174, 72, 0, 0, 0, 0, 200, 249, 245, 69, 0, 0, 0, 0, 126, 21, 187, 125, 0, 0, 0, 0, 191, 237, 121, 164, 0, 0, 0, 0, 185, 4, 32, 152, 77, 48, 0, 0, 0, 0, 126, 234, 155, 160, 0, 0, 0, 0, 241, 58, 3, 185, 0, 0, 0, 0, 68, 140, 179, 100, 0, 0, 0, 0, 185, 4, 33, 5, 32, 221, 0, 0, 0, 0, 37, 228, 49, 225, 0, 0, 0, 0, 226, 122, 161, 172, 0, 0, 0, 0, 143, 188, 126, 7, 0, 0, 0, 0, 185, 4, 146, 29, 153, 233, 0, 0, 0, 0, 223, 132, 242, 254, 0, 0, 0, 0, 69, 166, 12, 193, 0, 0, 0, 0, 39, 123, 191, 174, 0, 0, 0, 0, 185, 4, 183, 120, 73, 76, 0, 0, 0, 0, 164, 44, 32, 77, 0, 0, 0, 0, 44, 218, 68, 249, 0, 0, 0, 0, 167, 210, 130, 193, 0, 0, 0, 0, 185, 4, 165, 178, 178, 154, 0, 0, 0, 0, 65, 143, 132, 63, 0, 0, 0, 0, 94, 6, 207, 223, 0, 0, 0, 0, 238, 252, 121, 157, 0, 0, 0, 0, 185, 4, 229, 195, 93, 32, 0, 0, 0, 0, 110, 150, 99, 37, 0, 0, 0, 0, 87, 65, 247, 223, 0, 0, 0, 0, 18, 118, 160, 240, 0, 0, 0, 0, 185, 4, 158, 83, 55, 114, 0, 0, 0, 0, 138, 251, 48, 226, 0, 0, 0, 0, 210, 122, 51, 5, 0, 0, 0, 0, 154, 115, 55, 127, 0, 0, 0, 0, 185, 4, 71, 8, 76, 12, 0, 0, 0, 0, 74, 141, 101, 183, 0, 0, 0, 0, 238, 202, 192, 159, 0, 0, 0, 0, 194, 214, 106, 142, 0, 0, 0, 0, 185, 4, 43, 78, 74, 242, 0, 0, 0, 0, 170, 6, 153, 10, 0, 0, 0, 0, 14, 112, 70, 112, 0, 0, 0, 0, 143, 193, 71, 87, 0, 0, 0, 0, 185, 4, 161, 35, 84, 151, 0, 0, 0, 0, 43, 29, 219, 37, 0, 0, 0, 0, 10, 129, 188, 143, 0, 0, 0, 0, 160, 11, 238, 103, 0, 0, 0, 0, 185, 4, 165, 45, 169, 47, 0, 0, 0, 0, 6, 126, 169, 35, 0, 0, 0, 0, 198, 215, 255, 231, 0, 0, 0, 0, 148, 60, 78, 59, 0, 0, 0, 0, 185, 4, 81, 63, 125, 248, 0, 0, 0, 0, 34, 244, 140, 165, 0, 0, 0, 0, 11, 195, 169, 164, 0, 0, 0, 0, 191, 88, 223, 37, 0, 0, 0, 0, 185, 4, 254, 144, 199, 63, 0, 0, 0, 0, 19, 198, 43, 242, 0, 0, 0, 0, 116, 226, 48, 203, 0, 0, 0, 0, 20, 169, 196, 156, 0, 0, 0, 0, 185, 4, 231, 119, 75, 231, 0, 0, 0, 0, 142, 22, 16, 208, 0, 0, 0, 0, 213, 89, 242, 184, 0, 0, 0, 0, 28, 117, 219, 66, 0, 0, 0, 0, 185, 4, 204, 210, 182, 86, 0, 0, 0, 0, 179, 60, 216, 195, 0, 0, 0, 0, 230, 240, 45, 82, 0, 0, 0, 0, 80, 4, 79, 136, 0, 0, 0, 0, 185, 4, 139, 81, 9, 221, 0, 0, 0, 0, 237, 13, 114, 140, 0, 0, 0, 0, 70, 48, 238, 51, 0, 0, 0, 0, 69, 166, 149, 27, 0, 0, 0, 0, 185, 4, 9, 169, 15, 137, 0, 0, 0, 0, 211, 10, 25, 208, 0, 0, 0, 0, 166, 181, 231, 108, 0, 0, 0, 0, 128, 140, 27, 234, 0, 0, 0, 0, 185, 4, 177, 25, 37, 58, 0, 0, 0, 0, 49, 223, 91, 38, 0, 0, 0, 0, 242, 221, 140, 178, 0, 0, 0, 0, 43, 233, 223, 153, 0, 0, 0, 0, 185, 4, 55, 7, 2, 104, 0, 0, 0, 0, 65, 45, 119, 42, 0, 0, 0, 0, 85, 219, 41, 51, 0, 0, 0, 0, 209, 121, 8, 23, 0, 0, 0, 0, 185, 4, 137, 250, 251, 168, 0, 0, 0, 0, 88, 58, 130, 221, 0, 0, 0, 0, 153, 206, 40, 14, 0, 0, 0, 0, 52, 59, 142, 114, 0, 0, 0, 0, 185, 4, 217, 205, 119, 108, 0, 0, 0, 0, 120, 48, 212, 129, 0, 0, 0, 0, 92, 205, 231, 73, 0, 0, 0, 0, 28, 85, 97, 64, 0, 0, 0, 0, 185, 4, 34, 204, 231, 104, 0, 0, 0, 0, 226, 188, 75, 74, 0, 0, 0, 0, 161, 180, 159, 239, 0, 0, 0, 0, 192, 10, 3, 185, 0, 0, 0, 0, 185, 4, 96, 143, 112, 130, 0, 0, 0, 0, 171, 245, 41, 81, 0, 0, 0, 0, 58, 209, 116, 132, 0, 0, 0, 0, 159, 159, 185, 198, 0, 0, 0, 0, 185, 4, 186, 118, 210, 251, 0, 0, 0, 0, 108, 140, 181, 15, 0, 0, 0, 0, 50, 1, 202, 33, 0, 0, 0, 0, 41, 2, 2, 199, 0, 0, 0, 0, 185, 4, 110, 212, 184, 146, 0, 0, 0, 0, 75, 126, 217, 143, 0, 0, 0, 0, 92, 195, 253, 236, 0, 0, 0, 0, 218, 96, 96, 97, 0, 0, 0, 0, 185, 4, 190, 162, 204, 114, 0, 0, 0, 0, 44, 25, 221, 28, 0, 0, 0, 0, 73, 114, 191, 24, 0, 0, 0, 0, 121, 170, 56, 97, 0, 0, 0, 0, 185, 4, 42, 3, 241, 82, 0, 0, 0, 0, 105, 75, 131, 114, 0, 0, 0, 0, 158, 45, 126, 163, 0, 0, 0, 0, 42, 10, 138, 91, 0, 0, 0, 0, 185, 4, 14, 146, 194, 249, 0, 0, 0, 0, 196, 26, 18, 226, 0, 0, 0, 0, 223, 158, 213, 225, 0, 0, 0, 0, 75, 114, 237, 48, 0, 0, 0, 0, 185, 4, 212, 96, 90, 180, 0, 0, 0, 0, 172, 31, 186, 196, 0, 0, 0, 0, 171, 40, 170, 105, 0, 0, 0, 0, 84, 226, 140, 45, 0, 0, 0, 0, 185, 4, 156, 241, 243, 7, 0, 0, 0, 0, 10, 185, 83, 104, 0, 0, 0, 0, 54, 9, 56, 192, 0, 0, 0, 0, 153, 85, 75, 129, 0, 0, 0, 0, 185, 4, 253, 198, 118, 236, 0, 0, 0, 0, 116, 29, 40, 195, 0, 0, 0, 0, 147, 176, 18, 169, 0, 0, 0, 0, 98, 108, 254, 29, 0, 0, 0, 0, 185, 4, 194, 188, 182, 89, 0, 0, 0, 0, 164, 243, 51, 105, 0, 0, 0, 0, 226, 43, 219, 129, 0, 0, 0, 0, 59, 82, 115, 5, 0, 0, 0, 0, 185, 4, 101, 160, 197, 168, 0, 0, 0, 0, 149, 76, 149, 43, 0, 0, 0, 0, 184, 172, 6, 70, 0, 0, 0, 0, 35, 85, 169, 50, 0, 0, 0, 0, 185, 4, 128, 61, 26, 82, 0, 0, 0, 0, 210, 173, 154, 70, 0, 0, 0, 0, 81, 211, 178, 95, 0, 0, 0, 0, 211, 219, 65, 153, 0, 0, 0, 0, 185, 4, 228, 206, 239, 188, 0, 0, 0, 0, 20, 112, 73, 219, 0, 0, 0, 0, 145, 129, 251, 201, 0, 0, 0, 0, 46, 27, 97, 7, 0, 0, 0, 0, 185, 4, 70, 91, 33, 43, 0, 0, 0, 0, 137, 92, 35, 107, 0, 0, 0, 0, 34, 244, 0, 106, 0, 0, 0, 0, 2, 236, 108, 41, 0, 0, 0, 0, 185, 4, 254, 42, 161, 242, 0, 0, 0, 0, 230, 92, 103, 88, 0, 0, 0, 0, 136, 167, 198, 110, 0, 0, 0, 0, 244, 203, 36, 118, 0, 0, 0, 0, 185, 4, 224, 88, 209, 130, 0, 0, 0, 0, 165, 131, 205, 75, 0, 0, 0, 0, 92, 38, 67, 227, 0, 0, 0, 0, 191, 77, 32, 85, 0, 0, 0, 0, 185, 4, 4, 145, 161, 43, 0, 0, 0, 0, 157, 182, 250, 97, 0, 0, 0, 0, 67, 171, 227, 75, 0, 0, 0, 0, 40, 73, 225, 152, 0, 0, 0, 0, 185, 4, 165, 22, 103, 157, 0, 0, 0, 0, 41, 166, 229, 7, 0, 0, 0, 0, 253, 243, 168, 21, 0, 0, 0, 0, 16, 23, 238, 113, 0, 0, 0, 0, 185, 4, 248, 103, 242, 72, 0, 0, 0, 0, 154, 103, 253, 181, 0, 0, 0, 0, 102, 102, 211, 44, 0, 0, 0, 0, 131, 234, 198, 129, 0, 0, 0, 0, 185, 4, 107, 50, 56, 20, 0, 0, 0, 0, 16, 44, 26, 62, 0, 0, 0, 0, 77, 233, 147, 103, 0, 0, 0, 0, 226, 3, 107, 252, 0, 0, 0, 0, 185, 4, 60, 127, 137, 117, 0, 0, 0, 0, 141, 114, 73, 74, 0, 0, 0, 0, 125, 155, 45, 138, 0, 0, 0, 0, 241, 13, 104, 23, 0, 0, 0, 0, 185, 4, 105, 138, 72, 3, 0, 0, 0, 0, 164, 231, 75, 1, 0, 0, 0, 0, 187, 226, 1, 50, 0, 0, 0, 0, 83, 39, 174, 240, 0, 0, 0, 0, 185, 4, 177, 181, 234, 222, 0, 0, 0, 0, 221, 221, 46, 163, 0, 0, 0, 0, 163, 153, 60, 11, 0, 0, 0, 0, 163, 54, 216, 186, 0, 0, 0, 0, 185, 4, 53, 133, 193, 252, 0, 0, 0, 0, 81, 77, 227, 61, 0, 0, 0, 0, 226, 115, 187, 2, 0, 0, 0, 0, 18, 104, 34, 238, 0, 0, 0, 0, 185, 4, 66, 12, 28, 129, 0, 0, 0, 0, 93, 205, 142, 11, 0, 0, 0, 0, 237, 66, 26, 151, 0, 0, 0, 0, 31, 234, 197, 159, 0, 0, 0, 0, 185, 4, 159, 29, 29, 253, 0, 0, 0, 0, 80, 228, 152, 112, 0, 0, 0, 0, 27, 88, 175, 16, 0, 0, 0, 0, 50, 2, 234, 120, 0, 0, 0, 0, 185, 4, 47, 215, 81, 22, 0, 0, 0, 0, 50, 4, 122, 107, 0, 0, 0, 0, 24, 16, 167, 4, 0, 0, 0, 0, 173, 165, 204, 128, 0, 0, 0, 0, 185, 4, 4, 145, 233, 172, 0, 0, 0, 0, 95, 53, 153, 5, 0, 0, 0, 0, 219, 252, 206, 121, 0, 0, 0, 0, 10, 255, 61, 44, 0, 0, 0, 0, 185, 4, 196, 238, 53, 29, 0, 0, 0, 0, 103, 5, 236, 4, 0, 0, 0, 0, 40, 121, 187, 222, 0, 0, 0, 0, 59, 136, 120, 145, 0, 0, 0, 0, 185, 4, 213, 136, 119, 205, 0, 0, 0, 0, 226, 96, 83, 14, 0, 0, 0, 0, 119, 94, 208, 17, 0, 0, 0, 0, 112, 132, 35, 170, 0, 0, 0, 0, 185, 4, 67, 167, 62, 165, 0, 0, 0, 0, 237, 16, 221, 255, 0, 0, 0, 0, 7, 216, 167, 161, 0, 0, 0, 0, 224, 213, 174, 61, 0, 0, 0, 0, 185, 4, 30, 27, 172, 162, 0, 0, 0, 0, 215, 255, 118, 88, 0, 0, 0, 0, 52, 68, 86, 197, 0, 0, 0, 0, 211, 156, 244, 152, 0, 0, 0, 0, 185, 4, 152, 131, 150, 232, 0, 0, 0, 0, 25, 183, 155, 53, 0, 0, 0, 0, 247, 240, 238, 74, 0, 0, 0, 0, 88, 145, 70, 237, 0, 0, 0, 0, 185, 4, 73, 64, 135, 76, 0, 0, 0, 0, 127, 246, 77, 209, 0, 0, 0, 0, 49, 220, 235, 210, 0, 0, 0, 0, 119, 166, 172, 0, 0, 0, 0, 0, 185, 4, 125, 125, 91, 69, 0, 0, 0, 0, 182, 47, 34, 116, 0, 0, 0, 0, 49, 184, 145, 220, 0, 0, 0, 0, 83, 208, 156, 50, 0, 0, 0, 0, 185, 4, 142, 242, 6, 12, 0, 0, 0, 0, 97, 112, 2, 112, 0, 0, 0, 0, 19, 87, 214, 21, 0, 0, 0, 0, 108, 215, 181, 24, 0, 0, 0, 0, 185, 4, 97, 77, 111, 162, 0, 0, 0, 0, 65, 55, 227, 196, 0, 0, 0, 0, 97, 34, 19, 236, 0, 0, 0, 0, 121, 132, 59, 92, 0, 0, 0, 0, 185, 4, 38, 155, 165, 71, 0, 0, 0, 0, 102, 141, 38, 17, 0, 0, 0, 0, 124, 18, 20, 34, 0, 0, 0, 0, 204, 209, 169, 196, 0, 0, 0, 0, 185, 4, 113, 214, 127, 138, 0, 0, 0, 0, 174, 250, 61, 103, 0, 0, 0, 0, 51, 195, 87, 167, 0, 0, 0, 0, 140, 225, 189, 44, 0, 0, 0, 0, 185, 4, 47, 79, 72, 9, 0, 0, 0, 0, 5, 23, 39, 126, 0, 0, 0, 0, 69, 46, 199, 248, 0, 0, 0, 0, 167, 127, 134, 190, 0, 0, 0, 0, 185, 4, 166, 219, 212, 229, 0, 0, 0, 0, 220, 104, 150, 139, 0, 0, 0, 0, 39, 217, 21, 158, 0, 0, 0, 0, 203, 216, 123, 140, 0, 0, 0, 0, 185, 4, 2, 198, 181, 199, 0, 0, 0, 0, 208, 195, 169, 36, 0, 0, 0, 0, 118, 254, 171, 113, 0, 0, 0, 0, 162, 164, 13, 155, 0, 0, 0, 0, 185, 4, 187, 25, 156, 117, 0, 0, 0, 0, 35, 203, 91, 201, 0, 0, 0, 0, 141, 134, 5, 75, 0, 0, 0, 0, 30, 163, 245, 93, 0, 0, 0, 0, 185, 4, 230, 204, 246, 159, 0, 0, 0, 0, 245, 7, 10, 40, 0, 0, 0, 0, 71, 188, 100, 128, 0, 0, 0, 0, 190, 142, 166, 248, 0, 0, 0, 0, 185, 4, 0, 16, 5, 9, 0, 0, 0, 0, 138, 151, 198, 133, 0, 0, 0, 0, 14, 74, 224, 152, 0, 0, 0, 0, 139, 243, 60, 160, 0, 0, 0, 0, 185, 4, 155, 232, 185, 7, 0, 0, 0, 0, 38, 183, 59, 20, 0, 0, 0, 0, 31, 63, 149, 37, 0, 0, 0, 0, 242, 185, 209, 201, 0, 0, 0, 0, 185, 4, 134, 94, 231, 252, 0, 0, 0, 0, 183, 212, 66, 234, 0, 0, 0, 0, 132, 24, 45, 131, 0, 0, 0, 0, 67, 120, 39, 119, 0, 0, 0, 0, 185, 4, 105, 63, 158, 91, 0, 0, 0, 0, 100, 166, 187, 35, 0, 0, 0, 0, 42, 189, 6, 190, 0, 0, 0, 0, 124, 32, 150, 251, 0, 0, 0, 0, 185, 4, 19, 251, 160, 200, 0, 0, 0, 0, 150, 160, 104, 182, 0, 0, 0, 0, 139, 64, 81, 248, 0, 0, 0, 0, 86, 163, 91, 67, 0, 0, 0, 0, 185, 4, 47, 94, 98, 19, 0, 0, 0, 0, 8, 186, 8, 206, 0, 0, 0, 0, 242, 64, 14, 87, 0, 0, 0, 0, 14, 6, 107, 13, 0, 0, 0, 0, 185, 4, 195, 103, 50, 77, 0, 0, 0, 0, 57, 206, 244, 183, 0, 0, 0, 0, 163, 67, 181, 178, 0, 0, 0, 0, 132, 169, 63, 217, 0, 0, 0, 0, 185, 4, 46, 32, 49, 204, 0, 0, 0, 0, 33, 91, 168, 7, 0, 0, 0, 0, 82, 97, 224, 149, 0, 0, 0, 0, 234, 22, 152, 145, 0, 0, 0, 0, 185, 4, 139, 172, 2, 181, 0, 0, 0, 0, 177, 64, 166, 114, 0, 0, 0, 0, 32, 254, 84, 43, 0, 0, 0, 0, 105, 251, 66, 219, 0, 0, 0, 0, 185, 4, 193, 147, 153, 161, 0, 0, 0, 0, 28, 129, 217, 106, 0, 0, 0, 0, 243, 215, 133, 215, 0, 0, 0, 0, 114, 233, 232, 162, 0, 0, 0, 0, 185, 4, 206, 236, 80, 204, 0, 0, 0, 0, 249, 27, 126, 95, 0, 0, 0, 0, 215, 4, 72, 30, 0, 0, 0, 0, 121, 219, 251, 209, 0, 0, 0, 0, 185, 4, 9, 24, 212, 62, 0, 0, 0, 0, 106, 29, 47, 238, 0, 0, 0, 0, 207, 39, 199, 68, 0, 0, 0, 0, 77, 183, 1, 78, 0, 0, 0, 0, 185, 4, 76, 162, 195, 183, 0, 0, 0, 0, 39, 157, 242, 205, 0, 0, 0, 0, 68, 5, 226, 226, 0, 0, 0, 0, 123, 227, 157, 63, 0, 0, 0, 0, 185, 4, 202, 79, 52, 64, 0, 0, 0, 0, 153, 23, 97, 155, 0, 0, 0, 0, 89, 248, 139, 198, 0, 0, 0, 0, 145, 89, 124, 243, 0, 0, 0, 0, 185, 4, 221, 182, 167, 134, 0, 0, 0, 0, 60, 170, 164, 75, 0, 0, 0, 0, 212, 201, 177, 209, 0, 0, 0, 0, 186, 54, 245, 83, 0, 0, 0, 0, 185, 4, 119, 55, 21, 104, 0, 0, 0, 0, 140, 222, 6, 219, 0, 0, 0, 0, 228, 79, 128, 194, 0, 0, 0, 0, 16, 191, 97, 212, 0, 0, 0, 0, 185, 4, 132, 253, 243, 214, 0, 0, 0, 0, 142, 227, 135, 152, 0, 0, 0, 0, 237, 63, 13, 155, 0, 0, 0, 0, 6, 250, 98, 143, 0, 0, 0, 0, 185, 4, 14, 143, 29, 36, 0, 0, 0, 0, 169, 177, 55, 119, 0, 0, 0, 0, 190, 185, 34, 108, 0, 0, 0, 0, 219, 27, 106, 212, 0, 0, 0, 0, 185, 4, 141, 176, 210, 232, 0, 0, 0, 0, 213, 185, 177, 112, 0, 0, 0, 0, 139, 92, 211, 87, 0, 0, 0, 0, 77, 108, 241, 25, 0, 0, 0, 0, 185, 4, 141, 221, 16, 162, 0, 0, 0, 0, 50, 47, 217, 82, 0, 0, 0, 0, 93, 139, 215, 33, 0, 0, 0, 0, 14, 15, 207, 177, 0, 0, 0, 0, 185, 4, 227, 200, 147, 158, 0, 0, 0, 0, 91, 135, 188, 16, 0, 0, 0, 0, 170, 99, 12, 47, 0, 0, 0, 0, 78, 1, 86, 106, 0, 0, 0, 0, 185, 4, 202, 217, 194, 180, 0, 0, 0, 0, 176, 76, 35, 23, 0, 0, 0, 0, 123, 15, 228, 161, 0, 0, 0, 0, 78, 49, 179, 5, 0, 0, 0, 0, 185, 4, 110, 104, 29, 37, 0, 0, 0, 0, 93, 175, 37, 249, 0, 0, 0, 0, 177, 74, 187, 14, 0, 0, 0, 0, 24, 145, 237, 176, 0, 0, 0, 0, 185, 4, 250, 9, 126, 221, 0, 0, 0, 0, 161, 48, 133, 73, 0, 0, 0, 0, 224, 121, 61, 104, 0, 0, 0, 0, 11, 30, 224, 87, 0, 0, 0, 0, 185, 4, 98, 185, 23, 163, 0, 0, 0, 0, 75, 66, 50, 12, 0, 0, 0, 0, 68, 145, 69, 250, 0, 0, 0, 0, 24, 114, 189, 38, 0, 0, 0, 0, 185, 4, 31, 193, 165, 195, 0, 0, 0, 0, 58, 212, 95, 184, 0, 0, 0, 0, 220, 225, 252, 142, 0, 0, 0, 0, 99, 204, 189, 118, 0, 0, 0, 0, 185, 4, 71, 94, 154, 135, 0, 0, 0, 0, 37, 174, 161, 168, 0, 0, 0, 0, 171, 109, 180, 205, 0, 0, 0, 0, 250, 227, 11, 52, 0, 0, 0, 0, 185, 4, 59, 165, 58, 220, 0, 0, 0, 0, 211, 30, 229, 10, 0, 0, 0, 0, 204, 54, 64, 163, 0, 0, 0, 0, 19, 143, 2, 181, 0, 0, 0, 0, 185, 4, 198, 232, 81, 65, 0, 0, 0, 0, 136, 244, 97, 240, 0, 0, 0, 0, 91, 64, 202, 248, 0, 0, 0, 0, 151, 35, 48, 171, 0, 0, 0, 0, 185, 4, 64, 205, 50, 177, 0, 0, 0, 0, 225, 34, 146, 187, 0, 0, 0, 0, 81, 237, 102, 92, 0, 0, 0, 0, 224, 145, 127, 116, 0, 0, 0, 0, 185, 4, 110, 110, 158, 202, 0, 0, 0, 0, 194, 155, 234, 89, 0, 0, 0, 0, 51, 114, 185, 127, 0, 0, 0, 0, 2, 237, 194, 88, 0, 0, 0, 0, 185, 4, 214, 41, 223, 125, 0, 0, 0, 0, 150, 129, 113, 155, 0, 0, 0, 0, 24, 236, 180, 73, 0, 0, 0, 0, 139, 10, 33, 226, 0, 0, 0, 0, 185, 4, 106, 109, 174, 133, 0, 0, 0, 0, 241, 3, 127, 112, 0, 0, 0, 0, 50, 72, 200, 134, 0, 0, 0, 0, 88, 78, 153, 41, 0, 0, 0, 0, 185, 4, 244, 112, 253, 20, 0, 0, 0, 0, 109, 67, 14, 42, 0, 0, 0, 0, 228, 200, 6, 227, 0, 0, 0, 0, 158, 210, 188, 65, 0, 0, 0, 0, 185, 4, 66, 153, 237, 146, 0, 0, 0, 0, 146, 74, 214, 111, 0, 0, 0, 0, 239, 157, 121, 96, 0, 0, 0, 0, 254, 138, 113, 250, 0, 0, 0, 0, 185, 4, 59, 13, 160, 249, 0, 0, 0, 0, 145, 26, 189, 69, 0, 0, 0, 0, 27, 227, 200, 191, 0, 0, 0, 0, 93, 199, 224, 164, 0, 0, 0, 0, 185, 4, 101, 40, 47, 249, 0, 0, 0, 0, 205, 236, 138, 133, 0, 0, 0, 0, 244, 3, 93, 133, 0, 0, 0, 0, 248, 154, 18, 30, 0, 0, 0, 0, 185, 4, 41, 34, 60, 147, 0, 0, 0, 0, 111, 254, 53, 90, 0, 0, 0, 0, 20, 139, 239, 24, 0, 0, 0, 0, 134, 140, 188, 66, 0, 0, 0, 0, 185, 4, 110, 93, 187, 12, 0, 0, 0, 0, 192, 3, 171, 198, 0, 0, 0, 0, 175, 188, 204, 191, 0, 0, 0, 0, 55, 52, 158, 144, 0, 0, 0, 0, 185, 4, 144, 227, 74, 124, 0, 0, 0, 0, 6, 222, 86, 175, 0, 0, 0, 0, 150, 133, 255, 100, 0, 0, 0, 0, 16, 17, 205, 156, 0, 0, 0, 0, 185, 4, 163, 218, 28, 79, 0, 0, 0, 0, 72, 131, 241, 12, 0, 0, 0, 0, 92, 204, 23, 103, 0, 0, 0, 0, 56, 219, 75, 93, 0, 0, 0, 0, 185, 4, 106, 172, 59, 59, 0, 0, 0, 0, 243, 141, 254, 96, 0, 0, 0, 0, 89, 238, 173, 92, 0, 0, 0, 0, 124, 45, 133, 37, 0, 0, 0, 0, 185, 4, 211, 7, 253, 237, 0, 0, 0, 0, 29, 137, 103, 184, 0, 0, 0, 0, 87, 168, 170, 133, 0, 0, 0, 0, 77, 207, 34, 93, 0, 0, 0, 0, 185, 4, 241, 212, 69, 108, 0, 0, 0, 0, 213, 89, 159, 193, 0, 0, 0, 0, 186, 156, 72, 60, 0, 0, 0, 0, 58, 212, 106, 30, 0, 0, 0, 0, 185, 4, 202, 129, 58, 231, 0, 0, 0, 0, 19, 95, 172, 183, 0, 0, 0, 0, 118, 241, 28, 76, 0, 0, 0, 0, 240, 26, 26, 56, 0, 0, 0, 0, 185, 4, 189, 146, 164, 187, 0, 0, 0, 0, 77, 92, 82, 2, 0, 0, 0, 0, 205, 68, 15, 186, 0, 0, 0, 0, 53, 247, 93, 18, 0, 0, 0, 0, 185, 4, 213, 70, 181, 167, 0, 0, 0, 0, 42, 125, 178, 77, 0, 0, 0, 0, 175, 7, 227, 53, 0, 0, 0, 0, 243, 204, 94, 35, 0, 0, 0, 0, 185, 4, 189, 39, 219, 15, 0, 0, 0, 0, 80, 21, 149, 61, 0, 0, 0, 0, 91, 5, 163, 233, 0, 0, 0, 0, 148, 255, 131, 81, 0, 0, 0, 0, 185, 4, 160, 210, 206, 136, 0, 0, 0, 0, 247, 82, 252, 35, 0, 0, 0, 0, 164, 228, 229, 33, 0, 0, 0, 0, 84, 137, 116, 5, 0, 0, 0, 0, 185, 4, 2, 109, 208, 87, 0, 0, 0, 0, 45, 149, 205, 30, 0, 0, 0, 0, 55, 25, 182, 73, 0, 0, 0, 0, 176, 23, 46, 193, 0, 0, 0, 0, 185, 4, 128, 121, 84, 214, 0, 0, 0, 0, 230, 26, 113, 21, 0, 0, 0, 0, 180, 206, 28, 119, 0, 0, 0, 0, 167, 97, 8, 20, 0, 0, 0, 0, 185, 4, 141, 92, 175, 7, 0, 0, 0, 0, 49, 77, 221, 109, 0, 0, 0, 0, 15, 95, 113, 189, 0, 0, 0, 0, 124, 118, 70, 181, 0, 0, 0, 0, 185, 4, 186, 53, 172, 46, 0, 0, 0, 0, 209, 28, 39, 249, 0, 0, 0, 0, 168, 209, 75, 29, 0, 0, 0, 0, 89, 221, 224, 197, 0, 0, 0, 0, 185, 4, 150, 20, 9, 216, 0, 0, 0, 0, 125, 226, 254, 175, 0, 0, 0, 0, 46, 65, 34, 216, 0, 0, 0, 0, 116, 215, 203, 234, 0, 0, 0, 0, 185, 4, 142, 148, 142, 133, 0, 0, 0, 0, 183, 114, 17, 141, 0, 0, 0, 0, 136, 217, 32, 65, 0, 0, 0, 0, 11, 250, 248, 154, 0, 0, 0, 0, 185, 4, 46, 230, 184, 214, 0, 0, 0, 0, 86, 75, 80, 64, 0, 0, 0, 0, 151, 250, 17, 162, 0, 0, 0, 0, 174, 196, 50, 172, 0, 0, 0, 0, 185, 4, 89, 170, 70, 101, 0, 0, 0, 0, 133, 27, 180, 243, 0, 0, 0, 0, 239, 159, 107, 132, 0, 0, 0, 0, 148, 231, 227, 134, 0, 0, 0, 0, 185, 4, 1, 204, 2, 129, 0, 0, 0, 0, 36, 47, 246, 193, 0, 0, 0, 0, 20, 109, 218, 1, 0, 0, 0, 0, 47, 218, 98, 158, 0, 0, 0, 0, 185, 4, 242, 217, 108, 168, 0, 0, 0, 0, 16, 93, 88, 53, 0, 0, 0, 0, 231, 8, 133, 5, 0, 0, 0, 0, 138, 15, 131, 99, 0, 0, 0, 0, 185, 4, 254, 153, 5, 175, 0, 0, 0, 0, 195, 255, 58, 10, 0, 0, 0, 0, 149, 84, 136, 133, 0, 0, 0, 0, 114, 91, 84, 84, 0, 0, 0, 0, 185, 4, 233, 201, 147, 207, 0, 0, 0, 0, 186, 169, 181, 252, 0, 0, 0, 0, 97, 141, 135, 43, 0, 0, 0, 0, 10, 182, 51, 171, 0, 0, 0, 0, 185, 4, 222, 16, 59, 204, 0, 0, 0, 0, 48, 86, 205, 119, 0, 0, 0, 0, 169, 20, 28, 76, 0, 0, 0, 0, 80, 132, 152, 60, 0, 0, 0, 0, 185, 4, 143, 195, 29, 73, 0, 0, 0, 0, 139, 79, 114, 222, 0, 0, 0, 0, 173, 212, 218, 6, 0, 0, 0, 0, 176, 191, 133, 130, 0, 0, 0, 0, 185, 4, 60, 252, 130, 200, 0, 0, 0, 0, 14, 174, 220, 178, 0, 0, 0, 0, 51, 9, 237, 131, 0, 0, 0, 0, 27, 38, 222, 132, 0, 0, 0, 0, 185, 4, 116, 173, 21, 40, 0, 0, 0, 0, 73, 48, 178, 14, 0, 0, 0, 0, 141, 46, 91, 101, 0, 0, 0, 0, 111, 70, 48, 62, 0, 0, 0, 0, 185, 4, 152, 202, 17, 40, 0, 0, 0, 0, 14, 77, 110, 149, 0, 0, 0, 0, 209, 217, 67, 79, 0, 0, 0, 0, 195, 135, 6, 154, 0, 0, 0, 0, 185, 4, 38, 15, 210, 102, 0, 0, 0, 0, 167, 123, 194, 80, 0, 0, 0, 0, 152, 133, 109, 220, 0, 0, 0, 0, 208, 103, 253, 174, 0, 0, 0, 0, 185, 4, 72, 234, 81, 36, 0, 0, 0, 0, 202, 145, 150, 87, 0, 0, 0, 0, 71, 156, 199, 222, 0, 0, 0, 0, 164, 237, 75, 237, 0, 0, 0, 0, 185, 4, 16, 253, 249, 157, 0, 0, 0, 0, 107, 100, 97, 46, 0, 0, 0, 0, 151, 87, 93, 78, 0, 0, 0, 0, 45, 102, 199, 46, 0, 0, 0, 0, 185, 4, 119, 161, 74, 204, 0, 0, 0, 0, 201, 211, 134, 170, 0, 0, 0, 0, 34, 159, 232, 160, 0, 0, 0, 0, 89, 0, 183, 186, 0, 0, 0, 0, 185, 4, 130, 231, 62, 176, 0, 0, 0, 0, 10, 231, 137, 108, 0, 0, 0, 0, 179, 98, 97, 170, 0, 0, 0, 0, 229, 73, 101, 226, 0, 0, 0, 0, 185, 4, 199, 110, 161, 203, 0, 0, 0, 0, 5, 134, 108, 202, 0, 0, 0, 0, 31, 189, 128, 108, 0, 0, 0, 0, 143, 44, 133, 159, 0, 0, 0, 0, 185, 4, 134, 196, 136, 195, 0, 0, 0, 0, 153, 55, 201, 106, 0, 0, 0, 0, 46, 136, 77, 210, 0, 0, 0, 0, 6, 69, 169, 91, 0, 0, 0, 0, 185, 4, 69, 187, 230, 251, 0, 0, 0, 0, 177, 190, 108, 205, 0, 0, 0, 0, 163, 74, 30, 174, 0, 0, 0, 0, 210, 233, 42, 162, 0, 0, 0, 0, 185, 4, 208, 228, 108, 145, 0, 0, 0, 0, 187, 79, 160, 133, 0, 0, 0, 0, 233, 231, 80, 120, 0, 0, 0, 0, 188, 151, 1, 21, 0, 0, 0, 0, 185, 4, 106, 165, 119, 225, 0, 0, 0, 0, 156, 168, 60, 217, 0, 0, 0, 0, 195, 60, 124, 41, 0, 0, 0, 0, 66, 23, 147, 101, 0, 0, 0, 0, 185, 4, 104, 90, 169, 182, 0, 0, 0, 0, 189, 54, 120, 117, 0, 0, 0, 0, 46, 193, 234, 6, 0, 0, 0, 0, 16, 168, 254, 174, 0, 0, 0, 0, 185, 4, 59, 138, 167, 31, 0, 0, 0, 0, 18, 208, 236, 161, 0, 0, 0, 0, 108, 96, 139, 26, 0, 0, 0, 0, 97, 34, 111, 22, 0, 0, 0, 0, 185, 4, 160, 76, 229, 8, 0, 0, 0, 0, 213, 69, 74, 101, 0, 0, 0, 0, 12, 244, 94, 38, 0, 0, 0, 0, 24, 17, 134, 118, 0, 0, 0, 0, 185, 4, 176, 77, 16, 231, 0, 0, 0, 0, 229, 45, 141, 221, 0, 0, 0, 0, 44, 76, 208, 106, 0, 0, 0, 0, 149, 102, 11, 218, 0, 0, 0, 0, 185, 4, 181, 217, 147, 0, 0, 0, 0, 0, 110, 59, 29, 222, 0, 0, 0, 0, 130, 25, 200, 222, 0, 0, 0, 0, 166, 100, 246, 19, 0, 0, 0, 0, 185, 4, 136, 225, 187, 171, 0, 0, 0, 0, 180, 159, 35, 155, 0, 0, 0, 0, 30, 3, 33, 115, 0, 0, 0, 0, 132, 121, 104, 231, 0, 0, 0, 0, 185, 4, 212, 135, 8, 208, 0, 0, 0, 0, 137, 136, 107, 177, 0, 0, 0, 0, 181, 134, 222, 251, 0, 0, 0, 0, 1, 187, 232, 6, 0, 0, 0, 0, 185, 4, 169, 152, 38, 194, 0, 0, 0, 0, 28, 128, 14, 177, 0, 0, 0, 0, 192, 8, 83, 97, 0, 0, 0, 0, 87, 173, 38, 99, 0, 0, 0, 0, 185, 4, 223, 142, 73, 196, 0, 0, 0, 0, 240, 228, 218, 94, 0, 0, 0, 0, 132, 214, 13, 191, 0, 0, 0, 0, 20, 97, 195, 189, 0, 0, 0, 0, 185, 4, 218, 240, 31, 50, 0, 0, 0, 0, 130, 83, 4, 146, 0, 0, 0, 0, 201, 6, 62, 82, 0, 0, 0, 0, 163, 170, 58, 189, 0, 0, 0, 0, 185, 4, 147, 103, 35, 172, 0, 0, 0, 0, 160, 13, 68, 43, 0, 0, 0, 0, 31, 206, 119, 233, 0, 0, 0, 0, 117, 97, 29, 177, 0, 0, 0, 0, 185, 4, 10, 59, 174, 29, 0, 0, 0, 0, 158, 214, 102, 24, 0, 0, 0, 0, 2, 51, 134, 202, 0, 0, 0, 0, 34, 52, 62, 7, 0, 0, 0, 0, 185, 4, 132, 237, 168, 131, 0, 0, 0, 0, 161, 63, 166, 154, 0, 0, 0, 0, 193, 93, 224, 106, 0, 0, 0, 0, 46, 197, 18, 61, 0, 0, 0, 0, 185, 4, 229, 244, 139, 77, 0, 0, 0, 0, 225, 204, 225, 8, 0, 0, 0, 0, 127, 249, 149, 123, 0, 0, 0, 0, 50, 137, 181, 160, 0, 0, 0, 0, 185, 4, 152, 39, 87, 57, 0, 0, 0, 0, 118, 193, 141, 180, 0, 0, 0, 0, 50, 174, 221, 104, 0, 0, 0, 0, 238, 215, 182, 134, 0, 0, 0, 0, 185, 4, 104, 138, 31, 70, 0, 0, 0, 0, 10, 17, 52, 12, 0, 0, 0, 0, 250, 88, 191, 61, 0, 0, 0, 0, 83, 42, 57, 49, 0, 0, 0, 0, 185, 4, 15, 151, 235, 118, 0, 0, 0, 0, 45, 94, 93, 171, 0, 0, 0, 0, 122, 23, 251, 245, 0, 0, 0, 0, 77, 188, 92, 87, 0, 0, 0, 0, 185, 4, 123, 181, 217, 174, 0, 0, 0, 0, 239, 98, 151, 68, 0, 0, 0, 0, 108, 80, 56, 60, 0, 0, 0, 0, 5, 176, 100, 248, 0, 0, 0, 0, 185, 4, 8, 42, 50, 118, 0, 0, 0, 0, 128, 227, 140, 51, 0, 0, 0, 0, 29, 61, 11, 92, 0, 0, 0, 0, 171, 210, 122, 227, 0, 0, 0, 0, 185, 4, 154, 98, 189, 161, 0, 0, 0, 0, 74, 70, 185, 121, 0, 0, 0, 0, 41, 118, 175, 0, 0, 0, 0, 0, 233, 7, 26, 193, 0, 0, 0, 0, 185, 4, 142, 30, 191, 154, 0, 0, 0, 0, 3, 155, 5, 226, 0, 0, 0, 0, 170, 155, 217, 195, 0, 0, 0, 0, 212, 60, 30, 45, 0, 0, 0, 0, 185, 4, 141, 0, 235, 40, 0, 0, 0, 0, 207, 151, 20, 0, 0, 0, 0, 0, 220, 146, 84, 67, 0, 0, 0, 0, 212, 29, 150, 34, 0, 0, 0, 0, 185, 4, 194, 65, 95, 22, 0, 0, 0, 0, 230, 22, 1, 115, 0, 0, 0, 0, 78, 202, 74, 131, 0, 0, 0, 0, 46, 59, 208, 193, 0, 0, 0, 0, 185, 4, 82, 180, 150, 89, 0, 0, 0, 0, 0, 51, 75, 134, 0, 0, 0, 0, 85, 40, 67, 107, 0, 0, 0, 0, 174, 19, 138, 95, 0, 0, 0, 0, 185, 4, 69, 32, 127, 200, 0, 0, 0, 0, 128, 170, 166, 226, 0, 0, 0, 0, 235, 195, 208, 83, 0, 0, 0, 0, 223, 233, 199, 85, 0, 0, 0, 0, 185, 4, 66, 98, 176, 103, 0, 0, 0, 0, 176, 68, 42, 80, 0, 0, 0, 0, 79, 202, 211, 115, 0, 0, 0, 0, 165, 126, 126, 237, 0, 0, 0, 0, 185, 4, 156, 206, 50, 221, 0, 0, 0, 0, 150, 115, 246, 37, 0, 0, 0, 0, 34, 70, 3, 231, 0, 0, 0, 0, 2, 125, 173, 71, 0, 0, 0, 0, 185, 4, 20, 226, 71, 246, 0, 0, 0, 0, 154, 226, 194, 197, 0, 0, 0, 0, 111, 97, 199, 161, 0, 0, 0, 0, 183, 191, 83, 119, 0, 0, 0, 0, 185, 4, 51, 221, 200, 36, 0, 0, 0, 0, 219, 121, 79, 233, 0, 0, 0, 0, 104, 62, 9, 58, 0, 0, 0, 0, 21, 18, 20, 61, 0, 0, 0, 0, 185, 4, 230, 153, 0, 7, 0, 0, 0, 0, 152, 24, 43, 244, 0, 0, 0, 0, 86, 50, 105, 223, 0, 0, 0, 0, 101, 183, 81, 84, 0, 0, 0, 0, 185, 4, 183, 118, 102, 64, 0, 0, 0, 0, 228, 51, 182, 148, 0, 0, 0, 0, 113, 85, 129, 124, 0, 0, 0, 0, 169, 17, 212, 198, 0, 0, 0, 0, 185, 4, 224, 227, 10, 114, 0, 0, 0, 0, 189, 242, 130, 197, 0, 0, 0, 0, 192, 27, 187, 106, 0, 0, 0, 0, 74, 87, 54, 230, 0, 0, 0, 0, 185, 4, 221, 72, 86, 203, 0, 0, 0, 0, 138, 156, 114, 240, 0, 0, 0, 0, 103, 62, 216, 2, 0, 0, 0, 0, 169, 158, 175, 72, 0, 0, 0, 0, 185, 4, 121, 164, 126, 245, 0, 0, 0, 0, 254, 150, 178, 172, 0, 0, 0, 0, 89, 181, 203, 4, 0, 0, 0, 0, 197, 18, 226, 32, 0, 0, 0, 0, 185, 4, 217, 229, 87, 204, 0, 0, 0, 0, 148, 243, 157, 14, 0, 0, 0, 0, 214, 124, 131, 9, 0, 0, 0, 0, 89, 103, 147, 84, 0, 0, 0, 0, 185, 4, 161, 164, 226, 63, 0, 0, 0, 0, 23, 3, 40, 151, 0, 0, 0, 0, 13, 253, 250, 36, 0, 0, 0, 0, 79, 142, 107, 174, 0, 0, 0, 0, 185, 4, 48, 165, 213, 175, 0, 0, 0, 0, 93, 12, 150, 73, 0, 0, 0, 0, 252, 165, 38, 205, 0, 0, 0, 0, 4, 216, 82, 112, 0, 0, 0, 0, 185, 4, 104, 186, 45, 21, 0, 0, 0, 0, 69, 174, 151, 199, 0, 0, 0, 0, 207, 76, 152, 1, 0, 0, 0, 0, 229, 23, 144, 6, 0, 0, 0, 0, 185, 4, 184, 209, 106, 127, 0, 0, 0, 0, 51, 186, 198, 73, 0, 0, 0, 0, 139, 142, 15, 240, 0, 0, 0, 0, 52, 90, 53, 57, 0, 0, 0, 0, 185, 4, 121, 35, 151, 226, 0, 0, 0, 0, 153, 187, 43, 36, 0, 0, 0, 0, 126, 252, 170, 172, 0, 0, 0, 0, 242, 108, 126, 12, 0, 0, 0, 0, 185, 4, 74, 130, 195, 61, 0, 0, 0, 0, 234, 40, 177, 56, 0, 0, 0, 0, 130, 168, 28, 42, 0, 0, 0, 0, 134, 27, 219, 196, 0, 0, 0, 0, 185, 4, 226, 187, 61, 146, 0, 0, 0, 0, 111, 90, 172, 119, 0, 0, 0, 0, 203, 126, 45, 45, 0, 0, 0, 0, 194, 190, 191, 199, 0, 0, 0, 0, 185, 4, 253, 239, 73, 226, 0, 0, 0, 0, 184, 136, 230, 230, 0, 0, 0, 0, 55, 57, 193, 251, 0, 0, 0, 0, 199, 187, 242, 226, 0, 0, 0, 0, 185, 4, 212, 83, 214, 52, 0, 0, 0, 0, 75, 31, 238, 5, 0, 0, 0, 0, 172, 158, 242, 175, 0, 0, 0, 0, 147, 133, 214, 181, 0, 0, 0, 0, 185, 4, 49, 245, 200, 6, 0, 0, 0, 0, 107, 113, 54, 3, 0, 0, 0, 0, 238, 169, 12, 131, 0, 0, 0, 0, 20, 151, 16, 145, 0, 0, 0, 0, 185, 4, 25, 105, 252, 161, 0, 0, 0, 0, 184, 127, 4, 121, 0, 0, 0, 0, 2, 29, 46, 147, 0, 0, 0, 0, 229, 45, 239, 61, 0, 0, 0, 0, 185, 4, 182, 3, 24, 174, 0, 0, 0, 0, 70, 36, 41, 253, 0, 0, 0, 0, 39, 107, 166, 187, 0, 0, 0, 0, 250, 72, 245, 36, 0, 0, 0, 0, 185, 4, 233, 81, 26, 65, 0, 0, 0, 0, 204, 99, 1, 249, 0, 0, 0, 0, 254, 15, 189, 143, 0, 0, 0, 0, 109, 53, 79, 87, 0, 0, 0, 0, 185, 4, 246, 31, 106, 197, 0, 0, 0, 0, 189, 75, 198, 99, 0, 0, 0, 0, 11, 79, 191, 47, 0, 0, 0, 0, 21, 100, 35, 2, 0, 0, 0, 0, 185, 4, 238, 133, 213, 194, 0, 0, 0, 0, 50, 7, 178, 70, 0, 0, 0, 0, 78, 158, 144, 210, 0, 0, 0, 0, 228, 249, 141, 104, 0, 0, 0, 0, 185, 4, 235, 164, 119, 226, 0, 0, 0, 0, 118, 152, 193, 149, 0, 0, 0, 0, 241, 214, 19, 93, 0, 0, 0, 0, 82, 130, 88, 94, 0, 0, 0, 0, 185, 4, 70, 10, 234, 242, 0, 0, 0, 0, 104, 138, 142, 170, 0, 0, 0, 0, 142, 20, 128, 168, 0, 0, 0, 0, 176, 18, 212, 99, 0, 0, 0, 0, 185, 4, 231, 236, 157, 155, 0, 0, 0, 0, 54, 248, 205, 210, 0, 0, 0, 0, 179, 98, 45, 233, 0, 0, 0, 0, 41, 34, 88, 189, 0, 0, 0, 0, 185, 4, 36, 118, 2, 182, 0, 0, 0, 0, 166, 21, 78, 13, 0, 0, 0, 0, 14, 113, 66, 253, 0, 0, 0, 0, 6, 20, 109, 207, 0, 0, 0, 0, 185, 4, 100, 206, 169, 93, 0, 0, 0, 0, 219, 43, 49, 188, 0, 0, 0, 0, 216, 175, 42, 148, 0, 0, 0, 0, 44, 166, 230, 64, 0, 0, 0, 0, 185, 4, 65, 120, 82, 130, 0, 0, 0, 0, 135, 115, 224, 129, 0, 0, 0, 0, 64, 217, 161, 166, 0, 0, 0, 0, 139, 34, 249, 31, 0, 0, 0, 0, 185, 4, 182, 90, 166, 26, 0, 0, 0, 0, 40, 202, 92, 66, 0, 0, 0, 0, 6, 90, 248, 164, 0, 0, 0, 0, 59, 116, 214, 138, 0, 0, 0, 0, 185, 4, 120, 193, 85, 124, 0, 0, 0, 0, 0, 10, 104, 188, 0, 0, 0, 0, 136, 22, 184, 141, 0, 0, 0, 0, 40, 148, 154, 233, 0, 0, 0, 0, 185, 4, 115, 165, 28, 85, 0, 0, 0, 0, 57, 91, 113, 0, 0, 0, 0, 0, 111, 48, 64, 107, 0, 0, 0, 0, 144, 34, 169, 93, 0, 0, 0, 0, 185, 4, 97, 118, 169, 200, 0, 0, 0, 0, 104, 187, 185, 215, 0, 0, 0, 0, 185, 0, 56, 149, 0, 0, 0, 0, 122, 215, 112, 133, 0, 0, 0, 0, 185, 4, 193, 60, 196, 73, 0, 0, 0, 0, 117, 204, 42, 95, 0, 0, 0, 0, 115, 22, 221, 105, 0, 0, 0, 0, 43, 31, 114, 37, 0, 0, 0, 0, 185, 4, 53, 120, 54, 0, 0, 0, 0, 0, 227, 203, 95, 4, 0, 0, 0, 0, 79, 98, 166, 3, 0, 0, 0, 0, 21, 88, 63, 83, 0, 0, 0, 0, 185, 4, 89, 215, 191, 9, 0, 0, 0, 0, 48, 149, 112, 155, 0, 0, 0, 0, 50, 55, 44, 180, 0, 0, 0, 0, 243, 125, 103, 15, 0, 0, 0, 0, 185, 4, 205, 193, 188, 104, 0, 0, 0, 0, 17, 128, 40, 94, 0, 0, 0, 0, 165, 57, 21, 221, 0, 0, 0, 0, 57, 104, 114, 148, 0, 0, 0, 0, 185, 4, 71, 107, 196, 75, 0, 0, 0, 0, 88, 231, 253, 4, 0, 0, 0, 0, 214, 240, 36, 74, 0, 0, 0, 0, 28, 25, 210, 102, 0, 0, 0, 0, 185, 4, 210, 251, 112, 3, 0, 0, 0, 0, 39, 119, 109, 119, 0, 0, 0, 0, 82, 110, 93, 91, 0, 0, 0, 0, 11, 99, 75, 32, 0, 0, 0, 0, 185, 4, 219, 11, 200, 60, 0, 0, 0, 0, 114, 104, 125, 221, 0, 0, 0, 0, 95, 92, 158, 152, 0, 0, 0, 0, 131, 189, 105, 233, 0, 0, 0, 0, 185, 4, 78, 99, 78, 100, 0, 0, 0, 0, 73, 22, 254, 57, 0, 0, 0, 0, 110, 226, 82, 185, 0, 0, 0, 0, 81, 123, 170, 162, 0, 0, 0, 0, 185, 4, 131, 150, 117, 84, 0, 0, 0, 0, 170, 112, 56, 253, 0, 0, 0, 0, 76, 140, 14, 136, 0, 0, 0, 0, 3, 239, 92, 176, 0, 0, 0, 0, 185, 4, 221, 114, 107, 206, 0, 0, 0, 0, 150, 72, 64, 21, 0, 0, 0, 0, 197, 51, 65, 166, 0, 0, 0, 0, 185, 66, 150, 126, 0, 0, 0, 0, 185, 4, 111, 97, 163, 217, 0, 0, 0, 0, 24, 109, 212, 49, 0, 0, 0, 0, 231, 119, 211, 222, 0, 0, 0, 0, 103, 41, 166, 71, 0, 0, 0, 0, 185, 4, 132, 200, 245, 135, 0, 0, 0, 0, 105, 46, 8, 134, 0, 0, 0, 0, 242, 229, 118, 102, 0, 0, 0, 0, 87, 131, 19, 61, 0, 0, 0, 0, 185, 4, 27, 83, 198, 117, 0, 0, 0, 0, 44, 71, 152, 83, 0, 0, 0, 0, 248, 169, 210, 79, 0, 0, 0, 0, 104, 1, 204, 173, 0, 0, 0, 0, 185, 4, 159, 72, 100, 150, 0, 0, 0, 0, 242, 140, 226, 185, 0, 0, 0, 0, 118, 62, 215, 55, 0, 0, 0, 0, 147, 17, 122, 86, 0, 0, 0, 0, 185, 4, 84, 177, 79, 94, 0, 0, 0, 0, 199, 179, 187, 194, 0, 0, 0, 0, 39, 115, 93, 6, 0, 0, 0, 0, 169, 126, 67, 132, 0, 0, 0, 0, 185, 4, 6, 197, 191, 37, 0, 0, 0, 0, 52, 218, 53, 172, 0, 0, 0, 0, 238, 21, 23, 177, 0, 0, 0, 0, 47, 214, 6, 241, 0, 0, 0, 0, 185, 4, 13, 210, 19, 206, 0, 0, 0, 0, 77, 247, 83, 142, 0, 0, 0, 0, 59, 208, 7, 121, 0, 0, 0, 0, 14, 134, 246, 137, 0, 0, 0, 0, 185, 4, 200, 49, 187, 22, 0, 0, 0, 0, 40, 198, 106, 250, 0, 0, 0, 0, 207, 49, 54, 110, 0, 0, 0, 0, 31, 128, 66, 216, 0, 0, 0, 0, 185, 4, 138, 150, 211, 57, 0, 0, 0, 0, 61, 18, 101, 2, 0, 0, 0, 0, 67, 228, 67, 65, 0, 0, 0, 0, 60, 71, 94, 4, 0, 0, 0, 0, 185, 4, 169, 154, 124, 197, 0, 0, 0, 0, 60, 135, 17, 216, 0, 0, 0, 0, 184, 195, 80, 168, 0, 0, 0, 0, 147, 183, 205, 95, 0, 0, 0, 0, 185, 4, 160, 164, 41, 146, 0, 0, 0, 0, 18, 106, 178, 104, 0, 0, 0, 0, 126, 241, 247, 157, 0, 0, 0, 0, 28, 80, 28, 183, 0, 0, 0, 0, 185, 4, 95, 145, 184, 253, 0, 0, 0, 0, 173, 222, 86, 113, 0, 0, 0, 0, 178, 187, 201, 253, 0, 0, 0, 0, 201, 36, 145, 101, 0, 0, 0, 0, 185, 4, 74, 167, 42, 61, 0, 0, 0, 0, 69, 59, 66, 33, 0, 0, 0, 0, 60, 228, 252, 107, 0, 0, 0, 0, 101, 2, 145, 150, 0, 0, 0, 0, 185, 4, 190, 246, 183, 232, 0, 0, 0, 0, 136, 142, 221, 207, 0, 0, 0, 0, 104, 145, 173, 187, 0, 0, 0, 0, 212, 8, 70, 144, 0, 0, 0, 0, 185, 4, 160, 14, 251, 212, 0, 0, 0, 0, 117, 138, 134, 210, 0, 0, 0, 0, 31, 7, 219, 44, 0, 0, 0, 0, 237, 205, 229, 243, 0, 0, 0, 0, 185, 4, 253, 242, 131, 65, 0, 0, 0, 0, 228, 232, 90, 162, 0, 0, 0, 0, 138, 105, 177, 2, 0, 0, 0, 0, 91, 49, 24, 212, 0, 0, 0, 0, 185, 4, 44, 250, 40, 233, 0, 0, 0, 0, 165, 104, 158, 244, 0, 0, 0, 0, 9, 9, 115, 83, 0, 0, 0, 0, 134, 185, 249, 124, 0, 0, 0, 0, 185, 4, 74, 52, 64, 195, 0, 0, 0, 0, 209, 124, 215, 110, 0, 0, 0, 0, 115, 64, 159, 112, 0, 0, 0, 0, 36, 125, 172, 233, 0, 0, 0, 0, 185, 4, 155, 237, 131, 23, 0, 0, 0, 0, 99, 178, 175, 208, 0, 0, 0, 0, 75, 95, 30, 86, 0, 0, 0, 0, 178, 37, 169, 161, 0, 0, 0, 0, 185, 4, 81, 70, 64, 209, 0, 0, 0, 0, 183, 219, 237, 25, 0, 0, 0, 0, 248, 152, 238, 61, 0, 0, 0, 0, 201, 236, 30, 119, 0, 0, 0, 0, 185, 4, 88, 15, 0, 118, 0, 0, 0, 0, 214, 124, 208, 31, 0, 0, 0, 0, 239, 39, 234, 51, 0, 0, 0, 0, 107, 97, 1, 198, 0, 0, 0, 0, 185, 4, 134, 102, 243, 132, 0, 0, 0, 0, 92, 211, 80, 250, 0, 0, 0, 0, 200, 127, 136, 171, 0, 0, 0, 0, 57, 160, 184, 155, 0, 0, 0, 0, 185, 4, 199, 82, 217, 41, 0, 0, 0, 0, 131, 16, 32, 221, 0, 0, 0, 0, 197, 9, 191, 11, 0, 0, 0, 0, 76, 4, 161, 150, 0, 0, 0, 0, 185, 4, 77, 251, 10, 124, 0, 0, 0, 0, 82, 120, 233, 226, 0, 0, 0, 0, 161, 167, 252, 193, 0, 0, 0, 0, 103, 48, 255, 42, 0, 0, 0, 0, 185, 4, 221, 13, 99, 73, 0, 0, 0, 0, 156, 96, 21, 108, 0, 0, 0, 0, 145, 170, 244, 48, 0, 0, 0, 0, 29, 171, 16, 43, 0, 0, 0, 0, 185, 4, 96, 0, 220, 98, 0, 0, 0, 0, 2, 25, 60, 149, 0, 0, 0, 0, 59, 64, 12, 191, 0, 0, 0, 0, 224, 54, 193, 242, 0, 0, 0, 0, 185, 4, 75, 4, 54, 157, 0, 0, 0, 0, 47, 110, 84, 219, 0, 0, 0, 0, 130, 10, 103, 132, 0, 0, 0, 0, 65, 238, 75, 169, 0, 0, 0, 0, 185, 4, 31, 114, 229, 163, 0, 0, 0, 0, 224, 206, 25, 51, 0, 0, 0, 0, 253, 209, 251, 103, 0, 0, 0, 0, 91, 243, 103, 170, 0, 0, 0, 0, 185, 4, 6, 138, 67, 225, 0, 0, 0, 0, 230, 57, 147, 195, 0, 0, 0, 0, 117, 179, 5, 62, 0, 0, 0, 0, 81, 118, 183, 142, 0, 0, 0, 0, 185, 4, 65, 33, 103, 229, 0, 0, 0, 0, 201, 225, 72, 104, 0, 0, 0, 0, 122, 129, 17, 121, 0, 0, 0, 0, 177, 254, 144, 134, 0, 0, 0, 0, 185, 4, 203, 21, 69, 123, 0, 0, 0, 0, 243, 170, 253, 86, 0, 0, 0, 0, 136, 17, 87, 24, 0, 0, 0, 0, 239, 152, 13, 95, 0, 0, 0, 0, 185, 4, 226, 189, 155, 80, 0, 0, 0, 0, 96, 21, 104, 162, 0, 0, 0, 0, 17, 152, 38, 125, 0, 0, 0, 0, 109, 243, 52, 178, 0, 0, 0, 0, 185, 4, 210, 234, 187, 14, 0, 0, 0, 0, 216, 113, 203, 15, 0, 0, 0, 0, 112, 46, 37, 78, 0, 0, 0, 0, 91, 221, 1, 123, 0, 0, 0, 0, 185, 4, 130, 36, 170, 34, 0, 0, 0, 0, 153, 46, 32, 81, 0, 0, 0, 0, 47, 147, 218, 75, 0, 0, 0, 0, 23, 98, 36, 43, 0, 0, 0, 0, 185, 4, 115, 188, 203, 242, 0, 0, 0, 0, 254, 173, 102, 49, 0, 0, 0, 0, 89, 6, 202, 16, 0, 0, 0, 0, 112, 45, 186, 131, 0, 0, 0, 0, 185, 4, 242, 134, 217, 168, 0, 0, 0, 0, 117, 94, 208, 155, 0, 0, 0, 0, 208, 62, 97, 129, 0, 0, 0, 0, 234, 157, 78, 133, 0, 0, 0, 0, 185, 4, 206, 106, 159, 171, 0, 0, 0, 0, 163, 211, 183, 15, 0, 0, 0, 0, 4, 175, 85, 59, 0, 0, 0, 0, 162, 160, 231, 64, 0, 0, 0, 0, 185, 4, 32, 223, 144, 34, 0, 0, 0, 0, 24, 215, 40, 110, 0, 0, 0, 0, 222, 229, 131, 231, 0, 0, 0, 0, 198, 54, 44, 160, 0, 0, 0, 0, 185, 4, 225, 22, 57, 160, 0, 0, 0, 0, 16, 5, 191, 99, 0, 0, 0, 0, 140, 253, 90, 243, 0, 0, 0, 0, 1, 181, 189, 151, 0, 0, 0, 0, 185, 4, 56, 187, 222, 85, 0, 0, 0, 0, 52, 208, 74, 31, 0, 0, 0, 0, 72, 203, 39, 204, 0, 0, 0, 0, 170, 73, 100, 164, 0, 0, 0, 0, 185, 4, 253, 67, 125, 168, 0, 0, 0, 0, 75, 56, 99, 181, 0, 0, 0, 0, 56, 8, 210, 28, 0, 0, 0, 0, 159, 220, 253, 230, 0, 0, 0, 0, 185, 4, 55, 22, 202, 36, 0, 0, 0, 0, 8, 141, 72, 195, 0, 0, 0, 0, 45, 150, 2, 98, 0, 0, 0, 0, 151, 167, 208, 249, 0, 0, 0, 0, 185, 4, 66, 181, 234, 204, 0, 0, 0, 0, 242, 0, 131, 123, 0, 0, 0, 0, 55, 153, 173, 253, 0, 0, 0, 0, 194, 200, 75, 28, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 209, 3, 0, 0, 0, 0, 0, 0, 185, 4, 31, 133, 63, 207, 0, 0, 0, 0, 214, 130, 165, 212, 0, 0, 0, 0, 154, 181, 182, 112, 0, 0, 0, 0, 54, 193, 25, 172, 0, 0, 0, 0, 185, 4, 93, 93, 252, 141, 0, 0, 0, 0, 77, 198, 29, 31, 0, 0, 0, 0, 210, 166, 94, 177, 0, 0, 0, 0, 226, 171, 219, 211, 0, 0, 0, 0, 185, 4, 67, 230, 129, 153, 0, 0, 0, 0, 72, 159, 8, 233, 0, 0, 0, 0, 192, 72, 159, 151, 0, 0, 0, 0, 156, 18, 253, 51, 0, 0, 0, 0, 185, 4, 83, 41, 30, 35, 0, 0, 0, 0, 219, 102, 188, 41, 0, 0, 0, 0, 90, 46, 54, 215, 0, 0, 0, 0, 151, 32, 126, 72, 0, 0, 0, 0, 254, 125, 10, 5, 0, 254, 126, 10, 6, 0, 254, 127, 10, 10, 0, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 18, 0, 0, 0, 0, 0, 0, 0, 110, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 71, 167, 78, 1, 200, 18, 0, 0, 0, 0, 0, 0, 0, 108, 253, 46, 0, 200, 12, 0, 0, 0, 0, 0, 0, 0, 108, 200, 13, 0, 0, 0, 0, 0, 0, 0, 108, 200, 14, 0, 0, 0, 0, 0, 0, 0, 108, 200, 15, 0, 0, 0, 0, 0, 0, 0, 108, 200, 16, 0, 0, 0, 0, 0, 0, 0, 108, 200, 17, 0, 0, 0, 0, 0, 0, 0, 108, 186, 11, 0, 0, 0, 0, 0, 0, 0, 186, 10, 0, 0, 0, 0, 0, 0, 0, 186, 9, 0, 0, 0, 0, 0, 0, 0, 186, 8, 0, 0, 0, 0, 0, 0, 0, 186, 7, 0, 0, 0, 0, 0, 0, 0, 186, 6, 0, 0, 0, 0, 0, 0, 0, 186, 17, 0, 0, 0, 0, 0, 0, 0, 186, 16, 0, 0, 0, 0, 0, 0, 0, 186, 15, 0, 0, 0, 0, 0, 0, 0, 186, 14, 0, 0, 0, 0, 0, 0, 0, 186, 13, 0, 0, 0, 0, 0, 0, 0, 186, 12, 0, 0, 0, 0, 0, 0, 0, 186, 5, 0, 0, 0, 0, 0, 0, 0, 186, 4, 0, 0, 0, 0, 0, 0, 0, 186, 3, 0, 0, 0, 0, 0, 0, 0, 186, 2, 0, 0, 0, 0, 0, 0, 0, 186, 1, 0, 0, 0, 0, 0, 0, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 211, 12, 0, 107, 107, 194, 6, 0, 0, 0, 0, 0, 0, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 194, 7, 0, 0, 0, 0, 0, 0, 0, 200, 1, 0, 0, 0, 0, 0, 0, 0, 194, 8, 0, 0, 0, 0, 0, 0, 0, 200, 2, 0, 0, 0, 0, 0, 0, 0, 194, 9, 0, 0, 0, 0, 0, 0, 0, 200, 3, 0, 0, 0, 0, 0, 0, 0, 194, 10, 0, 0, 0, 0, 0, 0, 0, 200, 4, 0, 0, 0, 0, 0, 0, 0, 194, 11, 0, 0, 0, 0, 0, 0, 0, 200, 5, 0, 0, 0, 0, 0, 0, 0, 108, 1, 0, 254, 185, 10, 1, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 18, 0, 0, 0, 0, 0, 0, 0, 166, 200, 18, 0, 0, 0, 0, 0, 0, 0, 108, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 19, 0, 0, 0, 0, 0, 0, 0, 200, 18, 0, 0, 0, 0, 0, 0, 0, 108, 110, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 0, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 111, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 1, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 112, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 2, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 113, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 3, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 114, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 4, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108, 115, 185, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 5, 0, 0, 0, 0, 0, 0, 0, 151, 198, 108]),
("std::math::ext5",vm_assembly::ProcedureId([233, 105, 61, 244, 37, 115, 150, 121, 228, 144, 176, 20, 78, 188, 90, 0, 206, 194, 99, 16, 111, 100, 21, 222]),"#! Given two GF(p^5) elements on stack, this routine computes modular
#! addition over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#!
#! Expected stack state :
#!
#! [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#!
#! After application of routine stack :
#!
#! [c0, c1, c2, c3, c4, ...] s.t. c = a + b
#!
#! See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#!
#! For reference implementation in high level language, see 
#! https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L607-L616
export.add
    repeat.5
        movup.5
        add
        movdn.4
    end
end

#! Given two GF(p^5) elements on stack, this routine subtracts second
#! element from first one, over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#!
#! Expected stack state :
#!
#! [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#!
#! After application of routine stack :
#!
#! [c0, c1, c2, c3, c4, ...] s.t. c = a - b
#!
#! See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#!
#! For reference implementation in high level language, see 
#! https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L629-L638
export.sub
    repeat.5
        movup.5
        sub
        movdn.4
    end
end

#! Given two GF(p^5) elements on stack, this routine computes modular
#! multiplication ( including reduction by irreducible polynomial ) 
#! over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#!
#! Expected stack state :
#!
#! [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#!
#! After application of routine stack :
#!
#! [c0, c1, c2, c3, c4, ...] s.t. c = a * b
#!
#! See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#!
#! For reference implementation in high level language, see 
#! https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L676-L689
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

#! Given one GF(p^5) element on stack, this routine computes modular
#! squaring ( including reduction by irreducible polynomial ) 
#! over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#!
#! This routine has same effect as calling mul(a, a) | a ∈ GF(p^5)
#!
#! Expected stack state :
#!
#! [a0, a1, a2, a3, a4, ...]
#!
#! After application of routine stack :
#!
#! [b0, b1, b2, b3, b4, ...] s.t. b = a * a
#!
#! See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#!
#! For reference implementation in high level language, see 
#! https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L709-L715
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

#! Given an element a ∈ GF(p^5), this routine applies Frobenius operator
#! once, raising the element to the power of p | p = 2^64 - 2^32 + 1.
#!
#! Expected stack state :
#!
#! [a0, a1, a2, a3, a4, ...]
#!
#! Final stack state :
#!
#! [b0, b1, b2, b3, b4, ...]
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L723-L737
#! for reference implementation in high-level language.
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

#! Given an element a ∈ GF(p^5), this routine applies Frobenius operator
#! twice, raising the element to the power of p^2 | p = 2^64 - 2^32 + 1.
#!
#! Expected stack state :
#!
#! [a0, a1, a2, a3, a4, ...]
#!
#! Final stack state :
#!
#! [b0, b1, b2, b3, b4, ...]
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L739-L749
#! for reference implementation in high-level language.
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

#! Given one GF(p^5) element on stack, this routine computes multiplicative
#! inverse over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#!
#! Expected stack state :
#!
#! [a0, a1, a2, a3, a4, ...]
#!
#! After application of routine stack :
#!
#! [b0, b1, b2, b3, b4, ...] s.t. b = 1 / a
#!
#! See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#!
#! For reference implementation in high level language, see 
#! https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L751-L775
#!
#! Note, this routine will not panic even when operand `a` is zero.
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

#! Given two GF(p^5) elements ( say a, b ) on stack, this routine computes
#! modular division over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1
#!
#! Expected stack state :
#!
#! [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#!
#! After application of routine stack :
#!
#! [c0, c1, c2, c3, c4, ...] s.t. c = a / b
#!
#! See section 3.2 of https://eprint.iacr.org/2022/274.pdf
#!
#! For reference implementation in high level language, see 
#! https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L777-L781
export.div
    repeat.5
        movup.9
    end

    exec.inv
    exec.mul
end

#! Given an element v ∈ Z_q | q = 2^64 - 2^32 + 1, and n on stack, this routine
#! raises it to the power 2^n, by means of n successive squarings
#!
#! Expected stack stack
#!
#! [v, n, ...] | n >= 0
#!
#! After finishing execution stack
#!
#! [v', ...] s.t. v' = v ^ (2^n)
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L461-L469
#! for reference implementation in higher level language
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

#! Given an element v ∈ Z_q | q = 2^64 - 2^32 + 1, this routine attempts to compute
#! square root of v, if that number is a square.
#!
#! Expected stack state :
#!
#! [v, ...]
#!
#! After finishing execution stack looks like :
#!
#! [v', flg, ...]
#!
#! If flg = 1, it denotes v' is square root of v i.e. v' * v' = v ( mod q )
#! If flg = 0, then v' = 0, denoting v doesn't have a square root
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L349-L446
#! for reference implementation in higher level language.
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

#! Given an element v ∈ Z_q | q = 2^64 - 2^32 + 1, this routine computes
#! legendre symbol, by raising that element to the power (p-1) / 2
#!
#! Expected stack state :
#!
#! [v, ...]
#!
#! After finishing execution stack looks like
#!
#! [v', ...] s.t. v' = legendre symbol of v
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L448-L459
#! for reference implementation in higher level language.
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

#! Given an element v ∈ GF(p^5), this routine computes its legendre symbol,
#! which is an element ∈ GF(p) | p = 2^64 - 2^32 + 1
#!
#! At beginning stack looks like
#!
#! [a0, a1, a2, a3, a4, ...]
#!
#! At end stack looks like
#!
#! [b, ...] s.t. b = legendre symbol of a
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L857-L877
#! for reference implementation in higher level language.
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

#! Given an element v ∈ GF(p^5), this routine attempts to compute square root of v, 
#! if that number is a square.
#!
#! At beginning stack looks like
#!
#! [a0, a1, a2, a3, a4, ...]
#!
#! At end stack looks like
#!
#! [b0, b1, b2, b3, b4, flg, ...]
#!
#! If flg = 1, it denotes v' = {b0, b1, b2, b3, b4} is square root of v i.e. v' * v' = v ( mod GF(p^5) )
#! If flg = 0, then v' = {0, 0, 0, 0, 0}, denoting v doesn't have a square root
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L879-L910
#! for reference implementation in higher level language.
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

#! Given two elements a, b ∈ GF(p^5), this routine produces single field element r,
#! denoting whether a == b.
#!
#! Expected stack state 
#!
#! [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#!
#! Final stack state 
#!
#! [r, ...]
#!
#! If a == b { r = 1 } Else { r = 0 }
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L797-L806
#! for reference implementation.
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

#! Given two elements a, b ∈ GF(p^5), this routine produces single field element r,
#! denoting whether a != b.
#!
#! Expected stack state 
#!
#! [a0, a1, a2, a3, a4, b0, b1, b2, b3, b4, ...]
#!
#! Final stack state 
#!
#! [r, ...]
#!
#! If a != b { r = 1 } Else { r = 0 }
#!
#! See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L813-L822
#! for reference implementation.
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
",&[15, 0, 3, 97, 100, 100, 206, 1, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 71, 70, 40, 112, 94, 53, 41, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 109, 111, 100, 117, 108, 97, 114, 10, 97, 100, 100, 105, 116, 105, 111, 110, 32, 111, 118, 101, 114, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 102, 105, 101, 108, 100, 32, 71, 70, 40, 112, 94, 53, 41, 32, 115, 46, 116, 46, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 45, 32, 50, 94, 51, 50, 32, 43, 32, 49, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 46, 46, 46, 93, 10, 65, 102, 116, 101, 114, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 114, 111, 117, 116, 105, 110, 101, 32, 115, 116, 97, 99, 107, 32, 58, 10, 91, 99, 48, 44, 32, 99, 49, 44, 32, 99, 50, 44, 32, 99, 51, 44, 32, 99, 52, 44, 32, 46, 46, 46, 93, 32, 115, 46, 116, 46, 32, 99, 32, 61, 32, 97, 32, 43, 32, 98, 10, 83, 101, 101, 32, 115, 101, 99, 116, 105, 111, 110, 32, 51, 46, 50, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 101, 112, 114, 105, 110, 116, 46, 105, 97, 99, 114, 46, 111, 114, 103, 47, 50, 48, 50, 50, 47, 50, 55, 52, 46, 112, 100, 102, 10, 70, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 104, 105, 103, 104, 32, 108, 101, 118, 101, 108, 32, 108, 97, 110, 103, 117, 97, 103, 101, 44, 32, 115, 101, 101, 10, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 54, 48, 55, 45, 76, 54, 49, 54, 1, 0, 0, 1, 0, 254, 1, 0, 3, 0, 152, 3, 167, 3, 115, 117, 98, 221, 1, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 71, 70, 40, 112, 94, 53, 41, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 115, 117, 98, 116, 114, 97, 99, 116, 115, 32, 115, 101, 99, 111, 110, 100, 10, 101, 108, 101, 109, 101, 110, 116, 32, 102, 114, 111, 109, 32, 102, 105, 114, 115, 116, 32, 111, 110, 101, 44, 32, 111, 118, 101, 114, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 102, 105, 101, 108, 100, 32, 71, 70, 40, 112, 94, 53, 41, 32, 115, 46, 116, 46, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 45, 32, 50, 94, 51, 50, 32, 43, 32, 49, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 46, 46, 46, 93, 10, 65, 102, 116, 101, 114, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 114, 111, 117, 116, 105, 110, 101, 32, 115, 116, 97, 99, 107, 32, 58, 10, 91, 99, 48, 44, 32, 99, 49, 44, 32, 99, 50, 44, 32, 99, 51, 44, 32, 99, 52, 44, 32, 46, 46, 46, 93, 32, 115, 46, 116, 46, 32, 99, 32, 61, 32, 97, 32, 45, 32, 98, 10, 83, 101, 101, 32, 115, 101, 99, 116, 105, 111, 110, 32, 51, 46, 50, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 101, 112, 114, 105, 110, 116, 46, 105, 97, 99, 114, 46, 111, 114, 103, 47, 50, 48, 50, 50, 47, 50, 55, 52, 46, 112, 100, 102, 10, 70, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 104, 105, 103, 104, 32, 108, 101, 118, 101, 108, 32, 108, 97, 110, 103, 117, 97, 103, 101, 44, 32, 115, 101, 101, 10, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 54, 50, 57, 45, 76, 54, 51, 56, 1, 0, 0, 1, 0, 254, 8, 0, 3, 0, 152, 5, 167, 3, 109, 117, 108, 6, 2, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 71, 70, 40, 112, 94, 53, 41, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 109, 111, 100, 117, 108, 97, 114, 10, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 40, 32, 105, 110, 99, 108, 117, 100, 105, 110, 103, 32, 114, 101, 100, 117, 99, 116, 105, 111, 110, 32, 98, 121, 32, 105, 114, 114, 101, 100, 117, 99, 105, 98, 108, 101, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 41, 10, 111, 118, 101, 114, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 102, 105, 101, 108, 100, 32, 71, 70, 40, 112, 94, 53, 41, 32, 115, 46, 116, 46, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 45, 32, 50, 94, 51, 50, 32, 43, 32, 49, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 46, 46, 46, 93, 10, 65, 102, 116, 101, 114, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 114, 111, 117, 116, 105, 110, 101, 32, 115, 116, 97, 99, 107, 32, 58, 10, 91, 99, 48, 44, 32, 99, 49, 44, 32, 99, 50, 44, 32, 99, 51, 44, 32, 99, 52, 44, 32, 46, 46, 46, 93, 32, 115, 46, 116, 46, 32, 99, 32, 61, 32, 97, 32, 42, 32, 98, 10, 83, 101, 101, 32, 115, 101, 99, 116, 105, 111, 110, 32, 51, 46, 50, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 101, 112, 114, 105, 110, 116, 46, 105, 97, 99, 114, 46, 111, 114, 103, 47, 50, 48, 50, 50, 47, 50, 55, 52, 46, 112, 100, 102, 10, 70, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 104, 105, 103, 104, 32, 108, 101, 118, 101, 108, 32, 108, 97, 110, 103, 117, 97, 103, 101, 44, 32, 115, 101, 101, 10, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 54, 55, 54, 45, 76, 54, 56, 57, 1, 0, 0, 105, 0, 119, 111, 7, 119, 113, 7, 3, 118, 114, 7, 3, 117, 115, 7, 3, 116, 116, 7, 3, 119, 112, 7, 119, 114, 7, 3, 118, 115, 7, 3, 117, 116, 7, 3, 121, 117, 7, 7, 3, 119, 113, 7, 119, 115, 7, 3, 118, 116, 7, 3, 122, 117, 7, 7, 3, 121, 118, 7, 7, 3, 119, 114, 7, 119, 116, 7, 3, 123, 117, 7, 7, 3, 122, 118, 7, 7, 3, 121, 119, 7, 7, 3, 156, 152, 7, 159, 153, 7, 7, 3, 157, 153, 7, 7, 3, 155, 153, 7, 7, 3, 153, 153, 7, 7, 3, 6, 115, 113, 117, 97, 114, 101, 45, 2, 71, 105, 118, 101, 110, 32, 111, 110, 101, 32, 71, 70, 40, 112, 94, 53, 41, 32, 101, 108, 101, 109, 101, 110, 116, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 109, 111, 100, 117, 108, 97, 114, 10, 115, 113, 117, 97, 114, 105, 110, 103, 32, 40, 32, 105, 110, 99, 108, 117, 100, 105, 110, 103, 32, 114, 101, 100, 117, 99, 116, 105, 111, 110, 32, 98, 121, 32, 105, 114, 114, 101, 100, 117, 99, 105, 98, 108, 101, 32, 112, 111, 108, 121, 110, 111, 109, 105, 97, 108, 32, 41, 10, 111, 118, 101, 114, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 102, 105, 101, 108, 100, 32, 71, 70, 40, 112, 94, 53, 41, 32, 115, 46, 116, 46, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 45, 32, 50, 94, 51, 50, 32, 43, 32, 49, 10, 84, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 104, 97, 115, 32, 115, 97, 109, 101, 32, 101, 102, 102, 101, 99, 116, 32, 97, 115, 32, 99, 97, 108, 108, 105, 110, 103, 32, 109, 117, 108, 40, 97, 44, 32, 97, 41, 32, 124, 32, 97, 32, 226, 136, 136, 32, 71, 70, 40, 112, 94, 53, 41, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 46, 46, 46, 93, 10, 65, 102, 116, 101, 114, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 114, 111, 117, 116, 105, 110, 101, 32, 115, 116, 97, 99, 107, 32, 58, 10, 91, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 46, 46, 46, 93, 32, 115, 46, 116, 46, 32, 98, 32, 61, 32, 97, 32, 42, 32, 97, 10, 83, 101, 101, 32, 115, 101, 99, 116, 105, 111, 110, 32, 51, 46, 50, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 101, 112, 114, 105, 110, 116, 46, 105, 97, 99, 114, 46, 111, 114, 103, 47, 50, 48, 50, 50, 47, 50, 55, 52, 46, 112, 100, 102, 10, 70, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 104, 105, 103, 104, 32, 108, 101, 118, 101, 108, 32, 108, 97, 110, 103, 117, 97, 103, 101, 44, 32, 115, 101, 101, 10, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 55, 48, 57, 45, 76, 55, 49, 53, 1, 0, 0, 67, 0, 112, 113, 7, 115, 112, 7, 7, 3, 114, 113, 7, 7, 3, 114, 112, 7, 7, 114, 114, 7, 7, 3, 116, 117, 7, 7, 3, 113, 114, 7, 115, 114, 7, 7, 3, 117, 117, 7, 7, 3, 114, 114, 7, 7, 117, 118, 7, 7, 3, 118, 117, 7, 7, 3, 114, 152, 7, 155, 153, 7, 7, 3, 153, 153, 7, 7, 3, 14, 102, 114, 111, 98, 101, 110, 105, 117, 115, 95, 111, 110, 99, 101, 0, 0, 0, 0, 0, 9, 0, 151, 7, 151, 7, 151, 7, 151, 7, 151, 15, 102, 114, 111, 98, 101, 110, 105, 117, 115, 95, 116, 119, 105, 99, 101, 0, 0, 0, 0, 0, 9, 0, 151, 7, 151, 7, 151, 7, 151, 7, 151, 3, 105, 110, 118, 0, 2, 71, 105, 118, 101, 110, 32, 111, 110, 101, 32, 71, 70, 40, 112, 94, 53, 41, 32, 101, 108, 101, 109, 101, 110, 116, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 118, 101, 10, 105, 110, 118, 101, 114, 115, 101, 32, 111, 118, 101, 114, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 102, 105, 101, 108, 100, 32, 71, 70, 40, 112, 94, 53, 41, 32, 115, 46, 116, 46, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 45, 32, 50, 94, 51, 50, 32, 43, 32, 49, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 46, 46, 46, 93, 10, 65, 102, 116, 101, 114, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 114, 111, 117, 116, 105, 110, 101, 32, 115, 116, 97, 99, 107, 32, 58, 10, 91, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 46, 46, 46, 93, 32, 115, 46, 116, 46, 32, 98, 32, 61, 32, 49, 32, 47, 32, 97, 10, 83, 101, 101, 32, 115, 101, 99, 116, 105, 111, 110, 32, 51, 46, 50, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 101, 112, 114, 105, 110, 116, 46, 105, 97, 99, 114, 46, 111, 114, 103, 47, 50, 48, 50, 50, 47, 50, 55, 52, 46, 112, 100, 102, 10, 70, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 104, 105, 103, 104, 32, 108, 101, 118, 101, 108, 32, 108, 97, 110, 103, 117, 97, 103, 101, 44, 32, 115, 101, 101, 10, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 55, 53, 49, 45, 76, 55, 55, 53, 10, 78, 111, 116, 101, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 119, 105, 108, 108, 32, 110, 111, 116, 32, 112, 97, 110, 105, 99, 32, 101, 118, 101, 110, 32, 119, 104, 101, 110, 32, 111, 112, 101, 114, 97, 110, 100, 32, 96, 97, 96, 32, 105, 115, 32, 122, 101, 114, 111, 46, 1, 0, 0, 51, 0, 254, 213, 0, 1, 0, 114, 211, 4, 0, 254, 217, 0, 1, 0, 114, 211, 4, 0, 211, 2, 0, 254, 222, 0, 1, 0, 114, 211, 5, 0, 211, 2, 0, 152, 111, 7, 153, 116, 7, 7, 3, 153, 115, 7, 7, 3, 153, 114, 7, 7, 3, 153, 113, 7, 7, 3, 110, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 21, 3, 12, 152, 111, 7, 152, 112, 7, 152, 113, 7, 152, 114, 7, 152, 152, 7, 3, 100, 105, 118, 219, 1, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 71, 70, 40, 112, 94, 53, 41, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 40, 32, 115, 97, 121, 32, 97, 44, 32, 98, 32, 41, 32, 111, 110, 32, 115, 116, 97, 99, 107, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 10, 109, 111, 100, 117, 108, 97, 114, 32, 100, 105, 118, 105, 115, 105, 111, 110, 32, 111, 118, 101, 114, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 102, 105, 101, 108, 100, 32, 71, 70, 40, 112, 94, 53, 41, 32, 115, 46, 116, 46, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 45, 32, 50, 94, 51, 50, 32, 43, 32, 49, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 32, 58, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 46, 46, 46, 93, 10, 65, 102, 116, 101, 114, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 111, 102, 32, 114, 111, 117, 116, 105, 110, 101, 32, 115, 116, 97, 99, 107, 32, 58, 10, 91, 99, 48, 44, 32, 99, 49, 44, 32, 99, 50, 44, 32, 99, 51, 44, 32, 99, 52, 44, 32, 46, 46, 46, 93, 32, 115, 46, 116, 46, 32, 99, 32, 61, 32, 97, 32, 47, 32, 98, 10, 83, 101, 101, 32, 115, 101, 99, 116, 105, 111, 110, 32, 51, 46, 50, 32, 111, 102, 32, 104, 116, 116, 112, 115, 58, 47, 47, 101, 112, 114, 105, 110, 116, 46, 105, 97, 99, 114, 46, 111, 114, 103, 47, 50, 48, 50, 50, 47, 50, 55, 52, 46, 112, 100, 102, 10, 70, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 104, 105, 103, 104, 32, 108, 101, 118, 101, 108, 32, 108, 97, 110, 103, 117, 97, 103, 101, 44, 32, 115, 101, 101, 10, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 55, 55, 55, 45, 76, 55, 56, 49, 1, 0, 0, 3, 0, 254, 16, 1, 1, 0, 156, 211, 6, 0, 211, 2, 0, 12, 98, 97, 115, 101, 95, 109, 115, 113, 117, 97, 114, 101, 0, 0, 0, 0, 0, 5, 0, 130, 110, 24, 0, 0, 0, 0, 0, 0, 0, 0, 255, 7, 0, 5, 130, 110, 7, 130, 110, 24, 0, 0, 0, 0, 0, 0, 0, 0, 107, 9, 98, 97, 115, 101, 95, 115, 113, 114, 116, 0, 0, 0, 0, 0, 66, 2, 110, 185, 1, 31, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 110, 110, 7, 149, 110, 22, 0, 0, 0, 0, 0, 0, 0, 0, 3, 9, 110, 185, 1, 30, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 29, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 28, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 27, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 26, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 25, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 24, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 23, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 22, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 21, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 20, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 19, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 18, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 17, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 16, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 15, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 14, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 13, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 12, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 11, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 10, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 9, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 8, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 7, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 6, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 5, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 4, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 3, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 2, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 130, 211, 8, 0, 22, 0, 0, 0, 0, 255, 255, 255, 255, 111, 7, 149, 130, 112, 183, 112, 7, 150, 130, 150, 183, 130, 110, 22, 0, 0, 0, 0, 0, 0, 0, 0, 130, 22, 1, 0, 0, 0, 0, 0, 0, 0, 19, 130, 111, 7, 13, 98, 97, 115, 101, 95, 108, 101, 103, 101, 110, 100, 114, 101, 0, 0, 0, 0, 0, 8, 0, 254, 106, 3, 2, 0, 110, 7, 110, 254, 111, 3, 2, 0, 110, 7, 130, 110, 22, 0, 0, 0, 0, 0, 0, 0, 0, 3, 9, 8, 108, 101, 103, 101, 110, 100, 114, 101, 121, 1, 71, 105, 118, 101, 110, 32, 97, 110, 32, 101, 108, 101, 109, 101, 110, 116, 32, 118, 32, 226, 136, 136, 32, 71, 70, 40, 112, 94, 53, 41, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 99, 111, 109, 112, 117, 116, 101, 115, 32, 105, 116, 115, 32, 108, 101, 103, 101, 110, 100, 114, 101, 32, 115, 121, 109, 98, 111, 108, 44, 10, 119, 104, 105, 99, 104, 32, 105, 115, 32, 97, 110, 32, 101, 108, 101, 109, 101, 110, 116, 32, 226, 136, 136, 32, 71, 70, 40, 112, 41, 32, 124, 32, 112, 32, 61, 32, 50, 94, 54, 52, 32, 45, 32, 50, 94, 51, 50, 32, 43, 32, 49, 10, 65, 116, 32, 98, 101, 103, 105, 110, 110, 105, 110, 103, 32, 115, 116, 97, 99, 107, 32, 108, 111, 111, 107, 115, 32, 108, 105, 107, 101, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 46, 46, 46, 93, 10, 65, 116, 32, 101, 110, 100, 32, 115, 116, 97, 99, 107, 32, 108, 111, 111, 107, 115, 32, 108, 105, 107, 101, 10, 91, 98, 44, 32, 46, 46, 46, 93, 32, 115, 46, 116, 46, 32, 98, 32, 61, 32, 108, 101, 103, 101, 110, 100, 114, 101, 32, 115, 121, 109, 98, 111, 108, 32, 111, 102, 32, 97, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 56, 53, 55, 45, 76, 56, 55, 55, 10, 102, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 104, 105, 103, 104, 101, 114, 32, 108, 101, 118, 101, 108, 32, 108, 97, 110, 103, 117, 97, 103, 101, 46, 1, 0, 0, 31, 0, 254, 122, 3, 1, 0, 114, 211, 4, 0, 254, 126, 3, 1, 0, 114, 211, 4, 0, 211, 2, 0, 254, 131, 3, 1, 0, 114, 211, 5, 0, 211, 2, 0, 152, 7, 152, 152, 7, 7, 3, 151, 151, 7, 7, 3, 150, 150, 7, 7, 3, 149, 149, 7, 7, 3, 211, 10, 0, 4, 115, 113, 114, 116, 20, 2, 71, 105, 118, 101, 110, 32, 97, 110, 32, 101, 108, 101, 109, 101, 110, 116, 32, 118, 32, 226, 136, 136, 32, 71, 70, 40, 112, 94, 53, 41, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 97, 116, 116, 101, 109, 112, 116, 115, 32, 116, 111, 32, 99, 111, 109, 112, 117, 116, 101, 32, 115, 113, 117, 97, 114, 101, 32, 114, 111, 111, 116, 32, 111, 102, 32, 118, 44, 10, 105, 102, 32, 116, 104, 97, 116, 32, 110, 117, 109, 98, 101, 114, 32, 105, 115, 32, 97, 32, 115, 113, 117, 97, 114, 101, 46, 10, 65, 116, 32, 98, 101, 103, 105, 110, 110, 105, 110, 103, 32, 115, 116, 97, 99, 107, 32, 108, 111, 111, 107, 115, 32, 108, 105, 107, 101, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 46, 46, 46, 93, 10, 65, 116, 32, 101, 110, 100, 32, 115, 116, 97, 99, 107, 32, 108, 111, 111, 107, 115, 32, 108, 105, 107, 101, 10, 91, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 102, 108, 103, 44, 32, 46, 46, 46, 93, 10, 73, 102, 32, 102, 108, 103, 32, 61, 32, 49, 44, 32, 105, 116, 32, 100, 101, 110, 111, 116, 101, 115, 32, 118, 39, 32, 61, 32, 123, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 125, 32, 105, 115, 32, 115, 113, 117, 97, 114, 101, 32, 114, 111, 111, 116, 32, 111, 102, 32, 118, 32, 105, 46, 101, 46, 32, 118, 39, 32, 42, 32, 118, 39, 32, 61, 32, 118, 32, 40, 32, 109, 111, 100, 32, 71, 70, 40, 112, 94, 53, 41, 32, 41, 10, 73, 102, 32, 102, 108, 103, 32, 61, 32, 48, 44, 32, 116, 104, 101, 110, 32, 118, 39, 32, 61, 32, 123, 48, 44, 32, 48, 44, 32, 48, 44, 32, 48, 44, 32, 48, 125, 44, 32, 100, 101, 110, 111, 116, 105, 110, 103, 32, 118, 32, 100, 111, 101, 115, 110, 39, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 113, 117, 97, 114, 101, 32, 114, 111, 111, 116, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 56, 55, 57, 45, 76, 57, 49, 48, 10, 102, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 105, 110, 32, 104, 105, 103, 104, 101, 114, 32, 108, 101, 118, 101, 108, 32, 108, 97, 110, 103, 117, 97, 103, 101, 46, 1, 0, 0, 41, 0, 254, 161, 3, 1, 0, 114, 254, 164, 3, 2, 0, 254, 165, 3, 1, 0, 114, 211, 2, 0, 254, 170, 3, 1, 0, 114, 254, 173, 3, 2, 0, 254, 174, 3, 1, 0, 114, 211, 2, 0, 211, 7, 0, 254, 180, 3, 1, 0, 119, 211, 2, 0, 254, 184, 3, 1, 0, 114, 211, 5, 0, 211, 2, 0, 211, 4, 0, 254, 190, 3, 1, 0, 114, 211, 3, 0, 157, 7, 130, 160, 7, 7, 3, 130, 158, 7, 7, 3, 130, 156, 7, 7, 3, 130, 154, 7, 7, 3, 211, 9, 0, 254, 217, 3, 1, 0, 153, 211, 6, 0, 254, 221, 3, 3, 0, 151, 115, 7, 152, 107, 2, 101, 113, 87, 1, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 97, 44, 32, 98, 32, 226, 136, 136, 32, 71, 70, 40, 112, 94, 53, 41, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 112, 114, 111, 100, 117, 99, 101, 115, 32, 115, 105, 110, 103, 108, 101, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 32, 114, 44, 10, 100, 101, 110, 111, 116, 105, 110, 103, 32, 119, 104, 101, 116, 104, 101, 114, 32, 97, 32, 61, 61, 32, 98, 46, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 46, 46, 46, 93, 10, 70, 105, 110, 97, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 114, 44, 32, 46, 46, 46, 93, 10, 73, 102, 32, 97, 32, 61, 61, 32, 98, 32, 123, 32, 114, 32, 61, 32, 49, 32, 125, 32, 69, 108, 115, 101, 32, 123, 32, 114, 32, 61, 32, 48, 32, 125, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 55, 57, 55, 45, 76, 56, 48, 54, 10, 102, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 46, 1, 0, 0, 21, 0, 185, 1, 1, 0, 0, 0, 0, 0, 0, 0, 130, 153, 21, 18, 130, 152, 21, 18, 130, 151, 21, 18, 130, 150, 21, 18, 130, 149, 21, 18, 3, 110, 101, 113, 87, 1, 71, 105, 118, 101, 110, 32, 116, 119, 111, 32, 101, 108, 101, 109, 101, 110, 116, 115, 32, 97, 44, 32, 98, 32, 226, 136, 136, 32, 71, 70, 40, 112, 94, 53, 41, 44, 32, 116, 104, 105, 115, 32, 114, 111, 117, 116, 105, 110, 101, 32, 112, 114, 111, 100, 117, 99, 101, 115, 32, 115, 105, 110, 103, 108, 101, 32, 102, 105, 101, 108, 100, 32, 101, 108, 101, 109, 101, 110, 116, 32, 114, 44, 10, 100, 101, 110, 111, 116, 105, 110, 103, 32, 119, 104, 101, 116, 104, 101, 114, 32, 97, 32, 33, 61, 32, 98, 46, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 97, 48, 44, 32, 97, 49, 44, 32, 97, 50, 44, 32, 97, 51, 44, 32, 97, 52, 44, 32, 98, 48, 44, 32, 98, 49, 44, 32, 98, 50, 44, 32, 98, 51, 44, 32, 98, 52, 44, 32, 46, 46, 46, 93, 10, 70, 105, 110, 97, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 97, 116, 101, 10, 91, 114, 44, 32, 46, 46, 46, 93, 10, 73, 102, 32, 97, 32, 33, 61, 32, 98, 32, 123, 32, 114, 32, 61, 32, 49, 32, 125, 32, 69, 108, 115, 101, 32, 123, 32, 114, 32, 61, 32, 48, 32, 125, 10, 83, 101, 101, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 112, 111, 114, 110, 105, 110, 47, 101, 99, 103, 102, 112, 53, 47, 98, 108, 111, 98, 47, 99, 101, 48, 53, 57, 99, 54, 47, 112, 121, 116, 104, 111, 110, 47, 101, 99, 71, 70, 112, 53, 46, 112, 121, 35, 76, 56, 49, 51, 45, 76, 56, 50, 50, 10, 102, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 46, 1, 0, 0, 21, 0, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 130, 153, 23, 19, 130, 152, 23, 19, 130, 151, 23, 19, 130, 150, 23, 19, 130, 149, 23, 19]),
];
