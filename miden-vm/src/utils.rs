use processor::MemoryAddress;
use vm_core::Felt;

/// Prints the memory address along with the memory value at that address.
pub fn print_mem_address(addr: MemoryAddress, mem_value: Felt) {
    println!("{addr} {mem_value}")
}
