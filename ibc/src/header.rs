use crate::{Error, Kind};
use ibc_proto::ibc::core::client::v1::Height;
pub use ibc_proto::ibc::lightclients::wasm::v1::ClientState as IBCClientState;
pub use ibc_proto::ibc::lightclients::wasm::v1::ConsensusState as IBCConsensusState;
pub use ibc_proto::ibc::lightclients::wasm::v1::Header as IBCHeader;
pub use ibc_proto::ibc::lightclients::wasm::v1::Misbehaviour as IBCMisbehaviour;
use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Header {
    pub data: String,
    pub height: Height,
}
impl TryFrom<IBCHeader> for Header {
    type Error = Error;
    fn try_from(ibc: IBCHeader) -> Result<Self, Self::Error> {
        let s = Self {
            data: base64::encode(ibc.data),
            height: ibc.height.ok_or_else(|| Kind::MissingField {
                struct_name: String::from("IBCHeader"),
                field_name: String::from("height"),
            })?,
        };
        Ok(s)
    }
}
impl TryFrom<Header> for IBCHeader {
    type Error = base64::DecodeError;
    fn try_from(h: Header) -> Result<Self, Self::Error> {
        let s = Self {
            data: base64::decode(&h.data)?,
            height: Some(h.height),
        };
        Ok(s)
    }
}

pub fn extract_header(h: &Header) -> Result<celo_types::Header, Error> {
    let v: Vec<u8> = base64::decode(&h.data).map_err(|e| {
        let k: Kind = e.into();
        let e: Error = k.into();
        e
    })?;
    rlp::decode(&v).map_err(|e| {
        let k: Kind = e.into();
        k.into()
    })
}
