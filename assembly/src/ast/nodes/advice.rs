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
    PushExt2intt,
    PushSmtGet,
    PushMapVal,
    InsertMem,
}

impl Serializable for AdviceInjector {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        match self {
            AdviceInjector::PushU64div => target.write_u8(0),
            AdviceInjector::PushExt2intt => target.write_u8(3),
            AdviceInjector::PushSmtGet => target.write_u8(4),
            AdviceInjector::PushMapVal => target.write_u8(1),
            AdviceInjector::InsertMem => target.write_u8(5),
        }
    }
}

impl Deserializable for AdviceInjector {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            0 => Ok(AdviceInjector::PushU64div),
            3 => Ok(AdviceInjector::PushExt2intt),
            4 => Ok(AdviceInjector::PushSmtGet),
            1 => Ok(AdviceInjector::PushMapVal),
            5 => Ok(AdviceInjector::InsertMem),
            val => Err(DeserializationError::InvalidValue(val.to_string())),
        }
    }
}

impl fmt::Display for AdviceInjector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdviceInjector::PushU64div => write!(f, "push_u64div"),
            AdviceInjector::PushExt2intt => write!(f, "push_ext2intt"),
            AdviceInjector::PushSmtGet => write!(f, "push_smtget"),
            AdviceInjector::PushMapVal => write!(f, "push_mapval"),
            AdviceInjector::InsertMem => write!(f, "insert_mem"),
        }
    }
}
