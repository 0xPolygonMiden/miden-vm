use super::{AssemblyContext, AssemblyError, CodeBlock, Token, TokenStream};
pub use blocks::{combine_blocks, parse_body};
use u32_ops::U32OpMode;
use vm_core::{
    utils::{collections::Vec, string::ToString},
    AssemblyOp, Decorator, DecoratorList, Felt, FieldElement, Operation, StarkField,
};

mod blocks;
mod crypto_ops;
mod field_ops;
mod io_ops;
mod stack_ops;
mod u32_ops;

pub mod ast;
use ast::nodes::Instruction;

// OP PARSER
// ================================================================================================

/// Transforms an assembly instruction into a sequence of one or more VM instructions.
fn parse_op_instruction(
    instruction: &Instruction,
    span_ops: &mut Vec<Operation>,
    num_proc_locals: u32,
    decorators: &mut DecoratorList,
    in_debug_mode: bool,
) -> Result<(), AssemblyError> {
    let dec_len = decorators.len();
    // if assembler is in debug mode, populate decorators list with debug related
    // decorators like AsmOp.
    if in_debug_mode {
        decorators.push((
            span_ops.len(),
            Decorator::AsmOp(AssemblyOp::new(String::from("mem_storew.1"), 1)),
        ));
    }

    // based on the instruction, invoke the correct parser for the operation
    match instruction {
        // ----- field operations -----------------------------------------------------------------
        Instruction::Assert => field_ops::parse_assert(span_ops),
        Instruction::AssertEq => field_ops::parse_assert_eq(span_ops),
        Instruction::Assertz => field_ops::parse_assertz(span_ops),

        Instruction::Add => field_ops::parse_add(span_ops),
        Instruction::AddImm(felt) => field_ops::parse_add_imm(span_ops, *felt),

        Instruction::Sub => field_ops::parse_sub(span_ops),
        Instruction::SubImm(felt) => field_ops::parse_sub_imm(span_ops, *felt),
        Instruction::Mul => field_ops::parse_mul(span_ops),
        Instruction::MulImm(felt) => field_ops::parse_mul_imm(span_ops, *felt),
        Instruction::Div => field_ops::parse_div(span_ops),
        Instruction::DivImm(felt) => field_ops::parse_div_imm(span_ops, *felt),
        Instruction::Neg => field_ops::parse_neg(span_ops),
        Instruction::Inv => field_ops::parse_inv(span_ops),

        Instruction::Pow2 => field_ops::parse_pow2(span_ops),
        Instruction::Exp => field_ops::parse_exp(span_ops, 64),
        Instruction::ExpImm(felt) => field_ops::parse_exp_imm(span_ops, *felt),
        Instruction::ExpBitLength(bits_len) => field_ops::parse_exp(span_ops, *bits_len),

        Instruction::Not => field_ops::parse_not(span_ops),
        Instruction::And => field_ops::parse_and(span_ops),
        Instruction::Or => field_ops::parse_or(span_ops),
        Instruction::Xor => field_ops::parse_xor(span_ops),

        Instruction::Eq => field_ops::parse_eq(span_ops),
        Instruction::EqImm(felt) => field_ops::parse_eq_imm(span_ops, *felt),
        Instruction::Neq => field_ops::parse_neq(span_ops),
        Instruction::NeqImm(felt) => field_ops::parse_neq_imm(span_ops, *felt),
        Instruction::Lt => field_ops::parse_lt(span_ops),
        Instruction::Lte => field_ops::parse_lte(span_ops),
        Instruction::Gt => field_ops::parse_gt(span_ops),
        Instruction::Gte => field_ops::parse_gte(span_ops),
        Instruction::Eqw => field_ops::parse_eqw(span_ops),

        // ----- u32 operations -------------------------------------------------------------------
        Instruction::U32Test => u32_ops::parse_u32test(span_ops),
        Instruction::U32TestW => u32_ops::parse_u32testw(span_ops),
        Instruction::U32Assert => u32_ops::parse_u32assert(span_ops),
        Instruction::U32Assert2 => u32_ops::parse_u32assert2(span_ops),
        Instruction::U32AssertW => u32_ops::parse_u32assertw(span_ops),
        Instruction::U32Cast => u32_ops::parse_u32cast(span_ops),
        Instruction::U32Split => u32_ops::parse_u32split(span_ops),

        Instruction::U32CheckedAdd => {
            u32_ops::parse_u32add(instruction, span_ops, U32OpMode::Checked, None)
        }
        Instruction::U32CheckedAddImm(imm) => {
            u32_ops::parse_u32add(instruction, span_ops, U32OpMode::Checked, Some(*imm as u64))
        }
        Instruction::U32WrappingAdd => {
            u32_ops::parse_u32add(instruction, span_ops, U32OpMode::Wrapping, None)
        }
        Instruction::U32WrappingAddImm(imm) => {
            u32_ops::parse_u32add(instruction, span_ops, U32OpMode::Wrapping, Some(*imm))
        }
        Instruction::U32OverflowingAdd => {
            u32_ops::parse_u32add(instruction, span_ops, U32OpMode::Overflowing, None)
        }
        Instruction::U32OverflowingAddImm(imm) => {
            u32_ops::parse_u32add(instruction, span_ops, U32OpMode::Overflowing, Some(*imm))
        }

        Instruction::U32OverflowingAdd3 => u32_ops::parse_u32overflowing_add3(span_ops),
        Instruction::U32WrappingAdd3 => u32_ops::parse_u32wrapping_add3(span_ops),

        Instruction::U32CheckedSub => {
            u32_ops::parse_u32sub(instruction, span_ops, U32OpMode::Checked, None)
        }
        Instruction::U32CheckedSubImm(imm) => {
            u32_ops::parse_u32sub(instruction, span_ops, U32OpMode::Checked, Some(*imm as u64))
        }
        Instruction::U32WrappingSub => {
            u32_ops::parse_u32sub(instruction, span_ops, U32OpMode::Wrapping, None)
        }
        Instruction::U32WrappingSubImm(imm) => {
            u32_ops::parse_u32sub(instruction, span_ops, U32OpMode::Wrapping, Some(*imm))
        }
        Instruction::U32OverflowingSub => {
            u32_ops::parse_u32sub(instruction, span_ops, U32OpMode::Overflowing, None)
        }
        Instruction::U32OverflowingSubImm(imm) => {
            u32_ops::parse_u32sub(instruction, span_ops, U32OpMode::Overflowing, Some(*imm))
        }

        Instruction::U32CheckedMul => {
            u32_ops::parse_u32mul(instruction, span_ops, U32OpMode::Checked, None)
        }
        Instruction::U32CheckedMulImm(imm) => {
            u32_ops::parse_u32mul(instruction, span_ops, U32OpMode::Checked, Some(*imm as u64))
        }
        Instruction::U32WrappingMul => {
            u32_ops::parse_u32mul(instruction, span_ops, U32OpMode::Wrapping, None)
        }
        Instruction::U32WrappingMulImm(imm) => {
            u32_ops::parse_u32mul(instruction, span_ops, U32OpMode::Wrapping, Some(*imm))
        }
        Instruction::U32OverflowingMul => {
            u32_ops::parse_u32mul(instruction, span_ops, U32OpMode::Overflowing, None)
        }
        Instruction::U32OverflowingMulImm(imm) => {
            u32_ops::parse_u32mul(instruction, span_ops, U32OpMode::Overflowing, Some(*imm))
        }

        Instruction::U32OverflowingMadd => u32_ops::parse_u32overflowing_madd(span_ops),
        Instruction::U32WrappingMadd => u32_ops::parse_u32wrapping_madd(span_ops),

        Instruction::U32CheckedDiv => {
            u32_ops::parse_u32div(instruction, span_ops, U32OpMode::Checked, None)
        }
        Instruction::U32CheckedDivImm(imm) => {
            u32_ops::parse_u32div(instruction, span_ops, U32OpMode::Checked, Some(*imm as u64))
        }
        Instruction::U32UncheckedDiv => {
            u32_ops::parse_u32div(instruction, span_ops, U32OpMode::Unchecked, None)
        }
        Instruction::U32UncheckedDivImm(imm) => u32_ops::parse_u32div(
            instruction,
            span_ops,
            U32OpMode::Unchecked,
            Some(*imm as u64),
        ),
        Instruction::U32CheckedMod => {
            u32_ops::parse_u32mod(instruction, span_ops, U32OpMode::Checked, None)
        }
        Instruction::U32CheckedModImm(imm) => {
            u32_ops::parse_u32mod(instruction, span_ops, U32OpMode::Checked, Some(*imm as u64))
        }
        Instruction::U32UncheckedMod => {
            u32_ops::parse_u32mod(instruction, span_ops, U32OpMode::Unchecked, None)
        }
        Instruction::U32UncheckedModImm(imm) => u32_ops::parse_u32mod(
            instruction,
            span_ops,
            U32OpMode::Unchecked,
            Some(*imm as u64),
        ),

        Instruction::U32CheckedDivMod => {
            u32_ops::parse_u32divmod(instruction, span_ops, U32OpMode::Checked, None)
        }
        Instruction::U32CheckedDivModImm(imm) => {
            u32_ops::parse_u32divmod(instruction, span_ops, U32OpMode::Checked, Some(*imm as u64))
        }
        Instruction::U32UncheckedDivMod => {
            u32_ops::parse_u32divmod(instruction, span_ops, U32OpMode::Unchecked, None)
        }
        Instruction::U32UncheckedDivModImm(imm) => u32_ops::parse_u32divmod(
            instruction,
            span_ops,
            U32OpMode::Unchecked,
            Some(*imm as u64),
        ),

        Instruction::U32CheckedAnd => u32_ops::parse_u32and(span_ops),
        Instruction::U32CheckedOr => u32_ops::parse_u32or(span_ops),
        Instruction::U32CheckedXor => u32_ops::parse_u32xor(span_ops),
        Instruction::U32CheckedNot => u32_ops::parse_u32not(span_ops),

        Instruction::U32CheckedShr => u32_ops::parse_u32shr(span_ops, U32OpMode::Checked, None),
        Instruction::U32CheckedShrImm(imm) => {
            u32_ops::parse_u32shr(span_ops, U32OpMode::Checked, Some(*imm))
        }
        Instruction::U32UncheckedShr => u32_ops::parse_u32shr(span_ops, U32OpMode::Unchecked, None),
        Instruction::U32UncheckedShrImm(imm) => {
            u32_ops::parse_u32shr(span_ops, U32OpMode::Unchecked, Some(*imm))
        }

        Instruction::U32CheckedShl => u32_ops::parse_u32shl(span_ops, U32OpMode::Checked, None),
        Instruction::U32CheckedShlImm(imm) => {
            u32_ops::parse_u32shl(span_ops, U32OpMode::Checked, Some(*imm))
        }
        Instruction::U32UncheckedShl => u32_ops::parse_u32shl(span_ops, U32OpMode::Unchecked, None),
        Instruction::U32UncheckedShlImm(imm) => {
            u32_ops::parse_u32shl(span_ops, U32OpMode::Unchecked, Some(*imm))
        }

        Instruction::U32CheckedRotr => u32_ops::parse_u32rotr(span_ops, U32OpMode::Checked, None),
        Instruction::U32CheckedRotrImm(imm) => {
            u32_ops::parse_u32rotr(span_ops, U32OpMode::Checked, Some(*imm))
        }

        Instruction::U32UncheckedRotr => {
            u32_ops::parse_u32rotr(span_ops, U32OpMode::Unchecked, None)
        }
        Instruction::U32UncheckedRotrImm(imm) => {
            u32_ops::parse_u32rotr(span_ops, U32OpMode::Unchecked, Some(*imm))
        }

        Instruction::U32CheckedRotl => u32_ops::parse_u32rotl(span_ops, U32OpMode::Checked, None),
        Instruction::U32CheckedRotlImm(imm) => {
            u32_ops::parse_u32rotl(span_ops, U32OpMode::Checked, Some(*imm))
        }
        Instruction::U32UncheckedRotl => {
            u32_ops::parse_u32rotl(span_ops, U32OpMode::Unchecked, None)
        }
        Instruction::U32UncheckedRotlImm(imm) => {
            u32_ops::parse_u32rotl(span_ops, U32OpMode::Unchecked, Some(*imm))
        }

        Instruction::U32CheckedEq => u32_ops::parse_u32eq(instruction, span_ops, None),
        Instruction::U32CheckedEqImm(imm) => {
            u32_ops::parse_u32eq(instruction, span_ops, Some(*imm))
        }
        Instruction::U32CheckedNeq => u32_ops::parse_u32neq(instruction, span_ops, None),
        Instruction::U32CheckedNeqImm(imm) => {
            u32_ops::parse_u32neq(instruction, span_ops, Some(*imm))
        }

        Instruction::U32CheckedLt => u32_ops::parse_u32lt(span_ops, U32OpMode::Checked),
        Instruction::U32UncheckedLt => u32_ops::parse_u32lt(span_ops, U32OpMode::Unchecked),

        Instruction::U32CheckedLte => u32_ops::parse_u32lte(span_ops, U32OpMode::Checked),
        Instruction::U32UncheckedLte => u32_ops::parse_u32lte(span_ops, U32OpMode::Unchecked),

        Instruction::U32CheckedGt => u32_ops::parse_u32gt(span_ops, U32OpMode::Checked),
        Instruction::U32UncheckedGt => u32_ops::parse_u32gt(span_ops, U32OpMode::Unchecked),

        Instruction::U32CheckedGte => u32_ops::parse_u32gte(span_ops, U32OpMode::Checked),
        Instruction::U32UncheckedGte => u32_ops::parse_u32gte(span_ops, U32OpMode::Unchecked),

        Instruction::U32CheckedMin => u32_ops::parse_u32min(span_ops, U32OpMode::Checked),
        Instruction::U32UncheckedMin => u32_ops::parse_u32min(span_ops, U32OpMode::Unchecked),

        Instruction::U32CheckedMax => u32_ops::parse_u32max(span_ops, U32OpMode::Checked),
        Instruction::U32UncheckedMax => u32_ops::parse_u32max(span_ops, U32OpMode::Unchecked),

        // ----- stack manipulation ---------------------------------------------------------------
        Instruction::Drop => stack_ops::parse_drop(span_ops),
        Instruction::DropW => stack_ops::parse_dropw(span_ops),
        Instruction::PadW => stack_ops::parse_padw(span_ops),
        Instruction::Dup0 => stack_ops::parse_dup(span_ops, 0),
        Instruction::Dup1 => stack_ops::parse_dup(span_ops, 1),
        Instruction::Dup2 => stack_ops::parse_dup(span_ops, 2),
        Instruction::Dup3 => stack_ops::parse_dup(span_ops, 3),
        Instruction::Dup4 => stack_ops::parse_dup(span_ops, 4),
        Instruction::Dup5 => stack_ops::parse_dup(span_ops, 5),
        Instruction::Dup6 => stack_ops::parse_dup(span_ops, 6),
        Instruction::Dup7 => stack_ops::parse_dup(span_ops, 7),
        Instruction::Dup8 => stack_ops::parse_dup(span_ops, 8),
        Instruction::Dup9 => stack_ops::parse_dup(span_ops, 9),
        Instruction::Dup10 => stack_ops::parse_dup(span_ops, 10),
        Instruction::Dup11 => stack_ops::parse_dup(span_ops, 11),
        Instruction::Dup12 => stack_ops::parse_dup(span_ops, 12),
        Instruction::Dup13 => stack_ops::parse_dup(span_ops, 13),
        Instruction::Dup14 => stack_ops::parse_dup(span_ops, 14),
        Instruction::Dup15 => stack_ops::parse_dup(span_ops, 15),
        Instruction::DupW0 => stack_ops::parse_dupw(span_ops, 0),
        Instruction::DupW1 => stack_ops::parse_dupw(span_ops, 1),
        Instruction::DupW2 => stack_ops::parse_dupw(span_ops, 2),
        Instruction::DupW3 => stack_ops::parse_dupw(span_ops, 3),
        Instruction::Swap => stack_ops::parse_swap(span_ops, 1),
        Instruction::Swap2 => stack_ops::parse_swap(span_ops, 2),
        Instruction::Swap3 => stack_ops::parse_swap(span_ops, 3),
        Instruction::Swap4 => stack_ops::parse_swap(span_ops, 4),
        Instruction::Swap5 => stack_ops::parse_swap(span_ops, 5),
        Instruction::Swap6 => stack_ops::parse_swap(span_ops, 6),
        Instruction::Swap7 => stack_ops::parse_swap(span_ops, 7),
        Instruction::Swap8 => stack_ops::parse_swap(span_ops, 8),
        Instruction::Swap9 => stack_ops::parse_swap(span_ops, 9),
        Instruction::Swap10 => stack_ops::parse_swap(span_ops, 10),
        Instruction::Swap11 => stack_ops::parse_swap(span_ops, 11),
        Instruction::Swap12 => stack_ops::parse_swap(span_ops, 12),
        Instruction::Swap13 => stack_ops::parse_swap(span_ops, 13),
        Instruction::Swap14 => stack_ops::parse_swap(span_ops, 14),
        Instruction::Swap15 => stack_ops::parse_swap(span_ops, 15),
        Instruction::SwapW => stack_ops::parse_swapw(span_ops, 1),
        Instruction::SwapW2 => stack_ops::parse_swapw(span_ops, 2),
        Instruction::SwapW3 => stack_ops::parse_swapw(span_ops, 3),
        Instruction::SwapDW => stack_ops::parse_swapdw(span_ops),
        Instruction::MovDn2 => stack_ops::parse_movdn(span_ops, 2),
        Instruction::MovDn3 => stack_ops::parse_movdn(span_ops, 3),
        Instruction::MovDn4 => stack_ops::parse_movdn(span_ops, 4),
        Instruction::MovDn5 => stack_ops::parse_movdn(span_ops, 5),
        Instruction::MovDn6 => stack_ops::parse_movdn(span_ops, 6),
        Instruction::MovDn7 => stack_ops::parse_movdn(span_ops, 7),
        Instruction::MovDn8 => stack_ops::parse_movdn(span_ops, 8),
        Instruction::MovDn9 => stack_ops::parse_movdn(span_ops, 9),
        Instruction::MovDn10 => stack_ops::parse_movdn(span_ops, 10),
        Instruction::MovDn11 => stack_ops::parse_movdn(span_ops, 11),
        Instruction::MovDn12 => stack_ops::parse_movdn(span_ops, 12),
        Instruction::MovDn13 => stack_ops::parse_movdn(span_ops, 13),
        Instruction::MovDn14 => stack_ops::parse_movdn(span_ops, 14),
        Instruction::MovDn15 => stack_ops::parse_movdn(span_ops, 15),
        Instruction::MovUpW2 => stack_ops::parse_movupw(span_ops, 2),
        Instruction::MovUpW3 => stack_ops::parse_movupw(span_ops, 3),
        Instruction::MovUp2 => stack_ops::parse_movup(span_ops, 2),
        Instruction::MovUp3 => stack_ops::parse_movup(span_ops, 3),
        Instruction::MovUp4 => stack_ops::parse_movup(span_ops, 4),
        Instruction::MovUp5 => stack_ops::parse_movup(span_ops, 5),
        Instruction::MovUp6 => stack_ops::parse_movup(span_ops, 6),
        Instruction::MovUp7 => stack_ops::parse_movup(span_ops, 7),
        Instruction::MovUp8 => stack_ops::parse_movup(span_ops, 8),
        Instruction::MovUp9 => stack_ops::parse_movup(span_ops, 9),
        Instruction::MovUp10 => stack_ops::parse_movup(span_ops, 10),
        Instruction::MovUp11 => stack_ops::parse_movup(span_ops, 11),
        Instruction::MovUp12 => stack_ops::parse_movup(span_ops, 12),
        Instruction::MovUp13 => stack_ops::parse_movup(span_ops, 13),
        Instruction::MovUp14 => stack_ops::parse_movup(span_ops, 14),
        Instruction::MovUp15 => stack_ops::parse_movup(span_ops, 15),
        Instruction::MovDnW2 => stack_ops::parse_movdnw(span_ops, 2),
        Instruction::MovDnW3 => stack_ops::parse_movdnw(span_ops, 3),

        Instruction::CSwap => stack_ops::parse_cswap(span_ops),
        Instruction::CSwapW => stack_ops::parse_cswapw(span_ops),
        Instruction::CDrop => stack_ops::parse_cdrop(span_ops),
        Instruction::CDropW => stack_ops::parse_cdropw(span_ops),

        // ----- input / output operations --------------------------------------------------------
        Instruction::PushConstants(constants) => {
            io_ops::parse_push(instruction, span_ops, constants)
        }

        Instruction::Sdepth => io_ops::parse_sdepth(span_ops),
        Instruction::Locaddr(imm) => {
            io_ops::parse_locaddr(instruction, span_ops, num_proc_locals, *imm)
        }

        Instruction::MemLoad => io_ops::parse_mem_global(span_ops, true, true, None),
        Instruction::MemLoadImm(imm) => io_ops::parse_mem_global(span_ops, true, true, Some(*imm)),
        Instruction::LocLoad(index) => {
            io_ops::parse_mem_local(instruction, span_ops, num_proc_locals, true, true, *index)
        }

        Instruction::MemLoadW => io_ops::parse_mem_global(span_ops, false, true, None),
        Instruction::MemLoadWImm(imm) => {
            io_ops::parse_mem_global(span_ops, false, true, Some(*imm))
        }
        Instruction::LocLoadW(index) => {
            io_ops::parse_mem_local(instruction, span_ops, num_proc_locals, false, true, *index)
        }

        Instruction::MemStore => io_ops::parse_mem_global(span_ops, true, false, None),
        Instruction::MemStoreImm(imm) => {
            io_ops::parse_mem_global(span_ops, true, false, Some(*imm))
        }
        Instruction::LocStore(index) => {
            io_ops::parse_mem_local(instruction, span_ops, num_proc_locals, true, false, *index)
        }

        Instruction::MemStoreW => io_ops::parse_mem_global(span_ops, false, false, None),
        Instruction::MemStoreWImm(imm) => {
            io_ops::parse_mem_global(span_ops, false, false, Some(*imm))
        }
        Instruction::LocStoreW(index) => {
            io_ops::parse_mem_local(instruction, span_ops, num_proc_locals, false, false, *index)
        }

        Instruction::AdvPush(imm) => io_ops::parse_adv_push(instruction, span_ops, *imm),
        Instruction::AdvLoadW => io_ops::parse_adv_loadw(span_ops),

        Instruction::AdvU64Div => io_ops::parse_adv_inject_u64div(span_ops, decorators),
        Instruction::AdvKeyVal => io_ops::parse_adv_inject_keyval(span_ops, decorators),

        // ----- cryptographic operations ---------------------------------------------------------
        Instruction::RPHash => crypto_ops::parse_rphash(span_ops),
        Instruction::RPPerm => crypto_ops::parse_rpperm(span_ops),

        Instruction::MTreeGet => crypto_ops::parse_mtree_get(span_ops, decorators),
        Instruction::MTreeSet => crypto_ops::parse_mtree_set(span_ops, decorators),
        Instruction::MTreeCWM => crypto_ops::parse_mtree_cwm(span_ops, decorators),

        // ----- catch all ------------------------------------------------------------------------
        _ => return Err(AssemblyError::invalid_instruction(&instruction.to_string())),
    }?;

    if in_debug_mode {
        let op_start = decorators[dec_len].0;
        // edit the number of cycles corresponding to the asmop decorator at an index
        if let Decorator::AsmOp(assembly_op) = &mut decorators[dec_len].1 {
            assembly_op.set_num_cycles((span_ops.len() - op_start) as u8)
        }
    }

    Ok(())
}

