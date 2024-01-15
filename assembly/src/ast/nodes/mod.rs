use super::{
    AstFormatterContext, CodeBody, Felt, FormattableCodeBody, ProcedureId, RpoDigest, ToString, Vec,
};
use core::fmt;
use vm_core::DebugOptions;

mod advice;
pub use advice::AdviceInjectorNode;

mod format;
pub use format::*;

mod serde;

// TYPE ALIASES
// ================================================================================================

type ErrorCode = u32;

// NODES
// ================================================================================================

/// A node in a AST which can represent either a single instruction or a body of a control flow
/// expression.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Node {
    Instruction(Instruction),
    IfElse {
        true_case: CodeBody,
        false_case: CodeBody,
    },
    Repeat {
        times: u32,
        body: CodeBody,
    },
    While {
        body: CodeBody,
    },
}

/// An instruction of Miden assembly program, excluding control flow instruction.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Instruction {
    Assert,
    AssertWithError(ErrorCode),
    AssertEq,
    AssertEqWithError(ErrorCode),
    AssertEqw,
    AssertEqwWithError(ErrorCode),
    Assertz,
    AssertzWithError(ErrorCode),
    Add,
    AddImm(Felt),
    Sub,
    SubImm(Felt),
    Mul,
    MulImm(Felt),
    Div,
    DivImm(Felt),
    Neg,
    Inv,
    Incr,
    Pow2,
    Exp,
    ExpImm(Felt),
    ExpBitLength(u8),
    Not,
    And,
    Or,
    Xor,
    Eq,
    EqImm(Felt),
    Neq,
    NeqImm(Felt),
    Eqw,
    Lt,
    Lte,
    Gt,
    Gte,
    IsOdd,

    // ----- ext2 operations ----------------------------------------------------------------------
    Ext2Add,
    Ext2Sub,
    Ext2Mul,
    Ext2Div,
    Ext2Neg,
    Ext2Inv,

    // ----- u32 manipulation ---------------------------------------------------------------------
    U32Test,
    U32TestW,
    U32Assert,
    U32AssertWithError(ErrorCode),
    U32Assert2,
    U32Assert2WithError(ErrorCode),
    U32AssertW,
    U32AssertWWithError(ErrorCode),
    U32Split,
    U32Cast,
    U32WrappingAdd,
    U32WrappingAddImm(u32),
    U32OverflowingAdd,
    U32OverflowingAddImm(u32),
    U32OverflowingAdd3,
    U32WrappingAdd3,
    U32WrappingSub,
    U32WrappingSubImm(u32),
    U32OverflowingSub,
    U32OverflowingSubImm(u32),
    U32WrappingMul,
    U32WrappingMulImm(u32),
    U32OverflowingMul,
    U32OverflowingMulImm(u32),
    U32OverflowingMadd,
    U32WrappingMadd,
    U32Div,
    U32DivImm(u32),
    U32Mod,
    U32ModImm(u32),
    U32DivMod,
    U32DivModImm(u32),
    U32And,
    U32Or,
    U32Xor,
    U32Not,
    U32Shr,
    U32ShrImm(u8),
    U32Shl,
    U32ShlImm(u8),
    U32Rotr,
    U32RotrImm(u8),
    U32Rotl,
    U32RotlImm(u8),
    U32Popcnt,
    U32Lt,
    U32Lte,
    U32Gt,
    U32Gte,
    U32Min,
    U32Max,

    // ----- stack manipulation -------------------------------------------------------------------
    Drop,
    DropW,
    PadW,
    Dup0,
    Dup1,
    Dup2,
    Dup3,
    Dup4,
    Dup5,
    Dup6,
    Dup7,
    Dup8,
    Dup9,
    Dup10,
    Dup11,
    Dup12,
    Dup13,
    Dup14,
    Dup15,
    DupW0,
    DupW1,
    DupW2,
    DupW3,
    Swap1,
    Swap2,
    Swap3,
    Swap4,
    Swap5,
    Swap6,
    Swap7,
    Swap8,
    Swap9,
    Swap10,
    Swap11,
    Swap12,
    Swap13,
    Swap14,
    Swap15,
    SwapW1,
    SwapW2,
    SwapW3,
    SwapDw,
    MovUp2,
    MovUp3,
    MovUp4,
    MovUp5,
    MovUp6,
    MovUp7,
    MovUp8,
    MovUp9,
    MovUp10,
    MovUp11,
    MovUp12,
    MovUp13,
    MovUp14,
    MovUp15,
    MovUpW2,
    MovUpW3,
    MovDn2,
    MovDn3,
    MovDn4,
    MovDn5,
    MovDn6,
    MovDn7,
    MovDn8,
    MovDn9,
    MovDn10,
    MovDn11,
    MovDn12,
    MovDn13,
    MovDn14,
    MovDn15,
    MovDnW2,
    MovDnW3,
    CSwap,
    CSwapW,
    CDrop,
    CDropW,

    // ----- input / output operations ------------------------------------------------------------
    PushU8(u8),
    PushU16(u16),
    PushU32(u32),
    PushFelt(Felt),
    PushWord([Felt; 4]),
    PushU8List(Vec<u8>),
    PushU16List(Vec<u16>),
    PushU32List(Vec<u32>),
    PushFeltList(Vec<Felt>),
    Locaddr(u16),
    Sdepth,
    Caller,
    Clk,

    MemLoad,
    MemLoadImm(u32),
    MemLoadW,
    MemLoadWImm(u32),
    LocLoad(u16),
    LocLoadW(u16),

    MemStore,
    MemStoreImm(u32),
    LocStore(u16),
    MemStoreW,
    MemStoreWImm(u32),
    LocStoreW(u16),

    MemStream,
    AdvPipe,

    AdvPush(u8),
    AdvLoadW,

    AdvInject(AdviceInjectorNode),

    // ----- cryptographic operations -------------------------------------------------------------
    Hash,
    HMerge,
    HPerm,
    MTreeGet,
    MTreeSet,
    MTreeMerge,
    MTreeVerify,

    // ----- STARK proof verification -------------------------------------------------------------
    FriExt2Fold4,

    // ----- exec / call --------------------------------------------------------------------------
    ExecLocal(u16),
    ExecImported(ProcedureId),
    CallLocal(u16),
    CallMastRoot(RpoDigest),
    CallImported(ProcedureId),
    SysCall(ProcedureId),
    DynExec,
    DynCall,
    ProcRefLocal(u16),
    ProcRefImported(ProcedureId),

    // ----- debug decorators ---------------------------------------------------------------------
    Breakpoint,
    Debug(DebugOptions),

    // ----- emit instruction ---------------------------------------------------------------------
    Emit(u32),

    // ----- trace instruction --------------------------------------------------------------------
    Trace(u32),
}

