use crate::bls::verify_aggregated_seal;
use crate::errors::{Error, Kind};
use crate::serialization::rlp::{rlp_field_from_bytes, rlp_list_field_from_bytes};
use crate::traits::{FromRlp, StateConfig, ToRlp};
use crate::types::header::{Address, Hash};
use crate::types::istanbul::{IstanbulAggregatedSeal, SerializedPublicKey};

use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use rlp_derive::{RlpDecodable, RlpEncodable};

/// Validator identifies block producer by public key and address
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Validator {
    #[serde(with = "crate::serialization::bytes::hexstring")]
    pub address: Address,

    #[serde(with = "crate::serialization::bytes::hexstring")]
    pub public_key: SerializedPublicKey,
}

impl Encodable for Validator {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);

        s.append(&self.address.as_ref());
        s.append(&self.public_key.as_ref());
    }
}

impl Decodable for Validator {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Validator {
            address: rlp_field_from_bytes(&rlp.at(0)?)?,
            public_key: rlp_field_from_bytes(&rlp.at(1)?)?,
        })
    }
}

impl ToRlp for Vec<Validator> {
    fn to_rlp(&self) -> Vec<u8> {
        rlp::encode_list(&self)
    }
}

impl FromRlp for Vec<Validator> {
    fn from_rlp(bytes: &[u8]) -> Result<Self, Error> {
        Ok(rlp::decode_list(&bytes))
    }
}

/// Config contains state related configuration flags
#[derive(Serialize, Deserialize, RlpEncodable, RlpDecodable, Clone, PartialEq, Eq, Debug)]
pub struct Config {
    pub epoch_size: u64,
    pub allowed_clock_skew: u64,
    pub verify_epoch_headers: bool,
    pub verify_non_epoch_headers: bool,
    pub verify_header_timestamp: bool,
}

impl ToRlp for Config {
    fn to_rlp(&self) -> Vec<u8> {
        rlp::encode(self)
    }
}

impl FromRlp for Config {
    fn from_rlp(bytes: &[u8]) -> Result<Self, Error> {
        rlp::decode(&bytes).map_err(|e| Kind::RlpDecodeError.context(e).into())
    }
}

impl StateConfig for Config {
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

/// Snapshot represents an IBFT consensus state at specified block height
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Snapshot {
    /// Block number at which the snapshot was created
    pub number: u64,

    /// Block creation time
    pub timestamp: u64,

    /// Snapshot of current validator set
    pub validators: Vec<Validator>,

    // Hash and aggregated seal are required to validate the header against the validator set
    /// Block hash
    pub hash: Hash,

    /// Block aggregated seal
    pub aggregated_seal: IstanbulAggregatedSeal,
}

impl Encodable for Snapshot {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(5);

        s.append(&self.number);
        s.append(&self.timestamp);

        s.begin_list(self.validators.len());
        for validator in self.validators.iter() {
            s.append(validator);
        }

        s.append(&self.hash.as_ref());
        s.append(&self.aggregated_seal);
    }
}

impl Decodable for Snapshot {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let validators: Result<Vec<Validator>, DecoderError> =
            rlp.at(2)?.iter().map(|r| r.as_val()).collect();

        Ok(Snapshot {
            validators: validators?,
            number: rlp.val_at(0)?,
            timestamp: rlp.val_at(1)?,
            hash: rlp_list_field_from_bytes(rlp, 3)?,
            aggregated_seal: rlp.val_at(4)?,
        })
    }
}

impl Snapshot {
    pub fn new() -> Self {
        Self {
            number: 0,
            timestamp: 0,
            validators: Vec::new(),
            hash: Hash::default(),
            aggregated_seal: IstanbulAggregatedSeal::new(),
        }
    }

    pub fn verify(&self) -> Result<(), Error> {
        verify_aggregated_seal(self.hash, &self.validators, &self.aggregated_seal)
    }
}

impl ToRlp for Snapshot {
    fn to_rlp(&self) -> Vec<u8> {
        rlp::encode(self)
    }
}

impl FromRlp for Snapshot {
    fn from_rlp(bytes: &[u8]) -> Result<Self, Error> {
        rlp::decode(&bytes).map_err(|e| Kind::RlpDecodeError.context(e).into())
    }
}
