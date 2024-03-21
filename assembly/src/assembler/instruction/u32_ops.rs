use super::{field_ops::append_pow2_op, push_u32_value, validate_param, SpanBuilder};
use crate::{
    diagnostics::{RelatedError, Report},
    AssemblyContext, AssemblyError, Span, MAX_U32_ROTATE_VALUE, MAX_U32_SHIFT_VALUE,
};
use vm_core::{
    AdviceInjector, Felt,
    Operation::{self, *},
    ZERO,
};

/// This enum is intended to determine the mode of operation passed to the parsing function
#[derive(PartialEq, Eq)]
pub enum U32OpMode {
    Wrapping,
    Overflowing,
}

// CONVERSIONS AND TESTS
// ================================================================================================

/// Translates u32testw assembly instruction to VM operations.
///
/// Implemented by executing DUP U32SPLIT SWAP DROP EQZ on each element in the word
/// and combining the results using AND operation (total of 23 VM cycles)
pub fn u32testw(span_builder: &mut SpanBuilder) {
    #[rustfmt::skip]
    let ops = [
         // Test the fourth element
        Dup3, U32split, Swap, Drop, Eqz,

        // Test the third element
        Dup3, U32split, Swap, Drop, Eqz, And,

         // Test the second element
        Dup2, U32split, Swap, Drop, Eqz, And,

        // Test the first element
        Dup1, U32split, Swap, Drop, Eqz, And,
    ];
    span_builder.push_ops(ops);
}

/// Translates u32assertw assembly instruction to VM operations.
///
/// Implemented by executing `U32ASSERT2` on each pair of elements in the word.
/// Total of 6 VM cycles.
pub fn u32assertw(span_builder: &mut SpanBuilder, err_code: Felt) {
    #[rustfmt::skip]
    let ops = [
        // Test the first and the second elements
        U32assert2(err_code),

        // Move 3 and 4 to the top of the stack
        MovUp3, MovUp3,

        // Test them
        U32assert2(err_code),

        // Move the elements back into place
        MovUp3, MovUp3,
    ];
    span_builder.push_ops(ops);
}

// ARITHMETIC OPERATIONS
// ================================================================================================

/// Translates u32add assembly instructions to VM operations.
///
/// The base operation is `U32ADD`, but depending on the mode, additional operations may be
/// inserted. Please refer to the docs of `handle_arithmetic_operation` for more details.
///
/// VM cycles per mode:
/// - u32wrapping_add: 2 cycles
/// - u32wrapping_add.b: 3 cycles
/// - u32overflowing_add: 1 cycles
/// - u32overflowing_add.b: 2 cycles
pub fn u32add(span_builder: &mut SpanBuilder, op_mode: U32OpMode, imm: Option<u32>) {
    handle_arithmetic_operation(span_builder, U32add, op_mode, imm);
}

/// Translates u32sub assembly instructions to VM operations.
///
/// The base operation is `U32SUB`, but depending on the mode, additional operations may be
/// inserted. Please refer to the docs of `handle_arithmetic_operation` for more details.
///
/// VM cycles per mode:
/// - u32wrapping_sub: 2 cycles
/// - u32wrapping_sub.b: 3 cycles
/// - u32overflowing_sub: 1 cycles
/// - u32overflowing_sub.b: 2 cycles
pub fn u32sub(span_builder: &mut SpanBuilder, op_mode: U32OpMode, imm: Option<u32>) {
    handle_arithmetic_operation(span_builder, U32sub, op_mode, imm);
}

/// Translates u32mul assembly instructions to VM operations.
///
/// The base operation is `U32MUL`, but depending on the mode, additional operations may be
/// inserted. Please refer to the docs of `handle_arithmetic_operation` for more details.
///
/// VM cycles per mode:
/// - u32wrapping_mul: 2 cycles
/// - u32wrapping_mul.b: 3 cycles
/// - u32overflowing_mul: 1 cycles
/// - u32overflowing_mul.b: 2 cycles
pub fn u32mul(span_builder: &mut SpanBuilder, op_mode: U32OpMode, imm: Option<u32>) {
    handle_arithmetic_operation(span_builder, U32mul, op_mode, imm);
}

