use crate::Error;

use cosmwasm_std::Binary;
use ethereum_types::H256;
use ibc_proto::ibc::core::client::v1::Height;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ConsensusState {
    pub data: Binary,
    pub timestamp: u64,
    root: [u8; H256::len_bytes()], // H256 does not derive JsonSchema
}

impl ConsensusState {
    pub fn new<T: rlp::Encodable>(lc: &T, timestamp: u64, root: H256) -> Self {
        let r = rlp::encode(lc);
        Self {
            data: Binary::from(r.as_ref()),
            timestamp,
            root: root.0,
        }
    }
    pub fn from_raw(data: Binary, timestamp: u64, root: H256) -> Self {
        Self {
            data,
            timestamp,
            root: root.0,
        }
    }
    pub fn root(&self) -> H256 {
        H256::from(self.root)
    }
}

pub fn extract_consensus<T: rlp::Decodable>(cs: &ConsensusState) -> Result<T, Error> {
    rlp::decode(&cs.data).map_err(Error::from)
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ClientState {
    pub data: cosmwasm_std::Binary,
    pub code_id: cosmwasm_std::Binary,
    pub latest_height: Height,
    pub frozen_height: Option<Height>,
}

impl ClientState {
    pub fn from_raw(
        data: Binary,
        code_id: Binary,
        latest_height: Height,
        frozen_height: Option<Height>,
    ) -> Self {
        Self {
            data,
            code_id,
            latest_height,
            frozen_height,
        }
    }
    pub fn new<T: rlp::Encodable>(lc: &T, code_id: Binary, latest_height: Height) -> Self {
        let r = rlp::encode(lc);
        Self {
            data: Binary::from(r.as_ref()),
            code_id,
            latest_height,
            frozen_height: None,
        }
    }
}

pub fn extract_client<T: rlp::Decodable>(cs: &ClientState) -> Result<T, Error> {
    rlp::decode(&cs.data).map_err(Error::from)
}
