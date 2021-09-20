use cosmwasm_std::{from_slice, StdError, StdResult};
use serde::de::DeserializeOwned;
use std::any::type_name;

pub fn from_base64<T: AsRef<[u8]>>(data: T, target_type: &str) -> Result<Vec<u8>, StdError> {
    match base64::decode(data) {
        Ok(bytes) => Ok(bytes),
        Err(e) => {
            return Err(StdError::parse_err(
                target_type,
                format!("Unable to base64 decode data. Error: {}", e),
            ))
        }
    }
}

pub fn from_base64_rlp<T, D>(base64_data: D, target_type: &str) -> Result<T, StdError>
where
    T: rlp::Decodable,
    D: AsRef<[u8]>,
{
    let bytes = from_base64(base64_data, target_type.clone())?;

    rlp::decode(bytes.as_slice()).map_err(|e| {
        StdError::parse_err(
            target_type,
            format!("Unable to rlp decode from base64 data. Error: {}", e),
        )
    })
}

pub fn from_base64_json_slice<T>(base64_data: &str, target_type: &str) -> Result<T, StdError>
where
    T: DeserializeOwned,
{
    let bytes = from_base64(base64_data, target_type)?;

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
        Some(vec) => from_slice(vec),
        None => Err(StdError::not_found(type_name::<T>())),
    }
}
