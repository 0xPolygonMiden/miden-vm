use core::fmt;

use crate::ast::{ImmU8, ImmU16, ImmU32};

// DEBUG OPTIONS
// ================================================================================================

/// A proxy for [vm_core::DebugOptions], but with [super::Immediate] values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DebugOptions {
    StackAll,
    StackTop(ImmU8),
    MemAll,
    MemInterval(ImmU32, ImmU32),
    LocalInterval(ImmU16, ImmU16),
    LocalRangeFrom(ImmU16),
    LocalAll,
    AdvStackTop(ImmU8),
}

impl crate::prettier::PrettyPrint for DebugOptions {
    fn render(&self) -> crate::prettier::Document {
        crate::prettier::display(self)
    }
}

impl TryFrom<DebugOptions> for vm_core::DebugOptions {
    type Error = ();

    fn try_from(options: DebugOptions) -> Result<Self, Self::Error> {
        match options {
            DebugOptions::StackAll => Ok(Self::StackAll),
            DebugOptions::StackTop(ImmU8::Value(n)) => Ok(Self::StackTop(n.into_inner())),
            DebugOptions::MemAll => Ok(Self::MemAll),
            DebugOptions::MemInterval(ImmU32::Value(start), ImmU32::Value(end)) => {
                Ok(Self::MemInterval(start.into_inner(), end.into_inner()))
            },
            DebugOptions::LocalInterval(ImmU16::Value(start), ImmU16::Value(end)) => {
                let start = start.into_inner();
                let end = end.into_inner();
                Ok(Self::LocalInterval(start, end, end - start))
            },
            DebugOptions::AdvStackTop(ImmU8::Value(n)) => Ok(Self::AdvStackTop(n.into_inner())),
            _ => Err(()),
        }
    }
}

impl fmt::Display for DebugOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StackAll => write!(f, "stack"),
            Self::StackTop(n) => write!(f, "stack.{n}"),
            Self::MemAll => write!(f, "mem"),
            Self::MemInterval(n, m) => write!(f, "mem.{n}.{m}"),
            Self::LocalAll => write!(f, "local"),
            Self::LocalRangeFrom(start) => write!(f, "local.{start}"),
            Self::LocalInterval(start, end) => {
                write!(f, "local.{start}.{end}")
            },
            Self::AdvStackTop(n) => write!(f, "adv_stack.{n}"),
        }
    }
}
