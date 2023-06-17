use super::{AdviceInjector, AdviceProvider, AdviceSource, Decorator, ExecutionError, Process};

mod adv_map_injectors;
mod adv_stack_injectors;

#[cfg(test)]
mod tests;

// DECORATORS
// ================================================================================================

impl<A> Process<A>
where
    A: AdviceProvider,
{
    /// Executes the specified decorator
    pub(super) fn execute_decorator(
        &mut self,
        decorator: &Decorator,
    ) -> Result<(), ExecutionError> {
        match decorator {
            Decorator::Advice(injector) => self.dec_advice(injector)?,
            Decorator::AsmOp(assembly_op) => {
                if self.decoder.in_debug_mode() {
                    self.decoder.append_asmop(self.system.clk(), assembly_op.clone());
                }
            }
        }
        Ok(())
    }

    // ADVICE INJECTION
    // --------------------------------------------------------------------------------------------

    /// Process the specified advice injector.
    pub fn dec_advice(&mut self, injector: &AdviceInjector) -> Result<(), ExecutionError> {
        match injector {
            AdviceInjector::MerkleNodeMerge => self.merge_merkle_nodes(),
            AdviceInjector::MerkleNodeToStack => self.copy_merkle_node_to_adv_stack(),
            AdviceInjector::MapValueToStack {
                include_len,
                key_offset,
            } => self.copy_map_value_to_adv_stack(*include_len, *key_offset),
            AdviceInjector::DivU64 => self.push_u64_div_result(),
            AdviceInjector::Ext2Inv => self.push_ext2_inv_result(),
            AdviceInjector::Ext2Intt => self.push_ext2_intt_result(),
            AdviceInjector::SmtGet => self.push_smtget_inputs(),
            AdviceInjector::MemToMap => self.insert_mem_values_into_adv_map(),
            AdviceInjector::HdwordToMap { domain } => self.insert_hdword_into_adv_map(*domain),
        }
    }

    // INJECTOR HELPERS
    // --------------------------------------------------------------------------------------------

    /// Creates a new Merkle tree in the advice provider by combining Merkle trees with the
    /// specified roots. The root of the new tree is defined as `Hash(LEFT_ROOT, RIGHT_ROOT)`.
    ///
    /// Inputs:
    ///   Operand stack: [RIGHT_ROOT, LEFT_ROOT, ...]
    ///   Merkle store: {RIGHT_ROOT, LEFT_ROOT}
    ///
    /// Outputs:
    ///   Operand stack: [RIGHT_ROOT, LEFT_ROOT, ...]
    ///   Merkle store: {RIGHT_ROOT, LEFT_ROOT, hash(LEFT_ROOT, RIGHT_ROOT)}
    ///
    /// After the operation, both the original trees and the new tree remains in the advice
    /// provider (i.e., the input trees are not removed).
    ///
    /// # Errors
    /// Return an error if a Merkle tree for either of the specified roots cannot be found in this
    /// advice provider.
    fn merge_merkle_nodes(&mut self) -> Result<(), ExecutionError> {
        // fetch the arguments from the stack
        let lhs = self.stack.get_word(1);
        let rhs = self.stack.get_word(0);

        // perform the merge
        self.advice_provider.merge_roots(lhs, rhs)?;

        Ok(())
    }
}