impl Instruction {
    /// Returns true if the instruction should yield a breakpoint.
    pub const fn should_break(&self) -> bool {
        matches!(self, Self::Breakpoint)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assert => write!(f, "assert"),
            Self::AssertWithError(err_code) => write!(f, "assert.err={err_code}"),
            Self::AssertEq => write!(f, "assert_eq"),
            Self::AssertEqWithError(err_code) => write!(f, "assert_eq.err={err_code}"),
            Self::AssertEqw => write!(f, "assert_eqw"),
            Self::AssertEqwWithError(err_code) => write!(f, "assert_eqw.err={err_code}"),
            Self::Assertz => write!(f, "assertz"),
            Self::AssertzWithError(err_code) => write!(f, "assertz.err={err_code}"),
            Self::Add => write!(f, "add"),
            Self::AddImm(value) => write!(f, "add.{value}"),
            Self::Sub => write!(f, "sub"),
            Self::SubImm(value) => write!(f, "sub.{value}"),
            Self::Mul => write!(f, "mul"),
            Self::MulImm(value) => write!(f, "mul.{value}"),
            Self::Div => write!(f, "div"),
            Self::DivImm(value) => write!(f, "div.{value}"),
            Self::Neg => write!(f, "neg"),
            Self::Inv => write!(f, "inv"),
            Self::Incr => write!(f, "add.1"),
            Self::Pow2 => write!(f, "pow2"),
            Self::Exp => write!(f, "exp"),
            Self::ExpImm(value) => write!(f, "exp.{value}"),
            Self::ExpBitLength(value) => write!(f, "exp.u{value}"),
            Self::Not => write!(f, "not"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Xor => write!(f, "xor"),
            Self::Eq => write!(f, "eq"),
            Self::EqImm(value) => write!(f, "eq.{value}"),
            Self::Neq => write!(f, "neq"),
            Self::NeqImm(value) => write!(f, "neq.{value}"),
            Self::Eqw => write!(f, "eqw"),
            Self::Lt => write!(f, "lt"),
            Self::Lte => write!(f, "lte"),
            Self::Gt => write!(f, "gt"),
            Self::Gte => write!(f, "gte"),
            Self::IsOdd => write!(f, "is_odd"),

            // ----- ext2 operations --------------------------------------------------------------
            Self::Ext2Add => write!(f, "ext2add"),
            Self::Ext2Sub => write!(f, "ext2sub"),
            Self::Ext2Mul => write!(f, "ext2mul"),
            Self::Ext2Div => write!(f, "ext2div"),
            Self::Ext2Neg => write!(f, "ext2neg"),
            Self::Ext2Inv => write!(f, "ext2inv"),

            // ----- u32 manipulation ---------------------------------------------------------------
            Self::U32Test => write!(f, "u32test"),
            Self::U32TestW => write!(f, "u32testw"),
            Self::U32Assert => write!(f, "u32assert"),
            Self::U32AssertWithError(err_code) => write!(f, "u32assert.err={err_code}"),
            Self::U32Assert2 => write!(f, "u32assert2"),
            Self::U32Assert2WithError(err_code) => write!(f, "u32assert2.err={err_code}"),
            Self::U32AssertW => write!(f, "u32assertw"),
            Self::U32AssertWWithError(err_code) => write!(f, "u32assertw.err={err_code}"),
            Self::U32Split => write!(f, "u32split"),
            Self::U32Cast => write!(f, "u32cast"),
            Self::U32WrappingAdd => write!(f, "u32wrapping_add"),
            Self::U32WrappingAddImm(value) => write!(f, "u32wrapping_add.{value}"),
            Self::U32OverflowingAdd => write!(f, "u32overflowing_add"),
            Self::U32OverflowingAddImm(value) => write!(f, "u32overflowing_add.{value}"),
            Self::U32OverflowingAdd3 => write!(f, "u32overflowing_add3"),
            Self::U32WrappingAdd3 => write!(f, "u32wrapping_add3"),
            Self::U32WrappingSub => write!(f, "u32wrapping_sub"),
            Self::U32WrappingSubImm(value) => write!(f, "u32wrapping_sub.{value}"),
            Self::U32OverflowingSub => write!(f, "u32overflowing_sub"),
            Self::U32OverflowingSubImm(value) => write!(f, "u32overflowing_sub.{value}"),
            Self::U32WrappingMul => write!(f, "u32wrapping_mul"),
            Self::U32WrappingMulImm(value) => write!(f, "u32wrapping_mul.{value}"),
            Self::U32OverflowingMul => write!(f, "u32overflowing_mul"),
            Self::U32OverflowingMulImm(value) => write!(f, "u32overflowing_mul.{value}"),
            Self::U32OverflowingMadd => write!(f, "u32overflowing_madd"),
            Self::U32WrappingMadd => write!(f, "u32wrapping_madd"),
            Self::U32Div => write!(f, "u32div"),
            Self::U32DivImm(value) => write!(f, "u32div.{value}"),
            Self::U32Mod => write!(f, "u32mod"),
            Self::U32ModImm(value) => write!(f, "u32mod.{value}"),
            Self::U32DivMod => write!(f, "u32divmod"),
            Self::U32DivModImm(value) => write!(f, "u32divmod.{value}"),
            Self::U32And => write!(f, "u32and"),
            Self::U32Or => write!(f, "u32or"),
            Self::U32Xor => write!(f, "u32xor"),
            Self::U32Not => write!(f, "u32not"),
            Self::U32Shr => write!(f, "u32shr"),
            Self::U32ShrImm(value) => write!(f, "u32shr.{value}"),
            Self::U32Shl => write!(f, "u32shl"),
            Self::U32ShlImm(value) => write!(f, "u32shl.{value}"),
            Self::U32Rotr => write!(f, "u32rotr"),
            Self::U32RotrImm(value) => write!(f, "u32rotr.{value}"),
            Self::U32Rotl => write!(f, "u32rotl"),
            Self::U32RotlImm(value) => write!(f, "u32rotl.{value}"),
            Self::U32Popcnt => write!(f, "u32popcnt"),
            Self::U32Lt => write!(f, "u32lt"),
            Self::U32Lte => write!(f, "u32lte"),
            Self::U32Gt => write!(f, "u32gt"),
            Self::U32Gte => write!(f, "u32gte"),
            Self::U32Min => write!(f, "u32min"),
            Self::U32Max => write!(f, "u32max"),

            // ----- stack manipulation ---------------------------------------------------------------
            Self::Drop => write!(f, "drop"),
            Self::DropW => write!(f, "dropw"),
            Self::PadW => write!(f, "padw"),
            Self::Dup0 => write!(f, "dup.0"),
            Self::Dup1 => write!(f, "dup.1"),
            Self::Dup2 => write!(f, "dup.2"),
            Self::Dup3 => write!(f, "dup.3"),
            Self::Dup4 => write!(f, "dup.4"),
            Self::Dup5 => write!(f, "dup.5"),
            Self::Dup6 => write!(f, "dup.6"),
            Self::Dup7 => write!(f, "dup.7"),
            Self::Dup8 => write!(f, "dup.8"),
            Self::Dup9 => write!(f, "dup.9"),
            Self::Dup10 => write!(f, "dup.10"),
            Self::Dup11 => write!(f, "dup.11"),
            Self::Dup12 => write!(f, "dup.12"),
            Self::Dup13 => write!(f, "dup.13"),
            Self::Dup14 => write!(f, "dup.14"),
            Self::Dup15 => write!(f, "dup.15"),
            Self::DupW0 => write!(f, "dupw.0"),
            Self::DupW1 => write!(f, "dupw.1"),
            Self::DupW2 => write!(f, "dupw.2"),
            Self::DupW3 => write!(f, "dupw.3"),
            Self::Swap1 => write!(f, "swap.1"),
            Self::Swap2 => write!(f, "swap.2"),
            Self::Swap3 => write!(f, "swap.3"),
            Self::Swap4 => write!(f, "swap.4"),
            Self::Swap5 => write!(f, "swap.5"),
            Self::Swap6 => write!(f, "swap.6"),
            Self::Swap7 => write!(f, "swap.7"),
            Self::Swap8 => write!(f, "swap.8"),
            Self::Swap9 => write!(f, "swap.9"),
            Self::Swap10 => write!(f, "swap.10"),
            Self::Swap11 => write!(f, "swap.11"),
            Self::Swap12 => write!(f, "swap.12"),
            Self::Swap13 => write!(f, "swap.13"),
            Self::Swap14 => write!(f, "swap.14"),
            Self::Swap15 => write!(f, "swap.15"),
            Self::SwapW1 => write!(f, "swapw.1"),
            Self::SwapW2 => write!(f, "swapw.2"),
            Self::SwapW3 => write!(f, "swapw.3"),
            Self::SwapDw => write!(f, "swapdw"),
            Self::MovUp2 => write!(f, "movup.2"),
            Self::MovUp3 => write!(f, "movup.3"),
            Self::MovUp4 => write!(f, "movup.4"),
            Self::MovUp5 => write!(f, "movup.5"),
            Self::MovUp6 => write!(f, "movup.6"),
            Self::MovUp7 => write!(f, "movup.7"),
            Self::MovUp8 => write!(f, "movup.8"),
            Self::MovUp9 => write!(f, "movup.9"),
            Self::MovUp10 => write!(f, "movup.10"),
            Self::MovUp11 => write!(f, "movup.11"),
            Self::MovUp12 => write!(f, "movup.12"),
            Self::MovUp13 => write!(f, "movup.13"),
            Self::MovUp14 => write!(f, "movup.14"),
            Self::MovUp15 => write!(f, "movup.15"),
            Self::MovUpW2 => write!(f, "movupw.2"),
            Self::MovUpW3 => write!(f, "movupw.3"),
            Self::MovDn2 => write!(f, "movdn.2"),
            Self::MovDn3 => write!(f, "movdn.3"),
            Self::MovDn4 => write!(f, "movdn.4"),
            Self::MovDn5 => write!(f, "movdn.5"),
            Self::MovDn6 => write!(f, "movdn.6"),
            Self::MovDn7 => write!(f, "movdn.7"),
            Self::MovDn8 => write!(f, "movdn.8"),
            Self::MovDn9 => write!(f, "movdn.9"),
            Self::MovDn10 => write!(f, "movdn.10"),
            Self::MovDn11 => write!(f, "movdn.11"),
            Self::MovDn12 => write!(f, "movdn.12"),
            Self::MovDn13 => write!(f, "movdn.13"),
            Self::MovDn14 => write!(f, "movdn.14"),
            Self::MovDn15 => write!(f, "movdn.15"),
            Self::MovDnW2 => write!(f, "movdnw.2"),
            Self::MovDnW3 => write!(f, "movdnw.3"),
            Self::CSwap => write!(f, "cswap"),
            Self::CSwapW => write!(f, "cswapw"),
            Self::CDrop => write!(f, "cdrop"),
            Self::CDropW => write!(f, "cdropw"),

            // ----- input / output operations ----------------------------------------------------
            Self::PushU8(value) => write!(f, "push.{value}"),
            Self::PushU16(value) => write!(f, "push.{value}"),
            Self::PushU32(value) => write!(f, "push.{value}"),
            Self::PushFelt(value) => write!(f, "push.{value}"),
            Self::PushWord(values) => display_push_vec(f, values),
            Self::PushU8List(values) => display_push_vec(f, values),
            Self::PushU16List(values) => display_push_vec(f, values),
            Self::PushU32List(values) => display_push_vec(f, values),
            Self::PushFeltList(values) => display_push_vec(f, values),

            Self::Locaddr(value) => write!(f, "locaddr.{value}"),
            Self::Sdepth => write!(f, "sdepth"),
            Self::Caller => write!(f, "caller"),
            Self::Clk => write!(f, "clk"),

            Self::MemLoad => write!(f, "mem_load"),
            Self::MemLoadImm(value) => write!(f, "mem_load.{value}"),
            Self::MemLoadW => write!(f, "mem_loadw"),
            Self::MemLoadWImm(value) => write!(f, "mem_loadw.{value}"),
            Self::LocLoad(value) => write!(f, "loc_load.{value}"),
            Self::LocLoadW(value) => write!(f, "loc_loadw.{value}"),

            Self::MemStore => write!(f, "mem_store"),
            Self::MemStoreImm(value) => write!(f, "mem_store.{value}"),
            Self::LocStore(value) => write!(f, "loc_store.{value}"),
            Self::MemStoreW => write!(f, "mem_storew"),
            Self::MemStoreWImm(value) => write!(f, "mem_storew.{value}"),
            Self::LocStoreW(value) => write!(f, "loc_storew.{value}"),

            Self::MemStream => write!(f, "mem_stream"),
            Self::AdvPipe => write!(f, "adv_pipe"),

            Self::AdvPush(value) => write!(f, "adv_push.{value}"),
            Self::AdvLoadW => write!(f, "adv_loadw"),

            Self::AdvInject(injector) => write!(f, "adv.{injector}"),

            // ----- cryptographic operations -----------------------------------------------------
            Self::Hash => write!(f, "hash"),
            Self::HMerge => write!(f, "hmerge"),
            Self::HPerm => write!(f, "hperm"),
            Self::MTreeGet => write!(f, "mtree_get"),
            Self::MTreeSet => write!(f, "mtree_set"),
            Self::MTreeMerge => write!(f, "mtree_merge"),
            Self::MTreeVerify => write!(f, "mtree_verify"),
            Self::FriExt2Fold4 => write!(f, "fri_ext2fold4"),

            // ----- exec / call ------------------------------------------------------------------
            Self::ExecLocal(index) => write!(f, "exec.{index}"),
            Self::ExecImported(proc_id) => write!(f, "exec.{proc_id}"),
            Self::CallLocal(index) => write!(f, "call.{index}"),
            Self::CallMastRoot(root) => {
                write!(f, "call.")?;
                display_hex_bytes(f, &root.as_bytes())
            }
            Self::CallImported(proc_id) => write!(f, "call.{proc_id}"),
            Self::SysCall(proc_id) => write!(f, "syscall.{proc_id}"),
            Self::DynExec => write!(f, "dynexec"),
            Self::DynCall => write!(f, "dyncall"),
            Self::ProcRefLocal(index) => write!(f, "procref.{index}"),
            Self::ProcRefImported(proc_id) => write!(f, "procref.{proc_id}"),

            // ----- debug decorators -------------------------------------------------------------
            Self::Breakpoint => write!(f, "breakpoint"),
            Self::Debug(options) => write!(f, "debug.{options}"),

            // ----- emit instruction -------------------------------------------------------------
            Self::Emit(value) => write!(f, "emit.{value}"),

            // ----- trace instruction ------------------------------------------------------------
            Self::Trace(value) => write!(f, "trace.{value}"),
        }
    }
}

// TESTS
// ================================================================================================

#[test]
fn test_instruction_display() {
    let instruction = format!("{}", Instruction::Assert);
    assert_eq!("assert", instruction);

    let instruction = format!("{}", Instruction::Add);
    assert_eq!("add", instruction);

    let instruction = format!("{}", Instruction::AddImm(Felt::new(5)));
    assert_eq!("add.5", instruction);

    let instruction = format!("{}", Instruction::ExpBitLength(32));
    assert_eq!("exp.u32", instruction);

    let instruction = format!(
        "{}",
        Instruction::PushFeltList(vec![Felt::new(3), Felt::new(4), Felt::new(8), Felt::new(9)])
    );
    assert_eq!("push.3.4.8.9", instruction);

    let hash = [7; 20];
    let proc_id = ProcedureId::from(hash);
    let instruction = format!("{}", Instruction::ExecImported(proc_id));
    assert_eq!("exec.0x0707070707070707070707070707070707070707", instruction);
}
