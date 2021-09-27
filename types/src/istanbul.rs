use ethereum_types::*;
use fixed_hash::construct_fixed_hash;
use impl_rlp::impl_fixed_hash_rlp;
use impl_serde::impl_fixed_hash_serde;
use rlp::DecoderError;
use rlp_derive::{RlpDecodable, RlpEncodable};

use crate::errors::{Error, Kind};

pub type IstanbulExtraVanity = H256;
construct_fixed_hash! {
    /// SerializedPublicKey is a public key of a validator that is used to i.e sign the validator set in the header
    pub struct SerializedPublicKey(96);
}
impl_fixed_hash_rlp!(SerializedPublicKey, 96);
impl_fixed_hash_serde!(SerializedPublicKey, 96);

///https://pkg.go.dev/github.com/celo-org/celo-blockchain/core/types#IstanbulAggregatedSeal
#[derive(Clone, PartialEq, Debug, Default)]
pub struct IstanbulAggregatedSeal {
    pub bitmap: U128,
    pub signature: Vec<u8>,
    pub round: U128,
}

impl rlp::Decodable for IstanbulAggregatedSeal {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, DecoderError> {
        Ok(IstanbulAggregatedSeal {
            bitmap: rlp.val_at(0)?,
            signature: rlp.val_at(1)?,
            round: rlp.val_at(2)?,
        })
    }
}
impl rlp::Encodable for IstanbulAggregatedSeal {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.begin_list(3);
        s.append(&self.bitmap);
        s.append(&self.signature);
        s.append(&self.round);
    }
}

///https://pkg.go.dev/github.com/celo-org/celo-blockchain/core/types#IstanbulExtra
#[derive(Clone, PartialEq, Debug, Default)]
pub struct IstanbulExtra {
    pub added_validators: Vec<Address>,
    pub added_validators_public_keys: Vec<SerializedPublicKey>,
    pub removed_validators: U128,
    pub seal: Vec<u8>,
    pub aggregated_seal: IstanbulAggregatedSeal,
    pub parent_aggregated_seal: IstanbulAggregatedSeal,
}

impl IstanbulExtra {
    pub(crate) fn from_rlp(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < IstanbulExtraVanity::len_bytes() {
            return Err(Kind::RlpDecodeError
                .context(DecoderError::Custom("invalid istanbul header extra-data"))
                .into());
        }
        rlp::decode(&bytes[IstanbulExtraVanity::len_bytes()..])
            .map_err(|e| Kind::RlpDecodeError.context(e).into())
    }

    pub(crate) fn to_rlp(&self, vanity: &IstanbulExtraVanity) -> Vec<u8> {
        let payload = rlp::encode(self);
        [&vanity[..], &payload[..]].concat()
    }
}

impl rlp::Decodable for IstanbulExtra {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, DecoderError> {
        let added_validators: Result<Vec<Address>, DecoderError> =
            rlp.at(0)?.iter().map(|r| Address::decode(&r)).collect();
        let added_validators_public_keys: Result<Vec<SerializedPublicKey>, DecoderError> = rlp
            .at(1)?
            .iter()
            .map(|r| SerializedPublicKey::decode(&r))
            .collect();
        Ok(IstanbulExtra {
            added_validators: added_validators?,
            added_validators_public_keys: added_validators_public_keys?,
            removed_validators: rlp.val_at(2)?,
            seal: rlp.val_at(3)?,
            aggregated_seal: rlp.val_at(4)?,
            parent_aggregated_seal: rlp.val_at(5)?,
        })
    }
}
impl rlp::Encodable for IstanbulExtra {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.begin_list(6);
        s.begin_list(self.added_validators.len());
        for address in self.added_validators.iter() {
            s.append(&address.as_ref());
        }
        s.begin_list(self.added_validators_public_keys.len());
        for address in self.added_validators_public_keys.iter() {
            s.append(&address.as_ref());
        }
        s.append(&self.removed_validators);
        s.append(&self.seal);
        s.append(&self.aggregated_seal);
        s.append(&self.parent_aggregated_seal);
    }
}

