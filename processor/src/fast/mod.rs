use alloc::vec::Vec;

use vm_core::{
    mast::{BasicBlockNode, MastForest, MastNode, MastNodeId},
    stack::MIN_STACK_DEPTH,
    Felt, Operation, Program, StackOutputs, ONE, ZERO,
};

use crate::ExecutionError;

// temporary module to
pub mod experiments;
mod macro_ops;

#[cfg(test)]
mod tests;

/// A fast processor which doesn't generate any trace.
#[derive(Debug)]
pub struct SpeedyGonzales {
    // The stack is stored in reverse order, so that the last element is at the top of the stack.
    stack: smallvec::SmallVec<[Felt; 512]>,
}

impl SpeedyGonzales {
    pub fn new(stack_inputs: Vec<Felt>) -> Self {
        assert!(stack_inputs.len() <= MIN_STACK_DEPTH);

        SpeedyGonzales { stack: stack_inputs.into() }
    }

    pub fn execute(&mut self, program: &Program) -> Result<StackOutputs, ExecutionError> {
        self.execute_mast_node(program.entrypoint(), program.mast_forest())?;

        StackOutputs::new(self.stack.iter().rev().copied().collect())
            .map_err(|_| ExecutionError::OutputStackOverflow(self.stack.len() - MIN_STACK_DEPTH))
    }

    fn execute_mast_node(
        &mut self,
        node_id: MastNodeId,
        program: &MastForest,
    ) -> Result<(), ExecutionError> {
        let node = program
            .get_node_by_id(node_id)
            .ok_or(ExecutionError::MastNodeNotFoundInForest { node_id })?;

        match node {
            MastNode::Block(basic_block_node) => {
                self.execute_basic_block_node(basic_block_node, program)
            },
            MastNode::Join(join_node) => {
                self.execute_mast_node(join_node.first(), program)?;
                self.execute_mast_node(join_node.second(), program)
            },
            MastNode::Split(split_node) => {
                let condition = self.stack[0];
                if condition == ONE {
                    self.execute_mast_node(split_node.on_true(), program)
                } else if condition == ZERO {
                    self.execute_mast_node(split_node.on_false(), program)
                } else {
                    return Err(ExecutionError::NotBinaryValue(condition));
                }
            },
            MastNode::Loop(_loop_node) => todo!(),
            MastNode::Call(_call_node) => todo!(),
            MastNode::Dyn(_dyn_node) => todo!(),
            MastNode::External(_external_node) => todo!(),
        }
    }

    fn execute_basic_block_node(
        &mut self,
        basic_block_node: &BasicBlockNode,
        _program: &MastForest,
    ) -> Result<(), ExecutionError> {
        let mut triplet_op_iter = TripletIterator::new(basic_block_node.operations());

        while let Some((op1, op2, op3)) = triplet_op_iter.next() {
            match (op1, op2, op3) {
                (Operation::Swap, Operation::Dup1, Operation::Add) => self.swap_dup1_add(),
                _ => {
                    // No macro operation found; execute the first operation, and push the other two
                    // back at the front of the iterator
                    self.execute_op(op1, _program)?;
                    triplet_op_iter.push_front(op2, op3);
                },
            }
        }

        // Execute any remaining operations
        {
            let (first, second) = triplet_op_iter.remainder();
            if let Some(op) = first {
                self.execute_op(op, _program)?;
            }
            if let Some(op) = second {
                self.execute_op(op, _program)?;
            }
        }

        Ok(())
    }

