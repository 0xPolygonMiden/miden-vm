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
            AdviceInjector::MerkleNode => self.inject_merkle_node(),
            AdviceInjector::MerkleMerge => self.inject_merkle_merge(),
            AdviceInjector::DivResultU64 => self.inject_div_result_u64(),
            AdviceInjector::MapValue => self.inject_map_value(),
            AdviceInjector::Ext2Inv => self.inject_ext2_inv_result(),
            AdviceInjector::Ext2INTT => self.inject_ext2_intt_result(),
            AdviceInjector::SmtGet => self.inject_smtget(),
            AdviceInjector::Memory => self.inject_mem_values(),
        }
    }

    // INJECTOR HELPERS
    // --------------------------------------------------------------------------------------------

    /// Creates a new Merkle tree in the advice provider by combining Merkle trees with the
    /// specified roots. The root of the new tree is defined as `hash(left_root, right_root)`.
    ///
    /// The operand stack is expected to be arranged as follows:
    /// - root of the right tree, 4 elements
    /// - root of the left tree, 4 elements
    ///
    /// After the operation, both the original trees and the new tree remains in the advice
    /// provider (i.e., the input trees are not removed).
    ///
    /// # Errors
    /// Return an error if a Merkle tree for either of the specified roots cannot be found in this
    /// advice provider.
    fn inject_merkle_merge(&mut self) -> Result<(), ExecutionError> {
        // fetch the arguments from the stack
        let lhs = self.stack.get_word(1);
        let rhs = self.stack.get_word(0);

        // perform the merge
        self.advice_provider.merge_roots(lhs, rhs)?;

        Ok(())
    }
}
