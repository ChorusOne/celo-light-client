use crate::{Error, Kind};
use ibc_proto::ibc::core::channel::v1::{Channel as IBCChannel, Counterparty as IBCCounterparty};
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
impl TryFrom<IBCChannel> for Channel {
    type Error = Error;
    fn try_from(c: IBCChannel) -> Result<Self, Self::Error> {
        let s = Self {
            state: c.state,
            ordering: c.ordering,
            counterparty: c.counterparty.map(Counterparty::from).ok_or_else(|| {
                Kind::MissingField {
                    struct_name: String::from("Channel"),
                    field_name: String::from("counterparty"),
                }
            })?,
            connection_hops: c.connection_hops,
            version: c.version,
        };
        Ok(s)
    }
}
impl From<Channel> for IBCChannel {
    fn from(c: Channel) -> Self {
        Self {
            state: c.state,
            ordering: c.ordering,
            counterparty: Some(IBCCounterparty::from(c.counterparty)),
            connection_hops: c.connection_hops,
            version: c.version,
        }
    }
}

// Origin: ibc.core.channel.v1 (compiled proto)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Counterparty {
    pub port_id: String,
    pub channel_id: String,
}
impl From<IBCCounterparty> for Counterparty {
    fn from(c: IBCCounterparty) -> Self {
        Self {
            port_id: c.port_id,
            channel_id: c.channel_id,
        }
    }
}
impl From<Counterparty> for IBCCounterparty {
    fn from(c: Counterparty) -> Self {
        Self {
            port_id: c.port_id,
            channel_id: c.channel_id,
        }
    }
}
