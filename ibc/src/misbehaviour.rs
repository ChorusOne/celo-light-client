
use crate::{Header, Error, Kind};
use std::convert::{From, TryFrom, TryInto};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Misbehaviour {
    pub client_id: String,
    pub header_1: Header,
    pub header_2: Header,
}

