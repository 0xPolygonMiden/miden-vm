use vm_core::SignatureKind;
use super::{ByteReader, ByteWriter, DeserializationError, ToString};

const RPOFALCON512: u8 = 0;

/// Writes the provided [SignatureKind] into the provided target.
pub fn write_options_into<W: ByteWriter>(target: &mut W, options: &SignatureKind) {
    match options {
        SignatureKind::RpoFalcon512 => {
            target.write_u8(RPOFALCON512);
        }
    }
}

/// Reads [SignatureKind] from the provided source.
pub fn read_options_from<R: ByteReader>(
    source: &mut R,
) -> Result<SignatureKind, DeserializationError> {
    match source.read_u8()? {
        RPOFALCON512 => Ok(SignatureKind::RpoFalcon512),
        val => Err(DeserializationError::InvalidValue(val.to_string())),
    }
}