// HELPER FUNCTIONS
// ================================================================================================

/// Parses a single parameter into a valid field element.
fn parse_element_param(op: &Token, param_idx: usize) -> Result<Felt, AssemblyError> {
    // make sure that the parameter value is available
    if op.num_parts() <= param_idx {
        return Err(AssemblyError::missing_param(op));
    }
    let param_value = op.parts()[param_idx];

    if let Some(param_value) = param_value.strip_prefix("0x") {
        // parse hexadecimal number
        parse_hex_param(op, param_idx, param_value)
    } else {
        // parse decimal number
        parse_decimal_param(op, param_idx, param_value)
    }
}

/// Parses a decimal parameter value into valid a field element.
fn parse_decimal_param(
    op: &Token,
    param_idx: usize,
    param_str: &str,
) -> Result<Felt, AssemblyError> {
    match param_str.parse::<u64>() {
        Ok(value) => get_valid_felt(op, param_idx, value),
        Err(_) => Err(AssemblyError::invalid_param(op, param_idx)),
    }
}

/// Parses a hexadecimal parameter value into a valid field element.
fn parse_hex_param(op: &Token, param_idx: usize, param_str: &str) -> Result<Felt, AssemblyError> {
    match u64::from_str_radix(param_str, 16) {
        Ok(value) => get_valid_felt(op, param_idx, value),
        Err(_) => Err(AssemblyError::invalid_param(op, param_idx)),
    }
}

