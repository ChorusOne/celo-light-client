use crate::errors::Error;
use ethereum_types::*;
use fixed_hash::construct_fixed_hash;
use impl_rlp::impl_fixed_hash_rlp;
use rlp::DecoderError;
use rlp_derive::{RlpDecodable, RlpEncodable};

pub type IstanbulExtraVanity = H256;
construct_fixed_hash! {
    /// SerializedPublicKey is a public key of a validator that is used to i.e sign the validator set in the header
    pub struct SerializedPublicKey(96);
}
impl_fixed_hash_rlp!(SerializedPublicKey, 96);
#[cfg(any(test, feature = "serialize"))]
impl_serde::impl_fixed_hash_serde!(SerializedPublicKey, 96);

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
            return Err(Error::IstanbulDataLength{current: bytes.len(), expected: IstanbulExtraVanity::len_bytes()});
        }
        rlp::decode(&bytes[IstanbulExtraVanity::len_bytes()..])
            .map_err(|e| Error::RlpDecodeError(String::from("IstanbulExtra"), e))
    }

    #[cfg(test)]
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
#[cfg_attr(
    any(test, feature = "serialize"),
    derive(serde::Deserialize, serde::Serialize)
)]
pub struct ValidatorData {
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "Address"))]
    pub address: Address,
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "BLSPublicKey"))]
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

pub enum IstanbulMsg {
    PrePrepare,
    Prepare,
    Commit,
    RoundChange,
}

pub fn min_quorum_size(total_validators: usize) -> usize {
    // non-float equivalent of:
    //  ((2.0*(total_validators as f64) / 3.0) as f64).ceil() as usize
    ((2 * total_validators) - 1 + 3) / 3
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn istanbul_seal_1() {
        let hexa = String::from("f38808e42fb750c1e60da014abd5c47c0be87b0454596baad2e62829913e9dcecbf2b9dee45c6606908694884ef27ef395ccba5e");
        let seal = IstanbulAggregatedSeal {
            bitmap: U128::from(640689511373858317 as u64),
            signature: hex::decode(
                "14abd5c47c0be87b0454596baad2e62829913e9dcecbf2b9dee45c6606908694",
            )
            .unwrap(),
            round: U128::from(5688748863977732702 as u64),
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
        let hexa = String::from("f90201f854940000000000000000000000003ff5908bd346cea0940000000000000000000000005a24d687edd344f1940000000000000000000000005872dec9a7cf7f8894000000000000000000000000285374a5f56fc72df90126b8600b9f55af8d098bf818245df362fdc5959b4842e7c0c2a85baeed05fa472c07ec44b2157eb5a3aa6710e245455581e2a7a2671859efb7a32e7a3ff17e369aeba96bb1e1ade64a88d6f0ff14deeb373e542956311aa953d9c11a47dd898dcec4fdb860a72a1700ec91a111ec79588ba850b0c82b66c651760a6433b9618e59798a46e9779dd48996d28e22f71b2950cd3bebc9e12631575bf9e38b646e01da4da873d22b7e7e451f0474f2a51803421304f75e13536aa899ed6356ce2229ee33f82a91b86022e7ed76440face71019d1c68acb1485bb448c105368b00b70eaa1819d70e8ecb40381b248feef78e2a2a185aba377171674537cca30858606ef9e253bab4e5bc52e8b45e28ae82b86b0947bf7ac20e7b62c698b4b79da1d33de6a27b0c195c48858d199624d0959d190e41841b4fbb3b12647692a334a290588f38870448e3a9d2a2d26a07e9b3813ee0b1e8df15ee53509572dadd1f7241f7e1f1980b6cd7f3032dcbf1a88494e0234ac4c502ef3883ca13b76e2c8b14aa0fcc908a347af29c6432de41537b4c4dcdb23689d41b24a07f42dfacacec2cf418865e5c8804f910d7f");
        let ista = IstanbulExtra{added_validators: vec![H160::from_str("0000000000000000000000003ff5908bd346cea0").unwrap(), H160::from_str("0000000000000000000000005a24d687edd344f1").unwrap(), H160::from_str("0000000000000000000000005872dec9a7cf7f88").unwrap(), H160::from_str("000000000000000000000000285374a5f56fc72d").unwrap()], added_validators_public_keys: vec![SerializedPublicKey::from_str("0b9f55af8d098bf818245df362fdc5959b4842e7c0c2a85baeed05fa472c07ec44b2157eb5a3aa6710e245455581e2a7a2671859efb7a32e7a3ff17e369aeba96bb1e1ade64a88d6f0ff14deeb373e542956311aa953d9c11a47dd898dcec4fd").unwrap(), SerializedPublicKey::from_str("a72a1700ec91a111ec79588ba850b0c82b66c651760a6433b9618e59798a46e9779dd48996d28e22f71b2950cd3bebc9e12631575bf9e38b646e01da4da873d22b7e7e451f0474f2a51803421304f75e13536aa899ed6356ce2229ee33f82a91").unwrap(), SerializedPublicKey::from_str("22e7ed76440face71019d1c68acb1485bb448c105368b00b70eaa1819d70e8ecb40381b248feef78e2a2a185aba377171674537cca30858606ef9e253bab4e5bc52e8b45e28ae82b86b0947bf7ac20e7b62c698b4b79da1d33de6a27b0c195c4").unwrap()], removed_validators: U128::from(6400065192948488657 as u64), seal: hex::decode("e41841b4fbb3b12647692a334a290588").unwrap(), aggregated_seal: IstanbulAggregatedSeal{bitmap: U128::from(8089747213060287782 as u64),signature: hex::decode("7e9b3813ee0b1e8df15ee53509572dadd1f7241f7e1f1980b6cd7f3032dcbf1a").unwrap(),round: U128::from(5282161838204407854 as u64)},parent_aggregated_seal: IstanbulAggregatedSeal{bitmap: U128::from(4368838495323074890 as u64),signature: hex::decode("fcc908a347af29c6432de41537b4c4dcdb23689d41b24a07f42dfacacec2cf41").unwrap(),round: U128::from(7342495220913737087 as u64)}};
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
