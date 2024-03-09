use alloc::{rc::Rc, string::ToString, vec::Vec};

use crate::{
    assert_diagnostic_lines,
    ast::{Module, ModuleKind},
    diagnostics::Report,
    regex, source_file,
    testing::{Pattern, TestContext},
    Assembler, AssemblyContext, Library, LibraryNamespace, LibraryPath, MaslLibrary, Version,
};

type TestResult = Result<(), Report>;

use pretty_assertions::{assert_eq, assert_str_eq};

macro_rules! assert_assembler_diagnostic {
    ($context:ident, $source:expr, $($expected:literal),+) => {{
        let error = $context
            .compile($source)
            .expect_err("expected diagnostic to be raised, but compilation succeeded");
        assert_diagnostic_lines!(error, $($expected),*);
    }};

    ($context:ident, $source:expr, $($expected:expr),+) => {{
        let error = $context
            .compile($source)
            .expect_err("expected diagnostic to be raised, but compilation succeeded");
        assert_diagnostic_lines!(error, $($expected),*);
    }};
}

// SIMPLE PROGRAMS
// ================================================================================================

#[test]
fn simple_instructions() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("begin push.0 assertz end");
    let program = context.compile(source)?;
    let expected = "\
begin
    span pad eqz assert(0) end
end";
    assert_str_eq!(format!("{program}"), expected);

    let source = source_file!("begin push.10 push.50 push.2 u32wrapping_madd end");
    let program = context.compile(source)?;
    let expected = "\
begin
    span push(10) push(50) push(2) u32madd drop end
end";
    assert_str_eq!(format!("{program}"), expected);

    let source = source_file!("begin push.10 push.50 push.2 u32wrapping_add3 end");
    let program = context.compile(source)?;
    let expected = "\
begin
    span push(10) push(50) push(2) u32add3 drop end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

/// TODO(pauls): Do we want to allow this in Miden Assembly?
#[test]
#[ignore]
fn empty_program() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("begin end");
    let program = context.compile(source)?;
    let expected = "begin span noop end end";
    assert_eq!(expected, format!("{}", program));
    Ok(())
}

/// TODO(pauls): Do we want to allow this in Miden Assembly
#[test]
#[ignore]
fn empty_if() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("begin if.true end end");
    let program = context.compile(source)?;
    let expected = "\
begin
    if.true
        span noop end
    else
        span noop end
    end
end";
    assert_str_eq!(format!("{}", program), expected);
    Ok(())
}

/// TODO(pauls): Do we want to allow this in Miden Assembly
#[test]
#[ignore]
fn empty_while() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("begin while.true end end");
    let program = context.compile(source)?;
    let expected = "\
begin
    while.true
        span noop end
    end
end";
    assert_str_eq!(format!("{}", program), expected);
    Ok(())
}

/// TODO(pauls): Do we want to allow this in Miden Assembly
#[test]
#[ignore]
fn empty_repeat() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("begin repeat.5 end end");
    let program = context.compile(source)?;
    let expected = "\
begin
    span noop noop noop noop noop end
end";
    assert_str_eq!(format!("{}", program), expected);
    Ok(())
}

