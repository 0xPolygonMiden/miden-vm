use crate::{
    ast::{ModuleAst, ProgramAst},
    Assembler, AssemblyContextType, Library, LibraryNamespace, LibraryPath, Module, Version,
};
use core::slice::Iter;

// SIMPLE PROGRAMS
// ================================================================================================

#[test]
fn simple_instructions() {
    let assembler = super::Assembler::default();
    let source = "begin push.0 assertz end";
    let program = assembler.compile(source).unwrap();
    let expected = "\
        begin \
                span pad eqz assert end \
        end";
    assert_eq!(expected, format!("{program}"));

    let source = "begin push.10 push.50 push.2 u32wrapping_madd end";
    let program = assembler.compile(source).unwrap();
    let expected = "\
        begin \
            span push(10) push(50) push(2) u32madd drop end \
        end";
    assert_eq!(expected, format!("{program}"));

    let source = "begin push.10 push.50 push.2 u32wrapping_add3 end";
    let program = assembler.compile(source).unwrap();
    let expected = "\
        begin \
            span push(10) push(50) push(2) u32add3 drop end \
        end";
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn empty_program() {
    let assembler = super::Assembler::default();
    let source = "begin end";
    let program = assembler.compile(source).unwrap();
    let expected = "begin span noop end end";
    assert_eq!(expected, format!("{}", program));
}

#[test]
fn empty_if() {
    let assembler = super::Assembler::default();
    let source = "begin if.true end end";
    let program = assembler.compile(source).unwrap();
    let expected = "begin if.true span noop end else span noop end end end";
    assert_eq!(expected, format!("{}", program));
}

#[test]
fn empty_while() {
    let assembler = super::Assembler::default();
    let source = "begin while.true end end";
    let program = assembler.compile(source).unwrap();
    let expected = "begin while.true span noop end end end";
    assert_eq!(expected, format!("{}", program));
}

#[test]
fn empty_repeat() {
    let assembler = super::Assembler::default();
    let source = "begin repeat.5 end end";
    let program = assembler.compile(source).unwrap();
    let expected = "begin span noop noop noop noop noop end end";
    assert_eq!(expected, format!("{}", program));
}

#[test]
fn single_span() {
    let assembler = super::Assembler::default();
    let source = "begin push.1 push.2 add end";
    let program = assembler.compile(source).unwrap();
    let expected = "begin span pad incr push(2) add end end";
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn span_and_simple_if() {
    let assembler = super::Assembler::default();

    // if with else
    let source = "begin push.2 push.3 if.true add else mul end end";
    let program = assembler.compile(source).unwrap();
    let expected = "\
        begin \
            join \
                span push(2) push(3) end \
                if.true span add end else span mul end end \
            end \
        end";
    assert_eq!(expected, format!("{program}"));

    // if without else
    let source = "begin push.2 push.3 if.true add end end";
    let program = assembler.compile(source).unwrap();
    let expected = "\
        begin \
            join \
                span push(2) push(3) end \
                if.true span add end else span noop end end \
            end \
        end";
    assert_eq!(expected, format!("{program}"));
}

// PROGRAM WITH #main CALL
// ================================================================================================

#[test]
fn simple_main_call() {
    // instantiate assembler
    let assembler = super::Assembler::default();

    // compile account module
    let account_path = LibraryPath::new("context::account").unwrap();
    let account_code = ModuleAst::parse(
        "\
    export.account_method_1
        push.2.1 add
    end
    
    export.account_method_2
        push.3.1 sub
    end
    ",
    )
    .unwrap();
    let account_module = Module::new(account_path, account_code);
    let _method_roots = assembler
        .compile_module(
            &account_module,
            &mut super::AssemblyContext::new(AssemblyContextType::Module),
        )
        .unwrap();

    // compile note 1 program
    let note_1 =
        ProgramAst::parse("use.context::account begin call.account::account_method_1 end").unwrap();
    let _note_1_root = assembler
        .compile_in_context(&note_1, &mut super::AssemblyContext::new(AssemblyContextType::Program))
        .unwrap();

    // compile note 2 program
    let note_2 =
        ProgramAst::parse("use.context::account begin call.account::account_method_2 end").unwrap();
    let _note_2_root = assembler
        .compile_in_context(&note_2, &mut super::AssemblyContext::new(AssemblyContextType::Program))
        .unwrap();
}

// CONSTANTS
// ================================================================================================

#[test]
fn simple_constant() {
    let assembler = super::Assembler::default();
    let source = "const.TEST_CONSTANT=7 \
    begin \
    push.TEST_CONSTANT \
    end \
    ";
    let expected = "\
    begin \
        span \
            push(7) \
        end \
    end";
    let program = assembler.compile(source).unwrap();
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn multiple_constants_push() {
    let assembler = super::Assembler::default();
    let source = "const.CONSTANT_1=21 \
    const.CONSTANT_2=44 \
    begin \
    push.CONSTANT_1.64.CONSTANT_2.72 \
    end";
    let expected = "\
    begin \
        span \
            push(21) push(64) push(44) push(72) \
        end \
    end";
    let program = assembler.compile(source).unwrap();
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn constants_must_be_uppercase() {
    let assembler = super::Assembler::default();
    let source = "const.constant_1=12 \
    begin \
    push.constant_1 \
    end";
    let result = assembler.compile(source);
    assert!(result.is_err());
    let err = result.err().unwrap();
    let expected_error = "invalid constant name: 'constant_1' cannot contain lower-case characters";
    assert_eq!(expected_error, err.to_string());
}

#[test]
fn duplicate_constant_name() {
    let assembler = super::Assembler::default();
    let source = "const.CONSTANT=12 \
    const.CONSTANT=14 \
    begin \
    push.CONSTANT \
    end";
    let result = assembler.compile(source);
    assert!(result.is_err());
    let err = result.err().unwrap();
    let expected_error = "duplicate constant name: 'CONSTANT'";
    assert_eq!(expected_error, err.to_string());
}

#[test]
fn constant_must_be_valid_felt() {
    let assembler = super::Assembler::default();
    let source = "const.CONSTANT=1122INVALID \
    begin \
    push.CONSTANT \
    end";
    let result = assembler.compile(source);
    assert!(result.is_err());
    let err = result.err().unwrap();
    let expected_error = "malformed constant `const.CONSTANT=1122INVALID` - invalid value: \
     `1122INVALID` - reason: invalid digit found in string";
    assert_eq!(expected_error, err.to_string());
}

#[test]
fn constant_must_be_within_valid_felt_range() {
    let assembler = super::Assembler::default();
    let source = "const.CONSTANT=18446744073709551615 \
    begin \
    push.CONSTANT \
    end";
    let result = assembler.compile(source);
    assert!(result.is_err());
    let err = result.err().unwrap();
    let expected_error = "malformed constant `const.CONSTANT=18446744073709551615` - invalid value: \
     `18446744073709551615` - reason: constant value must be greater than or equal to 0 and less than or \
      equal to 18446744069414584320";
    assert_eq!(expected_error, err.to_string());
}

#[test]
fn constants_defined_in_global_scope() {
    let assembler = super::Assembler::default();
    let source = "
    begin \
    const.CONSTANT=12
    push.CONSTANT \
    end";
    let result = assembler.compile(source);
    assert!(result.is_err());
    let err = result.err().unwrap();
    let expected_error = "invalid constant declaration: `const.CONSTANT=12` - constants can only be defined below imports and above procedure / program bodies";
    assert_eq!(expected_error, err.to_string());
}

#[test]
fn constant_not_found() {
    let assembler = super::Assembler::default();
    let source = "
    begin \
    push.CONSTANT \
    end";
    let result = assembler.compile(source);
    assert!(result.is_err());
    let err = result.err().unwrap();
    let expected_error = "constant used in operation `push.CONSTANT` not found";
    assert_eq!(expected_error, err.to_string());
}

#[test]
fn mem_operations_with_constants() {
    let assembler = super::Assembler::default();

    // Define constant values
    const PROC_LOC_STORE_PTR: u64 = 0;
    const PROC_LOC_LOAD_PTR: u64 = 1;
    const PROC_LOC_STOREW_PTR: u64 = 2;
    const PROC_LOC_LOADW_PTR: u64 = 3;
    const GLOBAL_STORE_PTR: u64 = 4;
    const GLOBAL_LOAD_PTR: u64 = 5;
    const GLOBAL_STOREW_PTR: u64 = 6;
    const GLOBAL_LOADW_PTR: u64 = 7;

    let source = format!(
        "\
    const.PROC_LOC_STORE_PTR={PROC_LOC_STORE_PTR}
    const.PROC_LOC_LOAD_PTR={PROC_LOC_LOAD_PTR}
    const.PROC_LOC_STOREW_PTR={PROC_LOC_STOREW_PTR}
    const.PROC_LOC_LOADW_PTR={PROC_LOC_LOADW_PTR}
    const.GLOBAL_STORE_PTR={GLOBAL_STORE_PTR}
    const.GLOBAL_LOAD_PTR={GLOBAL_LOAD_PTR}
    const.GLOBAL_STOREW_PTR={GLOBAL_STOREW_PTR}
    const.GLOBAL_LOADW_PTR={GLOBAL_LOADW_PTR}

    proc.test_const_loc.4
        # constant should resolve using locaddr operation
        locaddr.PROC_LOC_STORE_PTR

        # constant should resolve using loc_store operation
        loc_store.PROC_LOC_STORE_PTR

        # constant should resolve using loc_load operation
        loc_load.PROC_LOC_LOAD_PTR

        # constant should resolve using loc_storew operation
        loc_storew.PROC_LOC_STOREW_PTR

        # constant should resolve using loc_loadw opeartion
        loc_loadw.PROC_LOC_LOADW_PTR
    end

    begin
        # inline procedure
        exec.test_const_loc

        # constant should resolve using mem_store operation
        mem_store.GLOBAL_STORE_PTR

        # constant should resolve using mem_load operation
        mem_load.GLOBAL_LOAD_PTR

        # constant should resolve using mem_storew operation
        mem_storew.GLOBAL_STOREW_PTR

        # constant should resolve using mem_loadw operation
        mem_loadw.GLOBAL_LOADW_PTR
    end
    "
    );
    let program = assembler.compile(source).unwrap();

    // Define expected
    let expected = format!(
        "\
    proc.test_const_loc.4
        # constant should resolve using locaddr operation
        locaddr.{PROC_LOC_STORE_PTR}

        # constant should resolve using loc_store operation
        loc_store.{PROC_LOC_STORE_PTR}

        # constant should resolve using loc_load operation
        loc_load.{PROC_LOC_LOAD_PTR}

        # constant should resolve using loc_storew operation
        loc_storew.{PROC_LOC_STOREW_PTR}

        # constant should resolve using loc_loadw opeartion
        loc_loadw.{PROC_LOC_LOADW_PTR}
    end

    begin
        # inline procedure
        exec.test_const_loc

        # constant should resolve using mem_store operation
        mem_store.{GLOBAL_STORE_PTR}

        # constant should resolve using mem_load operation
        mem_load.{GLOBAL_LOAD_PTR}

        # constant should resolve using mem_storew operation
        mem_storew.{GLOBAL_STOREW_PTR}

        # constant should resolve using mem_loadw operation
        mem_loadw.{GLOBAL_LOADW_PTR}
    end
    "
    );
    let expected_program = assembler.compile(expected).unwrap();
    assert_eq!(expected_program.to_string(), program.to_string());
}

#[test]
fn const_conversion_failed_to_u16() {
    // Define constant value greater than u16::MAX
    let constant_value: u64 = u16::MAX as u64 + 1;

    let source = format!(
        "\
    const.CONSTANT={constant_value}

    proc.test_constant_overflow.1
        loc_load.CONSTANT
    end

    begin
        exec.test_constant_overflow
    end
    "
    );
    let assembler = super::Assembler::default();
    let result = assembler.compile(source);
    assert!(result.is_err());
    let err = result.err().unwrap();
    let expected_error =
        "failed to convert u64 constant used in `loc_load.CONSTANT` to required type u16";
    assert_eq!(expected_error, err.to_string());
}

#[test]
fn const_conversion_failed_to_u32() {
    // Define constant value greater than u16::MAX
    let constant_value: u64 = u32::MAX as u64 + 1;

    let source = format!(
        "\
    const.CONSTANT={constant_value}

    begin
        mem_load.CONSTANT
    end
    "
    );
    let assembler = super::Assembler::default();
    let result = assembler.compile(source);
    assert!(result.is_err());
    let err = result.err().unwrap();
    let expected_error =
        "failed to convert u64 constant used in `mem_load.CONSTANT` to required type u32";
    assert_eq!(expected_error, err.to_string());
}

// NESTED CONTROL BLOCKS
// ================================================================================================

#[test]
fn nested_control_blocks() {
    let assembler = super::Assembler::default();

    // if with else
    let source = "begin \
        push.2 push.3 \
        if.true \
            add while.true push.7 push.11 add end \
        else \
            mul repeat.2 push.8 end if.true mul end  \
        end
        push.3 add
        end";
    let program = assembler.compile(source).unwrap();
    let expected = "\
        begin \
            join \
                join \
                    span push(2) push(3) end \
                    if.true \
                        join \
                            span add end \
                            while.true span push(7) push(11) add end end \
                        end \
                    else \
                        join \
                            span mul push(8) push(8) end \
                            if.true span mul end else span noop end end \
                        end \
                    end \
                end \
            span push(3) add end \
            end \
        end";
    assert_eq!(expected, format!("{program}"));
}

// PROGRAMS WITH PROCEDURES
// ================================================================================================

#[test]
fn program_with_one_procedure() {
    let assembler = super::Assembler::default();
    let source = "proc.foo push.3 push.7 mul end begin push.2 push.3 add exec.foo end";
    let program = assembler.compile(source).unwrap();
    let expected = "begin span push(2) push(3) add push(3) push(7) mul end end";
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn program_with_one_empty_procedure() {
    let assembler = super::Assembler::default();
    let source = "proc.foo end begin exec.foo end";
    let program = assembler.compile(source).unwrap();
    let expected = "begin span noop end end";
    assert_eq!(expected, format!("{}", program));
}

#[test]
fn program_with_nested_procedure() {
    let assembler = super::Assembler::default();
    let source = "\
        proc.foo push.3 push.7 mul end \
        proc.bar push.5 exec.foo add end \
        begin push.2 push.4 add exec.foo push.11 exec.bar sub end";
    let program = assembler.compile(source).unwrap();
    let expected = "begin \
        span push(2) push(4) add push(3) push(7) mul \
        push(11) push(5) push(3) push(7) mul add neg add \
        end end";
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn program_with_proc_locals() {
    let assembler = super::Assembler::default();
    let source = "\
        proc.foo.1 \
            loc_store.0 \
            add \
            loc_load.0 \
            mul \
        end \
        begin \
            push.4 push.3 push.2 \
            exec.foo \
        end";
    let program = assembler.compile(source).unwrap();
    let expected = "\
        begin \
            span \
                push(4) push(3) push(2) \
                push(1) fmpupdate \
                pad fmpadd mstore drop \
                add \
                pad fmpadd mload \
                mul \
                push(18446744069414584320) fmpupdate \
            end \
        end";
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn program_with_exported_procedure() {
    let assembler = super::Assembler::default();
    let source = "export.foo push.3 push.7 mul end begin push.2 push.3 add exec.foo end";
    assert!(assembler.compile(source).is_err());
}

// MAST ROOT CALLS
// ================================================================================================

#[test]
fn program_with_incorrect_mast_root_length() {
    let assembler = super::Assembler::default();
    let source = "begin call.0x1234 end";
    let result = assembler.compile(source);
    let err = result.err().unwrap();
    let expected_error = "invalid procedure root invocation: 0x1234 - rpo digest hex label must have 66 characters, but was 6";
    assert_eq!(expected_error, err.to_string());
}

#[test]
fn program_with_invalid_mast_root_chars() {
    let assembler = super::Assembler::default();
    let source =
        "begin call.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a21xyzb end";
    let result = assembler.compile(source);
    let err = result.err().unwrap();
    let expected_error = "invalid procedure root invocation: 0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a21xyzb - \
    '0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a21xyzb' contains invalid hex characters";
    assert_eq!(expected_error, err.to_string());
}

#[test]
fn program_with_invalid_rpo_digest_call() {
    let assembler = super::Assembler::default();
    let source =
        "begin call.0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff end";
    let result = assembler.compile(source);
    let err = result.err().unwrap();
    let expected_error = "invalid procedure root invocation: 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff - \
    '0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff' is not a valid Rpo Digest hex label";
    assert_eq!(expected_error, err.to_string());
}

#[test]
fn program_with_mast_root_call_that_does_not_exist() {
    let assembler = super::Assembler::default();
    let source =
        "begin call.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a213dae end";
    let result = assembler.compile(source);
    let err = result.err().unwrap();
    let expected_error = "procedure mast root not found for digest - 0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a213dae";
    assert_eq!(expected_error, err.to_string());
}

// IMPORTS
// ================================================================================================

#[test]
fn program_with_one_import_and_hex_call() {
    const NAMESPACE: &str = "dummy";
    const MODULE: &str = "math::u256";
    const PROCEDURE: &str = r#"
        export.iszero_unsafe
            eq.0
            repeat.7
                swap
                eq.0
                and
            end
        end"#;

    let namespace = LibraryNamespace::try_from(NAMESPACE.to_string()).unwrap();
    let path = LibraryPath::try_from(MODULE.to_string()).unwrap().prepend(&namespace).unwrap();
    let ast = ModuleAst::parse(PROCEDURE).unwrap();
    let modules = vec![Module { path, ast }];
    let library = DummyLibrary::new(namespace, modules);

    let assembler = super::Assembler::default().with_library(&library).unwrap();
    let source = format!(
        r#"
        use.{NAMESPACE}::{MODULE}
        begin
            push.4 push.3
            exec.u256::iszero_unsafe
            call.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a213dae
        end"#
    );
    let program = assembler.compile(source).unwrap();
    let expected = "\
        begin \
            join \
                span \
                    push(4) push(3) \
                    eqz \
                    swap eqz and \
                    swap eqz and \
                    swap eqz and \
                    swap eqz and \
                    swap eqz and \
                    swap eqz and \
                    swap eqz and \
                end \
                call.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a213dae \
            end \
        end";
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn program_with_reexported_proc_in_same_library() {
    // exprted proc is in same library
    const NAMESPACE: &str = "dummy1";
    const REF_MODULE: &str = "math::u64";
    const REF_MODULE_BODY: &str = r#"
        export.checked_eqz
            u32assert.2
            eq.0
            swap
            eq.0
            and
        end
        export.unchecked_eqz
            eq.0
            swap
            eq.0
            and
        end
    "#;

    const MODULE: &str = "math::u256";
    const MODULE_BODY: &str = r#"
        use.dummy1::math::u64

        #! checked_eqz checks if the value is u32 and zero and returns 1 if it is, 0 otherwise
        export.u64::checked_eqz # re-export

        #! unchecked_eqz checks if the value is zero and returns 1 if it is, 0 otherwise
        export.u64::unchecked_eqz->notchecked_eqz # re-export with alias
    "#;

    let namespace = LibraryNamespace::try_from(NAMESPACE.to_string()).unwrap();
    let path = LibraryPath::try_from(MODULE.to_string()).unwrap().prepend(&namespace).unwrap();
    let ast = ModuleAst::parse(MODULE_BODY).unwrap();

    // check docs
    let docs_checked_eqz = ast.reexported_procs().get(0).unwrap().docs().unwrap();
    assert_eq!(
        docs_checked_eqz,
        "checked_eqz checks if the value is u32 and zero and returns 1 if it is, 0 otherwise"
    );
    let docs_unchecked_eqz = ast.reexported_procs().get(1).unwrap().docs().unwrap();
    assert_eq!(
        docs_unchecked_eqz,
        "unchecked_eqz checks if the value is zero and returns 1 if it is, 0 otherwise"
    );

    let ref_path = LibraryPath::try_from(REF_MODULE.to_string())
        .unwrap()
        .prepend(&namespace)
        .unwrap();
    let ref_ast = ModuleAst::parse(REF_MODULE_BODY).unwrap();
    let modules = vec![
        Module { path, ast },
        Module {
            path: ref_path,
            ast: ref_ast,
        },
    ];
    let assembler = super::Assembler::default()
        .with_library(&DummyLibrary::new(namespace, modules))
        .unwrap();
    let source = format!(
        r#"
        use.{NAMESPACE}::{MODULE}
        begin
            push.4 push.3
            exec.u256::checked_eqz
            exec.u256::notchecked_eqz
        end"#
    );
    let program = assembler.compile(source).unwrap();
    let expected = "\
        begin \
            span \
                push(4) push(3) \
                u32assert2 \
                eqz swap eqz and \
                eqz swap eqz and \
            end \
        end";
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn program_with_reexported_proc_in_another_library() {
    // when re-exported proc is part of a different library
    const NAMESPACE: &str = "dummy1";
    const REF_NAMESPACE: &str = "dummy2";
    const REF_MODULE: &str = "math::u64";
    const REF_MODULE_BODY: &str = r#"
        export.checked_eqz
            u32assert.2
            eq.0
            swap
            eq.0
            and
        end
        export.unchecked_eqz
            eq.0
            swap
            eq.0
            and
        end
    "#;

    const MODULE: &str = "math::u256";
    const MODULE_BODY: &str = r#"
        use.dummy2::math::u64
        export.u64::checked_eqz # re-export
        export.u64::unchecked_eqz->notchecked_eqz # re-export with alias
    "#;
    let namespace = LibraryNamespace::try_from(NAMESPACE.to_string()).unwrap();
    let path = LibraryPath::try_from(MODULE.to_string()).unwrap().prepend(&namespace).unwrap();
    let ast = ModuleAst::parse(MODULE_BODY).unwrap();

    let ref_namespace = LibraryNamespace::try_from(REF_NAMESPACE.to_string()).unwrap();
    let ref_path = LibraryPath::try_from(REF_MODULE.to_string())
        .unwrap()
        .prepend(&ref_namespace)
        .unwrap();
    let ref_ast = ModuleAst::parse(REF_MODULE_BODY).unwrap();
    let modules = vec![Module { path, ast }];
    let ref_modules = vec![Module {
        path: ref_path,
        ast: ref_ast,
    }];
    let dummy_library_1 = DummyLibrary::new(namespace, modules);
    let dummy_library_2 = DummyLibrary::new(ref_namespace, ref_modules);
    let assembler = super::Assembler::default()
        .with_libraries([&dummy_library_1, &dummy_library_2].into_iter())
        .unwrap();
    let source = format!(
        r#"
        use.{NAMESPACE}::{MODULE}
        begin
            push.4 push.3
            exec.u256::checked_eqz
            exec.u256::notchecked_eqz
        end"#
    );
    let program = assembler.compile(source).unwrap();
    let expected = "\
        begin \
            span \
                push(4) push(3) \
                u32assert2 \
                eqz swap eqz and \
                eqz swap eqz and \
            end \
        end";
    assert_eq!(expected, format!("{program}"));

    // when the re-exported proc is part of a different library and the library is not passed to
    // the assembler it should fail
    let assembler = super::Assembler::default().with_library(&dummy_library_1).unwrap();
    let source = format!(
        r#"
        use.{NAMESPACE}::{MODULE}
        begin
            push.4 push.3
            exec.u256::checked_eqz
            exec.u256::notchecked_eqz
        end"#
    );
    assert!(assembler.compile(source).is_err());
}

#[test]
fn program_with_import_errors() {
    // --- non-existent import ------------------------------------------------
    let assembler = super::Assembler::default();
    let source = "\
        use.std::math::u512
        begin \
            push.4 push.3 \
            exec.u512::iszero_unsafe \
        end";
    assert!(assembler.compile(source).is_err());

    // --- non-existent procedure in import -----------------------------------
    let assembler = super::Assembler::default();
    let source = "\
        use.std::math::u256
        begin \
            push.4 push.3 \
            exec.u256::foo \
        end";
    assert!(assembler.compile(source).is_err());
}

// COMMENTS
// ================================================================================================

#[test]
fn comment_simple() {
    let assembler = super::Assembler::default();
    let source = "begin # simple comment \n push.1 push.2 add end";
    let program = assembler.compile(source).unwrap();
    let expected = "begin span pad incr push(2) add end end";
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn comment_in_nested_control_blocks() {
    let assembler = super::Assembler::default();

    // if with else
    let source = "begin \
        push.1 push.2 \
        if.true \
            # nested comment \n\
            add while.true push.7 push.11 add end \
        else \
            mul repeat.2 push.8 end if.true mul end  \
            # nested comment \n\
        end
        push.3 add
        end";
    let program = assembler.compile(source).unwrap();
    let expected = "\
        begin \
            join \
                join \
                    span pad incr push(2) end \
                    if.true \
                        join \
                            span add end \
                            while.true span push(7) push(11) add end end \
                        end \
                    else \
                        join \
                            span mul push(8) push(8) end \
                            if.true span mul end else span noop end end \
                        end \
                    end \
                end \
            span push(3) add end \
            end \
        end";
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn comment_before_program() {
    let assembler = super::Assembler::default();
    let source = " # starting comment \n begin push.1 push.2 add end";
    let program = assembler.compile(source).unwrap();
    let expected = "begin span pad incr push(2) add end end";
    assert_eq!(expected, format!("{program}"));
}

#[test]
fn comment_after_program() {
    let assembler = super::Assembler::default();
    let source = "begin push.1 push.2 add end # closing comment";
    let program = assembler.compile(source).unwrap();
    let expected = "begin span pad incr push(2) add end end";
    assert_eq!(expected, format!("{program}"));
}

// ERRORS
// ================================================================================================

#[test]
fn invalid_program() {
    let assembler = super::Assembler::default();
    let source = "";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "source code cannot be an empty string");
    }

    let source = " ";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "source code cannot be an empty string");
    }

    let source = "none";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "unexpected token: expected 'begin' but was 'none'");
    }

    let source = "begin add";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "begin without matching end");
    }

    let source = "begin add end mul";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "dangling instructions after program end");
    }
}

#[test]
fn invalid_proc() {
    let assembler = Assembler::default();

    let source = "proc.foo add mul begin push.1 end";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "procedure 'foo' has no matching end");
    }

    let source = "proc.foo add mul proc.bar push.3 end begin push.1 end";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "procedure 'foo' has no matching end");
    }

    let source = "proc.foo add mul end begin push.1 exec.bar end";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "undefined local procedure: bar");
    }

    let source = "proc.123 add mul end begin push.1 exec.123 end";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "invalid procedure name: '123' does not start with a letter");
    }

    let source = "proc.foo add mul end proc.foo push.3 end begin push.1 end";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "duplicate procedure name: foo");
    }
}

