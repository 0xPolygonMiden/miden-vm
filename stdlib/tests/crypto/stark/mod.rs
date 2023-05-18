mod verifier_recursive;

use verifier_recursive::{generate_advice_inputs, VerifierData};

use crate::build_test;
use assembly::Assembler;
use miden_air::{FieldExtension, HashFunction, PublicInputs, Felt, StarkField, FieldElement};
use test_utils::{
    prove, AdviceInputs, MemAdviceProvider, ProgramInfo, ProofOptions, StackInputs, VerifierError, rand::rand_value,
};

use self::verifier_recursive::QuadExt;

#[test]
fn random_combinate_main_1() {

    let source = "
        use.std::crypto::stark::random_combine

        begin
            # Store randomness [a0, a1, a0', a1']
            mem_storew.1000
            dropw

            # Store OOD evaluation of column i at z and gz
            mem_storew.2000
            dropw

            exec.random_combine::main_1
        end
        ";

    let b = 0_u64;
    let a_addr = 1000_u64;
    let z_addr = 2000_u64;
    let x_addr = 3000_u64;

    let r0: Felt = rand_value();
    let r1: Felt = rand_value();
    let p0: Felt = rand_value();
    let p1: Felt = rand_value();

    let t0: Felt = rand_value();
    let t1: Felt = rand_value();
    let t2: Felt = rand_value();
    let t3: Felt = rand_value();
    let t4: Felt = rand_value();
    let t5: Felt = rand_value();
    let t6: Felt = rand_value();
    let t7: Felt = rand_value();
    
    let tz0: Felt = rand_value();
    let tz1: Felt = rand_value();
    let tgz0: Felt = rand_value();
    let tgz1: Felt = rand_value();

    let a00: Felt = rand_value();
    let a01: Felt = rand_value();
    let a10: Felt = rand_value();
    let a11: Felt = rand_value();

    // Compute the result of ɑ_i * (T_i(x) - T_i(z)). `random_combine::main_1` uses the deepest of
    // the two ɑ_i's while `random_combine::main_2` uses the other one.
    let r_new = QuadExt::new(r0, r1);
    let alpha_0 = QuadExt::new(a00, a01);
    let t_i_x = QuadExt::new(t0, Felt::ZERO);
    let t_i_z = QuadExt::new(tz0, tz1);
    let res_z = r_new + alpha_0 * (t_i_x - t_i_z);
    let (res_z0, res_z1) = (res_z.base_element(0), res_z.base_element(1));

    // Compute the result of ɑ_i * (T_i(x) - T_i(gz))
    let p_new = QuadExt::new(p0, p1);
    let t_i_x = QuadExt::new(t0, Felt::ZERO);
    let t_i_gz = QuadExt::new(tgz0, tgz1);
    let res_gz = p_new + alpha_0 * (t_i_x - t_i_gz);
    let (res_gz0, res_gz1) = (res_gz.base_element(0), res_gz.base_element(1));

    let mut stack = vec![];

    stack.push(b);
    stack.push(a_addr);
    stack.push(z_addr);
    stack.push(x_addr);

    stack.push(r0.as_int());
    stack.push(r1.as_int());
    stack.push(p0.as_int());
    stack.push(p1.as_int());

    stack.push(t0.as_int());
    stack.push(t1.as_int());
    stack.push(t2.as_int());
    stack.push(t3.as_int());
    stack.push(t4.as_int());
    stack.push(t5.as_int());
    stack.push(t6.as_int());
    stack.push(t7.as_int());

    stack.push(tz0.as_int());
    stack.push(tz1.as_int());
    stack.push(tgz0.as_int());
    stack.push(tgz1.as_int());

    stack.push(a00.as_int());
    stack.push(a01.as_int());
    stack.push(a10.as_int());
    stack.push(a11.as_int());

    let test = build_test!(source, &stack[..]);

    let mut stack = vec![];

    stack.push(1 - b);
    stack.push(a_addr + b);
    stack.push(z_addr + 1);
    stack.push(x_addr);
    stack.push(res_z0.as_int());
    stack.push(res_z1.as_int());
    stack.push(res_gz0.as_int());
    stack.push(res_gz1.as_int());
    stack.push(t1.as_int());
    stack.push(t2.as_int());
    stack.push(t3.as_int());
    stack.push(t4.as_int());
    stack.push(t5.as_int());
    stack.push(t6.as_int());
    stack.push(t7.as_int());
    stack.push(t0.as_int());

    stack.reverse();

    test.expect_stack(&stack[..]);
}

