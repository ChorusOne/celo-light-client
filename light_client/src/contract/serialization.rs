use cosmwasm_std::{from_slice, StdError, StdResult};
use serde::de::DeserializeOwned;
use std::any::type_name;

pub fn must_deserialize<T: DeserializeOwned>(value: &Option<Vec<u8>>) -> StdResult<T> {
    match value {
        Some(vec) => from_slice(vec),
        None => Err(StdError::not_found(type_name::<T>())),
    }
}
