use crate::traits::FromRlp;
use cosmwasm_std::{from_slice, StdError, StdResult};
use serde::de::DeserializeOwned;
use std::any::type_name;

pub fn from_base64<S: Into<String>>(
    base64_data: &String,
    target_type: S,
) -> Result<Vec<u8>, StdError> {
    match base64::decode(base64_data) {
        Ok(bytes) => Ok(bytes),
        Err(e) => {
            return Err(StdError::parse_err(
                target_type,
                format!("Unable to base64 decode data. Error: {}", e),
            ))
        }
    }
}

pub fn from_base64_rlp<T, S>(base64_data: &String, target_type: S) -> Result<T, StdError>
where
    T: FromRlp,
    S: Into<String> + Clone,
{
    let bytes = from_base64(&base64_data, target_type.clone())?;

    Ok(T::from_rlp(bytes.as_slice()).map_err(|e| {
        StdError::parse_err(
            target_type,
            format!("Unable to rlp decode from base64 data. Error: {}", e),
        )
    })?)
}

pub fn from_base64_json_slice<T, S>(base64_data: &String, target_type: S) -> Result<T, StdError>
where
    T: DeserializeOwned,
    S: Into<String> + Clone,
{
    let bytes = from_base64(base64_data, target_type.clone())?;

    let t: T = from_slice(&bytes).map_err(|e| {
        StdError::parse_err(
            target_type,
            format!("Unable to json decode data. Error: {}", e),
        )
    })?;

    Ok(t)
}

pub fn must_deserialize<T: DeserializeOwned>(value: &Option<Vec<u8>>) -> StdResult<T> {
    match value {
        Some(vec) => from_slice(&vec),
        None => Err(StdError::not_found(type_name::<T>())),
    }
}
