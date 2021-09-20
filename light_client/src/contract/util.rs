use cosmwasm_std::{to_binary, Response, StdError, StdResult, Binary};
use serde::Serialize;
use std::fmt::Display;

pub fn to_generic_err<T>(err: T) -> StdError
where
    T: Display,
{
    StdError::GenericErr {
        msg: err.to_string(),
    }
}

/*
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
*/

pub fn wrap_response<T>(result: T, action: &'static str) -> StdResult<Response>
where
    T: Serialize,
{
    let response_data = to_binary(&result)?;
    let response = Response::new()
        .add_attribute("action", action)
        .set_data(response_data);

    Ok(response)
}
