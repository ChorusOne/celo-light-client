use crate::{Error, Kind};
use std::convert::TryFrom;

// Origin: ibc.core.channel.v1 (compiled proto)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Channel {
    pub state: i32,
    pub ordering: i32,
    pub counterparty: Counterparty,
    pub connection_hops: Vec<String>,
    pub version: String,
}

// Origin: ibc.core.channel.v1 (compiled proto)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Counterparty {
    pub port_id: String,
    pub channel_id: String,
}