#[test]
fn single_span() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("begin push.1 push.2 add end");
    let program = context.compile(source)?;
    let expected = "\
begin
    span pad incr push(2) add end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn span_and_simple_if() -> TestResult {
    let mut context = TestContext::default();

    // if with else
    let source = source_file!("begin push.2 push.3 if.true add else mul end end");
    let program = context.compile(source)?;
    let expected = "\
begin
    join
        span push(2) push(3) end
        if.true
            span add end
        else
            span mul end
        end
    end
end";
    assert_str_eq!(format!("{program}"), expected);

    // if without else
    let source = source_file!("begin push.2 push.3 if.true add end end");
    let program = context.compile(source)?;
    let expected = "\
begin
    join
        span push(2) push(3) end
        if.true
            span add end
        else
            span noop end
        end
    end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

// PROGRAM WITH #main CALL
// ================================================================================================

#[test]
fn simple_main_call() -> TestResult {
    let mut context = TestContext::default();

    // compile account module
    let account_path = LibraryPath::new("context::account").unwrap();
    let account_code = context.parse_module_with_path(
        account_path,
        source_file!(
            "\
        export.account_method_1
            push.2.1 add
        end

        export.account_method_2
            push.3.1 sub
        end
        "
        ),
    )?;

    context.add_module(account_code)?;

    // compile note 1 program
    context.compile(source_file!(
        "
        use.context::account
        begin
          call.account::account_method_1
        end
        "
    ))?;

    // compile note 2 program
    context.compile(source_file!(
        "
        use.context::account
        begin
          call.account::account_method_2
        end
        "
    ))?;
    Ok(())
}

#[test]
fn call_without_path() -> TestResult {
    let mut context = TestContext::default();
    // compile first module
    //context.add_module_from_source(
    context.compile_module_from_source(
        "account_code1".parse().unwrap(),
        source_file!(
            "\
    export.account_method_1
        push.2.1 add
    end

    export.account_method_2
        push.3.1 sub
    end
    "
        ),
    )?;

    //---------------------------------------------------------------------------------------------

    // compile second module
    //context.add_module_from_source(
    context.compile_module_from_source(
        "account_code2".parse().unwrap(),
        source_file!(
            "\
    export.account_method_1
        push.2.2 add
    end

    export.account_method_2
        push.4.1 sub
    end
    "
        ),
    )?;

    //---------------------------------------------------------------------------------------------

    // compile program in which functions from different modules but with equal names are called
    context.compile(source_file!(
        "
        begin
            # call the account_method_1 from the first module (account_code1)
            call.0x81e0b1afdbd431e4c9d4b86599b82c3852ecf507ae318b71c099cdeba0169068

            # call the account_method_2 from the first module (account_code1)
            call.0x1bc375fc794af6637af3f428286bf6ac1a24617640ed29f8bc533f48316c6d75

            # call the account_method_1 from the second module (account_code2)
            call.0xcfadd74886ea075d15826a4f59fb4db3a10cde6e6e953603cba96b4dcbb94321

            # call the account_method_2 from the second module (account_code2)
            call.0x1976bf72d457bd567036d3648b7e3f3c22eca4096936931e59796ec05c0ecb10
        end
        "
    ))?;
    Ok(())
}

// PROGRAM WITH PROCREF
// ================================================================================================

#[test]
fn procref_call() -> TestResult {
    let mut context = TestContext::default();
    // compile first module
    context.add_module_from_source(
        "module::path::one".parse().unwrap(),
        source_file!(
            "
        export.aaa
            push.7.8
        end

        export.foo
            push.1.2
        end"
        ),
    )?;

    // compile second module
    context.add_module_from_source(
        "module::path::two".parse().unwrap(),
        source_file!(
            "
        use.module::path::one
        export.one::foo

        export.bar
            procref.one::aaa
        end"
        ),
    )?;

    // compile program with procref calls
    context.compile(source_file!(
        "
        use.module::path::two

        proc.baz.4
            push.3.4
        end

        begin
            procref.two::bar
            procref.two::foo
            procref.baz
        end"
    ))?;
    Ok(())
}

#[test]
fn get_proc_name_of_unknown_module() -> TestResult {
    let mut context = TestContext::default();
    // Module `two` is unknown. This program should return
    // `AssemblyError::UndefinedProcedure`, referencing the
    // use of `bar`
    let module_source1 = source_file!(
        "
    use.module::path::two

    export.foo
        procref.two::bar
    end"
    );
    let module_path_one = "module::path::one".parse().unwrap();
    let module1 = context.parse_module_with_path(module_path_one, module_source1)?;

    let masl_lib =
        MaslLibrary::new(module1.namespace().clone(), Version::default(), [module1], vec![])
            .unwrap();

    // instantiate assembler
    context.add_library(&masl_lib)?;

    // compile program with procref calls
    let source = source_file!(
        "
        use.module::path::one

        begin
            procref.one::foo
        end"
    );
    assert_assembler_diagnostic!(
        context,
        source,
        "undefined module 'module::path::two'",
        regex!(r#",-\[test[\d]+:5:22\]"#),
        "4 |     export.foo",
        "5 |         procref.two::bar",
        "  :                      ^^^",
        "6 |     end",
        "  `----"
    );
    Ok(())
}

// CONSTANTS
// ================================================================================================

#[test]
fn simple_constant() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "\
    const.TEST_CONSTANT=7
    begin
        push.TEST_CONSTANT
    end"
    );
    let expected = "\
begin
    span push(7) end
end";
    let program = context.compile(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn multiple_constants_push() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "const.CONSTANT_1=21 \
    const.CONSTANT_2=44 \
    begin \
    push.CONSTANT_1.64.CONSTANT_2.72 \
    end"
    );
    let expected = "\
begin
    span push(21) push(64) push(44) push(72) end
end";
    let program = context.compile(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn constant_numeric_expression() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "const.TEST_CONSTANT=11-2+4*(12-(10+1))+9+8//4*2 \
    begin \
    push.TEST_CONSTANT \
    end \
    "
    );
    let expected = "\
begin
    span push(26) end
end";
    let program = context.compile(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn constant_alphanumeric_expression() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "const.TEST_CONSTANT_1=(18-1+10)*6-((13+7)*2) \
    const.TEST_CONSTANT_2=11-2+4*(12-(10+1))+9
    const.TEST_CONSTANT_3=(TEST_CONSTANT_1-(TEST_CONSTANT_2+10))//5+3
    begin \
    push.TEST_CONSTANT_3 \
    end \
    "
    );
    let expected = "\
begin
    span push(21) end
end";
    let program = context.compile(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn constant_hexadecimal_value() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "const.TEST_CONSTANT=0xFF \
    begin \
    push.TEST_CONSTANT \
    end \
    "
    );
    let expected = "\
begin
    span push(255) end
end";
    let program = context.compile(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn constant_field_division() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "const.TEST_CONSTANT=(17//4)/4*(1//2)+2 \
    begin \
    push.TEST_CONSTANT \
    end \
    "
    );
    let expected = "\
begin
    span push(2) end
end";
    let program = context.compile(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn constant_err_const_not_initialized() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "const.TEST_CONSTANT=5+A \
    begin \
    push.TEST_CONSTANT \
    end"
    );
    assert_assembler_diagnostic!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "symbol undefined: no such name found in scope",
        regex!(r#",-\[test[\d]+:1:23\]"#),
        "1 | const.TEST_CONSTANT=5+A begin push.TEST_CONSTANT end",
        "  :                       ^",
        "  `----"
    );
    Ok(())
}

#[test]
fn constant_err_div_by_zero() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "const.TEST_CONSTANT=5/0 \
    begin \
    push.TEST_CONSTANT \
    end"
    );
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid constant expression: division by zero",
        regex!(r#",-\[test[\d]+:1:21\]"#),
        "1 | const.TEST_CONSTANT=5/0 begin push.TEST_CONSTANT end",
        "  :                     ^^^",
        "  `----"
    );

    let source = source_file!(
        "const.TEST_CONSTANT=5//0 \
    begin \
    push.TEST_CONSTANT \
    end"
    );
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid constant expression: division by zero",
        regex!(r#",-\[test[\d]+:1:21\]"#),
        "1 | const.TEST_CONSTANT=5//0 begin push.TEST_CONSTANT end",
        "  :                     ^^^^",
        "  `----"
    );
    Ok(())
}

#[test]
fn constants_must_be_uppercase() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "const.constant_1=12 \
    begin \
    push.constant_1 \
    end"
    );

    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:7\]"#),
        "1 | const.constant_1=12 begin push.constant_1 end",
        "  :       ^^^^^|^^^^",
        "  :            `-- found a identifier here",
        "  `----"
    );

    Ok(())
}

