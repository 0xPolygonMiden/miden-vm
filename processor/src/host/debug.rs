use super::{Felt, ProcessState};
use crate::Vec;
use vm_core::DebugOptions;

// DEBUG HANDLER
// ================================================================================================

/// Prints the info about the VM state specified by the provided options to stdout.
pub fn print_debug_info<S: ProcessState>(process: &S, options: &DebugOptions) {
    let clk = process.clk();
    match options {
        DebugOptions::StackAll => {
            let stack = process.get_stack_state();
            let n = stack.len();
            print_vm_stack(clk, stack, n);
        }
        DebugOptions::StackTop(n) => {
            let stack = process.get_stack_state();
            print_vm_stack(clk, stack, *n as usize);
        }
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