#[test]
fn random_combinate_main_2() {

    let source = "
        use.std::crypto::stark::random_combine

        begin
            # Store randomness [a0, a1, a0', a1']
            mem_storew.1000
            dropw

            # Store OOD evaluation of column i at z and gz
            mem_storew.2000
            dropw

            exec.random_combine::main_2
        end
        ";

    let b = 1_u64;
    let a_addr = 1000_u64;
    let z_addr = 2000_u64;
    let x_addr = 3000_u64;

    let r0: Felt = rand_value();
    let r1: Felt = rand_value();
    let p0: Felt = rand_value();
    let p1: Felt = rand_value();

    let t0: Felt = rand_value();
    let t1: Felt = rand_value();
    let t2: Felt = rand_value();
    let t3: Felt = rand_value();
    let t4: Felt = rand_value();
    let t5: Felt = rand_value();
    let t6: Felt = rand_value();
    let t7: Felt = rand_value();
    
    let tz0: Felt = rand_value();
    let tz1: Felt = rand_value();
    let tgz0: Felt = rand_value();
    let tgz1: Felt = rand_value();

    let a00: Felt = rand_value();
    let a01: Felt = rand_value();
    let a10: Felt = rand_value();
    let a11: Felt = rand_value();

    // Compute the result of ɑ_i * (T_i(x) - T_i(z)). `random_combine::main_1` uses the deepest of
    // the two ɑ_i's while `random_combine::main_2` uses the other one.
    let r_new = QuadExt::new(r0, r1);
    let alpha_1 = QuadExt::new(a10, a11);
    let t_i_x = QuadExt::new(t0, Felt::ZERO);
    let t_i_z = QuadExt::new(tz0, tz1);
    let res_z = r_new + alpha_1 * (t_i_x - t_i_z);
    let (res_z0, res_z1) = (res_z.base_element(0), res_z.base_element(1));

    // Compute the result of ɑ_i * (T_i(x) - T_i(gz))
    let p_new = QuadExt::new(p0, p1);
    let t_i_x = QuadExt::new(t0, Felt::ZERO);
    let t_i_gz = QuadExt::new(tgz0, tgz1);
    let res_gz = p_new + alpha_1 * (t_i_x - t_i_gz);
    let (res_gz0, res_gz1) = (res_gz.base_element(0), res_gz.base_element(1));

    let mut stack = vec![];

    stack.push(b);
    stack.push(a_addr);
    stack.push(z_addr);
    stack.push(x_addr);

    stack.push(r0.as_int());
    stack.push(r1.as_int());
    stack.push(p0.as_int());
    stack.push(p1.as_int());

    stack.push(t0.as_int());
    stack.push(t1.as_int());
    stack.push(t2.as_int());
    stack.push(t3.as_int());
    stack.push(t4.as_int());
    stack.push(t5.as_int());
    stack.push(t6.as_int());
    stack.push(t7.as_int());

    stack.push(tz0.as_int());
    stack.push(tz1.as_int());
    stack.push(tgz0.as_int());
    stack.push(tgz1.as_int());

    stack.push(a00.as_int());
    stack.push(a01.as_int());
    stack.push(a10.as_int());
    stack.push(a11.as_int());

    let test = build_test!(source, &stack[..]);

    let mut stack = vec![];

    stack.push(1 - b);
    stack.push(a_addr + b);
    stack.push(z_addr + 1);
    stack.push(x_addr);
    stack.push(res_z0.as_int());
    stack.push(res_z1.as_int());
    stack.push(res_gz0.as_int());
    stack.push(res_gz1.as_int());
    stack.push(t1.as_int());
    stack.push(t2.as_int());
    stack.push(t3.as_int());
    stack.push(t4.as_int());
    stack.push(t5.as_int());
    stack.push(t6.as_int());
    stack.push(t7.as_int());
    stack.push(t0.as_int());

    stack.reverse();

    test.expect_stack(&stack[..]);
}

