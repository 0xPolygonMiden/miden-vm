use core::fmt;
use math::fields::f62::BaseElement;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operation {
    Noop,
    Push(BaseElement),
    Add,
    Neg,
    Mul,
    Inv,
}

impl Operation {
    pub const OP_BITS: usize = 7;

    /// Returns the opcode of this operation.
    pub fn op_code(&self) -> u8 {
        match self {
            Self::Noop => 0b0000_0000,
            Self::Push(_) => 0b0000_0001,
            Self::Add => 0b0000_0010,
            Self::Neg => 0b0000_0011,
            Self::Mul => 0b0000_0100,
            Self::Inv => 0b0000_0101,
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
            Self::Noop => write!(f, "noop"),
            Self::Push(value) => write!(f, "push({})", value),
            Self::Add => write!(f, "add"),
            Self::Neg => write!(f, "neg"),
            Self::Mul => write!(f, "mul"),
            Self::Inv => write!(f, "inv"),
        }
    }
}
