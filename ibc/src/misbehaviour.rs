
use crate::{Header, Error, Kind};
pub use ibc_proto::ibc::lightclients::wasm::v1::ClientState as IBCClientState;
pub use ibc_proto::ibc::lightclients::wasm::v1::ConsensusState as IBCConsensusState;
pub use ibc_proto::ibc::lightclients::wasm::v1::Header as IBCHeader;
pub use ibc_proto::ibc::lightclients::wasm::v1::Misbehaviour as IBCMisbehaviour;
use std::convert::{From, TryFrom, TryInto};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Misbehaviour {
    pub client_id: String,
    pub header_1: Header,
    pub header_2: Header,
}
impl TryFrom<IBCMisbehaviour> for Misbehaviour {
    type Error = Error;
    fn try_from(ibc: IBCMisbehaviour) -> Result<Self, Self::Error> {
        let head1 = ibc.header_1.ok_or_else(|| Kind::MissingField {
            struct_name: String::from("IBCMisbehaviour"),
            field_name: String::from("header_1"),
        })?;
        let head2 = ibc.header_2.ok_or_else(|| Kind::MissingField {
            struct_name: String::from("IBCMisbehaviour"),
            field_name: String::from("header_1"),
        })?;

        let s = Self {
            client_id: ibc.client_id,
            header_1: Header::try_from(head1)?,
            header_2: Header::try_from(head2)?,
        };
        Ok(s)
    }
}
impl TryFrom<Misbehaviour> for IBCMisbehaviour {
    type Error = base64::DecodeError;
    fn try_from(mis: Misbehaviour) -> Result<Self, Self::Error> {
        let s = Self {
            client_id: mis.client_id,
            header_1: Some(mis.header_1.try_into()?),
            header_2: Some(mis.header_2.try_into()?),
        };
        Ok(s)
    }
}

