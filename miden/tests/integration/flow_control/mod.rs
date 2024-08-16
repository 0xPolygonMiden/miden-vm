use alloc::sync::Arc;

use assembly::{ast::ModuleKind, Assembler, LibraryPath, Report, SourceManager};
use miden_vm::Module;
use processor::ExecutionError;
use prover::Digest;
use stdlib::StdLibrary;
use test_utils::{build_test, expect_exec_error, push_inputs, StackInputs, Test};

// SIMPLE FLOW CONTROL TESTS
// ================================================================================================

#[test]
fn conditional_execution() {
    // --- if without else ------------------------------------------------------------------------
    let source = "begin dup.1 dup.1 eq if.true add end end";

    let test = build_test!(source, &[1, 2]);
    test.expect_stack(&[2, 1]);

    let test = build_test!(source, &[3, 3]);
    test.expect_stack(&[6]);

    // --- if with else ------------------------------------------------------------------------
    let source = "begin dup.1 dup.1 eq if.true add else mul end end";

    let test = build_test!(source, &[2, 3]);
    test.expect_stack(&[6]);

    let test = build_test!(source, &[3, 3]);
    test.expect_stack(&[6]);
}

#[test]
fn conditional_loop() {
    // --- entering the loop ----------------------------------------------------------------------
    // computes sum of values from 0 to the value at the top of the stack
    let source = "
        begin
            dup push.0 movdn.2 neq.0
            while.true
                dup movup.2 add swap push.1 sub dup neq.0
            end
            drop
        end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[55]);

    // --- skipping the loop ----------------------------------------------------------------------
    let source = "begin dup eq.0 while.true add end end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[10]);
}

#[test]
fn counter_controlled_loop() {
    // --- entering the loop ----------------------------------------------------------------------
    // compute 2^10
    let source = "
        begin
            push.2
            push.1
            repeat.10
                dup.1 mul
            end
            swap drop
        end";

    let test = build_test!(source);
    test.expect_stack(&[1024]);
}

// NESTED CONTROL FLOW
// ================================================================================================

#[test]
fn if_in_loop() {
    let source = "
        begin
            dup push.0 movdn.2 neq.0
            while.true
                dup movup.2 dup.1 eq.5
                if.true
                    mul
                else
                    add
                end
                swap push.1 sub dup neq.0
            end
            drop
        end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[210]);
}

#[test]
fn if_in_loop_in_if() {
    let source = "
        begin
            dup eq.10
            if.true
                dup push.0 movdn.2 neq.0
                while.true
                    dup movup.2 dup.1 eq.5
                    if.true
                        mul
                    else
                        add
                    end
                    swap push.1 sub dup neq.0
                end
                drop
            else
                dup mul
            end
        end";

    let test = build_test!(source, &[10]);
    test.expect_stack(&[210]);

    let test = build_test!(source, &[11]);
    test.expect_stack(&[121]);
}

// FUNCTION CALLS
// ================================================================================================

#[test]
fn local_fn_call() {
    // returning from a function with non-empty overflow table should result in an error
    let source = "
        proc.foo
            push.1
        end

        begin
            call.foo
        end";

    let build_test = build_test!(source, &[1, 2]);
    expect_exec_error!(build_test, ExecutionError::InvalidStackDepthOnReturn(17));

    let inputs = (1_u64..18).collect::<Vec<_>>();

    // dropping values from the stack in the current execution context should not affect values
    // in the overflow table from the parent execution context
    let source = format!(
        "
        use.std::sys
        
        proc.foo
            repeat.20
                drop
            end
        end

        begin
            {inputs}
            push.18
            call.foo
            repeat.16
                drop
            end

            exec.sys::truncate_stack
        end",
        inputs = push_inputs(&inputs)
    );

    let test = build_test!(source, &[]);
    test.expect_stack(&[2, 1]);

    test.prove_and_verify(vec![], false);
}

#[test]
fn local_fn_call_with_mem_access() {
    // foo should be executed in a different memory context; thus, when we read from memory after
    // calling foo, the value saved into memory[0] before calling foo should still be there.
    let source = "
        use.std::sys

        proc.foo
            mem_store.0
        end

        begin
            mem_store.0
            call.foo
            mem_load.0
            eq.7
            
            exec.sys::truncate_stack
        end";

    let test = build_test!(source, &[3, 7]);
    test.expect_stack(&[1]);

    test.prove_and_verify(vec![3, 7], false);
}

#[test]
fn simple_syscall() {
    let kernel_source = "
        export.foo
            add
        end
    ";

    let program_source = "
        begin
            syscall.foo
        end";

    // TODO: update and use macro?
    let mut test = Test::new(&format!("test{}", line!()), program_source, false);
    test.stack_inputs = StackInputs::try_from_ints([1, 2]).unwrap();
    test.kernel_source = Some(
        test.source_manager
            .load(&format!("kernel{}", line!()), kernel_source.to_string()),
    );
    test.expect_stack(&[3]);

    test.prove_and_verify(vec![1, 2], false);
}

// DYNAMIC CODE EXECUTION
// ================================================================================================

