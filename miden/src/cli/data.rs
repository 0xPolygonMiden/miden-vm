use std::{
    collections::{BTreeMap, HashMap},
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

use assembly::{
    ast::{Module, ModuleKind},
    diagnostics::{IntoDiagnostic, Report, WrapErr},
    Assembler, Library, LibraryNamespace,
};
use miden_vm::{
    crypto::{MerkleStore, MerkleTree, NodeIndex, PartialMerkleTree, RpoDigest, SimpleSmt},
    math::Felt,
    utils::{Deserializable, SliceReader},
    AdviceInputs, Digest, ExecutionProof, MemAdviceProvider, Program, StackInputs, StackOutputs,
    Word,
};
use serde_derive::{Deserialize, Serialize};
use stdlib::StdLibrary;
pub use tracing::{event, instrument, Level};

// CONSTANTS
// ================================================================================================
const SIMPLE_SMT_DEPTH: u8 = u64::BITS as u8;

// HELPERS
// ================================================================================================

/// Indicates whether debug mode is on or off.
pub enum Debug {
    On,
    Off,
}

impl Debug {
    /// Returns true if debug mode is on.
    fn is_on(&self) -> bool {
        matches!(self, Self::On)
    }
}

// MERKLE DATA
// ================================================================================================

/// Struct used to deserialize merkle data from input file. Merkle data can be represented as a
/// merkle tree or a Sparse Merkle Tree.
#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Debug)]
pub enum MerkleData {
    /// String representation of a merkle tree. The merkle tree is represented as a vector of
    /// 32 byte hex strings where each string represents a leaf in the tree.
    #[serde(rename = "merkle_tree")]
    MerkleTree(Vec<String>),
    /// String representation of a Sparse Merkle Tree. The Sparse Merkle Tree is represented as a
    /// vector of tuples where each tuple consists of a u64 node index and a 32 byte hex string
    /// representing the value of the node.
    #[serde(rename = "sparse_merkle_tree")]
    SparseMerkleTree(Vec<(u64, String)>),
    /// String representation of a Partial Merkle Tree. The Partial Merkle Tree is represented as a
    /// vector of tuples where each tuple consists of a leaf index tuple (depth, index) and a 32
    /// byte hex string representing the value of the leaf.
    #[serde(rename = "partial_merkle_tree")]
    PartialMerkleTree(Vec<((u8, u64), String)>),
}

// INPUT FILE
// ================================================================================================

// TODO consider using final types instead of string representations.
/// Input file struct that is used to deserialize input data from file. It consists of four
/// components:
/// - operand_stack
/// - advice_stack
/// - advice_map
/// - merkle_store
#[derive(Deserialize, Debug)]
pub struct InputFile {
    /// String representation of the initial operand stack, composed of chained field elements.
    pub operand_stack: Vec<String>,
    /// Optional string representation of the initial advice stack, composed of chained field
    /// elements.
    pub advice_stack: Option<Vec<String>>,
    /// Optional map of 32 byte hex strings to vectors of u64s representing the initial advice map.
    pub advice_map: Option<HashMap<String, Vec<u64>>>,
    /// Optional vector of merkle data which will be loaded into the initial merkle store. Merkle
    /// data is represented as 32 byte hex strings and node indexes are represented as u64s.
    pub merkle_store: Option<Vec<MerkleData>>,
}

/// Helper methods to interact with the input file
impl InputFile {
    #[instrument(name = "read_input_file", skip_all)]
    pub fn read(inputs_path: &Option<PathBuf>, program_path: &Path) -> Result<Self, Report> {
        // if file not specified explicitly and corresponding file with same name as program_path
        // with '.inputs' extension does't exist, set operand_stack to empty vector
        if !inputs_path.is_some() && !program_path.with_extension("inputs").exists() {
            return Ok(Self {
                operand_stack: Vec::new(),
                advice_stack: Some(Vec::new()),
                advice_map: Some(HashMap::new()),
                merkle_store: None,
            });
        }

        // If inputs_path has been provided then use this as path. Alternatively we will
        // replace the program_path extension with `.inputs` and use this as a default.
        let path = match inputs_path {
            Some(path) => path.clone(),
            None => program_path.with_extension("inputs"),
        };

        // read input file to string
        let inputs_file = fs::read_to_string(&path)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to open input file `{}`", path.display()))?;

        // deserialize input data
        let inputs: InputFile = serde_json::from_str(&inputs_file)
            .into_diagnostic()
            .wrap_err("Failed to deserialize input data")?;

        Ok(inputs)
    }

    /// Parse advice provider data from the input file.
    pub fn parse_advice_provider(&self) -> Result<MemAdviceProvider, String> {
        let mut advice_inputs = AdviceInputs::default();

        let stack = self
            .parse_advice_stack()
            .map_err(|e| format!("failed to parse advice provider: {e}"))?;
        advice_inputs = advice_inputs.with_stack_values(stack).map_err(|e| e.to_string())?;

        if let Some(map) = self
            .parse_advice_map()
            .map_err(|e| format!("failed to parse advice provider: {e}"))?
        {
            advice_inputs = advice_inputs.with_map(map);
        }

        if let Some(merkle_store) = self
            .parse_merkle_store()
            .map_err(|e| format!("failed to parse advice provider: {e}"))?
        {
            advice_inputs = advice_inputs.with_merkle_store(merkle_store);
        }

        Ok(MemAdviceProvider::from(advice_inputs))
    }

    /// Parse advice stack data from the input file.
    fn parse_advice_stack(&self) -> Result<Vec<u64>, String> {
        self.advice_stack
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|v| {
                v.parse::<u64>()
                    .map_err(|e| format!("failed to parse advice stack value '{v}': {e}"))
            })
            .collect::<Result<Vec<_>, _>>()
    }

    /// Parse advice map data from the input file.
    fn parse_advice_map(&self) -> Result<Option<BTreeMap<RpoDigest, Vec<Felt>>>, String> {
        let advice_map = match &self.advice_map {
            Some(advice_map) => advice_map,
            None => return Ok(None),
        };

        let map = advice_map
            .iter()
            .map(|(k, v)| {
                // Convert key to RpoDigest
                let key = RpoDigest::try_from(k)
                    .map_err(|e| format!("failed to decode advice map key '{k}': {e}"))?;

                // convert values to Felt
                let values = v
                    .iter()
                    .map(|v| {
                        Felt::try_from(*v).map_err(|e| {
                            format!("failed to convert advice map value '{v}' to Felt: {e}")
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok((key, values))
            })
            .collect::<Result<BTreeMap<RpoDigest, Vec<Felt>>, String>>()?;

        Ok(Some(map))
    }

    /// Parse merkle store data from the input file.
    fn parse_merkle_store(&self) -> Result<Option<MerkleStore>, String> {
        let merkle_data = match &self.merkle_store {
            Some(merkle_data) => merkle_data,
            None => return Ok(None),
        };

        let mut merkle_store = MerkleStore::default();
        for data in merkle_data {
            match data {
                MerkleData::MerkleTree(data) => {
                    let leaves = Self::parse_merkle_tree(data)?;
                    let tree = MerkleTree::new(leaves)
                        .map_err(|e| format!("failed to parse a Merkle tree: {e}"))?;
                    merkle_store.extend(tree.inner_nodes());
                    event!(
                        Level::TRACE,
                        "Added Merkle tree with root {} to the Merkle store",
                        tree.root()
                    );
                },
                MerkleData::SparseMerkleTree(data) => {
                    let entries = Self::parse_sparse_merkle_tree(data)?;
                    let tree = SimpleSmt::<SIMPLE_SMT_DEPTH>::with_leaves(entries)
                        .map_err(|e| format!("failed to parse a Sparse Merkle Tree: {e}"))?;
                    merkle_store.extend(tree.inner_nodes());
                    event!(
                        Level::TRACE,
                        "Added Sparse Merkle tree with root {} to the Merkle store",
                        tree.root()
                    );
                },
                MerkleData::PartialMerkleTree(data) => {
                    let entries = Self::parse_partial_merkle_tree(data)?;
                    let tree = PartialMerkleTree::with_leaves(entries)
                        .map_err(|e| format!("failed to parse a Partial Merkle Tree: {e}"))?;
                    merkle_store.extend(tree.inner_nodes());
                    event!(
                        Level::TRACE,
                        "Added Partial Merkle tree with root {} to the Merkle store",
                        tree.root()
                    );
                },
            }
        }

        Ok(Some(merkle_store))
    }

    /// Parse and return merkle tree leaves.
    fn parse_merkle_tree(tree: &[String]) -> Result<Vec<Word>, String> {
        tree.iter()
            .map(|v| {
                let leaf = Self::parse_word(v)?;
                Ok(leaf)
            })
            .collect()
    }

    /// Parse and return Sparse Merkle Tree entries.
    fn parse_sparse_merkle_tree(tree: &[(u64, String)]) -> Result<Vec<(u64, Word)>, String> {
        tree.iter()
            .map(|(index, v)| {
                let leaf = Self::parse_word(v)?;
                Ok((*index, leaf))
            })
            .collect()
    }

    /// Parse and return Partial Merkle Tree entries.
    fn parse_partial_merkle_tree(
        tree: &[((u8, u64), String)],
    ) -> Result<Vec<(NodeIndex, RpoDigest)>, String> {
        tree.iter()
            .map(|((depth, index), v)| {
                let node_index = NodeIndex::new(*depth, *index).map_err(|e| {
                    format!(
                        "failed to create node index with depth {depth} and index {index} - {e}"
                    )
                })?;
                let leaf = Self::parse_word(v)?;
                Ok((node_index, RpoDigest::new(leaf)))
            })
            .collect()
    }

    /// Parse a `Word` from a hex string.
    pub fn parse_word(word_hex: &str) -> Result<Word, String> {
        let word_value = &word_hex[2..];
        let mut word_data = [0u8; 32];
        hex::decode_to_slice(word_value, &mut word_data)
            .map_err(|e| format!("failed to decode `Word` from hex {word_hex} - {e}"))?;
        let mut word = Word::default();
        for (i, value) in word_data.chunks(8).enumerate() {
            word[i] = Felt::try_from(value).map_err(|e| {
                format!("failed to convert `Word` data {word_hex} (element {i}) to Felt - {e}")
            })?;
        }
        Ok(word)
    }

    /// Parse and return the stack inputs for the program.
    pub fn parse_stack_inputs(&self) -> Result<StackInputs, String> {
        let stack_inputs = self
            .operand_stack
            .iter()
            .map(|v| v.parse::<u64>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        StackInputs::try_from_ints(stack_inputs).map_err(|e| e.to_string())
    }
}

// OUTPUT FILE
// ================================================================================================

/// Output file struct
#[derive(Deserialize, Serialize, Debug)]
pub struct OutputFile {
    pub stack: Vec<String>,
}

/// Helper methods to interact with the output file
impl OutputFile {
    /// Returns a new [OutputFile] from the specified outputs vectors
    pub fn new(stack_outputs: &StackOutputs) -> Self {
        Self {
            stack: stack_outputs.stack().iter().map(|&v| v.to_string()).collect::<Vec<String>>(),
        }
    }

    /// Read the output file
    #[instrument(name = "read_output_file",
        fields(path = %outputs_path.clone().unwrap_or(program_path.with_extension("outputs")).display()), skip_all)]
    pub fn read(outputs_path: &Option<PathBuf>, program_path: &Path) -> Result<Self, String> {
        // If outputs_path has been provided then use this as path.  Alternatively we will
        // replace the program_path extension with `.outputs` and use this as a default.
        let path = match outputs_path {
            Some(path) => path.clone(),
            None => program_path.with_extension("outputs"),
        };

        // read outputs file to string
        let outputs_file = fs::read_to_string(&path)
            .map_err(|err| format!("Failed to open outputs file `{}` - {}", path.display(), err))?;

        // deserialize outputs data
        let outputs: OutputFile = serde_json::from_str(&outputs_file)
            .map_err(|err| format!("Failed to deserialize outputs data - {}", err))?;

        Ok(outputs)
    }

    /// Write the output file
    #[instrument(name = "write_data_to_output_file", fields(path = %path.display()), skip_all)]
    pub fn write(stack_outputs: &StackOutputs, path: &PathBuf) -> Result<(), String> {
        // if path provided, create output file
        let file = fs::File::create(path).map_err(|err| {
            format!("Failed to create output file `{}` - {}", path.display(), err)
        })?;

        // write outputs to output file
        serde_json::to_writer_pretty(file, &Self::new(stack_outputs))
            .map_err(|err| format!("Failed to write output data - {}", err))
    }

    /// Converts stack output vector to [StackOutputs].
    pub fn stack_outputs(&self) -> Result<StackOutputs, String> {
        let stack = self.stack.iter().map(|v| v.parse::<u64>().unwrap()).collect::<Vec<u64>>();

        StackOutputs::try_from_ints(stack)
            .map_err(|e| format!("Construct stack outputs failed {e}"))
    }
}

// PROGRAM FILE
// ================================================================================================

pub struct ProgramFile {
    ast: Box<Module>,
    source_manager: Arc<dyn assembly::SourceManager>,
}

/// Helper methods to interact with masm program file.
impl ProgramFile {
    /// Reads the masm file at the specified path and parses it into a [ProgramFile].
    pub fn read(path: impl AsRef<Path>) -> Result<Self, Report> {
        let source_manager = Arc::new(assembly::DefaultSourceManager::default());
        Self::read_with(path, source_manager)
    }

    /// Reads the masm file at the specified path and parses it into a [ProgramFile], using the
    /// provided [assembly::SourceManager] implementation.
    #[instrument(name = "read_program_file", skip(source_manager), fields(path = %path.as_ref().display()))]
    pub fn read_with(
        path: impl AsRef<Path>,
        source_manager: Arc<dyn assembly::SourceManager>,
    ) -> Result<Self, Report> {
        // parse the program into an AST
        let path = path.as_ref();
        let mut parser = Module::parser(ModuleKind::Executable);
        let ast = parser
            .parse_file(LibraryNamespace::Exec.into(), path, &source_manager)
            .wrap_err_with(|| format!("Failed to parse program file `{}`", path.display()))?;

        Ok(Self { ast, source_manager })
    }

    /// Compiles this program file into a [Program].
    #[instrument(name = "compile_program", skip_all)]
    pub fn compile<'a, I>(&self, debug: &Debug, libraries: I) -> Result<Program, Report>
    where
        I: IntoIterator<Item = &'a Library>,
    {
        // compile program
        let mut assembler =
            Assembler::new(self.source_manager.clone()).with_debug_mode(debug.is_on());
        assembler.add_library(StdLibrary::default()).wrap_err("Failed to load stdlib")?;

        for library in libraries {
            assembler.add_library(library).wrap_err("Failed to load libraries")?;
        }

        let program: Program = assembler
            .assemble_program(self.ast.as_ref())
            .wrap_err("Failed to compile program")?;

        Ok(program)
    }
}

// PROOF FILE
// ================================================================================================

pub struct ProofFile;

/// Helper methods to interact with proof file
impl ProofFile {
    /// Read stark proof from file
    #[instrument(name = "read_proof_file",
        fields(path = %proof_path.clone().unwrap_or(program_path.with_extension("proof")).display()), skip_all)]
    pub fn read(
        proof_path: &Option<PathBuf>,
        program_path: &Path,
    ) -> Result<ExecutionProof, String> {
        // If proof_path has been provided then use this as path.  Alternatively we will
        // replace the program_path extension with `.proof` and use this as a default.
        let path = match proof_path {
            Some(path) => path.clone(),
            None => program_path.with_extension("proof"),
        };

        // read the file to bytes
        let file = fs::read(&path)
            .map_err(|err| format!("Failed to open proof file `{}` - {}", path.display(), err))?;

        // deserialize bytes into a stark proof
        ExecutionProof::from_bytes(&file)
            .map_err(|err| format!("Failed to decode proof data - {}", err))
    }

    /// Write stark proof to file
    #[instrument(name = "write_data_to_proof_file",
                 fields(
                    path = %proof_path.clone().unwrap_or(program_path.with_extension("proof")).display(),
                    size = format!("{} KB", proof.to_bytes().len() / 1024)), skip_all)]
    pub fn write(
        proof: ExecutionProof,
        proof_path: &Option<PathBuf>,
        program_path: &Path,
    ) -> Result<(), String> {
        // If proof_path has been provided then use this as path.  Alternatively we will
        // replace the program_path extension with `.proof` and use this as a default.
        let path = match proof_path {
            Some(path) => path.clone(),
            None => program_path.with_extension("proof"),
        };

        // create output fille
        let mut file = fs::File::create(&path)
            .map_err(|err| format!("Failed to create proof file `{}` - {}", path.display(), err))?;

        let proof_bytes = proof.to_bytes();

        // write proof bytes to file
        file.write_all(&proof_bytes).unwrap();

        Ok(())
    }
}

