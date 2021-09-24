//use ibc_proto::ibc::core::{client::v1::Height};
use celo_ibc::{Height, MerkleRoot};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Without the other side of the bridge (Tendermint LC on Celo)
// we don't know how the consensus or client state will look like.
//
// NOTE: This is just a placeholder
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CosmosConsensusState {
    pub root: MerkleRoot,
}

// NOTE: This is just a placeholder
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct CosmosClientState {
    pub latest_height: Height,
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartialConsensusState {
    pub code_id: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub enum Status {
    Active,
    Frozen,
    Exipred,
    Unknown
}


