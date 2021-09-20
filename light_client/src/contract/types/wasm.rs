use ibc_proto::ibc::core::{client::v1::Height, commitment::v1::MerkleRoot};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Without the other side of the bridge (Tendermint LC on Celo)
// we don't know how the consensus or client state will look like.
//
// NOTE: This is just a placeholder
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct CosmosConsensusState {
    pub root: MerkleRoot,
}

// NOTE: This is just a placeholder
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct CosmosClientState {
    pub latest_height: Height,
}

#[derive(prost::Message, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartialConsensusState {
    #[prost(bytes = "vec", tag = "1")]
    pub code_id: Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub enum Status {
    Active,
    Frozen,
    Exipred,
    Unknown
}


