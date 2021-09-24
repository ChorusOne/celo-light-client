use crate::{Error, Kind, Height, ProofSpec, MerkleRoot};
use celo_types::{client::LightClientState, consensus::LightConsensusState};
use std::convert::{From, TryFrom};


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
impl Default for ConsensusState {
    fn default() -> Self {
        let lc = LightConsensusState::default();
        ConsensusState::new(&lc, String::default(), 0, MerkleRoot::default())
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

#[derive(
    Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct ClientState {
    pub data: String,
    pub code_id: String,
    pub latest_height: Height,
    pub proof_specs: Vec<ProofSpec>,
    pub frozen_height: Option<Height>,
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
