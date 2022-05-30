use super::Felt;
use core::fmt;

mod advice;
pub use advice::AdviceInjector;
mod debug;
pub use debug::DebugOptions;
#[cfg(test)]
mod tests;

// OPERATIONS
// ================================================================================================

/// TODO: add docs
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    // ----- system operations --------------------------------------------------------------------
    /// Advances cycle counter, but does not change the state of user stack.
    Noop,

    /// Pops the stack; if the popped value is not 1, execution fails.
    Assert,

    /// Pops an element off the stack, adds the current value of the `fmp` register to it, and
    /// pushes the result back onto the stack.
    FmpAdd,

    /// Pops an element off the stack and adds it to the current value of `fmp` register.
    FmpUpdate,

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

    /// Indicates the end of the program. This is used primarily to pad the execution trace to
    /// the required length. Once HALT operation is executed, no other operations can be executed
    /// by the VM (HALT operation itself excepted).
    Halt,

    // ----- field operations ---------------------------------------------------------------------
    /// Pops two elements off the stack, adds them, and pushes the result back onto the stack.
    Add,

    /// Pops an element off the stack, negates it, and pushes the result back onto the stack.
    Neg,

    /// Pops two elements off the stack, multiplies them, and pushes the result back onto the stack.
    Mul,

    /// Pops an element off the stack, computes its multiplicative inverse, and pushes the result
    /// back onto the stack.
    Inv,

    /// Pops an element off the stack, adds 1 to it, and pushes the result back onto the stack.
    Incr,

    /// Pops one element `x` off the stack, computes the power of 2^x, and pushes the result back
    /// onto the stack.
    ///
    /// If the element is greater than or equal to 64, execution fails.
    Pow2,

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

    /// Compares the first word (four elements) with the second word on the stack, if the words are
    /// equal, pushes 1 onto the stack, otherwise pushes 0 onto the stack.
    Eqw,

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

    /// Pops two elements off the stack and checks if each of them represents a 32-bit value.
    /// If both of them are, they are pushed back onto the stack, otherwise an error is returned.
    U32assert2,

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

    /// Pushes a copy of stack element 9 onto the stack.
    Dup9,

    /// Pushes a copy of stack element 11 onto the stack.
    Dup11,

    /// Pushes a copy of stack element 13 onto the stack.
    Dup13,

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

    /// Moves stack element 5 to the top of the stack.
    MovUp5,

    /// Moves stack element 6 to the top of the stack.
    MovUp6,

    /// Moves stack element 7 to the top of the stack.
    MovUp7,

    /// Moves stack element 9 to the top of the stack.
    MovUp9,

    /// Moves stack element 11 to the top of the stack.
    MovUp11,

    /// Moves stack element 13 to the top of the stack.
    MovUp13,

    /// Moves stack element 15 to the top of the stack.
    MovUp15,

    /// Moves the top stack element to position 2 on the stack.
    MovDn2,

    /// Moves the top stack element to position 3 on the stack.
    MovDn3,

    /// Moves the top stack element to position 4 on the stack.
    MovDn4,

    /// Moves the top stack element to position 5 on the stack.
    MovDn5,

    /// Moves the top stack element to position 6 on the stack.
    MovDn6,

    /// Moves the top stack element to position 7 on the stack.
    MovDn7,

    /// Moves the top stack element to position 9 on the stack.
    MovDn9,

    /// Moves the top stack element to position 11 on the stack.
    MovDn11,

    /// Moves the top stack element to position 13 on the stack.
    MovDn13,

    /// Moves the top stack element to position 15 on the stack.
    MovDn15,

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
    Push(Felt),

    /// Removes the next element from the advice tape and pushes it onto the stack.
    Read,

    /// Removes a a word (4 elements) from the advice tape and overwrites the top four stack
    /// elements with it.
    ReadW,

    /// Pops an element off the stack, interprets it as a memory address, and replaces the
    /// remaining 4 elements at the top of the stack with values located at the specified address.
    LoadW,

    /// Pops an element off the stack, interprets it as a memory address, and writes the remaining
    /// 4 elements at the top of the stack into memory at the specified address.
    StoreW,

    /// Pushes the current depth of the stack onto the stack.
    SDepth,

    // ----- cryptographic operations -------------------------------------------------------------
    /// Applies Rescue Prime permutation to the top 12 elements of the stack. The rate part of the
    /// sponge is assumed to be on top of the stack, and the capacity is expected to be deepest in
    /// the stack, starting at stack[8]. For a Rescue Prime permutation of [A, B, C] where A is the
    /// capacity, the stack should look like [C, B, A, ...] from the top.
    RpPerm,

    /// Computes a root of a Merkle path for the specified node. This operation can be used to
    /// prove that the prover knows a path in the specified Merkle tree which starts with the
    /// specified node.
    ///
    /// The stack is expected to be arranged as follows (from the top):
    /// - depth of the path, 1 element.
    /// - index of the node, 1 element.
    /// - value of the node, 4 elements.
    /// - root of the tree, 4 elements.
    ///
    /// The Merkle path itself is expected to be provided by the prover non-deterministically (via
    /// advice sets). At the end of the operation, and the node values are replaced with the
    /// computed root, but everything else remains the same. Thus, if the correct Merkle path was
    /// provided, the computed root and the provided root must be the same.
    MpVerify,

    /// Computes a new root of a Merkle tree where a node at the specified position is updated to
    /// the specified value.
    ///
    /// The stack is expected to be arranged as follows (from the top):
    /// - depth of the node, 1 element
    /// - index of the node, 1 element
    /// - old value of the node, 4 element
    /// - new value of the node, 4 element
    /// - current root of the tree, 4 elements
    ///
    /// The Merkle path for the node is expected to be provided by the prover non-deterministically
    /// (via advice sets). At the end of the operation, the old node value is replaced with the
    /// old root value computed based on the provided path, the new node value is replaced by the
    /// new root value computed based on the same path. Everything else on the stack remains the
    /// same.
    ///
    /// If the boolean parameter is set to false, at the end of the operation the advice set with
    /// the specified root will be removed from the advice provider. Otherwise, the advice
    /// provider will keep track of both, the old and the new advice sets.
    MrUpdate(bool),

    // ----- decorators ---------------------------------------------------------------------------
    /// Prints out the state of the VM. This operation has no effect on the VM state, and does not
    /// advance VM clock.
    ///
    /// TODO: add debug options to specify what is to be printed out.
    Debug(DebugOptions),

    /// Injects zero or more values at the head of the advice tape as specified by the injector.
    /// This operation affects only the advice tape, but has no effect on other VM components
    /// (e.g., stack, memory), and does not advance VM clock.
    Advice(AdviceInjector),
}