/// Translates u32div assembly instructions to VM operations.
///
/// VM cycles per mode:
/// - u32div: 2 cycles
/// - u32div.b:
///    - 4 cycles if b is 1
///    - 3 cycles if b is not 1
pub fn u32div(
    span_builder: &mut SpanBuilder,
    ctx: &AssemblyContext,
    imm: Option<Span<u32>>,
) -> Result<(), AssemblyError> {
    handle_division(span_builder, ctx, imm)?;
    span_builder.push_op(Drop);
    Ok(())
}

/// Translates u32mod assembly instructions to VM operations.
///
/// VM cycles per mode:
/// - u32mod: 3 cycle
/// - u32mod.b:
///    - 5 cycles if b is 1
///    - 4 cycles if b is not 1
pub fn u32mod(
    span_builder: &mut SpanBuilder,
    ctx: &AssemblyContext,
    imm: Option<Span<u32>>,
) -> Result<(), AssemblyError> {
    handle_division(span_builder, ctx, imm)?;
    span_builder.push_ops([Swap, Drop]);
    Ok(())
}

/// Translates u32divmod assembly instructions to VM operations.
///
/// VM cycles per mode:
/// - u32divmod: 1 cycle
/// - u32divmod.b:
///    - 3 cycles if b is 1
///    - 2 cycles if b is not 1
pub fn u32divmod(
    span_builder: &mut SpanBuilder,
    ctx: &AssemblyContext,
    imm: Option<Span<u32>>,
) -> Result<(), AssemblyError> {
    handle_division(span_builder, ctx, imm)
}

// BITWISE OPERATIONS
// ================================================================================================

/// Translates u32not assembly instruction to VM operations.
///
/// The reason this method works is because 2^32 -1 provides a bit mask of ones, which after
/// subtracting the element, flips the bits of the original value to perform a bitwise NOT.
///
/// This takes 5 VM cycles.
pub fn u32not(span_builder: &mut SpanBuilder) {
    #[rustfmt::skip]
    let ops = [
        // Perform the operation
        Push(Felt::from(u32::MAX)),
        U32assert2(ZERO),
        Swap,
        U32sub,

        // Drop the underflow flag
        Drop,
    ];
    span_builder.push_ops(ops);
}

/// Translates u32shl assembly instructions to VM operations.
///
/// The operation is implemented by putting a power of 2 on the stack, then multiplying it with
/// the value to be shifted and splitting the result.
///
/// VM cycles per mode:
/// - u32shl: 18 cycles
/// - u32shl.b: 3 cycles
pub fn u32shl(span_builder: &mut SpanBuilder, imm: Option<u8>) -> Result<(), AssemblyError> {
    prepare_bitwise::<MAX_U32_SHIFT_VALUE>(span_builder, imm)?;
    if imm != Some(0) {
        span_builder.push_ops([U32mul, Drop]);
    }
    Ok(())
}

/// Translates u32shr assembly instructions to VM operations.
///
/// The operation is implemented by putting a power of 2 on the stack, then dividing the value to
/// be shifted by it and returning the quotient.
///
/// VM cycles per mode:
/// - u32shr: 18 cycles
/// - u32shr.b: 3 cycles
pub fn u32shr(span_builder: &mut SpanBuilder, imm: Option<u8>) -> Result<(), AssemblyError> {
    prepare_bitwise::<MAX_U32_SHIFT_VALUE>(span_builder, imm)?;
    if imm != Some(0) {
        span_builder.push_ops([U32div, Drop]);
    }
    Ok(())
}

/// Translates u32rotl assembly instructions to VM operations.
///
/// The base operation is implemented by putting a power of 2 on the stack, then multiplying the
/// value to be shifted by it and adding the overflow limb to the shifted limb.
///
/// VM cycles per mode:
/// - u32rotl: 18 cycles
/// - u32rotl.b: 3 cycles
pub fn u32rotl(span_builder: &mut SpanBuilder, imm: Option<u8>) -> Result<(), AssemblyError> {
    prepare_bitwise::<MAX_U32_ROTATE_VALUE>(span_builder, imm)?;
    if imm != Some(0) {
        span_builder.push_ops([U32mul, Add]);
    }
    Ok(())
}

