use core::fmt;
use math::fields::f62::BaseElement;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operation {
    // ----- flow control operations --------------------------------------------------------------
    Noop,
    Join,
    Split,
    Loop,
    Repeat,
    Span,
    Respan,
    End,

    // ----- field operations ---------------------------------------------------------------------
    Assert,

    Add,
    Neg,
    Mul,
    Inv,
    Incr,
    And,
    Or,
    Not,

    Eq,
    Eqz,

    // ----- u32 operations -----------------------------------------------------------------------

    // ----- stack manipulation -------------------------------------------------------------------
    Pad,

    Dup0,
    Dup1,
    Dup2,
    Dup3,

    Swap,

    Movup2,
    Movup3,

    Movdn2,
    Movdn3,

    // ----- input / output -----------------------------------------------------------------------
    Push(BaseElement),
    // ----- cryptographic operations -------------------------------------------------------------
}

impl Operation {
    pub const OP_BITS: usize = 7;

    /// Returns the opcode of this operation.
    pub fn op_code(&self) -> u8 {
        match self {
            Self::Noop => 0b0000_0000,
            Self::Push(_) => 0b0000_0001,

            Self::Assert => 0b0000_0010,

            Self::Add => 0b0000_0011,
            Self::Neg => 0b0000_0100,
            Self::Mul => 0b0000_0101,
            Self::Inv => 0b0000_0110,
            Self::Incr => 0b0000_0110,
            Self::And => 0b0000_0111,
            Self::Or => 0b0000_1000,
            Self::Not => 0b0000_1001,

            Self::Eq => 0b0000_1010,
            Self::Eqz => 0b0000_1011,

            Self::Pad => 0b0000_1100,

            Self::Dup0 => 0b0000_1101,
            Self::Dup1 => 0b0000_1110,
            Self::Dup2 => 0b0000_1111,
            Self::Dup3 => 0b0001_0000,

            Self::Swap => 0b0001_0001,

            Self::Movup2 => 0b0001_0010,
            Self::Movup3 => 0b0001_0011,

            Self::Movdn2 => 0b0001_0100,
            Self::Movdn3 => 0b0001_0101,

            Self::Join => 0b0001_0110,
            Self::Split => 0b0001_0111,
            Self::Loop => 0b0001_1000,
            Self::Repeat => 0b0001_1001,
            Self::Span => 0b0001_1010,
            Self::Respan => 0b0001_1011,
            Self::End => 0b0001_1100,
        }
    }

    /// Returns an immediate value carried by this operation.
    pub fn imm_value(&self) -> Option<BaseElement> {
        match self {
            Self::Push(imm) => Some(*imm),
            _ => None,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ----- flow control operations ------------------------------------------------------
            Self::Noop => write!(f, "noop"),
            Self::Join => write!(f, "join"),
            Self::Split => write!(f, "split"),
            Self::Loop => write!(f, "loop"),
            Self::Repeat => write!(f, "repeat"),
            Self::Span => write!(f, "span"),
            Self::Respan => write!(f, "respan"),
            Self::End => write!(f, "end"),

            // ----- field operations -------------------------------------------------------------
            Self::Assert => write!(f, "assert"),

            Self::Add => write!(f, "add"),
            Self::Neg => write!(f, "neg"),
            Self::Mul => write!(f, "mul"),
            Self::Inv => write!(f, "inv"),
            Self::Incr => write!(f, "incr"),

            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Not => write!(f, "not"),

            Self::Eq => write!(f, "eq"),
            Self::Eqz => write!(f, "eqz"),

            // ----- stack manipulation -----------------------------------------------------------
            Self::Pad => write!(f, "pad"),

            Self::Dup0 => write!(f, "dup0"),
            Self::Dup1 => write!(f, "dup1"),
            Self::Dup2 => write!(f, "dup2"),
            Self::Dup3 => write!(f, "dup3"),

            Self::Swap => write!(f, "swap"),

            Self::Movup2 => write!(f, "movup2"),
            Self::Movup3 => write!(f, "movup3"),

            Self::Movdn2 => write!(f, "movdn2"),
            Self::Movdn3 => write!(f, "movdn3"),

            // ----- input / output ---------------------------------------------------------------
            Self::Push(value) => write!(f, "push({})", value),
        }
    }
}
