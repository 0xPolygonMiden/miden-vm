use super::Felt;
use core::fmt;
mod decorators;
pub use decorators::{AdviceInjector, AssemblyOp, Decorator, DecoratorIterator, DecoratorList};

// OPERATIONS
// ================================================================================================

/// A set of native VM operations.
///
/// These operations take exactly one cycle to execute.
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

    /// Pushes the current depth of the stack onto the stack.
    SDepth,

    /// Overwrites the top four stack items with the hash of a function which initiated the current
    /// SYSCALL. Thus, this operation can be executed only inside a SYSCALL code block.
    Caller,

    /// Pushes the current value of the clock cycle onto the stack. This operation can be used to
    /// measure the number of cycles it has taken to execute the program up to the current instruction.
    Clk,

    // ----- flow control operations --------------------------------------------------------------
    /// Marks the beginning of a join block.
    Join,

    /// Marks the beginning of a split block.
    Split,

    /// Marks the beginning of a loop block.
    Loop,

    /// Marks the beginning of a function call.
    Call,

    /// Marks the beginning of a kernel call.
    SysCall,

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

    /// Computes a single turn of exponent accumulation for the given inputs. This operation can be
    /// be used to compute a single turn of power of a field element.
    ///
    /// The top 4 elements of the stack are expected to be arranged as follows (form the top):
    /// - least significant bit of the exponent in the previous trace if there's an expacc call,
    /// otherwise ZERO
    /// - exponent of base number `a` for this turn
    /// - accumulated power of base number `a` so far
    /// - number which needs to be shifted to the right
    ///
    /// At the end of the operation, exponent is replaced with its square, current value of power of base
    /// number `a` on exponent is incorported into the accumulator and the number is shifted to the right
    /// by one bit.
    Expacc,

    // ----- ext2 operations -----------------------------------------------------------------------
    /// Computes the product of two elements in the extension field of degree 2 and pushes the
    /// result back onto the stack as the third and fourth elemtns. Pushes 0 onto the stack as
    /// the first and second elements.
    Ext2Mul,

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
    U32add3,

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

    /// Swaps stack elements 0, 1, 2, 3, 4, 5, 6, and 7 with elements 8, 9, 10, 11, 12, 13, 14, and 15.
    SwapDW,

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

    /// Moves stack element 8 to the top of the stack.
    MovUp8,

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

    /// Moves the top stack element to position 8 on the stack.
    MovDn8,

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

    /// Removes the next element from the advice stack and pushes it onto the operand stack.
    AdvPop,

    /// Removes a word (4 elements) from the advice stack and overwrites the top four operand
    /// stack elements with it.
    AdvPopW,

    /// Pops an element off the stack, interprets it as a memory address, and replaces the
    /// remaining 4 elements at the top of the stack with values located at the specified address.
    MLoadW,

    /// Pops an element off the stack, interprets it as a memory address, and writes the remaining
    /// 4 elements at the top of the stack into memory at the specified address.
    MStoreW,

    /// Pops an element off the stack, interprets it as a memory address, and pushes the first
    /// element of the word located at the specified address to the stack.
    MLoad,

    /// Pops an element off the stack, interprets it as a memory address, and writes the remaining
    /// element at the top of the stack into the first element of the word located at the specified
    /// memory address. The remaining 3 elements of the word are not affected.
    MStore,

    /// Loads two words from memory, and replaces the top 8 elements of the stack with them,
    /// element-wise, in stack order.
    ///
    /// The operation works as follows:
    /// - The memory address of the first word is retrieved from 13th stack element (position 12).
    /// - Two consecutive words, starting at this address, are loaded from memory.
    /// - The top 8 elements of the stack are overwritten with these words (element-wise, in stack
    ///   order).
    /// - Memory address (in position 12) is incremented by 2.
    /// - All other stack elements remain the same.
    MStream,

    /// Pops two words from the advice stack, writes them to memory, and replaces the top 8 elements
    /// of the stack with them, element-wise, in stack order.
    ///
    /// The operation works as follows:
    /// - Two words are popped from the advice stack.
    /// - The destination memory address for the first word is retrieved from the 13th stack element
    ///   (position 12).
    /// - The two words are written to memory consecutively, starting at this address.
    /// - The top 8 elements of the stack are overwritten with these words (element-wise, in stack
    ///   order).
    /// - Memory address (in position 12) is incremented by 2.
    /// - All other stack elements remain the same.
    Pipe,

    // ----- cryptographic operations -------------------------------------------------------------
    /// Applies a permutation of Rescue Prime Optimized to the top 12 elements of the stack. The
    /// rate part of the sponge is assumed to be on top of the stack, and the capacity is expected
    /// to be deepest in the stack, starting at stack[8]. For an RPO permutation of [A, B, C] where
    /// A is the capacity, the stack should look like [C, B, A, ...] from the top.
    HPerm,

    /// Verifies that a Merkle path from the specified node resolves to the specified root. This
    /// operation can be used to prove that the prover knows a path in the specified Merkle tree
    /// which starts with the specified node.
    ///
    /// The stack is expected to be arranged as follows (from the top):
    /// - value of the node, 4 elements.
    /// - depth of the path, 1 element.
    /// - index of the node, 1 element.
    /// - root of the tree, 4 elements.
    ///
    /// The Merkle path itself is expected to be provided by the prover non-deterministically (via
    /// merkle sets). If the prover is not able to provide the required path, the operation fails.
    /// The state of the stack does not change.
    MpVerify,

    /// Computes a new root of a Merkle tree where a node at the specified position is updated to
    /// the specified value.
    ///
    /// The stack is expected to be arranged as follows (from the top):
    /// - old value of the node, 4 element
    /// - depth of the node, 1 element
    /// - index of the node, 1 element
    /// - current root of the tree, 4 elements
    /// - new value of the node, 4 element
    ///
    /// The Merkle path for the node is expected to be provided by the prover non-deterministically
    /// via the advice provider. At the end of the operation, the old node value is replaced with
    /// the new root value, that is computed based on the provided path. Everything else on the
    /// stack remains the same.
    ///
    /// The tree will always be copied into a new instance, meaning the advice provider will keep
    /// track of both the old and new Merkle trees.
    MrUpdate,

    /// TODO: add docs
    FriE2F4,
}

