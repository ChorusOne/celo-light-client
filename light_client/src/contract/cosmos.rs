use ibc_proto::ibc::core::client::v1::Height;
use ibc_proto::ibc::core::commitment::v1::MerkleRoot;
//use cosmwasm_std::Binary;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ::prost::Message,
)]
pub struct ConsensusState {
    #[prost(message, optional, tag = "1")]
    pub root: Option<MerkleRoot>,
}
#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct ClientState {
    #[prost(message, optional, tag = "1")]
    pub latest_height: Option<Height>,
}

//#[derive(Default, Serialize, Deserialize, Clone, PartialEq)]
//pub struct PartialConsensusState {
//pub data: Binary,
//}

//#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
//pub enum Status {
//Active,
//Frozen,
//Exipred,
//Unknown,
//}
