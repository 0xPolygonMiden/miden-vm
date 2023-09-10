use super::{super::DebugOptions, ByteReader, ByteWriter, DeserializationError, ToString};

const STACK_ALL: u8 = 0;
const STACK_TOP: u8 = 1;

/// Writes the provided [DebugOptions] into the provided target.
pub fn write_options_into<W: ByteWriter>(target: &mut W, options: &DebugOptions) {
    match options {
        DebugOptions::StackAll => target.write_u8(STACK_ALL),
        DebugOptions::StackTop(n) => {
            target.write_u8(STACK_TOP);
            target.write_u16(*n);
        }
    }
}

/// Reads [DebugOptions] from the provided source.
pub fn read_options_from<R: ByteReader>(
    source: &mut R,
) -> Result<DebugOptions, DeserializationError> {
    match source.read_u8()? {
        STACK_ALL => Ok(DebugOptions::StackAll),
        STACK_TOP => {
            let n = source.read_u16()?;
            if n == 0 {
                return Err(DeserializationError::InvalidValue(n.to_string()));
            }
            Ok(DebugOptions::StackTop(n))
        }
        val => Err(DeserializationError::InvalidValue(val.to_string())),
    }
}
