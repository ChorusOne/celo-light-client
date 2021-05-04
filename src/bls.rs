use crate::algebra::CanonicalDeserialize;
use crate::errors::{Error, Kind};
use crate::istanbul::min_quorum_size;
use crate::serialization::rlp::big_int_to_rlp_compat_bytes;
use crate::types::header::Hash;
use crate::types::istanbul::{IstanbulAggregatedSeal, IstanbulMsg};
use crate::types::state::Validator;
use bls_crypto::{hash_to_curve::try_and_increment::DIRECT_HASH_TO_G1, PublicKey, Signature};
use num_bigint::BigInt as Integer;

/// Uses BLS signature verification to validate header against provided validator set
pub fn verify_aggregated_seal(
    header_hash: Hash,
    validators: &[Validator],
    aggregated_seal: &IstanbulAggregatedSeal,
) -> Result<(), Error> {
    let proposal_seal = prepare_commited_seal(header_hash, &aggregated_seal.round);
    let expected_quorum_size = min_quorum_size(validators.len());

    // Find which public keys signed from the provided validator set
    let public_keys = validators
        .iter()
        .enumerate()
        .filter(|(i, _)| aggregated_seal.bitmap.bit(*i as u64))
        .map(|(_, validator)| deserialize_pub_key(&validator.public_key))
        .collect::<Result<Vec<PublicKey>, Error>>()?;

    if public_keys.len() < expected_quorum_size {
        return Err(Kind::MissingSeals {
            current: public_keys.len(),
            expected: expected_quorum_size,
        }
        .into());
    }

    let sig = deserialize_signature(&aggregated_seal.signature)?;
    let apk = PublicKey::aggregate(public_keys);

    match apk.verify(&proposal_seal, &[], &sig, &*DIRECT_HASH_TO_G1) {
        Ok(_) => Ok(()),
        Err(_) => Err(Kind::BlsVerifyError.into()),
    }
}

fn prepare_commited_seal(hash: Hash, round: &Integer) -> Vec<u8> {
    let round_bytes = big_int_to_rlp_compat_bytes(&round);
    let commit_bytes = [IstanbulMsg::Commit as u8];

    [&hash[..], &round_bytes[..], &commit_bytes[..]].concat()
}

fn deserialize_signature(signature: &[u8]) -> Result<Signature, Error> {
    Signature::deserialize(signature).map_err(|e| Kind::BlsInvalidSignature.context(e).into())
}

fn deserialize_pub_key(key: &[u8]) -> Result<PublicKey, Error> {
    PublicKey::deserialize(key).map_err(|e| Kind::BlsInvalidPublicKey.context(e).into())
}
