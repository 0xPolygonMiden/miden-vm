use super::{ExecutionError, Felt, HostResult, ProcessState};
use vm_core::DebugOptions;

// DEBUG HANDLER
// ================================================================================================

pub trait DebugHandler {
    /// Handles the provided debug request.
    fn handle_debug<S: ProcessState>(
        &self,
        process: &S,
        options: &DebugOptions,
    ) -> Result<HostResult, ExecutionError> {
        match options {
            DebugOptions::StackAll | DebugOptions::StackTop(_) => {
                self.handle_stack_debug(process, options)
            }
        }
    }

    /// Handles the provided stack debug request.
    fn handle_stack_debug<S: ProcessState>(
        &self,
        process: &S,
        options: &DebugOptions,
    ) -> Result<HostResult, ExecutionError> {
        let clk = process.system().clk();
        match options {
            DebugOptions::StackAll => {
                let stack = process.stack().get_state_at(clk);
                let n = stack.len();
                print_vm_stack(clk, stack, n);
            }
            DebugOptions::StackTop(n) => {
                let stack = process.stack().get_state_at(clk);
                print_vm_stack(clk, stack, *n as usize);
            }
        }

        Ok(HostResult::Unit)
    }

    /// Handles the provided memory debug request.
    fn handle_mem<S: ProcessState>(
        &self,
        _process: &S,
        _options: &DebugOptions,
    ) -> Result<HostResult, ExecutionError> {
        todo!()
    }

    /// Handles the provided local debug request.
    fn handle_local<S: ProcessState>(
        &self,
        _process: &S,
        _options: &DebugOptions,
    ) -> Result<HostResult, ExecutionError> {
        todo!()
    }
}

// HELPER FUNCTIONS
// ================================================================================================

#[cfg(feature = "std")]
fn print_vm_stack(clk: u32, stack: Vec<Felt>, n: usize) {
    // determine how many items to print out
    let num_items = core::cmp::min(stack.len(), n);

    // print all items except for the last one
    println!("Stack state before step {clk}:");
    for (i, element) in stack.iter().take(num_items - 1).enumerate() {
        println!("├── {i:>2}: {element}");
    }

    // print the last item, and in case the stack has more items, print the total number of
    // un-printed items
    let i = num_items - 1;
    if num_items == stack.len() {
        println!("└── {i:>2}: {}", stack[i]);
    } else {
        println!("├── {i:>2}: {}", stack[i]);
        println!("└── ({} more items)", stack.len() - num_items);
    }
}

#[cfg(not(feature = "std"))]
fn print_vm_stack(_clk: u32, _stack: Vec<Felt>, _n: usize) {
    // in no_std environments, this is a NOOP
}