impl Operation {
    pub const OP_BITS: usize = 7;

    /// Returns the opcode of this operation.
    pub fn op_code(&self) -> Option<u8> {
        match self {
            Self::Noop => Some(0),
            Self::Assert => Some(1),

            Self::FmpAdd => Some(2),
            Self::FmpUpdate => Some(3),

            Self::Push(_) => Some(4),

            Self::Eq => Some(5),
            Self::Eqz => Some(6),
            Self::Eqw => Some(7),

            Self::Add => Some(8),
            Self::Neg => Some(9),
            Self::Mul => Some(10),
            Self::Inv => Some(11),
            Self::Incr => Some(12),
            Self::Pow2 => Some(13),
            Self::And => Some(14),
            Self::Or => Some(15),
            Self::Not => Some(16),

            Self::Pad => Some(17),
            Self::Drop => Some(18),

            Self::Dup0 => Some(19),
            Self::Dup1 => Some(20),
            Self::Dup2 => Some(21),
            Self::Dup3 => Some(22),
            Self::Dup4 => Some(23),
            Self::Dup5 => Some(24),
            Self::Dup6 => Some(25),
            Self::Dup7 => Some(26),
            Self::Dup9 => Some(27),
            Self::Dup11 => Some(28),
            Self::Dup13 => Some(29),
            Self::Dup15 => Some(30),

            Self::Swap => Some(31),
            Self::SwapW => Some(32),
            Self::SwapW2 => Some(33),
            Self::SwapW3 => Some(34),

            Self::MovUp2 => Some(35),
            Self::MovUp3 => Some(36),
            Self::MovUp4 => Some(37),
            Self::MovUp5 => Some(38),
            Self::MovUp6 => Some(39),
            Self::MovUp7 => Some(40),
            Self::MovUp9 => Some(41),
            Self::MovUp11 => Some(42),
            Self::MovUp13 => Some(43),
            Self::MovUp15 => Some(44),

            Self::MovDn2 => Some(45),
            Self::MovDn3 => Some(46),
            Self::MovDn4 => Some(47),
            Self::MovDn5 => Some(48),
            Self::MovDn6 => Some(49),
            Self::MovDn7 => Some(50),
            Self::MovDn9 => Some(51),
            Self::MovDn11 => Some(52),
            Self::MovDn13 => Some(53),
            Self::MovDn15 => Some(54),

            Self::CSwap => Some(55),
            Self::CSwapW => Some(56),

            Self::U32assert2 => Some(57),
            Self::U32split => Some(58),
            Self::U32add => Some(59),
            Self::U32addc => Some(60),
            Self::U32sub => Some(61),
            Self::U32mul => Some(62),
            Self::U32madd => Some(63),
            Self::U32div => Some(64),

            Self::U32and => Some(65),
            Self::U32or => Some(66),
            Self::U32xor => Some(67),

            Self::LoadW => Some(68),
            Self::StoreW => Some(69),

            Self::Read => Some(70),
            Self::ReadW => Some(71),

            Self::SDepth => Some(72),

            Self::RpPerm => Some(73),
            Self::MpVerify => Some(74),
            Self::MrUpdate(_) => Some(75),

            Self::End => Some(76),
            Self::Join => Some(77),
            Self::Split => Some(78),
            Self::Loop => Some(79),
            Self::Repeat => Some(80),
            Self::Respan => Some(81),
            Self::Span => Some(82),
            Self::Halt => Some(83),

            Self::Debug(_) => None,
            Self::Advice(_) => None,
        }
    }

