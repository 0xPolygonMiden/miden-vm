use alloc::string::String;
use core::{fmt, num::IntErrorKind, str::FromStr};

use logos::{Lexer, Logos};
use vm_core::{Felt, FieldElement, StarkField};

use super::{HexErrorKind, LiteralErrorKind, ParsingError, SourceSpan};

#[derive(Default, Copy, Clone)]
pub struct TokenExtra {
    pub line: usize,
}

#[derive(Debug, Clone, Logos)]
#[logos(error = ParsingError)]
#[logos(extras = TokenExtra)]
#[logos(skip r"[ \t\r]+")]
pub enum Token<'input> {
    #[regex(r"\n", nl)]
    Newline,
    #[token("add")]
    Add,
    #[token("adv.insert_hdword")]
    AdvInsertHdword,
    #[token("adv.insert_hperm")]
    AdvInsertHperm,
    #[token("adv.insert_mem")]
    AdvInsertMem,
    #[token("adv_loadw")]
    AdvLoadw,
    #[token("adv_pipe")]
    AdvPipe,
    #[token("adv_push")]
    AdvPush,
    #[token("adv.push_ext2intt")]
    AdvPushExt2intt,
    #[token("adv.push_mapval")]
    AdvPushMapval,
    #[token("adv.push_mapvaln")]
    AdvPushMapvaln,
    #[token("adv.push_mtnode")]
    AdvPushMtnode,
    #[token("adv.push_sig")]
    AdvPushSig,
    #[token("adv.push_smtpeek")]
    AdvPushSmtpeek,
    #[token("adv.push_smtset")]
    AdvPushSmtset,
    #[token("adv.push_smtget")]
    AdvPushSmtget,
    #[token("adv.push_u64div")]
    AdvPushU64Div,
    #[token("and")]
    And,
    #[token("assert")]
    Assert,
    #[token("assertz")]
    Assertz,
    #[token("assert_eq")]
    AssertEq,
    #[token("assert_eqw")]
    AssertEqw,
    #[token("begin")]
    Begin,
    #[token("caller")]
    Caller,
    #[token("call")]
    Call,
    #[token("cdrop")]
    Cdrop,
    #[token("cdropw")]
    Cdropw,
    #[token("clk")]
    Clk,
    #[token("const")]
    Const,
    #[token("cswap")]
    Cswap,
    #[token("cswapw")]
    Cswapw,
    #[token("debug")]
    Debug,
    #[token("div")]
    Div,
    #[token("drop")]
    Drop,
    #[token("dropw")]
    Dropw,
    #[token("dup")]
    Dup,
    #[token("dupw")]
    Dupw,
    #[token("dynexec")]
    Dynexec,
    #[token("dyncall")]
    Dyncall,
    #[token("else")]
    Else,
    #[token("emit")]
    Emit,
    #[token("end")]
    End,
    #[token("eq")]
    Eq,
    #[token("eqw")]
    Eqw,
    #[token("ext2add")]
    Ext2Add,
    #[token("ext2div")]
    Ext2Div,
    #[token("ext2inv")]
    Ext2Inv,
    #[token("ext2mul")]
    Ext2Mul,
    #[token("ext2neg")]
    Ext2Neg,
    #[token("ext2sub")]
    Ext2Sub,
    #[token("err")]
    Err,
    #[token("exec")]
    Exec,
    #[token("exp")]
    Exp,
    #[token("exp.u")]
    ExpU,
    #[token("export")]
    Export,
    #[token("fri_ext2fold4")]
    FriExt2Fold4,
    #[token("gt")]
    Gt,
    #[token("gte")]
    Gte,
    #[token("hash")]
    Hash,
    #[token("hperm")]
    Hperm,
    #[token("hmerge")]
    Hmerge,
    #[token("if.true")]
    If,
    #[token("ilog2")]
    ILog2,
    #[token("inv")]
    Inv,
    #[token("is_odd")]
    IsOdd,
    #[token("local")]
    Local,
    #[token("locaddr")]
    Locaddr,
    #[token("loc_load")]
    LocLoad,
    #[token("loc_loadw")]
    LocLoadw,
    #[token("loc_store")]
    LocStore,
    #[token("loc_storew")]
    LocStorew,
    #[token("lt")]
    Lt,
    #[token("lte")]
    Lte,
    #[token("mem")]
    Mem,
    #[token("mem_load")]
    MemLoad,
    #[token("mem_loadw")]
    MemLoadw,
    #[token("mem_store")]
    MemStore,
    #[token("mem_storew")]
    MemStorew,
    #[token("mem_stream")]
    MemStream,
    #[token("movdn")]
    Movdn,
    #[token("movdnw")]
    Movdnw,
    #[token("movup")]
    Movup,
    #[token("movupw")]
    Movupw,
    #[token("mtree_get")]
    MtreeGet,
    #[token("mtree_merge")]
    MtreeMerge,
    #[token("mtree_set")]
    MtreeSet,
    #[token("mtree_verify")]
    MtreeVerify,
    #[token("mul")]
    Mul,
    #[token("neg")]
    Neg,
    #[token("neq")]
    Neq,
    #[token("not")]
    Not,
    #[token("or")]
    Or,
    #[token("padw")]
    Padw,
    #[token("pow2")]
    Pow2,
    #[token("proc")]
    Proc,
    #[token("procref")]
    Procref,
    #[token("push")]
    Push,
    #[token("rcomb_base")]
    RCombBase,
    #[token("repeat")]
    Repeat,
    #[token("rpo_falcon512")]
    RpoFalcon512,
    #[token("sdepth")]
    Sdepth,
    #[token("stack")]
    Stack,
    #[token("sub")]
    Sub,
    #[token("swap")]
    Swap,
    #[token("swapw")]
    Swapw,
    #[token("swapdw")]
    Swapdw,
    #[token("syscall")]
    Syscall,
    #[token("trace")]
    Trace,
    #[token("use")]
    Use,
    #[token("u32and")]
    U32And,
    #[token("u32assert")]
    U32Assert,
    #[token("u32assert2")]
    U32Assert2,
    #[token("u32assertw")]
    U32Assertw,
    #[token("u32cast")]
    U32Cast,
    #[token("u32div")]
    U32Div,
    #[token("u32divmod")]
    U32Divmod,
    #[token("u32gt")]
    U32Gt,
    #[token("u32gte")]
    U32Gte,
    #[token("u32lt")]
    U32Lt,
    #[token("u32lte")]
    U32Lte,
    #[token("u32max")]
    U32Max,
    #[token("u32min")]
    U32Min,
    #[token("u32mod")]
    U32Mod,
    #[token("u32not")]
    U32Not,
    #[token("u32or")]
    U32Or,
    #[token("u32overflowing_add")]
    U32OverflowingAdd,
    #[token("u32overflowing_add3")]
    U32OverflowingAdd3,
    #[token("u32overflowing_madd")]
    U32OverflowingMadd,
    #[token("u32overflowing_mul")]
    U32OverflowingMul,
    #[token("u32overflowing_sub")]
    U32OverflowingSub,
    #[token("u32popcnt")]
    U32Popcnt,
    #[token("u32clz")]
    U32Clz,
    #[token("u32ctz")]
    U32Ctz,
    #[token("u32clo")]
    U32Clo,
    #[token("u32cto")]
    U32Cto,
    #[token("u32rotl")]
    U32Rotl,
    #[token("u32rotr")]
    U32Rotr,
    #[token("u32shl")]
    U32Shl,
    #[token("u32shr")]
    U32Shr,
    #[token("u32split")]
    U32Split,
    #[token("u32test")]
    U32Test,
    #[token("u32testw")]
    U32Testw,
    #[token("u32wrapping_add")]
    U32WrappingAdd,
    #[token("u32wrapping_add3")]
    U32WrappingAdd3,
    #[token("u32wrapping_madd")]
    U32WrappingMadd,
    #[token("u32wrapping_mul")]
    U32WrappingMul,
    #[token("u32wrapping_sub")]
    U32WrappingSub,
    #[token("u32xor")]
    U32Xor,
    #[token("while.true")]
    While,
    #[token("xor")]
    Xor,
    #[token("!")]
    Bang,
    #[token("::")]
    ColonColon,
    #[token(".")]
    Dot,
    #[token("=")]
    Equal,
    #[token("(")]
    Lparen,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("//")]
    SlashSlash,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,
    #[token(")")]
    Rparen,
    #[token("->")]
    Rstab,
    #[regex(r"(#![^\n]*\n)+", parse_docs)]
    DocComment(DocumentationType),
    #[regex(r"0x[A-Fa-f0-9]+", parse_hex)]
    HexValue(HexEncodedValue),
    #[regex(r"[0-9]+", parse_int)]
    Int(u64),
    #[regex(r"[a-z_][a-zA-Z0-9_$]*")]
    Ident(&'input str),
    #[regex(r"[A-Z][A-Z0-9_]*")]
    ConstantIdent(&'input str),
    #[regex(r#""[A-Za-z_.$][^"\n]*["\n]"#, unquote)]
    QuotedIdent(&'input str),
    #[regex(r#"[[:cntrl:]]"#, priority = 0)]
    Unknown,
    #[regex(r"#[^\n]*\n?", logos::skip)]
    #[end]
    Eof,
}

#[derive(Debug, Clone)]
pub enum DocumentationType {
    Module(String),
    Form(String),
}
impl From<DocumentationType> for String {
    fn from(doc: DocumentationType) -> Self {
        match doc {
            DocumentationType::Module(s) => s,
            DocumentationType::Form(s) => s,
        }
    }
}
impl core::ops::Deref for DocumentationType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Module(ref s) => s,
            Self::Form(ref s) => s,
        }
    }
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Newline => write!(f, "\\n"),
            Token::Add => write!(f, "add"),
            Token::AdvInsertHdword => write!(f, "adv.insert_hdword"),
            Token::AdvInsertHperm => write!(f, "adv.insert_hperm"),
            Token::AdvInsertMem => write!(f, "adv.insert_mem"),
            Token::AdvLoadw => write!(f, "adv_loadw"),
            Token::AdvPipe => write!(f, "adv_pipe"),
            Token::AdvPush => write!(f, "adv_push"),
            Token::AdvPushExt2intt => write!(f, "adv.push_ext2intt"),
            Token::AdvPushMapval => write!(f, "adv.push_mapval"),
            Token::AdvPushMapvaln => write!(f, "adv.push_mapvaln"),
            Token::AdvPushMtnode => write!(f, "adv.push_mtnode"),
            Token::AdvPushSig => write!(f, "adv.push_sig"),
            Token::AdvPushSmtpeek => write!(f, "adv.push_smtpeek"),
            Token::AdvPushSmtset => write!(f, "adv.push_smtset"),
            Token::AdvPushSmtget => write!(f, "adv.push_smtget"),
            Token::AdvPushU64Div => write!(f, "adv.push_u64div"),
            Token::And => write!(f, "and"),
            Token::Assert => write!(f, "assert"),
            Token::Assertz => write!(f, "assertz"),
            Token::AssertEq => write!(f, "assert_eq"),
            Token::AssertEqw => write!(f, "assert_eqw"),
            Token::Begin => write!(f, "begin"),
            Token::Caller => write!(f, "caller"),
            Token::Call => write!(f, "call"),
            Token::Cdrop => write!(f, "cdrop"),
            Token::Cdropw => write!(f, "cdropw"),
            Token::Clk => write!(f, "clk"),
            Token::Const => write!(f, "const"),
            Token::Cswap => write!(f, "cswap"),
            Token::Cswapw => write!(f, "cswapw"),
            Token::Debug => write!(f, "debug"),
            Token::Div => write!(f, "div"),
            Token::Drop => write!(f, "drop"),
            Token::Dropw => write!(f, "dropw"),
            Token::Dup => write!(f, "dup"),
            Token::Dupw => write!(f, "dupw"),
            Token::Dynexec => write!(f, "dynexec"),
            Token::Dyncall => write!(f, "dyncall"),
            Token::Else => write!(f, "else"),
            Token::Emit => write!(f, "emit"),
            Token::End => write!(f, "end"),
            Token::Eq => write!(f, "eq"),
            Token::Eqw => write!(f, "eqw"),
            Token::Ext2Add => write!(f, "ext2add"),
            Token::Ext2Div => write!(f, "ext2div"),
            Token::Ext2Inv => write!(f, "ext2inv"),
            Token::Ext2Mul => write!(f, "ext2mul"),
            Token::Ext2Neg => write!(f, "ext2neg"),
            Token::Ext2Sub => write!(f, "ext2sub"),
            Token::Err => write!(f, "err"),
            Token::Exec => write!(f, "exec"),
            Token::Exp => write!(f, "exp"),
            Token::ExpU => write!(f, "exp.u"),
            Token::Export => write!(f, "export"),
            Token::FriExt2Fold4 => write!(f, "fri_ext2fold4"),
            Token::Gt => write!(f, "gt"),
            Token::Gte => write!(f, "gte"),
            Token::Hash => write!(f, "hash"),
            Token::Hperm => write!(f, "hperm"),
            Token::Hmerge => write!(f, "hmerge"),
            Token::If => write!(f, "if.true"),
            Token::ILog2 => write!(f, "ilog2"),
            Token::Inv => write!(f, "inv"),
            Token::IsOdd => write!(f, "is_odd"),
            Token::Local => write!(f, "local"),
            Token::Locaddr => write!(f, "locaddr"),
            Token::LocLoad => write!(f, "loc_load"),
            Token::LocLoadw => write!(f, "loc_loadw"),
            Token::LocStore => write!(f, "loc_store"),
            Token::LocStorew => write!(f, "loc_storew"),
            Token::Lt => write!(f, "lt"),
            Token::Lte => write!(f, "lte"),
            Token::Mem => write!(f, "mem"),
            Token::MemLoad => write!(f, "mem_load"),
            Token::MemLoadw => write!(f, "mem_loadw"),
            Token::MemStore => write!(f, "mem_store"),
            Token::MemStorew => write!(f, "mem_storew"),
            Token::MemStream => write!(f, "mem_stream"),
            Token::Movdn => write!(f, "movdn"),
            Token::Movdnw => write!(f, "movdnw"),
            Token::Movup => write!(f, "movup"),
            Token::Movupw => write!(f, "movupw"),
            Token::MtreeGet => write!(f, "mtree_get"),
            Token::MtreeMerge => write!(f, "mtree_merge"),
            Token::MtreeSet => write!(f, "mtree_set"),
            Token::MtreeVerify => write!(f, "mtree_verify"),
            Token::Mul => write!(f, "mul"),
            Token::Neg => write!(f, "neg"),
            Token::Neq => write!(f, "neq"),
            Token::Not => write!(f, "not"),
            Token::Or => write!(f, "or"),
            Token::Padw => write!(f, "padw"),
            Token::Pow2 => write!(f, "pow2"),
            Token::Proc => write!(f, "proc"),
            Token::Procref => write!(f, "procref"),
            Token::Push => write!(f, "push"),
            Token::RCombBase => write!(f, "rcomb_base"),
            Token::Repeat => write!(f, "repeat"),
            Token::RpoFalcon512 => write!(f, "rpo_falcon512"),
            Token::Sdepth => write!(f, "sdepth"),
            Token::Stack => write!(f, "stack"),
            Token::Sub => write!(f, "sub"),
            Token::Swap => write!(f, "swap"),
            Token::Swapw => write!(f, "swapw"),
            Token::Swapdw => write!(f, "swapdw"),
            Token::Syscall => write!(f, "syscall"),
            Token::Trace => write!(f, "trace"),
            Token::Use => write!(f, "use"),
            Token::U32And => write!(f, "u32and"),
            Token::U32Assert => write!(f, "u32assert"),
            Token::U32Assert2 => write!(f, "u32assert2"),
            Token::U32Assertw => write!(f, "u32assertw"),
            Token::U32Cast => write!(f, "u32cast"),
            Token::U32Div => write!(f, "u32div"),
            Token::U32Divmod => write!(f, "u32divmod"),
            Token::U32Gt => write!(f, "u32gt"),
            Token::U32Gte => write!(f, "u32gte"),
            Token::U32Lt => write!(f, "u32lt"),
            Token::U32Lte => write!(f, "u32lte"),
            Token::U32Max => write!(f, "u32max"),
            Token::U32Min => write!(f, "u32min"),
            Token::U32Mod => write!(f, "u32mod"),
            Token::U32Not => write!(f, "u32not"),
            Token::U32Or => write!(f, "u32or"),
            Token::U32OverflowingAdd => write!(f, "u32overflowing_add"),
            Token::U32OverflowingAdd3 => write!(f, "u32overflowing_add3"),
            Token::U32OverflowingMadd => write!(f, "u32overflowing_madd"),
            Token::U32OverflowingMul => write!(f, "u32overflowing_mul"),
            Token::U32OverflowingSub => write!(f, "u32overflowing_sub"),
            Token::U32Popcnt => write!(f, "u32popcnt"),
            Token::U32Clz => write!(f, "u32clz"),
            Token::U32Ctz => write!(f, "u32ctz"),
            Token::U32Clo => write!(f, "u32clo"),
            Token::U32Cto => write!(f, "u32cto"),
            Token::U32Rotl => write!(f, "u32rotl"),
            Token::U32Rotr => write!(f, "u32rotr"),
            Token::U32Shl => write!(f, "u32shl"),
            Token::U32Shr => write!(f, "u32shr"),
            Token::U32Split => write!(f, "u32split"),
            Token::U32Test => write!(f, "u32test"),
            Token::U32Testw => write!(f, "u32testw"),
            Token::U32WrappingAdd => write!(f, "u32wrapping_add"),
            Token::U32WrappingAdd3 => write!(f, "u32wrapping_add3"),
            Token::U32WrappingMadd => write!(f, "u32wrapping_madd"),
            Token::U32WrappingMul => write!(f, "u32wrapping_mul"),
            Token::U32WrappingSub => write!(f, "u32wrapping_sub"),
            Token::U32Xor => write!(f, "u32xor"),
            Token::While => write!(f, "while.true"),
            Token::Xor => write!(f, "xor"),
            Token::Bang => write!(f, "!"),
            Token::ColonColon => write!(f, "::"),
            Token::Dot => write!(f, "."),
            Token::Equal => write!(f, "="),
            Token::Lparen => write!(f, "("),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::SlashSlash => write!(f, "//"),
            Token::Slash => write!(f, "/"),
            Token::Star => write!(f, "*"),
            Token::Rparen => write!(f, ")"),
            Token::Rstab => write!(f, "->"),
            Token::DocComment(DocumentationType::Module(_)) => f.write_str("module doc"),
            Token::DocComment(DocumentationType::Form(_)) => f.write_str("doc comment"),
            Token::HexValue(_) => f.write_str("hex-encoded value"),
            Token::Int(_) => f.write_str("integer"),
            Token::Ident(_) => f.write_str("identifier"),
            Token::ConstantIdent(_) => f.write_str("constant identifier"),
            Token::QuotedIdent(_) => f.write_str("quoted identifier"),
            Token::Unknown => f.write_str("invalid character"),
            Token::Eof => write!(f, "end of file"),
        }
    }
}