#[test]
fn invalid_if_else() {
    let assembler = super::Assembler::default();

    // --- unmatched if ---------------------------------------------------------------------------
    let source = "begin push.1 add if.true mul";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "if without matching else/end");
    }

    // --- unmatched else -------------------------------------------------------------------------
    let source = "begin push.1 add else mul end";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "else without matching if");
    }

    let source = "begin push.1 while.true add else mul end end";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "else without matching if");
    }

    let source = "begin push.1 if.true add else mul else push.1 end end end";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "else without matching if");
    }

    let source = "begin push.1 add if.true mul else add";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "else without matching end");
    }
}

#[test]
fn invalid_repeat() {
    let assembler = super::Assembler::default();

    // unmatched repeat
    let source = "begin push.1 add repeat.10 mul";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "repeat without matching end");
    }

    // invalid iter count
    let source = "begin push.1 add repeat.23x3 mul end end";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(
            error.to_string(),
            "malformed instruction `repeat.23x3`: parameter '23x3' is invalid"
        );
    }
}

#[test]
fn invalid_while() {
    let assembler = super::Assembler::default();

    let source = "begin push.1 add while mul end end";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "malformed instruction 'while': missing required parameter");
    }

    let source = "begin push.1 add while.abc mul end end";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(
            error.to_string(),
            "malformed instruction `while.abc`: parameter 'abc' is invalid"
        );
    }

    let source = "begin push.1 add while.true mul";
    let program = assembler.compile(source);
    assert!(program.is_err());
    if let Err(error) = program {
        assert_eq!(error.to_string(), "while without matching end");
    }
}

// DUMMY LIBRARY
// ================================================================================================

struct DummyLibrary {
    namespace: LibraryNamespace,
    modules: Vec<Module>,
    dependencies: Vec<LibraryNamespace>,
}

impl DummyLibrary {
    fn new(namespace: LibraryNamespace, modules: Vec<Module>) -> Self {
        Self {
            namespace,
            modules,
            dependencies: Vec::new(),
        }
    }
}

impl Library for DummyLibrary {
    type ModuleIterator<'a> = Iter<'a, Module>;

    fn root_ns(&self) -> &LibraryNamespace {
        &self.namespace
    }

    fn version(&self) -> &Version {
        &Version::MIN
    }

    fn modules(&self) -> Self::ModuleIterator<'_> {
        self.modules.iter()
    }

    fn dependencies(&self) -> &[LibraryNamespace] {
        &self.dependencies
    }
}
