use alloc::{string::ToString, vec::Vec};

use vm_core::{
    mast::{MastNode, MastNodeId},
    Program,
};

use crate::{
    assert_diagnostic_lines,
    ast::{Module, ModuleKind},
    diagnostics::{IntoDiagnostic, Report},
    regex, source_file,
    testing::{Pattern, TestContext},
    Assembler, Deserializable, LibraryPath, ModuleParser, Serializable,
};

type TestResult = Result<(), Report>;

use pretty_assertions::{assert_eq, assert_str_eq};

macro_rules! assert_assembler_diagnostic {
    ($context:ident, $source:expr, $($expected:literal),+) => {{
        let error = $context
            .assemble($source)
            .expect_err("expected diagnostic to be raised, but compilation succeeded");
        assert_diagnostic_lines!(error, $($expected),*);
    }};

    ($context:ident, $source:expr, $($expected:expr),+) => {{
        let error = $context
            .assemble($source)
            .expect_err("expected diagnostic to be raised, but compilation succeeded");
        assert_diagnostic_lines!(error, $($expected),*);
    }};
}

macro_rules! parse_module {
    ($context:expr, $path:literal, $source:expr) => {{
        let path = LibraryPath::new($path).into_diagnostic()?;
        let source_file =
            $context.source_manager().load(concat!("test", line!()), $source.to_string());
        Module::parse(path, ModuleKind::Library, source_file)?
    }};
}

// SIMPLE PROGRAMS
// ================================================================================================

#[test]
fn simple_instructions() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(&context, "begin push.0 assertz end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block pad eqz assert(0) end
end";
    assert_str_eq!(format!("{program}"), expected);

    let source = source_file!(&context, "begin push.10 push.50 push.2 u32wrapping_madd end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block push(10) push(50) push(2) u32madd drop end
end";
    assert_str_eq!(format!("{program}"), expected);

    let source = source_file!(&context, "begin push.10 push.50 push.2 u32wrapping_add3 end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block push(10) push(50) push(2) u32add3 drop end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

/// TODO(pauls): Do we want to allow this in Miden Assembly?
#[test]
#[ignore]
fn empty_program() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(&context, "begin end");
    let program = context.assemble(source)?;
    let expected = "begin basic_block noop end end";
    assert_eq!(expected, format!("{}", program));
    Ok(())
}

#[test]
fn empty_if() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(&context, "begin if.true end end");
    assert_assembler_diagnostic!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:15\]"#),
        "1 | begin if.true end end",
        "  :               ^|^",
        "  :                `-- found a end here",
        "  `----",
        " help: expected primitive opcode (e.g. \"add\"), or \"else\", or control flow",
        "       opcode (e.g. \"if.true\")"
    );
    Ok(())
}

#[test]
fn empty_if_true_then_branch() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(&context, "begin if.true nop end end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    if.true
        basic_block noop end
    else
        basic_block noop end
    end
end";
    assert_str_eq!(format!("{}", program), expected);
    Ok(())
}

/// TODO(pauls): Do we want to allow this in Miden Assembly
#[test]
#[ignore]
fn empty_while() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(&context, "begin while.true end end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    while.true
        basic_block noop end
    end
end";
    assert_str_eq!(format!("{}", program), expected);
    Ok(())
}

/// TODO(pauls): Do we want to allow this in Miden Assembly
#[test]
#[ignore]
fn empty_repeat() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(&context, "begin repeat.5 end end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block noop noop noop noop noop end
end";
    assert_str_eq!(format!("{}", program), expected);
    Ok(())
}

/// This test ensures that all iterations of a repeat control block are merged into a single basic
/// block.
#[test]
fn repeat_basic_blocks_merged() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(&context, "begin mul repeat.5 add end end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block mul add add add add add end
end";
    assert_str_eq!(format!("{}", program), expected);

    // Also ensure that dead code elimination works properly
    assert_eq!(program.mast_forest().num_nodes(), 1);
    Ok(())
}