/// Parses the bits length in `exp` assembly operation into usize.
fn parse_bit_len_param(op: &Token, param_idx: usize) -> Result<usize, AssemblyError> {
    let param_value = op.parts()[param_idx];

    if let Some(param) = param_value.strip_prefix('u') {
        // parse bits len param
        match param.parse::<usize>() {
            Ok(value) => Ok(value),
            Err(_) => Err(AssemblyError::invalid_param(op, param_idx)),
        }
    } else {
        Err(AssemblyError::invalid_param(op, param_idx))
    }
}

/// Checks that the u64 parameter value is a valid field element value and returns it as a field
/// element.
fn get_valid_felt(op: &Token, param_idx: usize, param: u64) -> Result<Felt, AssemblyError> {
    if param >= Felt::MODULUS {
        return Err(AssemblyError::invalid_param_with_reason(
            op,
            param_idx,
            format!("parameter value must be smaller than {}", Felt::MODULUS).as_str(),
        ));
    }

    Ok(Felt::new(param))
}

/// This is a helper function that checks the provided u32 param falls within the bounds specified by the caller.
///
/// # Errors
/// Returns an invalid param AssemblyError if:
/// - the parameter is outside the specified lower and upper bounds.
fn check_u32_param(
    instruction: &Instruction,
    param: u32,
    lower_bound: u32,
    upper_bound: u32,
) -> Result<u32, AssemblyError> {
    // check that the parameter is within the specified bounds
    if param < lower_bound || param > upper_bound {
        return Err(AssemblyError::invalid_instruction_with_reason(
            &instruction.to_string(),
            format!(
                "parameter value must be greater than or equal to {} and less than or equal to {}",
                lower_bound, upper_bound
            )
            .as_str(),
        ));
    }

    Ok(param)
}

