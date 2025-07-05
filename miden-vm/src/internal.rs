use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::{Path, PathBuf},
};

use assembly::diagnostics::{IntoDiagnostic, Report, WrapErr};
use serde_derive::Deserialize;
pub use tracing::{Level, event, instrument};
use vm_core::{Felt, WORD_SIZE};

use crate::{
    AdviceInputs, StackInputs, Word, ZERO,
    crypto::{MerkleStore, MerkleTree, NodeIndex, PartialMerkleTree, SimpleSmt},
};

// CONSTANTS
// ================================================================================================
const SIMPLE_SMT_DEPTH: u8 = u64::BITS as u8;

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

impl Default for InputFile {
    fn default() -> Self {
        Self {
            operand_stack: Vec::new(),
            advice_stack: Some(Vec::new()),
            advice_map: Some(HashMap::new()),
            merkle_store: None,
        }
    }
}

/// Helper methods to interact with the input file
impl InputFile {
    #[instrument(name = "read_input_file", skip_all)]
    pub fn read(inputs_path: &Option<PathBuf>, program_path: &Path) -> Result<Self, Report> {
        // if file not specified explicitly and corresponding file with same name as program_path
        // with '.inputs' extension does't exist, set operand_stack to empty vector
        if !inputs_path.is_some() && !program_path.with_extension("inputs").exists() {
            return Ok(Self::default());
        }

        // If inputs_path has been provided then use this as path. Alternatively we will
        // replace the program_path extension with .inputs and use this as a default.
        let path = match inputs_path {
            Some(path) => path.clone(),
            None => program_path.with_extension("inputs"),
        };

        // read input file to string
        let inputs_file = fs::read_to_string(&path)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to open input file {}", path.display()))?;

        // deserialize input data
        let inputs: InputFile = serde_json::from_str(&inputs_file)
            .into_diagnostic()
            .wrap_err("Failed to deserialize input data")?;

        Ok(inputs)
    }

    /// Parse advice inputs from the input file.
    pub fn parse_advice_inputs(&self) -> Result<AdviceInputs, String> {
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

        Ok(advice_inputs)
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
    fn parse_advice_map(&self) -> Result<Option<BTreeMap<Word, Vec<Felt>>>, String> {
        let advice_map = match &self.advice_map {
            Some(advice_map) => advice_map,
            None => return Ok(None),
        };

        let map = advice_map
            .iter()
            .map(|(k, v)| {
                // Convert key to Word
                let key = Word::try_from(k)
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
            .collect::<Result<BTreeMap<Word, Vec<Felt>>, String>>()?;

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
    ) -> Result<Vec<(NodeIndex, Word)>, String> {
        tree.iter()
            .map(|((depth, index), v)| {
                let node_index = NodeIndex::new(*depth, *index).map_err(|e| {
                    format!(
                        "failed to create node index with depth {depth} and index {index} - {e}"
                    )
                })?;
                let leaf = Self::parse_word(v)?;
                Ok((node_index, leaf))
            })
            .collect()
    }

    /// Parse a `Word` from a hex string.
    pub fn parse_word(word_hex: &str) -> Result<Word, String> {
        let word_value = &word_hex[2..];
        let mut word_data = [0u8; 32];
        hex::decode_to_slice(word_value, &mut word_data)
            .map_err(|e| format!("failed to decode `Word` from hex {word_hex} - {e}"))?;
        let mut word = [ZERO; WORD_SIZE];
        for (i, value) in word_data.chunks(8).enumerate() {
            word[i] = Felt::try_from(value).map_err(|e| {
                format!("failed to convert `Word` data {word_hex} (element {i}) to Felt - {e}")
            })?;
        }
        Ok(word.into())
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

#[cfg(test)]
mod tests {
    use super::*;

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