/// Represents one of the various types of values
/// that have a hex-encoded representation in Miden
/// Assembly source files.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HexEncodedValue {
    /// A tiny value
    U8(u8),
    /// A small value
    U16(u16),
    /// A u32 constant, typically represents a memory address
    U32(u32),
    /// A single field element, 8 bytes, encoded as 16 hex digits
    Felt(Felt),
    /// A set of 4 field elements, 32 bytes, encoded as a contiguous string of 64 hex digits
    Word([Felt; 4]),
}

#[inline]
fn nl<'input>(lexer: &mut Lexer<'input, Token<'input>>) -> logos::Skip {
    lexer.extras.line += 1;
    logos::Skip
}

#[inline]
fn unquote<'input>(lexer: &mut Lexer<'input, Token<'input>>) -> Result<&'input str, ParsingError> {
    let span = SourceSpan::try_from(lexer.span()).expect("invalid span: file too large");
    let tok = &lexer.slice()[1..];
    if let Some(unquoted) = tok.strip_suffix('"') {
        Ok(unquoted)
    } else {
        // We reached a newline before finding a closing quote, notify user
        Err(ParsingError::UnclosedQuotedIdentifier { span })
    }
}

fn parse_hex<'input>(
    lexer: &mut Lexer<'input, Token<'input>>,
) -> Result<HexEncodedValue, ParsingError> {
    let span = SourceSpan::try_from(lexer.span()).expect("invalid span: file too large");
    let hex_digits = &lexer.slice()[2..];
    match hex_digits.len() {
        n if n <= 16 && n.is_power_of_two() => {
            let value = u64::from_str_radix(hex_digits, 16).map_err(|error| {
                ParsingError::InvalidLiteral {
                    span,
                    kind: int_error_kind_to_literal_error_kind(
                        error.kind(),
                        LiteralErrorKind::FeltOverflow,
                    ),
                }
            })?;
            if value > Felt::MODULUS {
                return Err(ParsingError::InvalidLiteral {
                    span,
                    kind: LiteralErrorKind::FeltOverflow,
                });
            }
            Ok(shrink_u64(value))
        }
        // Word
        64 => {
            let mut word = [Felt::ZERO; 4];
            for (index, element) in word.iter_mut().enumerate() {
                let offset = index * 16;
                let mut felt_bytes = [0u8; 8];
                let digits = &hex_digits[offset..(offset + 16)];
                for (byte_idx, byte) in felt_bytes.iter_mut().enumerate() {
                    let byte_str = &digits[(byte_idx * 2)..((byte_idx * 2) + 2)];
                    *byte = u8::from_str_radix(byte_str, 16).map_err(|error| {
                        ParsingError::InvalidLiteral {
                            span,
                            kind: int_error_kind_to_literal_error_kind(
                                error.kind(),
                                LiteralErrorKind::FeltOverflow,
                            ),
                        }
                    })?;
                }
                let value = u64::from_le_bytes(felt_bytes);
                if value > Felt::MODULUS {
                    return Err(ParsingError::InvalidLiteral {
                        span,
                        kind: LiteralErrorKind::FeltOverflow,
                    });
                }
                *element = Felt::new(value);
            }
            Ok(HexEncodedValue::Word(word))
        }
        // Invalid
        n if n > 64 => Err(ParsingError::InvalidHexLiteral {
            span,
            kind: HexErrorKind::TooLong,
        }),
        n if !n.is_power_of_two() && n < 64 => Err(ParsingError::InvalidHexLiteral {
            span,
            kind: HexErrorKind::MissingDigits,
        }),
        _ => Err(ParsingError::InvalidHexLiteral {
            span,
            kind: HexErrorKind::Invalid,
        }),
    }
}

