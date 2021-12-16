use ethereum_types::{H256, U256};

/// https://eips.ethereum.org/EIPS/eip-1186
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(any(test, feature = "serialize"), derive(serde::Deserialize))]
///Proof struct returned by eth_getProof method
pub struct Proof {
    /// the balance of the account. See eth_getBalance
    pub balance: U256,
    ///  hash of the code of the account
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "codeHash"))]
    pub code_hash: H256,
    /// nonce of the account. See eth_getTransactionCount
    pub nonce: U256,
    /// SHA3 of the StorageRoot.
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "storageHash"))]
    pub storage_hash: H256,
    /// Array of rlp-serialized MerkleTree-Nodes, starting with the stateRoot-Node, following the path of the SHA3 (address) as key.
    #[cfg_attr(
        any(test, feature = "serialize"),
        serde(rename = "accountProof", deserialize_with = "rlp_nodes::deserialize")
    )]
    pub account_proof: Vec<Vec<u8>>,
    /// Array of storage-entries as requested
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "storageProof"))]
    pub storage_proof: Vec<StorageProof>,
}

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(any(test, feature = "serialize"), derive(serde::Deserialize))]
pub struct StorageProof {
    /// the requested storage key
    pub key: U256,
    /// the storage value
    pub value: U256,
    /// Array of rlp-serialized MerkleTree-Nodes, starting with the storageHash-Node, following the path of the SHA3 (key) as path.
    #[cfg_attr(
        any(test, feature = "serialize"),
        serde(deserialize_with = "rlp_nodes::deserialize")
    )]
    pub proof: Vec<Vec<u8>>,
}

#[derive(Debug, Default, Clone, rlp_derive::RlpEncodable, rlp_derive::RlpDecodable)]
pub struct Account {
    pub nonce: U256,
    pub balance: U256,
    pub storage_hash: H256,
    pub code_hash: H256,
}

#[cfg(any(test, feature = "serialize"))]
mod rlp_nodes {
    use serde::de::*;
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<Vec<u8>>, D::Error> {
        let nodes_str: Vec<String> = Vec::deserialize(deserializer)?;
        nodes_str
            .into_iter()
            .map(hex::decode)
            .collect::<Result<_, _>>()
            .map_err(serde::de::Error::custom)
    }
}
