use alloc::{string::String, vec::Vec};
use miden_crypto::hash::rpo::RpoDigest;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use thiserror::Error;
use winter_utils::{ByteWriter, Serializable};

use super::{MastForest, MastNode};

/// Specifies an offset into the `data` section of an encoded [`MastForest`].
type DataOffset = u32;

/// Magic string for detecting that a file is binary-encoded MAST.
const MAGIC: &[u8; 5] = b"MAST\0";

/// The format version.
///
/// If future modifications are made to this format, the version should be incremented by 1. A
/// version of `[255, 255, 255]` is reserved for future extensions that require extending the
/// version field itself, but should be considered invalid for now.
const VERSION: [u8; 3] = [0, 0, 0];

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid discriminant '{discriminant}' for type '{ty}'")]
    InvalidDiscriminant { ty: String, discriminant: u8 },
}

/// An entry in the `strings` table of an encoded [`MastForest`].
///
/// Strings are UTF8-encoded.
pub struct StringRef {
    /// Offset into the `data` section.
    offset: DataOffset,

    /// Length of the utf-8 string.
    len: u32,
}

impl Serializable for StringRef {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.offset.write_into(target);
        self.len.write_into(target);
    }
}

pub struct MastNodeInfo {
    ty: MastNodeType,
    offset: DataOffset,
    digest: RpoDigest,
}

impl Serializable for MastNodeInfo {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.ty.write_into(target);
        self.offset.write_into(target);
        self.digest.write_into(target);
    }
}

pub struct MastNodeType([u8; 8]);

impl Serializable for MastNodeType {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.0.write_into(target);
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum MastNodeTypeVariant {
    Join,
    Split,
    Loop,
    Call,
    Dyn,
    Block,
    External,
}

impl MastNodeTypeVariant {
    pub fn discriminant(&self) -> u8 {
        self.to_u8().expect("guaranteed to fit in a `u8` due to #[repr(u8)]")
    }

    pub fn try_from_discriminant(discriminant: u8) -> Result<Self, Error> {
        Self::from_u8(discriminant).ok_or_else(|| Error::InvalidDiscriminant {
            ty: "MastNode".into(),
            discriminant,
        })
    }
}

impl Serializable for MastForest {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let mut strings: Vec<StringRef> = Vec::new();
        let mut data: Vec<u8> = Vec::new();

        // magic & version
        target.write_bytes(MAGIC);
        target.write_bytes(&VERSION);

        // node count
        target.write_usize(self.nodes.len());

        // roots
        self.roots.write_into(target);

        // MAST node infos
        for mast_node in &self.nodes {
            let mast_node_info = convert_mast_node(mast_node, &mut data, &mut strings);

            mast_node_info.write_into(target);
        }

        // strings table
        strings.write_into(target);

        // data blob
        data.write_into(target);
    }
}

fn convert_mast_node(
    mast_node: &MastNode,
    data: &mut Vec<u8>,
    strings: &mut Vec<StringRef>,
) -> MastNodeInfo {
    todo!()
}
