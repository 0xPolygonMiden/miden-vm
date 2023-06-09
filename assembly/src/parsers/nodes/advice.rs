use super::super::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};
use core::fmt;

// ADVICE INJECTORS
// ================================================================================================

/// Instructions which inject data into the advice provider.
///
/// These instructions can be used to perform two broad sets of operations:
/// - Push new data onto the advice stack.
/// - Insert new data into the advice map.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AdviceInjector {
    PushU64div,
    PushMapVal,
    PushExt2inv,
    PushExt2intt,
    PushSmtGet,
    InsertMem,
}

impl Serializable for AdviceInjector {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        match self {
            AdviceInjector::PushU64div => target.write_u8(0),
            AdviceInjector::PushMapVal => target.write_u8(1),
            AdviceInjector::PushExt2inv => target.write_u8(2),
            AdviceInjector::PushExt2intt => target.write_u8(3),
            AdviceInjector::PushSmtGet => target.write_u8(4),
            AdviceInjector::InsertMem => target.write_u8(5),
        }
    }
}

impl Deserializable for AdviceInjector {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            0 => Ok(AdviceInjector::PushU64div),
            1 => Ok(AdviceInjector::PushMapVal),
            2 => Ok(AdviceInjector::PushExt2inv),
            3 => Ok(AdviceInjector::PushExt2intt),
            4 => Ok(AdviceInjector::PushSmtGet),
            5 => Ok(AdviceInjector::InsertMem),
            val => Err(DeserializationError::InvalidValue(val.to_string())),
        }
    }
}

impl fmt::Display for AdviceInjector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdviceInjector::PushU64div => write!(f, "u64div"), // push_u64div
            AdviceInjector::PushMapVal => write!(f, "keyval"), // push_mapval
            AdviceInjector::PushExt2inv => write!(f, "ext2inv"), // push_ext2inv
            AdviceInjector::PushExt2intt => write!(f, "ext2intt"), // push_ext2intt
            AdviceInjector::PushSmtGet => write!(f, "smtget"), // push_smtget
            AdviceInjector::InsertMem => write!(f, "mem"),     // inert_mem
        }
    }
}