#[test]
fn single_basic_block() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(&context, "begin push.1 push.2 add end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block pad incr push(2) add end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn basic_block_and_simple_if_true() -> TestResult {
    let context = TestContext::default();

    // if with else
    let source = source_file!(&context, "begin push.2 push.3 if.true add else mul end end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    join
        basic_block push(2) push(3) end
        if.true
            basic_block add end
        else
            basic_block mul end
        end
    end
end";
    assert_str_eq!(format!("{program}"), expected);

    // if without else
    let source = source_file!(&context, "begin push.2 push.3 if.true add end end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    join
        basic_block push(2) push(3) end
        if.true
            basic_block add end
        else
            basic_block noop end
        end
    end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn basic_block_and_simple_if_false() -> TestResult {
    let context = TestContext::default();

    // if with else
    let source = source_file!(&context, "begin push.2 push.3 if.false add else mul end end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    join
        basic_block push(2) push(3) end
        if.true
            basic_block mul end
        else
            basic_block add end
        end
    end
end";
    assert_str_eq!(format!("{program}"), expected);

    // if without else
    let source = source_file!(&context, "begin push.2 push.3 if.false add end end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    join
        basic_block push(2) push(3) end
        if.true
            basic_block noop end
        else
            basic_block add end
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
            &context,
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
    context.assemble(source_file!(
        &context,
        "
        use.context::account
        begin
          call.account::account_method_1
        end
        "
    ))?;

    // compile note 2 program
    context.assemble(source_file!(
        &context,
        "
        use.context::account
        begin
          call.account::account_method_2
        end
        "
    ))?;
    Ok(())
}

// TODO: Fix test after we implement the new `Assembler::add_library()`
#[ignore]
#[test]
fn call_without_path() -> TestResult {
    let context = TestContext::default();

    // compile first module
    context.assemble_module(
        "account_code1".parse().unwrap(),
        source_file!(
            &context,
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
    context.assemble_module(
        "account_code2".parse().unwrap(),
        source_file!(
            &context,
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
    context.assemble(source_file!(
        &context,
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
            &context,
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
            &context,
            "
        use.module::path::one
        export.one::foo

        export.bar
            procref.one::aaa
        end"
        ),
    )?;

    // compile program with procref calls
    context.assemble(source_file!(
        &context,
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
    let context = TestContext::default();
    // Module `two` is unknown. This program should return
    // `AssemblyError::UndefinedProcedure`, referencing the
    // use of `bar`
    let module_source1 = source_file!(
        &context,
        "
    use.module::path::two

    export.foo
        procref.two::bar
    end"
    );
    let module_path_one = "module::path::one".parse().unwrap();
    let module1 = context.parse_module_with_path(module_path_one, module_source1)?;

    let report = Assembler::new(context.source_manager())
        .assemble_library(core::iter::once(module1))
        .expect_err("expected unknown module error");

    assert_diagnostic_lines!(
        report,
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
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    const.TEST_CONSTANT=7
    begin
        push.TEST_CONSTANT
    end"
    );
    let expected = "\
begin
    basic_block push(7) end
end";
    let program = context.assemble(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn multiple_constants_push() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "const.CONSTANT_1=21 \
    const.CONSTANT_2=44 \
    begin \
    push.CONSTANT_1.64.CONSTANT_2.72 \
    end"
    );
    let expected = "\
begin
    basic_block push(21) push(64) push(44) push(72) end
end";
    let program = context.assemble(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn constant_numeric_expression() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "const.TEST_CONSTANT=11-2+4*(12-(10+1))+9+8//4*2 \
    begin \
    push.TEST_CONSTANT \
    end \
    "
    );
    let expected = "\
begin
    basic_block push(26) end
end";
    let program = context.assemble(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn constant_alphanumeric_expression() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
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
    basic_block push(21) end
end";
    let program = context.assemble(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn constant_hexadecimal_value() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "const.TEST_CONSTANT=0xFF \
    begin \
    push.TEST_CONSTANT \
    end \
    "
    );
    let expected = "\
begin
    basic_block push(255) end
end";
    let program = context.assemble(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn constant_field_division() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "const.TEST_CONSTANT=(17//4)/4*(1//2)+2 \
    begin \
    push.TEST_CONSTANT \
    end \
    "
    );
    let expected = "\
begin
    basic_block push(2) end
end";
    let program = context.assemble(source)?;
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn constant_err_const_not_initialized() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
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
    let context = TestContext::default();
    let source = source_file!(
        &context,
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
        &context,
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
    let context = TestContext::default();
    let source = source_file!(
        &context,
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
    let context = TestContext::default();
    let source = source_file!(
        &context,
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
    let context = TestContext::default();
    let source = source_file!(
        &context,
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
        " help: expected \"*\", or \"+\", or \"-\", or \"/\", or \"//\", or \"@\", or \"begin\", or \"const\", \
or \"export\", or \"proc\", or \"use\", or end of file, or doc comment"
    );
    Ok(())
}

#[test]
fn constant_must_be_within_valid_felt_range() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
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
    let context = TestContext::default();
    let source = source_file!(
        &context,
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
    let context = TestContext::new();
    let source = source_file!(
        &context,
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
    let context = TestContext::default();

    // Define constant values
    const PROC_LOC_STORE_PTR: u64 = 0;
    const PROC_LOC_LOAD_PTR: u64 = 1;
    const PROC_LOC_STOREW_PTR: u64 = 2;
    const PROC_LOC_LOADW_PTR: u64 = 3;
    const GLOBAL_STORE_PTR: u64 = 4;
    const GLOBAL_LOAD_PTR: u64 = 5;
    const GLOBAL_STOREW_PTR: u64 = 6;
    const GLOBAL_LOADW_PTR: u64 = 7;

    let source = source_file!(
        &context,
        format!(
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
        )
    );
    let program = context.assemble(source)?;

    // Define expected
    let expected = source_file!(
        &context,
        format!(
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
        )
    );
    let expected_program = context.assemble(expected)?;
    assert_str_eq!(expected_program.to_string(), program.to_string());
    Ok(())
}

#[test]
fn const_conversion_failed_to_u16() -> TestResult {
    // Define constant value greater than u16::MAX
    let constant_value: u64 = u16::MAX as u64 + 1;

    let context = TestContext::default();
    let source = source_file!(
        &context,
        format!(
            "\
    const.CONSTANT={constant_value}

    proc.test_constant_overflow.1
        loc_load.CONSTANT
    end

    begin
        exec.test_constant_overflow
    end
    "
        )
    );

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
    let context = TestContext::default();
    // Define constant value greater than u16::MAX
    let constant_value: u64 = u32::MAX as u64 + 1;

    let source = source_file!(
        &context,
        format!(
            "\
    const.CONSTANT={constant_value}

    begin
        mem_load.CONSTANT
    end
    "
        )
    );

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

// DECORATORS
// ================================================================================================

#[test]
fn decorators_basic_block() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    begin
        trace.0
        add
        trace.1
        mul
        trace.2
    end"
    );
    let expected = "\
begin
    basic_block trace(0) add trace(1) mul trace(2) end
end";
    let program = context.assemble(source)?;
    assert_str_eq!(expected, format!("{program}"));
    Ok(())
}

#[test]
fn decorators_repeat_one_basic_block() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    begin
        trace.0
        repeat.2 add end
        trace.1
        repeat.2 mul end
        trace.2
    end"
    );
    let expected = "\
begin
    basic_block trace(0) add add trace(1) mul mul trace(2) end
end";
    let program = context.assemble(source)?;
    assert_str_eq!(expected, format!("{program}"));
    Ok(())
}

#[test]
fn decorators_repeat_split() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    begin
        trace.0
        repeat.2
            if.true
                trace.1 push.42 trace.2
            else
                trace.3 push.22 trace.3
            end
            trace.4
        end
        trace.5
    end"
    );
    let expected = "\
begin
    join
        trace(0)
        if.true
            basic_block trace(1) push(42) trace(2) end
        else
            basic_block trace(3) push(22) trace(3) end
        end
        trace(4)
        if.true
            basic_block trace(1) push(42) trace(2) end
        else
            basic_block trace(3) push(22) trace(3) end
        end
        trace(4)
    end
    trace(5)
end";
    let program = context.assemble(source)?;
    assert_str_eq!(expected, format!("{program}"));
    Ok(())
}

#[test]
fn decorators_call() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    begin
        trace.0 trace.1
        call.0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
        trace.2
    end"
    );
    let expected = "\
begin
    trace(0) trace(1)
    call.0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
    trace(2)
end";
    let program = context.assemble(source)?;
    assert_str_eq!(expected, format!("{program}"));
    Ok(())
}

#[test]
fn decorators_dyn() -> TestResult {
    // single line
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    begin
        trace.0
        dynexec
        trace.1
    end"
    );
    let expected = "\
begin
    trace(0) dyn trace(1)
end";
    let program = context.assemble(source)?;
    assert_str_eq!(expected, format!("{program}"));

    // multi line
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    begin
        trace.0 trace.1 trace.2 trace.3 trace.4
        dynexec
        trace.5 trace.6 trace.7 trace.8 trace.9
    end"
    );
    let expected = "\
begin
    trace(0) trace(1) trace(2) trace(3) trace(4)
    dyn
    trace(5) trace(6) trace(7) trace(8) trace(9)
end";
    let program = context.assemble(source)?;
    assert_str_eq!(expected, format!("{program}"));
    Ok(())
}

#[test]
fn decorators_external() -> TestResult {
    let context = TestContext::default();
    let baz = r#"
        export.f
            push.7 push.8 sub
        end
    "#;
    let baz = parse_module!(&context, "lib::baz", baz);

    let lib = Assembler::new(context.source_manager()).assemble_library([baz])?;

    let program_source = source_file!(
        &context,
        "\
    use.lib::baz
    begin
        trace.0
        exec.baz::f
        trace.1
    end"
    );

    let expected = "\
begin
    trace(0)
    external.0xe776df8dc02329acc43a09fe8e510b44a87dfd876e375ad383891470ece4f6de
    trace(1)
end";
    let program = Assembler::new(context.source_manager())
        .with_library(lib)?
        .assemble_program(program_source)?;
    assert_str_eq!(expected, format!("{program}"));

    Ok(())
}

#[test]
fn decorators_join_and_split() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    begin
        trace.0 trace.1
        if.true
            trace.2 add trace.3
        else
            trace.4 mul trace.5
        end
        trace.6
        if.true
            trace.7 push.42 trace.8
        else
            trace.9 push.22 trace.10
        end
        trace.11
    end"
    );
    let expected = "\
begin
    join
        trace(0) trace(1)
        if.true
            basic_block trace(2) add trace(3) end
        else
            basic_block trace(4) mul trace(5) end
        end
        trace(6)
        if.true
            basic_block trace(7) push(42) trace(8) end
        else
            basic_block trace(9) push(22) trace(10) end
        end
    end
    trace(11)
end";
    let program = context.assemble(source)?;
    assert_str_eq!(expected, format!("{program}"));
    Ok(())
}

// ASSERTIONS
// ================================================================================================

#[test]
fn assert_with_code() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    const.ERR1=1

    begin
        assert
        assert.err=ERR1
        assert.err=2
    end
    "
    );
    let program = context.assemble(source)?;

    let expected = "\
begin
    basic_block assert(0) assert(1) assert(2) end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn assertz_with_code() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    const.ERR1=1

    begin
        assertz
        assertz.err=ERR1
        assertz.err=2
    end
    "
    );
    let program = context.assemble(source)?;

    let expected = "\
begin
    basic_block eqz assert(0) eqz assert(1) eqz assert(2) end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn assert_eq_with_code() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    const.ERR1=1

    begin
        assert_eq
        assert_eq.err=ERR1
        assert_eq.err=2
    end
    "
    );
    let program = context.assemble(source)?;

    let expected = "\
begin
    basic_block eq assert(0) eq assert(1) eq assert(2) end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn assert_eqw_with_code() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    const.ERR1=1

    begin
        assert_eqw
        assert_eqw.err=ERR1
        assert_eqw.err=2
    end
    "
    );
    let program = context.assemble(source)?;

    let expected = "\
begin
    basic_block
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
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    const.ERR1=1

    begin
        u32assert
        u32assert.err=ERR1
        u32assert.err=2
    end
    "
    );
    let program = context.assemble(source)?;

    let expected = "\
begin
    basic_block
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
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    const.ERR1=1

    begin
        u32assert2
        u32assert2.err=ERR1
        u32assert2.err=2
    end
    "
    );
    let program = context.assemble(source)?;

    let expected = "\
begin
    basic_block u32assert2(0) u32assert2(1) u32assert2(2) end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn u32assertw_with_code() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    const.ERR1=1

    begin
        u32assertw
        u32assertw.err=ERR1
        u32assertw.err=2
    end
    "
    );
    let program = context.assemble(source)?;

    let expected = "\
begin
    basic_block
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

