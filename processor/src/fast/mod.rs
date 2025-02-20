use alloc::vec::Vec;

use vm_core::{
    mast::{BasicBlockNode, MastForest, MastNode, MastNodeId}, stack::MIN_STACK_DEPTH, Felt, Operation, Program, StackOutputs, ONE, ZERO
};

use crate::ExecutionError;

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
        self.execute_mast_node(program.entrypoint(), &program.mast_forest())?;

        Ok(StackOutputs::new(self.stack.iter().rev().copied().collect())
            .map_err(|_| ExecutionError::OutputStackOverflow(self.stack.len() - MIN_STACK_DEPTH))?)
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
        for operation in basic_block_node.operations() {
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
                Operation::Dup1 => {
                    match self.stack.len() {
                        0 => (),
                        1 => self.stack.push(ZERO),
                        _ => {
                            let to_dup_index = self.stack.len() - 2;
                            self.stack.push(self.stack[to_dup_index]);
                        },
                    }
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
        }

        Ok(())
    }
}
