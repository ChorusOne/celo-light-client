use crate::contract::types::ibc::Height;

use cosmwasm_std::{StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read};

pub const KEY_STATE_CLIENT: &[u8] = b"client_state";

// processed_time_key returns the key under which the processed time will be stored in the client store
pub fn processed_time_key(height: Height) -> Vec<u8> {
    // consensusStates/ path is defined in ICS 24
    format!("consensusStates/{}/processedTime", height)
        .as_bytes()
        .to_owned()
}

// set_processed_time stores the time at which a header was processed and the corresponding consensus state was created.
// This is useful when validating whether a packet has reached the specified delay period in the
// tendermint client's verification functions
pub fn set_processed_time(storage: &mut dyn Storage, height: Height, time: u64) -> StdResult<()> {
    let key = processed_time_key(height);
    singleton(storage, &key).save(&time)
}

// get_processed_time gets the time (in nanoseconds) at which this chain recieved and processed a celo header.
// This is used to validate that a recieved packet has passed the delay period
pub fn get_processed_time(storage: &dyn Storage, height: Height) -> StdResult<u64> {
    let key = processed_time_key(height);
    singleton_read(storage, &key).load()
}