/// Translates u32rotr assembly instructions to VM operations.
///
/// The base operation is implemented by multiplying the value to be shifted by 2^(32-b), where
/// b is the shift amount, then adding the overflow limb to the shifted limb.
///
/// VM cycles per mode:
/// - u32rotr: 22 cycles
/// - u32rotr.b: 3 cycles
pub fn u32rotr(span_builder: &mut SpanBuilder, imm: Option<u8>) -> Result<(), AssemblyError> {
    match imm {
        Some(0) => {
            // if rotation is performed by 0, do nothing (Noop)
            span_builder.push_op(Noop);
            return Ok(());
        }
        Some(imm) => {
            validate_param(imm, 1..=MAX_U32_ROTATE_VALUE)?;
            span_builder.push_op(Push(Felt::new(1 << (32 - imm))));
        }
        None => {
            span_builder.push_ops([Push(Felt::new(32)), Swap, U32sub, Drop]);
            append_pow2_op(span_builder);
        }
    }
    span_builder.push_ops([U32mul, Add]);
    Ok(())
}

/// Translates u32popcnt assembly instructions to VM operations.
///
/// This operation takes 33 cycles.
pub fn u32popcnt(span_builder: &mut SpanBuilder) {
    #[rustfmt::skip]
    let ops = [
        // i = i - ((i >> 1) & 0x55555555);
        Dup0,
        Push(Felt::new(1 << 1)), U32div, Drop,
        Push(Felt::new(0x55555555)),
        U32and,
        U32sub, Drop,
        // i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
        Dup0,
        Push(Felt::new(1 << 2)), U32div, Drop,
        Push(Felt::new(0x33333333)),
        U32and,
        Swap,
        Push(Felt::new(0x33333333)),
        U32and,
        U32add, Drop,
        // i = (i + (i >> 4)) & 0x0F0F0F0F;
        Dup0,
        Push(Felt::new(1 << 4)), U32div, Drop,
        U32add, Drop,
        Push(Felt::new(0x0F0F0F0F)),
        U32and,
        // return (i * 0x01010101) >> 24;
        Push(Felt::new(0x01010101)),
        U32mul, Drop,
        Push(Felt::new(1 << 24)), U32div, Drop
    ];
    span_builder.push_ops(ops);
}

/// Translates `u32clz` assembly instruction to VM operations. `u32clz` counts the number of
/// leading zeros of the value using non-deterministic technique (i.e. it takes help of advice
/// provider).
///
/// This operation takes 37 VM cycles.
pub fn u32clz(span: &mut SpanBuilder) {
    span.push_advice_injector(AdviceInjector::U32Clz);
    span.push_op(AdvPop); // [clz, n, ...]

    calculate_clz(span);
}

/// Translates `u32ctz` assembly instruction to VM operations. `u32ctz` counts the number of
/// trailing zeros of the value using non-deterministic technique (i.e. it takes help of advice
/// provider).
///
/// This operation takes 34 VM cycles.
pub fn u32ctz(span: &mut SpanBuilder) {
    span.push_advice_injector(AdviceInjector::U32Ctz);
    span.push_op(AdvPop); // [ctz, n, ...]

    calculate_ctz(span);
}

/// Translates `u32clo` assembly instruction to VM operations. `u32clo` counts the number of
/// leading ones of the value using non-deterministic technique (i.e. it takes help of advice
/// provider).
///
/// This operation takes 36 VM cycles.
pub fn u32clo(span: &mut SpanBuilder) {
    span.push_advice_injector(AdviceInjector::U32Clo);
    span.push_op(AdvPop); // [clo, n, ...]

    calculate_clo(span);
}

/// Translates `u32cto` assembly instruction to VM operations. `u32cto` counts the number of
/// trailing ones of the value using non-deterministic technique (i.e. it takes help of advice
/// provider).
///
/// This operation takes 33 VM cycles.
pub fn u32cto(span: &mut SpanBuilder) {
    span.push_advice_injector(AdviceInjector::U32Cto);
    span.push_op(AdvPop); // [cto, n, ...]

    calculate_cto(span);
}

/// Specifically handles these specific inputs per the spec.
/// - Wrapping: does not check if the inputs are u32 values; overflow or underflow bits are
///   discarded.
/// - Overflowing: does not check if the inputs are u32 values; overflow or underflow bits are
///   pushed onto the stack.
fn handle_arithmetic_operation(
    span_builder: &mut SpanBuilder,
    op: Operation,
    op_mode: U32OpMode,
    imm: Option<u32>,
) {
    if let Some(imm) = imm {
        push_u32_value(span_builder, imm);
    }

    span_builder.push_op(op);

    // in the wrapping mode, drop high 32 bits
    if matches!(op_mode, U32OpMode::Wrapping) {
        span_builder.push_op(Drop);
    }
}

