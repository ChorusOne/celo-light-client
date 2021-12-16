use crate::{Error, Kind};
use ibc_proto::ibc::core::client::v1::Height;
use cosmwasm_std::Binary;

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Header<T> {
    pub data: Binary,
    pub height: Height,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Header<T> {
    pub fn from_raw(data: Binary, height: Height) -> Self {
        Self{data, height, phantom: std::marker::PhantomData}
    }
}

impl<T: rlp::Encodable> Header<T> {
    pub fn new(ch: &T, height: Height) -> Self {
        let r = rlp::encode(ch);
        Self {
            data: Binary::from(r.as_ref()),
            height,
            phantom: std::marker::PhantomData,
        }
    }
}

pub fn extract_header<T: rlp::Decodable>(h: &Header<T>) -> Result<T, Error> {
    rlp::decode(&h.data).map_err(|e| {
        let k: Kind = e.into();
        k.into()
    })
}
