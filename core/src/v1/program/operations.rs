use core::fmt;
use math::fields::f62::BaseElement;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operation {
    // ----- system operations --------------------------------------------------------------------
    /// Advances cycle counter, but does not change the state of user stack.
    Noop,

    /// Pops the stack; if the popped value is not 1, execution fails.
    Assert,

    // ----- flow control operations --------------------------------------------------------------
    /// Marks the beginning of a join block.
    Join,

    /// Marks the beginning of a split block.
    Split,

    /// Marks the beginning of a loop block.
    Loop,

    /// Marks the beginning of a span code block.
    Span,

    /// Marks the end of a program block.
    End,

    /// Indicates that body of an executing loop should be executed again.
    Repeat,

    /// Starts processing a new operation batch.
    Respan,

    // ----- field operations ---------------------------------------------------------------------
    /// Pops two elements off the stack, adds them, and pushes the result back onto the stack.
    Add,

    //// Pops an element off the stack, negates it, and pushes the result back onto the stack.
    Neg,

    /// Pops two elements off the stack, multiplies them, and pushes the result back onto the stack.
    Mul,

    /// Pops an element off the stack, computes its multiplicative inverse, and pushes the result
    /// back onto the stack.
    Inv,

    /// Pops an element off the stack, adds 1 to it, and pushes the result back onto the stack.
    Incr,

    /// Pops two elements off the stack, multiplies them, and pushes the result back onto the stack.
    ///
    /// If either of the elements is greater than 1, execution fails. This operation is equivalent
    /// to boolean AND.
    And,

    /// Pops two elements off the stack and subtracts their product from their sum.
    ///
    /// If either of the elements is greater than 1, execution fails. This operation is equivalent
    /// to boolean OR.
    Or,

    /// Pops an element off the stack and subtracts it from 1.
    ///
    /// If the element is greater than one, the execution fails. This operation is equivalent to
    /// boolean NOT.
    Not,

    /// Pops two elements off the stack and compares them. If the elements are equal, pushes 1
    /// onto the stack, otherwise pushes 0 onto the stack.
    Eq,

    /// Pops an element off the stack and compares it to 0. If the element is 0, pushes 1 onto
    /// the stack, otherwise pushes 0 onto the stack.
    Eqz,

    // ----- u32 operations -----------------------------------------------------------------------
    /// Pops an element off the stack, splits it into upper and lower 32-bit values, and pushes
    /// these values back onto the stack.
    U32split,

    /// Pops two elements off the stack, adds them, and splits the result into upper and lower
    /// 32-bit values. Then pushes these values back onto the stack.
    ///
    /// If either of these elements is greater than or equal to 2^32, the result of this
    /// operation is undefined.
    U32add,

    /// Pops three elements off the stack, adds them together, and splits the result into upper
    /// and lower 32-bit values. Then pushes the result back onto the stack.
    ///
    /// If either of the top two elements is greater than or equal to 2^32, the result of this
    /// operation is undefined. If the third element is greater than 1, execution fails.
    U32addc,

    /// Pops two elements off the stack and subtracts the first element from the second. Then,
    /// the result, together with a flag indicating whether subtraction underflowed is pushed
    /// onto the stack.
    ///
    /// If their of the values is greater than or equal to 2^32, the result of this operation is
    /// undefined.
    U32sub,

    /// Pops two elements off the stack, multiplies them, and splits the result into upper and
    /// lower 32-bit values. Then pushes these values back onto the stack.
    ///
    /// If their of the values is greater than or equal to 2^32, the result of this operation is
    /// undefined.
    U32mul,

    /// Pops two elements off the stack and multiplies them. Then pops the third element off the
    /// stack, and adds it to the result. Finally, splits the result into upper and lower 32-bit
    /// values, and pushes them onto the stack.
    ///
    /// If any of the three values is greater than or equal to 2^32, the result of this operation
    /// is undefined.
    U32madd,

    /// Pops two elements off the stack and divides the second element by the first. Then pushes
    /// the integer result of the division, together with the remainder, onto the stack.
    ///
    /// If their of the values is greater than or equal to 2^32, the result of this operation is
    /// undefined.
    U32div,

    /// Pops two elements off the stack, computes their binary AND, and pushes the result back
    /// onto the stack.
    ///
    /// If either of the elements is greater than or equal to 2^32, execution fails.
    U32and,

    /// Pops two elements off the stack, computes their binary OR, and pushes the result back onto
    /// the stack.
    ///
    /// If either fo the elements is greater than or equal to 2^32, execution fails.
    U32or,

    /// Pops two elements off the stack, computes their binary XOR, and pushes the result back
    /// onto the stack.
    ///
    /// If either of the elements is greater than or equal to 2^32, execution fails.
    U32xor,

    // ----- stack manipulation -------------------------------------------------------------------
    /// Pushes 0 onto the stack.
    Pad,

    /// Removes to element from the stack.
    Drop,

    /// Pushes a copy of stack element 0 onto the stack.
    Dup0,

    /// Pushes a copy of stack element 1 onto the stack.
    Dup1,

    /// Pushes a copy of stack element 2 onto the stack.
    Dup2,

    /// Pushes a copy of stack element 3 onto the stack.
    Dup3,

    /// Pushes a copy of stack element 4 onto the stack.
    Dup4,

    /// Pushes a copy of stack element 5 onto the stack.
    Dup5,

    /// Pushes a copy of stack element 6 onto the stack.
    Dup6,

    /// Pushes a copy of stack element 7 onto the stack.
    Dup7,

    /// Pushes a copy of stack element 8 onto the stack.
    Dup8,

    /// Pushes a copy of stack element 9 onto the stack.
    Dup9,

    /// Pushes a copy of stack element 10 onto the stack.
    Dup10,

    /// Pushes a copy of stack element 11 onto the stack.
    Dup11,

    /// Pushes a copy of stack element 12 onto the stack.
    Dup12,

    /// Pushes a copy of stack element 13 onto the stack.
    Dup13,

    /// Pushes a copy of stack element 14 onto the stack.
    Dup14,

    /// Pushes a copy of stack element 15 onto the stack.
    Dup15,

    /// Swaps stack elements 0 and 1.
    Swap,

    /// Swaps stack elements 0, 1, 2, and 3 with elements 4, 5, 6, and 7.
    SwapW,

    /// Swaps stack elements 0, 1, 2, and 3 with elements 8, 9, 10, and 11.
    SwapW2,

    /// Swaps stack elements 0, 1, 2, and 3, with elements 12, 13, 14, and 15.
    SwapW3,

    /// Moves stack element 2 to the top of the stack.
    MovUp2,

    /// Moves stack element 3 to the top of the stack.
    MovUp3,

    /// Moves stack element 4 to the top of the stack.
    MovUp4,

    /// Moves stack element 8 to the top of the stack.
    MovUp8,

    /// Moves stack element 12 to the top of the stack.
    MovUp12,

    /// Moves the top stack element to position 2 on the stack.
    MovDn2,

    /// Moves the top stack element to position 3 on the stack.
    MovDn3,

    /// Moves the top stack element to position 4 on the stack.
    MovDn4,

    /// Moves the top stack element to position 8 on the stack.
    MovDn8,

    /// Moves the top stack element to position 12 on the stack.
    MovDn12,

    /// Pops an element off the stack, and if the element is 1, swaps the top two remaining
    /// elements on the stack. If the popped element is 0, the stack remains unchanged.
    ///
    /// If the popped element is neither 0 nor 1, execution fails.
    CSwap,

    /// Pops an element off the stack, and if the element is 1, swaps the remaining elements
    /// 0, 1, 2, and 3 with elements 4, 5, 6, and 7. If the popped element is 0, the stack
    /// remains unchanged.
    ///
    /// If the popped element is neither 0 nor 1, execution fails.
    CSwapW,

    // ----- input / output -----------------------------------------------------------------------
    /// Pushes the immediate value onto the stack.
    Push(BaseElement),

    Read,  // TODO: add tape label?
    ReadW, // TODO: add tape label?

    /// Pops an element off the stack, interprets it as a memory address, and replaces the
    /// remaining 4 elements at the top of the stack with values located at the specified address.
    LoadW,

    /// Pops an element off the stack, interprets it as a memory address, and writes the remaining
    /// 4 elements at the top of the stack into memory at hte specified address.
    StoreW,

    // ----- cryptographic operations -------------------------------------------------------------
    RpHash,
    RpPerm,
}