// PROGRAM HASH
// ================================================================================================

pub struct ProgramHash;

/// Helper method to parse program hash from hex
impl ProgramHash {
    #[instrument(name = "read_program_hash", skip_all)]
    pub fn read(hash_hex_string: &String) -> Result<Digest, String> {
        // decode hex to bytes
        let program_hash_bytes = hex::decode(hash_hex_string)
            .map_err(|err| format!("Failed to convert program hash to bytes {}", err))?;

        // create slice reader from bytes
        let mut program_hash_slice = SliceReader::new(&program_hash_bytes);

        // create hash digest from slice
        let program_hash = Digest::read_from(&mut program_hash_slice)
            .map_err(|err| format!("Failed to deserialize program hash from bytes - {}", err))?;

        Ok(program_hash)
    }
}

// LIBRARY FILE
// ================================================================================================
pub struct Libraries {
    pub libraries: Vec<Library>,
}

impl Libraries {
    /// Creates a new instance of [Libraries] from a list of library paths.
    #[instrument(name = "read_library_files", skip_all)]
    pub fn new<P, I>(paths: I) -> Result<Self, Report>
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = P>,
    {
        let mut libraries = Vec::new();

        for path in paths {
            // TODO(plafer): How to create a `Report` from an error that doesn't derive
            // `Diagnostic`?
            let library = Library::deserialize_from_file(path).unwrap();
            libraries.push(library);
        }

        Ok(Self { libraries })
    }
}

