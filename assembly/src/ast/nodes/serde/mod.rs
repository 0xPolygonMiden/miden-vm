use num_enum::TryFromPrimitive;
use vm_core::utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

use super::{CodeBody, Felt, Instruction, Node, ProcedureId, RpoDigest, ToString};
use crate::MAX_PUSH_INPUTS;

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
    Not = 23,
    And = 24,
    Or = 25,
    Xor = 26,
    Eq = 27,
    EqImm = 28,
    Neq = 29,
    NeqImm = 30,
    Eqw = 31,
    Lt = 32,
    Lte = 33,
    Gt = 34,
    Gte = 35,
    IsOdd = 36,

    // ----- ext2 operations ----------------------------------------------------------------------
    Ext2Add = 37,
    Ext2Sub = 38,
    Ext2Mul = 39,
    Ext2Div = 40,
    Ext2Neg = 41,
    Ext2Inv = 42,

    // ----- u32 manipulation ---------------------------------------------------------------------
    U32Test = 43,
    U32TestW = 44,
    U32Assert = 45,
    U32AssertWithError = 46,
    U32Assert2 = 47,
    U32Assert2WithError = 48,
    U32AssertW = 49,
    U32AssertWWithError = 50,
    U32Split = 51,
    U32Cast = 52,
    U32WrappingAdd = 53,
    U32WrappingAddImm = 54,
    U32OverflowingAdd = 55,
    U32OverflowingAddImm = 56,
    U32OverflowingAdd3 = 57,
    U32WrappingAdd3 = 58,
    U32WrappingSub = 59,
    U32WrappingSubImm = 60,
    U32OverflowingSub = 61,
    U32OverflowingSubImm = 62,
    U32WrappingMul = 63,
    U32WrappingMulImm = 64,
    U32OverflowingMul = 65,
    U32OverflowingMulImm = 66,
    U32OverflowingMadd = 67,
    U32WrappingMadd = 68,
    U32Div = 69,
    U32DivImm = 70,
    U32Mod = 71,
    U32ModImm = 72,
    U32DivMod = 73,
    U32DivModImm = 74,
    U32And = 75,
    U32Or = 76,
    U32Xor = 77,
    U32Not = 78,
    U32Shr = 79,
    U32ShrImm = 80,
    U32Shl = 81,
    U32ShlImm = 82,
    U32Rotr = 83,
    U32RotrImm = 84,
    U32Rotl = 85,
    U32RotlImm = 86,
    U32Popcnt = 87,
    U32Lt = 88,
    U32Lte = 89,
    U32Gt = 90,
    U32Gte = 91,
    U32Min = 92,
    U32Max = 93,

    // ----- stack manipulation -------------------------------------------------------------------
    Drop = 94,
    DropW = 95,
    PadW = 96,
    Dup0 = 97,
    Dup1 = 98,
    Dup2 = 99,
    Dup3 = 100,
    Dup4 = 101,
    Dup5 = 102,
    Dup6 = 103,
    Dup7 = 104,
    Dup8 = 105,
    Dup9 = 106,
    Dup10 = 107,
    Dup11 = 108,
    Dup12 = 109,
    Dup13 = 110,
    Dup14 = 111,
    Dup15 = 112,
    DupW0 = 113,
    DupW1 = 114,
    DupW2 = 115,
    DupW3 = 116,
    Swap1 = 117,
    Swap2 = 118,
    Swap3 = 119,
    Swap4 = 120,
    Swap5 = 121,
    Swap6 = 122,
    Swap7 = 123,
    Swap8 = 124,
    Swap9 = 125,
    Swap10 = 126,
    Swap11 = 127,
    Swap12 = 128,
    Swap13 = 129,
    Swap14 = 130,
    Swap15 = 131,
    SwapW1 = 132,
    SwapW2 = 133,
    SwapW3 = 134,
    SwapDW = 135,
    MovUp2 = 136,
    MovUp3 = 137,
    MovUp4 = 138,
    MovUp5 = 139,
    MovUp6 = 140,
    MovUp7 = 141,
    MovUp8 = 142,
    MovUp9 = 143,
    MovUp10 = 144,
    MovUp11 = 145,
    MovUp12 = 146,
    MovUp13 = 147,
    MovUp14 = 148,
    MovUp15 = 149,
    MovUpW2 = 150,
    MovUpW3 = 151,
    MovDn2 = 152,
    MovDn3 = 153,
    MovDn4 = 154,
    MovDn5 = 155,
    MovDn6 = 156,
    MovDn7 = 157,
    MovDn8 = 158,
    MovDn9 = 159,
    MovDn10 = 160,
    MovDn11 = 161,
    MovDn12 = 162,
    MovDn13 = 163,
    MovDn14 = 164,
    MovDn15 = 165,
    MovDnW2 = 166,
    MovDnW3 = 167,
    CSwap = 168,
    CSwapW = 169,
    CDrop = 170,
    CDropW = 171,

    // ----- input / output operations ------------------------------------------------------------
    PushU8 = 172,
    PushU16 = 173,
    PushU32 = 174,
    PushFelt = 175,
    PushWord = 176,
    PushU8List = 177,
    PushU16List = 178,
    PushU32List = 179,
    PushFeltList = 180,

    Locaddr = 181,
    Sdepth = 182,
    Caller = 183,
    Clk = 184,

    MemLoad = 185,
    MemLoadImm = 186,
    MemLoadW = 187,
    MemLoadWImm = 188,
    LocLoad = 189,
    LocLoadW = 190,
    MemStore = 191,
    MemStoreImm = 192,
    LocStore = 193,
    MemStoreW = 194,
    MemStoreWImm = 195,
    LocStoreW = 196,

    MemStream = 197,
    AdvPipe = 198,

    AdvPush = 199,
    AdvLoadW = 200,

    AdvInject = 201,

    // ----- cryptographic operations -------------------------------------------------------------
    Hash = 202,
    HMerge = 203,
    HPerm = 204,
    MTreeGet = 205,
    MTreeSet = 206,
    MTreeMerge = 207,
    MTreeVerify = 208,

    // ----- STARK proof verification -------------------------------------------------------------
    FriExt2Fold4 = 209,

    // ----- exec / call --------------------------------------------------------------------------
    ExecLocal = 210,
    ExecImported = 211,
    CallLocal = 212,
    CallMastRoot = 213,
    CallImported = 214,
    SysCall = 215,
    DynExec = 216,
    DynCall = 217,
    ProcRefLocal = 218,
    ProcRefImported = 219,

    // ----- debugging ----------------------------------------------------------------------------
    Debug = 220,

    // ----- emit --------------------------------------------------------------------------------
    Emit = 221,

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
