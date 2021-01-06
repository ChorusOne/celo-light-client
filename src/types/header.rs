use crate::istanbul::istanbul_filtered_header;
use crate::types::istanbul::ISTANBUL_EXTRA_VANITY_LENGTH;
use crate::traits::default::{DefaultFrom, FromBytes};
use crate::slice_as_array_ref;
use crate::errors::Error;
use rug::{integer::Order, Integer};
use rlp::{Encodable, RlpStream};
use sha3::{Digest, Keccak256};

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

impl Header {
    pub fn new() -> Self {
        Self {
            parent_hash: Hash::default(),
            coinbase: Address::default(),
            root: Hash::default(),
            tx_hash: Hash::default(),
            receipt_hash: Hash::default(),
            bloom: Bloom::default(),
            number: Integer::default(),
            gas_used: u64::default(),
            time: u64::default(),
            extra: Vec::default(),
        }
    }

    pub fn hash(&self) -> Result<Hash, Error> {
        if self.extra.len() >= ISTANBUL_EXTRA_VANITY_LENGTH {
            let istanbul_header = istanbul_filtered_header(&self, true);
            if istanbul_header.is_ok() {
                return rlp_hash(&istanbul_header.unwrap());
            }
        }

        rlp_hash(self)
    }
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
        s.append(&self.number.to_digits(Order::Msf));

        // gas_used
        s.append(&self.gas_used);

        // time
        s.append(&self.time);

        // extra
        s.append(&self.extra);
    }
}

impl DefaultFrom for Bloom {
    fn default() -> Self {
        [0; BLOOM_BYTE_LENGTH]
    }
}

impl FromBytes for Address {
    fn from_bytes(data: &[u8]) -> Result<&Address, Error> {
        slice_as_array_ref!(
            &data[..ADDRESS_LENGTH],
            ADDRESS_LENGTH
        )
    }
}

fn rlp_hash(header: &Header) -> Result<Hash, Error> {
    let digest = Keccak256::digest(&rlp::encode(header));

    Ok(slice_as_array_ref!(&digest[..HASH_LENGTH], HASH_LENGTH)?.to_owned())
}
