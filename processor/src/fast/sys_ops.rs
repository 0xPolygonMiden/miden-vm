use vm_core::{Felt, ZERO, mast::MastForest, sys_events::SystemEvent};

use super::{ExecutionError, FastProcessor, ONE};
use crate::{
    ErrorContext, FMP_MIN, Host, ProcessState,
    operations::sys_ops::sys_event_handlers::{
        HDWORD_TO_MAP_WITH_DOMAIN_DOMAIN_OFFSET, copy_map_value_to_adv_stack,
        copy_merkle_node_to_adv_stack, insert_hdword_into_adv_map, insert_hperm_into_adv_map,
        insert_mem_values_into_adv_map, merge_merkle_nodes, push_ext2_intt_result,
        push_ext2_inv_result, push_falcon_mod_result, push_ilog2, push_leading_ones,
        push_leading_zeros, push_smtpeek_result, push_trailing_ones, push_trailing_zeros,
        push_u64_div_result,
    },
    system::FMP_MAX,
};

impl FastProcessor {
    /// Analogous to `Process::op_assert`.
    #[inline(always)]
    pub fn op_assert(
        &mut self,
        err_code: Felt,
        op_idx: usize,
        host: &mut impl Host,
        program: &MastForest,
        err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        if self.stack_get(0) != ONE {
            host.on_assert_failed(&mut ProcessState::new_fast(self, op_idx), err_code);
            let err_msg = program.resolve_error_message(err_code);
            return Err(ExecutionError::failed_assertion(
                self.clk + op_idx,
                err_code,
                err_msg,
                err_ctx,
            ));
        }
        self.decrement_stack_size();
        Ok(())
    }

    /// Analogous to `Process::op_fmpadd`.
    pub fn op_fmpadd(&mut self) {
        let fmp = self.fmp;
        let top = self.stack_get_mut(0);

        *top += fmp;
    }

    /// Analogous to `Process::op_fmpupdate`.
    pub fn op_fmpupdate(&mut self) -> Result<(), ExecutionError> {
        let top = self.stack_get(0);

        let new_fmp = self.fmp + top;
        let new_fmp_int = new_fmp.as_int();
        if !(FMP_MIN..=FMP_MAX).contains(&new_fmp_int) {
            return Err(ExecutionError::InvalidFmpValue(self.fmp, new_fmp));
        }

        self.fmp = new_fmp;
        self.decrement_stack_size();
        Ok(())
    }

    /// Analogous to `Process::op_sdepth`.
    pub fn op_sdepth(&mut self) {
        let depth = self.stack_depth();
        self.increment_stack_size();
        self.stack_write(0, depth.into());
    }

    /// Analogous to `Process::op_caller`.
    pub fn op_caller(&mut self) -> Result<(), ExecutionError> {
        if !self.in_syscall {
            return Err(ExecutionError::CallerNotInSyscall);
        }

        let caller_hash = self.caller_hash;
        self.stack_write_word(0, &caller_hash);

        Ok(())
    }

    /// Analogous to `Process::op_clk`.
    pub fn op_clk(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        self.increment_stack_size();
        self.stack_write(0, (self.clk + op_idx).into());
        Ok(())
    }

    /// Analogous to `Process::op_emit`.
    #[inline(always)]
    pub fn op_emit(
        &mut self,
        event_id: u32,
        op_idx: usize,
        host: &mut impl Host,
        err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        // If it's a system event, handle it directly. Otherwise, forward it to the host.
        if let Some(system_event) = SystemEvent::from_event_id(event_id) {
            self.handle_system_event(system_event, op_idx, err_ctx)
        } else {
            host.on_event(&mut ProcessState::new_fast(self, op_idx), event_id, err_ctx)
        }
    }

    // HELPERS
    // ------------------------------------------------------------------------------------------

    pub(super) fn handle_system_event(
        &mut self,
        system_event: SystemEvent,
        op_idx: usize,
        err_ctx: &impl ErrorContext,
    ) -> Result<(), ExecutionError> {
        let process_state = &mut ProcessState::new_fast(self, op_idx);
        match system_event {
            SystemEvent::MerkleNodeMerge => merge_merkle_nodes(process_state, err_ctx),
            SystemEvent::MerkleNodeToStack => copy_merkle_node_to_adv_stack(process_state, err_ctx),
            SystemEvent::MapValueToStack => {
                copy_map_value_to_adv_stack(process_state, false, err_ctx)
            },
            SystemEvent::MapValueToStackN => {
                copy_map_value_to_adv_stack(process_state, true, err_ctx)
            },
            SystemEvent::U64Div => push_u64_div_result(process_state, err_ctx),
            SystemEvent::FalconDiv => push_falcon_mod_result(process_state, err_ctx),
            SystemEvent::Ext2Inv => push_ext2_inv_result(process_state, err_ctx),
            SystemEvent::Ext2Intt => push_ext2_intt_result(process_state, err_ctx),
            SystemEvent::SmtPeek => push_smtpeek_result(process_state, err_ctx),
            SystemEvent::U32Clz => push_leading_zeros(process_state, err_ctx),
            SystemEvent::U32Ctz => push_trailing_zeros(process_state, err_ctx),
            SystemEvent::U32Clo => push_leading_ones(process_state, err_ctx),
            SystemEvent::U32Cto => push_trailing_ones(process_state, err_ctx),
            SystemEvent::ILog2 => push_ilog2(process_state, err_ctx),

            SystemEvent::MemToMap => insert_mem_values_into_adv_map(process_state),
            SystemEvent::HdwordToMap => insert_hdword_into_adv_map(process_state, ZERO),
            SystemEvent::HdwordToMapWithDomain => {
                let domain = process_state.get_stack_item(HDWORD_TO_MAP_WITH_DOMAIN_DOMAIN_OFFSET);
                insert_hdword_into_adv_map(process_state, domain)
            },
            SystemEvent::HpermToMap => insert_hperm_into_adv_map(process_state),
        }
    }
}