#[test]
fn duplicate_constant_name() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "const.CONSTANT=12 \
    const.CONSTANT=14 \
    begin \
    push.CONSTANT \
    end"
    );

    assert_assembler_diagnostic!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "symbol conflict: found duplicate definitions of the same name",
        regex!(r#",-\[test[\d]+:1:1\]"#),
        "1 | const.CONSTANT=12 const.CONSTANT=14 begin push.CONSTANT end",
        "  : ^^^^^^^^|^^^^^^^^ ^^^^^^^^|^^^^^^^^",
        "  :         |                 `-- conflict occurs here",
        "  :         `-- previously defined here",
        "  `----"
    );
    Ok(())
}

#[test]
fn constant_must_be_valid_felt() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "const.CONSTANT=1122INVALID \
    begin \
    push.CONSTANT \
    end"
    );

    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:20\]"#),
        "1 | const.CONSTANT=1122INVALID begin push.CONSTANT end",
        "  :                    ^^^|^^^",
        "  :                       `-- found a constant identifier here",
        "  `----",
        " help: expected \"*\", or \"+\", or \"-\", or \"/\", or \"//\", or \"begin\", or \"const\", \
or \"export\", or \"proc\", or \"use\", or end of file, or doc comment"
    );
    Ok(())
}

#[test]
fn constant_must_be_within_valid_felt_range() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "const.CONSTANT=18446744073709551615 \
    begin \
    push.CONSTANT \
    end"
    );

    assert_assembler_diagnostic!(
        context,
        source,
        "invalid literal: value overflowed the field modulus",
        regex!(r#",-\[test[\d]+:1:16\]"#),
        "1 | const.CONSTANT=18446744073709551615 begin push.CONSTANT end",
        "  :                ^^^^^^^^^^^^^^^^^^^^",
        "  `----"
    );
    Ok(())
}

#[test]
fn constants_defined_in_global_scope() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "
    begin \
    const.CONSTANT=12
    push.CONSTANT \
    end"
    );

    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:2:11\]"#),
        "1 |",
        "2 |     begin const.CONSTANT=12",
        "  :           ^^|^^",
        "  :             `-- found a const here",
        "3 |     push.CONSTANT end",
        "  `----",
        r#" help: expected primitive opcode (e.g. "add"), or control flow opcode (e.g. "if.true")"#
    );
    Ok(())
}

#[test]
fn constant_not_found() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "
    begin \
    push.CONSTANT \
    end"
    );

    assert_assembler_diagnostic!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "symbol undefined: no such name found in scope",
        regex!(r#",-\[test[\d]+:2:16\]"#),
        "1 |",
        "2 |     begin push.CONSTANT end",
        "  :                ^^^^^^^^",
        "  `----"
    );
    Ok(())
}

#[test]
fn mem_operations_with_constants() -> TestResult {
    let mut context = TestContext::default();

    // Define constant values
    const PROC_LOC_STORE_PTR: u64 = 0;
    const PROC_LOC_LOAD_PTR: u64 = 1;
    const PROC_LOC_STOREW_PTR: u64 = 2;
    const PROC_LOC_LOADW_PTR: u64 = 3;
    const GLOBAL_STORE_PTR: u64 = 4;
    const GLOBAL_LOAD_PTR: u64 = 5;
    const GLOBAL_STOREW_PTR: u64 = 6;
    const GLOBAL_LOADW_PTR: u64 = 7;

    let source = source_file!(format!(
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
    ));
    let program = context.compile(source)?;

    // Define expected
    let expected = source_file!(format!(
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
    ));
    let expected_program = context.compile(expected)?;
    assert_str_eq!(expected_program.to_string(), program.to_string());
    Ok(())
}

#[test]
fn const_conversion_failed_to_u16() -> TestResult {
    // Define constant value greater than u16::MAX
    let constant_value: u64 = u16::MAX as u64 + 1;

    let source = source_file!(format!(
        "\
    const.CONSTANT={constant_value}

    proc.test_constant_overflow.1
        loc_load.CONSTANT
    end

    begin
        exec.test_constant_overflow
    end
    "
    ));
    let mut context = TestContext::default();

    assert_assembler_diagnostic!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "invalid immediate: value is larger than expected range",
        regex!(r#",-\[test[\d]+:4:18\]"#),
        "3 |     proc.test_constant_overflow.1",
        "4 |         loc_load.CONSTANT",
        "  :                  ^^^^^^^^",
        "5 |     end",
        "  `----"
    );
    Ok(())
}

#[test]
fn const_conversion_failed_to_u32() -> TestResult {
    let mut context = TestContext::default();
    // Define constant value greater than u16::MAX
    let constant_value: u64 = u32::MAX as u64 + 1;

    let source = source_file!(format!(
        "\
    const.CONSTANT={constant_value}

    begin
        mem_load.CONSTANT
    end
    "
    ));

    assert_assembler_diagnostic!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "invalid immediate: value is larger than expected range",
        regex!(r#",-\[test[\d]+:4:18\]"#),
        "3 |     begin",
        "4 |         mem_load.CONSTANT",
        "  :                  ^^^^^^^^",
        "5 |     end",
        "  `----"
    );
    Ok(())
}

// ASSERTIONS
// ================================================================================================

