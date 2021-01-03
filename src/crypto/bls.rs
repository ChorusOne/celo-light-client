// TODO: clumsy rust style
use rug::{Integer, integer::Order};
use crate::algebra::{CanonicalDeserialize};
use crate::types::header::Hash;
use crate::types::istanbul::IstanbulAggregatedSeal;
use bls_crypto::{
    PublicKey, Signature,
    hash_to_curve::try_and_increment::DIRECT_HASH_TO_G1,
    BLSError
};
use crate::state::Validator;

fn prepare_commited_seal(hash: crate::types::header::Hash, round: &Integer) -> Vec<u8> {
    return itertools::concat(vec![
        hash.to_vec(),
        round.to_digits::<u8>(Order::Msf),
        vec![crate::types::istanbul::IstanbulMsg::Commit as u8],
    ]);
}

pub fn verify_aggregated_seal(header_hash: Hash, validators: Vec<Validator>, aggregated_seal: IstanbulAggregatedSeal) -> Result<(), BLSError>{
    let proposal_seal = prepare_commited_seal(header_hash, &aggregated_seal.round);

    // Find which public keys signed from the provided validator set
    let public_keys: Vec<PublicKey> = validators.iter()
        .enumerate()
        .filter(|(i, _)| aggregated_seal.bitmap.get_bit(*i as u32))
        .map(|(_, validator)| PublicKey::deserialize(&*validator.public_key.to_vec()).unwrap())
        .collect();

    if (public_keys.len() as u64) < crate::state::min_quorum_size(&validators) {
        panic!("Aggregated seal does not aggregate enough seals");
    }

    let try_and_increment = &*DIRECT_HASH_TO_G1;
    let sig = Signature::deserialize(aggregated_seal.signature.as_slice()).unwrap();
    let apk = PublicKey::aggregate(public_keys);
    
    apk.verify(&proposal_seal, &[], &sig, try_and_increment)
}