impl Operation {
    pub const OP_BITS: usize = 7;

    /// Returns the opcode of this operation.
    ///
    /// Opcode patterns have the following meanings:
    /// - 00xxxxx operations do not shift the stack; constraint degree can be up to 2.
    /// - 010xxxx operations shift the stack the left; constraint degree can be up to 2.
    /// - 011xxxx operations shift the stack to the right; constraint degree can be up to 2.
    /// - 100xxx-: operations consume 4 range checks; constraint degree can be up to 3. These are
    ///   used to encode most u32 operations.
    /// - 101xxx-: operations where constraint degree can be up to 3. These include control flow
    ///   operations and some other operations requiring high degree constraints.
    /// - 11xxx--: operations where constraint degree can be up to 5. These include control flow
    ///   operations and some other operations requiring very high degree constraints.
    #[rustfmt::skip]
    pub const fn op_code(&self) -> u8 {
        match self {
            Self::Noop      => 0b0000_0000,
            Self::Eqz       => 0b0000_0001,
            Self::Neg       => 0b0000_0010,
            Self::Inv       => 0b0000_0011,
            Self::Incr      => 0b0000_0100,
            Self::Not       => 0b0000_0101,
            Self::FmpAdd    => 0b0000_0110,
            Self::MLoad     => 0b0000_0111,
            Self::Swap      => 0b0000_1000,
            Self::Caller    => 0b0000_1001,
            Self::MovUp2    => 0b0000_1010,
            Self::MovDn2    => 0b0000_1011,
            Self::MovUp3    => 0b0000_1100,
            Self::MovDn3    => 0b0000_1101,
            Self::AdvPopW   => 0b0000_1110,
            Self::Expacc    => 0b0000_1111,

            Self::MovUp4    => 0b0001_0000,
            Self::MovDn4    => 0b0001_0001,
            Self::MovUp5    => 0b0001_0010,
            Self::MovDn5    => 0b0001_0011,
            Self::MovUp6    => 0b0001_0100,
            Self::MovDn6    => 0b0001_0101,
            Self::MovUp7    => 0b0001_0110,
            Self::MovDn7    => 0b0001_0111,
            Self::SwapW     => 0b0001_1000,
            Self::Ext2Mul   => 0b0001_1001,
            Self::MovUp8    => 0b0001_1010,
            Self::MovDn8    => 0b0001_1011,
            Self::SwapW2    => 0b0001_1100,
            Self::SwapW3    => 0b0001_1101,
            Self::SwapDW    => 0b0001_1110,
            // <empty>      => 0b0001_1111,

            Self::Assert    => 0b0010_0000,
            Self::Eq        => 0b0010_0001,
            Self::Add       => 0b0010_0010,
            Self::Mul       => 0b0010_0011,
            Self::And       => 0b0010_0100,
            Self::Or        => 0b0010_0101,
            Self::U32and    => 0b0010_0110,
            Self::U32xor    => 0b0010_0111,
            Self::FriE2F4   => 0b0010_1000,
            Self::Drop      => 0b0010_1001,
            Self::CSwap     => 0b0010_1010,
            Self::CSwapW    => 0b0010_1011,
            Self::MLoadW    => 0b0010_1100,
            Self::MStore    => 0b0010_1101,
            Self::MStoreW   => 0b0010_1110,
            Self::FmpUpdate => 0b0010_1111,

            Self::Pad       => 0b0011_0000,
            Self::Dup0      => 0b0011_0001,
            Self::Dup1      => 0b0011_0010,
            Self::Dup2      => 0b0011_0011,
            Self::Dup3      => 0b0011_0100,
            Self::Dup4      => 0b0011_0101,
            Self::Dup5      => 0b0011_0110,
            Self::Dup6      => 0b0011_0111,
            Self::Dup7      => 0b0011_1000,
            Self::Dup9      => 0b0011_1001,
            Self::Dup11     => 0b0011_1010,
            Self::Dup13     => 0b0011_1011,
            Self::Dup15     => 0b0011_1100,
            Self::AdvPop    => 0b0011_1101,
            Self::SDepth    => 0b0011_1110,
            Self::Clk       => 0b0011_1111,

            Self::U32add    => 0b0100_0000,
            Self::U32sub    => 0b0100_0010,
            Self::U32mul    => 0b0100_0100,
            Self::U32div    => 0b0100_0110,
            Self::U32split  => 0b0100_1000,
            Self::U32assert2 => 0b0100_1010,
            Self::U32add3   => 0b0100_1100,
            Self::U32madd   => 0b0100_1110,

            Self::HPerm     => 0b0101_0000,
            Self::MpVerify  => 0b0101_0010,
            Self::Pipe      => 0b0101_0100,
            Self::MStream   => 0b0101_0110,
            Self::Span      => 0b0101_1000,
            Self::Join      => 0b0101_1010,
            Self::Split     => 0b0101_1100,
            Self::Loop      => 0b0101_1110,

            Self::MrUpdate  => 0b0110_0000,
            Self::Push(_)   => 0b0110_0100,
            Self::SysCall   => 0b0110_1000,
            Self::Call      => 0b0110_1100,
            Self::End       => 0b0111_0000,
            Self::Repeat    => 0b0111_0100,
            Self::Respan    => 0b0111_1000,
            Self::Halt      => 0b0111_1100,
        }
    }

