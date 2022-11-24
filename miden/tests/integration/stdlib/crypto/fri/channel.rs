use air::{Felt, FieldElement};
use miden::AdviceSet;
use winter_fri::{utils::hash_values, FriProof, VerifierError};
use winter_prover::{
    crypto::{BatchMerkleProof, ElementHasher, Hasher, MerkleTree},
    DeserializationError,
};
use winter_utils::transpose_slice;

pub trait UnBatch<E: FieldElement, H: ElementHasher> {
    fn unbatch<const N: usize, const W: usize>(
        &mut self,
        positions: &[usize],
        domain_size: usize,
        layer_commitments: Vec<<H as Hasher>::Digest>,
    ) -> (Vec<AdviceSet>, Vec<([u8; 32], Vec<Felt>)>);
}

pub struct MidenFriVerifierChannel<E: FieldElement, H: ElementHasher<BaseField = E::BaseField>> {
    layer_commitments: Vec<H::Digest>,
    layer_proofs: Vec<BatchMerkleProof<H>>,
    layer_queries: Vec<Vec<E>>,
    remainder: Vec<E>,
}

impl<E, H> MidenFriVerifierChannel<E, H>
where
    E: FieldElement,
    H: ElementHasher<BaseField = E::BaseField>,
{
    /// Builds a new verifier channel from the specified [FriProof].
    ///
    /// # Errors
    /// Returns an error if the specified `proof` could not be parsed correctly.
    pub fn new(
        proof: FriProof,
        layer_commitments: Vec<H::Digest>,
        domain_size: usize,
        folding_factor: usize,
    ) -> Result<Self, DeserializationError> {
        let remainder = proof.parse_remainder()?;
        let (layer_queries, layer_proofs) =
            proof.parse_layers::<H, E>(domain_size, folding_factor)?;

        Ok(MidenFriVerifierChannel {
            layer_commitments,
            layer_proofs,
            layer_queries,
            remainder,
        })
    }

    pub fn take_fri_remainder(&mut self) -> Vec<E> {
        self.remainder.clone()
    }

    pub fn layer_proofs(&mut self) -> Vec<BatchMerkleProof<H>> {
        self.layer_proofs.drain(..).collect()
    }

    pub fn layer_queries(&mut self) -> Vec<Vec<E>> {
        self.layer_queries.clone()
    }

    pub fn read_fri_layer_commitments(&mut self) -> Vec<H::Digest> {
        self.layer_commitments.drain(..).collect()
    }

    pub fn read_remainder<const N: usize>(
        &mut self,
        commitment: &<H as Hasher>::Digest,
    ) -> Result<Vec<E>, VerifierError> {
        let remainder = self.take_fri_remainder();

        // build remainder Merkle tree
        let remainder_values = transpose_slice(&remainder);
        let hashed_values = hash_values::<H, E, N>(&remainder_values);
        let remainder_tree = MerkleTree::<H>::new(hashed_values)
            .map_err(|err| VerifierError::RemainderTreeConstructionFailed(format!("{}", err)))?;

        // make sure the root of the tree matches the committed root of the last layer
        if commitment != remainder_tree.root() {
            return Err(VerifierError::RemainderCommitmentMismatch);
        }

        Ok(remainder)
    }
}
