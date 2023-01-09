use super::{AssemblyError, CodeBlock, Operation::*, SpanBuilder};
use vm_core::{AdviceInjector::Ext2Inv, Decorator, Felt, Operation};

/// Given a stack in the following initial configuration [a1, a0, b1, b0, ...] where a = (a0, a1)
/// and b = (b0, b1) represent elements in the extension field of degree 2, this series of
/// operations outputs the result c = (c1, c0) where c1 = a1 + b1 and c0 = a0 + b0.
pub fn ext2_add(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    #[rustfmt::skip]
    let ops = [
        Swap,           // [a0,a1,b1,b0,...]
        MovUp3,         // [b0,a0,a1,b1,...]
        Add,            // [b0+a0,a1,b1,...]
        MovDn2,         // [a1,b1,b0+a0,...]
        Add             // [a1+b1,b0+a0,...]
    ];
    span.add_ops(ops)
}

/// Given a stack in the following initial configuration [a1, a0, b1, b0, ...] where a = (a0, a1)
/// and b = (b0, b1) represent elements in the extension field of degree 2, this series of
/// operations outputs the result c = (c1, c0) where c1 = a1 - b1 and c0 = a0 - b0.
pub fn ext2_sub(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    #[rustfmt::skip]
    let ops = [
        Swap,           // [a0,a1,b1,b0,...]
        
        MovUp3,         // [b0,a0,a1,b1,...]
        
        Neg,            // [-b0,a0,a1,b1,...]
        
        Add,            // [a0-b0,a1,b1,...]
        
        MovDn2,         // [a1,b1,a0-b0,...]
        
        Swap,           // [b1,a1,a0-b0,...]
        
        Neg,            // [-b1,a1,a0-b0,...]
        
        Add             // [a1-b1,a0-b0,...]
    ];
    span.add_ops(ops)
}

/// Given a stack with initial configuration given by [a1, a0, b1, b0, ...] where a = (a0, a1) and
/// b = (b0, b1) represent elements in the extension field of degree 2, this series of operations
/// outputs the product c = (c1, c0) where c0 = a0b0 - 2(a1b1) and c1 = (a0 + a1)(b0 + b1) - a0b0
pub fn ext2_mul(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    span.add_ops(ext2_mul_ops())
}

/// Given a stack in the following initial configuration [a1,a0,b1,b0,...] where a = (a0, a1) and
/// b = (b0, b1) represent elements in the extension field of degree 2, this series of operations
/// outputs the result c = (c1, c0) where c1 = a1 / b1 and c0 = a0 / b0.
pub fn ext2_div(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    span.add_decorator(Decorator::Advice(Ext2Inv))?;
    #[rustfmt::skip]
    let mut ops = vec![
        Read,
        Read,
        Dup1,
        Dup1,
        MovUp5,
        MovUp5,
    ];
    #[rustfmt::skip]
    ops.extend_from_slice(&ext2_mul_ops());
    ops.extend_from_slice(&[Eqz, Assert, Assert]);
    ops.extend_from_slice(&ext2_mul_ops());

    span.add_ops(ops)
}

/// Given a stack with initial configuration given by [a1, a0, ...] where a = (a0,a1) represents
/// elements in the extension field of degree 2, the procedure outputs the negative of a, i.e.
/// [-a1,-a0,...]
pub fn ext2_neg(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    #[rustfmt::skip]
    let ops = [
        // [a1,a0,...]
        Neg,
        // [a0,-a1,...]
        Swap,
        // [-a0,-a1,...]
        Neg,
        // [-a1,-a0,...]
        Swap
        ];
    span.add_ops(ops)
}

