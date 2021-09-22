use crate::{Error, IBCMerklePrefix, Kind, MerklePrefix};
pub use ibc_proto::ibc::core::connection::v1::{
    ConnectionEnd as IBCConnectionEnd, Counterparty as IBCCounterparty, Version as IBCVersion,
};
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
impl TryFrom<IBCConnectionEnd> for ConnectionEnd {
    type Error = Error;
    fn try_from(ibc: IBCConnectionEnd) -> Result<Self, Self::Error> {
        let counter = if let Some(cp) = ibc.counterparty {
            Some(Counterparty::try_from(cp)?)
        } else {
            None
        };
        let s = Self {
            client_id: ibc.client_id,
            versions: ibc.versions.into_iter().map(Version::from).collect(),
            state: ibc.state,
            counterparty: counter.ok_or_else(|| Kind::MissingField {
                struct_name: String::from("ConnectionEnd"),
                field_name: String::from("Counterparty"),
            })?,
            delay_period: ibc.delay_period,
        };
        Ok(s)
    }
}

// Origin: ibc.core.connection.v1 (compiled proto)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Counterparty {
    pub client_id: String,
    pub connection_id: String,
    pub prefix: MerklePrefix,
}
impl TryFrom<IBCCounterparty> for Counterparty {
    type Error = Error;
    fn try_from(c: IBCCounterparty) -> Result<Self, Self::Error> {
        let s = Self {
            client_id: c.client_id,
            connection_id: c.connection_id,
            prefix: c
                .prefix
                .map(MerklePrefix::from)
                .ok_or_else(|| Kind::MissingField {
                    struct_name: String::from("Counterparty"),
                    field_name: String::from("prefix"),
                })?,
        };
        Ok(s)
    }
}
impl TryFrom<Counterparty> for IBCCounterparty {
    type Error = base64::DecodeError;
    fn try_from(c: Counterparty) -> Result<Self, Self::Error> {
        let s = Self {
            client_id: c.client_id,
            connection_id: c.connection_id,
            prefix: Some(IBCMerklePrefix::try_from(c.prefix)?),
        };
        Ok(s)
    }
}

// Origin: ibc.core.connection.v1 (compiled proto)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Version {
    pub identifier: String,
    pub features: Vec<String>,
}
impl From<IBCVersion> for Version {
    fn from(v: IBCVersion) -> Self {
        Self {
            identifier: v.identifier,
            features: v.features,
        }
    }
}
impl From<Version> for IBCVersion {
    fn from(v: Version) -> Self {
        Self {
            identifier: v.identifier,
            features: v.features,
        }
    }
}
