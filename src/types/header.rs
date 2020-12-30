// SOURCE: core/types/block.go
// SOURCE: common/types.go
// SOURCe: core/types/bloom9.go

use rug::{integer::Order, Integer};
use rlp::{Encodable, RlpStream};
use sha3::{Digest, Keccak256};
use crate::istanbul::istanbul_filtered_header;
use crate::types::istanbul::{ISTANBUL_EXTRA_VANITY_LENGTH};

/// HASH_LENGTH represents the number of bytes used in a header hash
pub const HASH_LENGTH: usize = 32;

/// ADDRESS_LENGTH represents the number of bytes used in a header Ethereum account address
pub const ADDRESS_LENGTH: usize = 20;

/// BLOOM_BYTE_LENGTH represents the number of bytes used in a header log bloom
pub const BLOOM_BYTE_LENGTH: usize = 256;

/// Hash is the output of the cryptographic digest function
pub type Hash = [u8; HASH_LENGTH];

/// Address represents the 20 byte address of an Ethereum account
pub type Address = [u8; ADDRESS_LENGTH];

/// Bloom represents a 2048 bit bloom filter
pub type Bloom = [u8; BLOOM_BYTE_LENGTH];

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    #[serde(with = "crate::serialization::bytes::hexstring")]
    pub parent_hash: Hash,

    #[serde(with = "crate::serialization::bytes::hexstring")]
    #[serde(rename = "miner")]
    pub coinbase: Address,

    #[serde(with = "crate::serialization::bytes::hexstring")]
    #[serde(rename = "stateRoot")]
    pub root: Hash,

    #[serde(with = "crate::serialization::bytes::hexstring")]
    #[serde(rename = "transactionsRoot")]
    pub tx_hash: Hash,

    #[serde(with = "crate::serialization::bytes::hexstring")]
    #[serde(rename = "receiptsRoot")]
    pub receipt_hash: Hash,

    #[serde(with = "crate::serialization::bytes::hexstring")]
    #[serde(rename = "logsBloom")]
    pub bloom: Bloom,

    #[serde(with = "crate::serialization::bytes::hexbigint")]
    pub number: Integer,

    #[serde(with = "crate::serialization::bytes::hexnum")]
    pub gas_used: u64,

    #[serde(rename = "timestamp")]
    #[serde(with = "crate::serialization::bytes::hexnum")]
    pub time: u64,

    #[serde(with = "crate::serialization::bytes::hexstring")]
    #[serde(rename = "extraData")]
    pub extra: Vec<u8>
}

impl Encodable for Header {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(10);

        // parent_hash
        s.append(&self.parent_hash.to_vec());

        // coinbase
        s.append(&self.coinbase.to_vec()); // TODO: can we do it without conversion?

        // root
        s.append(&self.root.to_vec());

        // tx_hash
        s.append(&self.tx_hash.to_vec());

        // receipt_hash
        s.append(&self.receipt_hash.to_vec());

        // bloom
        s.append(&self.bloom.to_vec());

        // number
        s.append(&self.number.to_digits(Order::LsfBe));

        // gas_used
        s.append(&self.gas_used);

        // time
        s.append(&self.time);

        // extra
        s.append(&self.extra);
    }
}

impl Header {
    pub fn hash(&self) -> Hash {
        if self.extra.len() >= ISTANBUL_EXTRA_VANITY_LENGTH {
            let istanbul_header = istanbul_filtered_header(&self, true);
            if istanbul_header.is_ok() {
                return rlp_hash(&istanbul_header.unwrap());
            }
        }

        rlp_hash(self)
    }
}

fn rlp_hash(header: &Header) -> Hash {
    let digest = Keccak256::digest(&rlp::encode(header));
    let mut hash: Hash = Hash::default();

    hash.copy_from_slice(digest.as_slice());
    hash
}
