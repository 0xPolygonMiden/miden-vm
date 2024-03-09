use alloc::{string::ToString, sync::Arc, vec::Vec};

use crate::{
    assert_diagnostic, assert_diagnostic_lines,
    ast::*,
    diagnostics::{reporting::PrintDiagnostic, Report},
    regex, source_file,
    testing::{Pattern, TestContext},
    Felt, Span,
};

use pretty_assertions::assert_eq;

macro_rules! inst {
    ($inst:ident ($value:expr)) => {
        Op::Inst(Span::unknown(Instruction::$inst($value)))
    };

    ($inst:ident) => {
        Op::Inst(Span::unknown(Instruction::$inst))
    };
}

macro_rules! exec {
    ($name:ident) => {
        inst!(Exec(InvocationTarget::ProcedureName(
            stringify!($name).parse().expect("invalid procedure name")
        )))
    };

    ($name:path) => {{
        let path = stringify!($name);
        let (module, name) = path.split_once("::").expect("invalid procedure path");
        let name =
            Ident::new_unchecked(Span::unknown(Arc::from(name.to_string().into_boxed_str())));
        let name = ProcedureName::new_unchecked(name);

        inst!(Exec(InvocationTarget::ProcedurePath {
            name,
            module: module.parse().unwrap(),
        }))
    }};
}

#[allow(unused_macros)]
macro_rules! call {
    ($name:ident) => {
        inst!(Call(InvocationTarget::ProcedureName(stringify!($name).parse())))
    };

    ($name:path) => {{
        let path = stringify!($name);
        let (module, name) = path.split_once("::").expect("invalid procedure path");
        let name = ProcedureName::new_unchecked(Default::default(), name);

        inst!(Call(InvocationTarget::ProcedurePath { name, module }))
    }};
}

macro_rules! block {
    ($($insts:expr),+) => {
        Block::new(Default::default(), Vec::from([$($insts),*]))
    }
}

macro_rules! moduledoc {
    ($doc:literal) => {
        Form::ModuleDoc(Span::unknown($doc.to_string()))
    };

    ($doc:ident) => {
        Form::ModuleDoc(Span::unknown($doc.to_string()))
    };
}

macro_rules! doc {
    ($doc:literal) => {
        Form::Doc(Span::unknown($doc.to_string()))
    };

    ($doc:ident) => {
        Form::Doc(Span::unknown($doc.to_string()))
    };
}

macro_rules! begin {
    ($($insts:expr),+) => {
        Form::Begin(block!($($insts),*))
    }
}

macro_rules! if_true {
    ($then_blk:expr) => {
        Op::If {
            span: Default::default(),
            then_blk: $then_blk,
            else_blk: Block::default(),
        }
    };

    ($then_blk:expr, $else_blk:expr) => {
        Op::If {
            span: Default::default(),
            then_blk: $then_blk,
            else_blk: $else_blk,
        }
    };
}

macro_rules! while_true {
    ($body:expr) => {
        Op::While {
            span: Default::default(),
            body: $body,
        }
    };
}

macro_rules! import {
    ($name:literal) => {{
        let path: crate::LibraryPath = $name.parse().expect("invalid import path");
        let name = path.last().parse().unwrap();
        Form::Import(Import {
            span: crate::SourceSpan::default(),
            name,
            path,
            uses: 0,
        })
    }};

    ($name:literal -> $alias:literal) => {
        let path: LibraryPath = $name.parse().expect("invalid import path");
        let name = $alias.parse().expect("invalid import alias");
        Form::Import(Import {
            span: SourceSpan::default(),
            name,
            path,
            uses: 0,
        })
    };
}

macro_rules! proc {
    ($name:ident, $num_locals:literal, $body:expr) => {
        Form::Procedure(Export::Procedure(Procedure::new(
            Default::default(),
            Visibility::Private,
            stringify!($name).parse().expect("invalid procedure name"),
            $num_locals,
            $body,
        )))
    };

    ($docs:expr, $name:ident, $num_locals:literal, $body:expr) => {
        Form::Procedure(Export::Procedure(
            Procedure::new(
                Default::default(),
                Visibility::Private,
                stringify!($name).parse().expect("invalid procedure name"),
                $num_locals,
                $body,
            )
            .with_docs(Some(Span::unknown($docs.to_string()))),
        ))
    };
}

