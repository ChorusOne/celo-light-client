// SOURCE: core/types/block.go
// SOURCE: common/types.go
// SOURCe: core/types/bloom9.go

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

    #[serde(with = "crate::serialization::bytes::hexnum")]
    pub number: u64, // NOTE: originally big.Int but core/types/block.go#L91 indicates uint64

    #[serde(with = "crate::serialization::bytes::hexnum")]
    pub gas_used: u64,

    #[serde(rename = "timestamp")]
    #[serde(with = "crate::serialization::bytes::hexnum")]
    pub time: u64,

    #[serde(with = "crate::serialization::bytes::hexstring")]
    #[serde(rename = "extraData")]
    pub extra: Vec<u8>
}