/// Handles common parts of u32div, u32mod, and u32divmod operations, including handling of
/// immediate parameters.
fn handle_division(
    span_builder: &mut SpanBuilder,
    ctx: &AssemblyContext,
    imm: Option<Span<u32>>,
) -> Result<(), AssemblyError> {
    if let Some(imm) = imm {
        if imm == 0 {
            let source_file = ctx.unwrap_current_procedure().source_file();
            let error =
                Report::new(crate::parser::ParsingError::DivisionByZero { span: imm.span() });
            return if let Some(source_file) = source_file {
                Err(AssemblyError::Other(RelatedError::new(error.with_source_code(source_file))))
            } else {
                Err(AssemblyError::Other(RelatedError::new(error)))
            };
        }
        push_u32_value(span_builder, imm.into_inner());
    }

    span_builder.push_op(U32div);
    Ok(())
}

// BITWISE OPERATIONS - HELPERS
// ================================================================================================

/// Mutate the first two elements of the stack from `[b, a, ..]` into `[2^b, a, ..]`, with `b`
/// either as a provided immediate value, or as an element that already exists in the stack.
fn prepare_bitwise<const MAX_VALUE: u8>(
    span_builder: &mut SpanBuilder,
    imm: Option<u8>,
) -> Result<(), AssemblyError> {
    match imm {
        Some(0) => {
            // if shift/rotation is performed by 0, do nothing (Noop)
            span_builder.push_op(Noop);
        }
        Some(imm) => {
            validate_param(imm, 1..=MAX_VALUE)?;
            span_builder.push_op(Push(Felt::new(1 << imm)));
        }
        None => {
            append_pow2_op(span_builder);
        }
    }
    Ok(())
}

/// Appends relevant operations to the span block for the correctness check of the `U32Clz`
/// injector.
/// The idea is to compare the actual value with a bitmask consisting of `clz` leading ones to
/// check that every bit in `clz` leading bits is zero and `1` additional one to check that
/// `clz + 1`'th leading bit is one:
/// ```text
/// 000000000...000100...10 <-- actual value
/// └─ clz zeros ─┘
///
/// 1111111111...11100...00 <-- bitmask
/// └─  clz ones ─┘│
///                └─ additional one
/// ```
/// After applying a `u32and` bit operation on this values the result's leading `clz` bits should
/// be zeros, otherwise there were some ones in initial value's `clz` leading bits, and therefore
/// `clz` value is incorrect. `clz + 1`'th leading bit of the result should be one, otherwise this
/// bit in the initial value wasn't one and `clz` value is incorrect:
/// ```text
///  0000...00|1|10...10
/// &
///  1111...11|1|00...00
///  ↓↓↓↓   ↓↓ ↓
///  0000...00|1|00...00
/// ```
///
/// ---
/// The stack is expected to be arranged as follows (from the top):
/// - number of the leading zeros (`clz`), 1 element
/// - value for which we count the number of leading zeros (`n`), 1 element
///
/// After the operations are executed, the stack will be arranged as follows:
/// - number of the leading zeros (`clz`), 1 element
///
/// `[clz, n, ... ] -> [clz, ... ]`
///
/// VM cycles: 36
fn calculate_clz(span: &mut SpanBuilder) {
    // [clz, n, ...]
    #[rustfmt::skip]
    let ops_group_1 = [
        Swap, Push(32u8.into()), Dup2, Neg, Add // [32 - clz, n, clz, ...]
    ];
    span.push_ops(ops_group_1);

    append_pow2_op(span); // [pow2(32 - clz), n, clz, ...]

    #[rustfmt::skip]
    let ops_group_2 = [
        Push(Felt::new(u32::MAX as u64 + 1)), // [2^32, pow2(32 - clz), n, clz, ...]

        Dup1, Neg, Add, // [2^32 - pow2(32 - clz), pow2(32 - clz), n, clz, ...]
                        // `2^32 - pow2(32 - clz)` is equal to `clz` leading ones and `32 - clz`
                        // zeros:
                        // 1111111111...1110000...0
                        // └─ `clz` ones ─┘

        Swap, Push(2u8.into()), U32div, Drop, // [pow2(32 - clz) / 2, 2^32 - pow2(32 - clz), n, clz, ...]
                                              // pow2(32 - clz) / 2 is equal to `clz` leading
                                              // zeros, `1` one and all other zeros.

        Swap, Dup1, Add, // [bit_mask, pow2(32 - clz) / 2, n, clz, ...]
                         // 1111111111...111000...0 <-- bitmask
                         // └─  clz ones ─┘│
                         //                └─ additional one

        MovUp2, U32and, // [m, pow2(32 - clz) / 2, clz]
                        // If calcualtion of `clz` is correct, m should be equal to
                        // pow2(32 - clz) / 2

        Eq, Assert(0) // [clz, ...]
    ];

    span.push_ops(ops_group_2);
}