#[inline]
fn shrink_u64(n: u64) -> HexEncodedValue {
    if n <= (u8::MAX as u64) {
        HexEncodedValue::U8(n as u8)
    } else if n <= (u16::MAX as u64) {
        HexEncodedValue::U16(n as u16)
    } else if n <= (u32::MAX as u64) {
        HexEncodedValue::U32(n as u32)
    } else {
        HexEncodedValue::Felt(Felt::new(n))
    }
}

/// Parses a sequence of lines starting with `#!` into a single string without the
/// doc comment prefix, and without extraneous whitespace.
///
/// Lines are concatenated together unless an empty `#!` line is encountered, in which
/// case a newline is inserted for that line. This preserves paragaraphs, but removes
/// line breaks introduced just to keep the line length to 80 characters (or whatever)
fn parse_docs<'input>(lexer: &mut Lexer<'input, Token<'input>>) -> Option<DocumentationType> {
    let is_moduledoc = lexer.extras.line == 0;
    let raw = lexer.slice();
    let mut docs = String::with_capacity(raw.len());
    let mut started = false;
    for line in raw.lines() {
        let line = line.strip_prefix("#!").unwrap_or(line).trim();
        if line.is_empty() && !started {
            continue;
        }
        if !started {
            started = true;
        }
        docs.push_str(line);
        docs.push('\n');
        lexer.extras.line += 1;
    }
    Some(if is_moduledoc {
        DocumentationType::Module(docs)
    } else {
        DocumentationType::Form(docs)
    })
}