macro_rules! export {
    ($name:ident, $num_locals:literal, $body:expr) => {
        Form::Procedure(Export::Procedure(Procedure::new(
            Default::default(),
            Visibility::Public,
            stringify!($name).parse().expect("invalid procedure name"),
            $num_locals,
            $body,
        )))
    };

    ($docs:expr, $name:ident, $num_locals:literal, $body:expr) => {
        Form::Procedure(Export::Procedure(
            Procedure::new(
                Default::default(),
                Visibility::Public,
                stringify!($name).parse().expect("invalid procedure name"),
                $num_locals,
                $body,
            )
            .with_docs(Some(Span::unknown($docs.to_string()))),
        ))
    };
}

macro_rules! module {
    ($($forms:expr),+) => {
        Vec::<Form>::from([
            $(
                Form::from($forms),
            )*
        ])
    }
}

macro_rules! assert_forms {
    ($context:ident, $source:expr, $expected:expr) => {
        match $context.parse_forms($source.clone()) {
            Ok(forms) => assert_eq!(forms, $expected),
            Err(report) => {
                panic!(
                    "expected parsing to succeed but failed with error:
{}",
                    crate::diagnostics::reporting::PrintDiagnostic::new_without_color(report)
                );
            }
        }
    };
}

macro_rules! assert_parse_diagnostic {
    ($source:expr, $expected:literal) => {{
        let source = $source.clone();
        let error = crate::parser::parse_forms(source.clone())
            .map_err(|err| Report::new(err).with_source_code(source))
            .expect_err("expected diagnostic to be raised, but parsing succeeded");
        assert_diagnostic!(error, $expected);
    }};

    ($source:expr, $expected:expr) => {{
        let source = $source.clone();
        let error = crate::parser::parse_forms(source.clone())
            .map_err(|err| Report::new(err).with_source_code(source))
            .expect_err("expected diagnostic to be raised, but parsing succeeded");
        assert_diagnostic!(error, $expected);
    }};
}

macro_rules! assert_parse_diagnostic_lines {
    ($source:expr, $($expected:literal),+) => {{
        let error = crate::parser::parse_forms(source.clone())
            .map_err(|err| Report::new(err).with_source_code(source))
            .expect_err("expected diagnostic to be raised, but parsing succeeded");
        assert_diagnostic_lines!(error, $($expected),*);
    }};

    ($source:expr, $($expected:expr),+) => {{
        let source = $source.clone();
        let error = crate::parser::parse_forms(source.clone())
            .map_err(|err| Report::new(err).with_source_code(source))
            .expect_err("expected diagnostic to be raised, but parsing succeeded");
        assert_diagnostic_lines!(error, $($expected),*);
    }};
}

macro_rules! assert_module_diagnostic_lines {
    ($context:ident, $source:expr, $($expected:literal),+) => {{
        let error = $context
            .parse_module($source)
            .expect_err("expected diagnostic to be raised, but parsing succeeded");
        assert_diagnostic_lines!(error, $($expected),*);
    }};

    ($context:ident, $source:expr, $($expected:expr),+) => {{
        let error = $context
            .parse_module($source)
            .expect_err("expected diagnostic to be raised, but parsing succeeded");
        assert_diagnostic_lines!(error, $($expected),*);
    }};
}

macro_rules! assert_program_diagnostic_lines {
    ($context:ident, $source:expr, $($expected:literal),+) => {{
        let error = $context
            .parse_program($source)
            .expect_err("expected diagnostic to be raised, but parsing succeeded");
        assert_diagnostic_lines!(error, $($expected),*);
    }};

    ($context:ident, $source:expr, $($expected:expr),+) => {{
        let error = $context
            .parse_program($source)
            .expect_err("expected diagnostic to be raised, but parsing succeeded");
        assert_diagnostic_lines!(error, $($expected),*);
    }};
}

// UNIT TESTS
// ================================================================================================

/// Tests the AST parsing
#[test]
fn test_ast_parsing_program_simple() -> Result<(), Report> {
    let mut context = TestContext::new();

    let source = source_file!("begin push.0 assertz add.1 end");
    let forms = module!(begin!(inst!(PushU8(0)), inst!(Assertz), inst!(Incr)));

    assert_eq!(context.parse_forms(source)?, forms);

    Ok(())
}