    /// Returns an immediate value carried by this operation.
    pub fn imm_value(&self) -> Option<Felt> {
        match self {
            Self::Push(imm) => Some(*imm),
            _ => None,
        }
    }

    /// Returns true if this operation is a decorator.
    ///
    /// Decorators do not advance VM clock cycle and do not affect deterministic VM state (i.e.,
    /// stack, memory), but they can change non-deterministic components (e.g., advice tape).
    ///
    /// Additionally, decorators do not have assigned op codes.
    pub fn is_decorator(&self) -> bool {
        matches!(self, Self::Debug(_) | Self::Advice(_))
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ----- system operations ------------------------------------------------------------
            Self::Noop => write!(f, "noop"),
            Self::Assert => write!(f, "assert"),

            Self::FmpAdd => write!(f, "fmpadd"),
            Self::FmpUpdate => write!(f, "fmpupdate"),

            // ----- flow control operations ------------------------------------------------------
            Self::Join => write!(f, "join"),
            Self::Split => write!(f, "split"),
            Self::Loop => write!(f, "loop"),
            Self::Repeat => write!(f, "repeat"),
            Self::Span => write!(f, "span"),
            Self::Respan => write!(f, "respan"),
            Self::End => write!(f, "end"),
            Self::Halt => write!(f, "halt"),

            // ----- field operations -------------------------------------------------------------
            Self::Add => write!(f, "add"),
            Self::Neg => write!(f, "neg"),
            Self::Mul => write!(f, "mul"),
            Self::Inv => write!(f, "inv"),
            Self::Incr => write!(f, "incr"),
            Self::Pow2 => write!(f, "pow2"),

            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Not => write!(f, "not"),

            Self::Eq => write!(f, "eq"),
            Self::Eqz => write!(f, "eqz"),
            Self::Eqw => write!(f, "eqw"),

            // ----- u32 operations ---------------------------------------------------------------
            Self::U32assert2 => write!(f, "u32assert2"),
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
            Self::Dup9 => write!(f, "dup9"),
            Self::Dup11 => write!(f, "dup11"),
            Self::Dup13 => write!(f, "dup13"),
            Self::Dup15 => write!(f, "dup15"),

            Self::Swap => write!(f, "swap"),
            Self::SwapW => write!(f, "swapw"),
            Self::SwapW2 => write!(f, "swapw2"),
            Self::SwapW3 => write!(f, "swapw3"),

            Self::MovUp2 => write!(f, "movup2"),
            Self::MovUp3 => write!(f, "movup3"),
            Self::MovUp4 => write!(f, "movup4"),
            Self::MovUp5 => write!(f, "movup5"),
            Self::MovUp6 => write!(f, "movup6"),
            Self::MovUp7 => write!(f, "movup7"),
            Self::MovUp9 => write!(f, "movup9"),
            Self::MovUp11 => write!(f, "movup11"),
            Self::MovUp13 => write!(f, "movup13"),
            Self::MovUp15 => write!(f, "movup15"),

            Self::MovDn2 => write!(f, "movdn2"),
            Self::MovDn3 => write!(f, "movdn3"),
            Self::MovDn4 => write!(f, "movdn4"),
            Self::MovDn5 => write!(f, "movdn5"),
            Self::MovDn6 => write!(f, "movdn6"),
            Self::MovDn7 => write!(f, "movdn7"),
            Self::MovDn9 => write!(f, "movdn9"),
            Self::MovDn11 => write!(f, "movdn11"),
            Self::MovDn13 => write!(f, "movdn13"),
            Self::MovDn15 => write!(f, "movdn15"),

            Self::CSwap => write!(f, "cswap"),
            Self::CSwapW => write!(f, "cswapw"),

            // ----- input / output ---------------------------------------------------------------
            Self::Push(value) => write!(f, "push({})", value),

            Self::Read => write!(f, "read"),
            Self::ReadW => write!(f, "readw"),

            Self::LoadW => write!(f, "loadw"),
            Self::StoreW => write!(f, "storew"),

            Self::SDepth => write!(f, "sdepth"),

            // ----- cryptographic operations -----------------------------------------------------
            Self::RpPerm => write!(f, "rpperm"),
            Self::MpVerify => write!(f, "mpverify"),
            Self::MrUpdate(copy) => {
                if *copy {
                    write!(f, "mrupdate(copy)")
                } else {
                    write!(f, "mrupdate(move)")
                }
            }

            // ----- decorators -------------------------------------------------------------------
            Self::Debug(options) => write!(f, "debug({})", options),
            Self::Advice(injector) => write!(f, "advice({})", injector),
        }
    }
}
