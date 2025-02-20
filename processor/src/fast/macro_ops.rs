
use super::SpeedyGonzales;

impl SpeedyGonzales {
    /// Executes Swap, Dup1, Add operations.
    #[inline(always)]
    pub fn swap_dup1_add(&mut self) {
        let stack_size = self.stack.len();
        match stack_size {
            0 => (),
            1 => {
                self.stack.push(self.stack[0]);
            },
            _ => {
                let stack_size = self.stack.len();
                let a = self.stack[stack_size - 1];
                let b = self.stack[stack_size - 2];
                self.stack[stack_size - 1] = a + b;
                self.stack[stack_size - 2] = a;
            },
        }
    }
}