#[test]
fn random_combinate_aux_1() {

    let source = "
        use.std::crypto::stark::random_combine

        begin
            # Store randomness [a0, a1, a0', a1']
            mem_storew.1000
            dropw

            # Store OOD evaluation of column i at z and gz
            mem_storew.2000
            dropw

            exec.random_combine::aux_1
        end
        ";

    let b = 0_u64;
    let a_addr = 1000_u64;
    let z_addr = 2000_u64;
    let x_addr = 3000_u64;

    let r0: Felt = rand_value();
    let r1: Felt = rand_value();
    let p0: Felt = rand_value();
    let p1: Felt = rand_value();

    let t00: Felt = rand_value();
    let t01: Felt = rand_value();
    let t10: Felt = rand_value();
    let t11: Felt = rand_value();
    let t20: Felt = rand_value();
    let t21: Felt = rand_value();
    let t30: Felt = rand_value();
    let t31: Felt = rand_value();
    
    let tz0: Felt = rand_value();
    let tz1: Felt = rand_value();
    let tgz0: Felt = rand_value();
    let tgz1: Felt = rand_value();

    let a00: Felt = rand_value();
    let a01: Felt = rand_value();
    let a10: Felt = rand_value();
    let a11: Felt = rand_value();

    // Compute the result of ɑ_i * (T_i(x) - T_i(z)). `random_combine::aux_1` uses the deepest of
    // the two ɑ_i's while `random_combine::aux_2` uses the other one.
    let r_new = QuadExt::new(r0, r1);
    let alpha_0 = QuadExt::new(a00, a01);
    let t_i_x = QuadExt::new(t00, t01);
    let t_i_z = QuadExt::new(tz0, tz1);
    let res_z = r_new + alpha_0 * (t_i_x - t_i_z);
    let (res_z0, res_z1) = (res_z.base_element(0), res_z.base_element(1));

    // Compute the result of ɑ_i * (T_i(x) - T_i(gz))
    let p_new = QuadExt::new(p0, p1);
    let t_i_x = QuadExt::new(t00, t01);
    let t_i_gz = QuadExt::new(tgz0, tgz1);
    let res_gz = p_new + alpha_0 * (t_i_x - t_i_gz);
    let (res_gz0, res_gz1) = (res_gz.base_element(0), res_gz.base_element(1));

    let mut stack = vec![];

    stack.push(b);
    stack.push(a_addr);
    stack.push(z_addr);
    stack.push(x_addr);
    
    stack.push(r0.as_int());
    stack.push(r1.as_int());
    stack.push(p0.as_int());
    stack.push(p1.as_int());

    stack.push(t00.as_int());
    stack.push(t01.as_int());
    stack.push(t10.as_int());
    stack.push(t11.as_int());
    stack.push(t20.as_int());
    stack.push(t21.as_int());
    stack.push(t30.as_int());
    stack.push(t31.as_int());

    stack.push(tz0.as_int());
    stack.push(tz1.as_int());
    stack.push(tgz0.as_int());
    stack.push(tgz1.as_int());

    stack.push(a00.as_int());
    stack.push(a01.as_int());
    stack.push(a10.as_int());
    stack.push(a11.as_int());

    let test = build_test!(source, &stack[..]);

    let mut stack = vec![];

    stack.push(1 - b);
    stack.push(a_addr + b);
    stack.push(z_addr + 1);
    stack.push(x_addr);
    stack.push(res_z0.as_int());
    stack.push(res_z1.as_int());
    stack.push(res_gz0.as_int());
    stack.push(res_gz1.as_int());
    stack.push(t10.as_int());
    stack.push(t11.as_int());
    stack.push(t20.as_int());
    stack.push(t21.as_int());
    stack.push(t30.as_int());
    stack.push(t31.as_int());
    stack.push(t00.as_int());
    stack.push(t01.as_int());

    stack.reverse();

    test.expect_stack(&stack[..]);
}

