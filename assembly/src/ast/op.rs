use core::fmt;

use super::{Block, Instruction};
use crate::{
    ast::AstSerdeOptions, ByteReader, ByteWriter, Deserializable, DeserializationError,
    Serializable, SourceSpan, Span, Spanned,
};

/// Represents the Miden Assembly instruction set syntax
///
/// This is separate from [Instruction] in order to distinguish control flow instructions and
/// instructions with block regions from the rest.
#[derive(Clone)]
#[repr(u8)]
pub enum Op {
    /// Represents a conditional branch
    ///
    /// Can be either `if`..`end`, or `if`..`else`..`end`.
    If {
        span: SourceSpan,
        /// This block is always present and non-empty
        then_blk: Block,
        /// This block will be empty if no `else` branch was present
        else_blk: Block,
    } = 0,
    /// Represents a condition-controlled loop
    While { span: SourceSpan, body: Block } = 1,
    /// Represents a counter-controlled loop.
    ///
    /// NOTE: The iteration count must be known at compile-time, so this is _not_ used for general
    /// `for`-style loops where the iteration count is dynamic.
    Repeat {
        span: SourceSpan,
        count: u32,
        body: Block,
    } = 2,
    /// A primitive operation, e.g. `add`
    Inst(Span<Instruction>) = 3,
}

/// Serialization
impl Op {
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        if options.debug_info {
            self.span().write_into(target);
        }
        target.write_u8(self.tag());
        match self {
            Self::If {
                ref then_blk,
                ref else_blk,
                ..
            } => {
                then_blk.write_into_with_options(target, options);
                else_blk.write_into_with_options(target, options);
            }
            Self::While { ref body, .. } => {
                body.write_into_with_options(target, options);
            }
            Self::Repeat {
                count, ref body, ..
            } => {
                target.write_u32(*count);
                body.write_into_with_options(target, options);
            }
            Self::Inst(ref inst) => {
                (**inst).write_into(target);
            }
        }
    }

    pub fn read_from_with_options<R: ByteReader>(
        source: &mut R,
        options: AstSerdeOptions,
    ) -> Result<Self, DeserializationError> {
        let span = if options.debug_info {
            SourceSpan::read_from(source)?
        } else {
            SourceSpan::default()
        };
        match source.read_u8()? {
            0 => {
                let then_blk = Block::read_from_with_options(source, options)?;
                let else_blk = Block::read_from_with_options(source, options)?;
                Ok(Self::If {
                    span,
                    then_blk,
                    else_blk,
                })
            }
            1 => {
                let body = Block::read_from_with_options(source, options)?;
                Ok(Self::While { span, body })
            }
            2 => {
                let count = source.read_u32()?;
                let body = Block::read_from_with_options(source, options)?;
                Ok(Self::Repeat { span, count, body })
            }
            3 => {
                let inst = Instruction::read_from(source)?;
                Ok(Self::Inst(Span::new(span, inst)))
            }
            n => Err(DeserializationError::InvalidValue(format!("{n} is not a valid op tag"))),
        }
    }

    fn tag(&self) -> u8 {
        // SAFETY: This is safe because we have given this enum a
        // primitive representation with #[repr(u8)], with the first
        // field of the underlying union-of-structs the discriminant.
        //
        // See the section on "accessing the numeric value of the discriminant"
        // here: https://doc.rust-lang.org/std/mem/fn.discriminant.html
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

#[cfg(feature = "formatter")]
impl crate::prettier::PrettyPrint for Op {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        match self {
            Self::If {
                ref then_blk,
                ref else_blk,
                ..
            } => {
                text("if.true")
                    + nl()
                    + then_blk.render()
                    + nl()
                    + text("else")
                    + nl()
                    + else_blk.render()
                    + nl()
                    + text("end")
            }
            Self::While { ref body, .. } => {
                text("while.true") + nl() + body.render() + nl() + text("end")
            }
            Self::Repeat {
                count, ref body, ..
            } => display(format!("repeat.{count}")) + nl() + body.render() + nl() + text("end"),
            Self::Inst(ref inst) => inst.render(),
        }
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::If {
                ref then_blk,
                ref else_blk,
                ..
            } => f.debug_struct("If").field("then", then_blk).field("else", else_blk).finish(),
            Self::While { ref body, .. } => f.debug_tuple("While").field(body).finish(),
            Self::Repeat {
                ref count,
                ref body,
                ..
            } => f.debug_struct("Repeat").field("count", count).field("body", body).finish(),
            Self::Inst(ref inst) => fmt::Debug::fmt(&**inst, f),
        }
    }
}

impl Eq for Op {}

impl PartialEq for Op {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::If {
                    then_blk: lt,
                    else_blk: le,
                    ..
                },
                Self::If {
                    then_blk: rt,
                    else_blk: re,
                    ..
                },
            ) => lt == rt && le == re,
            (Self::While { body: lbody, .. }, Self::While { body: rbody, .. }) => lbody == rbody,
            (
                Self::Repeat {
                    count: lcount,
                    body: lbody,
                    ..
                },
                Self::Repeat {
                    count: rcount,
                    body: rbody,
                    ..
                },
            ) => lcount == rcount && lbody == rbody,
            (Self::Inst(l), Self::Inst(r)) => l == r,
            _ => false,
        }
    }
}

impl Spanned for Op {
    fn span(&self) -> SourceSpan {
        match self {
            Self::If { span, .. } | Self::While { span, .. } | Self::Repeat { span, .. } => *span,
            Self::Inst(spanned) => spanned.span(),
        }
    }
}
