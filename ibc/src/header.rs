use crate::{Error, Kind, Height};
use std::convert::{From, TryFrom};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Header {
    pub data: String,
    pub height: Height,
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
