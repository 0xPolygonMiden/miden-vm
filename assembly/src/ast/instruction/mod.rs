pub mod advice;
pub mod debug;
mod print;

use alloc::vec::Vec;

pub use self::{advice::SystemEventNode, debug::DebugOptions};
use crate::{
    ast::{immediate::*, InvocationTarget},
    Felt, Word,
};

// INSTRUCTION
// ================================================================================================

/// Represents the set of primitive instructions in Miden Assembly syntax.
///
/// NOTE: For control flow instructions, see [crate::ast::Op].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Instruction {
    Nop,
    Assert,
    AssertWithError(ErrorCode),
    AssertEq,
    AssertEqWithError(ErrorCode),
    AssertEqw,
    AssertEqwWithError(ErrorCode),
    Assertz,
    AssertzWithError(ErrorCode),
    Add,
    AddImm(ImmFelt),
    Sub,
    SubImm(ImmFelt),
    Mul,
    MulImm(ImmFelt),
    Div,
    DivImm(ImmFelt),
    Neg,
    ILog2,
    Inv,
    Incr,
    Pow2,
    Exp,
    ExpImm(ImmFelt),
    ExpBitLength(u8),
    Not,
    And,
    Or,
    Xor,
    Eq,
    EqImm(ImmFelt),
    Neq,
    NeqImm(ImmFelt),
    Eqw,
    Lt,
    Lte,
    Gt,
    Gte,
    IsOdd,

    // ----- ext2 operations ---------------------------------------------------------------------
    Ext2Add,
    Ext2Sub,
    Ext2Mul,
    Ext2Div,
    Ext2Neg,
    Ext2Inv,

    // ----- u32 manipulation --------------------------------------------------------------------
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
    U32WrappingAddImm(ImmU32),
    U32OverflowingAdd,
    U32OverflowingAddImm(ImmU32),
    U32OverflowingAdd3,
    U32WrappingAdd3,
    U32WrappingSub,
    U32WrappingSubImm(ImmU32),
    U32OverflowingSub,
    U32OverflowingSubImm(ImmU32),
    U32WrappingMul,
    U32WrappingMulImm(ImmU32),
    U32OverflowingMul,
    U32OverflowingMulImm(ImmU32),
    U32OverflowingMadd,
    U32WrappingMadd,
    U32Div,
    U32DivImm(ImmU32),
    U32Mod,
    U32ModImm(ImmU32),
    U32DivMod,
    U32DivModImm(ImmU32),
    U32And,
    U32Or,
    U32Xor,
    U32Not,
    U32Shr,
    U32ShrImm(ImmU8),
    U32Shl,
    U32ShlImm(ImmU8),
    U32Rotr,
    U32RotrImm(ImmU8),
    U32Rotl,
    U32RotlImm(ImmU8),
    U32Popcnt,
    U32Ctz,
    U32Clz,
    U32Clo,
    U32Cto,
    U32Lt,
    U32Lte,
    U32Gt,
    U32Gte,
    U32Min,
    U32Max,

    // ----- stack manipulation ------------------------------------------------------------------
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

    // ----- input / output operations -----------------------------------------------------------
    Push(ImmFelt),
    PushU8(u8),
    PushU16(u16),
    PushU32(u32),
    PushFelt(Felt),
    PushWord(Word),
    PushU8List(Vec<u8>),
    PushU16List(Vec<u16>),
    PushU32List(Vec<u32>),
    PushFeltList(Vec<Felt>),
    Locaddr(ImmU16),
    Sdepth,
    Caller,
    Clk,

    MemLoad,
    MemLoadImm(ImmU32),
    MemLoadW,
    MemLoadWImm(ImmU32),
    LocLoad(ImmU16),
    LocLoadW(ImmU16),

    MemStore,
    MemStoreImm(ImmU32),
    LocStore(ImmU16),
    MemStoreW,
    MemStoreWImm(ImmU32),
    LocStoreW(ImmU16),

    MemStream,
    AdvPipe,

    AdvPush(ImmU8),
    AdvLoadW,

    SysEvent(SystemEventNode),

    // ----- cryptographic operations ------------------------------------------------------------
    Hash,
    HMerge,
    HPerm,
    MTreeGet,
    MTreeSet,
    MTreeMerge,
    MTreeVerify,
    MTreeVerifyWithError(ErrorCode),

    // ----- STARK proof verification ------------------------------------------------------------
    FriExt2Fold4,
    HornerBase,
    HornerExt,

    // ----- exec / call -------------------------------------------------------------------------
    Exec(InvocationTarget),
    Call(InvocationTarget),
    SysCall(InvocationTarget),
    DynExec,
    DynCall,
    ProcRef(InvocationTarget),

    // ----- debug decorators --------------------------------------------------------------------
    Breakpoint,
    Debug(DebugOptions),

    // ----- event decorators --------------------------------------------------------------------
    Emit(ImmU32),
    Trace(ImmU32),
}

impl Instruction {
    /// Returns true if the instruction should yield a breakpoint.
    pub const fn should_break(&self) -> bool {
        matches!(self, Self::Breakpoint)
    }
}

impl core::fmt::Display for Instruction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::prettier::PrettyPrint;

        self.pretty_print(f)
    }
}
