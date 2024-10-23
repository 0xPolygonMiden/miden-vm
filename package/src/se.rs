use alloc::vec;
use processor::Digest;
use vm_core::utils::Serializable;

use crate::package::MastArtifact;

pub fn serialize_digest<S>(digest: &Digest, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serde_bytes::serialize(&digest.as_bytes(), serializer)
}

pub fn serialize_mast<S>(mast: &MastArtifact, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut buffer = vec![];
    match mast {
        MastArtifact::Executable(program) => {
            buffer.extend(b"PRG\0");
            program.write_into(&mut buffer);
        },
        MastArtifact::Library(library) => {
            buffer.extend(b"LIB\0");
            library.write_into(&mut buffer);
        },
    }

    serde_bytes::serialize(&buffer, serializer)
}
