use crate::contract::serialization::must_deserialize;
use ibc_proto::ibc::core::client::v1::Height;

use cosmwasm_std::{to_vec, Env, StdError, StdResult, Storage};

pub const SUBJECT_PREFIX: &str = "subject/";
pub const SUBSTITUTE_PREFIX: &str = "substitute/";
pub const EMPTY_PREFIX: &str = "";

// processed_height_key returns the key under which the processed processed height will be stored in the client store
pub fn processed_height_key(prefix: &'static str, height: &Height) -> Vec<u8> {
    // consensusStates/ path is defined in ICS 24
    format!(
        "{}consensusStates/{}-{}/processedHeight",
        prefix, height.revision_number, height.revision_height
    )
    .as_bytes()
    .to_owned()
}

// processed_time_key returns the key under which the processed time will be stored in the client store
pub fn processed_time_key(prefix: &'static str, height: &Height) -> Vec<u8> {
    // consensusStates/ path is defined in ICS 24
    format!(
        "{}consensusStates/{}-{}/processedTime",
        prefix, height.revision_number, height.revision_height
    )
    .as_bytes()
    .to_owned()
}

// consensus_state_key returns the key under which the consensys state will be stored in the client store
pub fn consensus_state_key(prefix: &'static str, height: &Height) -> Vec<u8> {
    // consensusStates/ path is defined in ICS 24
    format!(
        "{}consensusStates/{}-{}",
        prefix, height.revision_number, height.revision_height
    )
    .as_bytes()
    .to_owned()
}

// set_processed_height stores the height at which a header was processed and the corresponding consensus state was created.
// This is useful when validating whether a packet has reached the specified block delay period in the light client's
// verification functions
fn set_processed_height(
    storage: &mut dyn Storage,
    prefix: &'static str,
    consensus_height: &Height,
    processed_height: &Height,
) -> StdResult<()> {
    let key = processed_height_key(prefix, consensus_height);
    storage.set(&key, &to_vec(processed_height)?);

    Ok(())
}

// get_processed_time gets the height at which this chain received and processed a celo header.
// This is used to validate that a received packet has passed the block delay period.
pub fn get_processed_height(
    storage: &dyn Storage,
    prefix: &'static str,
    height: &Height,
) -> StdResult<Height> {
    let key = processed_height_key(prefix, height);

    must_deserialize(&storage.get(&key))
}

// set_processed_time stores the time at which a header was processed and the corresponding consensus state was created.
// This is useful when validating whether a packet has reached the specified delay period in the
// light client's verification functions
fn set_processed_time(
    storage: &mut dyn Storage,
    prefix: &'static str,
    height: &Height,
    time: &cosmwasm_std::Timestamp,
) -> StdResult<()> {
    let key = processed_time_key(prefix, height);
    storage.set(&key, &to_vec(time)?);

    Ok(())
}

pub fn get_processed_time(
    storage: &dyn Storage,
    prefix: &'static str,
    height: &Height,
) -> StdResult<cosmwasm_std::Timestamp> {
    let key = processed_time_key(prefix, height);

    must_deserialize(&storage.get(&key))
}

pub fn set_consensus_meta(
    env: &Env,
    storage: &mut dyn Storage,
    prefix: &'static str,
    height: &Height,
) -> StdResult<()> {
    set_processed_time(storage, prefix, height, &env.block.time)?;
    set_processed_height(storage, prefix, height, &get_self_height(env.block.height))?;

    Ok(())
}

pub fn get_consensus_state(
    storage: &dyn Storage,
    prefix: &'static str,
    height: &Height,
) -> StdResult<Vec<u8>> {
    let key = consensus_state_key(prefix, height);

    match storage.get(&key) {
        Some(vec) => Ok(vec),
        None => Err(StdError::not_found("consensus state not found")),
    }
}

pub fn set_consensus_state(
    storage: &mut dyn Storage,
    prefix: &'static str,
    height: &Height,
    bytes: &Vec<u8>,
) -> StdResult<()> {
    let key = consensus_state_key(prefix, height);
    storage.set(&key, &to_vec(bytes)?);

    Ok(())
}

pub fn get_self_height(block_height: u64) -> Height {
    Height {
        revision_number: 0,
        revision_height: block_height,
    }
}
