use alloc::string::ToString;
use core::fmt;

use crate::{
    ast::{ImmU16, ImmU32},
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable,
};

const STACK_ALL: u8 = 0;
const STACK_TOP: u8 = 1;
const MEM_ALL: u8 = 2;
const MEM_INTERVAL: u8 = 3;
const LOCAL_INTERVAL: u8 = 4;
const LOCAL_RANGE_FROM: u8 = 5;
const LOCAL_ALL: u8 = 6;

/// A proxy for [vm_core::DebugOptions], but with [Immediate] values.
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DebugOptions {
    StackAll = STACK_ALL,
    StackTop(ImmU16) = STACK_TOP,
    MemAll = MEM_ALL,
    MemInterval(ImmU32, ImmU32) = MEM_INTERVAL,
    LocalInterval(ImmU16, ImmU16) = LOCAL_INTERVAL,
    LocalRangeFrom(ImmU16) = LOCAL_RANGE_FROM,
    LocalAll = LOCAL_ALL,
}

impl DebugOptions {
    fn tag(&self) -> u8 {
        // SAFETY: This is safe because we have given this enum a
        // primitive representation with #[repr(u8)], with the first
        // field of the underlying union-of-structs the discriminant
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}
#[cfg(feature = "formatter")]
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
            DebugOptions::StackTop(ImmU16::Value(n)) => Ok(Self::StackTop(n.into_inner())),
            DebugOptions::MemAll => Ok(Self::MemAll),
            DebugOptions::MemInterval(ImmU32::Value(start), ImmU32::Value(end)) => {
                Ok(Self::MemInterval(start.into_inner(), end.into_inner()))
            }
            DebugOptions::LocalInterval(ImmU16::Value(start), ImmU16::Value(end)) => {
                let start = start.into_inner();
                let end = end.into_inner();
                Ok(Self::LocalInterval(start, end, end - start))
            }
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
            }
        }
    }
}

impl Serializable for DebugOptions {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8(self.tag());
        match self {
            Self::StackAll | Self::MemAll | Self::LocalAll => (),
            Self::StackTop(ImmU16::Value(n)) => {
                target.write_u16(n.into_inner());
            }
            Self::MemInterval(ImmU32::Value(n), ImmU32::Value(m)) => {
                target.write_u32(n.into_inner());
                target.write_u32(m.into_inner());
            }
            Self::LocalRangeFrom(ImmU16::Value(start)) => {
                let start = start.into_inner();
                target.write_u16(start);
            }
            Self::LocalInterval(ImmU16::Value(start), ImmU16::Value(end)) => {
                let start = start.into_inner();
                let end = end.into_inner();
                target.write_u16(start);
                target.write_u16(end);
                target.write_u16(end - start);
            }
            options => unimplemented!("unimplemented debug options: {options}"),
        }
    }
}

impl Deserializable for DebugOptions {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            STACK_ALL => Ok(Self::StackAll),
            STACK_TOP => {
                let n = source.read_u16()?;
                if n == 0 {
                    return Err(DeserializationError::InvalidValue(n.to_string()));
                }
                Ok(Self::StackTop(n.into()))
            }
            MEM_ALL => Ok(Self::MemAll),
            MEM_INTERVAL => {
                let n = source.read_u32()?;
                let m = source.read_u32()?;
                Ok(Self::MemInterval(n.into(), m.into()))
            }
            LOCAL_INTERVAL => {
                let n = source.read_u16()?;
                let m = source.read_u16()?;
                source.read_u16()?;
                match (n, m) {
                    (0, u16::MAX) => Ok(Self::LocalAll),
                    (n, u16::MAX) => Ok(Self::LocalRangeFrom(n.into())),
                    (n, m) => Ok(Self::LocalInterval(n.into(), m.into())),
                }
            }
            LOCAL_RANGE_FROM => {
                let n = source.read_u16()?;
                Ok(Self::LocalRangeFrom(n.into()))
            }
            LOCAL_ALL => Ok(Self::LocalAll),
            val => Err(DeserializationError::InvalidValue(val.to_string())),
        }
    }
}