/// This is a helper function that appends a PUSH operation to the span block which puts the
/// provided value parameter onto the stack.
///
/// When the value is 0, PUSH operation is replaced with PAD. When the value is 1, PUSH operation
/// is replaced with PAD INCR because in most cases this will be more efficient than doing a PUSH.
fn push_value(span_ops: &mut Vec<Operation>, value: Felt) {
    if value == Felt::ZERO {
        span_ops.push(Operation::Pad);
    } else if value == Felt::ONE {
        span_ops.push(Operation::Pad);
        span_ops.push(Operation::Incr);
    } else {
        span_ops.push(Operation::Push(value));
    }
}

/// Validates an op Token against a provided instruction string and/or an expected number of
/// parameter inputs and returns an appropriate AssemblyError if the operation Token is invalid.
///
/// * To fully validate an operation, pass all of the following:
/// - the parsed operation Token
/// - a string describing a valid instruction, with variants separated by '|' and parameters
///   excluded.
/// - an integer or range for the number of allowed parameters
/// This will attempt to fully validate the operation, so a full-length instruction must be
/// described. For example, `popw.mem` accepts 0 or 1 inputs and can be validated by:
/// ```validate_operation!(op_token, "popw.mem", 0..1)```
///
/// * To validate only the operation parameters, specify @only_params before passing the same inputs
/// used for full validation (above). This will skip validating each part of the instruction.
/// For example, to validate only the parameters of `popw.mem` use:
/// ```validate_operation!(@only_params op_token, "popw.mem", 0..1)```
///
/// * To validate only the instruction portion of the operation, exclude the specification for the
/// number of parameters. This will only validate up to the number of parts in the provided
/// instruction string. For example, `pop.local` and `pop.mem` are the two valid instruction
/// variants for `pop`, so the first 2 parts of `pop` can be validated by:
/// ```validate_operation!(op_token, "pop.local|mem")```
/// or the first part can be validated by:
/// ```validate_operation!(op_token, "pop")```
#[macro_export]
macro_rules! validate_operation {
    // validate that the number of parameters is within the allowed range
    (@only_params $token:expr, $instr:literal, $min_params:literal..$max_params:expr ) => {
        let num_parts = $token.num_parts();
        let num_instr_parts = $instr.split(".").count();

        // token has too few parts to contain the required parameters
        if num_parts < num_instr_parts + $min_params {
            return Err(AssemblyError::missing_param($token));
        }
        // token has more than the maximum number of parts
        if num_parts > num_instr_parts + $max_params {
            return Err(AssemblyError::extra_param($token));
        }
    };
    // validate the exact number of parameters
    (@only_params $token:expr, $instr:literal, $num_params:literal) => {
        validate_operation!(@only_params $token, $instr, $num_params..$num_params);
    };

    // validate the instruction string and an optional parameter range
    ($token:expr, $instr:literal $(, $min_params:literal..$max_params:expr)?) => {
        // split the expected instruction into a vector of parts
        let instr_parts: Vec<Vec<&str>> = $instr
            .split(".")
            .map(|part| part.split("|").collect())
            .collect();

        let num_parts = $token.num_parts();
        let num_instr_parts = instr_parts.len();

        // token has too few parts to contain the full instruction
        if num_parts < num_instr_parts {
            return Err(AssemblyError::invalid_op($token));
        }

        // compare the parts to make sure they match
        for (part_variants, token_part) in instr_parts.iter().zip($token.parts()) {
            if !part_variants.contains(token_part) {
                return Err(AssemblyError::unexpected_token($token, $instr));
            }
        }

        $(
            // validate the parameter range, if provided
            validate_operation!(@only_params $token, $instr, $min_params..$max_params);
        )?
    };
    // validate the instruction string and an exact number of parameters
    ($token:expr, $instr:literal, $num_params:literal) => {
        validate_operation!($token, $instr, $num_params..$num_params);
    };
}