/// Appends relevant operations to the span block for the correctness check of the `U32Clo`
/// injector.
/// The idea is to compare the actual value with a bitmask consisting of `clo` leading ones to
/// check that every bit in `clo` leading bits is one and `1` additional one to check that
/// `clo + 1`'th leading bit is zero:
/// ```text
/// 11111111...111010...10 <-- actual value
/// └─ clo ones ─┘
///
/// 111111111...11100...00 <-- bitmask
/// └─ clo ones ─┘│
///               └─ additional one
/// ```
/// After applying a `u32and` bit operation on this values the result's leading `clo` bits should
/// be ones, otherwise there were some zeros in initial value's `clo` leading bits, and therefore
/// `clo` value is incorrect. `clo + 1`'th leading bit of the result should be zero, otherwise this
/// bit in the initial value wasn't zero and `clo` value is incorrect:
/// ```text
///  1111...11|0|10...10
/// &
///  1111...11|1|00...00
///  ↓↓↓↓   ↓↓ ↓
///  1111...11|0|00...00
/// ```
///
/// ---
/// The stack is expected to be arranged as follows (from the top):
/// - number of the leading ones (`clo`), 1 element
/// - value for which we count the number of leading ones (`n`), 1 element
///
/// After the operations are executed, the stack will be arranged as follows:
/// - number of the leading ones (`clo`), 1 element
///
/// `[clo, n, ... ] -> [clo, ... ]`
///
/// VM cycles: 35
fn calculate_clo(span: &mut SpanBuilder) {
    // [clo, n, ...]
    #[rustfmt::skip]
    let ops_group_1 = [
        Swap, Push(32u8.into()), Dup2, Neg, Add // [32 - clo, n, clo, ...]
    ];
    span.push_ops(ops_group_1);

    append_pow2_op(span); // [pow2(32 - clo), n, clo, ...]

    #[rustfmt::skip]
    let ops_group_2 = [
        Push(Felt::new(u32::MAX as u64 + 1)), // [2^32, pow2(32 - clo), n, clo, ...]

        Dup1, Neg, Add, // [2^32 - pow2(32 - clo), pow2(32 - clo), n, clo, ...]
                        // `2^32 - pow2(32 - clo)` is equal to `clo` leading ones and `32 - clo`
                        // zeros:
                        // 11111111...1110000...0
                        // └─ clo ones ─┘

        Swap, Push(2u8.into()), U32div, Drop, // [pow2(32 - clo) / 2, 2^32 - pow2(32 - clo), n, clo, ...]
                                              // pow2(32 - clo) / 2 is equal to `clo` leading
                                              // zeros, `1` one and all other zeros.

        Dup1, Add, // [bit_mask, 2^32 - pow2(32 - clo), n, clo, ...]
                   // 111111111...111000...0 <-- bitmask
                   // └─ clo ones ─┘│
                   //               └─ additional one

        MovUp2, U32and, // [m, 2^32 - pow2(32 - clo), clo]
                        // If calcualtion of `clo` is correct, m should be equal to
                        // 2^32 - pow2(32 - clo)

        Eq, Assert(0) // [clo, ...]
    ];

    span.push_ops(ops_group_2);
}

