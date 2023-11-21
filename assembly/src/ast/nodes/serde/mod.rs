use super::{CodeBody, Felt, Instruction, Node, ProcedureId, RpoDigest, ToString};
use crate::MAX_PUSH_INPUTS;
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
    U32AssertLt = 51,
    U32AssertLtImm = 52,
    U32Split = 53,
    U32Cast = 54,
    U32WrappingAdd = 55,
    U32WrappingAddImm = 56,
    U32OverflowingAdd = 57,
    U32OverflowingAddImm = 58,
    U32OverflowingAdd3 = 59,
    U32WrappingAdd3 = 60,
    U32WrappingSub = 61,
    U32WrappingSubImm = 62,
    U32OverflowingSub = 63,
    U32OverflowingSubImm = 64,
    U32WrappingMul = 65,
    U32WrappingMulImm = 66,
    U32OverflowingMul = 67,
    U32OverflowingMulImm = 68,
    U32OverflowingMadd = 69,
    U32WrappingMadd = 70,
    U32Div = 71,
    U32DivImm = 72,
    U32Mod = 73,
    U32ModImm = 74,
    U32DivMod = 75,
    U32DivModImm = 76,
    U32And = 77,
    U32Or = 78,
    U32Xor = 79,
    U32Not = 80,
    U32Shr = 81,
    U32ShrImm = 82,
    U32Shl = 83,
    U32ShlImm = 84,
    U32Rotr = 85,
    U32RotrImm = 86,
    U32Rotl = 87,
    U32RotlImm = 88,
    U32Popcnt = 89,
    U32Lt = 90,
    U32Lte = 91,
    U32Gt = 92,
    U32Gte = 93,
    U32Min = 94,
    U32Max = 95,

    // ----- stack manipulation -------------------------------------------------------------------
    Drop = 96,
    DropW = 97,
    PadW = 98,
    Dup0 = 99,
    Dup1 = 100,
    Dup2 = 101,
    Dup3 = 102,
    Dup4 = 103,
    Dup5 = 104,
    Dup6 = 105,
    Dup7 = 106,
    Dup8 = 107,
    Dup9 = 108,
    Dup10 = 109,
    Dup11 = 110,
    Dup12 = 111,
    Dup13 = 112,
    Dup14 = 113,
    Dup15 = 114,
    DupW0 = 115,
    DupW1 = 116,
    DupW2 = 117,
    DupW3 = 118,
    Swap1 = 119,
    Swap2 = 120,
    Swap3 = 121,
    Swap4 = 122,
    Swap5 = 123,
    Swap6 = 124,
    Swap7 = 125,
    Swap8 = 126,
    Swap9 = 127,
    Swap10 = 128,
    Swap11 = 129,
    Swap12 = 130,
    Swap13 = 131,
    Swap14 = 132,
    Swap15 = 133,
    SwapW1 = 134,
    SwapW2 = 135,
    SwapW3 = 136,
    SwapDW = 137,
    MovUp2 = 138,
    MovUp3 = 139,
    MovUp4 = 140,
    MovUp5 = 141,
    MovUp6 = 142,
    MovUp7 = 143,
    MovUp8 = 144,
    MovUp9 = 145,
    MovUp10 = 146,
    MovUp11 = 147,
    MovUp12 = 148,
    MovUp13 = 149,
    MovUp14 = 150,
    MovUp15 = 151,
    MovUpW2 = 152,
    MovUpW3 = 153,
    MovDn2 = 154,
    MovDn3 = 155,
    MovDn4 = 156,
    MovDn5 = 157,
    MovDn6 = 158,
    MovDn7 = 159,
    MovDn8 = 160,
    MovDn9 = 161,
    MovDn10 = 162,
    MovDn11 = 163,
    MovDn12 = 164,
    MovDn13 = 165,
    MovDn14 = 166,
    MovDn15 = 167,
    MovDnW2 = 168,
    MovDnW3 = 169,
    CSwap = 170,
    CSwapW = 171,
    CDrop = 172,
    CDropW = 173,

    // ----- input / output operations ------------------------------------------------------------
    PushU8 = 174,
    PushU16 = 175,
    PushU32 = 176,
    PushFelt = 177,
    PushWord = 178,
    PushU8List = 179,
    PushU16List = 180,
    PushU32List = 181,
    PushFeltList = 182,

    Locaddr = 183,
    Sdepth = 184,
    Caller = 185,
    Clk = 186,

    MemLoad = 187,
    MemLoadImm = 188,
    MemLoadW = 189,
    MemLoadWImm = 190,
    LocLoad = 191,
    LocLoadW = 192,
    MemStore = 193,
    MemStoreImm = 194,
    LocStore = 195,
    MemStoreW = 196,
    MemStoreWImm = 197,
    LocStoreW = 198,

    MemStream = 199,
    AdvPipe = 200,

    AdvPush = 201,
    AdvLoadW = 202,

    AdvInject = 203,

    // ----- cryptographic operations -------------------------------------------------------------
    Hash = 204,
    HMerge = 205,
    HPerm = 206,
    MTreeGet = 207,
    MTreeSet = 208,
    MTreeMerge = 209,
    MTreeVerify = 210,

    // ----- STARK proof verification -------------------------------------------------------------
    FriExt2Fold4 = 211,

    // ----- exec / call --------------------------------------------------------------------------
    ExecLocal = 212,
    ExecImported = 213,
    CallLocal = 214,
    CallMastRoot = 215,
    CallImported = 216,
    SysCall = 217,
    DynExec = 218,
    DynCall = 219,
    ProcRefLocal = 220,
    ProcRefImported = 221,

    // ----- debugging ----------------------------------------------------------------------------
    Debug = 222,

    // ----- emit --------------------------------------------------------------------------------
    Emit = 223,

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
