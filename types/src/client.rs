use rlp_derive::{RlpDecodable, RlpEncodable};

#[derive(RlpDecodable, RlpEncodable, Clone, PartialEq, Debug, Default)]
pub struct LightClientState {
    pub chain_id: u64,
    pub epoch_size: u64,
    pub allowed_clock_skew: u64,
    pub trusting_period: u64,
    pub upgrade_path: Vec<String>,

    pub verify_epoch_headers: bool,
    pub verify_non_epoch_headers: bool,
    pub verify_header_timestamp: bool,

    pub allow_update_after_misbehavior: bool,
    pub allow_update_after_expiry: bool,
}

/// Config contains state related configuration flags
#[derive(RlpEncodable, RlpDecodable, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(any(test, feature = "serialize"), derive(serde::Deserialize))]
pub struct Config {
    pub chain_id: u64,
    pub epoch_size: u64,
    pub allowed_clock_skew: u64,
    pub verify_epoch_headers: bool,
    pub verify_non_epoch_headers: bool,
    pub verify_header_timestamp: bool,
}

pub trait StateConfig {
    /// ChainID as in eth_chainId
    fn chain_id(&self) -> u64;
    /// Epoch size expressed in number of blocks
    fn epoch_size(&self) -> u64;
    /// Defines how far block timestamp can go in the future
    fn allowed_clock_skew(&self) -> u64;
    /// Whether to validate (BLS signature) epoch headers. It should always be set to true.
    fn verify_epoch_headers(&self) -> bool;
    /// Whether to validate (BLS signature) non epoch headers. Since non-epoch don't affect
    /// validator set, it's acceptable to disable validation
    fn verify_non_epoch_headers(&self) -> bool;
    /// Whether to verify headers time against current time. It's recommended to keep it true
    fn verify_header_timestamp(&self) -> bool;
}

impl StateConfig for LightClientState {
    fn chain_id(&self) -> u64 {
        self.chain_id
    }
    fn epoch_size(&self) -> u64 {
        self.epoch_size
    }
    fn allowed_clock_skew(&self) -> u64 {
        self.allowed_clock_skew
    }

    fn verify_epoch_headers(&self) -> bool {
        self.verify_epoch_headers
    }
    fn verify_non_epoch_headers(&self) -> bool {
        self.verify_non_epoch_headers
    }
    fn verify_header_timestamp(&self) -> bool {
        self.verify_header_timestamp
    }
}
impl StateConfig for Config {
    fn chain_id(&self) -> u64 {
        self.chain_id
    }
    fn epoch_size(&self) -> u64 {
        self.epoch_size
    }
    fn allowed_clock_skew(&self) -> u64 {
        self.allowed_clock_skew
    }

    fn verify_epoch_headers(&self) -> bool {
        self.verify_epoch_headers
    }
    fn verify_non_epoch_headers(&self) -> bool {
        self.verify_non_epoch_headers
    }
    fn verify_header_timestamp(&self) -> bool {
        self.verify_header_timestamp
    }
}