    fn execute_op(
        &mut self,
        operation: &Operation,
        _program: &MastForest,
    ) -> Result<(), ExecutionError> {
        match operation {
            Operation::Noop => todo!(),
            Operation::Assert(_) => todo!(),
            Operation::FmpAdd => todo!(),
            Operation::FmpUpdate => todo!(),
            Operation::SDepth => todo!(),
            Operation::Caller => todo!(),
            Operation::Clk => todo!(),
            Operation::Emit(_) => todo!(),
            Operation::Join => todo!(),
            Operation::Split => todo!(),
            Operation::Loop => todo!(),
            Operation::Call => todo!(),
            Operation::Dyn => todo!(),
            Operation::Dyncall => todo!(),
            Operation::SysCall => todo!(),
            Operation::Span => todo!(),
            Operation::End => todo!(),
            Operation::Repeat => todo!(),
            Operation::Respan => todo!(),
            Operation::Halt => todo!(),
            Operation::Add => match self.stack.len() {
                0 | 1 => (),
                _ => {
                    let top = self.stack.pop().unwrap_or(ZERO);
                    *self.stack.last_mut().unwrap() += top;
                },
            },
            Operation::Neg => todo!(),
            Operation::Mul => todo!(),
            Operation::Inv => todo!(),
            Operation::Incr => todo!(),
            Operation::And => todo!(),
            Operation::Or => todo!(),
            Operation::Not => todo!(),
            Operation::Eq => todo!(),
            Operation::Eqz => todo!(),
            Operation::Expacc => todo!(),
            Operation::Ext2Mul => todo!(),
            Operation::U32split => todo!(),
            Operation::U32add => todo!(),
            Operation::U32assert2(_) => todo!(),
            Operation::U32add3 => todo!(),
            Operation::U32sub => todo!(),
            Operation::U32mul => todo!(),
            Operation::U32madd => todo!(),
            Operation::U32div => todo!(),
            Operation::U32and => todo!(),
            Operation::U32xor => todo!(),
            Operation::Pad => todo!(),
            Operation::Drop => todo!(),
            Operation::Dup0 => match self.stack.last() {
                Some(&last_element) => self.stack.push(last_element),
                // stack is empty, so we're duplicating ZERO
                None => (),
            },
            Operation::Dup1 => match self.stack.len() {
                0 => (),
                1 => self.stack.push(ZERO),
                _ => {
                    let to_dup_index = self.stack.len() - 2;
                    self.stack.push(self.stack[to_dup_index]);
                },
            },
            Operation::Dup2 => todo!(),
            Operation::Dup3 => todo!(),
            Operation::Dup4 => todo!(),
            Operation::Dup5 => todo!(),
            Operation::Dup6 => todo!(),
            Operation::Dup7 => todo!(),
            Operation::Dup9 => todo!(),
            Operation::Dup11 => todo!(),
            Operation::Dup13 => todo!(),
            Operation::Dup15 => todo!(),
            Operation::Swap => {
                match self.stack.len() {
                    // We're swapping ZERO with ZERO, which is a no-op
                    0 => (),
                    // the second element on the stack is implicitly ZERO, so swapping puts a
                    // ZERO on top
                    1 => self.stack.push(ZERO),
                    _ => {
                        let last = self.stack.len() - 1;
                        // TODO(plafer): try swap_unchecked
                        self.stack.swap(last, last - 1);
                    },
                }
            },
            Operation::SwapW => todo!(),
            Operation::SwapW2 => todo!(),
            Operation::SwapW3 => todo!(),
            Operation::SwapDW => todo!(),
            Operation::MovUp2 => todo!(),
            Operation::MovUp3 => todo!(),
            Operation::MovUp4 => todo!(),
            Operation::MovUp5 => todo!(),
            Operation::MovUp6 => todo!(),
            Operation::MovUp7 => todo!(),
            Operation::MovUp8 => todo!(),
            Operation::MovDn2 => todo!(),
            Operation::MovDn3 => todo!(),
            Operation::MovDn4 => todo!(),
            Operation::MovDn5 => todo!(),
            Operation::MovDn6 => todo!(),
            Operation::MovDn7 => todo!(),
            Operation::MovDn8 => todo!(),
            Operation::CSwap => todo!(),
            Operation::CSwapW => todo!(),
            Operation::Push(_) => todo!(),
            Operation::AdvPop => todo!(),
            Operation::AdvPopW => todo!(),
            Operation::MLoadW => todo!(),
            Operation::MStoreW => todo!(),
            Operation::MLoad => todo!(),
            Operation::MStore => todo!(),
            Operation::MStream => todo!(),
            Operation::Pipe => todo!(),
            Operation::HPerm => todo!(),
            Operation::MpVerify(_) => todo!(),
            Operation::MrUpdate => todo!(),
            Operation::FriE2F4 => todo!(),
            Operation::RCombBase => todo!(),
        }

        Ok(())
    }
}

// ITER TOOL
// ===============================================================================================

/// An iterator over the elements of another iterator in groups of 3.
///
/// Allows pushing back two elements to be placed at the front of the iterator.
#[derive(Debug)]
pub struct TripletIterator<T>
where
    T: Iterator,
{
    iterator: T,
    first: Option<T::Item>,
    second: Option<T::Item>,
}

impl<T> TripletIterator<T>
where
    T: Iterator,
{
    pub fn new(iterator: T) -> Self {
        TripletIterator { iterator, first: None, second: None }
    }

    /// Pushes back two elements to be placed at the front of the iterator.
    ///
    /// They will be placed in order [first, second].
    pub fn push_front(&mut self, first: T::Item, second: T::Item) {
        debug_assert!(
            self.first.is_none() && self.second.is_none(),
            "push_front was called twice without calling next"
        );

        self.first = Some(first);
        self.second = Some(second);
    }

    /// Returns the remainder of the iterator.
    pub fn remainder(self) -> (Option<T::Item>, Option<T::Item>) {
        (self.first, self.second)
    }
}

impl<T> Iterator for TripletIterator<T>
where
    T: Iterator,
{
    type Item = (T::Item, T::Item, T::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.first.take(), self.second.take()) {
            (Some(first), Some(second)) => {
                // We have front elements, so we try to pull the third element from the iterator
                match self.iterator.next() {
                    Some(third) => Some((first, second, third)),
                    None => {
                        // We don't have a third element, so we store back the first two elements.
                        self.first = Some(first);
                        self.second = Some(second);
                        None
                    },
                }
            },
            (None, None) => {
                // We don't have front elements, so we pull the three elements from the iterator
                let first = self.iterator.next();
                let second = self.iterator.next();
                let third = self.iterator.next();
                match (first, second, third) {
                    (Some(first), Some(second), Some(third)) => Some((first, second, third)),
                    (Some(first), Some(second), None) => {
                        self.first = Some(first);
                        self.second = Some(second);
                        None
                    },
                    (Some(first), None, None) => {
                        self.first = Some(first);
                        None
                    },
                    (None, None, None) => None,
                    _ => unreachable!("we assume that iterator.next() returning `None` implies that future calls will also return `None`"),
                }
            },
            // This should never happen, as we can only `push_front()` two elements at a time.
            _ => unreachable!("first and second should be both Some or both None"),
        }
    }
}