/// Ensure that there is no collision between `Assert`, `U32assert2`, and `MpVerify`  instructions
/// with different inner values (which all don't contribute to the MAST root).
#[test]
fn asserts_and_mpverify_with_code_in_duplicate_procedure() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    proc.f1
        u32assert.err=1
    end
    proc.f2
        u32assert.err=2
    end
    proc.f12
        u32assert.err=1
        u32assert.err=2
    end
    proc.f21
        u32assert.err=2
        u32assert.err=1
    end
    proc.g1
        assert.err=1
    end
    proc.g2
        assert.err=2
    end
    proc.g12
        assert.err=1
        assert.err=2
    end
    proc.g21
        assert.err=2
        assert.err=1
    end
    proc.fg
        assert.err=1
        u32assert.err=1
        assert.err=2
        u32assert.err=2

        u32assert.err=1
        assert.err=1
        u32assert.err=2
        assert.err=2
    end

    proc.mpverify
        mtree_verify.err=1
        mtree_verify.err=2
        mtree_verify.err=2
        mtree_verify.err=1
    end

    begin
        exec.f1
        exec.f2
        exec.f12
        exec.f21
        exec.g1
        exec.g2
        exec.g12
        exec.g21
        exec.fg
        exec.mpverify
    end
    "
    );
    let program = context.assemble(source)?;

    let expected = "\
begin
    basic_block
        pad
        u32assert2(1)
        drop
        pad
        u32assert2(2)
        drop
        pad
        u32assert2(1)
        drop
        pad
        u32assert2(2)
        drop
        pad
        u32assert2(2)
        drop
        pad
        u32assert2(1)
        drop
        assert(1)
        assert(2)
        assert(1)
        assert(2)
        assert(2)
        assert(1)
        assert(1)
        pad
        u32assert2(1)
        drop
        assert(2)
        pad
        u32assert2(2)
        drop
        pad
        u32assert2(1)
        drop
        assert(1)
        pad
        u32assert2(2)
        drop
        assert(2)
        mpverify(1)
        mpverify(2)
        mpverify(2)
        mpverify(1)
    end
