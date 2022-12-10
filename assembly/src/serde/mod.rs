use super::{Felt, SerializationError, StarkField, String, Vec};
use core::str::from_utf8;

// BYTE WRITER
// ================================================================================================

/// Contains a vector for storing serialized objects
#[derive(Default)]
pub struct ByteWriter(Vec<u8>);

impl ByteWriter {
    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn write_bool(&mut self, val: bool) {
        self.write_u8(val as u8);
    }

    pub fn write_u8(&mut self, val: u8) {
        self.0.push(val);
    }

    pub fn write_u16(&mut self, val: u16) {
        self.0.extend(val.to_le_bytes());
    }

    pub fn write_u32(&mut self, val: u32) {
        self.0.extend(val.to_le_bytes());
    }

    pub fn write_u64(&mut self, val: u64) {
        self.0.extend(val.to_le_bytes());
    }

    pub fn write_len(&mut self, val: usize) -> Result<(), SerializationError> {
        if val > u16::MAX as usize {
            return Err(SerializationError::LengthTooLong);
        }
        self.write_u16(val as u16);
        Ok(())
    }

    pub fn write_felt(&mut self, val: Felt) {
        self.write_u64(val.as_int());
    }

    pub fn write_str(&mut self, val: &str) -> Result<(), SerializationError> {
        self.write_len(val.len())?;
        self.0.extend(val.as_bytes());
        Ok(())
    }

    pub fn write_bytes(&mut self, val: &[u8]) {
        self.0.extend(val);
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

// SERIALIZABLE TRAIT IMPLEMENTATIONS
// ================================================================================================

/// Converts `self` into bytes and writes them to the provided `ByteWriter` struct
pub trait Serializable: Sized {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError>;

    fn to_bytes(&self) -> Result<Vec<u8>, SerializationError> {
        let mut target = ByteWriter::default();
        self.write_into(&mut target)?;
        Ok(target.into_bytes())
    }
}

impl Serializable for () {
    fn write_into(&self, _target: &mut ByteWriter) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl Serializable for &'_ str {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        target.write_str(self)
    }
}

impl Serializable for String {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        target.write_str(self)
    }
}

impl<T> Serializable for Vec<T>
where
    T: Serializable,
{
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        target.write_len(self.len())?;
        self.iter().try_for_each(|t| t.write_into(target))
    }
}

impl<T> Serializable for Option<T>
where
    T: Serializable,
{
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        match self {
            Some(t) => {
                target.write_bool(true);
                t.write_into(target)
            }
            None => {
                target.write_bool(false);
                Ok(())
            }
        }
    }
}

// BYTE READER
// ================================================================================================

/// Contains bytes for deserialization and current reading position
pub struct ByteReader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        ByteReader { bytes, pos: 0 }
    }

    pub fn read_bool(&mut self) -> Result<bool, SerializationError> {
        self.check_eor(1)?;
        let result = self.bytes[self.pos];
        self.pos += 1;
        u8_to_bool(result)
    }

    pub fn read_u8(&mut self) -> Result<u8, SerializationError> {
        self.check_eor(1)?;
        let result = self.bytes[self.pos];
        self.pos += 1;
        Ok(result)
    }

    pub fn peek_u8(&self) -> Result<u8, SerializationError> {
        self.check_eor(1)?;
        let result = self.bytes[self.pos];
        Ok(result)
    }

    pub fn read_u16(&mut self) -> Result<u16, SerializationError> {
        self.check_eor(2)?;
        let result = &self.bytes[self.pos..self.pos + 2];
        self.pos += 2;
        Ok(u16::from_le_bytes(result.try_into().expect("u16 conversion failure")))
    }

    pub fn read_u32(&mut self) -> Result<u32, SerializationError> {
        self.check_eor(4)?;
        let result = &self.bytes[self.pos..self.pos + 4];
        self.pos += 4;
        Ok(u32::from_le_bytes(result.try_into().expect("u32 conversion failure")))
    }

    pub fn read_u64(&mut self) -> Result<u64, SerializationError> {
        self.check_eor(8)?;
        let result = &self.bytes[self.pos..self.pos + 8];
        self.pos += 8;
        Ok(u64::from_le_bytes(result.try_into().expect("u64 conversion failure")))
    }

    pub fn read_len(&mut self) -> Result<usize, SerializationError> {
        self.read_u16().map(|l| l as usize)
    }

    pub fn read_felt(&mut self) -> Result<Felt, SerializationError> {
        let value = self.read_u64()?;
        if value >= Felt::MODULUS {
            Err(SerializationError::InvalidFieldElement)
        } else {
            Ok(Felt::new(value))
        }
    }

    pub fn read_str(&mut self) -> Result<&str, SerializationError> {
        let len = self.read_u16()? as usize;
        self.check_eor(len)?;
        let string = &self.bytes[self.pos..self.pos + len];
        self.pos += len;
        from_utf8(string).map_err(|_| SerializationError::InvalidUtf8)
    }

    pub fn read_bytes(&mut self, num_bytes: usize) -> Result<&[u8], SerializationError> {
        self.check_eor(num_bytes)?;
        let bytes = &self.bytes[self.pos..self.pos + num_bytes];
        self.pos += num_bytes;
        Ok(bytes)
    }

    /// Checks if it is possible to read at least `num_bytes` bytes from ByteReader
    ///
    /// # Errors
    /// Returns an error if, when reading the requested number of bytes, we go beyond the boundaries of the array
    fn check_eor(&self, num_bytes: usize) -> Result<(), SerializationError> {
        if self.pos + num_bytes > self.bytes.len() {
            return Err(SerializationError::EndOfReader);
        }
        Ok(())
    }
}

// DESERIALIZABLE TRAIT IMPLEMENTATIONS
// ================================================================================================

/// Returns `self` from its byte representation stored in provided `ByteReader` struct.
pub trait Deserializable: Sized {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError>;

    fn read_from_bytes(bytes: &[u8]) -> Result<Self, SerializationError> {
        Self::read_from(&mut ByteReader::new(bytes))
    }
}

impl Deserializable for () {
    fn read_from(_bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        Ok(())
    }
}

impl Deserializable for String {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        bytes.read_str().map(String::from)
    }
}

impl<T> Deserializable for Vec<T>
where
    T: Deserializable,
{
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let len = bytes.read_len()?;
        (0..len).map(|_| T::read_from(bytes)).collect()
    }
}

impl<T> Deserializable for Option<T>
where
    T: Deserializable,
{
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let is_some = bytes.read_bool()?;
        is_some.then(|| T::read_from(bytes)).transpose()
    }
}

// HELPER FUNCTIONS
// ================================================================================================
fn u8_to_bool(param: u8) -> Result<bool, SerializationError> {
    match param {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(SerializationError::InvalidBoolValue),
    }
}
