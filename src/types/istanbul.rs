use crate::types::header::Address;
use crate::types::header::ADDRESS_LENGTH;

use rlp::{DecoderError, Decodable, Rlp, Encodable, RlpStream};
use rug::{integer::Order, Integer};

// SOURCE: github.com/celo-org/celo-bls-go@v0.1.6/bls/bls.go
pub const PUBLIC_KEY_LENGTH: usize = 96;

// SOURCE: crypto/bls/bls.go
pub type SerializedPublicKey = [u8; PUBLIC_KEY_LENGTH];

// SOURCE: core/types/istanbul.go
pub const ISTANBUL_EXTRA_VANITY_LENGTH: usize = 32;

#[derive(Clone, PartialEq, Debug)]
pub struct IstanbulAggregatedSeal {
    /// Bitmap is a bitmap having an active bit for each validator that signed this block
    pub bitmap: Integer,

    /// Signature is an aggregated BLS signature resulting from signatures by each validator that signed this block
    pub signature: Vec<u8>,

    /// Round is the round in which the signature was created.
    pub round: Integer,
}

impl IstanbulAggregatedSeal {
    pub fn new() -> Self {
        Self {
            bitmap: Integer::new(),
            signature: Vec::new(),
            round: Integer::new(),
        }
    }
}

impl Encodable for IstanbulAggregatedSeal {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(3);

        // bitmap
        s.append(&self.bitmap.to_digits(Order::LsfBe));

        // signature
        s.append(&self.signature);

        // round
        s.append(&self.round.to_digits(Order::LsfBe));
    }
}

impl Decodable for IstanbulAggregatedSeal {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(IstanbulAggregatedSeal{
            bitmap: rlp_to_big_int(rlp, 0)?,
            signature: rlp.val_at(1)?,
            round: rlp_to_big_int(rlp, 2)?
        })
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct IstanbulExtra {
    /// AddedValidators are the validators that have been added in the block
    pub added_validators: Vec<Address>,

    /// AddedValidatorsPublicKeys are the BLS public keys for the validators added in the block
    pub added_validators_public_keys: Vec<SerializedPublicKey>,
    
    /// RemovedValidators is a bitmap having an active bit for each removed validator in the block
    pub removed_validators: Integer,

    /// Seal is an ECDSA signature by the proposer
    pub seal: Vec<u8>,

    /// AggregatedSeal contains the aggregated BLS signature created via IBFT consensus.
    pub aggregated_seal: IstanbulAggregatedSeal,

    /// ParentAggregatedSeal contains and aggregated BLS signature for the previous block.
    pub parent_aggregated_seal: IstanbulAggregatedSeal

}

impl IstanbulExtra {
    pub fn from_rlp(bytes: &[u8]) -> Result<Self, DecoderError>{
        if bytes.len() < ISTANBUL_EXTRA_VANITY_LENGTH {
            return Err(DecoderError::Custom("invalid istanbul header extra-data"));
        }

        rlp::decode(&bytes[ISTANBUL_EXTRA_VANITY_LENGTH..])
    }

    pub fn to_rlp(&self, vanity: &[u8]) -> Vec<u8> {
        let payload = rlp::encode(self);

        [&vanity[..], &payload[..]].concat()
    }


}

impl Encodable for IstanbulExtra {
    fn rlp_append(&self, s: &mut RlpStream) {
        // added_validators
        s.begin_list(6);
        s.begin_list(self.added_validators.len());
        for address in self.added_validators.iter() {
            s.append(&address.to_vec());
        }

        // added_validators_public_keys
        s.begin_list(self.added_validators_public_keys.len());
        for address in self.added_validators_public_keys.iter() {
            s.append(&address.to_vec()); // TODO: can we do it without conversion?
        }

        // removed_validators
        s.append(&self.removed_validators.to_digits(Order::LsfBe));

        // seal
        s.append(&self.seal);

        // aggregated_seal
        s.append(&self.aggregated_seal);

        // parent_aggregated_seal
        s.append(&self.parent_aggregated_seal);
    }
}

impl Decodable for IstanbulExtra {
        fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
            let added_validators: Vec<Address> = rlp
                .at(0)?
                .iter()
                .map(|r| r.decoder().decode_value(bytes_to_address).unwrap())
                .collect();

