#![allow(clippy::unusual_byte_groupings)]

use winterfell::math::{fields::f128::BaseElement, FieldElement};

// FLOW CONTROL OPERATIONS
// ================================================================================================
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlowOps {
    Hacc = 0b000,
    Begin = 0b001,
    Tend = 0b010,
    Fend = 0b011,
    Loop = 0b100,
    Wrap = 0b101,
    Break = 0b110,
    Void = 0b111,
}

impl FlowOps {
    pub fn op_index(&self) -> usize {
        (*self as usize) & 0b111
    }
}

impl std::fmt::Display for FlowOps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FlowOps::Hacc => write!(f, "hacc"),

            FlowOps::Begin => write!(f, "begin"),
            FlowOps::Tend => write!(f, "tend"),
            FlowOps::Fend => write!(f, "fend"),

            FlowOps::Loop => write!(f, "loop"),
            FlowOps::Wrap => write!(f, "wrap"),
            FlowOps::Break => write!(f, "break"),

            FlowOps::Void => write!(f, "void"),
        }
    }
}

// USER OPERATIONS
// ================================================================================================
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UserOps {
    // low-degree operations
    Assert = 0b0_11_00000,   // left shift: 1
    AssertEq = 0b0_11_00001, // left shift: 2
    Eq = 0b0_11_00010,       // left shift: 2
    Drop = 0b0_11_00011,     // left shift: 1
    Drop4 = 0b0_11_00100,    // left shift: 4
    Choose = 0b0_11_00101,   // left shift: 2
    Choose2 = 0b0_11_00110,  // left shift: 4
    CSwap2 = 0b0_11_00111,   // left shift: 2

    Add = 0b0_11_01000, // left shift: 1
    Mul = 0b0_11_01001, // left shift: 1
    And = 0b0_11_01010, // left shift: 1
    Or = 0b0_11_01011,  // left shift: 1
    Inv = 0b0_11_01100, // no shift
    Neg = 0b0_11_01101, // no shift
    Not = 0b0_11_01110, // no shift
    //??? = 0b0_11_01111,
    Read = 0b0_11_10000,  // right shift: 1
    Read2 = 0b0_11_10001, // right shift: 2
    Dup = 0b0_11_10010,   // right shift: 1
    Dup2 = 0b0_11_10011,  // right shift: 2
    Dup4 = 0b0_11_10100,  // right shift: 4
    Pad2 = 0b0_11_10101,  // right shift: 2
    //??? = 0b0_11_10110,
    //??? = 0b0_11_10111,
    Swap = 0b0_11_11000,   // no shift
    Swap2 = 0b0_11_11001,  // no shift
    Swap4 = 0b0_11_11010,  // no shift
    Roll4 = 0b0_11_11011,  // no shift
    Roll8 = 0b0_11_11100,  // no shift
    BinAcc = 0b0_11_11101, // no shift
    //??? = 0b0_11_11110,

    // high-degree operations
    Push = 0b0_00_11111,  // right shift: 1
    Cmp = 0b0_01_11111,   // no shift
    RescR = 0b0_10_11111, // no shift

    // composite operations
    Begin = 0b0_00_00000, // no shift
    Noop = 0b0_11_11111,  // no shift
}

impl UserOps {
    pub fn ld_index(&self) -> usize {
        match self {
            UserOps::Push | UserOps::Cmp | UserOps::RescR => {
                panic!("{} is not a low-degree operation", self);
            }
            _ => (*self as usize) & 0b11111,
        }
    }

    pub fn hd_index(&self) -> usize {
        match self {
            UserOps::Push | UserOps::Cmp | UserOps::RescR | UserOps::Noop | UserOps::Begin => {
                ((*self as usize) >> 5) & 0b11
            }
            _ => {
                panic!("{} is not a high-degree operation", self);
            }
        }
    }
}

impl std::fmt::Display for UserOps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserOps::Begin => write!(f, "begin"),
            UserOps::Noop => write!(f, "noop"),

            UserOps::Assert => write!(f, "assert"),
            UserOps::AssertEq => write!(f, "asserteq"),

            UserOps::Push => write!(f, "push"),
            UserOps::Read => write!(f, "read"),
            UserOps::Read2 => write!(f, "read2"),

            UserOps::Dup => write!(f, "dup"),
            UserOps::Dup2 => write!(f, "dup2"),
            UserOps::Dup4 => write!(f, "dup4"),
            UserOps::Pad2 => write!(f, "pad2"),

            UserOps::Drop => write!(f, "drop"),
            UserOps::Drop4 => write!(f, "drop4"),

            UserOps::Swap => write!(f, "swap"),
            UserOps::Swap2 => write!(f, "swap2"),
            UserOps::Swap4 => write!(f, "swap4"),

            UserOps::Roll4 => write!(f, "roll4"),
            UserOps::Roll8 => write!(f, "roll8"),

            UserOps::Choose => write!(f, "choose"),
            UserOps::Choose2 => write!(f, "choose2"),
            UserOps::CSwap2 => write!(f, "cswap2"),

            UserOps::Add => write!(f, "add"),
            UserOps::Mul => write!(f, "mul"),
            UserOps::Inv => write!(f, "inv"),
            UserOps::Neg => write!(f, "neg"),
            UserOps::Not => write!(f, "not"),
            UserOps::And => write!(f, "and"),
            UserOps::Or => write!(f, "or"),

            UserOps::Eq => write!(f, "eq"),
            UserOps::Cmp => write!(f, "cmp"),
            UserOps::BinAcc => write!(f, "binacc"),

            UserOps::RescR => write!(f, "rescr"),
        }
    }
}

// OPERATION HINTS
// ================================================================================================
#[derive(Copy, Clone, Debug)]
pub enum OpHint {
    EqStart,
    RcStart(u32),
    CmpStart(u32),
    PmpathStart(u32),
    PushValue(BaseElement),
    None,
}

impl OpHint {
    pub fn value(&self) -> BaseElement {
        match self {
            OpHint::PushValue(value) => *value,
            _ => BaseElement::ZERO,
        }
    }
}

impl std::fmt::Display for OpHint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpHint::EqStart => write!(f, "::eq"),
            OpHint::RcStart(value) => write!(f, ".{}", value),
            OpHint::CmpStart(value) => write!(f, ".{}", value),
            OpHint::PmpathStart(value) => write!(f, ".{}", value),
            OpHint::PushValue(value) => write!(f, "({})", value),
            OpHint::None => Ok(()),
        }
    }
}
