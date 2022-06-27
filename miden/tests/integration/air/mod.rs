use crate::build_test;
use rand_utils::rand_vector;

mod bitwise;
mod hasher;
mod memory;
mod range;

#[test]
fn aux_table() {
    // Test a script that uses all of the co-processors in the auxiliary table.
    let script = "begin
        rpperm                  # hasher operation
        push.5 push.10 u32or    # bitwise operation
        pow2                    # power of two operation
        push.mem                # memory operation
        drop                    # make sure the stack overflow table is empty
    end";
    let pub_inputs = rand_vector::<u64>(8);

    build_test!(script, &pub_inputs).prove_and_verify(pub_inputs, 0, false);
}