// TESTS
// ================================================================================================
#[cfg(test)]
mod test {
    use super::InputFile;

    #[test]
    fn test_merkle_data_parsing() {
        let program_with_pmt = "
        {
            \"operand_stack\": [\"1\"],
            \"merkle_store\": [
                {
                    \"partial_merkle_tree\": [
                        [
                            [2, 0],
                            \"0x1400000000000000000000000000000000000000000000000000000000000000\"
                        ],
                        [
                            [2, 1],
                            \"0x1500000000000000000000000000000000000000000000000000000000000000\"
                        ],
                        [
                            [1, 1],
                            \"0x0b00000000000000000000000000000000000000000000000000000000000000\"
                        ]
                    ]
                }
            ]
        }";
        let inputs: InputFile = serde_json::from_str(program_with_pmt).unwrap();
        let merkle_store = inputs.parse_merkle_store().unwrap();
        assert!(merkle_store.is_some());

        let program_with_smt = "
        {
            \"operand_stack\": [\"1\"],
            \"merkle_store\": [
              {
                \"sparse_merkle_tree\": [
                  [
                    0,
                    \"0x1400000000000000000000000000000000000000000000000000000000000000\"
                  ],
                  [
                    1,
                    \"0x1500000000000000000000000000000000000000000000000000000000000000\"
                  ],
                  [
                    3,
                    \"0x1700000000000000000000000000000000000000000000000000000000000000\"
                  ]
                ]
              }
            ]
          }";
        let inputs: InputFile = serde_json::from_str(program_with_smt).unwrap();
        let merkle_store = inputs.parse_merkle_store().unwrap();
        assert!(merkle_store.is_some());

        let program_with_merkle_tree = "
        {
            \"operand_stack\": [\"1\"],
            \"merkle_store\": [
                {
                    \"merkle_tree\": [
                        \"0x1400000000000000000000000000000000000000000000000000000000000000\",
                        \"0x1500000000000000000000000000000000000000000000000000000000000000\",
                        \"0x1600000000000000000000000000000000000000000000000000000000000000\",
                        \"0x1700000000000000000000000000000000000000000000000000000000000000\"
                    ]
                }
            ]
        }";
        let inputs: InputFile = serde_json::from_str(program_with_merkle_tree).unwrap();
        let merkle_store = inputs.parse_merkle_store().unwrap();
        assert!(merkle_store.is_some());
    }
}
