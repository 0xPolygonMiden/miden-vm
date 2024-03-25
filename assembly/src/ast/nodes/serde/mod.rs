use super::{CodeBody, Felt, Instruction, Node, ProcedureId, RpoDigest};
use crate::MAX_PUSH_INPUTS;
use alloc::string::ToString;
use num_enum::TryFromPrimitive;
use vm_core::utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

mod debug;
mod deserialization;
mod serialization;
pub mod signatures;

// OPERATION CODES ENUM
// ================================================================================================

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum OpCode {
    Assert = 0,
    AssertWithError = 1,
    AssertEq = 2,
    AssertEqWithError = 3,
    AssertEqw = 4,
    AssertEqwWithError = 5,
    Assertz = 6,
    AssertzWithError = 7,
    Add = 8,
    AddImm = 9,
    Sub = 10,
    SubImm = 11,
    Mul = 12,
    MulImm = 13,
    Div = 14,
    DivImm = 15,
    Neg = 16,
    Inv = 17,
    Incr = 18,
    Pow2 = 19,
    Exp = 20,
    ExpImm = 21,
    ExpBitLength = 22,
    ILog2 = 23,
    Not = 24,
    And = 25,
    Or = 26,
    Xor = 27,
    Eq = 28,
    EqImm = 29,
    Neq = 30,
    NeqImm = 31,
    Eqw = 32,
    Lt = 33,
    Lte = 34,
    Gt = 35,
    Gte = 36,
    IsOdd = 37,

    // ----- ext2 operations ----------------------------------------------------------------------
    Ext2Add = 38,
    Ext2Sub = 39,
    Ext2Mul = 40,
    Ext2Div = 41,
    Ext2Neg = 42,
    Ext2Inv = 43,

    // ----- u32 manipulation ---------------------------------------------------------------------
    U32Test = 44,
    U32TestW = 45,
    U32Assert = 46,
    U32AssertWithError = 47,
    U32Assert2 = 48,
    U32Assert2WithError = 49,
    U32AssertW = 50,
    U32AssertWWithError = 51,
    U32Split = 52,
    U32Cast = 53,
    U32WrappingAdd = 54,
    U32WrappingAddImm = 55,
    U32OverflowingAdd = 56,
    U32OverflowingAddImm = 57,
    U32OverflowingAdd3 = 58,
    U32WrappingAdd3 = 59,
    U32WrappingSub = 60,
    U32WrappingSubImm = 61,
    U32OverflowingSub = 62,
    U32OverflowingSubImm = 63,
    U32WrappingMul = 64,
    U32WrappingMulImm = 65,
    U32OverflowingMul = 66,
    U32OverflowingMulImm = 67,
    U32OverflowingMadd = 68,
    U32WrappingMadd = 69,
    U32Div = 70,
    U32DivImm = 71,
    U32Mod = 72,
    U32ModImm = 73,
    U32DivMod = 74,
    U32DivModImm = 75,
    U32And = 76,
    U32Or = 77,
    U32Xor = 78,
    U32Not = 79,
    U32Shr = 80,
    U32ShrImm = 81,
    U32Shl = 82,
    U32ShlImm = 83,
    U32Rotr = 84,
    U32RotrImm = 85,
    U32Rotl = 86,
    U32RotlImm = 87,
    U32Popcnt = 88,
    U32Clz = 89,
    U32Ctz = 90,
    U32Clo = 91,
    U32Cto = 92,
    U32Lt = 93,
    U32Lte = 94,
    U32Gt = 95,
    U32Gte = 96,
    U32Min = 97,
    U32Max = 98,

    // ----- stack manipulation -------------------------------------------------------------------
    Drop = 99,
    DropW = 100,
    PadW = 101,
    Dup0 = 102,
    Dup1 = 103,
    Dup2 = 104,
    Dup3 = 105,
    Dup4 = 106,
    Dup5 = 107,
    Dup6 = 108,
    Dup7 = 109,
    Dup8 = 110,
    Dup9 = 111,
    Dup10 = 112,
    Dup11 = 113,
    Dup12 = 114,
    Dup13 = 115,
    Dup14 = 116,
    Dup15 = 117,
    DupW0 = 118,
    DupW1 = 119,
    DupW2 = 120,
    DupW3 = 121,
    Swap1 = 122,
    Swap2 = 123,
    Swap3 = 124,
    Swap4 = 125,
    Swap5 = 126,
    Swap6 = 127,
    Swap7 = 128,
    Swap8 = 129,
    Swap9 = 130,
    Swap10 = 131,
    Swap11 = 132,
    Swap12 = 133,
    Swap13 = 134,
    Swap14 = 135,
    Swap15 = 136,
    SwapW1 = 137,
    SwapW2 = 138,
    SwapW3 = 139,
    SwapDW = 140,
    MovUp2 = 141,
    MovUp3 = 142,
    MovUp4 = 143,
    MovUp5 = 144,
    MovUp6 = 145,
    MovUp7 = 146,
    MovUp8 = 147,
    MovUp9 = 148,
    MovUp10 = 149,
    MovUp11 = 150,
    MovUp12 = 151,
    MovUp13 = 152,
    MovUp14 = 153,
    MovUp15 = 154,
    MovUpW2 = 155,
    MovUpW3 = 156,
    MovDn2 = 157,
    MovDn3 = 158,
    MovDn4 = 159,
    MovDn5 = 160,
    MovDn6 = 161,
    MovDn7 = 162,
    MovDn8 = 163,
    MovDn9 = 164,
    MovDn10 = 165,
    MovDn11 = 166,
    MovDn12 = 167,
    MovDn13 = 168,
    MovDn14 = 169,
    MovDn15 = 170,
    MovDnW2 = 171,
    MovDnW3 = 172,
    CSwap = 173,
    CSwapW = 174,
    CDrop = 175,
    CDropW = 176,

    // ----- input / output operations ------------------------------------------------------------
    PushU8 = 177,
    PushU16 = 178,
    PushU32 = 179,
    PushFelt = 180,
    PushWord = 181,
    PushU8List = 182,
    PushU16List = 183,
    PushU32List = 184,
    PushFeltList = 185,

    Locaddr = 186,
    Sdepth = 187,
    Caller = 188,
    Clk = 189,

    MemLoad = 190,
    MemLoadImm = 191,
    MemLoadW = 192,
    MemLoadWImm = 193,
    LocLoad = 194,
    LocLoadW = 195,
    MemStore = 196,
    MemStoreImm = 197,
    LocStore = 198,
    MemStoreW = 199,
    MemStoreWImm = 200,
    LocStoreW = 201,

    MemStream = 202,
    AdvPipe = 203,

    AdvPush = 204,
    AdvLoadW = 205,

    AdvInject = 206,

    // ----- cryptographic operations -------------------------------------------------------------
    Hash = 207,
    HMerge = 208,
    HPerm = 209,
    MTreeGet = 210,
    MTreeSet = 211,
    MTreeMerge = 212,
    MTreeVerify = 213,

    // ----- STARK proof verification -------------------------------------------------------------
    FriExt2Fold4 = 214,
    RCombBase = 215,

    // ----- exec / call --------------------------------------------------------------------------
    ExecLocal = 216,
    ExecImported = 217,
    CallLocal = 218,
    CallMastRoot = 219,
    CallImported = 220,
    SysCall = 221,
    DynExec = 222,
    DynCall = 223,
    ProcRefLocal = 224,
    ProcRefImported = 225,

    // ----- debugging ----------------------------------------------------------------------------
    Debug = 226,

    // ----- event decorators ---------------------------------------------------------------------
    Emit = 227,
    Trace = 228,

    // ----- control flow -------------------------------------------------------------------------
    IfElse = 253,
    Repeat = 254,
    While = 255,
}

impl Serializable for OpCode {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8(*self as u8);
    }
}

impl Deserializable for OpCode {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value = source.read_u8()?;
        Self::try_from(value).map_err(|_| {
            DeserializationError::InvalidValue("could not read a valid opcode".to_string())
        })
    }
}