end";

    assert_str_eq!(expected, format!("{program}"));
    Ok(())
}

#[test]
fn mtree_verify_with_code() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
    const.ERR1=1

    begin
        mtree_verify
        mtree_verify.err=ERR1
        mtree_verify.err=2
    end
    "
    );

    let program = context.assemble(source)?;

    let expected = "\
begin
    basic_block mpverify(0) mpverify(1) mpverify(2) end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

// NESTED CONTROL BLOCKS
// ================================================================================================

#[test]
fn nested_control_blocks() -> TestResult {
    let context = TestContext::default();

    // if with else
    let source = source_file!(
        &context,
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
    let program = context.assemble(source)?;
    let expected = "\
begin
    join
        join
            basic_block push(2) push(3) end
            if.true
                join
                    basic_block add end
                    while.true
                        basic_block push(7) push(11) add end
                    end
                end
            else
                join
                    basic_block mul push(8) push(8) end
                    if.true
                        basic_block mul end
                    else
                        basic_block noop end
                    end
                end
            end
        end
        basic_block push(3) add end
    end
end";
    assert_str_eq!(expected, format!("{program}"));
    Ok(())
}

// PROGRAMS WITH PROCEDURES
// ================================================================================================

/// If the program has 2 procedures with the same MAST root (but possibly different decorators), the
/// correct procedure is chosen on exec
#[test]
fn ensure_correct_procedure_selection_on_collision() -> TestResult {
    let context = TestContext::default();

    // if with else
    let source = source_file!(
        &context,
        "
        proc.f
            add
        end

        proc.g
            trace.2
            add
        end

        begin
            if.true
                exec.f
            else
                exec.g
            end
        end"
    );
    let program = context.assemble(source)?;

    // Note: those values were taken from adding prints to the assembler at the time of writing. It
    // is possible that this test starts failing if we end up ordering procedures differently.
    let expected_f_node_id =
        MastNodeId::from_u32_safe(1_u32, program.mast_forest().as_ref()).unwrap();
    let expected_g_node_id =
        MastNodeId::from_u32_safe(0_u32, program.mast_forest().as_ref()).unwrap();

    let (exec_f_node_id, exec_g_node_id) = {
        let split_node_id = program.entrypoint();
        let split_node = match &program.mast_forest()[split_node_id] {
            MastNode::Split(split_node) => split_node,
            _ => panic!("expected split node"),
        };

        (split_node.on_true(), split_node.on_false())
    };

    assert_eq!(program.mast_forest()[expected_f_node_id], program.mast_forest()[exec_f_node_id]);
    assert_eq!(program.mast_forest()[expected_g_node_id], program.mast_forest()[exec_g_node_id]);

    Ok(())
}

#[test]
fn program_with_one_procedure() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "proc.foo push.3 push.7 mul end begin push.2 push.3 add exec.foo end"
    );
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block push(2) push(3) add push(3) push(7) mul end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn program_with_nested_procedure() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
        proc.foo push.3 push.7 mul end \
        proc.bar push.5 exec.foo add end \
        begin push.2 push.4 add exec.foo push.11 exec.bar sub end"
    );
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block
        push(2)
        push(4)
        add
        push(3)
        push(7)
        mul
        push(11)
        push(5)
        push(3)
        push(7)
        mul
        add
        neg
        add
    end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn program_with_proc_locals() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
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
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block
        push(4)
        push(3)
        push(2)
        push(1)
        fmpupdate
        pad
        fmpadd
        mstore
        drop
        add
        pad
        fmpadd
        mload
        mul
        push(18446744069414584320)
        fmpupdate
    end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn program_with_proc_locals_fail() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "\
        proc.foo \
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
    assert_assembler_diagnostic!(
        context,
        source,
        "number of procedure locals was not set (or set to 0), but local values were used"
    );

    Ok(())
}

