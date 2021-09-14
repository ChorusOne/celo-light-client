use crate::errors::{Error, Kind};
use crate::traits::{FromRlp, ToRlp, StateConfig};
use crate::types::state::Snapshot;

use rlp_derive::{RlpEncodable, RlpDecodable};

pub type LightConsensusState = Snapshot;

#[derive(Serialize, Deserialize, RlpDecodable, RlpEncodable, Clone, PartialEq, Debug)]
pub struct LightClientState {
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

impl ToRlp for LightClientState {
    fn to_rlp(&self) -> Vec<u8> {
        rlp::encode(self)
    }
}

impl FromRlp for LightClientState {
    fn from_rlp(bytes: &[u8]) -> Result<Self, Error> {
        match rlp::decode(&bytes) {
            Ok(config) => Ok(config),
            Err(err) => Err(Kind::RlpDecodeError.context(err).into()),
        }
    }
}

impl StateConfig for LightClientState {
    fn epoch_size(&self) -> u64 { self.epoch_size }
    fn allowed_clock_skew(&self) -> u64 { self.allowed_clock_skew }

    fn verify_epoch_headers(&self) -> bool { self.verify_epoch_headers }
    fn verify_non_epoch_headers(&self) -> bool { self.verify_non_epoch_headers }
    fn verify_header_timestamp(&self) -> bool { self.verify_header_timestamp }
}
