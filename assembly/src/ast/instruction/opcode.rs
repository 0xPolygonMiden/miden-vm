use crate::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};
use alloc::string::ToString;

/// NOTE: If the number or order of variants in this enumeration changes,
/// then the version number of the serialized format must be incremented,
/// and explicit handling must be present to translate from the old values
/// to the new values. The recommended approach would be to have separate
/// enums, one for each distinct version of the format, with a set of
/// translations working upwards from the lowest supported version.
///
/// However, since serialized MASM is likely going away in favor of MAST,
/// this may be a non-issue soon anyway.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCode {
    Assert = 0,
    AssertWithError,
    AssertEq,
    AssertEqWithError,
    AssertEqw,
    AssertEqwWithError,
    Assertz,
    AssertzWithError,
    Add,
    AddImm,
    Sub,
    SubImm,
    Mul,
    MulImm,
    Div,
    DivImm,
    Neg,
    Inv,
    Incr,
    Pow2,
    Exp,
    ExpImm,
    ExpBitLength,
    ILog2,
    Not,
    And,
    Or,
    Xor,
    Eq,
    EqImm,
    Neq,
    NeqImm,
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
    U32AssertWithError,
    U32Assert2,
    U32Assert2WithError,
    U32AssertW,
    U32AssertWWithError,
    U32Split,
    U32Cast,
    U32WrappingAdd,
    U32WrappingAddImm,
    U32OverflowingAdd,
    U32OverflowingAddImm,
    U32OverflowingAdd3,
    U32WrappingAdd3,
    U32WrappingSub,
    U32WrappingSubImm,
    U32OverflowingSub,
    U32OverflowingSubImm,
    U32WrappingMul,
    U32WrappingMulImm,
    U32OverflowingMul,
    U32OverflowingMulImm,
    U32OverflowingMadd,
    U32WrappingMadd,
    U32Div,
    U32DivImm,
    U32Mod,
    U32ModImm,
    U32DivMod,
    U32DivModImm,
    U32And,
    U32Or,
    U32Xor,
    U32Not,
    U32Shr,
    U32ShrImm,
    U32Shl,
    U32ShlImm,
    U32Rotr,
    U32RotrImm,
    U32Rotl,
    U32RotlImm,
    U32Popcnt,
    U32Clz,
    U32Ctz,
    U32Clo,
    U32Cto,
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
    SwapDW,
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
    PushU8,
    PushU16,
    PushU32,
    PushFelt,
    PushWord,
    PushU8List,
    PushU16List,
    PushU32List,
    PushFeltList,

    Locaddr,
    Sdepth,
    Caller,
    Clk,

    MemLoad,
    MemLoadImm,
    MemLoadW,
    MemLoadWImm,
    LocLoad,
    LocLoadW,
    MemStore,
    MemStoreImm,
    LocStore,
    MemStoreW,
    MemStoreWImm,
    LocStoreW,

    MemStream,
    AdvPipe,

    AdvPush,
    AdvLoadW,

    AdvInject,

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
    RCombBase,

    // ----- exec / call --------------------------------------------------------------------------
    Exec,
    Call,
    SysCall,
    DynExec,
    DynCall,
    ProcRef,

    // ----- debugging ----------------------------------------------------------------------------
    Debug,

    // ----- event decorators ---------------------------------------------------------------------
    Emit,
    Trace,

    // ----- control flow -------------------------------------------------------------------------
    IfElse,
    Repeat,
    While,
    // NOTE: If any further variants are added here, make sure you update the `MAX_DISCRIMINANT` constant
}

impl OpCode {
    const MAX_DISCRIMINANT: u8 = Self::While as u8;
}

impl Serializable for OpCode {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8(*self as u8);
    }
}

impl Deserializable for OpCode {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value = source.read_u8()?;
        if value > Self::MAX_DISCRIMINANT {
            return Err(DeserializationError::InvalidValue(
                "could not read a valid opcode".to_string(),
            ));
        }

        // SAFETY: This is guaranteed safe for the following reasons:
        //
        // * OpCode is defined as repr(u8), giving it a stable representation
        // equivalent to a u8 integer value
        //
        // * We have specified the discriminants for all of the OpCode variants.
        // Specifically, we explicitly set the first variant to `0`, and each
        // subsequent variant is incremented by 1. Thus the range from the first
        // variant to the last variant is closed, and all integers in that range
        // are valid discriminant values.
        //
        // * In Rust, constructing a repr(u*) fieldless enum from an integer is
        // always valid if the integer value corresponds to a valid discriminant
        // value, which as we've outlined above, is guaranteed to be true for all
        // values <= OpCode::MAX_DISCRIMINANT
        //
        // NOTE: This safety property holds only so long as the number of variants
        // does not _decrease_. It should be noted that it will be safe, but not
        // correct, if the order of variants changes, or additional variants are
        // added, without corresponding changes to the serialization code.
        unsafe { Ok(core::mem::transmute::<u8, OpCode>(value)) }
    }
}
