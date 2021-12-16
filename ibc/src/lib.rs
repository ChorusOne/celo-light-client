pub mod error;
pub mod header;
pub mod misbehaviour;
pub mod state;
pub mod wasm;
pub mod conversions;


use error::{Error, Kind};

use ethereum_types::H256;
use ibc_proto::ibc::core::commitment::v1::MerkleRoot;

pub fn convert_hash2root(h: H256) -> MerkleRoot {
    MerkleRoot {
        hash: h.as_bytes().to_vec(),
    }
}
pub fn convert_root2hash(root: MerkleRoot) -> H256 {
    H256::from_slice(root.hash.as_slice())
}
