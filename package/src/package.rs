use core::{fmt, str::FromStr};

use alloc::{collections::BTreeSet, format, string::String, sync::Arc, vec::Vec};

use assembly::{ast::QualifiedProcedureName, Library, Report};
use processor::Digest;
use serde::{Deserialize, Serialize};
use vm_core::{utils::DisplayHex, Program};

use super::{de, se};
use crate::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Package {
    /// Name of the package
    pub name: String,
    /// Content digest of the package
    #[serde(
        serialize_with = "se::serialize_digest",
        deserialize_with = "de::deserialize_digest"
    )]
    pub digest: Digest,
    /// The package type and MAST
    #[serde(serialize_with = "se::serialize_mast", deserialize_with = "de::deserialize_mast")]
    pub mast: MastArtifact,
    /// The rodata segments required by the code in this package
    pub rodata: Vec<Rodata>,
    /// The package manifest, containing the set of exported procedures and their signatures,
    /// if known.
    pub manifest: PackageManifest,
}

// impl fmt::Debug for Package {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct("Package")
//             .field("name", &self.name)
//             .field("digest", &format_args!("{}", DisplayHex::new(&self.digest.as_bytes())))
//             .field_with("rodata", |f| f.debug_list().entries(self.rodata.iter()).finish())
//             .field("manifest", &self.manifest)
//             .finish_non_exhaustive()
//     }
// }

// impl Emit for Package {
//     fn name(&self) -> Option<Symbol> {
//         Some(self.name)
//     }

//     fn output_type(&self, mode: midenc_session::OutputMode) -> midenc_session::OutputType {
//         use midenc_session::OutputMode;
//         match mode {
//             OutputMode::Text => self.mast.output_type(mode),
//             OutputMode::Binary => midenc_session::OutputType::Masp,
//         }
//     }

//     fn write_to<W: std::io::Write>(
//         &self,
//         mut writer: W,
//         mode: midenc_session::OutputMode,
//         session: &Session,
//     ) -> std::io::Result<()> {
//         use midenc_session::OutputMode;
//         match mode {
//             OutputMode::Text => self.mast.write_to(writer, mode, session),
//             OutputMode::Binary => {
//                 // Write magic
//                 writer.write_all(b"MASP\0")?;
//                 // Write format version
//                 writer.write_all(b"1.0\0")?;
//                 let data = bitcode::serialize(self).map_err(std::io::Error::other)?;
//                 writer.write_all(data.as_slice())
//             },
//         }
//     }
// }

/// The artifact produced by lowering a [Program] to a Merkelized Abstract Syntax Tree
///
/// This type is used in compilation pipelines to abstract over the type of output requested.
#[derive(Clone)]
pub enum MastArtifact {
    /// A MAST artifact which can be executed by the VM directly
    Executable(Arc<Program>),
    /// A MAST artifact which can be used as a dependency by a [miden_core::Program]
    Library(Arc<Library>),
}

/// The types of dependencies in a package
#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
)]
#[repr(u8)]
pub enum DependencyKind {
    /// A compiled MAST library
    #[default]
    Mast,
    /// A source-form MASM library, using the standard project layout
    Masm,
    // A Miden package (MASP)
    Masp,
}

impl fmt::Display for DependencyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mast => f.write_str("mast"),
            Self::Masm => f.write_str("masm"),
            Self::Masp => f.write_str("masp"),
        }
    }
}

impl FromStr for DependencyKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mast" | "masl" => Ok(Self::Mast),
            "masm" => Ok(Self::Masm),
            "masp" => Ok(Self::Masp),
            _ => Err(()),
        }
    }
}

/// A library dependency
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyInfo {
    /// The name of the dependency.
    pub name: String,
    /// If specified, the path from which this dependency should be loaded.
    pub path: Option<String>,
    /// The kind of dependency to load.
    pub kind: DependencyKind,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PackageManifest {
    /// The set of exports in this package.
    pub exports: BTreeSet<PackageExport>,
    /// The libraries linked against by this package, which must be provided when executing the
    /// program.
    pub dependencies: Vec<DependencyInfo>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageExport {
    pub id: QualifiedProcedureName,
    #[serde(
        serialize_with = "se::serialize_digest",
        deserialize_with = "de::deserialize_digest"
    )]
    pub digest: Digest,
    // Signature will be added in the future when the type signatures are available in the Assembly
    // #[serde(default)]
    // pub signature: Option<Signature>,
}

impl fmt::Debug for PackageExport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PackageExport")
            .field("id", &format_args!("{}", self.id))
            .field("digest", &format_args!("{}", DisplayHex::new(&self.digest.as_bytes())))
            // .field("signature", &self.signature)
            .finish()
    }
}

impl PartialOrd for PackageExport {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PackageExport {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id).then_with(|| self.digest.cmp(&other.digest))
    }
}

/// This type represents the raw data of a constant.
///
/// The data is expected to be in little-endian order.
#[derive(Debug, Clone, PartialEq, Eq, Default, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ConstantData(#[serde(with = "serde_bytes")] Vec<u8>);

impl ConstantData {
    /// Return the number of bytes in the constant.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return the data as a slice.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

/// This represents a descriptor for a pointer referencing data in Miden's linear memory.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PtrDesc {
    /// This is the address of the word containing the first byte of data
    pub waddr: u32,
    /// This is the element index of the word referenced by `waddr` containing the first byte of
    /// data
    ///
    /// Each element is assumed to be a 32-bit value/chunk
    pub index: u8,
    /// This is the byte offset into the 32-bit chunk referenced by `index`
    ///
    /// This offset is where the data referenced by the pointer actually starts.
    pub offset: u8,
}