#[test]
fn test_ast_parsing_program_push() -> Result<(), Report> {
    let mut context = TestContext::new();

    let source = source_file!(
        r#"
    begin
        push.10 push.500 push.70000 push.5000000000
        push.5000000000.7000000000.9000000000.11000000000
        push.5.7
        push.500.700
        push.70000.90000
        push.5000000000.7000000000

        push.0x0000000000000000010000000000000002000000000000000300000000000000
    end"#
    );
    let forms = module!(begin!(
        inst!(PushU8(10)),
        inst!(PushU16(500)),
        inst!(PushU32(70000)),
        inst!(PushFelt(Felt::new(5000000000_u64))),
        inst!(PushFelt(Felt::new(5000000000_u64))),
        inst!(PushFelt(Felt::new(7000000000_u64))),
        inst!(PushFelt(Felt::new(9000000000_u64))),
        inst!(PushFelt(Felt::new(11000000000_u64))),
        inst!(PushU8(5)),
        inst!(PushU8(7)),
        inst!(PushU16(500)),
        inst!(PushU16(700)),
        inst!(PushU32(70000)),
        inst!(PushU32(90000)),
        inst!(PushFelt(Felt::new(5000000000_u64))),
        inst!(PushFelt(Felt::new(7000000000_u64))),
        inst!(PushWord([Felt::new(0), Felt::new(1), Felt::new(2), Felt::new(3)]))
    ));

    assert_eq!(context.parse_forms(source)?, forms);

    // Push a hexadecimal string containing more than 4 values
    let source_too_long = source_file!("begin push.0x00000000000000001000000000000000200000000000000030000000000000004000000000000000");
    assert_parse_diagnostic!(source_too_long, "long hex strings must contain exactly 64 digits");

    // Push a hexadecimal string containing less than 4 values
    let source_too_long = source_file!("begin push.0x00000000000000001000000000000000");
    assert_parse_diagnostic!(source_too_long, "expected 2, 4, 8, 16, or 64 hex digits");

    Ok(())
}

#[test]
fn test_ast_parsing_program_u32() -> Result<(), Report> {
    let mut context = TestContext::new();

    let source = source_file!(
        r#"
    begin
        push.3

        u32wrapping_add.5
        u32overflowing_add.5

        u32wrapping_sub.1
        u32overflowing_sub.1

        u32wrapping_mul.2
        u32overflowing_mul.2

    end"#
    );
    let forms = module!(begin!(
        inst!(PushU8(3)),
        inst!(U32WrappingAddImm(5u32.into())),
        inst!(U32OverflowingAddImm(5u32.into())),
        inst!(U32WrappingSubImm(1u32.into())),
        inst!(U32OverflowingSubImm(1u32.into())),
        inst!(U32WrappingMulImm(2u32.into())),
        inst!(U32OverflowingMulImm(2u32.into()))
    ));

    assert_eq!(context.parse_forms(source)?, forms);

    Ok(())
}

#[test]
fn test_ast_parsing_program_proc() -> Result<(), Report> {
    let mut context = TestContext::new();

    let source = source_file!(
        r#"
    proc.foo.1
        loc_load.0
    end
    proc.bar.2
        padw
    end
    begin
        exec.foo
        exec.bar
    end"#
    );

    let forms = module!(
        proc!(foo, 1, block!(inst!(LocLoad(0u16.into())))),
        proc!(bar, 2, block!(inst!(PadW))),
        begin!(exec!(foo), exec!(bar))
    );
    assert_eq!(context.parse_forms(source)?, forms);

    Ok(())
}

#[test]
fn test_ast_parsing_module() -> Result<(), Report> {
    let mut context = TestContext::new();
    let source = source_file!(
        r#"
    export.foo.1
        loc_load.0
    end"#
    );
    let forms = module!(export!(foo, 1, block!(inst!(LocLoad(0u16.into())))));
    assert_eq!(context.parse_forms(source)?, forms);
    Ok(())
}

#[test]
fn test_ast_parsing_adv_ops() -> Result<(), Report> {
    let mut context = TestContext::new();
    let source = source_file!("begin adv_push.1 adv_loadw end");
    let forms = module!(begin!(inst!(AdvPush(1u8.into())), inst!(AdvLoadW)));
    assert_eq!(context.parse_forms(source)?, forms);
    Ok(())
}

