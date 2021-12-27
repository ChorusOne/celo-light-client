use crate::Error;
use ibc_proto::ibc::core::client::v1::Height;
use cosmwasm_std::Binary;

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Header {
    pub data: Binary,
    pub height: Height,
}

impl Header {
    pub fn from_raw(data: Binary, height: Height) -> Self {
        Self{data, height}
    }
    pub fn new<T: rlp::Encodable>(ch: &T, height: Height) -> Self {
        let r = rlp::encode(ch);
        Self {
            data: Binary::from(r.as_ref()),
            height,
        }
    }
}

pub fn extract_header<T: rlp::Decodable>(h: &Header) -> Result<T, rlp::DecoderError> {
    rlp::decode(&h.data)
}
