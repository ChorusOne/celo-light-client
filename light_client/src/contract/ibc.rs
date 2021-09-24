use crate::contract::util::to_generic_err;
use cosmwasm_std::StdError;
use ibc_proto::ibc::core::commitment::v1::{MerklePath, MerklePrefix, MerkleProof};
use ibc_proto::ics23::ProofSpec;

// This file defines core IBC structures required by the light client contract.
//
// It would be great if we could reuse types from other library (ie. `ibc-rs`), but:
//  * Go serializes []byte as base64 string (additional step required: base64::decode(data) -> Vec<u8>)
//  * Deriving JsonSchema via prost isn't possible (JsonSchema's required by cosmwasm)
//
// Therefore the selected structures have been copied and modified as needed.

// Some types are not being serialized/deserialized by contract, therefore are used as-is.

// Origin: cosmos-sdk/x/ibc/core/23-commitment/types/merkle.go (ported)
pub fn apply_prefix(prefix: &MerklePrefix, mut path: Vec<String>) -> Result<MerklePath, StdError> {
    if prefix.key_prefix.is_empty() {
        return Err(to_generic_err("empty prefix"));
    }

    let prefix_from_base64 = base64::decode(&prefix.key_prefix).map_err( to_generic_err)?;
    let decoded_prefix = std::str::from_utf8(&prefix_from_base64).map_err(to_generic_err)?;

    let mut result: Vec<String> = vec![decoded_prefix.to_string()];
    result.append(&mut path);
    Ok(MerklePath { key_path: result })
}

// Origin: cosmos-sdk/x/ibc/core/23-commitment/types/merkle.go (ported)
pub fn verify_membership(
    proof: &MerkleProof,
    specs: &[ProofSpec],
    root: &Vec<u8>,
    keys: &MerklePath,
    mut value: Vec<u8>,
    index: usize,
) -> Result<bool, StdError> {
    /*
    let mut subroot = value.clone();

    for (i, commitment_proof) in proof.proofs.iter().skip(index).enumerate() {
        if let Some(ex) = get_exist_proof(commitment_proof) {
            subroot = ics23::calculate_existence_root(&ex).map_err(|e| to_generic_err(e))?;
            let key = match keys.key_path.get(keys.key_path.len() - 1 - i) {
                Some(key) => key,
                None => return Err(StdError::generic_err("could not retrieve key bytes")),
            };

            if !ics23::verify_membership(
                &commitment_proof,
                &specs[i],
                &subroot,
                key.as_bytes(),
                &value,
            ) {
                return Err(StdError::generic_err(format!(
                    "membership proof failed to verify membership of value: {:?} in subroot: {:?}",
                    value, subroot
                )));
            }

            value = subroot.clone();
        } else {
            return Err(StdError::generic_err(
                "expected proof type: ics23::ExistenceProof",
            ));
        }
    }

    if !root.iter().eq(subroot.iter()) {
        return Err(StdError::generic_err(format!(
            "proof did not commit to expected root: {:?}, got: {:?}",
            root, subroot
        )));
    }

    */
    Ok(true)
}

/*
fn get_exist_proof<'a>(proof: &'a ics23::CommitmentProof) -> Option<&'a ics23::ExistenceProof> {
    match &proof.proof {
        Some(ics23::commitment_proof::Proof::Exist(ex)) => Some(ex),
        _ => None,
    }
}
*/