#[test]
fn simple_dyn_exec() {
    let program_source = "
        proc.foo
            # drop the top 4 values, since that will be the code hash when we call this dynamically
            dropw
            add
        end

        begin
            # call foo directly so it will get added to the CodeBlockTable
            padw
            call.foo

            # move the first result of foo out of the way
            movdn.4

            # use dynexec to call foo again via its hash, which is on the stack
            dynexec
        end";

    // The hash of foo can be obtained from the code block table by:
    // let program = test.compile();
    // let cb_table = program.cb_table();
    // Result:
    //   [BaseElement(14592192105906586403), BaseElement(9256464248508904838),
    //    BaseElement(17436090329036592832), BaseElement(10814467189528518943)]
    // Integer values can be obtained via Felt::from_mont(14592192105906586403).as_int(), etc.
    // As ints:
    //   [16045159387802755434, 10308872899350860082, 17306481765929021384, 16642043361554117790]

    let test = Test {
        stack_inputs: StackInputs::try_from_ints([
            3,
            // put the hash of foo on the stack
            16045159387802755434,
            10308872899350860082,
            17306481765929021384,
            16642043361554117790,
            1,
            2,
        ])
        .unwrap(),
        ..Test::new(&format!("test{}", line!()), program_source, false)
    };

    test.expect_stack(&[6]);

    test.prove_and_verify(
        vec![
            3,
            16045159387802755434,
            10308872899350860082,
            17306481765929021384,
            16642043361554117790,
            1,
            2,
        ],
        false,
    );
}

#[test]
fn dynexec_with_procref() {
    let program_source = "
    use.external::module

    proc.foo
        dropw
        push.1.2
        u32wrapping_add
    end

    begin
        procref.foo
        dynexec

        procref.module::func
        dynexec

        dup
        push.4
        assert_eq.err=101
    end";

    let mut test = build_test!(program_source, &[]);
    test.libraries = vec![StdLibrary::default().into()];
    test.add_module(
        "external::module".parse().unwrap(),
        "\
        export.func
            dropw
            u32wrapping_add.1
        end
        ",
    );

    test.expect_stack(&[4]);
}

#[test]
fn simple_dyncall() {
    let program_source = "
        use.std::sys

        proc.foo
            # drop the top 4 values, since that will be the code hash when we call this dynamically
            dropw

            # test that the execution context has changed
            mem_load.0 assertz

            # add the two values on top of the stack
            add
        end

        begin
            # write to memory so we can test that `call` and `dyncall` change the execution context
            push.5 mem_store.0

            # call foo directly so it will get added to the CodeBlockTable
            padw
            call.foo

            # move the first result of foo out of the way
            movdn.4

            # use dyncall to call foo again via its hash, which is on the stack
            dyncall

            exec.sys::truncate_stack
        end";

    // The hash of foo can be obtained from the code block table by:
    // let program = test.compile();
    // let cb_table = program.cb_table();
    // Result:
    //   [BaseElement(3961142802598954486), BaseElement(5305628994393606376),
    //    BaseElement(7971171833137344204), BaseElement(10465350313512331391)]
    // Integer values can be obtained via Felt::from_mont(14592192105906586403).as_int(), etc.
    // As ints:
    //   [8324248212344458853, 17691992706129158519, 18131640149172243086, 16129275750103409835]

    let test = Test {
        stack_inputs: StackInputs::try_from_ints([
            3,
            // put the hash of foo on the stack
            8324248212344458853,
            17691992706129158519,
            18131640149172243086,
            16129275750103409835,
            1,
            2,
        ])
        .unwrap(),
        libraries: vec![StdLibrary::default().into()],
        ..Test::new(&format!("test{}", line!()), program_source, false)
    };

    test.expect_stack(&[6]);

    test.prove_and_verify(
        vec![
            3,
            8324248212344458853,
            17691992706129158519,
            18131640149172243086,
            16129275750103409835,
            1,
            2,
        ],
        false,
    );
}

// PROCREF INSTRUCTION
// ================================================================================================

#[test]
fn procref() -> Result<(), Report> {
    let module_source = "
    use.std::math::u64
    export.u64::overflowing_add

    export.foo.4
        push.3.4
    end
    ";

    // obtain procedures' MAST roots by compiling them as module
    let mast_roots: Vec<Digest> = {
        let source_manager = Arc::new(assembly::DefaultSourceManager::default());
        let module_path = "test::foo".parse::<LibraryPath>().unwrap();
        let mut parser = Module::parser(ModuleKind::Library);
        let module = parser.parse_str(module_path, module_source, &source_manager)?;
        let library = Assembler::new(source_manager)
            .with_library(StdLibrary::default())
            .unwrap()
            .assemble_library([module])
            .unwrap();

        let module_info = library.module_infos().next().unwrap();

        module_info.procedure_digests().collect()
    };

    let source = "
    use.std::math::u64
    use.std::sys

    proc.foo.4
        push.3.4
    end

    begin
        procref.u64::overflowing_add
        push.0
        procref.foo

        exec.sys::truncate_stack
    end";

    let mut test = build_test!(source, &[]);
    test.libraries = vec![StdLibrary::default().into()];

    test.expect_stack(&[
        mast_roots[0][3].as_int(),
        mast_roots[0][2].as_int(),
        mast_roots[0][1].as_int(),
        mast_roots[0][0].as_int(),
        0,
        mast_roots[1][3].as_int(),
        mast_roots[1][2].as_int(),
        mast_roots[1][1].as_int(),
        mast_roots[1][0].as_int(),
    ]);

    test.prove_and_verify(vec![], false);
    Ok(())
}