/// Given an invertible quadratic extension field element on the stack, this routine computes
/// multiplicative inverse of that element, using non-deterministic technique ( i.e. it takes help
/// of advice provider ). To ensure that non-deterministic computation resulted in correct value,
/// it multiplies input operand with computed output, over quadratic extension field which must
/// produce multiplicative identity (1, 0) of quadratic extension field. In case input operand is
/// additive identity which can't be inverted, program execution fails, as advice provider won't
/// calculate multiplicative inverse in that case.
///
/// Expected input stack
///
/// [a1, a0, ...] | a = (a0, a1) ∈ Quadratic extension field over F_q, q = 2^64 - 2^32 + 1
///
/// Expected output stack
///
/// [a'1, a'0, ...] | a' = (a'0, a'1) ∈ Quadratic extension field over F_q, q = 2^64 - 2^32 + 1
///
/// Following is what is checked after reading result of computation, performed outside of VM
///
/// a  = (a0, a1)
/// a' = (a'0, a'1) ( = a ^ -1 )
///
/// b  = a * a' ( mod P ) | P = irreducible polynomial x^2 - x + 2 over F_q, q = 2^64 - 2^32 + 1
/// assert b  = (1, 0) | (1, 0) is the multiplicative identity of extension field
pub fn ext2_inv(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    span.add_decorator(Decorator::Advice(Ext2Inv))?;
    #[rustfmt::skip]
    let mut ops = vec![
        Read,
        Read,
        Dup1,
        Dup1,
        MovUp5,
        MovUp5,
    ];
    #[rustfmt::skip]
    ops.extend_from_slice(&ext2_mul_ops());
    ops.extend_from_slice(&[Eqz, Assert, Assert]);
    span.add_ops(ops)
}

/// A helper function that returns the operations to compute the product of two elements in the
/// extension field of degree 2.
/// Given a stack with initial configuration given by [a1,a0,b1,b0,...] where a = (a0,a1) and
/// b = (b0,b1) represent elements in the extension field of degree 2, the procedure outputs the
/// product c = (c1,c0) where c0 = a0b0 - 2(a1b1) and c1 = (a0 + a1)(b0 + b1) - a0b0
fn ext2_mul_ops() -> Vec<Operation> {
    #[rustfmt::skip]
    let ops = vec![
        Dup3, Dup3, Dup3, Dup3,   // [a1,a0,b1,b0,a1,a0,b1,b0,...]
        MovDn2, MovUp3,           // [b0,a0,b1,a1,a1,a0,b1,b0,...]
        Mul,                      // [b0a0,a1,a1,a0,b1,b0,...]
        Dup0,                     // [b0a0,b0a0,a1,a1,a0,b1,b0,...]
        MovDn7,                   // [b0a0,b1,a1,a1,a0,b1,b0,b0a0,...]
        MovDn2,                   // [b1,a1,b0a0,a1,a0,b1,b0,b0a0,...]
        Push(Felt::new(2)),       // [2,b1,a1,b0a0,a1,a0,b1,b0,b0a0,...]
        Mul,                      // [2b1,a1,b0a0,a1,a0,b1,b0,b0a0,...]
        Mul,                      // [2b1a1,b0a0,a1,a0,b1,b0,b0a0,...]
        Neg,                      // [-2b1a1,b0a0,a1,a0,b1,b0,b0a0,...]
        Add,                      // [b0a0-2b1a1,a1,a0,b1,b0,b0a0,...]
        MovDn5,                   // [a1,a0,b1,b0,b0a0,b0a0-2b1a1,...]
        Add,                      // [a1+a0,b1,b0,b0a0,b0a0-2b1a1,...]
        Swap, MovUp2,             // [b0,b1,a1+a0,b0a0,b0a0-2b1a1,...]
        Add,                      // [b0+b1,a1+a0,b0a0,b0a0-2b1a1,...]
        Mul,                      // [(b0+b1)(a1+a0),b0a0,b0a0-2b1a1,...]
        Swap,                     // [b0a0,(b0+b1)(a1+a0),b0a0-2b1a1,...]
        Neg,                      // [-b0a0,(b0+b1)(a1+a0),b0a0-2b1a1,...]
        Add,                      // [(b0+b1)(a1+a0)-b0a0,b0a0-2b1a1,...]
    ];
    ops
}