///https://pkg.go.dev/github.com/celo-org/celo-blockchain/consensus/istanbul#ValidatorData
#[derive(Clone, PartialEq, Debug, RlpEncodable, RlpDecodable)]
pub struct ValidatorData {
    pub address: Address,
    pub public_key: SerializedPublicKey,
}
impl From<(Address, SerializedPublicKey)> for ValidatorData {
    fn from((address, public_key): (Address, SerializedPublicKey)) -> Self {
        Self {
            address,
            public_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn istanbul_seal_1() {
        let hexa = String::from("f3882fbf64b1967f8c53a01b97bb9f4bb472e89f5b1484f25209c9d9343e92ba09dd9d52dfd79b4d76429b88680c5ba53b0d9f9f");
        let seal = IstanbulAggregatedSeal {
            bitmap: U128::from(3440579354231278675 as u64),
            signature: hex::decode(
                "1b97bb9f4bb472e89f5b1484f25209c9d9343e92ba09dd9d52dfd79b4d76429b",
            )
            .unwrap(),
            round: U128::from(7497468244883513247 as u64),
        };
        let decoded_seal: IstanbulAggregatedSeal =
            rlp::decode(hex::decode(&hexa).unwrap().as_slice()).unwrap();

        assert_eq!(decoded_seal.bitmap, seal.bitmap, "bitmap");
        assert_eq!(decoded_seal.signature, seal.signature, "signature");
        assert_eq!(decoded_seal.round, seal.round, "round");

        let encoded_seal = hex::encode(rlp::encode(&decoded_seal).to_vec());
        assert_eq!(hexa, encoded_seal);
    }

    #[test]
    fn istanbul_extra_1() {
        let hexa = String::from("f90201f85494000000000000000000000000313585884c14d6c09400000000000000000000000031079b70e0cb1a849400000000000000000000000052bc75d3613f0858940000000000000000000000005e2919f9f41db402f90126b860617a0ce18fda9e6f82e54e748e81e79e4bbd6fe34cdcba843ee8d63e8c4ffe1cebea546d8fac13dd1aac04ce2ea2877c5579cfa2c78e1b0bafae881b82a751108a42ed3c903caa43465a78620616978aed0ce3c6c4f3ae7bc3e0495b5712fefdb860be0c102887e100dacd2d885f692cb607da00a11c1c7071e796a2dc2dc25a5b74b2e129705e273f05c92326828e2b056e3817658e1061498947fdf344410ed4c116023fa8e3576b6fed27ff8974bac0cafd9ad05692b13619e738964dfdc79e8db860534373661cfd66d74fec1e1b89491ab7236e4b75216290cf2beb42c3ca27328560f1aac067cea6e8bf46d4ab2b4680402c5fb2820e885d3260f1de978283d4a09a36f96c20941746e3ed4da646a9ae8b4fa7b4fc3a20bafa1a75ed327a86b8b088781912b5b3d2cff190c39aea79533abf448d2c479d32607533f3881df01cccc8145990a09da39e0e929d024abcbcb169397bb734e7ef0a6e01f1854deb5fe424fef7ac218857aab8a22a295e97f388157333b3cd617c04a00891f5ebe72c27a098c02197dae6b732c351df668f874e2c9f1ce09ca86017e78872ed481ce1231b1c");
        let ista = IstanbulExtra{added_validators: vec![H160::from_str("000000000000000000000000313585884c14d6c0").unwrap(), H160::from_str("00000000000000000000000031079b70e0cb1a84").unwrap(), H160::from_str("00000000000000000000000052bc75d3613f0858").unwrap(), H160::from_str("0000000000000000000000005e2919f9f41db402").unwrap()], added_validators_public_keys: vec![SerializedPublicKey::from_str("617a0ce18fda9e6f82e54e748e81e79e4bbd6fe34cdcba843ee8d63e8c4ffe1cebea546d8fac13dd1aac04ce2ea2877c5579cfa2c78e1b0bafae881b82a751108a42ed3c903caa43465a78620616978aed0ce3c6c4f3ae7bc3e0495b5712fefd").unwrap(), SerializedPublicKey::from_str("be0c102887e100dacd2d885f692cb607da00a11c1c7071e796a2dc2dc25a5b74b2e129705e273f05c92326828e2b056e3817658e1061498947fdf344410ed4c116023fa8e3576b6fed27ff8974bac0cafd9ad05692b13619e738964dfdc79e8d").unwrap(), SerializedPublicKey::from_str("534373661cfd66d74fec1e1b89491ab7236e4b75216290cf2beb42c3ca27328560f1aac067cea6e8bf46d4ab2b4680402c5fb2820e885d3260f1de978283d4a09a36f96c20941746e3ed4da646a9ae8b4fa7b4fc3a20bafa1a75ed327a86b8b0").unwrap()], removed_validators: U128::from(8653968730584436721 as u64), seal: hex::decode("c39aea79533abf448d2c479d32607533").unwrap(), aggregated_seal: IstanbulAggregatedSeal{bitmap: U128::from(2157255887366150544 as u64),signature: hex::decode("9da39e0e929d024abcbcb169397bb734e7ef0a6e01f1854deb5fe424fef7ac21").unwrap(),round: U128::from(6317064433972108951 as u64)},parent_aggregated_seal: IstanbulAggregatedSeal{bitmap: U128::from(1545635944456092676 as u64),signature: hex::decode("0891f5ebe72c27a098c02197dae6b732c351df668f874e2c9f1ce09ca86017e7").unwrap(),round: U128::from(8281354578677668636 as u64)}};
        let extra_bytes = prepend_vanity(&hexa);
        let decoded_ist = IstanbulExtra::from_rlp(&extra_bytes).unwrap();

        assert_eq!(decoded_ist.added_validators, ista.added_validators);
        assert_eq!(
            decoded_ist.added_validators_public_keys,
            ista.added_validators_public_keys
        );
        assert_eq!(decoded_ist.removed_validators, ista.removed_validators);
        assert_eq!(decoded_ist.seal, ista.seal);
        assert_eq!(decoded_ist.aggregated_seal, ista.aggregated_seal);
        assert_eq!(
            decoded_ist.parent_aggregated_seal,
            ista.parent_aggregated_seal
        );

        let encoded_ist = hex::encode(rlp::encode(&decoded_ist).to_vec());
        assert_eq!(hexa, encoded_ist);
    }

    #[test]
    fn rejects_insufficient_vanity() {
        let bytes = vec![0; IstanbulExtraVanity::len_bytes() - 1];

        assert!(IstanbulExtra::from_rlp(&bytes).is_err());
    }

    fn prepend_vanity(data: &str) -> Vec<u8> {
        let data = hex::decode(data).unwrap();
        let vanity = IstanbulExtraVanity::default();

        [&vanity[..], &data[..]].concat()
    }
}