impl Operation {
    pub const OP_BITS: usize = 7;

    /// Returns the opcode of this operation.
    pub fn op_code(&self) -> u8 {
        match self {
            Self::Noop => 0b0000_0000,
            Self::Assert => 0b0000_0001,

            Self::Push(_) => 0b0000_0010,

            Self::Eq => 0b0000_0011,
            Self::Eqz => 0b0000_0100,

            Self::Add => 0b0000_0101,
            Self::Neg => 0b0000_0110,
            Self::Mul => 0b0000_0111,
            Self::Inv => 0b0000_1000,
            Self::Incr => 0b0000_1001,
            Self::And => 0b0000_1010,
            Self::Or => 0b0000_1011,
            Self::Not => 0b0000_1100,

            Self::Pad => 0b0000_1101,
            Self::Drop => 0b0000_1110,

            Self::Dup0 => 0b0001_0000,
            Self::Dup1 => 0b0001_0001,
            Self::Dup2 => 0b0001_0010,
            Self::Dup3 => 0b0001_0011,
            Self::Dup4 => 0b0001_0100,
            Self::Dup5 => 0b0001_0101,
            Self::Dup6 => 0b0001_0110,
            Self::Dup7 => 0b0001_0111,
            Self::Dup8 => 0b0001_1000,
            Self::Dup9 => 0b0001_1001,
            Self::Dup10 => 0b0001_1010,
            Self::Dup11 => 0b0001_1011,
            Self::Dup12 => 0b0001_1100,
            Self::Dup13 => 0b0001_1101,
            Self::Dup14 => 0b0001_1110,
            Self::Dup15 => 0b0001_1111,

            Self::Swap => 0b0010_0000,
            Self::SwapW => 0b0010_0001,
            Self::SwapW2 => 0b0010_0010,
            Self::SwapW3 => 0b0010_0011,

            Self::MovUp2 => 0b0010_0001,
            Self::MovUp3 => 0b0010_0010,
            Self::MovUp4 => 0b0010_0011,
            Self::MovUp8 => 0b0010_0100,
            Self::MovUp12 => 0b0010_0101,

            Self::MovDn2 => 0b0010_0110,
            Self::MovDn3 => 0b0010_0111,
            Self::MovDn4 => 0b0010_1000,
            Self::MovDn8 => 0b0010_1001,
            Self::MovDn12 => 0b0010_1010,

            Self::CSwap => 0b0010_1010,
            Self::CSwapW => 0b0010_1010,

            Self::U32split => 0b0011_0000,
            Self::U32add => 0b0011_0001,
            Self::U32addc => 0b0011_0010,
            Self::U32sub => 0b0011_0011,
            Self::U32mul => 0b0011_0100,
            Self::U32madd => 0b0011_0101,
            Self::U32div => 0b0011_0110,

            Self::U32and => 0b0011_0111,
            Self::U32or => 0b0011_1000,
            Self::U32xor => 0b0011_1001,

            Self::LoadW => 0b0011_1010,
            Self::StoreW => 0b0011_1011,

            Self::Read => 0b0011_1100,
            Self::ReadW => 0b0011_1101,

            Self::RpHash => 0b0011_1110,
            Self::RpPerm => 0b0011_1111,

            Self::End => 0b0111_0000,
            Self::Join => 0b0111_0001,
            Self::Split => 0b0111_0010,
            Self::Loop => 0b0111_0011,
            Self::Repeat => 0b0111_0100,
            Self::Respan => 0b0111_1000,
            Self::Span => 0b0111_1111,
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
            // ----- system operations ------------------------------------------------------------
            Self::Noop => write!(f, "noop"),
            Self::Assert => write!(f, "assert"),

            // ----- flow control operations ------------------------------------------------------
            Self::Join => write!(f, "join"),
            Self::Split => write!(f, "split"),
            Self::Loop => write!(f, "loop"),
            Self::Repeat => write!(f, "repeat"),
            Self::Span => write!(f, "span"),
            Self::Respan => write!(f, "respan"),
            Self::End => write!(f, "end"),

            // ----- field operations -------------------------------------------------------------
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

            // ----- u32 operations ---------------------------------------------------------------
            Self::U32split => write!(f, "u32split"),
            Self::U32add => write!(f, "u32add"),
            Self::U32addc => write!(f, "u32addc"),
            Self::U32sub => write!(f, "u32sub"),
            Self::U32mul => write!(f, "u32mul"),
            Self::U32madd => write!(f, "u32madd"),
            Self::U32div => write!(f, "u32div"),

            Self::U32and => write!(f, "u32and"),
            Self::U32or => write!(f, "u32or"),
            Self::U32xor => write!(f, "u32xor"),

            // ----- stack manipulation -----------------------------------------------------------
            Self::Drop => write!(f, "drop"),
            Self::Pad => write!(f, "pad"),

            Self::Dup0 => write!(f, "dup0"),
            Self::Dup1 => write!(f, "dup1"),
            Self::Dup2 => write!(f, "dup2"),
            Self::Dup3 => write!(f, "dup3"),
            Self::Dup4 => write!(f, "dup4"),
            Self::Dup5 => write!(f, "dup5"),
            Self::Dup6 => write!(f, "dup6"),
            Self::Dup7 => write!(f, "dup7"),
            Self::Dup8 => write!(f, "dup8"),
            Self::Dup9 => write!(f, "dup9"),
            Self::Dup10 => write!(f, "dup10"),
            Self::Dup11 => write!(f, "dup11"),
            Self::Dup12 => write!(f, "dup12"),
            Self::Dup13 => write!(f, "dup13"),
            Self::Dup14 => write!(f, "dup14"),
            Self::Dup15 => write!(f, "dup15"),

            Self::Swap => write!(f, "swap"),
            Self::SwapW => write!(f, "swapw"),
            Self::SwapW2 => write!(f, "swapw2"),
            Self::SwapW3 => write!(f, "swapw3"),

            Self::MovUp2 => write!(f, "movup2"),
            Self::MovUp3 => write!(f, "movup3"),
            Self::MovUp4 => write!(f, "movup4"),
            Self::MovUp8 => write!(f, "movup8"),
            Self::MovUp12 => write!(f, "movup12"),

            Self::MovDn2 => write!(f, "movdn2"),
            Self::MovDn3 => write!(f, "movdn3"),
            Self::MovDn4 => write!(f, "movdn4"),
            Self::MovDn8 => write!(f, "movdn8"),
            Self::MovDn12 => write!(f, "movdn12"),

            Self::CSwap => write!(f, "cswap"),
            Self::CSwapW => write!(f, "cswapw"),

            // ----- input / output ---------------------------------------------------------------
            Self::Push(value) => write!(f, "push({})", value),

            Self::Read => write!(f, "read"),
            Self::ReadW => write!(f, "readw"),

            Self::LoadW => write!(f, "loadw"),
            Self::StoreW => write!(f, "storew"),

            // ----- cryptographic operations -----------------------------------------------------
            Self::RpHash => write!(f, "rphash"),
            Self::RpPerm => write!(f, "rpperm"),
        }
    }
}
