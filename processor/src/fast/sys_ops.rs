use super::{ExecutionError, SpeedyGonzales, ONE};
use crate::{system::FMP_MAX, FMP_MIN};

impl<const N: usize> SpeedyGonzales<N> {
    pub fn op_assert(&mut self, err_code: u32, op_idx: usize) -> Result<(), ExecutionError> {
        // TODO(plafer): to delegate to the host, we need to create a `ProcessState` from the
        // processor, which requires changes to `ProcessState`.
        if self.stack[self.stack_top_idx - 1] != ONE {
            return Err(ExecutionError::FailedAssertion {
                clk: self.clk + op_idx,
                err_code,
                err_msg: None,
            });
        }

        self.decrement_stack_size();
        Ok(())
    }

    pub fn op_fmp_add(&mut self) {
        let top = &mut self.stack[self.stack_top_idx - 1];
        *top += self.fmp;
    }

    pub fn op_fmp_update(&mut self) -> Result<(), ExecutionError> {
        let top = self.stack[self.stack_top_idx - 1];

        let new_fmp = self.fmp + top;
        if new_fmp.as_int() < FMP_MIN || new_fmp.as_int() > FMP_MAX {
            return Err(ExecutionError::InvalidFmpValue(self.fmp, new_fmp));
        }

        self.fmp = new_fmp;
        self.decrement_stack_size();
        Ok(())
    }

    pub fn op_sdepth(&mut self) {
        let depth = (self.stack_top_idx - self.stack_bot_idx) as u32;
        self.stack[self.stack_top_idx] = depth.into();
        self.increment_stack_size();
    }

    pub fn op_caller(&mut self) -> Result<(), ExecutionError> {
        todo!()
    }

    pub fn op_clk(&mut self) -> Result<(), ExecutionError> {
        todo!()
    }

    pub fn op_emit(&mut self, _event_id: u32) -> Result<(), ExecutionError> {
        todo!()
    }
}
