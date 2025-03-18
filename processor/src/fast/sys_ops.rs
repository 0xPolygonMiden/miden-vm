use vm_core::{sys_events::SystemEvent, utils::range, WORD_SIZE, ZERO};

use super::{ExecutionError, FastProcessor, ONE};
use crate::{
    operations::sys_ops::sys_event_handlers::{
        copy_map_value_to_adv_stack, copy_merkle_node_to_adv_stack, insert_hdword_into_adv_map,
        insert_hperm_into_adv_map, insert_mem_values_into_adv_map, merge_merkle_nodes,
        push_ext2_intt_result, push_ext2_inv_result, push_falcon_mod_result, push_ilog2,
        push_leading_ones, push_leading_zeros, push_smtpeek_result, push_trailing_ones,
        push_trailing_zeros, push_u64_div_result, HDWORD_TO_MAP_WITH_DOMAIN_DOMAIN_OFFSET,
    },
    system::FMP_MAX,
    Host, ProcessState, FMP_MIN,
};

impl FastProcessor {
    /// Analogous to `Process::op_assert`.
    pub fn op_assert(
        &mut self,
        err_code: u32,
        op_idx: usize,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        if self.stack[self.stack_top_idx - 1] != ONE {
            return Err(host.on_assert_failed(ProcessState::new_fast(self, op_idx), err_code));
        }

        self.decrement_stack_size();
        Ok(())
    }

    /// Analogous to `Process::op_fmpadd`.
    pub fn op_fmpadd(&mut self) {
        let top = &mut self.stack[self.stack_top_idx - 1];
        *top += self.fmp;
    }

    /// Analogous to `Process::op_fmpupdate`.
    pub fn op_fmpupdate(&mut self) -> Result<(), ExecutionError> {
        let top = self.stack[self.stack_top_idx - 1];

        let new_fmp = self.fmp + top;
        if new_fmp.as_int() < FMP_MIN || new_fmp.as_int() > FMP_MAX {
            return Err(ExecutionError::InvalidFmpValue(self.fmp, new_fmp));
        }

        self.fmp = new_fmp;
        self.decrement_stack_size();
        Ok(())
    }

    /// Analogous to `Process::op_sdepth`.
    pub fn op_sdepth(&mut self) {
        let depth = (self.stack_top_idx - self.stack_bot_idx) as u32;
        self.stack[self.stack_top_idx] = depth.into();
        self.increment_stack_size();
    }

    /// Analogous to `Process::op_caller`.
    pub fn op_caller(&mut self) -> Result<(), ExecutionError> {
        if !self.in_syscall {
            return Err(ExecutionError::CallerNotInSyscall);
        }

        self.stack[range(self.stack_top_idx - WORD_SIZE, WORD_SIZE)]
            .copy_from_slice(&self.caller_hash);

        Ok(())
    }

    /// Analogous to `Process::op_clk`.
    pub fn op_clk(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        self.stack[self.stack_top_idx] = (self.clk + op_idx).into();
        self.increment_stack_size();
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
            if system_event != SystemEvent::FalconSigToStack {
                self.handle_system_event(system_event, op_idx, host)
            } else {
                // TODO: this is a temporary solution to not classify FalconSigToStack as a system
                // event; this way, we delegate signature generation to the host so that we can
                // apply different strategies for signature generation.
                host.on_event(ProcessState::new_fast(self, op_idx), event_id)
            }
        } else {
            host.on_event(ProcessState::new_fast(self, op_idx), event_id)
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
            SystemEvent::MerkleNodeMerge => merge_merkle_nodes(advice_provider, process_state),
            SystemEvent::MerkleNodeToStack => {
                copy_merkle_node_to_adv_stack(advice_provider, process_state)
            },
            SystemEvent::MapValueToStack => {
                copy_map_value_to_adv_stack(advice_provider, process_state, false)
            },
            SystemEvent::MapValueToStackN => {
                copy_map_value_to_adv_stack(advice_provider, process_state, true)
            },
            SystemEvent::U64Div => push_u64_div_result(advice_provider, process_state),
            SystemEvent::FalconDiv => push_falcon_mod_result(advice_provider, process_state),
            SystemEvent::Ext2Inv => push_ext2_inv_result(advice_provider, process_state),
            SystemEvent::Ext2Intt => push_ext2_intt_result(advice_provider, process_state),
            SystemEvent::SmtPeek => push_smtpeek_result(advice_provider, process_state),
            SystemEvent::U32Clz => push_leading_zeros(advice_provider, process_state),
            SystemEvent::U32Ctz => push_trailing_zeros(advice_provider, process_state),
            SystemEvent::U32Clo => push_leading_ones(advice_provider, process_state),
            SystemEvent::U32Cto => push_trailing_ones(advice_provider, process_state),
            SystemEvent::ILog2 => push_ilog2(advice_provider, process_state),

            SystemEvent::MemToMap => insert_mem_values_into_adv_map(advice_provider, process_state),
            SystemEvent::HdwordToMap => {
                insert_hdword_into_adv_map(advice_provider, process_state, ZERO)
            },
            SystemEvent::HdwordToMapWithDomain => {
                let domain = self.stack_get(HDWORD_TO_MAP_WITH_DOMAIN_DOMAIN_OFFSET);
                insert_hdword_into_adv_map(advice_provider, process_state, domain)
            },
            SystemEvent::HpermToMap => insert_hperm_into_adv_map(advice_provider, process_state),
            SystemEvent::FalconSigToStack => unreachable!("not treated as a system event"),
        }
    }
}