/// Appends relevant operations to the span block for the correctness check of the `U32Ctz`
/// injector.
/// The idea is to compare the actual value with a bitmask consisting of `ctz` trailing ones to
/// check that every bit in `ctz` trailing bits is zero and `1` additional one to check that
/// `ctz + 1`'th trailing bit is one:
/// ```text
/// 10..001000000000000000 <-- actual value
///        └─ ctz zeros ─┘
///
/// 00..0011111111111...11 <-- bitmask
///       │└─  ctz ones ─┘
///       └─ additional one
/// ```
/// After applying a `u32and` bit operation on this values the result's trailing `ctz` bits should
/// be zeros, otherwise there were some ones in initial value's `ctz` trailing bits, and therefore
/// `ctz` value is incorrect. `ctz + 1`'th trailing bit of the result should be one, otherwise this
/// bit in the initial value wasn't one and `ctz` value is incorrect:
/// ```text
///  10...10|1|00...00
/// &
///  00...00|1|11...11
/// =        ↓ ↓↓   ↓↓
///  00...00|1|00...00
/// ```
///
/// ---
/// The stack is expected to be arranged as follows (from the top):
/// - number of the trailing zeros (`ctz`), 1 element
/// - value for which we count the number of trailing zeros (`n`), 1 element
///
/// After the operations are executed, the stack will be arranged as follows:
/// - number of the trailing zeros (`ctz`), 1 element
///
/// `[ctz, n, ... ] -> [ctz, ... ]`
///
/// VM cycles: 33
fn calculate_ctz(span: &mut SpanBuilder) {
    // [ctz, n, ...]
    #[rustfmt::skip]
    let ops_group_1 = [
        Swap, Dup1, // [ctz, n, ctz, ...]
    ];
    span.push_ops(ops_group_1);

    append_pow2_op(span); // [pow2(ctz), n, ctz, ...]

    #[rustfmt::skip]
    let ops_group_2 = [
        Dup0, // [pow2(ctz), pow2(ctz), n, ctz, ...]
              // pow2(ctz) is equal to all zeros with only one on the `ctz`'th trailing position

        Pad, Incr, Neg, Add, // [pow2(ctz) - 1, pow2(ctz), n, ctz, ...]

        Swap, U32split, Drop, // [pow2(ctz), pow2(ctz) - 1, n, ctz, ...]
                              // We need to drop the high bits of `pow2(ctz)` because if `ctz`
                              // equals 32 `pow2(ctz)` will exceed the u32. Also in that case there
                              // is no need to check the dividing one, since it is absent (value is
                              // all 0's).

        Dup0, MovUp2, Add, // [bit_mask, pow2(ctz), n, ctz]
                           // 00..001111111111...11 <-- bitmask
                           //       │└─ ctz ones ─┘
                           //       └─ additional one

        MovUp2, U32and, // [m, pow2(ctz), ctz]
                        // If calcualtion of `ctz` is correct, m should be equal to
                        // pow2(ctz)

        Eq, Assert(0), // [ctz, ...]
    ];

    span.push_ops(ops_group_2);
}

/// Appends relevant operations to the span block for the correctness check of the `U32Cto`
/// injector.
/// The idea is to compare the actual value with a bitmask consisting of `cto` trailing ones to
/// check that every bit in `cto` trailing bits is one and `1` additional one to check that
/// `cto + 1`'th trailing bit is zero:
/// ```text
/// 10..01011111111111111 <-- actual value
///        └─ cto ones ─┘
///
/// 00..001111111111...11 <-- bitmask
///       │└─ cto ones ─┘
///       └─ additional one
/// ```
/// After applying a `u32and` bit operation on this values the result's trailing `cto` bits should
/// be ones, otherwise there were some zeros in initial value's `cto` trailing bits, and therefore
/// `cto` value is incorrect. `cto + 1`'th trailing bit of the result should be zero, otherwise
/// this bit in the initial value wasn't zero and `cto` value is incorrect:
/// ```text
///  10...11|0|11...11
/// &
///  00...00|1|11...11
/// =        ↓ ↓↓   ↓↓
///  00...00|0|11...11
/// ```
///
/// ---
/// The stack is expected to be arranged as follows (from the top):
/// - number of the trailing ones (`cto`), 1 element
/// - value for which we count the number of trailing zeros (`n`), 1 element
///
/// After the operations are executed, the stack will be arranged as follows:
/// - number of the trailing zeros (`cto`), 1 element
///
/// `[cto, n, ... ] -> [cto, ... ]`
///
/// VM cycles: 32
fn calculate_cto(span: &mut SpanBuilder) {
    // [cto, n, ...]
    #[rustfmt::skip]
    let ops_group_1 = [
        Swap, Dup1, // [cto, n, cto, ...]
    ];
    span.push_ops(ops_group_1);

    append_pow2_op(span); // [pow2(cto), n, cto, ...]

    #[rustfmt::skip]
    let ops_group_2 = [
        Dup0, // [pow2(cto), pow2(cto), n, cto, ...]
              // pow2(cto) is equal to all zeros with only one on the `cto`'th trailing position

        Pad, Incr, Neg, Add, // [pow2(cto) - 1, pow2(cto), n, cto, ...]

        Swap, U32split, Drop, // [pow2(cto), pow2(cto) - 1, n, cto, ...]
                              // We need to drop the high bits of `pow2(cto)` because if `cto`
                              // equals 32 `pow2(cto)` will exceed the u32. Also in that case there
                              // is no need to check the dividing zero, since it is absent (value
                              // is all 1's).

        Dup1, Add, // [bit_mask, pow2(cto) - 1, n, cto]
                   // 00..001111111111...11 <-- bitmask
                   //       │└─ cto ones ─┘
                   //       └─ additional one

        MovUp2, U32and, // [m, pow2(cto) - 1, cto]
                        // If calcualtion of `cto` is correct, m should be equal to
                        // pow2(cto) - 1

        Eq, Assert(0), // [cto, ...]
    ];

    span.push_ops(ops_group_2);
}

