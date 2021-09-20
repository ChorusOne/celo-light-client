use crate::error::{Error, Kind};
use crate::{
    IBCClientState, IBCConsensusState, IBCMerkleRoot, ICSProofSpec, MerkleRoot, ProofSpec,
};

use celo_types::{client::LightClientState, consensus::LightConsensusState};
use ibc_proto::ibc::core::client::v1::Height;
use std::convert::{From, TryFrom};
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ConsensusState {
    pub data: String,
    pub code_id: String,
    pub timestamp: u64,
    pub root: MerkleRoot,
}
impl ConsensusState {
    pub fn new(
        lc: &LightConsensusState,
        code_id: String,
        timestamp: u64,
        root: MerkleRoot,
    ) -> Self {
        let r = rlp::encode(lc);
        Self {
            data: base64::encode(r),
            code_id,
            timestamp,
            root,
        }
    }
}
// these two converters work both as converters, but also as type check in case IBCConsensusState
// struct changes
impl TryFrom<IBCConsensusState> for ConsensusState {
    type Error = Error;
    fn try_from(ibc: IBCConsensusState) -> Result<Self, Self::Error> {
        let root = ibc.root.ok_or_else(|| Kind::MissingField {
            struct_name: String::from("IBCConsensusState"),
            field_name: String::from("root"),
        })?;
        let s = Self {
            data: base64::encode(ibc.data),
            code_id: base64::encode(ibc.code_id),
            timestamp: ibc.timestamp,
            root: MerkleRoot::from(root),
        };
        Ok(s)
    }
}
impl TryFrom<ConsensusState> for IBCConsensusState {
    type Error = base64::DecodeError;
    fn try_from(cs: ConsensusState) -> Result<Self, Self::Error> {
        let s = Self {
            data: base64::decode(&cs.data)?,
            code_id: base64::decode(&cs.code_id)?,
            timestamp: cs.timestamp,
            root: Some(IBCMerkleRoot::try_from(cs.root)?),
        };
        Ok(s)
    }
}
pub fn extract_lc_consensus_state(cs: &ConsensusState) -> Result<LightConsensusState, Error> {
    let v: Vec<u8> = base64::decode(&cs.data).map_err(|e| {
        let k: Kind = e.into();
        let e: Error = k.into();
        e
    })?;
    rlp::decode(&v).map_err(|e| {
        let k: Kind = e.into();
        k.into()
    })
}
pub fn extract_code_id_from_consensus(cs: &ConsensusState) -> Result<Vec<u8>, Error> {
    base64::decode(&cs.code_id).map_err(|e| {
        let k: Kind = e.into();
        k.into()
    })
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ClientState {
    pub data: String,
    pub code_id: String,
    pub latest_height: Height,
    pub proof_specs: Vec<ProofSpec>,
    pub frozen_height: Option<Height>,
}
impl TryFrom<IBCClientState> for ClientState {
    type Error = Error;
    fn try_from(ibc: IBCClientState) -> Result<Self, Self::Error> {
        let specs = ibc.proof_specs.into_iter().map(ProofSpec::from).collect();
        let s = Self {
            data: base64::encode(ibc.data),
            code_id: base64::encode(ibc.code_id),
            latest_height: ibc.latest_height.ok_or_else(|| Kind::MissingField {
                struct_name: String::from("IBCClientState"),
                field_name: String::from("latest_height"),
            })?,
            proof_specs: specs,
            frozen_height: None,
        };
        Ok(s)
    }
}
impl TryFrom<ClientState> for IBCClientState {
    type Error = base64::DecodeError;
    fn try_from(cs: ClientState) -> Result<Self, Self::Error> {
        let specs: Vec<ICSProofSpec> = cs
            .proof_specs
            .into_iter()
            .map(ICSProofSpec::try_from)
            .collect::<Result<_,_>>()?;
        let s = Self {
            data: base64::decode(&cs.data)?,
            code_id: base64::decode(&cs.code_id)?,
            latest_height: Some(cs.latest_height),
            proof_specs: specs,
        };
        Ok(s)
    }
}

pub fn extract_lc_client_state(cs: &ClientState) -> Result<LightClientState, Error> {
    let v: Vec<u8> = base64::decode(&cs.data).map_err(|e| {
        let k: Kind = e.into();
        let e: Error = k.into();
        e
    })?;
    rlp::decode(&v).map_err(|e| {
        let k: Kind = e.into();
        k.into()
    })
}