/// Parses an unsigned integer in decimal format
fn parse_int<'input>(lexer: &mut Lexer<'input, Token<'input>>) -> Result<u64, ParsingError> {
    lexer.slice().parse::<u64>().map_err(|error| ParsingError::InvalidLiteral {
        span: SourceSpan::try_from(lexer.span()).expect("invalid span: source file too large"),
        kind: int_error_kind_to_literal_error_kind(error.kind(), LiteralErrorKind::FeltOverflow),
    })
}

#[inline]
fn int_error_kind_to_literal_error_kind(
    kind: &IntErrorKind,
    overflow: LiteralErrorKind,
) -> LiteralErrorKind {
    match kind {
        IntErrorKind::Empty => LiteralErrorKind::Empty,
        IntErrorKind::InvalidDigit => LiteralErrorKind::InvalidDigit,
        IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => overflow,
        _ => unreachable!(),
    }
}

impl Token<'_> {
    pub fn is_instruction(&self) -> bool {
        matches!(
            self,
            Token::Add
                | Token::AdvInsertHdword
                | Token::AdvInsertHperm
                | Token::AdvInsertMem
                | Token::AdvLoadw
                | Token::AdvPipe
                | Token::AdvPush
                | Token::AdvPushExt2intt
                | Token::AdvPushMapval
                | Token::AdvPushMapvaln
                | Token::AdvPushMtnode
                | Token::AdvPushSig
                | Token::AdvPushSmtpeek
                | Token::AdvPushSmtset
                | Token::AdvPushSmtget
                | Token::AdvPushU64Div
                | Token::And
                | Token::Assert
                | Token::Assertz
                | Token::AssertEq
                | Token::AssertEqw
                | Token::Caller
                | Token::Call
                | Token::Cdrop
                | Token::Cdropw
                | Token::Clk
                | Token::Cswap
                | Token::Cswapw
                | Token::Debug
                | Token::Div
                | Token::Drop
                | Token::Dropw
                | Token::Dup
                | Token::Dupw
                | Token::Dynexec
                | Token::Dyncall
                | Token::Emit
                | Token::Eq
                | Token::Eqw
                | Token::Ext2Add
                | Token::Ext2Div
                | Token::Ext2Inv
                | Token::Ext2Mul
                | Token::Ext2Neg
                | Token::Ext2Sub
                | Token::Exec
                | Token::Exp
                | Token::ExpU
                | Token::FriExt2Fold4
                | Token::Gt
                | Token::Gte
                | Token::Hash
                | Token::Hperm
                | Token::Hmerge
                | Token::ILog2
                | Token::Inv
                | Token::IsOdd
                | Token::Local
                | Token::Locaddr
                | Token::LocLoad
                | Token::LocLoadw
                | Token::LocStore
                | Token::LocStorew
                | Token::Lt
                | Token::Lte
                | Token::Mem
                | Token::MemLoad
                | Token::MemLoadw
                | Token::MemStore
                | Token::MemStorew
                | Token::MemStream
                | Token::Movdn
                | Token::Movdnw
                | Token::Movup
                | Token::Movupw
                | Token::MtreeGet
                | Token::MtreeMerge
                | Token::MtreeSet
                | Token::MtreeVerify
                | Token::Mul
                | Token::Neg
                | Token::Neq
                | Token::Not
                | Token::Or
                | Token::Padw
                | Token::Pow2
                | Token::Procref
                | Token::Push
                | Token::RCombBase
                | Token::Repeat
                | Token::Sdepth
                | Token::Stack
                | Token::Sub
                | Token::Swap
                | Token::Swapw
                | Token::Swapdw
                | Token::Syscall
                | Token::Trace
                | Token::U32And
                | Token::U32Assert
                | Token::U32Assert2
                | Token::U32Assertw
                | Token::U32Cast
                | Token::U32Div
                | Token::U32Divmod
                | Token::U32Gt
                | Token::U32Gte
                | Token::U32Lt
                | Token::U32Lte
                | Token::U32Max
                | Token::U32Min
                | Token::U32Mod
                | Token::U32Not
                | Token::U32Or
                | Token::U32OverflowingAdd
                | Token::U32OverflowingAdd3
                | Token::U32OverflowingMadd
                | Token::U32OverflowingMul
                | Token::U32OverflowingSub
                | Token::U32Popcnt
                | Token::U32Clz
                | Token::U32Ctz
                | Token::U32Clo
                | Token::U32Cto
                | Token::U32Rotl
                | Token::U32Rotr
                | Token::U32Shl
                | Token::U32Shr
                | Token::U32Split
                | Token::U32Test
                | Token::U32Testw
                | Token::U32WrappingAdd
                | Token::U32WrappingAdd3
                | Token::U32WrappingMadd
                | Token::U32WrappingMul
                | Token::U32WrappingSub
                | Token::U32Xor
                | Token::Xor
        )
    }
}

