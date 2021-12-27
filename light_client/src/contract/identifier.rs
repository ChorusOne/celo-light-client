use ethereum_types::{U256};
use sha3::{Digest, Keccak256, Sha3_256};
use cosmwasm_std::{StdResult, StdError, Binary};
use ibc_proto::ibc::core::commitment::v1::MerklePrefix;
use ibc_proto::ibc::core::client::v1::Height;


pub(crate) fn hash_committed_value<T: prost::Message>(original: &T) -> U256 {
    let encoded = original.encode_to_vec();
    let hashed = Keccak256::digest(&encoded);
    U256::from(hashed.as_slice())
} 

pub(crate) fn build_key(map_position: U256, map_key: U256) -> U256 {
    let mut placeholder : [u8; 64] = [0; 64];
    map_key.to_little_endian(&mut placeholder[0..32]);
    map_position.to_little_endian(&mut placeholder[32..]);
    let encoded_key = Keccak256::digest(&placeholder);
    U256::from(encoded_key.as_slice())
}

// https://github.com/datachainlab/yui-ibc-solidity/blob/master/contracts/core/IBCIdentifier.sol#L17
pub(crate) fn client_commitment_key(map_position: U256, prefix: MerklePrefix, client_id: &str) -> StdResult<U256> {
    if prefix.key_prefix.len() != 1 {
        return Err(StdError::InvalidDataSize{expected: 1, actual: prefix.key_prefix.len() as u64})
    }
    let mut key = vec![prefix.key_prefix[0]];
    let mut c_id = client_id.as_bytes().to_vec();
    key.append(&mut c_id);
    let encoded_key = Keccak256::digest(&key);
    Ok(build_key(map_position, U256::from(encoded_key.as_slice())))
}

// https://github.com/datachainlab/yui-ibc-solidity/blob/master/contracts/core/IBCIdentifier.sol#L21
pub(crate) fn consensus_commitment_key(map_position: U256, prefix: MerklePrefix, client_id: &str, height: &Height) -> StdResult<U256> {
    if prefix.key_prefix.len() != 1 {
        return Err(StdError::InvalidDataSize{expected: 1, actual: prefix.key_prefix.len() as u64})
    }
    let mut key = vec![prefix.key_prefix[0]];
    let mut c_id = client_id.as_bytes().to_vec();
    key.append(&mut c_id);
    key.push(b'/');
    let mut h = height.revision_height.to_be_bytes().to_vec();
    key.append(&mut h);
    let encoded_key = Keccak256::digest(&key);
    Ok(build_key(map_position, U256::from(encoded_key.as_slice())))
}

// https://github.com/datachainlab/yui-ibc-solidity/blob/master/contracts/core/IBCIdentifier.sol#L25
pub(crate) fn connection_commitment_key(map_position: U256, prefix: MerklePrefix, connection_id: &str) -> StdResult<U256> {
    if prefix.key_prefix.len() != 1 {
        return Err(StdError::InvalidDataSize{expected: 1, actual: prefix.key_prefix.len() as u64})
    }
    let mut key = vec![prefix.key_prefix[0]];
    let mut c_id = connection_id.as_bytes().to_vec();
    key.append(&mut c_id);
    let encoded_key = Keccak256::digest(&key);
    Ok(build_key(map_position, U256::from(encoded_key.as_slice())))
}

// https://github.com/datachainlab/yui-ibc-solidity/blob/master/contracts/core/IBCIdentifier.sol#L29
pub(crate) fn channel_commitment_key(map_position: U256, prefix: MerklePrefix, port_id: &str, channel_id: &str) -> StdResult<U256> {
    if prefix.key_prefix.len() != 1 {
        return Err(StdError::InvalidDataSize{expected: 1, actual: prefix.key_prefix.len() as u64})
    }
    let mut key = vec![prefix.key_prefix[0]];
    let mut p_id = port_id.as_bytes().to_vec();
    key.append(&mut p_id);
    key.push(b'/');
    let mut c_id = channel_id.as_bytes().to_vec();
    key.append(&mut c_id);
    let encoded_key = Keccak256::digest(&key);
    Ok(build_key(map_position, U256::from(encoded_key.as_slice())))
}

// https://github.com/datachainlab/yui-ibc-solidity/blob/master/contracts/core/IBCIdentifier.sol#L33
// https://github.com/datachainlab/yui-ibc-solidity/blob/1f86d0108145868ebbca182d6fc8e716aacb7533/contracts/core/IBCIdentifier.sol#L37
pub(crate) fn packet_key(map_position: U256, prefix: MerklePrefix, port_id: &str, channel_id: &str, sequence: u64) -> StdResult<U256> {
    if prefix.key_prefix.len() != 1 {
        return Err(StdError::InvalidDataSize{expected: 1, actual: prefix.key_prefix.len() as u64})
    }
    let mut key = vec![prefix.key_prefix[0]];
    let mut p_id = port_id.as_bytes().to_vec();
    key.append(&mut p_id);
    key.push(b'/');
    let mut c_id = channel_id.as_bytes().to_vec();
    key.append(&mut c_id);
    key.push(b'/');
    let mut s = sequence.to_be_bytes().to_vec();
    key.append(&mut s);
    let encoded_key = Keccak256::digest(&key);
    Ok(build_key(map_position, U256::from(encoded_key.as_slice())))
}


pub(crate) fn packet_commit_ack(ack: &Binary) -> U256 {
    U256::from(Sha3_256::digest(ack.as_slice()).as_slice())
}
