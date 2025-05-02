use vm_core::{Felt, ZERO, mast::MastForest, sys_events::SystemEvent};

use super::{ExecutionError, FastProcessor, ONE};
use crate::{
    FMP_MIN, Host, ProcessState,
    errors::ErrorContext,
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
    pub fn op_assert(
        &mut self,
        err_code: Felt,
        op_idx: usize,
        host: &mut impl Host,
        program: &MastForest,
    ) -> Result<(), ExecutionError> {
        if self.stack_get(0) != ONE {
            return Err(host.on_assert_failed(
                ProcessState::new_fast(self, op_idx),
                err_code,
                &ErrorContext::default(),
                program,
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
    pub fn op_emit(
        &mut self,
        event_id: u32,
        op_idx: usize,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        // If it's a system event, handle it directly. Otherwise, forward it to the host.
        if let Some(system_event) = SystemEvent::from_event_id(event_id) {
            self.handle_system_event(system_event, op_idx, host)
        } else {
            host.on_event(ProcessState::new_fast(self, op_idx), event_id, &ErrorContext::default())
        }
    }

    // HELPERS
    // ------------------------------------------------------------------------------------------

    pub(super) fn handle_system_event(
        &self,
        system_event: SystemEvent,
        op_idx: usize,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        let advice_provider = host.advice_provider_mut();
        let process_state = ProcessState::new_fast(self, op_idx);
        match system_event {
            SystemEvent::MerkleNodeMerge => {
                merge_merkle_nodes(advice_provider, process_state, &ErrorContext::default())
            },
            SystemEvent::MerkleNodeToStack => copy_merkle_node_to_adv_stack(
                advice_provider,
                process_state,
                &ErrorContext::default(),
            ),
            SystemEvent::MapValueToStack => copy_map_value_to_adv_stack(
                advice_provider,
                process_state,
                false,
                &ErrorContext::default(),
            ),
            SystemEvent::MapValueToStackN => copy_map_value_to_adv_stack(
                advice_provider,
                process_state,
                true,
                &ErrorContext::default(),
            ),
            SystemEvent::U64Div => {
                push_u64_div_result(advice_provider, process_state, &ErrorContext::default())
            },
            SystemEvent::FalconDiv => {
                push_falcon_mod_result(advice_provider, process_state, &ErrorContext::default())
            },
            SystemEvent::Ext2Inv => {
                push_ext2_inv_result(advice_provider, process_state, &ErrorContext::default())
            },
            SystemEvent::Ext2Intt => {
                push_ext2_intt_result(advice_provider, process_state, &ErrorContext::default())
            },
            SystemEvent::SmtPeek => {
                push_smtpeek_result(advice_provider, process_state, &ErrorContext::default())
            },
            SystemEvent::U32Clz => {
                push_leading_zeros(advice_provider, process_state, &ErrorContext::default())
            },
            SystemEvent::U32Ctz => {
                push_trailing_zeros(advice_provider, process_state, &ErrorContext::default())
            },
            SystemEvent::U32Clo => {
                push_leading_ones(advice_provider, process_state, &ErrorContext::default())
            },
            SystemEvent::U32Cto => {
                push_trailing_ones(advice_provider, process_state, &ErrorContext::default())
            },
            SystemEvent::ILog2 => {
                push_ilog2(advice_provider, process_state, &ErrorContext::default())
            },

            SystemEvent::MemToMap => insert_mem_values_into_adv_map(advice_provider, process_state),
            SystemEvent::HdwordToMap => {
                insert_hdword_into_adv_map(advice_provider, process_state, ZERO)
            },
            SystemEvent::HdwordToMapWithDomain => {
                let domain = self.stack_get(HDWORD_TO_MAP_WITH_DOMAIN_DOMAIN_OFFSET);
                insert_hdword_into_adv_map(advice_provider, process_state, domain)
            },
            SystemEvent::HpermToMap => insert_hperm_into_adv_map(advice_provider, process_state),
        }
    }
}
