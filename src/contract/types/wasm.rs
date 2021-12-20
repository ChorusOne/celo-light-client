use crate::contract::types::ibc::{Height, MerkleRoot};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use prost_derive::Message;

// Without the other side of the bridge (Tendermint LC on Celo)
// we don't know how the consensus or client state will look like.
//
// NOTE: This is just a placeholder
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CosmosConsensusState {
    pub root: MerkleRoot,
}

// NOTE: This is just a placeholder
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct CosmosClientState {
    pub latest_height: Height,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct ConsensusState {
    pub code_id: String, // Go serializes []byte to base64 encoded string
    pub data: String,    // Go serializes []byte to base64 encoded string
    pub timestamp: u64,
    pub root: MerkleRoot,
}

#[derive(Message, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartialConsensusState {
    #[prost(bytes = "vec", tag = "1")]
    pub code_id: Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct ClientState {
    pub data: String,    // Go serializes []byte to base64 encoded string
    pub code_id: String, // Go serializes []byte to base64 encoded string

    #[serde(default)]
    pub frozen: bool,
    pub frozen_height: Option<Height>,
    pub latest_height: Option<Height>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct WasmHeader {
    pub data: String, // Go serializes []byte to base64 encoded string
    pub height: Height,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct Misbehaviour {
    pub code_id: String, // Go serializes []byte to base64 encoded string
    pub client_id: String,
    pub header_1: WasmHeader,
    pub header_2: WasmHeader,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub enum Status {
    Active,
    Frozen,
    Exipred,
    Unknown
}