// COMPARISON OPERATIONS
// ================================================================================================

/// Translates u32lt assembly instructions to VM operations.
///
/// This operation takes 3 cycles.
pub fn u32lt(span_builder: &mut SpanBuilder) {
    compute_lt(span_builder);
}

/// Translates u32lte assembly instructions to VM operations.
///
/// This operation takes 5 cycles.
pub fn u32lte(span_builder: &mut SpanBuilder) {
    // Compute the lt with reversed number to get a gt check
    span_builder.push_op(Swap);
    compute_lt(span_builder);

    // Flip the final results to get the lte results.
    span_builder.push_op(Not);
}

/// Translates u32gt assembly instructions to VM operations.
///
/// This operation takes 4 cycles.
pub fn u32gt(span_builder: &mut SpanBuilder) {
    // Reverse the numbers so we can get a gt check.
    span_builder.push_op(Swap);

    compute_lt(span_builder);
}

/// Translates u32gte assembly instructions to VM operations.
///
/// This operation takes 4 cycles.
pub fn u32gte(span_builder: &mut SpanBuilder) {
    compute_lt(span_builder);

    // Flip the final results to get the gte results.
    span_builder.push_op(Not);
}

/// Translates u32min assembly instructions to VM operations.
///
/// Specifically, we subtract the top value from the second to the top value (U32SUB), check the
/// underflow flag (EQZ), and perform a conditional swap (CSWAP) to have the max number in front.
/// Then we finally drop the top element to keep the min.
///
/// This operation takes 8 cycles.
pub fn u32min(span_builder: &mut SpanBuilder) {
    compute_max_and_min(span_builder);

    // Drop the max and keep the min
    span_builder.push_op(Drop);
}

/// Translates u32max assembly instructions to VM operations.
///
/// Specifically, we subtract the top value from the second to the top value (U32SUB), check the
/// underflow flag (EQZ), and perform a conditional swap (CSWAP) to have the max number in front.
/// Then we finally drop the 2nd element to keep the max.
///
/// This operation takes 9 cycles.
pub fn u32max(span_builder: &mut SpanBuilder) {
    compute_max_and_min(span_builder);

    // Drop the min and keep the max
    span_builder.push_ops([Swap, Drop]);
}

// COMPARISON OPERATIONS - HELPERS
// ================================================================================================

/// Inserts the VM operations to check if the second element is less than
/// the top element. This takes 3 cycles.
fn compute_lt(span_builder: &mut SpanBuilder) {
    span_builder.push_ops([
        U32sub, Swap, Drop, // Perform the operations
    ])
}

/// Duplicate the top two elements in the stack and determine the min and max between them.
///
/// The maximum number will be at the top of the stack and minimum will be at the 2nd index.
fn compute_max_and_min(span_builder: &mut SpanBuilder) {
    // Copy top two elements of the stack.
    span_builder.push_ops([Dup1, Dup1]);

    #[rustfmt::skip]
    span_builder.push_ops([
        U32sub, Swap, Drop,

        // Check the underflow flag, if it's zero
        // then the second number is equal or larger than the first.
        Eqz, CSwap,
    ]);
}