/// Represents a read-only data segment, combined with its content digest
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rodata {
    /// The content digest computed for `data`
    #[serde(
        serialize_with = "se::serialize_digest",
        deserialize_with = "de::deserialize_digest"
    )]
    pub digest: Digest,
    /// The address at which the data for this segment begins
    pub start: PtrDesc,
    /// The raw binary data for this segment
    pub data: Arc<ConstantData>,
}

// impl fmt::Debug for Rodata {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Rodata")
//             .field("digest", &format_args!("{}", DisplayHex::new(&self.digest.as_bytes())))
//             .field("start", &self.start)
//             .field_with("data", |f| {
//                 f.debug_struct("ConstantData")
//                     .field("len", &self.data.len())
//                     .finish_non_exhaustive()
//             })
//             .finish()
//     }
// }

impl Rodata {
    pub fn size_in_bytes(&self) -> usize {
        self.data.len()
    }

    pub fn size_in_felts(&self) -> usize {
        self.data.len().next_multiple_of(4) / 4
    }

    pub fn size_in_words(&self) -> usize {
        self.size_in_felts().next_multiple_of(4) / 4
    }

    // /// Attempt to convert this rodata object to its equivalent representation in felts
    // ///
    // /// The resulting felts will be in padded out to the nearest number of words, i.e. if the data
    // /// only takes up 3 felts worth of bytes, then the resulting `Vec` will contain 4 felts, so that
    // /// the total size is a valid number of words.
    // pub fn to_elements(&self) -> Result<Vec<processor::Felt>, String> {
    //     use processor::Felt;
    //     use vm_core::FieldElement;

    //     let data = self.data.as_slice();
    //     let mut felts = Vec::with_capacity(data.len() / 4);
    //     let mut iter = data.iter().copied().array_chunks::<4>();
    //     felts.extend(iter.by_ref().map(|bytes| Felt::new(u32::from_le_bytes(bytes) as u64)));
    //     if let Some(remainder) = iter.into_remainder() {
    //         let mut chunk = [0u8; 4];
    //         for (i, byte) in remainder.into_iter().enumerate() {
    //             chunk[i] = byte;
    //         }
    //         felts.push(Felt::new(u32::from_le_bytes(chunk) as u64));
    //     }

    //     let padding = (self.size_in_words() * 4).abs_diff(felts.len());
    //     felts.resize(felts.len() + padding, Felt::ZERO);

    //     Ok(felts)
    // }
}

impl Package {
    // pub fn read_from_file<P>(path: P) -> std::io::Result<Self>
    // where
    //     P: AsRef<std::path::Path>,
    // {
    //     let path = path.as_ref();
    //     let bytes = std::fs::read(path)?;

    //     Self::read_from_bytes(bytes).map_err(std::io::Error::other)
    // }

    pub fn read_from_bytes<B>(bytes: B) -> Result<Self, Report>
    where
        B: AsRef<[u8]>,
    {
        use alloc::borrow::Cow;

        let bytes = bytes.as_ref();

        let bytes = bytes
            .strip_prefix(b"MASP\0")
            .ok_or_else(|| Report::msg("invalid package: missing header"))?;
        let bytes = bytes.strip_prefix(b"1.0\0").ok_or_else(|| {
            Report::msg(format!(
                "invalid package: incorrect version, expected '1.0', got '{}'",
                bytes.get(0..4).map(String::from_utf8_lossy).unwrap_or(Cow::Borrowed("")),
            ))
        })?;

        bitcode::deserialize(bytes).map_err(Report::msg)
    }

    pub fn is_program(&self) -> bool {
        matches!(self.mast, MastArtifact::Executable(_))
    }

    pub fn is_library(&self) -> bool {
        matches!(self.mast, MastArtifact::Library(_))
    }

    pub fn unwrap_program(&self) -> Arc<Program> {
        match self.mast {
            MastArtifact::Executable(ref prog) => Arc::clone(prog),
            _ => panic!("expected package to contain a program, but got a library"),
        }
    }

    pub fn unwrap_library(&self) -> Arc<Library> {
        match self.mast {
            MastArtifact::Library(ref lib) => Arc::clone(lib),
            _ => panic!("expected package to contain a library, but got an executable"),
        }
    }

    pub fn make_executable(&self, entrypoint: &QualifiedProcedureName) -> Result<Self, Report> {
        let MastArtifact::Library(ref library) = self.mast else {
            return Err(Report::msg("expected library but got an executable"));
        };

        let module = library
            .module_infos()
            .find(|info| info.path() == &entrypoint.module)
            .ok_or_else(|| {
                Report::msg(format!(
                    "invalid entrypoint: library does not contain a module named '{}'",
                    entrypoint.module
                ))
            })?;
        if let Some(digest) = module.get_procedure_digest_by_name(&entrypoint.name) {
            let node_id = library.mast_forest().find_procedure_root(digest).ok_or_else(|| {
                Report::msg(
                    "invalid entrypoint: malformed library - procedure exported, but digest has \
                     no node in the forest",
                )
            })?;

            let exports = BTreeSet::from_iter(self.manifest.exports.iter().find_map(|export| {
                if export.digest == digest {
                    Some(export.clone())
                } else {
                    None
                }
            }));

            Ok(Self {
                name: self.name.clone(),
                digest,
                mast: MastArtifact::Executable(Arc::new(Program::new(
                    library.mast_forest().clone(),
                    node_id,
                ))),
                rodata: self.rodata.clone(),
                manifest: PackageManifest {
                    exports,
                    dependencies: self.manifest.dependencies.clone(),
                },
            })
        } else {
            Err(Report::msg(format!(
                "invalid entrypoint: library does not export '{}'",
                entrypoint
            )))
        }
    }
}
