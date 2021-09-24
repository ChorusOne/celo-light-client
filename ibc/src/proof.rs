use std::convert::{From, TryFrom, TryInto};
use ethereum_types::H256;
use crate::{Error, Kind};

// Origin: ibc.core.commitment.v1 (compiled proto)
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct MerklePrefix {
    pub key_prefix: String, // Go serializes []byte to base64 encoded string
}

// Origin: ibc.core.commitment.v1 (compiled proto)
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct MerkleRoot {
    pub hash: String, // Go serializes []byte to base64 encoded string
}
impl From<H256> for MerkleRoot{
    fn from(h: H256) -> Self {
        Self{
            hash : base64::encode(h.as_bytes()),
        }
    }
}
impl TryFrom<MerkleRoot> for H256 {
    type Error = Error;
    fn try_from(m: MerkleRoot) -> Result<Self,Self::Error> {
        let h_bytes = base64::decode(m.hash)
            .map_err(|base64_error| Kind::Base64DecodeError{base64_error})?;
        let h_array : &[u8;H256::len_bytes()] = h_bytes.as_slice().try_into()
            .map_err(|from_slice_error| Kind::TryFromSliceError{from_slice_error})?;
        Ok(H256::from(h_array))
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ProofSpec {
    pub leaf_spec: Option<LeafOp>,
    pub inner_spec: Option<InnerSpec>,
    pub max_depth: i32,
    pub min_depth: i32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct InnerSpec {
    pub child_order: Vec<i32>,
    pub child_size: i32,
    pub min_prefix_length: i32,
    pub max_prefix_length: i32,
    pub empty_child: String, // ::prost::alloc::vec::Vec<u8>,
    pub hash: i32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct LeafOp {
    pub hash: i32,
    pub prehash_key: i32,
    pub prehash_value: i32,
    pub length: i32,
    pub prefix: String,
}
