use crate::{IBCMerkleRoot, ICSInnerSpec, ICSLeafOp, ICSProofSpec, Error, Kind};
use std::convert::{From, TryFrom, TryInto};
use ethereum_types::H256;

// Origin: ibc.core.commitment.v1 (compiled proto)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct MerklePrefix {
    pub key_prefix: String, // Go serializes []byte to base64 encoded string
}

// Origin: ibc.core.commitment.v1 (compiled proto)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct MerkleRoot {
    pub hash: String, // Go serializes []byte to base64 encoded string
}
impl From<IBCMerkleRoot> for MerkleRoot {
    fn from(ibc: IBCMerkleRoot) -> Self {
        Self {
            hash: base64::encode(ibc.hash),
        }
    }
}
impl TryFrom<MerkleRoot> for IBCMerkleRoot {
    type Error = base64::DecodeError;
    fn try_from(m: MerkleRoot) -> Result<Self, Self::Error> {
        let s = Self {
            hash: base64::decode(m.hash)?,
        };
        Ok(s)
    }
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
impl From<ICSProofSpec> for ProofSpec {
    fn from(ics: ICSProofSpec) -> Self {
        Self {
            leaf_spec: ics.leaf_spec.map(LeafOp::from),
            inner_spec: ics.inner_spec.map(InnerSpec::from),
            max_depth: ics.max_depth,
            min_depth: ics.min_depth,
        }
    }
}
impl TryFrom<ProofSpec> for ICSProofSpec {
    type Error = base64::DecodeError;
    fn try_from(p: ProofSpec) -> Result<Self, Self::Error> {
        let inner = if let Some(i) = p.inner_spec {
            Some(ICSInnerSpec::try_from(i)?)
        } else {
            None
        };
        let leaf = if let Some(l) = p.leaf_spec {
            Some(ICSLeafOp::try_from(l)?)
        } else {
            None
        };
        let s = Self {
            leaf_spec: leaf,
            inner_spec: inner,
            max_depth: p.max_depth,
            min_depth: p.min_depth,
        };
        Ok(s)
    }
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
impl From<ICSInnerSpec> for InnerSpec {
    fn from(ics: ICSInnerSpec) -> Self {
        Self {
            child_order: ics.child_order,
            child_size: ics.child_size,
            min_prefix_length: ics.min_prefix_length,
            max_prefix_length: ics.max_prefix_length,
            empty_child: base64::encode(ics.empty_child),
            hash: ics.hash,
        }
    }
}
impl TryFrom<InnerSpec> for ICSInnerSpec {
    type Error = base64::DecodeError;
    fn try_from(i: InnerSpec) -> Result<Self, Self::Error> {
        let s = Self {
            child_order: i.child_order,
            child_size: i.child_size,
            min_prefix_length: i.min_prefix_length,
            max_prefix_length: i.max_prefix_length,
            empty_child: base64::decode(i.empty_child)?,
            hash: i.hash,
        };
        Ok(s)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct LeafOp {
    pub hash: i32,
    pub prehash_key: i32,
    pub prehash_value: i32,
    pub length: i32,
    pub prefix: String,
}
impl From<ICSLeafOp> for LeafOp {
    fn from(ics: ICSLeafOp) -> Self {
        Self {
            hash: ics.hash,
            prehash_key: ics.prehash_key,
            prehash_value: ics.prehash_value,
            length: ics.length,
            prefix: base64::encode(ics.prefix),
        }
    }
}
impl TryFrom<LeafOp> for ICSLeafOp {
    type Error = base64::DecodeError;
    fn try_from(l: LeafOp) -> Result<Self, Self::Error> {
        let s = Self {
            hash: l.hash,
            prehash_key: l.prehash_key,
            prehash_value: l.prehash_value,
            length: l.length,
            prefix: base64::decode(l.prefix)?,
        };
        Ok(s)
    }
}
