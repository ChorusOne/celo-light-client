use crate::{Error, Kind, MerklePrefix};
use std::convert::{From, TryFrom};

// Origin: ibc.core.connection.v1 (compiled proto)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ConnectionEnd {
    pub client_id: String,
    pub versions: Vec<Version>,
    pub state: i32,
    pub counterparty: Counterparty,
    pub delay_period: u64,
}

// Origin: ibc.core.connection.v1 (compiled proto)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Counterparty {
    pub client_id: String,
    pub connection_id: String,
    pub prefix: MerklePrefix,
}

// Origin: ibc.core.connection.v1 (compiled proto)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Version {
    pub identifier: String,
    pub features: Vec<String>,
}