impl<'input> FromStr for Token<'input> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "\\n" => Ok(Token::Newline),
            "add" => Ok(Token::Add),
            "adv.insert_hdword" => Ok(Token::AdvInsertHdword),
            "adv.insert_hperm" => Ok(Token::AdvInsertHperm),
            "adv.insert_mem" => Ok(Token::AdvInsertMem),
            "adv_loadw" => Ok(Token::AdvLoadw),
            "adv_pipe" => Ok(Token::AdvPipe),
            "adv_push" => Ok(Token::AdvPush),
            "adv.push_ext2intt" => Ok(Token::AdvPushExt2intt),
            "adv.push_mapval" => Ok(Token::AdvPushMapval),
            "adv.push_mapvaln" => Ok(Token::AdvPushMapvaln),
            "adv.push_mtnode" => Ok(Token::AdvPushMtnode),
            "adv.push_sig" => Ok(Token::AdvPushSig),
            "adv.push_smtpeek" => Ok(Token::AdvPushSmtpeek),
            "adv.push_smtset" => Ok(Token::AdvPushSmtset),
            "adv.push_smtget" => Ok(Token::AdvPushSmtget),
            "adv.push_u64div" => Ok(Token::AdvPushU64Div),
            "and" => Ok(Token::And),
            "assert" => Ok(Token::Assert),
            "assertz" => Ok(Token::Assertz),
            "assert_eq" => Ok(Token::AssertEq),
            "assert_eqw" => Ok(Token::AssertEqw),
            "begin" => Ok(Token::Begin),
            "caller" => Ok(Token::Caller),
            "call" => Ok(Token::Call),
            "cdrop" => Ok(Token::Cdrop),
            "cdropw" => Ok(Token::Cdropw),
            "clk" => Ok(Token::Clk),
            "const" => Ok(Token::Const),
            "cswap" => Ok(Token::Cswap),
            "cswapw" => Ok(Token::Cswapw),
            "debug" => Ok(Token::Debug),
            "div" => Ok(Token::Div),
            "drop" => Ok(Token::Drop),
            "dropw" => Ok(Token::Dropw),
            "dup" => Ok(Token::Dup),
            "dupw" => Ok(Token::Dupw),
            "dynexec" => Ok(Token::Dynexec),
            "dyncall" => Ok(Token::Dyncall),
            "else" => Ok(Token::Else),
            "emit" => Ok(Token::Emit),
            "end" => Ok(Token::End),
            "eq" => Ok(Token::Eq),
            "eqw" => Ok(Token::Eqw),
            "ext2add" => Ok(Token::Ext2Add),
            "ext2div" => Ok(Token::Ext2Div),
            "ext2inv" => Ok(Token::Ext2Inv),
            "ext2mul" => Ok(Token::Ext2Mul),
            "ext2neg" => Ok(Token::Ext2Neg),
            "ext2sub" => Ok(Token::Ext2Sub),
            "err" => Ok(Token::Err),
            "exec" => Ok(Token::Exec),
            "exp" => Ok(Token::Exp),
            "exp.u" => Ok(Token::ExpU),
            "export" => Ok(Token::Export),
            "fri_ext2fold4" => Ok(Token::FriExt2Fold4),
            "gt" => Ok(Token::Gt),
            "gte" => Ok(Token::Gte),
            "hash" => Ok(Token::Hash),
            "hperm" => Ok(Token::Hperm),
            "hmerge" => Ok(Token::Hmerge),
            "if.true" => Ok(Token::If),
            "ilog2" => Ok(Token::ILog2),
            "inv" => Ok(Token::Inv),
            "is_odd" => Ok(Token::IsOdd),
            "local" => Ok(Token::Local),
            "locaddr" => Ok(Token::Locaddr),
            "loc_load" => Ok(Token::LocLoad),
            "loc_loadw" => Ok(Token::LocLoadw),
            "loc_store" => Ok(Token::LocStore),
            "loc_storew" => Ok(Token::LocStorew),
            "lt" => Ok(Token::Lt),
            "lte" => Ok(Token::Lte),
            "mem" => Ok(Token::Mem),
            "mem_load" => Ok(Token::MemLoad),
            "mem_loadw" => Ok(Token::MemLoadw),
            "mem_store" => Ok(Token::MemStore),
            "mem_storew" => Ok(Token::MemStorew),
            "mem_stream" => Ok(Token::MemStream),
            "movdn" => Ok(Token::Movdn),
            "movdnw" => Ok(Token::Movdnw),
            "movup" => Ok(Token::Movup),
            "movupw" => Ok(Token::Movupw),
            "mtree_get" => Ok(Token::MtreeGet),
            "mtree_merge" => Ok(Token::MtreeMerge),
            "mtree_set" => Ok(Token::MtreeSet),
            "mtree_verify" => Ok(Token::MtreeVerify),
            "mul" => Ok(Token::Mul),
            "neg" => Ok(Token::Neg),
            "neq" => Ok(Token::Neq),
            "not" => Ok(Token::Not),
            "or" => Ok(Token::Or),
            "padw" => Ok(Token::Padw),
            "pow2" => Ok(Token::Pow2),
            "proc" => Ok(Token::Proc),
            "procref" => Ok(Token::Procref),
            "push" => Ok(Token::Push),
            "rcomb_base" => Ok(Token::RCombBase),
            "repeat" => Ok(Token::Repeat),
            "rpo_falcon512" => Ok(Token::RpoFalcon512),
            "sdepth" => Ok(Token::Sdepth),
            "stack" => Ok(Token::Stack),
            "sub" => Ok(Token::Sub),
            "swap" => Ok(Token::Swap),
            "swapw" => Ok(Token::Swapw),
            "swapdw" => Ok(Token::Swapdw),
            "syscall" => Ok(Token::Syscall),
            "trace" => Ok(Token::Trace),
            "use" => Ok(Token::Use),
            "u32and" => Ok(Token::U32And),
            "u32assert" => Ok(Token::U32Assert),
            "u32assert2" => Ok(Token::U32Assert2),
            "u32assertw" => Ok(Token::U32Assertw),
            "u32cast" => Ok(Token::U32Cast),
            "u32div" => Ok(Token::U32Div),
            "u32divmod" => Ok(Token::U32Divmod),
            "u32gt" => Ok(Token::U32Gt),
            "u32gte" => Ok(Token::U32Gte),
            "u32lt" => Ok(Token::U32Lt),
            "u32lte" => Ok(Token::U32Lte),
            "u32max" => Ok(Token::U32Max),
            "u32min" => Ok(Token::U32Min),
            "u32mod" => Ok(Token::U32Mod),
            "u32not" => Ok(Token::U32Not),
            "u32or" => Ok(Token::U32Or),
            "u32overflowing_add" => Ok(Token::U32OverflowingAdd),
            "u32overflowing_add3" => Ok(Token::U32OverflowingAdd3),
            "u32overflowing_madd" => Ok(Token::U32OverflowingMadd),
            "u32overflowing_mul" => Ok(Token::U32OverflowingMul),
            "u32overflowing_sub" => Ok(Token::U32OverflowingSub),
            "u32popcnt" => Ok(Token::U32Popcnt),
            "u32clz" => Ok(Token::U32Clz),
            "u32ctz" => Ok(Token::U32Ctz),
            "u32clo" => Ok(Token::U32Clo),
            "u32cto" => Ok(Token::U32Cto),
            "u32rotl" => Ok(Token::U32Rotl),
            "u32rotr" => Ok(Token::U32Rotr),
            "u32shl" => Ok(Token::U32Shl),
            "u32shr" => Ok(Token::U32Shr),
            "u32split" => Ok(Token::U32Split),
            "u32test" => Ok(Token::U32Test),
            "u32testw" => Ok(Token::U32Testw),
            "u32wrapping_add" => Ok(Token::U32WrappingAdd),
            "u32wrapping_add3" => Ok(Token::U32WrappingAdd3),
            "u32wrapping_madd" => Ok(Token::U32WrappingMadd),
            "u32wrapping_mul" => Ok(Token::U32WrappingMul),
            "u32wrapping_sub" => Ok(Token::U32WrappingSub),
            "u32xor" => Ok(Token::U32Xor),
            "while.true" => Ok(Token::While),
            "xor" => Ok(Token::Xor),
            "!" => Ok(Token::Bang),
            "::" => Ok(Token::ColonColon),
            "." => Ok(Token::Dot),
            "=" => Ok(Token::Equal),
            "(" => Ok(Token::Lparen),
            "-" => Ok(Token::Minus),
            "+" => Ok(Token::Plus),
            "//" => Ok(Token::SlashSlash),
            "/" => Ok(Token::Slash),
            "*" => Ok(Token::Star),
            ")" => Ok(Token::Rparen),
            "->" => Ok(Token::Rstab),
            "end of file" => Ok(Token::Eof),
            "module doc" => Ok(Token::DocComment(DocumentationType::Module(String::new()))),
            "doc comment" => Ok(Token::DocComment(DocumentationType::Form(String::new()))),
            "hex-encoded value" => Ok(Token::HexValue(HexEncodedValue::U8(0))),
            "integer" => Ok(Token::Int(0)),
            "identifier" => Ok(Token::Ident("")),
            "constant identifier" => Ok(Token::ConstantIdent("")),
            "quoted identifier" => Ok(Token::QuotedIdent("")),
            "invalid character" => Ok(Token::Unknown),
            _ => Err(()),
        }
    }
}
