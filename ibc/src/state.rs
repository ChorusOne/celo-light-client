use crate::{Error, Kind};
use cosmwasm_std::Binary;
use ibc_proto::ibc::core::client::v1::Height;
use ethereum_types::H256;


#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ConsensusState<T> {
    pub data: Binary,
    pub timestamp: u64,
    root: [u8;H256::len_bytes()], // H256 does not derive JsonSchema
    phantom: std::marker::PhantomData<T>,
}

impl<T> ConsensusState<T> {
    pub fn from_raw(data: Binary, timestamp: u64, root: H256) -> Self {
        Self{data, timestamp, root: root.0, phantom: std::marker::PhantomData}
    }
    pub fn root(&self) -> H256 {
        H256::from(self.root)
    }
}

impl<T: rlp::Encodable> ConsensusState<T> {
    pub fn new(lc: &T, timestamp: u64, root: H256) -> Self {
        let r = rlp::encode(lc);
        Self {
            data: Binary::from(r.as_ref()),
            timestamp,
            root:   root.0,
            phantom: std::marker::PhantomData,
        }
    }
}

pub fn extract_consensus<T: rlp::Decodable>(cs: &ConsensusState<T>) -> Result<T, Error> {
    rlp::decode(&cs.data).map_err(|e| {
        let k: Kind = e.into();
        k.into()
    })
}

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ClientState<T> {
    pub data: cosmwasm_std::Binary,
    pub code_id: cosmwasm_std::Binary,
    pub latest_height: Height,
    pub frozen_height: Option<Height>,
    phantom: std::marker::PhantomData<T>,
}

impl<T> ClientState<T> {
    pub fn from_raw(data: Binary, code_id: Binary, latest_height: Height, frozen_height: Option<Height>) -> Self {
        Self{data, code_id, latest_height, frozen_height, phantom: std::marker::PhantomData}
    }
}

impl<T: rlp::Encodable> ClientState<T> {
    pub fn new(lc: &T, code_id: Binary, latest_height: Height) -> Self {
        let r = rlp::encode(lc);
        Self {
            data: Binary::from(r.as_ref()),
            code_id,
            latest_height,
            frozen_height: None,
            phantom: std::marker::PhantomData,
        }
    }
}

pub fn extract_client<T: rlp::Decodable>(cs: &ClientState<T>) -> Result<T, Error> {
    rlp::decode(&cs.data).map_err(|e| {
        let k: Kind = e.into();
        k.into()
    })
}