#[test]
fn random_combinate_aux_2() {

    let source = "
        use.std::crypto::stark::random_combine

        begin
            # Store randomness [a0, a1, a0', a1']
            mem_storew.1000
            dropw

            # Store OOD evaluation of column i at z and gz
            mem_storew.2000
            dropw

            exec.random_combine::aux_2
        end
        ";

    let b = 0_u64;
    let a_addr = 1000_u64;
    let z_addr = 2000_u64;
    let x_addr = 3000_u64;

    let r0: Felt = rand_value();
    let r1: Felt = rand_value();
    let p0: Felt = rand_value();
    let p1: Felt = rand_value();

    let t00: Felt = rand_value();
    let t01: Felt = rand_value();
    let t10: Felt = rand_value();
    let t11: Felt = rand_value();
    let t20: Felt = rand_value();
    let t21: Felt = rand_value();
    let t30: Felt = rand_value();
    let t31: Felt = rand_value();
    
    let tz0: Felt = rand_value();
    let tz1: Felt = rand_value();
    let tgz0: Felt = rand_value();
    let tgz1: Felt = rand_value();

    let a00: Felt = rand_value();
    let a01: Felt = rand_value();
    let a10: Felt = rand_value();
    let a11: Felt = rand_value();

    // Compute the result of ɑ_i * (T_i(x) - T_i(z)). `random_combine::aux_1` uses the deepest of
    // the two ɑ_i's while `random_combine::aux_2` uses the other one.
    let r_new = QuadExt::new(r0, r1);
    let alpha_1 = QuadExt::new(a10, a11);
    let t_i_x = QuadExt::new(t00, t01);
    let t_i_z = QuadExt::new(tz0, tz1);
    let res_z = r_new + alpha_1 * (t_i_x - t_i_z);
    let (res_z0, res_z1) = (res_z.base_element(0), res_z.base_element(1));

    // Compute the result of ɑ_i * (T_i(x) - T_i(gz))
    let p_new = QuadExt::new(p0, p1);
    let t_i_x = QuadExt::new(t00, t01);
    let t_i_gz = QuadExt::new(tgz0, tgz1);
    let res_gz = p_new + alpha_1 * (t_i_x - t_i_gz);
    let (res_gz0, res_gz1) = (res_gz.base_element(0), res_gz.base_element(1));

    let mut stack = vec![];

    stack.push(b);
    stack.push(a_addr);
    stack.push(z_addr);
    stack.push(x_addr);
    
    stack.push(r0.as_int());
    stack.push(r1.as_int());
    stack.push(p0.as_int());
    stack.push(p1.as_int());

    stack.push(t00.as_int());
    stack.push(t01.as_int());
    stack.push(t10.as_int());
    stack.push(t11.as_int());
    stack.push(t20.as_int());
    stack.push(t21.as_int());
    stack.push(t30.as_int());
    stack.push(t31.as_int());

    stack.push(tz0.as_int());
    stack.push(tz1.as_int());
    stack.push(tgz0.as_int());
    stack.push(tgz1.as_int());

    stack.push(a00.as_int());
    stack.push(a01.as_int());
    stack.push(a10.as_int());
    stack.push(a11.as_int());

    let test = build_test!(source, &stack[..]);

    let mut stack = vec![];

    stack.push(1 - b);
    stack.push(a_addr + b);
    stack.push(z_addr + 1);
    stack.push(x_addr);
    stack.push(res_z0.as_int());
    stack.push(res_z1.as_int());
    stack.push(res_gz0.as_int());
    stack.push(res_gz1.as_int());
    stack.push(t10.as_int());
    stack.push(t11.as_int());
    stack.push(t20.as_int());
    stack.push(t21.as_int());
    stack.push(t30.as_int());
    stack.push(t31.as_int());
    stack.push(t00.as_int());
    stack.push(t01.as_int());

    stack.reverse();

    test.expect_stack(&stack[..]);
}

#[test]
fn stark_verifier_e2f4() {
    // An example MASM program to be verified inside Miden VM
    // Note that output stack-overflow is not yet supported because of the way we handle public inputs
    // in the STARK verifier is not yet general enough. Thus the output stack should be of size exactly 16.
    let example_source = "begin
            repeat.32
                swap dup.1 add
            end
        end";
    let mut stack_inputs = vec![0_u64; 16];
    stack_inputs[15] = 0;
    stack_inputs[14] = 1;

    let VerifierData {
        initial_stack,
        tape,
        store,
        advice_map,
    } = generate_recursive_verifier_data(example_source, stack_inputs).unwrap();

    // Verify inside Miden VM
    let source = "
        use.std::crypto::stark::verifier

        begin
            exec.verifier::verify
        end
        ";

    let test = build_test!(source, &initial_stack, &tape, store, advice_map);

    test.expect_stack(&[]);
}

// Helper function for recursive verification
pub fn generate_recursive_verifier_data(
    source: &str,
    stack_inputs: Vec<u64>,
) -> Result<VerifierData, VerifierError> {
    let program = Assembler::default().compile(&source).unwrap();
    let stack_inputs = StackInputs::try_from_values(stack_inputs).unwrap();
    let advice_inputs = AdviceInputs::default();
    let advice_provider = MemAdviceProvider::from(advice_inputs);

    let options =
        ProofOptions::new(27, 8, 16, FieldExtension::Quadratic, 4, 7, HashFunction::Rpo256);

    let (stack_outputs, proof) =
        prove(&program, stack_inputs.clone(), advice_provider, options).unwrap();

    let program_info = ProgramInfo::from(program);

    // build public inputs and generate the advice data needed for recursive proof verification
    let pub_inputs = PublicInputs::new(program_info, stack_inputs, stack_outputs);
    let (_, proof) = proof.into_parts();
    Ok(generate_advice_inputs(proof, pub_inputs).unwrap())
}