#[test]
fn assert_with_code() -> TestResult {
    let source = source_file!(
        "\
    const.ERR1=1

    begin
        assert
        assert.err=ERR1
        assert.err=2
    end
    "
    );
    let mut context = TestContext::default();
    let program = context.compile(source)?;

    let expected = "\
begin
    span assert(0) assert(1) assert(2) end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn assertz_with_code() -> TestResult {
    let source = source_file!(
        "\
    const.ERR1=1

    begin
        assertz
        assertz.err=ERR1
        assertz.err=2
    end
    "
    );
    let mut context = TestContext::default();
    let program = context.compile(source)?;

    let expected = "\
begin
    span eqz assert(0) eqz assert(1) eqz assert(2) end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn assert_eq_with_code() -> TestResult {
    let source = source_file!(
        "\
    const.ERR1=1

    begin
        assert_eq
        assert_eq.err=ERR1
        assert_eq.err=2
    end
    "
    );
    let mut context = TestContext::default();
    let program = context.compile(source)?;

    let expected = "\
begin
    span eq assert(0) eq assert(1) eq assert(2) end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn assert_eqw_with_code() -> TestResult {
    let source = source_file!(
        "\
    const.ERR1=1

    begin
        assert_eqw
        assert_eqw.err=ERR1
        assert_eqw.err=2
    end
    "
    );
    let mut context = TestContext::default();
    let program = context.compile(source)?;

    let expected = "\
begin
    span
        movup4
        eq
        assert(0)
        movup3
        eq
        assert(0)
        movup2
        eq
        assert(0)
        eq
        assert(0)
        movup4
        eq
        assert(1)
        movup3
        eq
        assert(1)
        movup2
        eq
        assert(1)
        eq
        assert(1)
        movup4
        eq
        assert(2)
        movup3
        eq
        assert(2)
        movup2
        eq
        assert(2)
        eq
        assert(2)
    end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn u32assert_with_code() -> TestResult {
    let source = source_file!(
        "\
    const.ERR1=1

    begin
        u32assert
        u32assert.err=ERR1
        u32assert.err=2
    end
    "
    );
    let mut context = TestContext::default();
    let program = context.compile(source)?;

    let expected = "\
begin
    span
        pad
        u32assert2(0)
        drop
        pad
        u32assert2(1)
        drop
        pad
        u32assert2(2)
        drop
    end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn u32assert2_with_code() -> TestResult {
    let source = source_file!(
        "\
    const.ERR1=1

    begin
        u32assert2
        u32assert2.err=ERR1
        u32assert2.err=2
    end
    "
    );
    let mut context = TestContext::default();
    let program = context.compile(source)?;

    let expected = "\
begin
    span u32assert2(0) u32assert2(1) u32assert2(2) end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn u32assertw_with_code() -> TestResult {
    let source = source_file!(
        "\
    const.ERR1=1

    begin
        u32assertw
        u32assertw.err=ERR1
        u32assertw.err=2
    end
    "
    );
    let mut context = TestContext::default();
    let program = context.compile(source)?;

    let expected = "\
begin
    span
        u32assert2(0)
        movup3
        movup3
        u32assert2(0)
        movup3
        movup3
        u32assert2(1)
        movup3
        movup3
        u32assert2(1)
        movup3
        movup3
        u32assert2(2)
        movup3
        movup3
        u32assert2(2)
        movup3
        movup3
    end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

// NESTED CONTROL BLOCKS
// ================================================================================================

#[test]
fn nested_control_blocks() -> TestResult {
    let mut context = TestContext::default();

    // if with else
    let source = source_file!(
        "begin \
        push.2 push.3 \
        if.true \
            add while.true push.7 push.11 add end \
        else \
            mul repeat.2 push.8 end if.true mul end  \
        end
        push.3 add
        end"
    );
    let program = context.compile(source)?;
    let expected = "\
begin
    join
        join
            span push(2) push(3) end
            if.true
                join
                    span add end
                    while.true
                        span push(7) push(11) add end
                    end
                end
            else
                join
                    span mul push(8) push(8) end
                    if.true
                        span mul end
                    else
                        span noop end
                    end
                end
            end
        end
        span push(3) add end
    end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

// PROGRAMS WITH PROCEDURES
// ================================================================================================

#[test]
fn program_with_one_procedure() -> TestResult {
    let mut context = TestContext::default();
    let source =
        source_file!("proc.foo push.3 push.7 mul end begin push.2 push.3 add exec.foo end");
    let program = context.compile(source)?;
    let foo = context.display_digest_from_cache(&"#exec::foo".parse().unwrap());
    let expected = format!(
        "\
begin
    join
        span push(2) push(3) add end
        proxy.{foo}
    end
end"
    );
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

// TODO(pauls): Do we want to support this in the surface MASM syntax?
#[test]
#[ignore]
fn program_with_one_empty_procedure() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("proc.foo end begin exec.foo end");
    let program = context.compile(source)?;
    let foo = context.display_digest_from_cache(&"#exec::foo".parse().unwrap());
    let expected = format!(
        "\
begin
    proxy.{foo}
end"
    );
    assert_str_eq!(format!("{}", program), expected);
    Ok(())
}

#[test]
fn program_with_nested_procedure() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "\
        proc.foo push.3 push.7 mul end \
        proc.bar push.5 exec.foo add end \
        begin push.2 push.4 add exec.foo push.11 exec.bar sub end"
    );
    let program = context.compile(source)?;
    let foo = context.display_digest_from_cache(&"#exec::foo".parse().unwrap());
    let bar = context.display_digest_from_cache(&"#exec::bar".parse().unwrap());
    let expected = format!(
        "\
begin
    join
        join
            join
                span push(2) push(4) add end
                proxy.{foo}
            end
            join
                span push(11) end
                proxy.{bar}
            end
        end
        span neg add end
    end
end"
    );
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn program_with_proc_locals() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "\
        proc.foo.1 \
            loc_store.0 \
            add \
            loc_load.0 \
            mul \
        end \
        begin \
            push.4 push.3 push.2 \
            exec.foo \
        end"
    );
    let program = context.compile(source)?;
    let foo = context.display_digest_from_cache(&"#exec::foo".parse().unwrap());
    let expected = format!(
        "\
begin
    join
        span push(4) push(3) push(2) end
        proxy.{foo}
    end
end"
    );
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn program_with_exported_procedure() -> TestResult {
    let mut context = TestContext::default();
    let source =
        source_file!("export.foo push.3 push.7 mul end begin push.2 push.3 add exec.foo end");

    assert_assembler_diagnostic!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "invalid program: procedure exports are not allowed",
        regex!(r#",-\[test[\d]+:1:1\]"#),
        "1 | export.foo push.3 push.7 mul end begin push.2 push.3 add exec.foo end",
        "  : ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^",
        "  `----"
    );
    Ok(())
}

// PROGRAMS WITH DYNAMIC CODE BLOCKS
// ================================================================================================

#[test]
fn program_with_dynamic_code_execution() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("begin dynexec end");
    let program = context.compile(source)?;
    let expected = "\
begin
    dyn
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn program_with_dynamic_code_execution_in_new_context() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("begin dyncall end");
    let program = context.compile(source)?;
    let expected = "\
begin
    call.0xc75c340ec6a69e708457544d38783abbb604d881b7dc62d00bfc2b10f52808e6
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

// MAST ROOT CALLS
// ================================================================================================

#[test]
fn program_with_incorrect_mast_root_length() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("begin call.0x1234 end");

    assert_assembler_diagnostic!(
        context,
        source,
        "invalid MAST root literal",
        regex!(r#",-\[test[\d]+:1:12\]"#),
        "1 | begin call.0x1234 end",
        "  :            ^^^^^^",
        "  `----"
    );
    Ok(())
}

#[test]
fn program_with_invalid_mast_root_chars() {
    let mut context = TestContext::default();
    let source = source_file!(
        "begin call.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a21xyzb end"
    );

    assert_assembler_diagnostic!(
        context,
        source,
        "invalid literal: expected 2, 4, 8, 16, or 64 hex digits",
        regex!(r#",-\[test[\d]+:1:12\]"#),
        "1 | begin call.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a21xyzb end",
        "  :            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^",
        "  `----"
    );
}

#[test]
fn program_with_invalid_rpo_digest_call() {
    let mut context = TestContext::default();
    let source = source_file!(
        "begin call.0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff end"
    );

    assert_assembler_diagnostic!(
        context,
        source,
        "invalid literal: value overflowed the field modulus",
        regex!(r#",-\[test[\d]+:1:12\]"#),
        "1 | begin call.0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff end",
        "  :            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^",
        "  `----"
    );
}

#[test]
fn program_with_phantom_mast_call() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(
        "begin call.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a213dae end"
    );
    let ast = context.parse_program(source)?;

    // phantom calls not allowed
    let mut assembler = Assembler::default().with_debug_mode(true);

    let mut context = AssemblyContext::for_program(ast.path()).with_phantom_calls(false);
    let err = assembler
        .compile_in_context(ast.clone(), &mut context)
        .expect_err("expected compilation to fail with phantom calls");
    assert_diagnostic_lines!(
        err,
        "cannot call phantom procedure: phantom calls are disabled",
        regex!(r#",-\[test[\d]+:1:12\]"#),
        "1 | begin call.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a213dae end",
        "  :            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^",
        "  :                                             `-- the procedure referenced here is not available",
        "  `----",
        " help: mast root is 0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a213dae"
    );

    // phantom calls allowed
    let mut context = AssemblyContext::for_program(ast.path()).with_phantom_calls(true);
    assembler.compile_in_context(ast, &mut context)?;
    Ok(())
}

// IMPORTS
// ================================================================================================

#[test]
fn program_with_one_import_and_hex_call() -> TestResult {
    const MODULE: &str = "dummy::math::u256";
    const PROCEDURE: &str = r#"
        export.iszero_unsafe
            eq.0
            repeat.7
                swap
                eq.0
                and
            end
        end"#;

    let mut context = TestContext::default();
    let path = MODULE.parse().unwrap();
    let ast = context.parse_module_with_path(path, source_file!(PROCEDURE.to_string()))?;
    let ns = ast.namespace().clone();
    let library = DummyLibrary::new(ns, vec![Rc::from(ast)]);

    context.add_library(&library)?;

    let source = source_file!(format!(
        r#"
        use.{MODULE}
        begin
            push.4 push.3
            exec.u256::iszero_unsafe
            call.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a213dae
        end"#
    ));
    let program = context.compile(source)?;

    let iszero_unsafe =
        context.display_digest_from_cache(&"dummy::math::u256::iszero_unsafe".parse().unwrap());
    let expected = format!(
        "\
begin
    join
        join
            span push(4) push(3) end
            proxy.{iszero_unsafe}
        end
        call.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a213dae
    end
end"
    );
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn program_with_two_imported_procs_with_same_mast_root() -> TestResult {
    const MODULE: &str = "dummy::math::u256";
    const PROCEDURE: &str = r#"
        export.iszero_unsafe_dup
            eq.0
            repeat.7
                swap
                eq.0
                and
            end
        end

        export.iszero_unsafe
            eq.0
            repeat.7
                swap
                eq.0
                and
            end
        end"#;

    let mut context = TestContext::default();
    let path = MODULE.parse().unwrap();
    let ast = context.parse_module_with_path(path, source_file!(PROCEDURE.to_string()))?;
    let ns = ast.namespace().clone();
    let library = DummyLibrary::new(ns, vec![Rc::from(ast)]);

    context.add_library(&library)?;

    let source = source_file!(format!(
        r#"
        use.{MODULE}
        begin
            push.4 push.3
            exec.u256::iszero_unsafe
            exec.u256::iszero_unsafe_dup
        end"#
    ));
    context.compile(source)?;
    Ok(())
}

#[test]
fn program_with_reexported_proc_in_same_library() -> TestResult {
    // exprted proc is in same library
    const REF_MODULE: &str = "dummy1::math::u64";
    const REF_MODULE_BODY: &str = r#"
        export.checked_eqz
            u32assert2
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

    const MODULE: &str = "dummy1::math::u256";
    const MODULE_BODY: &str = r#"
        use.dummy1::math::u64

        #! checked_eqz checks if the value is u32 and zero and returns 1 if it is, 0 otherwise
        export.u64::checked_eqz # re-export

        #! unchecked_eqz checks if the value is zero and returns 1 if it is, 0 otherwise
        export.u64::unchecked_eqz->notchecked_eqz # re-export with alias
    "#;

    let mut context = TestContext::new();
    let ast = Module::parse_str(MODULE.parse().unwrap(), ModuleKind::Library, MODULE_BODY).unwrap();

    // check docs
    let docs_checked_eqz =
        ast.procedures().find(|p| p.name() == "checked_eqz").unwrap().docs().unwrap();
    assert_eq!(
        docs_checked_eqz,
        "checked_eqz checks if the value is u32 and zero and returns 1 if it is, 0 otherwise\n"
    );
    let docs_unchecked_eqz =
        ast.procedures().find(|p| p.name() == "notchecked_eqz").unwrap().docs().unwrap();
    assert_eq!(
        docs_unchecked_eqz,
        "unchecked_eqz checks if the value is zero and returns 1 if it is, 0 otherwise\n"
    );

    let ref_ast =
        Module::parse_str(REF_MODULE.parse().unwrap(), ModuleKind::Library, REF_MODULE_BODY)
            .unwrap();

    let ns = ref_ast.namespace().clone();
    let library = DummyLibrary::new(ns, vec![Rc::from(ast), Rc::from(ref_ast)]);

    context.add_library(&library)?;

    let source = source_file!(format!(
        r#"
        use.{MODULE}
        begin
            push.4 push.3
            exec.u256::checked_eqz
            exec.u256::notchecked_eqz
        end"#
    ));
    let program = context.compile(source)?;
    let checked_eqz =
        context.display_digest_from_cache(&"dummy1::math::u64::checked_eqz".parse().unwrap());
    let notchecked_eqz =
        context.display_digest_from_cache(&"dummy1::math::u64::unchecked_eqz".parse().unwrap());
    let expected = format!(
        "\
begin
    join
        join
            span push(4) push(3) end
            proxy.{checked_eqz}
        end
        proxy.{notchecked_eqz}
    end
end"
    );
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn program_with_reexported_proc_in_another_library() -> TestResult {
    // when re-exported proc is part of a different library
    const REF_MODULE: &str = "dummy2::math::u64";
    const REF_MODULE_BODY: &str = r#"
        export.checked_eqz
            u32assert2
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

    const MODULE: &str = "dummy1::math::u256";
    const MODULE_BODY: &str = r#"
        use.dummy2::math::u64
        export.u64::checked_eqz # re-export
        export.u64::unchecked_eqz->notchecked_eqz # re-export with alias
    "#;

    let mut context = TestContext::default();
    let ast = Module::parse_str(MODULE.parse().unwrap(), ModuleKind::Library, MODULE_BODY).unwrap();
    let ns = ast.namespace().clone();
    let dummy_library_1 = DummyLibrary::new(ns, vec![Rc::from(ast)]);

    let ref_ast =
        Module::parse_str(REF_MODULE.parse().unwrap(), ModuleKind::Library, REF_MODULE_BODY)
            .unwrap();
    let ns = ref_ast.namespace().clone();
    let dummy_library_2 = DummyLibrary::new(ns, vec![Rc::from(ref_ast)]);

    context.add_library(&dummy_library_1)?;
    context.add_library(&dummy_library_2)?;

    let source = source_file!(format!(
        r#"
        use.{MODULE}
        begin
            push.4 push.3
            exec.u256::checked_eqz
            exec.u256::notchecked_eqz
        end"#
    ));
    let program = context.compile(source)?;

    let checked_eqz =
        context.display_digest_from_cache(&"dummy2::math::u64::checked_eqz".parse().unwrap());
    let notchecked_eqz =
        context.display_digest_from_cache(&"dummy2::math::u64::unchecked_eqz".parse().unwrap());
    let expected = format!(
        "\
begin
    join
        join
            span push(4) push(3) end
            proxy.{checked_eqz}
        end
        proxy.{notchecked_eqz}
    end
end"
    );
    assert_str_eq!(format!("{program}"), expected);

    // when the re-exported proc is part of a different library and the library is not passed to
    // the assembler it should fail
    let mut context = TestContext::default();
    context.add_library(&dummy_library_1)?;
    let source = source_file!(format!(
        r#"
        use.{MODULE}
        begin
            push.4 push.3
            exec.u256::checked_eqz
            exec.u256::notchecked_eqz
        end"#
    ));
    assert_assembler_diagnostic!(context, source, "undefined module 'dummy2::math::u64'");
    Ok(())
}

#[test]
fn module_alias() -> TestResult {
    const MODULE: &str = "dummy::math::u64";
    const PROCEDURE: &str = r#"
        export.checked_add
            swap
            movup.3
            u32assert2
            u32overflowing_add
            movup.3
            movup.3
            u32assert2
            u32overflowing_add3
            eq.0
            assert
        end"#;

    let mut context = TestContext::default();
    let ast = Module::parse_str(MODULE.parse().unwrap(), ModuleKind::Library, PROCEDURE).unwrap();
    let ns = ast.namespace().clone();
    let library = DummyLibrary::new(ns, vec![Rc::from(ast)]);

    context.add_library(&library)?;

    let source = "
        use.dummy::math::u64->bigint

        begin
            push.1.0
            push.2.0
            exec.bigint::checked_add
        end";
    let ast =
        Module::parse_str(LibraryNamespace::Exec.into(), ModuleKind::Executable, source).unwrap();

    let program = context.compile_ast(ast)?;
    let checked_add =
        context.display_digest_from_cache(&"dummy::math::u64::checked_add".parse().unwrap());
    let expected = format!(
        "\
begin
    join
        span pad incr pad push(2) pad end
        proxy.{checked_add}
    end
end"
    );
    assert_str_eq!(format!("{program}"), expected);

    // --- invalid module alias -----------------------------------------------
    let source = source_file!(
        "
        use.dummy::math::u64->bigint->invalidname

        begin
            push.1.0
            push.2.0
            exec.bigint->invalidname::checked_add
        end"
    );
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:2:37\]"#),
        "1 |",
        "2 |         use.dummy::math::u64->bigint->invalidname",
        "  :                                     ^|",
        "  :                                      `-- found a -> here",
        "3 |",
        "  `----",
        r#" help: expected "begin", or "const", or "export", or "proc", or "use", or end of file, or doc comment"#
    );

    // --- duplicate module import --------------------------------------------
    let source = source_file!(
        "
        use.dummy::math::u64
        use.dummy::math::u64->bigint

        begin
            push.1.0
            push.2.0
            exec.bigint::checked_add
        end"
    );

    assert_assembler_diagnostic!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "unused import",
        regex!(r#",-\[test[\d]+:2:9\]"#),
        "1 |",
        "2 |         use.dummy::math::u64",
        "  :         ^^^^^^^^^^^^^^^^^^^^",
        "3 |         use.dummy::math::u64->bigint",
        "  `----",
        " help: this import is never used and can be safely removed"
    );

    // --- duplicate module imports with different aliases --------------------
    // TODO: Do we actually want this to be a warning/error? If the imports
    // have different aliases, there might be some use for that when refactoring
    // code or something. Anyway, I'm disabling the test that expects this to
    // fail for the time being
    /*
    let source = source_file!(
        "
        use.dummy::math::u64->bigint
        use.dummy::math::u64->bigint2

        begin
            push.1.0
            push.2.0
            exec.bigint::checked_add
            exec.bigint2::checked_add
        end"
    );
    */
    Ok(())
}

#[test]
fn program_with_import_errors() {
    let mut context = TestContext::default();
    // --- non-existent import ------------------------------------------------
    let source = source_file!(
        "\
        use.std::math::u512
        begin \
            push.4 push.3 \
            exec.u512::iszero_unsafe \
        end"
    );

    assert_assembler_diagnostic!(
        context,
        source,
        "undefined module 'std::math::u512'",
        regex!(r#",-\[test[\d]+:2:40\]"#),
        "1 | use.std::math::u512",
        "2 |         begin push.4 push.3 exec.u512::iszero_unsafe end",
        "  :                                        ^^^^^^^^^^^^^",
        "  `----"
    );

    // --- non-existent procedure in import -----------------------------------
    let source = source_file!(
        "\
        use.std::math::u256
        begin \
            push.4 push.3 \
            exec.u256::foo \
        end"
    );

    assert_assembler_diagnostic!(
        context,
        source,
        "undefined module 'std::math::u256'",
        regex!(r#",-\[test[\d]+:2:40\]"#),
        "1 | use.std::math::u256",
        "2 |         begin push.4 push.3 exec.u256::foo end",
        "  :                                        ^^^",
        "  `----"
    );
}

// COMMENTS
// ================================================================================================

#[test]
fn comment_simple() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("begin # simple comment \n push.1 push.2 add end");
    let program = context.compile(source)?;
    let expected = "\
begin
    span pad incr push(2) add end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn comment_in_nested_control_blocks() -> TestResult {
    let mut context = TestContext::default();

    // if with else
    let source = source_file!(
        "begin \
        push.1 push.2 \
        if.true \
            # nested comment \n\
            add while.true push.7 push.11 add end \
        else \
            mul repeat.2 push.8 end if.true mul end  \
            # nested comment \n\
        end
        push.3 add
        end"
    );
    let program = context.compile(source)?;
    let expected = "\
begin
    join
        join
            span pad incr push(2) end
            if.true
                join
                    span add end
                    while.true
                        span push(7) push(11) add end
                    end
                end
            else
                join
                    span mul push(8) push(8) end
                    if.true
                        span mul end
                    else
                        span noop end
                    end
                end
            end
        end
        span push(3) add end
    end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn comment_before_program() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!(" # starting comment \n begin push.1 push.2 add end");
    let program = context.compile(source)?;
    let expected = "\
begin
    span pad incr push(2) add end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn comment_after_program() -> TestResult {
    let mut context = TestContext::default();
    let source = source_file!("begin push.1 push.2 add end # closing comment");
    let program = context.compile(source)?;
    let expected = "\
begin
    span pad incr push(2) add end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

// ERRORS
// ================================================================================================

#[test]
fn invalid_empty_program() {
    let mut context = TestContext::default();
    assert_assembler_diagnostic!(
        context,
        source_file!(""),
        "unexpected end of file",
        regex!(r#",-\[test[\d]+:1:1\]"#),
        "`----",
        r#" help: expected "begin", or "const", or "export", or "proc", or "use", or doc comment"#
    );

    assert_assembler_diagnostic!(
        context,
        source_file!(""),
        "unexpected end of file",
        regex!(r#",-\[test[\d]+:1:1\]"#),
        "  `----",
        r#" help: expected "begin", or "const", or "export", or "proc", or "use", or doc comment"#
    );
}

#[test]
fn invalid_program_unrecognized_token() {
    let mut context = TestContext::default();
    assert_assembler_diagnostic!(
        context,
        source_file!("none"),
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:1\]"#),
        "1 | none",
        "  : ^^|^",
        "  :   `-- found a identifier here",
        "  `----",
        r#" help: expected "begin", or "const", or "export", or "proc", or "use", or doc comment"#
    );
}

#[test]
fn invalid_program_unmatched_begin() {
    let mut context = TestContext::default();
    assert_assembler_diagnostic!(
        context,
        source_file!("begin add"),
        "unexpected end of file",
        regex!(r#",-\[test[\d]+:1:10\]"#),
        "1 | begin add",
        "  `----",
        r#" help: expected ".", or primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );
}

#[test]
fn invalid_program_invalid_top_level_token() {
    let mut context = TestContext::default();
    assert_assembler_diagnostic!(
        context,
        source_file!("begin add end mul"),
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:15\]"#),
        "1 | begin add end mul",
        "  :               ^|^",
        "  :                `-- found a mul here",
        "  `----",
        r#" help: expected "begin", or "const", or "export", or "proc", or "use", or end of file, or doc comment"#
    );
}

#[test]
fn invalid_proc_missing_end_unexpected_begin() {
    let mut context = TestContext::default();
    let source = source_file!("proc.foo add mul begin push.1 end");
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:18\]"#),
        "1 | proc.foo add mul begin push.1 end",
        "  :                  ^^|^^",
        "  :                    `-- found a begin here",
        "  `----",
        r#" help: expected ".", or primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );
}

#[test]
fn invalid_proc_missing_end_unexpected_proc() {
    let mut context = TestContext::default();
    let source = source_file!("proc.foo add mul proc.bar push.3 end begin push.1 end");
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:18\]"#),
        "1 | proc.foo add mul proc.bar push.3 end begin push.1 end",
        "  :                  ^^|^",
        "  :                    `-- found a proc here",
        "  `----",
        r#" help: expected ".", or primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );
}

#[test]
fn invalid_proc_undefined_local() {
    let mut context = TestContext::default();
    let source = source_file!("proc.foo add mul end begin push.1 exec.bar end");
    assert_assembler_diagnostic!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "symbol undefined: no such name found in scope",
        regex!(r#",-\[test[\d]+:1:40\]"#),
        "1 | proc.foo add mul end begin push.1 exec.bar end",
        "  :                                        ^^^",
        "  `----",
        " help: are you missing an import?"
    );
}

#[test]
fn invalid_proc_invalid_numeric_name() {
    let mut context = TestContext::default();
    let source = source_file!("proc.123 add mul end begin push.1 exec.123 end");
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:6\]"#),
        "1 | proc.123 add mul end begin push.1 exec.123 end",
        "  :      ^|^",
        "  :       `-- found a integer here",
        "  `----",
        " help: expected",
        "identifier, or quoted identifier"
    );
}

#[test]
fn invalid_proc_duplicate_procedure_name() {
    let mut context = TestContext::default();
    let source = source_file!("proc.foo add mul end proc.foo push.3 end begin push.1 end");
    assert_assembler_diagnostic!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "symbol conflict: found duplicate definitions of the same name",
        regex!(r#",-\[test[\d]+:1:6\]"#),
        "1 | proc.foo add mul end proc.foo push.3 end begin push.1 end",
        "  :      ^|^             ^^^^^^^^^|^^^^^^^^^",
        "  :       |                       `-- conflict occurs here",
        "  :       `-- previously defined here",
        "  `----"
    );
}

#[test]
fn invalid_if_missing_end_no_else() {
    let mut context = TestContext::default();
    let source = source_file!("begin push.1 add if.true mul");
    assert_assembler_diagnostic!(
        context,
        source,
        "unexpected end of file",
        regex!(r#",-\[test[\d]+:1:29\]"#),
        "1 | begin push.1 add if.true mul",
        "  `----",
        r#" help: expected ".", or primitive opcode (e.g. "add"), or "else", or "end", or control flow opcode (e.g. "if.true")"#
    );
}

#[test]
fn invalid_else_with_no_if() {
    let mut context = TestContext::default();
    let source = source_file!("begin push.1 add else mul end");
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:18\]"#),
        "1 | begin push.1 add else mul end",
        "  :                  ^^|^",
        "  :                    `-- found a else here",
        "  `----",
        r#" help: expected primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );

    let source = source_file!("begin push.1 while.true add else mul end end");
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:29\]"#),
        "1 | begin push.1 while.true add else mul end end",
        "  :                             ^^|^",
        "  :                               `-- found a else here",
        "  `----",
        r#" help: expected "end""#
    );
}

#[test]
fn invalid_unmatched_else_within_if_else() {
    let mut context = TestContext::default();

    let source = source_file!("begin push.1 if.true add else mul else push.1 end end end");
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:35\]"#),
        "1 | begin push.1 if.true add else mul else push.1 end end end",
        "  :                                   ^^|^",
        "  :                                     `-- found a else here",
        "  `----",
        r#" help: expected "end""#
    );
}

#[test]
fn invalid_if_else_no_matching_end() {
    let mut context = TestContext::default();

    let source = source_file!("begin push.1 add if.true mul else add");
    assert_assembler_diagnostic!(
        context,
        source,
        "unexpected end of file",
        regex!(r#",-\[test[\d]+:1:38\]"#),
        "1 | begin push.1 add if.true mul else add",
        "  `----",
        r#" help: expected ".", or primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );
}

#[test]
fn invalid_repeat() -> TestResult {
    let mut context = TestContext::default();

    // unmatched repeat
    let source = source_file!("begin push.1 add repeat.10 mul");
    assert_assembler_diagnostic!(
        context,
        source,
        "unexpected end of file",
        regex!(r#",-\[test[\d]+:1:31\]"#),
        "1 | begin push.1 add repeat.10 mul",
        "  `----",
        r#" help: expected ".", or primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );

    // invalid iter count
    let source = source_file!("begin push.1 add repeat.23x3 mul end end");
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:27\]"#),
        "1 | begin push.1 add repeat.23x3 mul end end",
        "  :                           ^|",
        "  :                            `-- found a identifier here",
        "  `----",
        r#" help: expected primitive opcode (e.g. "add"), or control flow opcode (e.g. "if.true")"#
    );

    Ok(())
}

#[test]
fn invalid_while() -> TestResult {
    let mut context = TestContext::default();

    let source = source_file!("begin push.1 add while mul end end");
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:24\]"#),
        "1 | begin push.1 add while mul end end",
        "  :                        ^|^",
        "  :                         `-- found a mul here",
        "  `----",
        r#" help: expected ".""#
    );

    let source = source_file!("begin push.1 add while.abc mul end end");
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:24\]"#),
        "1 | begin push.1 add while.abc mul end end",
        "  :                        ^|^",
        "  :                         `-- found a identifier here",
        "  `----",
        r#" help: expected "true""#
    );

    let source = source_file!("begin push.1 add while.true mul");
    assert_assembler_diagnostic!(
        context,
        source,
        "unexpected end of file",
        regex!(r#",-\[test[\d]+:1:32\]"#),
        "1 | begin push.1 add while.true mul",
        "  `----",
        r#" help: expected ".", or primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );
    Ok(())
}

// DUMMY LIBRARY
// ================================================================================================

struct DummyLibrary {
    namespace: LibraryNamespace,
    modules: Vec<Rc<Module>>,
    dependencies: Vec<LibraryNamespace>,
}

impl DummyLibrary {
    fn new(namespace: LibraryNamespace, modules: Vec<Rc<Module>>) -> Self {
        Self {
            namespace,
            modules,
            dependencies: Vec::new(),
        }
    }
}

impl Library for DummyLibrary {
    fn root_ns(&self) -> &LibraryNamespace {
        &self.namespace
    }

    fn version(&self) -> &Version {
        const MIN: Version = Version::min();
        &MIN
    }

    fn modules(&self) -> impl ExactSizeIterator<Item = &Module> + '_ {
        self.modules.iter().map(|p| p.as_ref())
    }

    fn dependencies(&self) -> &[LibraryNamespace] {
        &self.dependencies
    }
}