#[test]
fn program_with_exported_procedure() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "export.foo push.3 push.7 mul end begin push.2 push.3 add exec.foo end"
    );

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
    let context = TestContext::default();
    let source = source_file!(&context, "begin dynexec end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    dyn
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn program_with_dynamic_code_execution_in_new_context() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(&context, "begin dyncall end");
    let program = context.assemble(source)?;
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
    let context = TestContext::default();
    let source = source_file!(&context, "begin call.0x1234 end");

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
    let context = TestContext::default();
    let source = source_file!(
        &context,
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
    let context = TestContext::default();
    let source = source_file!(
        &context,
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
    let context = TestContext::default();
    let source = source_file!(
        &context,
        "begin call.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a213dae end"
    );
    let ast = context.parse_program(source)?;

    let assembler = Assembler::new(context.source_manager()).with_debug_mode(true);
    assembler.assemble_program(ast)?;
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
    let ast =
        context.parse_module_with_path(path, source_file!(&context, PROCEDURE.to_string()))?;
    let library = Assembler::new(context.source_manager())
        .assemble_library(core::iter::once(ast))
        .unwrap();

    context.add_library(&library)?;

    let source = source_file!(
        &context,
        format!(
            r#"
        use.{MODULE}
        begin
            push.4 push.3
            exec.u256::iszero_unsafe
            call.0x20234ee941e53a15886e733cc8e041198c6e90d2a16ea18ce1030e8c3596dd38
        end"#
        )
    );
    let program = context.assemble(source)?;

    let expected = "\
begin
    join
        join
            basic_block push(4) push(3) end
            external.0xc2545da99d3a1f3f38d957c7893c44d78998d8ea8b11aba7e22c8c2b2a213dae
        end
        call.0x20234ee941e53a15886e733cc8e041198c6e90d2a16ea18ce1030e8c3596dd38
    end
end";
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
    let ast =
        context.parse_module_with_path(path, source_file!(&context, PROCEDURE.to_string()))?;
    let library = Assembler::new(context.source_manager())
        .assemble_library(core::iter::once(ast))
        .unwrap();

    context.add_library(&library)?;

    let source = source_file!(
        &context,
        format!(
            r#"
        use.{MODULE}
        begin
            push.4 push.3
            exec.u256::iszero_unsafe
            exec.u256::iszero_unsafe_dup
        end"#
        )
    );
    context.assemble(source)?;
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
    let mut parser = Module::parser(ModuleKind::Library);
    let ast = parser
        .parse_str(MODULE.parse().unwrap(), MODULE_BODY, &context.source_manager())
        .unwrap();

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

    let mut parser = Module::parser(ModuleKind::Library);
    let ref_ast = parser
        .parse_str(REF_MODULE.parse().unwrap(), REF_MODULE_BODY, &context.source_manager())
        .unwrap();

    let library = Assembler::new(context.source_manager())
        .assemble_library([ast, ref_ast])
        .unwrap();

    context.add_library(&library)?;

    let source = source_file!(
        &context,
        format!(
            r#"
        use.{MODULE}
        begin
            push.4 push.3
            exec.u256::checked_eqz
            exec.u256::notchecked_eqz
        end"#
        )
    );
    let program = context.assemble(source)?;
    let expected = "\
begin
    join
        join
            basic_block push(4) push(3) end
            external.0xb9691da1d9b4b364aca0a0990e9f04c446a2faa622c8dd0d8831527dbec61393
        end
        external.0xcb08c107c81c582788cbf63c99f6b455e11b33bb98ca05fe1cfa17c087dfa8f1
    end
end";
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
    let mut parser = Module::parser(ModuleKind::Library);
    let source_manager = context.source_manager();
    // We reference code in this module
    let ref_ast =
        parser.parse_str(REF_MODULE.parse().unwrap(), REF_MODULE_BODY, &source_manager)?;
    // But only exports from this module are exposed by the library
    let ast = parser.parse_str(MODULE.parse().unwrap(), MODULE_BODY, &source_manager)?;

    let dummy_library =
        Assembler::new(source_manager).with_module(ref_ast)?.assemble_library([ast])?;

    // Now we want to use the the library we've compiled
    context.add_library(&dummy_library)?;

    let source = source_file!(
        &context,
        format!(
            r#"
        use.{MODULE}
        begin
            push.4 push.3
            exec.u256::checked_eqz
            exec.u256::notchecked_eqz
        end"#
        )
    );
    let program = context.assemble(source)?;

    let expected = "\
begin
    join
        join
            basic_block push(4) push(3) end
            external.0xb9691da1d9b4b364aca0a0990e9f04c446a2faa622c8dd0d8831527dbec61393
        end
        external.0xcb08c107c81c582788cbf63c99f6b455e11b33bb98ca05fe1cfa17c087dfa8f1
    end
end";
    assert_str_eq!(format!("{program}"), expected);

    // We also want to assert that exports from the referenced module do not leak
    let mut context = TestContext::default();
    context.add_library(dummy_library)?;

    let source = source_file!(
        &context,
        format!(
            r#"
        use.{REF_MODULE}
        begin
            push.4 push.3
            exec.u64::checked_eqz
            exec.u64::notchecked_eqz
        end"#
        )
    );
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
    let source_manager = context.source_manager();
    let mut parser = Module::parser(ModuleKind::Library);
    let ast = parser.parse_str(MODULE.parse().unwrap(), PROCEDURE, &source_manager).unwrap();
    let library = Assembler::new(source_manager).assemble_library([ast]).unwrap();

    context.add_library(&library)?;

    let source = source_file!(
        &context,
        "
        use.dummy::math::u64->bigint

        begin
            push.1.0
            push.2.0
            exec.bigint::checked_add
        end"
    );

    let program = context.assemble(source)?;
    let expected = "\
begin
    join
        basic_block pad incr pad push(2) pad end
        external.0x3cff5b58a573dc9d25fd3c57130cc57e5b1b381dc58b5ae3594b390c59835e63
    end
end";
    assert_str_eq!(format!("{program}"), expected);

    // --- invalid module alias -----------------------------------------------
    let source = source_file!(
        &context,
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
        r#" help: expected "@", or "begin", or "const", or "export", or "proc", or "use", or end of file, or doc comment"#
    );

    // --- duplicate module import --------------------------------------------
    let source = source_file!(
        &context,
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
    &context,
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
    let context = TestContext::default();
    // --- non-existent import ------------------------------------------------
    let source = source_file!(
        &context,
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
        &context,
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
    let context = TestContext::default();
    let source = source_file!(&context, "begin # simple comment \n push.1 push.2 add end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block pad incr push(2) add end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn comment_in_nested_control_blocks() -> TestResult {
    let context = TestContext::default();

    // if with else
    let source = source_file!(
        &context,
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
    let program = context.assemble(source)?;
    let expected = "\
begin
    join
        join
            basic_block pad incr push(2) end
            if.true
                join
                    basic_block add end
                    while.true
                        basic_block push(7) push(11) add end
                    end
                end
            else
                join
                    basic_block mul push(8) push(8) end
                    if.true
                        basic_block mul end
                    else
                        basic_block noop end
                    end
                end
            end
        end
        basic_block push(3) add end
    end
end";
    assert_str_eq!(expected, format!("{program}"));
    Ok(())
}

#[test]
fn comment_before_program() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(&context, " # starting comment \n begin push.1 push.2 add end");
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block pad incr push(2) add end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

#[test]
fn comment_after_program() -> TestResult {
    let context = TestContext::default();
    let source = source_file!(&context, "begin push.1 push.2 add end # closing comment");
    let program = context.assemble(source)?;
    let expected = "\
begin
    basic_block pad incr push(2) add end
end";
    assert_str_eq!(format!("{program}"), expected);
    Ok(())
}

// ERRORS
// ================================================================================================

#[test]
fn invalid_empty_program() {
    let context = TestContext::default();
    assert_assembler_diagnostic!(
        context,
        source_file!(&context, ""),
        "unexpected end of file",
        regex!(r#",-\[test[\d]+:1:1\]"#),
        "`----",
        r#" help: expected "@", or "begin", or "const", or "export", or "proc", or "use", or doc comment"#
    );

    assert_assembler_diagnostic!(
        context,
        source_file!(&context, ""),
        "unexpected end of file",
        regex!(r#",-\[test[\d]+:1:1\]"#),
        "  `----",
        r#" help: expected "@", or "begin", or "const", or "export", or "proc", or "use", or doc comment"#
    );
}

#[test]
fn invalid_program_unrecognized_token() {
    let context = TestContext::default();
    assert_assembler_diagnostic!(
        context,
        source_file!(&context, "none"),
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:1\]"#),
        "1 | none",
        "  : ^^|^",
        "  :   `-- found a identifier here",
        "  `----",
        r#" help: expected "@", or "begin", or "const", or "export", or "proc", or "use", or doc comment"#
    );
}

#[test]
fn invalid_program_unmatched_begin() {
    let context = TestContext::default();
    assert_assembler_diagnostic!(
        context,
        source_file!(&context, "begin add"),
        "unexpected end of file",
        regex!(r#",-\[test[\d]+:1:10\]"#),
        "1 | begin add",
        "  `----",
        r#" help: expected ".", or primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );
}

#[test]
fn invalid_program_invalid_top_level_token() {
    let context = TestContext::default();
    assert_assembler_diagnostic!(
        context,
        source_file!(&context, "begin add end mul"),
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:15\]"#),
        "1 | begin add end mul",
        "  :               ^|^",
        "  :                `-- found a mul here",
        "  `----",
        r#" help: expected "@", or "begin", or "const", or "export", or "proc", or "use", or end of file, or doc comment"#
    );
}

#[test]
fn invalid_proc_missing_end_unexpected_begin() {
    let context = TestContext::default();
    let source = source_file!(&context, "proc.foo add mul begin push.1 end");
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
    let context = TestContext::default();
    let source = source_file!(&context, "proc.foo add mul proc.bar push.3 end begin push.1 end");
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
    let context = TestContext::default();
    let source = source_file!(&context, "proc.foo add mul end begin push.1 exec.bar end");
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
    let context = TestContext::default();
    let source = source_file!(&context, "proc.123 add mul end begin push.1 exec.123 end");
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
    let context = TestContext::default();
    let source =
        source_file!(&context, "proc.foo add mul end proc.foo push.3 end begin push.1 end");
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
    let context = TestContext::default();
    let source = source_file!(&context, "begin push.1 add if.true mul");
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
    let context = TestContext::default();
    let source = source_file!(&context, "begin push.1 add else mul end");
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

    let source = source_file!(&context, "begin push.1 while.true add else mul end end");
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
    let context = TestContext::default();

    let source =
        source_file!(&context, "begin push.1 if.true add else mul else push.1 end end end");
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
    let context = TestContext::default();

    let source = source_file!(&context, "begin push.1 add if.true mul else add");
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
    let context = TestContext::default();

    // unmatched repeat
    let source = source_file!(&context, "begin push.1 add repeat.10 mul");
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
    let source = source_file!(&context, "begin push.1 add repeat.23x3 mul end end");
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
    let context = TestContext::default();

    let source = source_file!(&context, "begin push.1 add while mul end end");
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

    let source = source_file!(&context, "begin push.1 add while.abc mul end end");
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

    let source = source_file!(&context, "begin push.1 add while.true mul");
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

// COMPILED LIBRARIES
// ================================================================================================
#[test]
fn test_compiled_library() {
    let context = TestContext::new();
    let mut mod_parser = ModuleParser::new(ModuleKind::Library);
    let mod1 = {
        let source = source_file!(
            &context,
            "
    proc.internal
        push.5
    end
    export.foo
        push.1
        drop
    end
    export.bar
        exec.internal
        drop
    end
    "
        );
        mod_parser.parse(LibraryPath::new("mylib::mod1").unwrap(), source).unwrap()
    };

    let mod2 = {
        let source = source_file!(
            &context,
            "
    export.foo
        push.7
        add.5
    end
    # Same definition as mod1::foo
    export.bar
        push.1
        drop
    end
    "
        );
        mod_parser.parse(LibraryPath::new("mylib::mod2").unwrap(), source).unwrap()
    };

    let compiled_library = {
        let assembler = Assembler::new(context.source_manager());
        assembler.assemble_library([mod1, mod2]).unwrap()
    };

    assert_eq!(compiled_library.exports().count(), 4);

    // Compile program that uses compiled library
    let mut assembler = Assembler::new(context.source_manager());

    assembler.add_library(&compiled_library).unwrap();

    let program_source = "
    use.mylib::mod1
    use.mylib::mod2

    proc.foo
        push.1
        drop
    end

    begin
        exec.mod1::foo
        exec.mod1::bar
        exec.mod2::foo
        exec.mod2::bar
        exec.foo
    end
    ";

    let _program = assembler.assemble_program(program_source).unwrap();
}

#[test]
fn test_reexported_proc_with_same_name_as_local_proc_diff_locals() {
    let context = TestContext::new();
    let mut mod_parser = ModuleParser::new(ModuleKind::Library);
    let mod1 = {
        let source = source_file!(
            &context,
            "export.foo.2
                push.1
                drop
            end
            "
        );
        mod_parser.parse(LibraryPath::new("test::mod1").unwrap(), source).unwrap()
    };

    let mod2 = {
        let source = source_file!(
            &context,
            "use.test::mod1
            export.foo
                exec.mod1::foo
            end
            "
        );
        mod_parser.parse(LibraryPath::new("test::mod2").unwrap(), source).unwrap()
    };

    let compiled_library = {
        let assembler = Assembler::new(context.source_manager());
        assembler.assemble_library([mod1, mod2]).unwrap()
    };

    assert_eq!(compiled_library.exports().count(), 2);

    // Compile program that uses compiled library
    let mut assembler = Assembler::new(context.source_manager());

    assembler.add_library(&compiled_library).unwrap();

    let program_source = "
    use.test::mod1
    use.test::mod2

    proc.foo.1
        exec.mod1::foo
        exec.mod2::foo
    end

    begin
        exec.foo
    end
    ";

    let _program = assembler.assemble_program(program_source).unwrap();
}

// PROGRAM SERIALIZATION AND DESERIALIZATION
// ================================================================================================
#[test]
fn test_program_serde_simple() {
    let source = "
    begin
        push.1.2
        add
        drop
    end
    ";

    let assembler = Assembler::default();
    let original_program = assembler.assemble_program(source).unwrap();

    let mut target = Vec::new();
    original_program.write_into(&mut target);
    let deserialized_program = Program::read_from_bytes(&target).unwrap();

    assert_eq!(original_program, deserialized_program);
}

#[test]
fn test_program_serde_with_decorators() {
    let source = "
    const.DEFAULT_CONST=100

    proc.foo
        push.1.2 add
        debug.stack.8
    end

    begin
        emit.DEFAULT_CONST

        exec.foo

        debug.stack.4

        drop

        trace.DEFAULT_CONST
    end
    ";

    let assembler = Assembler::default().with_debug_mode(true);
    let original_program = assembler.assemble_program(source).unwrap();

    let mut target = Vec::new();
    original_program.write_into(&mut target);
    let deserialized_program = Program::read_from_bytes(&target).unwrap();

    assert_eq!(original_program, deserialized_program);
}