#[test]
fn test_ast_parsing_adv_injection() -> Result<(), Report> {
    use super::AdviceInjectorNode::*;

    let mut context = TestContext::new();
    let source =
        source_file!("begin adv.push_u64div adv.push_mapval adv.push_smtget adv.insert_mem end");
    let forms = module!(begin!(
        inst!(AdvInject(PushU64Div)),
        inst!(AdvInject(PushMapVal)),
        inst!(AdvInject(PushSmtGet)),
        inst!(AdvInject(InsertMem))
    ));
    assert_eq!(context.parse_forms(source)?, forms);
    Ok(())
}

#[test]
fn test_ast_parsing_bitwise_counters() -> Result<(), Report> {
    let mut context = TestContext::new();
    let source = source_file!("begin u32clz u32ctz u32clo u32cto end");
    let forms = module!(begin!(inst!(U32Clz), inst!(U32Ctz), inst!(U32Clo), inst!(U32Cto)));

    assert_eq!(context.parse_forms(source)?, forms);
    Ok(())
}

#[test]
fn test_ast_parsing_ilog2() -> Result<(), Report> {
    let mut context = TestContext::new();
    let source = source_file!("begin push.8 ilog2 end");
    let forms = module!(begin!(inst!(PushU8(8)), inst!(ILog2)));

    assert_eq!(context.parse_forms(source)?, forms);
    Ok(())
}

#[test]
fn test_ast_parsing_use() -> Result<(), Report> {
    let mut context = TestContext::new();
    let source = source_file!(
        r#"
    use.std::abc::foo
    begin
        exec.foo::bar
    end"#
    );
    let forms = module!(import!("std::abc::foo"), begin!(exec!(foo::bar)));
    assert_eq!(context.parse_forms(source)?, forms);
    // TODO: Assert fully-resolved name is `std::abc::foo::bar`
    Ok(())
}

#[test]
fn test_ast_parsing_module_nested_if() -> Result<(), Report> {
    let mut context = TestContext::new();
    let source = source_file!(
        r#"
    proc.foo
        push.1
        if.true
            push.0
            push.1
            if.true
                push.0
                sub
            else
                push.1
                sub
            end
        end
    end"#
    );

    let forms = module!(proc!(
        foo,
        0,
        block!(
            inst!(PushU8(1)),
            if_true!(block!(
                inst!(PushU8(0)),
                inst!(PushU8(1)),
                if_true!(
                    block!(inst!(PushU8(0)), inst!(Sub)),
                    block!(inst!(PushU8(1)), inst!(Sub))
                )
            ))
        )
    ));
    assert_eq!(context.parse_forms(source)?, forms);
    Ok(())
}

#[test]
fn test_ast_parsing_module_sequential_if() -> Result<(), Report> {
    let mut context = TestContext::new();
    let source = source_file!(
        r#"
    proc.foo
        push.1
        if.true
            push.5
            push.1
        end
        if.true
            push.0
            sub
        else
            push.1
            sub
        end
    end"#
    );

    let forms = module!(proc!(
        foo,
        0,
        block!(
            inst!(PushU8(1)),
            if_true!(block!(inst!(PushU8(5)), inst!(PushU8(1)))),
            if_true!(block!(inst!(PushU8(0)), inst!(Sub)), block!(inst!(PushU8(1)), inst!(Sub)))
        )
    ));

    assert_eq!(context.parse_forms(source)?, forms);
    Ok(())
}

#[test]
fn parsed_while_if_body() {
    let mut context = TestContext::new();
    let source = source_file!(
        "\
    begin
        push.1
        while.true
            mul
        end
        add
        if.true
            div
        end
        mul
    end
    "
    );

    let forms = module!(begin!(
        inst!(PushU8(1)),
        while_true!(block!(inst!(Mul))),
        inst!(Add),
        if_true!(block!(inst!(Div))),
        inst!(Mul)
    ));

    assert_forms!(context, source, forms);
}

// PROCEDURE IMPORTS
// ================================================================================================