            let added_validators_public_keys: Vec<SerializedPublicKey> = rlp
                .at(1)?
                .iter()
                .map(|r| r.decoder().decode_value(bytes_to_serialized_public_key).unwrap())
                .collect();

            Ok(IstanbulExtra{
                added_validators,
                added_validators_public_keys,
                removed_validators: rlp_to_big_int(rlp, 2)?,
                seal: rlp.val_at(3)?,
                aggregated_seal: rlp.val_at(4)?,
                parent_aggregated_seal: rlp.val_at(5)?,
            })
        }
}

fn rlp_to_big_int(rlp: &Rlp, index: usize) -> Result<Integer, DecoderError> {
    rlp.at(index)?.decoder().decode_value(
        |bytes| Ok(Integer::from_digits(bytes, Order::LsfBe))
    )
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

    const ISTANBUL_EXTRA_TINY: &str = "f6ea9444add0ec310f115a0e603b2d7db9f067778eaf8a94294fc7e8f22b3bcdcf955dd7ff3ba2ed833f8212c00c80c3808080c3808080";
    const ISTANBUL_EXTRA_DUMPED: &str = "d983010101846765746889676f312e31342e3132856c696e7578000000000000f8cac0c080b841a8b3d35da347081f80afc933db0c011236d4d6ae70f3be01034d51ec4df705f0052b3b0ce8f00ea5725bc3c85d1ca9db576bf232e88dba796035dd3f5d38993601f8408d06fa6fbfdf8e21dedfff7424f5b057da2494d732eb6acce49e264bfef4b7f04e09e2df3091b3db50f144a1a312096f7503af61c7b1e15d49880cb55d8a0080f8408d0fffffffffffffffffffffffffb05816c52ed2c34ad42672a2c603ce73a0966987583243c9f3cbf8f4d6c4b924023e2d1ccdc61f807539fad03bd0573f8080";

    fn get_istanbul_extra(data: &str, vanity: Vec<u8>) -> Vec<u8> {
        let extra = hex::decode(data).unwrap();

        vanity.into_iter().chain(extra.into_iter()).collect()
    }

    #[test]
    fn contructs_valid_istanbul_extra() {
        let bytes = get_istanbul_extra(ISTANBUL_EXTRA_TINY, vec![0; ISTANBUL_EXTRA_VANITY_LENGTH]);
        let parsed = IstanbulExtra::from_rlp(&bytes).unwrap();
        let expected = IstanbulExtra {
            added_validators: vec![
                bytes_to_address(hex::decode("44add0ec310f115a0e603b2d7db9f067778eaf8a").unwrap().as_slice()).unwrap(),
                bytes_to_address(hex::decode("294fc7e8f22b3bcdcf955dd7ff3ba2ed833f8212").unwrap().as_slice()).unwrap(),
            ],
            added_validators_public_keys: vec![],
            removed_validators: Integer::from(12),
            seal: Vec::new(),
            aggregated_seal: IstanbulAggregatedSeal{
                bitmap: Integer::new(),
                signature: Vec::new(),
                round: Integer::new(),
            },
            parent_aggregated_seal: IstanbulAggregatedSeal{
                bitmap: Integer::new(),
                signature: Vec::new(),
                round: Integer::new(),
            },
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn rejects_insufficient_vanity() {
        let bytes = get_istanbul_extra("", vec![0; ISTANBUL_EXTRA_VANITY_LENGTH-1]);
        
        assert_eq!(
            IstanbulExtra::from_rlp(&bytes).unwrap_err(),
            DecoderError::Custom("invalid istanbul header extra-data")
        );
    }

    #[test]
    fn encodes_istanbul_extra() {
        for extra_bytes in vec![
            get_istanbul_extra(ISTANBUL_EXTRA_TINY, vec![0; ISTANBUL_EXTRA_VANITY_LENGTH]),
            hex::decode(&ISTANBUL_EXTRA_DUMPED).unwrap(),
        ] {
            let decoded_ist = IstanbulExtra::from_rlp(&extra_bytes).unwrap();
            let encoded_ist_bytes = decoded_ist.to_rlp(&extra_bytes[..ISTANBUL_EXTRA_VANITY_LENGTH]);

            assert_eq!(encoded_ist_bytes, extra_bytes);
        }
    }
}
