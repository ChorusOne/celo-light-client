use crate::types::header::Address;
use crate::types::header::ADDRESS_LENGTH;

use rlp::{DecoderError, Decodable, Rlp};
use rlp_derive::RlpDecodable;

// SOURCE: github.com/celo-org/celo-bls-go@v0.1.6/bls/bls.go
pub const PUBLIC_KEY_LENGTH: usize = 96;

// SOURCE: crypto/bls/bls.go
pub type SerializedPublicKey = [u8; PUBLIC_KEY_LENGTH];

// SOURCE: core/types/istanbul.go
pub const ISTANBUL_EXTRA_VANITY_LENGTH: usize = 32;

#[derive(RlpDecodable, Clone, PartialEq, Debug)]
pub struct IstanbulAggregatedSeal {
    /// Bitmap is a bitmap having an active bit for each validator that signed this block
    pub bitmap: u64, // NOTE it was a big.Int

    /// Signature is an aggregated BLS signature resulting from signatures by each validator that signed this block
    pub signature: Vec<u8>,

    /// Round is the round in which the signature was created.
    pub round: u64, // NOTE: it was a big.Int
}

#[derive(Clone, PartialEq, Debug)]
pub struct IstanbulExtra {
    /// AddedValidators are the validators that have been added in the block
    pub added_validators: Vec<Address>,

    /// AddedValidatorsPublicKeys are the BLS public keys for the validators added in the block
    pub added_validators_public_keys: Vec<SerializedPublicKey>,
    
    /// RemovedValidators is a bitmap having an active bit for each removed validator in the block
    pub removed_validators: u64, // NOTE: it was a big.Int

    /// Seal is an ECDSA signature by the proposer
    pub seal: Vec<u8>,

    /// AggregatedSeal contains the aggregated BLS signature created via IBFT consensus.
    pub aggregated_seal: IstanbulAggregatedSeal,

    /// ParentAggregatedSeal contains and aggregated BLS signature for the previous block.
    pub parent_aggregated_seal: IstanbulAggregatedSeal

}

impl IstanbulExtra {
    pub fn new(bytes: &[u8]) -> Result<Self, DecoderError>{
        if bytes.len() < ISTANBUL_EXTRA_VANITY_LENGTH {
            return Err(DecoderError::Custom("invalid istanbul header extra-data"));
        }

        rlp::decode(&bytes[ISTANBUL_EXTRA_VANITY_LENGTH..])
    }
}

impl Decodable for IstanbulExtra {
        fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
            let added_validators: Vec<Address> = rlp
                .at(0)
                .unwrap()
                .iter()
                .map(|r| r.decoder().decode_value(bytes_to_address).unwrap())
                .collect();

            let added_validators_public_keys: Vec<SerializedPublicKey> = rlp
                .at(1)
                .unwrap()
                .iter()
                .map(|r| r.decoder().decode_value(bytes_to_serialized_public_key).unwrap())
                .collect();

            Ok(IstanbulExtra{
                added_validators,
                added_validators_public_keys,
                removed_validators: rlp.val_at(2)?,
                seal: rlp.val_at(3)?,
                aggregated_seal: rlp.val_at(4)?,
                parent_aggregated_seal: rlp.val_at(5)?,
            })
        }
}

fn bytes_to_address(bytes: &[u8]) -> Result<Address, DecoderError> {
    if bytes.len() != ADDRESS_LENGTH {
        return Err(DecoderError::Custom("invalid data length while rlp decoding Address type"));
    }

    let mut address: Address = [0 as u8; ADDRESS_LENGTH];
    address.copy_from_slice(bytes);

    Ok(address)
}

fn bytes_to_serialized_public_key(bytes: &[u8]) -> Result<SerializedPublicKey, DecoderError> {
    if bytes.len() != PUBLIC_KEY_LENGTH {
        return Err(DecoderError::Custom("invalid data length while rlp decoding SerializedPublicKey type"));
    }

    // NOTE: There is no Default trait impl for SerializedPublicKey, so we need to have two
    // separate methods (instead of one with generic type)
    let mut key = [0 as u8; PUBLIC_KEY_LENGTH];
    key.copy_from_slice(bytes);

    Ok(key)
}


#[cfg(test)]
mod tests {
    use super::*;

    const ISTANBUL_EXTRA_HEX: &str = "f6ea9444add0ec310f115a0e603b2d7db9f067778eaf8a94294fc7e8f22b3bcdcf955dd7ff3ba2ed833f8212c00c80c3808080c3808080";

    fn get_istanbul_extra(data: &str, vanity: Vec<u8>) -> Vec<u8> {
        let extra = hex::decode(data).unwrap();

        vanity.into_iter().chain(extra.into_iter()).collect()
    }

    #[test]
    fn contructs_valid_istanbul_extra() {
        let bytes = get_istanbul_extra(ISTANBUL_EXTRA_HEX, vec![0; ISTANBUL_EXTRA_VANITY_LENGTH]);
        let parsed = IstanbulExtra::new(bytes.as_slice()).unwrap();
        let expected = IstanbulExtra {
            added_validators: vec![
                bytes_to_address(hex::decode("44add0ec310f115a0e603b2d7db9f067778eaf8a").unwrap().as_slice()).unwrap(),
                bytes_to_address(hex::decode("294fc7e8f22b3bcdcf955dd7ff3ba2ed833f8212").unwrap().as_slice()).unwrap(),
            ],
            added_validators_public_keys: vec![],
            removed_validators: 12,
            seal: Vec::new(),
            aggregated_seal: IstanbulAggregatedSeal{
                bitmap: 0,
                signature: Vec::new(),
                round: 0
            },
            parent_aggregated_seal: IstanbulAggregatedSeal{
                bitmap: 0,
                signature: Vec::new(),
                round: 0
            },
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn rejects_insufficient_vanity() {
        let bytes = get_istanbul_extra("", vec![0; ISTANBUL_EXTRA_VANITY_LENGTH-1]);
        
        assert_eq!(
            IstanbulExtra::new(bytes.as_slice()).unwrap_err(),
            DecoderError::Custom("invalid istanbul header extra-data")
        );
    }
}