#[test]
fn test_missing_import() {
    let mut context = TestContext::new();
    let source = source_file!(
        r#"
    begin
        exec.u64::add
    end"#
    );

    assert_program_diagnostic_lines!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "missing import: the referenced module has not been imported",
        regex!(r#",-\[test[\d]+:3:19\]"#),
        "2 |     begin",
        "3 |         exec.u64::add",
        "  :                   ^|^",
        "  :                    `-- this reference is invalid without a corresponding import",
        "4 |     end",
        "  `----"
    );
}

// INVALID BODY TESTS
// ================================================================================================

#[test]
fn test_use_in_proc_body() {
    let source = source_file!(
        r#"
    export.foo.1
        loc_load.0
        use
    end"#
    );

    assert_parse_diagnostic_lines!(
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:4:9\]"#),
        "3 |         loc_load.0",
        "4 |         use",
        " :         ^|^",
        "  :          `-- found a use here",
        "5 |     end",
        "  `----",
        r#" help: expected primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );
}

#[test]
fn test_unterminated_proc() {
    let source = source_file!("proc.foo add mul begin push.1 end");

    assert_parse_diagnostic_lines!(
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
fn test_unterminated_if() {
    let source = source_file!("proc.foo add mul if.true add.2 begin push.1 end");

    assert_parse_diagnostic_lines!(
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:1:32\]"#),
        "1 | proc.foo add mul if.true add.2 begin push.1 end",
        "  :                                ^^|^^",
        "  :                                  `-- found a begin here",
        "  `----",
        r#" help: expected primitive opcode (e.g. "add"), or "else", or "end", or control flow opcode (e.g. "if.true")"#
    );
}

// DOCUMENTATION PARSING TESTS
// ================================================================================================

#[test]
fn test_ast_parsing_simple_docs() -> Result<(), Report> {
    let mut context = TestContext::new();
    let source = source_file!(
        r#"
    #! proc doc
    export.foo.1
        loc_load.0
    end"#
    );

    let forms = module!(doc!("proc doc\n"), export!(foo, 1, block!(inst!(LocLoad(0u16.into())))));
    assert_eq!(context.parse_forms(source)?, forms);
    Ok(())
}

#[test]
fn test_ast_parsing_module_docs_valid() {
    let mut context = TestContext::new();

    let source = source_file!(
        "\
#! Test documentation for the whole module in parsing test. Lorem ipsum dolor sit amet,
#! consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
#!
#! This comment is intentionally longer than 256 characters, since we need to be sure that the size
#! of the comments is correctly parsed. There was a bug here earlier.


#! Test documentation for export procedure foo in parsing test. Lorem ipsum dolor sit amet,
#! consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
#! This comment is intentionally longer than 256 characters, since we need to be sure that the size
#! of the comments is correctly parsed. There was a bug here earlier.
export.foo.1
    loc_load.0
end

#! Test documentation for internal procedure bar in parsing test. Lorem ipsum dolor sit amet,
#! consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna
#! aliqua.
proc.bar.2
    padw
end

#! Test documentation for export procedure baz in parsing test. Lorem ipsum dolor sit amet,
#! consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna
#! aliqua.
export.baz.3
    padw
    push.0
end"
    );

    const MODULE_DOC: &str = "Test documentation for the whole module in parsing test. \
    Lorem ipsum dolor sit amet,\n\
    consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\
    \n\n\
    This comment is intentionally longer than 256 characters, since we need to be sure that the size\n\
    of the comments is correctly parsed. There was a bug here earlier.\n";

    const FOO_DOC: &str = "Test documentation for export procedure foo in parsing test. \
    Lorem ipsum dolor sit amet,\n\
    consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\
    This comment is intentionally longer than 256 characters, since we need to be sure that the size\n\
    of the comments is correctly parsed. There was a bug here earlier.\n";

    const BAR_DOC: &str = "Test documentation for internal procedure bar in parsing test. Lorem ipsum dolor sit amet,\n\
    consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna\n\
    aliqua.\n";

    const BAZ_DOC: &str = "Test documentation for export procedure baz in parsing test. Lorem ipsum dolor sit amet,\n\
    consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna\n\
    aliqua.\n";

    let expected_forms = module!(
        moduledoc!(MODULE_DOC),
        doc!(FOO_DOC),
        export!(foo, 1, block!(inst!(LocLoad(0u16.into())))),
        doc!(BAR_DOC),
        proc!(bar, 2, block!(inst!(PadW))),
        doc!(BAZ_DOC),
        export!(baz, 3, block!(inst!(PadW), inst!(PushU8(0))))
    );

    let actual_forms = context.parse_forms(source.clone()).unwrap();
    assert_eq!(actual_forms, expected_forms);

    let module = context.parse_module(source).unwrap();
    assert_eq!(module.docs(), Some(Span::unknown(MODULE_DOC)));
    let baz = "baz".parse().unwrap();
    let baz_idx = module.index_of_name(&baz).expect("could not find baz");
    let baz_docs = module.get(baz_idx).unwrap().docs();
    assert_eq!(baz_docs, Some(BAZ_DOC));
}

#[test]
fn test_ast_parsing_module_docs_fail() {
    let mut context = TestContext::new();
    let source = source_file!(
        "\
    #! module doc

    #! proc doc
    export.foo.1
        loc_load.0
    end

    #! malformed doc
    "
    );
    assert_module_diagnostic_lines!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "Warning:   ! unused docstring",
        regex!(r#",-\[test[\d]+:8:5\]"#),
        "7 |",
        "8 |     #! malformed doc",
        "  :     ^^^^^^^^^^^^^^^^^",
        "9 |",
        "  `----",
        "help: this docstring is immediately followed by at least one empty line, then another",
        "docstring,if you intended these to be a single docstring, you should remove the empty lines"
    );

    let source = source_file!(
        "\
    #! proc doc
    export.foo.1
        loc_load.0
    end

    #! malformed doc
    "
    );
    assert_module_diagnostic_lines!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "Warning:   ! unused docstring",
        regex!(r#",-\[test[\d]+:6:5\]"#),
        "5 |",
        "6 |     #! malformed doc",
        "  :     ^^^^^^^^^^^^^^^^^",
        "7 |",
        "  `----",
        "help: this docstring is immediately followed by at least one empty line, then another",
        "docstring,if you intended these to be a single docstring, you should remove the empty lines"
    );

    let source = source_file!(
        "\
    #! module doc

    #! malformed doc
    "
    );
    assert_module_diagnostic_lines!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "Warning:   ! unused docstring",
        regex!(r#",-\[test[\d]+:3:5\]"#),
        "2 |",
        "3 |     #! malformed doc",
        "  :     ^^^^^^^^^^^^^^^^^",
        "4 |",
        "  `----",
        "help: this docstring is immediately followed by at least one empty line, then another",
        "docstring,if you intended these to be a single docstring, you should remove the empty lines"
    );

    let source = source_file!(
        "\
    export.foo.1
        loc_load.0
    end

    #! malformed doc
    "
    );
    assert_module_diagnostic_lines!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "Warning:   ! unused docstring",
        regex!(r#",-\[test[\d]+:5:5\]"#),
        "4 |",
        "5 |     #! malformed doc",
        "  :     ^^^^^^^^^^^^^^^^^",
        "6 |",
        "  `----",
        "help: this docstring is immediately followed by at least one empty line, then another",
        "docstring,if you intended these to be a single docstring, you should remove the empty lines"
    );

    let source = source_file!(
        "\
    #! module doc

    export.foo.1
        loc_load.0
    end

    #! malformed doc
    "
    );
    assert_module_diagnostic_lines!(
        context,
        source,
        "syntax error",
        "help: see emitted diagnostics for details",
        "Warning:   ! unused docstring",
        regex!(r#",-\[test[\d]+:7:5\]"#),
        "6 |",
        "7 |     #! malformed doc",
        "  :     ^^^^^^^^^^^^^^^^^",
        "8 |",
        "  `----",
        "help: this docstring is immediately followed by at least one empty line, then another",
        "docstring,if you intended these to be a single docstring, you should remove the empty lines"
    );

    let source = source_file!(
        "\
    #! proc doc
    export.foo.1
        #! malformed doc
        loc_load.0
    end
    "
    );
    assert_module_diagnostic_lines!(
        context,
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:3:9\]"#),
        "2 |     export.foo.1",
        "3 |         #! malformed doc",
        "  :         ^^^^^^^^|^^^^^^^^",
        "  :                 `-- found a doc comment here",
        "4 |         loc_load.0",
        "5 |     end",
        "  `----",
        r#" help: expected primitive opcode (e.g. "add"), or control flow opcode (e.g. "if.true")"#
    );
}

// BEGIN
// ================================================================================================

#[test]
fn assert_parsing_line_unmatched_begin() {
    let source = source_file!(
        "\
        begin
          push.1.2

        add
        mul"
    );
    assert_parse_diagnostic_lines!(
        source,
        "unexpected end of file",
        regex!(r#",-\[test[\d]+:5:12\]"#),
        "4 |         add",
        "5 |         mul",
        "  `----",
        r#"help: expected ".", or primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );
}

#[test]
fn assert_parsing_line_extra_param() {
    let source = source_file!(
        "\
        begin
          add.1.2
        end"
    );
    assert_parse_diagnostic_lines!(
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:2:16\]"#),
        "1 | begin",
        "2 |           add.1.2",
        "  :                |",
        "  :                `-- found a . here",
        "3 |         end",
        "  `----",
        r#" help: expected primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );
}

#[test]
fn assert_parsing_line_invalid_op() {
    let source = source_file!(
        "\
    begin
        repeat.3
            push.1
            push.0.1
        end

        # some comments

        if.true
            and
            loc_store.0
        else
            padw
        end

        # more comments
        # to test if line is correct

        while.true
            push.5.7
            u32wrapping_add
            loc_store.1
            push.0
        end

        repeat.3
            push.2
            u32overflowing_mulx
        end

    end"
    );
    assert_parse_diagnostic_lines!(
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:28:13\]"#),
        "27 |             push.2",
        "28 |             u32overflowing_mulx",
        "   :             ^^^^^^^^^|^^^^^^^^^",
        "   :                      `-- found a identifier here",
        "29 |         end",
        "   `----",
        r#" help: expected ".", or primitive opcode (e.g. "add"), or "end", or control flow opcode (e.g. "if.true")"#
    );
}

#[test]
fn assert_parsing_line_unexpected_token() {
    let source = source_file!(
        "\
    proc.foo
      add
    end

    mul"
    );
    assert_parse_diagnostic_lines!(
        source,
        "invalid syntax",
        regex!(r#",-\[test[\d]+:5:5\]"#),
        "4 |",
        "5 |     mul",
        "  :     ^|^",
        "  :      `-- found a mul here",
        "  `----",
        r#" help: expected "begin", or "const", or "export", or "proc", or "use", or end of file, or doc comment"#
    );
}

// SERIALIZATION AND DESERIALIZATION TESTS
// ================================================================================================

#[cfg(feature = "nope")]
mod serialization {

    #[test]
    fn test_ast_program_serde_simple() {
        let source = "begin push.0xabc234 push.0 assertz end";
        assert_correct_program_serialization(source, true);
    }

    #[test]
    fn test_ast_program_serde_local_procs() {
        let source = "\
    proc.foo.1
        loc_load.0
    end
    proc.bar.2
        padw
    end
    begin
        exec.foo
        exec.bar
    end";
        assert_correct_program_serialization(source, true);
    }

    #[test]
    fn test_ast_program_serde_exported_procs() {
        let source = "\
    export.foo.1
        loc_load.0
    end
    export.bar.2
        padw
    end";
        assert_correct_module_serialization(source, true);
    }

    #[test]
    fn test_ast_program_serde_control_flow() {
        let source = "\
    begin
        repeat.3
            push.1
            push.0.1
        end

        if.true
            and
            loc_store.0
        else
            padw
        end

        while.true
            push.5.7
            u32wrapping_add
            loc_store.1
            push.0
        end

        repeat.3
            push.2
            u32overflowing_mul
        end

    end";
        assert_correct_program_serialization(source, true);
    }

    #[test]
    fn test_ast_program_serde_imports_serialized() {
        let source = "\
    use.std::math::u64
    use.std::crypto::fri

    begin
        push.0
        push.1
        exec.u64::wrapping_add
    end";
        assert_correct_program_serialization(source, true);
    }

    #[test]
    fn test_ast_program_serde_imports_not_serialized() {
        let source = "\
    use.std::math::u64
    use.std::crypto::fri

    begin
        push.0
        push.1
        exec.u64::wrapping_add
    end";
        assert_correct_program_serialization(source, false);
    }

    #[test]
    fn test_ast_module_serde_imports_serialized() {
        let source = "\
    use.std::math::u64
    use.std::crypto::fri

    proc.foo.2
        push.0
        push.1
        exec.u64::wrapping_add
    end";
        assert_correct_module_serialization(source, true);
    }

    #[test]
    fn test_ast_module_serde_imports_not_serialized() {
        let source = "\
    use.std::math::u64
    use.std::crypto::fri

    proc.foo.2
        push.0
        push.1
        exec.u64::wrapping_add
    end";
        assert_correct_module_serialization(source, false);
    }

    #[test]
    fn test_repeat_with_constant_count() {
        let source = "\
    const.A=3
    const.B=A*3+5

    begin
        repeat.A
            push.1
        end

        repeat.B
            push.0
        end
    end";

        assert_correct_program_serialization(source, false);

        let nodes: Vec<Node> = vec![
            Node::Repeat {
                times: 3,
                body: CodeBody::new(vec![Node::Instruction(Instruction::PushU8(1))]),
            },
            Node::Repeat {
                times: 14,
                body: CodeBody::new(vec![Node::Instruction(Instruction::PushU8(0))]),
            },
        ];

        assert_program_output(source, BTreeMap::new(), nodes);
    }

    /// Clears the module's imports.
    ///
    /// Serialization of imports is optional, so if they are not serialized, then they have to be
    /// cleared before testing for equality
    fn clear_imports_module(module: &mut ModuleAst) {
        module.clear_imports();
    }

    /// Clears the program's imports.
    ///
    /// Serialization of imports is optional, so if they are not serialized, then they have to be
    /// cleared before testing for equality
    fn clear_imports_program(program: &mut ProgramAst) {
        program.clear_imports();
    }

    fn assert_correct_program_serialization(source: &str, serialize_imports: bool) {
        let program = ProgramAst::parse(source).unwrap();

        // assert the correct program serialization
        let program_serialized = program.to_bytes(AstSerdeOptions::new(serialize_imports));
        let mut program_deserialized =
            ProgramAst::from_bytes(program_serialized.as_slice()).unwrap();
        let mut clear_program = clear_procs_loc_program(program.clone());
        if !serialize_imports {
            clear_imports_program(&mut clear_program);
        }
        assert_eq!(clear_program, program_deserialized);

        // assert the correct locations serialization
        let mut locations = Vec::new();
        program.write_source_locations(&mut locations);

        // assert empty locations
        {
            let mut locations = program_deserialized.source_locations();
            let start = locations.next().unwrap();
            assert_eq!(start, &SourceLocation::default());
            assert!(locations.next().is_none());
        }

        program_deserialized
            .load_source_locations(&mut SliceReader::new(&locations))
            .unwrap();

        let program_deserialized = if !serialize_imports {
            program_deserialized.with_import_info(program.import_info().clone())
        } else {
            program_deserialized
        };

        assert_eq!(program, program_deserialized);
    }

    fn assert_correct_module_serialization(source: &str, serialize_imports: bool) {
        let module = ModuleAst::parse(source).unwrap();
        let module_serialized = module.to_bytes(AstSerdeOptions::new(serialize_imports));
        let mut module_deserialized = ModuleAst::from_bytes(module_serialized.as_slice()).unwrap();
        let mut clear_module = clear_procs_loc_module(module.clone());
        if !serialize_imports {
            clear_imports_module(&mut clear_module);
        }
        assert_eq!(clear_module, module_deserialized);

        // assert the correct locations serialization
        let mut locations = Vec::new();
        module.write_source_locations(&mut locations);

        // assert module locations are empty
        module_deserialized.procs().iter().for_each(|m| {
            let mut locations = m.source_locations();
            let start = locations.next().unwrap();
            assert_eq!(start, &SourceLocation::default());
            assert!(locations.next().is_none());
        });

        module_deserialized
            .load_source_locations(&mut SliceReader::new(&locations))
            .unwrap();

        module_deserialized = if !serialize_imports {
            module_deserialized.with_import_info(module.import_info().clone())
        } else {
            module_deserialized
        };

        assert_eq!(module, module_deserialized);
    }
}