    /// Returns an immediate value carried by this operation.
    pub fn imm_value(&self) -> Option<Felt> {
        match self {
            Self::Push(imm) => Some(*imm),
            _ => None,
        }
    }

    /// Returns true if this operation is a control operation.
    pub fn is_control_op(&self) -> bool {
        matches!(
            self,
            Self::End
                | Self::Join
                | Self::Split
                | Self::Loop
                | Self::Repeat
                | Self::Respan
                | Self::Span
                | Self::Halt
                | Self::Call
                | Self::SysCall
        )
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

            Self::SDepth => write!(f, "sdepth"),
            Self::Caller => write!(f, "caller"),

            Self::Clk => write!(f, "clk"),

            // ----- flow control operations ------------------------------------------------------
            Self::Join => write!(f, "join"),
            Self::Split => write!(f, "split"),
            Self::Loop => write!(f, "loop"),
            Self::Call => writeln!(f, "call"),
            Self::SysCall => writeln!(f, "syscall"),
            Self::Span => write!(f, "span"),
            Self::End => write!(f, "end"),
            Self::Repeat => write!(f, "repeat"),
            Self::Respan => write!(f, "respan"),
            Self::Halt => write!(f, "halt"),

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

            Self::Expacc => write!(f, "expacc"),

            // ----- ext2 operations --------------------------------------------------------------
            Self::Ext2Mul => write!(f, "ext2mul"),

            // ----- u32 operations ---------------------------------------------------------------
            Self::U32assert2 => write!(f, "u32assert2"),
            Self::U32split => write!(f, "u32split"),
            Self::U32add => write!(f, "u32add"),
            Self::U32add3 => write!(f, "u32add3"),
            Self::U32sub => write!(f, "u32sub"),
            Self::U32mul => write!(f, "u32mul"),
            Self::U32madd => write!(f, "u32madd"),
            Self::U32div => write!(f, "u32div"),

            Self::U32and => write!(f, "u32and"),
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
            Self::SwapDW => write!(f, "swapdw"),

            Self::MovUp2 => write!(f, "movup2"),
            Self::MovUp3 => write!(f, "movup3"),
            Self::MovUp4 => write!(f, "movup4"),
            Self::MovUp5 => write!(f, "movup5"),
            Self::MovUp6 => write!(f, "movup6"),
            Self::MovUp7 => write!(f, "movup7"),
            Self::MovUp8 => write!(f, "movup8"),

            Self::MovDn2 => write!(f, "movdn2"),
            Self::MovDn3 => write!(f, "movdn3"),
            Self::MovDn4 => write!(f, "movdn4"),
            Self::MovDn5 => write!(f, "movdn5"),
            Self::MovDn6 => write!(f, "movdn6"),
            Self::MovDn7 => write!(f, "movdn7"),
            Self::MovDn8 => write!(f, "movdn8"),

            Self::CSwap => write!(f, "cswap"),
            Self::CSwapW => write!(f, "cswapw"),

            // ----- input / output ---------------------------------------------------------------
            Self::Push(value) => write!(f, "push({value})"),

            Self::AdvPop => write!(f, "advpop"),
            Self::AdvPopW => write!(f, "advpopw"),

            Self::MLoadW => write!(f, "mloadw"),
            Self::MStoreW => write!(f, "mstorew"),

            Self::MLoad => write!(f, "mload"),
            Self::MStore => write!(f, "mstore"),

            Self::MStream => write!(f, "mstream"),
            Self::Pipe => write!(f, "pipe"),

            // ----- cryptographic operations -----------------------------------------------------
            Self::HPerm => write!(f, "hperm"),
            Self::MpVerify => write!(f, "mpverify"),
            Self::MrUpdate => write!(f, "mrupdate"),
            Self::FriE2F4 => write!(f, "frie2f4"),
        }
    }
}
