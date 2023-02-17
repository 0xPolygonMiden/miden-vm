use super::{
    ByteReader, ByteWriter, Deserializable, Instruction, Node, ProcedureId, Serializable,
    SerializationError,
};
use crate::MAX_PUSH_INPUTS;
use num_enum::TryFromPrimitive;

mod deserialization;
mod serialization;

// OPERATION CODES ENUM
// ================================================================================================

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum OpCode {
    Assert = 0,
    AssertEq = 1,
    Assertz = 2,
    Add = 3,
    AddImm = 4,
    Sub = 5,
    SubImm = 6,
    Mul = 7,
    MulImm = 8,
    Div = 9,
    DivImm = 10,
    Neg = 11,
    Inv = 12,
    Incr = 13,
    Pow2 = 14,
    Exp = 15,
    ExpImm = 16,
    ExpBitLength = 17,
    Not = 18,
    And = 19,
    Or = 20,
    Xor = 21,
    Eq = 22,
    EqImm = 23,
    Neq = 24,
    NeqImm = 25,
    Eqw = 26,
    Lt = 27,
    Lte = 28,
    Gt = 29,
    Gte = 30,

    // ----- ext2 operations ----------------------------------------------------------------------
    Ext2Add = 31,
    Ext2Sub = 32,
    Ext2Mul = 33,
    Ext2Div = 34,
    Ext2Neg = 35,
    Ext2Inv = 36,

    // ----- u32 manipulation ---------------------------------------------------------------------
    U32Test = 37,
    U32TestW = 38,
    U32Assert = 39,
    U32Assert2 = 40,
    U32AssertW = 41,
    U32Split = 42,
    U32Cast = 43,
    U32CheckedAdd = 44,
    U32CheckedAddImm = 45,
    U32WrappingAdd = 46,
    U32WrappingAddImm = 47,
    U32OverflowingAdd = 48,
    U32OverflowingAddImm = 49,
    U32OverflowingAdd3 = 50,
    U32WrappingAdd3 = 51,
    U32CheckedSub = 52,
    U32CheckedSubImm = 53,
    U32WrappingSub = 54,
    U32WrappingSubImm = 55,
    U32OverflowingSub = 56,
    U32OverflowingSubImm = 57,
    U32CheckedMul = 58,
    U32CheckedMulImm = 59,
    U32WrappingMul = 60,
    U32WrappingMulImm = 61,
    U32OverflowingMul = 62,
    U32OverflowingMulImm = 63,
    U32OverflowingMadd = 64,
    U32WrappingMadd = 65,
    U32CheckedDiv = 66,
    U32CheckedDivImm = 67,
    U32UncheckedDiv = 68,
    U32UncheckedDivImm = 69,
    U32CheckedMod = 70,
    U32CheckedModImm = 71,
    U32UncheckedMod = 72,
    U32UncheckedModImm = 73,
    U32CheckedDivMod = 74,
    U32CheckedDivModImm = 75,
    U32UncheckedDivMod = 76,
    U32UncheckedDivModImm = 77,
    U32CheckedAnd = 78,
    U32CheckedOr = 79,
    U32CheckedXor = 80,
    U32CheckedNot = 81,
    U32CheckedShr = 82,
    U32CheckedShrImm = 83,
    U32UncheckedShr = 84,
    U32UncheckedShrImm = 85,
    U32CheckedShl = 86,
    U32CheckedShlImm = 87,
    U32UncheckedShl = 88,
    U32UncheckedShlImm = 89,
    U32CheckedRotr = 90,
    U32CheckedRotrImm = 91,
    U32UncheckedRotr = 92,
    U32UncheckedRotrImm = 93,
    U32CheckedRotl = 94,
    U32CheckedRotlImm = 95,
    U32UncheckedRotl = 96,
    U32UncheckedRotlImm = 97,
    U32CheckedPopcnt = 98,
    U32UncheckedPopcnt = 99,
    U32CheckedEq = 100,
    U32CheckedEqImm = 101,
    U32CheckedNeq = 102,
    U32CheckedNeqImm = 103,
    U32CheckedLt = 104,
    U32UncheckedLt = 105,
    U32CheckedLte = 106,
    U32UncheckedLte = 107,
    U32CheckedGt = 108,
    U32UncheckedGt = 109,
    U32CheckedGte = 110,
    U32UncheckedGte = 111,
    U32CheckedMin = 112,
    U32UncheckedMin = 113,
    U32CheckedMax = 114,
    U32UncheckedMax = 115,

    // ----- stack manipulation -------------------------------------------------------------------
    Drop = 116,
    DropW = 117,
    PadW = 118,
    Dup0 = 119,
    Dup1 = 120,
    Dup2 = 121,
    Dup3 = 122,
    Dup4 = 123,
    Dup5 = 124,
    Dup6 = 125,
    Dup7 = 126,
    Dup8 = 127,
    Dup9 = 128,
    Dup10 = 129,
    Dup11 = 130,
    Dup12 = 131,
    Dup13 = 132,
    Dup14 = 133,
    Dup15 = 134,
    DupW0 = 135,
    DupW1 = 136,
    DupW2 = 137,
    DupW3 = 138,
    Swap1 = 139,
    Swap2 = 140,
    Swap3 = 141,
    Swap4 = 142,
    Swap5 = 143,
    Swap6 = 144,
    Swap7 = 145,
    Swap8 = 146,
    Swap9 = 147,
    Swap10 = 148,
    Swap11 = 149,
    Swap12 = 150,
    Swap13 = 151,
    Swap14 = 152,
    Swap15 = 153,
    SwapW1 = 154,
    SwapW2 = 155,
    SwapW3 = 156,
    SwapDW = 157,
    MovUp2 = 158,
    MovUp3 = 159,
    MovUp4 = 160,
    MovUp5 = 161,
    MovUp6 = 162,
    MovUp7 = 163,
    MovUp8 = 164,
    MovUp9 = 165,
    MovUp10 = 166,
    MovUp11 = 167,
    MovUp12 = 168,
    MovUp13 = 169,
    MovUp14 = 170,
    MovUp15 = 171,
    MovUpW2 = 172,
    MovUpW3 = 173,
    MovDn2 = 174,
    MovDn3 = 175,
    MovDn4 = 176,
    MovDn5 = 177,
    MovDn6 = 178,
    MovDn7 = 179,
    MovDn8 = 180,
    MovDn9 = 181,
    MovDn10 = 182,
    MovDn11 = 183,
    MovDn12 = 184,
    MovDn13 = 185,
    MovDn14 = 186,
    MovDn15 = 187,
    MovDnW2 = 188,
    MovDnW3 = 189,
    CSwap = 190,
    CSwapW = 191,
    CDrop = 192,
    CDropW = 193,

    // ----- input / output operations ------------------------------------------------------------
    PushU8 = 194,
    PushU16 = 195,
    PushU32 = 196,
    PushFelt = 197,
    PushWord = 198,
    PushU8List = 199,
    PushU16List = 200,
    PushU32List = 201,
    PushFeltList = 202,

    Locaddr = 203,
    Sdepth = 204,
    Caller = 205,
    Clk = 206,

    MemLoad = 207,
    MemLoadImm = 208,
    MemLoadW = 209,
    MemLoadWImm = 210,
    LocLoad = 211,
    LocLoadW = 212,
    MemStore = 213,
    MemStoreImm = 214,
    LocStore = 215,
    MemStoreW = 216,
    MemStoreWImm = 217,
    LocStoreW = 218,

    MemStream = 219,
    AdvPipe = 220,

    AdvPush = 221,
    AdvLoadW = 222,

    AdvU64Div = 223,
    AdvKeyval = 224,
    AdvMem = 225,
    AdvExt2Inv = 226,
    AdvExt2INTT = 227,

    // ----- cryptographic operations -------------------------------------------------------------
    Hash = 228,
    HMerge = 229,
    HPerm = 230,
    MTreeGet = 231,
    MTreeSet = 232,
    MTreeCwm = 233,

    // ----- exec / call --------------------------------------------------------------------------
    ExecLocal = 234,
    ExecImported = 235,
    CallLocal = 236,
    CallImported = 237,
    SysCall = 238,

    // ----- control flow -------------------------------------------------------------------------
    IfElse = 253,
    Repeat = 254,
    While = 255,
}

impl Serializable for OpCode {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        target.write_u8(*self as u8);
        Ok(())
    }
}

impl Deserializable for OpCode {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let value = bytes.read_u8()?;
        Self::try_from(value).map_err(|_| SerializationError::InvalidOpCode)
    }
}
