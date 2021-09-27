use cosmwasm_std::{to_binary, Response, StdError, StdResult};
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
