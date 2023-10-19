use super::{super::DebugOptions, ByteReader, ByteWriter, DeserializationError, ToString};

const STACK_ALL: u8 = 0;
const STACK_TOP: u8 = 1;
const MEM_ALL: u8 = 2;
const MEM_INTERVAL: u8 = 3;
const LOCAL_INTERVAL: u8 = 4;

/// Writes the provided [DebugOptions] into the provided target.
pub fn write_options_into<W: ByteWriter>(target: &mut W, options: &DebugOptions) {
    match options {
        DebugOptions::StackAll => target.write_u8(STACK_ALL),
        DebugOptions::StackTop(n) => {
            target.write_u8(STACK_TOP);
            target.write_u16(*n);
        }
        DebugOptions::MemAll => target.write_u8(MEM_ALL),
        DebugOptions::MemInterval(n, m) => {
            target.write_u8(MEM_INTERVAL);
            target.write_u32(*n);
            target.write_u32(*m);
        }
        DebugOptions::LocalInterval(start, end, num_locals) => {
            target.write_u8(LOCAL_INTERVAL);
            target.write_u16(*start);
            target.write_u16(*end);
            target.write_u16(*num_locals);
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
        MEM_ALL => Ok(DebugOptions::MemAll),
        MEM_INTERVAL => {
            let n = source.read_u32()?;
            let m = source.read_u32()?;
            Ok(DebugOptions::MemInterval(n, m))
        }
        LOCAL_INTERVAL => {
            let n = source.read_u16()?;
            let m = source.read_u16()?;
            let num_locals = source.read_u16()?;
            Ok(DebugOptions::LocalInterval(n, m, num_locals))
        }
        val => Err(DeserializationError::InvalidValue(val.to_string())),
    }
}
