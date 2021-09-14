use byteorder::{BigEndian, ByteOrder};
use cosmwasm_std::{attr, to_vec, Binary};
use cosmwasm_std::{HandleResponse, StdError, StdResult};
use serde::Serialize;
use std::fmt::Display;

pub fn u64_to_big_endian(value: u64) -> Vec<u8> {
    let mut buf = [0; 8];
    BigEndian::write_u64(&mut buf, value);

    buf.to_vec()
}

pub fn wrap_response<T>(result: T, action: &'static str) -> Result<HandleResponse, StdError>
where
    T: Serialize,
{
    let response_data = Binary(to_vec(&result)?);

    Ok(HandleResponse {
        messages: vec![],
        attributes: vec![attr("action", action)],
        data: Some(response_data),
    })
}

pub fn to_generic_err<T>(err: T) -> StdError
where
    T: Display,
{
    StdError::GenericErr {
        msg: err.to_string(),
    }
}

// TODO: temporary remap, once ICS is settled this should be removed
pub fn to_binary(t: HandleResponse) -> Binary {
    t.data.unwrap()
}
